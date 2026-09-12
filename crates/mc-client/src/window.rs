//! Windowed fly-camera (minifb). Falls back gracefully when no display
//! exists (headless servers) — see `main.rs`.

use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

use crate::camera::Camera;
use crate::headless::{FOG_COLOR, SKY_BOTTOM, SKY_TOP};
use crate::math::{Mat4, Vec3};
use crate::mesher::mesh_world;
use crate::raster::{Framebuf, RenderCtx};
use crate::textures::TextureLut;
use crate::world::World;

const MOVE_SPEED: f32 = 14.0;
const LOOK_SPEED: f32 = 1.6;

pub fn run_windowed(world: &World, lut: Option<&TextureLut>, width: usize, height: usize) -> anyhow::Result<()> {
    let mut window = Window::new(
        &format!("mc2rs — Minecraft {} in Rust", crate::build_info::MINECRAFT_VERSION),
        width,
        height,
        WindowOptions { resize: true, ..WindowOptions::default() },
    )?;

    let n = world.size_blocks() as f32;
    let mut cam = Camera {
        pos: Vec3::new(n / 2.0 + 8.0, 24.0, n / 2.0 + n * 0.5),
        yaw: 0.9,
        pitch: -0.45,
    };
    let quads = mesh_world(world, lut);
    let mut fb = Framebuf::new(width, height);
    let mut buf: Vec<u32> = vec![0; width * height];
    let mut last_title = Instant::now();
    let mut frames = 0u64;
    let mut fps = 0.0f64;
    let mut t_prev = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let t_now = Instant::now();
        let dt = t_now.duration_since(t_prev).as_secs_f32().min(0.1);
        t_prev = t_now;

        let keys = window.get_keys();
        let mut fwd = Vec3::ZERO;
        for k in &keys {
            let dir = cam.forward();
            let right = Vec3::new(dir.z, 0.0, -dir.x).normalized();
            match k {
                Key::W => fwd = fwd.add(dir),
                Key::S => fwd = fwd.sub(dir),
                Key::D => fwd = fwd.add(right),
                Key::A => fwd = fwd.sub(right),
                Key::Space => cam.pos.y += MOVE_SPEED * dt,
                Key::LeftShift => cam.pos.y -= MOVE_SPEED * dt,
                Key::Up => cam.pitch = (cam.pitch + LOOK_SPEED * dt).clamp(-1.5, 1.5),
                Key::Down => cam.pitch = (cam.pitch - LOOK_SPEED * dt).clamp(-1.5, 1.5),
                Key::Left => cam.yaw -= LOOK_SPEED * dt,
                Key::Right => cam.yaw += LOOK_SPEED * dt,
                _ => {}
            }
        }
        if fwd.length() > 1e-6 {
            cam.pos = cam.pos.add(fwd.normalized().scale(MOVE_SPEED * dt));
        }

        let t0 = Instant::now();
        fb.clear(SKY_TOP, SKY_BOTTOM);
        let ctx = RenderCtx {
            view: cam.view(),
            proj: Mat4::perspective(
                70.0f32.to_radians(),
                width as f32 / height as f32,
                0.1,
                512.0,
            ),
            fog_color: FOG_COLOR,
            fog_near: n * 0.5,
            fog_far: n * 1.6,
        };
        fb.draw_quads(&ctx, &quads);
        let render_ms = t0.elapsed().as_secs_f64() * 1000.0;
        buf = fb.to_argb();
        window.update_with_buffer(&buf, width, height)?;

        frames += 1;
        if last_title.elapsed().as_secs_f64() >= 1.0 {
            fps = frames as f64 / last_title.elapsed().as_secs_f64();
            frames = 0;
            last_title = Instant::now();
            window.set_title(&format!(
                "mc2rs — Minecraft {} in Rust — {:.0} fps (render {:.1} ms)",
                crate::build_info::MINECRAFT_VERSION, fps, render_ms
            ));
        }
    }
    Ok(())
}

/// Windowed GUI application: boot chain -> title -> menus, all driven by
/// generated data (official fonts, sprites, language, splashes).
pub fn run_gui_app(seed: u64, width: usize, height: usize) -> anyhow::Result<()> {
    use crate::gui::GuiAssets;
    use crate::screens::App;

    let mut window = Window::new(
        &format!("mc2rs — Minecraft {} (generated GUI)", crate::build_info::MINECRAFT_VERSION),
        width,
        height,
        WindowOptions { resize: true, ..WindowOptions::default() },
    )?;
    let mut app = App::new(seed);
    let mut assets = GuiAssets::open("assets-cache/jar");
    let mut fb = Framebuf::new(width, height);
    let mut buf: Vec<u32> = vec![0; width * height];

    while window.is_open() {
        let (mx, my) = window.get_mouse_pos(minifb::MouseMode::Discard).unwrap_or((0.0, 0.0));
        let gmx = (mx as usize / 2) as i32;
        let gmy = (my as usize / 2) as i32;
        if window.get_mouse_down(minifb::MouseButton::Left) {
            app.on_click(gmx, gmy);
        }
        app.update();
        fb.clear(crate::headless::SKY_TOP, crate::headless::SKY_BOTTOM);
        app.render(&mut fb, &mut assets);
        buf.iter_mut().zip(fb.to_argb()).for_each(|(o, c)| *o = c);
        window.update_with_buffer(&buf, width, height)?;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    Ok(())
}

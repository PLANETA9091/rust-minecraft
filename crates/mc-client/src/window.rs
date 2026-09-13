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

/// The live view's refresh cadence: re-materializing the world window is
/// the heavy part of an update, so streamed refreshes ship at most this
/// often (the FIRST refresh — the join frame itself — is never delayed;
/// a pending refresh flushes with the next packet, and vanilla servers
/// always have one in flight). The frame thread re-meshes each shipped
/// window in a few ms — the same work the join frame already paid.
const VIEW_CADENCE: std::time::Duration = std::time::Duration::from_millis(250);

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

/// The live join worker (off the frame thread): the full vanilla join
/// arc, the milestone shipped as the world screen's first view, then the
/// session KEEPS PUMPING — chunks, teleports and probes stream into the
/// SAME `LiveView` and every refreshed window ships to the frame thread
/// (cadence-limited, see [`VIEW_CADENCE`]). The stop flag ends the loop
/// between packets; dropping the session closes the socket — the
/// vanilla client's disconnect (no ghost client on the server).
fn run_join(
    id: u64,
    host: String,
    port: u16,
    tx: std::sync::mpsc::Sender<(u64, Result<crate::screens::LoginResult, String>)>,
    live_tx: std::sync::mpsc::Sender<
        Result<
            (
                crate::world::World,
                Option<crate::camera::Camera>,
                Option<String>,
            ),
            String,
        >,
    >,
    run: std::sync::Arc<std::sync::atomic::AtomicBool>,
) {
    // the REAL protocol number (login tolerates no 0-ping) and the
    // vanilla offline profile for the session name (screens::PLAYER_NAME).
    // The number speaks the GENERATED pack_claim manifest — the only
    // surface that refuses a jar which does not state it (build_info's
    // stamp may legally be -1 "unknown"; the manifest cannot) — and the
    // two generated surfaces are cross-pinned in screens::tests.
    // The live join retains the session past the world-load milestone —
    // the one-shot `connect_and_join_world` would close the stream there.
    match mc_protocol::connect_and_join_live(
        &host,
        port,
        mc_protocol::generated::pack_claim::PROTOCOL_VERSION,
        crate::screens::PLAYER_NAME,
    ) {
        Ok(mc_protocol::LiveJoin::Live { snapshot, mut session }) => {
            let (entity_id, view_distance, chunks) =
                (snapshot.info.entity_id, snapshot.info.view_distance, snapshot.chunks.len());
            // the profile's skin model: the login `textures` metadata the
            // session carried from the login phase — the slim/wide
            // selection the first-person arm renders with (None = classic)
            let slim_arm = session.skin_model.as_deref() == Some("slim");
            // the network world view: the milestone adopts into the LIVE
            // model and the first refresh IS the one-shot snapshot view
            // (same window decisions, same grid, same pose, same biome —
            // pinned by the world_view tests). Materializing happens here
            // off the frame thread; the frame thread only installs results.
            let mut view =
                crate::world_view::LiveView::from_snapshot(snapshot, crate::screens::WORLD_CHUNKS);
            // the sky pipeline's render state: the vanilla extract's
            // dimensionType.skybox() discipline, resolved through the
            // model the snapshot adopted into — the SYNCED registry's own
            // skybox is the server's truth (the wire decode), the codec's
            // optionalFieldOf default when the element states nothing,
            // the BUILT-IN table when the sync carried no dimension
            // table at all
            let skybox = view.model.skybox();
            // the fog pair rides with it: the dimension's STATED distances
            // (the nether's 10/96) or None — the window treatment's domain
            let fog = view.model.fog_distances();
            let first = view.refresh(crate::world_view::runtime_mapper);
            let spawn_biome = first.biome.clone();
            let shipped = tx.send((
                id,
                Ok(crate::screens::LoginResult::Loaded {
                    username: crate::screens::PLAYER_NAME.to_string(),
                    entity_id,
                    view_distance,
                    chunks,
                    world: first.world,
                    camera: first.camera,
                    spawn_biome,
                    slim_arm,
                    skybox,
                    fog,
                }),
            ));
            if shipped.is_err() {
                return; // receiver gone at shutdown — the session drops
            }
            // THE LIVE VIEW: the stream keeps feeding the same model —
            // re-sent chunks replace their columns, teleports re-anchor
            // the window and the pose, probes are answered in place
            let mut last_ship = std::time::Instant::now();
            loop {
                if !run.load(std::sync::atomic::Ordering::Relaxed) {
                    break; // the App closed the session (quit/cancel)
                }
                match session.pump() {
                    Ok(mc_protocol::PlayEvent::Disconnected { reason }) => {
                        // the server ended the session: report it (the App
                        // drops the report when it already left the world)
                        let _ = live_tx.send(Err(reason));
                        break;
                    }
                    Ok(event) => {
                        if view.apply(event) && last_ship.elapsed() >= VIEW_CADENCE {
                            let upd = view.refresh(crate::world_view::runtime_mapper);
                            if live_tx
                                .send(Ok((upd.world, upd.camera, upd.biome)))
                                .is_err()
                            {
                                break; // receiver gone at shutdown
                            }
                            last_ship = std::time::Instant::now();
                        }
                    }
                    Err(e) => {
                        // a transport or decode error kills the stream (a
                        // framed stream cannot resync) — report and end
                        let _ = live_tx.send(Err(format!("{e:#}")));
                        break;
                    }
                }
            }
        }
        Ok(mc_protocol::LiveJoin::Disconnected { reason }) => {
            let _ = tx.send((id, Ok(crate::screens::LoginResult::Disconnected { reason })));
        }
        Err(e) => {
            let _ = tx.send((id, Err(format!("{e:#}"))));
        }
    }
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
    // menu music: decode the real official .ogg tracks (fetched by mc2rs-gen
    // through the asset index), feed the scheduler real lengths and play the
    // schedule through the best output backend (rodio when built with
    // `--features audio` and a device exists; deterministic silence otherwise)
    let mut music = crate::audio::MenuMusic::start(&mut app);
    let mut assets = GuiAssets::open("assets-cache/jar");
    let mut fb = Framebuf::new(width, height);
    let mut buf: Vec<u32> = vec![0; width * height];
    // server-list pings run on detached worker threads (blocking status
    // requests, vanilla-style async pinger); finished results come back
    // through this channel and are applied at the top of each frame
    let (net_tx, net_rx) = std::sync::mpsc::channel::<
        (u64, Result<crate::screens::PingInfo, String>),
    >();
    let mut net_seen = 0usize; // cursor into the append-only request log
    let mut was_left_down = false; // the attack edge detector (vanilla left-click = swing)
    // offline-mode joins ride the same architecture: the "join" button
    // records the request, a detached worker runs the live join (login +
    // configuration + the play state pumped to the world-load milestone,
    // then the session keeps pumping), the outcome lands on the outcome
    // screen — a Loaded result carries REAL world facts from the network
    // and the LIVE VIEW channel keeps refreshing them while the session
    // lives (chunks stream in, teleports re-anchor the view)
    let (login_tx, login_rx) = std::sync::mpsc::channel::<
        (u64, Result<crate::screens::LoginResult, String>),
    >();
    let mut login_seen = 0usize; // cursor into the append-only request log
    let (live_tx, live_rx) = std::sync::mpsc::channel::<
        Result<
            (
                crate::world::World,
                Option<crate::camera::Camera>,
                Option<String>,
            ),
            String,
        >,
    >();

    while window.is_open() {
        if window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            app.on_escape();
        }
        let (mx, my) = window.get_mouse_pos(minifb::MouseMode::Discard).unwrap_or((0.0, 0.0));
        let gmx = (mx as usize / 2) as i32;
        let gmy = (my as usize / 2) as i32;
        let left_down = window.get_mouse_down(minifb::MouseButton::Left);
        if left_down {
            app.on_click(gmx, gmy);
        }
        // the vanilla attack: the LEFT press's rising edge swings the arm
        // (minifb has no edge API — the previous frame's state is tracked here)
        if left_down && !was_left_down {
            app.on_attack();
        }
        was_left_down = left_down;
        app.update();
        // dispatch newly requested server-list pings to worker threads
        for req in &app.net_requests[net_seen..] {
            let tx = net_tx.clone();
            let id = req.id;
            let host = req.host.clone();
            let port = req.port;
            std::thread::spawn(move || {
                let res = mc_protocol::ping(&host, port)
                    .map(|r| crate::screens::PingInfo {
                        motd: r.motd,
                        version: r.version_name,
                        ping_ms: r.latency_ms.min(i32::MAX as u128) as i32,
                    })
                    .map_err(|e| format!("{e:#}"));
                let _ = tx.send((id, res)); // receiver may be gone at shutdown
            });
        }
        net_seen = app.net_requests.len();
        // snapshot the new requests first (the dispatch loop attaches the
        // live-session flags — a mutable borrow of the app)
        let dispatched: Vec<(u64, String, u16)> = app.login_requests[login_seen..]
            .iter()
            .map(|r| (r.id, r.host.clone(), r.port))
            .collect();
        login_seen = app.login_requests.len();
        for (id, host, port) in dispatched {
            let tx = login_tx.clone();
            let live_tx = live_tx.clone();
            // the live session's stop flag: the App flips it when the join
            // is cancelled, superseded or quit — the worker checks it
            // between packets and closes the socket (like the vanilla
            // client's disconnect; no ghost client lingers on the server)
            let run = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
            app.attach_live_run(id, std::sync::Arc::clone(&run));
            std::thread::spawn(move || run_join(id, host, port, tx, live_tx, run));
        }
        while let Ok((id, res)) = net_rx.try_recv() {
            app.ping_finished(id, res);
        }
        while let Ok((id, res)) = login_rx.try_recv() {
            app.login_finished(id, res);
        }
        // the live view: refreshed world windows swap in (a re-sent chunk
        // replaces its column on screen, a teleport re-anchors the view);
        // a session death while in the network world lands on the
        // Disconnect screen, exactly like the original's kick
        while let Ok(upd) = live_rx.try_recv() {
            match upd {
                Ok((world, camera, biome)) => app.live_view_updated(world, camera, biome),
                Err(reason) => app.live_session_lost(reason),
            }
        }
        music.sync(&app.music);
        // UI one-shots (ui.button.click) ride the independent UI channel
        music.sync_ui(&app.ui_sounds);
        fb.clear(crate::headless::SKY_TOP, crate::headless::SKY_BOTTOM);
        app.render(&mut fb, &mut assets);
        buf.iter_mut().zip(fb.to_argb()).for_each(|(o, c)| *o = c);
        window.update_with_buffer(&buf, width, height)?;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    Ok(())
}

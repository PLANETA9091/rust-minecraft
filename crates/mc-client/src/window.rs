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
    let mut fps: f64; // read only after the title-tick's first compute
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
                Option<crate::world_view::LiveContainer>,
                Option<(i32, i32)>,
                [(i32, i32); 9],
                crate::world_view::HealthView,
                Vec<crate::world_view::WireMob>,
                Vec<crate::world_view::WireVoice>,
            ),
            String,
        >,
    >,
    clicks: std::sync::mpsc::Receiver<crate::world_view::ContainerCommand>,
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
            // the cloud-fog end rides with it: the dimension's STATED
            // cloud fog or None — the bytecode default's domain
            let cloud_fog_end = view.model.cloud_fog_end_distance();
            let first = view.refresh(crate::world_view::runtime_mapper);
            // the mobs' hurt + voice mailboxes ship with this view —
            // the drains empty them (each arm rides exactly one
            // refresh, the mob lane's own tick/audio edge owns the
            // countdown and the playback after the landing)
            view.model.drain_entity_hurts();
            view.model.drain_entity_sounds();
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
                    cloud_fog_end,
                }),
            ));
            if shipped.is_err() {
                return; // receiver gone at shutdown — the session drops
            }
            // THE LIVE VIEW: the stream keeps feeding the same model —
            // re-sent chunks replace their columns, teleports re-anchor
            // the window and the pose, probes are answered in place
            let mut last_ship = std::time::Instant::now();
            // the use arc's prediction sequence counter (the vanilla
            // startPrediction's own shape: each use/release speaks the
            // next number — the server matches its block-change acks
            // against it)
            let mut use_sequence = 0i32;
            loop {
                if !run.load(std::sync::atomic::Ordering::Relaxed) {
                    break; // the App closed the session (quit/cancel)
                }
                // THE CONTAINER COMMANDS: the frame thread's slot clicks
                // and the player's own close echo drain FIRST, so a
                // queued command rides the wire before the next read
                // blocks. The read itself is blocking (a framed stream
                // cannot resync) — a command emitted during a quiet
                // patch waits for the server's next packet, the same
                // honest cadence the view refreshes already ride
                // (vanilla servers always have one in flight).
                for cmd in clicks.try_iter() {
                    let sent = match cmd {
                        crate::world_view::ContainerCommand::Click(c) => {
                            // THE PREDICTION RIDE: the client's believed
                            // post-click state — the changed slot's new
                            // value and the held cursor — speaks the wire
                            // as the HashedStack count pairs (the EMPTY
                            // patch is the (item_id, count) store's own
                            // honest projection; the (0, 0) sentinel is
                            // the wire's own emptiness byte). The server's
                            // corrections ride the container lane back.
                            let input = match c.input {
                                crate::world_view::ContainerInput::Pickup => {
                                    mc_protocol::ContainerInput::Pickup
                                }
                                crate::world_view::ContainerInput::QuickMove => {
                                    mc_protocol::ContainerInput::QuickMove
                                }
                                crate::world_view::ContainerInput::Swap => {
                                    mc_protocol::ContainerInput::Swap
                                }
                                crate::world_view::ContainerInput::Clone => {
                                    mc_protocol::ContainerInput::Clone
                                }
                                crate::world_view::ContainerInput::Throw => {
                                    mc_protocol::ContainerInput::Throw
                                }
                                crate::world_view::ContainerInput::QuickCraft => {
                                    mc_protocol::ContainerInput::QuickCraft
                                }
                                crate::world_view::ContainerInput::PickupAll => {
                                    mc_protocol::ContainerInput::PickupAll
                                }
                            };
                            session.click_container(&mc_protocol::ContainerClick {
                                container_id: c.window_id,
                                state_id: c.state_id,
                                slot: c.slot,
                                button: c.button,
                                input,
                                changed_slots: c
                                    .changed
                                    .iter()
                                    .map(|(s, (id, cnt))| {
                                        (
                                            *s,
                                            if *cnt > 0 {
                                                mc_protocol::HashedStack::item(*id, *cnt)
                                            } else {
                                                mc_protocol::HashedStack::Empty
                                            },
                                        )
                                    })
                                    .collect(),
                                carried: if c.carried.1 > 0 {
                                    mc_protocol::HashedStack::item(c.carried.0, c.carried.1)
                                } else {
                                    mc_protocol::HashedStack::Empty
                                },
                            })
                        }
                        crate::world_view::ContainerCommand::Close { window_id } => {
                            session.close_container(window_id)
                        }
                        crate::world_view::ContainerCommand::SelectHotbar { selection } => {
                            // THE LOCAL SELECTION PREDICTION: the vanilla
                            // keyPress moves its own Inventory.selected
                            // immediately — the store's selection moves
                            // HERE (the hand follows through the store's
                            // own projection) and the serverbound echo
                            // tells the server (the dump's short-shaped
                            // slot; the authoritative Set Held Slot still
                            // lands through its gate)
                            view.model.select_hotbar(selection as usize);
                            session.set_carried_item(selection)
                        }
                        crate::world_view::ContainerCommand::UseItem { yaw, pitch } => {
                            // THE USE ARC's wire echo (the vanilla
                            // gameMode.useItem's own shape): the MAIN_HAND
                            // ordinal, the prediction sequence counter
                            // (each prediction speaks the next number —
                            // the startPrediction shape), the camera's
                            // angles in the vanilla degrees
                            use_sequence += 1;
                            session.use_item(0, use_sequence, yaw, pitch)
                        }
                        crate::world_view::ContainerCommand::UseItemOn {
                            pos,
                            face,
                            cursor,
                            inside,
                        } => {
                            // THE BLOCK HALF's wire echo (the vanilla
                            // gameMode.useItemOn's own shape): the MAIN_HAND
                            // ordinal, the clip's hit through the
                            // writeBlockHitResult law, the same monotonic
                            // prediction counter the use/release spoke
                            use_sequence += 1;
                            session.use_item_on(
                                0,
                                pos,
                                face,
                                cursor,
                                inside,
                                false,
                                use_sequence,
                            )
                        }
                        crate::world_view::ContainerCommand::ReleaseUse => {
                            // the release's own echo: the RELEASE_USE_ITEM
                            // action at BlockPos.ZERO / Direction.DOWN —
                            // the same counter the use spoke
                            use_sequence += 1;
                            session.release_use_item(use_sequence)
                        }
                        crate::world_view::ContainerCommand::MovePosRot {
                            x,
                            y,
                            z,
                            yaw,
                            pitch,
                            on_ground,
                            horizontal_collision,
                        } => {
                            // THE MOVEMENT WIRE (the body's sendPosition
                            // dispatch): the position AND the rotation
                            // this tick named
                            session.move_player_pos_rot(
                                x, y, z, yaw, pitch, on_ground, horizontal_collision,
                            )
                        }
                        crate::world_view::ContainerCommand::MovePos {
                            x,
                            y,
                            z,
                            on_ground,
                            horizontal_collision,
                        } => session.move_player_pos(x, y, z, on_ground, horizontal_collision),
                        crate::world_view::ContainerCommand::MoveRot {
                            yaw,
                            pitch,
                            on_ground,
                            horizontal_collision,
                        } => session.move_player_rot(yaw, pitch, on_ground, horizontal_collision),
                        crate::world_view::ContainerCommand::MoveStatus {
                            on_ground,
                            horizontal_collision,
                        } => session.move_player_status_only(on_ground, horizontal_collision),
                        crate::world_view::ContainerCommand::DigAction {
                            action,
                            pos,
                            face,
                        } => {
                            // THE DIG ARC's wire echo (the vanilla gameMode's
                            // destroy half): the same monotonic prediction
                            // counter the use arc speaks
                            use_sequence += 1;
                            session.player_action(action, pos, face, use_sequence)
                        }
                        crate::world_view::ContainerCommand::Swing => session.swing(0),
                        crate::world_view::ContainerCommand::Attack { entity_id } => {
                            // THE WIRE BODY'S OWN ATTACK (the vanilla
                            // gameMode.attack's send — the 26.2's
                            // dedicated Attack packet the record's ONE
                            // VarInt carries; the server's own id the
                            // wire lane keeps)
                            session.attack(entity_id)
                        }
                        crate::world_view::ContainerCommand::Respawn => {
                            // the death screen's own wire: the Client
                            // Command's PERFORM_RESPAWN action 0
                            session.client_command(0)
                        }
                    };
                    if let Err(e) = sent {
                        // a send failure is a transport error: a framed
                        // stream cannot resync — report and end (the
                        // pump's own error path)
                        let _ = live_tx.send(Err(format!("container send: {e:#}")));
                        return;
                    }
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
                            // the mobs' hurt + voice mailboxes ship with
                            // this view — the drains empty them (each
                            // arm rides exactly one refresh, the mob
                            // lane's own tick/audio edge owns the
                            // countdown and the playback after the
                            // landing; a fresh packet re-arms)
                            view.model.drain_entity_hurts();
                            view.model.drain_entity_sounds();
                            if live_tx
                                .send(Ok((
                                    upd.world,
                                    upd.camera,
                                    upd.biome,
                                    // the live window rides every shipped
                                    // refresh — the chest's slots stream to
                                    // the screen exactly like the terrain
                                    upd.container,
                                    // the held stack rides too — the
                                    // first-person hand follows the store's
                                    // selected hotbar slot exactly like the
                                    // chest follows the container
                                    upd.held,
                                    // the HUD's whole row rides with it —
                                    // the hotbar's icons and counts are the
                                    // store's projection, not the screen's
                                    upd.hotbar,
                                    // the vitals ride with the row — the
                                    // hearts follow the store exactly like
                                    // the hotbar's icons do
                                    upd.health,
                                    // the wire's own population rides with
                                    // the vitals — the mobs the server
                                    // spawned stream to the mob lane
                                    upd.mobs,
                                    // the mobs' voices ride with the
                                    // population — the sounds the server
                                    // played stream to the voice lane
                                    upd.voices,
                                )))
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
/// `held_item` seeds the OFFLINE held stack (the `--held-item` model id's
/// own route into the interaction domain — the hand renders the stack and
/// the press lanes speak it; the offline singleplayer's own behavior).
pub fn run_gui_app(seed: u64, width: usize, height: usize, held_item: Option<&str>) -> anyhow::Result<()> {
    use crate::gui::GuiAssets;
    use crate::screens::App;

    let mut window = Window::new(
        &format!("mc2rs — Minecraft {} (generated GUI)", crate::build_info::MINECRAFT_VERSION),
        width,
        height,
        WindowOptions { resize: true, ..WindowOptions::default() },
    )?;
    let mut app = App::new(seed);
    app.offline_held_updated(held_item);
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
    let mut was_right_down = false; // the right press's edge detector (the split-stack lane)
    // THE FIRST-PERSON BODY's per-frame state: the previous raw mouse
    // position (the delta look's own stream — None = re-arm without a
    // turn) and the world-screen edge detector (the cursor hides
    // in-world like the vanilla pointer-lock, shows on every menu)
    let mut prev_mouse: Option<(f32, f32)> = None;
    let mut was_in_world = false;
    // THE FIXED CLOCK's own state (vanilla `Minecraft.runTick`): the
    // wall time accrues between frames, spends in whole 50ms game ticks
    // (the 10-tick catch-up cap), the remainder carries AND rides the
    // frame's render as the partial tick the interpolation folds read
    let mut tick_acc_ms = 0.0f64;
    let mut t_clock = std::time::Instant::now();
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
                Option<crate::world_view::LiveContainer>,
                Option<(i32, i32)>,
                [(i32, i32); 9],
                crate::world_view::HealthView,
                Vec<crate::world_view::WireMob>,
                Vec<crate::world_view::WireVoice>,
            ),
            String,
        >,
    >();

    while window.is_open() {
        if window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            app.on_escape();
        }
        // the number keys' hotbar selection (the vanilla keyPress's own
        // lane): 1..9 move the selection, the live session's belief
        // rides the command channel (the worker echoes the wire AND
        // moves the store — the hand follows the keys)
        for (i, key) in [
            Key::Key1,
            Key::Key2,
            Key::Key3,
            Key::Key4,
            Key::Key5,
            Key::Key6,
            Key::Key7,
            Key::Key8,
            Key::Key9,
        ]
        .iter()
        .enumerate()
        {
            if window.is_key_pressed(*key, minifb::KeyRepeat::No) {
                app.on_number_key(i);
            }
        }
        let (mx, my) = window.get_mouse_pos(minifb::MouseMode::Discard).unwrap_or((0.0, 0.0));
        let gmx = (mx as usize / 2) as i32;
        let gmy = (my as usize / 2) as i32;
        // the live mouse stream: the carried stack's cursor-follow
        // render reads the last known GUI-space position (Discard mode
        // reports None when the cursor leaves — the follow goes with it;
        // the headless scenes never run this loop, the goldens never
        // see the pass)
        if let Some((px, py)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            app.on_mouse_move((px as usize / 2) as i32, (py as usize / 2) as i32);
        }
        // the keyboard lane's shift state (the vanilla hasShiftDown()
        // query's own shape): the press's rising edge routes the
        // shift-click's QUICK_MOVE lane through it
        app.on_shift_down(window.is_key_down(Key::LeftShift));
        let left_down = window.get_mouse_down(minifb::MouseButton::Left);
        if left_down {
            app.on_click(gmx, gmy);
        }
        // the LEFT press's rising edge routes the press WITH its
        // position (the vanilla MouseHandler's own dispatch): the chest
        // screen's press clicks the slot it landed on (the live
        // prediction + the wire echo), every other screen falls through
        // to the attack swing inside — minifb has no edge API, the
        // previous frame's state is tracked here
        if left_down && !was_left_down {
            app.on_press(gmx, gmy);
        }
        // the LEFT press's falling edge (the vanilla's own release lane):
        // the dig's hold flag drops and the destroy state ABORTs (the
        // continueAttack(false) tail — the crack overlay clears)
        if !left_down && was_left_down {
            app.on_left_release();
        }
        was_left_down = left_down;
        // the RIGHT press's rising edge: the container screen's own
        // split-stack lane (button 1 — the pickup table's other branch);
        // the world's right press is the USE lane (the using items'
        // hold — the vanilla rightClickStart, the App's own guard keeps
        // the rest honest). The FALLING edge releases the use (the
        // vanilla rightClickStop — the shield lowers, the bow looses)
        let right_down = window.get_mouse_down(minifb::MouseButton::Right);
        if right_down && !was_right_down {
            app.on_right_press(gmx, gmy);
        }
        if !right_down && was_right_down {
            app.on_right_release();
        }
        was_right_down = right_down;
        // THE FIRST-PERSON BODY (the vanilla in-world input discipline):
        // the world screen hides the cursor (the pointer-lock stand-in —
        // minifb has no grab, the raw Pass stream stands in), the mouse
        // deltas turn the body through the turnPlayer curve, the WASD /
        // Space / Shift / Ctrl keys speak the movement impulses, and the
        // OFFLINE world spawns its body at the surface once the screen
        // stands (the networked body rides the teleport re-anchor). The
        // pause and every menu show the cursor and freeze the lanes —
        // the vanilla's own pause law.
        let in_world = app.screen == crate::screens::ScreenKind::World && !app.paused;
        if in_world != was_in_world {
            window.set_cursor_visibility(!in_world);
            prev_mouse = None; // re-arm the delta stream (no jump on entry)
            was_in_world = in_world;
        }
        if in_world && app.body.is_none() && !app.world_is_network {
            app.spawn_offline_body();
        }
        if in_world {
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
                if let Some((px, py)) = prev_mouse {
                    app.on_look_delta((x - px) as f64, (y - py) as f64);
                }
                prev_mouse = Some((x, y));
            } else {
                prev_mouse = None;
            }
            let fwd = (window.is_key_down(Key::W) as i32 - window.is_key_down(Key::S) as i32) as f32;
            let strafe =
                (window.is_key_down(Key::A) as i32 - window.is_key_down(Key::D) as i32) as f32;
            app.on_movement_input(
                fwd,
                strafe,
                window.is_key_down(Key::Space),
                window.is_key_down(Key::LeftShift),
                window.is_key_down(Key::LeftCtrl),
            );
            // THE F5 PERSPECTIVE (vanilla handleKeybinds's
            // keyTogglePerspective lane): the rising edge cycles the
            // three views — first person, third-person back, third-person
            // front (minifb has no edge API, the press is read per frame)
            if window.is_key_pressed(Key::F5, minifb::KeyRepeat::No) {
                app.on_perspective_key();
            }
        }
        // THE FIXED CLOCK SPENDS (vanilla `Minecraft.runTick`'s own
        // while loop): the accrued wall time becomes whole 50ms game
        // ticks (the catch-up cap holds a stalled machine — the game
        // logic runs 20 TPS like vanilla, never 3x fast with the render
        // rate), the sub-tick remainder rides THIS frame's render as
        // the partial tick (the body/mob/particle interpolation folds)
        let now = std::time::Instant::now();
        tick_acc_ms += (now - t_clock).as_secs_f64() * 1000.0;
        t_clock = now;
        let (ticks, rem) = crate::screens::ticks_due(tick_acc_ms);
        tick_acc_ms = rem;
        for _ in 0..ticks {
            app.update();
        }
        app.partial_tick = (rem / crate::screens::MILLIS_PER_TICK) as f32;
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
            // the container command lane: the frame thread's slot clicks
            // and the player's own close echo ride into the worker's
            // drain (the session's send path — see run_join)
            let (click_tx, click_rx) = std::sync::mpsc::channel::<
                crate::world_view::ContainerCommand,
            >();
            app.attach_live_clicks(click_tx);
            std::thread::spawn(move || run_join(id, host, port, tx, live_tx, click_rx, run));
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
                Ok((world, camera, biome, container, held, hotbar, health, mobs, voices)) => {
                    app.live_view_updated(world, camera, biome, container);
                    // the hand's lane beside the world's: the selected
                    // hotbar stack's model loads on change (the reload
                    // key makes the refresh cadence cheap)
                    app.live_held_updated(held);
                    // the HUD's lane beside the hand's: the whole row
                    // lands verbatim (the icons and the counts render
                    // over the hotbar sprite, the emptiness skipped)
                    app.live_hotbar_updated(hotbar);
                    // the hearts' lane beside the row's: the vitals
                    // land verbatim (the flash, the death message and
                    // all — the health row speaks the store)
                    app.live_health_updated(health);
                    // the mob lane beside the vitals': the wire's own
                    // population lands verbatim (the server's poses the
                    // lerpTo law walks — the zombies the server spawned
                    // render at its word)
                    app.live_mobs_updated(mobs);
                    // the voice lane beside the mob lane's: the sounds
                    // the SERVER played for its mobs land at the named
                    // entities (the entity lookup's own gate — the
                    // voice the id doesn't name drops)
                    app.live_voices_updated(voices);
                }
                Err(reason) => app.live_session_lost(reason),
            }
        }
        music.sync(&app.music);
        // UI one-shots (ui.button.click) ride the independent UI channel
        music.sync_ui(&app.ui_sounds);
        // the block-sound lane's own one-shots (the dig beat's hit, the
        // break, the place, the footsteps) ride the same channel with the
        // vanilla gain + rate fold applied
        music.sync_block_sounds(&app.sounds.events);
        fb.clear(crate::headless::SKY_TOP, crate::headless::SKY_BOTTOM);
        app.render(&mut fb, &mut assets);
        buf.iter_mut().zip(fb.to_argb()).for_each(|(o, c)| *o = c);
        window.update_with_buffer(&buf, width, height)?;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    Ok(())
}

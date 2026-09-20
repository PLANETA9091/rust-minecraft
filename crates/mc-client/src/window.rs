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

/// THE PING MONITOR'S OWN CADENCE (the vanilla client tick's own
/// period — ClientPacketListener.tick() runs every 50ms, and the
/// `showNetworkCharts()` gate's raised lane sends ONE
/// ServerboundPingRequestPacket per tick the PingDebugMonitor.tick
/// walk keeps). The net worker's loop the tick lane stands in for:
/// the request re-arms at this cadence while the F3+3 chart shows.
const PING_REQUEST_CADENCE: std::time::Duration = std::time::Duration::from_millis(50);

/// THE MONITOR'S OWN CLOCK (the Util.getMillis() walk — the unix
/// millis the sent-time long carries, the echo the RTT difference
/// answers).
fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

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
                Vec<crate::world_view::WireBoat>,
                Vec<crate::world_view::WireArrow>,
                Vec<crate::world_view::WireVoice>,
                Vec<crate::world_view::WireWorldVoice>,
                Vec<crate::world_view::WireArrowSync>,
                Option<crate::world_view::WireSidebar>,
                crate::world_view::WirePlayerList,
                Option<crate::world_view::WireRide>,
                [(i32, i32); crate::world_view::INVENTORY_MENU_SLOTS],
                Vec<(i32, Vec<(i32, i32)>)>,
                i32,
                (i32, i32),
                Vec<i64>,
                Vec<i64>,
                Vec<[i64; 4]>,
                Option<u8>,
            ),
            String,
        >,
    >,
    clicks: std::sync::mpsc::Receiver<crate::world_view::ContainerCommand>,
    chat_tx: std::sync::mpsc::Sender<String>,
    suggestion_tx: std::sync::mpsc::Sender<mc_protocol::play::CommandSuggestionsReply>,
    commands_tx: std::sync::mpsc::Sender<mc_protocol::play::CommandTree>,
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
            // the synced registry's own roster: the entity-type id table
            // the snapshot carries (the chat's type= value walk the App
            // answers from now on) — captured before the snapshot moves
            // into the live view
            let entity_types = snapshot.registry.entity_types.clone();
            // the synced registry's own entity-type TAG table: the tag
            // name -> member NAMES the arrival-order roster resolves
            // (the id the roster never named the honest drop keeps) —
            // the chat's '#'-suggestion arms the App answers from now
            // on; captured before the snapshot moves into the live view
            let entity_type_tags: Vec<(String, Vec<String>)> = snapshot
                .registry
                .entity_type_tags
                .iter()
                .map(|(name, ids)| {
                    (
                        name.clone(),
                        ids.iter()
                            .filter_map(|i| snapshot.registry.entity_types.get(*i as usize).cloned())
                            .collect(),
                    )
                })
                .collect();
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
            view.model.drain_world_sounds();
            view.model.drain_arrow_syncs();
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
                    // the registry's roster rides the milestone beside
                    // the world (the type= walk's own source)
                    entity_types,
                    // the tag table rides beside the roster (the
                    // '#'-suggestion arms' own source)
                    entity_type_tags,
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
            // THE PING MONITOR'S OWN STATE (the vanilla PingDebugMonitor's
            // own lane): the F3+3 flag the frame thread's toggle rides
            // (the command channel the gate speaks), the request clock
            // the 50ms tick cadence arms, and the RTT ring the pong lane
            // logs into (the LocalSampleLogger law — 240 one-column
            // samples, the oldest the full-ring write drops; the
            // snapshot the chart-ordered Vec carries, idx 0 the
            // oldest)
            let mut network_charts = false;
            let mut last_ping_request: Option<std::time::Instant> = None;
            let mut ping_ring: std::collections::VecDeque<i64> =
                std::collections::VecDeque::with_capacity(crate::screens::FPS_CHART_CAPACITY);
            // THE BANDWIDTH MONITOR'S OWN STATE (the vanilla
            // BandwidthDebugMonitor's own lane): the wire-byte
            // accumulation the frames' counter feeds, the tick clock
            // the 50ms cadence arms, and the ring the tick's drain
            // logs into (the LocalSampleLogger law — 240 one-column
            // samples, the oldest the full-ring write drops; the
            // snapshot the chart-ordered Vec carries). The tick the
            // CONNECTION tick's own unconditional lane keeps — NO
            // chart gate the ping request's raised lane wears (the
            // Connection.tick walk's tail the monitor tick answers)
            let mut bw_bytes: u64 = 0;
            let mut last_bw_tick: Option<std::time::Instant> = None;
            let mut bw_ring: std::collections::VecDeque<i64> =
                std::collections::VecDeque::with_capacity(crate::screens::FPS_CHART_CAPACITY);
            // THE DEBUG SUBSCRIBER'S OWN STATE (the vanilla
            // ClientDebugSubscriber's own lane): the F3+2 flag the
            // frame thread's toggle rides (the command channel the
            // requestedSubscriptions walk answers), the last-SENT set
            // the equality gate compares (the ctor's `Set.of()`
            // initial state the charts-off word keeps — the connect
            // speaks NO empty request the vanilla equality swallows),
            // and the tick-sample ring the debug-sample lane logs
            // into (the LocalSampleLogger(4) law — 240 four-dimension
            // rows, the TpsDebugDimensions slots each row carries)
            let mut fps_charts = false;
            let mut subs_sent = false;
            let mut tick_ring: std::collections::VecDeque<[i64; 4]> =
                std::collections::VecDeque::with_capacity(crate::screens::FPS_CHART_CAPACITY);
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
                        crate::world_view::ContainerCommand::SelectBundleItem {
                            slot_id,
                            selected_item_index,
                        } => {
                            // THE BUNDLE SELECTION's wire half (the vanilla
                            // toggleSelectedBundleItem's own echo — the
                            // local set the App's walk already spoke, the
                            // packet the tail sends): the hovered slot's
                            // MENU id and the selection the wrap landed,
                            // the two VarInts the record's write speaks.
                            session.select_bundle_item(slot_id, selected_item_index)
                        }
                        crate::world_view::ContainerCommand::PickBlock {
                            pos,
                            include_data,
                        } => {
                            // THE PICK's wire half (the vanilla
                            // gameMode.handlePickItemFromBlock walk): the
                            // packet send ALONE — NO local prediction the
                            // vanilla never predicts, the inventory law
                            // server-side (the findSlotMatchingItem scan,
                            // the pickSlot swap, the Set Held Slot echo +
                            // broadcastChanges the ALWAYS tail speaks).
                            // The verdict rides the clientbound echo back
                            // through its gate — the store the server's
                            // to move.
                            session.pick_item_from_block(pos, include_data)
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
                        crate::world_view::ContainerCommand::Interact {
                            entity_id,
                            cursor,
                            shift,
                        } => {
                            // THE WIRE BODY'S OWN INTERACT (the vanilla
                            // gameMode.interact's send — the flat
                            // record the composite order carries: the
                            // VarInt id, the VarInt hand the MAIN
                            // ordinal speaks, the LpVec3 cursor, the
                            // Bool shift)
                            session.interact(entity_id, 0, cursor, shift)
                        }
                        crate::world_view::ContainerCommand::Respawn => {
                            // the death screen's own wire: the Client
                            // Command's PERFORM_RESPAWN action 0
                            session.client_command(0)
                        }
                        crate::world_view::ContainerCommand::PlayerInput {
                            forward,
                            backward,
                            left,
                            right,
                            jump,
                            shift,
                            sprint,
                        } => {
                            // THE INPUT WIRE (the vanilla LocalPlayer.tick's
                            // own echo): the seven booleans' record the ONE
                            // byte rides — the riding lanes read it
                            // server-side (the boat's steering, the shift's
                            // dismount ask)
                            session.player_input(mc_protocol::InputFlags {
                                forward,
                                backward,
                                left,
                                right,
                                jump,
                                shift,
                                sprint,
                            })
                        }
                        crate::world_view::ContainerCommand::PaddleBoat { left, right } => {
                            // THE PADDLE WIRE (the vanilla AbstractBoat.tick's
                            // own send): the two booleans the controlBoat law
                            // derived — the generated-id-35 packet, the
                            // server stores them into the boat's entity data
                            session.paddle_boat(left, right)
                        }
                        crate::world_view::ContainerCommand::PlayerAbilities { flying } => {
                            // THE ABILITIES WIRE (the vanilla
                            // LocalPlayer.onUpdateAbilities's own echo): the
                            // flying state the changed tick banks — the ONE
                            // byte the write's `ior 2` law keeps
                            session.player_abilities(flying)
                        }
                        crate::world_view::ContainerCommand::ChangeGameMode { mode_id } => {
                            // THE GAME-MODE WIRE (the vanilla
                            // GameModeSwitcherScreen.switchToHoveredGameMode's
                            // own send — the F4 apply's serverbound half):
                            // the GameType id the VarInt codec carries, the
                            // mode the SERVER's belief owns (no local
                            // prediction the walk keeps)
                            session.change_game_mode(mode_id)
                        }
                        crate::world_view::ContainerCommand::NetworkCharts(on) => {
                            // THE NETWORK CHARTS' OWN GATE (the vanilla
                            // ClientPacketListener.tick's
                            // showNetworkCharts() lane): no packet rides
                            // HERE — the flag arms the monitor the loop's
                            // own cadence speaks (the ping request the
                            // raised lane sends at the vanilla tick pace;
                            // the lower disarms it)
                            network_charts = on;
                            if !on {
                                last_ping_request = None;
                            }
                            Ok(())
                        }
                        crate::world_view::ContainerCommand::FpsCharts(on) => {
                            // THE FPS CHARTS' OWN GATE (the vanilla
                            // ClientDebugSubscriber.tick's
                            // showFpsCharts() lane): no packet rides
                            // HERE — the flag arms the subscription the
                            // loop's own equality gate speaks (the
                            // request the set's change sends; the
                            // charts-off set the lowered flag answers)
                            fps_charts = on;
                            Ok(())
                        }
                        crate::world_view::ContainerCommand::SendChat { message } => {
                            // THE CHAT SEND (the vanilla
                            // ClientPacketListener.sendChat's own wire —
                            // the ServerboundChatPacket the unsigned
                            // session's walk speaks: the wall-clock
                            // millis the Instant.toEpochMilli lane
                            // carries, the zero salt / absent signature
                            // / empty last-seen the no-signed-session
                            // lane honestly rides). The server's own
                            // echo comes back through the pump's Player
                            // Chat arm — the display queue never sees
                            // the LOCAL copy.
                            session.send_chat(&message, now_millis())
                        }
                        crate::world_view::ContainerCommand::SendChatCommand { command } => {
                            // THE COMMAND SEND (the vanilla sendCommand's
                            // own wire — the 26.2 packet ONE bare utf,
                            // the signed-era fields dropped; the slash
                            // the submit's substring walk already
                            // stripped). The server's answer rides the
                            // system/player chat lanes back.
                            session.send_chat_command(&command)
                        }
                        crate::world_view::ContainerCommand::RequestCommandSuggestions {
                            id,
                            command,
                        } => {
                            // THE SUGGESTION REQUEST SEND (the vanilla
                            // ClientSuggestionProvider.customSuggestion's
                            // own wire — the ServerboundCommandSuggestionPacket
                            // the '/'-typing lane sends on every edit:
                            // the transaction id the pending gate
                            // remembers, the FULL input the leading '/'
                            // rides). The reply rides the
                            // CommandSuggestions event back through the
                            // pump into the suggestion channel.
                            session.send_command_suggestion(id, &command)
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
                // THE DEBUG SUBSCRIBER'S OWN TICK (the vanilla
                // ClientDebugSubscriber.tick's equality gate — the
                // requested set the showFpsCharts flag answers, the
                // request ONLY the set's change sends: {DEDICATED_
                // SERVER_TICK_TIME} the raised lane speaks, the empty
                // set the lowered one)
                if fps_charts != subs_sent {
                    let sent = session.send_debug_subscription(fps_charts);
                    if let Err(e) = sent {
                        let _ = live_tx.send(Err(format!("debug subscription send: {e:#}")));
                        return;
                    }
                    subs_sent = fps_charts;
                }
                // THE PING MONITOR'S OWN TICK (the vanilla
                // ClientPacketListener.tick's showNetworkCharts gate —
                // the raised lane sends ONE ServerboundPingRequestPacket
                // per client tick the PingDebugMonitor.tick walk keeps:
                // the sent-time long the Util.getMillis() instant
                // carries, the echo the RTT sample answers). The loop's
                // own cadence the tick lane stands in for; a quiet
                // server's blocking read the request waits behind (the
                // honest silence the wire keeps)
                if network_charts
                    && last_ping_request
                        .map_or(true, |t| t.elapsed() >= PING_REQUEST_CADENCE)
                {
                    let sent = session.send_ping_request(now_millis());
                    if let Err(e) = sent {
                        let _ = live_tx.send(Err(format!("ping request send: {e:#}")));
                        return;
                    }
                    last_ping_request = Some(std::time::Instant::now());
                }
                match session.pump() {
                    Ok(mc_protocol::PlayEvent::Disconnected { reason }) => {
                        // the server ended the session: report it (the App
                        // drops the report when it already left the world)
                        let _ = live_tx.send(Err(reason));
                        break;
                    }
                    Ok(event) => {
                        // THE BANDWIDTH MONITOR'S OWN DRAIN (the vanilla
                        // Connection.tick's unconditional walk — the
                        // frames' wire-byte counter the monitor's
                        // getAndAdd accumulates, the tick's getAndSet(0)
                        // the sample speaks; the drain EVERY event the
                        // loop's own wake keeps, the tick the 50ms
                        // cadence gates)
                        bw_bytes += session.take_received_bytes();
                        let bw_changed =
                            if last_bw_tick.map_or(true, |t| t.elapsed() >= PING_REQUEST_CADENCE)
                            {
                                let sample = bw_bytes.min(i64::MAX as u64) as i64;
                                if bw_ring.len() >= crate::screens::FPS_CHART_CAPACITY {
                                    bw_ring.pop_front();
                                }
                                bw_ring.push_back(sample);
                                bw_bytes = 0;
                                last_bw_tick = Some(std::time::Instant::now());
                                true
                            } else {
                                false
                            };
                        // THE PONG'S OWN LOG (the PingDebugMonitor
                        // .onPongReceived's own walk — `Util.getMillis() -
                        // packet.time()` the difference the
                        // LocalSampleLogger.logSample stores: the RTT
                        // millis the chart's columns speak; the ring the
                        // LocalSampleLogger law keeps — 240 samples, the
                        // oldest the full-ring write drops)
                        let ping_changed = if let mc_protocol::PlayEvent::PongResponse(t) = &event
                        {
                            let rtt = now_millis() - t;
                            if ping_ring.len() >= crate::screens::FPS_CHART_CAPACITY {
                                ping_ring.pop_front();
                            }
                            ping_ring.push_back(rtt);
                            true
                        } else {
                            false
                        };
                        // THE DEBUG SAMPLE'S OWN LOG (the vanilla
                        // DebugScreenOverlay.logRemoteSample's own walk —
                        // the ClientboundDebugSamplePacket the
                        // RemoteSampleLogger.useSample broadcast rides,
                        // the remoteSupportingLoggers map the TICK_TIME
                        // type's own logger answers: the FOUR
                        // TpsDebugDimensions slots the logFullSample
                        // row carries — the off-width sample the
                        // honest skip keeps, the 4-dim logger's own
                        // shape the wire's one type speaks)
                        let tick_changed = if let
                            mc_protocol::PlayEvent::DebugSample(ds) = &event
                        {
                            if ds.sample.len() == 4 {
                                if tick_ring.len() >= crate::screens::FPS_CHART_CAPACITY {
                                    tick_ring.pop_front();
                                }
                                tick_ring.push_back([
                                    ds.sample[0],
                                    ds.sample[1],
                                    ds.sample[2],
                                    ds.sample[3],
                                ]);
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        // THE CHAT WIRE'S OWN INTERCEPT (the vanilla
                        // handleSystemChat/handlePlayerChat's own lane —
                        // the display queue the addMessage walk answers:
                        // the SYSTEM line the component flattened speaks,
                        // the PLAYER line the bound decoration's format
                        // walk fills — the en_us table the
                        // `chat.type.text` `<%s> %s` law carries. The
                        // lines ride the DEDICATED channel — the chat
                        // promptness the view cadence never gates).
                        match &event {
                            mc_protocol::PlayEvent::SystemChat(m) => {
                                let _ = chat_tx.send(m.content.clone());
                            }
                            mc_protocol::PlayEvent::PlayerChat(m) => {
                                let _ = chat_tx
                                    .send(crate::screens::format_player_chat_line(m));
                            }
                            mc_protocol::PlayEvent::DisguisedChat(m) => {
                                // THE DISGUISED WALK (the /say, /me and
                                // the plugin broadcasts' own lane — the
                                // same decoration fill the player chat
                                // answers, the server's own bound the
                                // sources name)
                                let _ = chat_tx.send(crate::screens::format_decoration_line(
                                    &m.chat_key,
                                    &m.parameters,
                                    &m.sender_name,
                                    m.target_name.as_deref(),
                                    &m.content,
                                ));
                            }
                            mc_protocol::PlayEvent::CommandSuggestions(m) => {
                                // THE SUGGESTION REPLY'S OWN LANE (the
                                // vanilla ClientPacketListener
                                // .handleCommandSuggestions's walk — the
                                // completeCustomSuggestions completion the
                                // pending id the consumer's gate answers:
                                // the stale replies the dropped lane keeps,
                                // the fresh list the chat screen renders)
                                let _ = suggestion_tx.send(m.clone());
                            }
                            mc_protocol::PlayEvent::Commands(m) => {
                                // THE COMMANDS TREE'S OWN LANE (the vanilla
                                // handleCommands walk — the Brigadier graph
                                // the local dispatcher wears, the chat
                                // screen's LOCAL suggestions the literals
                                // answer). The re-send the replace walk
                                // answers (a /reload the server's own tree
                                // re-syncs).
                                let _ = commands_tx.send(m.clone());
                            }
                            _ => {}
                        }
                        if view.apply(event) || ping_changed || bw_changed || tick_changed {
                            // THE SHIP GATE (the cadence the world
                            // updates ride — a logged ping or bandwidth
                            // sample ALONE paces its refresh too, the
                            // charts' columns stream while the world
                            // sleeps; the snapshots ride EVERY ship, the
                            // beliefs the newest one keeps)
                            if last_ship.elapsed() >= VIEW_CADENCE {
                                let ping_snapshot: Vec<i64> = ping_ring.iter().copied().collect();
                                let bw_snapshot: Vec<i64> = bw_ring.iter().copied().collect();
                                let tick_snapshot: Vec<[i64; 4]> =
                                    tick_ring.iter().copied().collect();
                                let upd = view.refresh(crate::world_view::runtime_mapper);
                            // the mobs' hurt + voice mailboxes ship with
                            // this view — the drains empty them (each
                            // arm rides exactly one refresh, the mob
                            // lane's own tick/audio edge owns the
                            // countdown and the playback after the
                            // landing; a fresh packet re-arms)
                            view.model.drain_entity_hurts();
                            view.model.drain_entity_sounds();
                            view.model.drain_world_sounds();
                            view.model.drain_arrow_syncs();
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
                                    // the hulls ride with the population —
                                    // the boats the server spawned stream to
                                    // the boat lane
                                    upd.boats,
                                    // the shots ride with the population —
                                    // the arrows the server spawned stream to
                                    // the net-arrow lane (the spawns, the
                                    // despawns — the sim the client's own
                                    // tick owns between them)
                                    upd.arrows,
                                    // the mobs' voices ride with the
                                    // population — the sounds the server
                                    // played stream to the voice lane
                                    upd.voices,
                                    // the world's own voices ride with
                                    // them — the GLOBAL sounds the server
                                    // played at a world spot stream to
                                    // the positioned lane
                                    upd.world_voices,
                                    // the arrows' sync words ride with the
                                    // voices — the Entity Position Sync
                                    // corrections stream to the net-arrow
                                    // lane's re-pose law (each word rides
                                    // exactly one shipped refresh)
                                    upd.arrow_syncs,
                                    // the scoreboard's belief rides with
                                    // them — the sidebar the wire trio
                                    // keeps streams to the HUD's lane
                                    upd.sidebar,
                                    // the tab's belief rides with them —
                                    // the listed rows the player-info
                                    // store keeps stream to the TAB
                                    // overlay's lane
                                    upd.player_list,
                                    // the ride's anchor rides with them —
                                    // the carried position the vehicle's
                                    // own snapshot keeps streams to the
                                    // body's ride lane
                                    upd.ride,
                                    // the player's OWN store rides with
                                    // them — the inventory screen's live
                                    // lane follows the window-0 store
                                    // exactly like the chest's slots do
                                    upd.inventory_slots,
                                    // and the store's own bundle rows —
                                    // the scroll lane's feeder the patches
                                    // carry
                                    upd.inventory_bundles,
                                    // the store's revision + the GUI
                                    // cursor — the click lane's own echo
                                    // inputs the packets speak
                                    upd.inventory_state_id,
                                    upd.inventory_carried,
                                    // the ping chart's own snapshot rides
                                    // with them — the RTT ring the net
                                    // worker's pong lane logs (idx 0 the
                                    // oldest, the chart's columns the
                                    // newest belief re-draws)
                                    ping_snapshot,
                                    // and the bandwidth chart's own — the
                                    // bytes-per-tick ring the monitor's
                                    // drain logs (the log-scale bars the
                                    // wire's own traffic speaks)
                                    bw_snapshot,
                                    // and the TPS chart's own — the
                                    // four-dimension tick-sample ring the
                                    // debug-sample lane logs (the stacked
                                    // bars the wire's own tick speaks)
                                    tick_snapshot,
                                    // and the GAME-MODE ECHO'S OWN WORD —
                                    // the banked CHANGE_GAME_MODEL id the
                                    // game-event arm keeps (the server's
                                    // own answer the frame thread's
                                    // installer banks; None until the
                                    // echo speaks)
                                    upd.mode_word,
                                )))
                                .is_err()
                            {
                                break; // receiver gone at shutdown
                            }
                            last_ship = std::time::Instant::now();
                            }
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
    // the live chat lines: ONE shared channel the per-join workers clone
    // the sender of (the system/player chat lines the pump intercept
    // carries — the frame thread's drain lands each line into the App's
    // display queue, the same addMessage walk the local messages ride)
    let (chat_tx, chat_rx) = std::sync::mpsc::channel::<String>();
    // the command suggestion replies: ONE shared channel the per-join
    // workers clone the sender of (the pump's CommandSuggestions arm —
    // the frame thread's drain lands each reply into the App's pending
    // gate, the stale ids the gate drops)
    let (suggestion_tx, suggestion_rx) =
        std::sync::mpsc::channel::<mc_protocol::play::CommandSuggestionsReply>();
    // the commands tree: ONE shared channel the per-join workers clone
    // the sender of (the pump's Commands arm — the login-time Brigadier
    // graph the frame thread's drain hands the App's local suggestion
    // lane, the re-send the replace walk answers)
    let (commands_tx, commands_rx) =
        std::sync::mpsc::channel::<mc_protocol::play::CommandTree>();
    let mut net_seen = 0usize; // cursor into the append-only request log
    let mut was_left_down = false; // the attack edge detector (vanilla left-click = swing)
    let mut was_right_down = false; // the right press's edge detector (the split-stack lane)
    let mut was_middle_down = false; // the middle press's edge detector (the pick lane)
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
    // THE FRAME-TIME SAMPLE'S OWN CLOCK (the vanilla runTick's
    // logFrameDuration ride — the inter-frame delta the previous
    // frame's close keeps, the nanosecond duration the LocalSampleLogger
    // ring appends, the FPS chart's own column data)
    let mut prev_frame_instant: Option<std::time::Instant> = None;
    // THE F3 MODIFIER'S OWN DOWN STATE (the KeyboardHandler.keyPress's
    // release lane — the bare release the overlay toggle answers, the
    // chord-spent release the usedDebugKeyAsModifier flag swallows;
    // minifb has no key event, the edge the down-state pair tracks)
    let mut f3_was_down = false;
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
                Vec<crate::world_view::WireBoat>,
                Vec<crate::world_view::WireArrow>,
                Vec<crate::world_view::WireVoice>,
                Vec<crate::world_view::WireWorldVoice>,
                Vec<crate::world_view::WireArrowSync>,
                Option<crate::world_view::WireSidebar>,
                crate::world_view::WirePlayerList,
                Option<crate::world_view::WireRide>,
                [(i32, i32); crate::world_view::INVENTORY_MENU_SLOTS],
                Vec<(i32, Vec<(i32, i32)>)>,
                i32,
                (i32, i32),
                Vec<i64>,
                Vec<i64>,
                Vec<[i64; 4]>,
                Option<u8>,
            ),
            String,
        >,
    >();

    // THE CHAT CHAR SINK (the minifb input callback's own lane — the
    // set_input_callback trait the per-char push feeds, the shared
    // queue the frame walk drains; minifb has no polling char API,
    // the Rc<RefCell> hand-off the callback's own ownership keeps)
    let chat_chars: std::rc::Rc<std::cell::RefCell<Vec<char>>> = Default::default();
    struct ChatCharSink(std::rc::Rc<std::cell::RefCell<Vec<char>>>);
    impl minifb::InputCallback for ChatCharSink {
        fn add_char(&mut self, uni_char: u32) {
            if let Some(c) = char::from_u32(uni_char) {
                self.0.borrow_mut().push(c);
            }
        }
    }
    window.set_input_callback(Box::new(ChatCharSink(chat_chars.clone())));

    while window.is_open() {
        // THE FRAME-TIME SAMPLE (the vanilla runTick's own ride — the
        // inter-frame delta the LocalSampleLogger ring keeps, one
        // sample per frame the chart's columns speak; the first
        // frame the None re-arm skips)
        let t_frame = std::time::Instant::now();
        if let Some(prev) = prev_frame_instant {
            app.log_frame_duration((t_frame - prev).as_nanos() as i64);
        }
        prev_frame_instant = Some(t_frame);
        // THE CHAT CHAR DRAIN (the ChatScreen's own typing lane — the
        // chars the open screen's EditBox answers, the closed screen
        // the on_chat_char gate keeps silent)
        for c in chat_chars.borrow_mut().drain(..) {
            app.on_chat_char(c);
        }
        if window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            // THE ESCAPE'S OWN ROUTE (the ChatScreen's onClose law —
            // the open chat the draft-discard close answers, the
            // world the pause lane keeps)
            if app.chat_open {
                app.close_chat();
            } else {
                app.on_escape();
            }
        }
        // THE E KEY'S OWN LANE (the vanilla keyPress's key.inventory
        // toggle): the world screen opens the survival inventory, the
        // inventory screen closes it back — the App's own death gate
        // keeps the DeathScreen modal. The open chat the vanilla
        // Screen's own key-consumption law keeps silent (the
        // ChatScreen the world keys never reads)
        if !app.chat_open && window.is_key_pressed(Key::E, minifb::KeyRepeat::No) {
            app.on_inventory_key();
        }
        // THE INVENTORY'S OWN Q (the AbstractContainerScreen.keyPressed's
        // keyDrop arm — the container screen's own chain beside the
        // world's handleKeybinds lane below): the hovered slot's stack
        // the Q throws, the Ctrl modifier the whole stack (the button
        // int the hasControlDown the keyPressed bytecode 124-136 rides).
        // The world screen's own Q rides the in_world lane — the App's
        // screen gates keep the two chains honest (the world Q the
        // inventory walk never passes, the container Q the handleKeybinds
        // gate never did)
        if !app.chat_open && window.is_key_pressed(Key::Q, minifb::KeyRepeat::No) {
            app.on_inventory_drop_key(window.is_key_down(Key::LeftCtrl));
        }
        // the number keys' hotbar selection (the vanilla keyPress's own
        // lane): 1..9 move the selection, the live session's belief
        // rides the command channel (the worker echoes the wire AND
        // moves the store — the hand follows the keys)
        if !app.chat_open {
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
        // the keyboard lane's TAB state (the vanilla extractTabList's
        // keyPlayerList.isDown() gate's own shape): the HELD state the
        // overlay's visibility reads — the list shows while the key
        // sleeps on the keyboard, hides the frame the key lifts
        app.on_tab_key(window.is_key_down(Key::Tab));
        let left_down = window.get_mouse_down(minifb::MouseButton::Left);
        // THE CHAT SCREEN'S OWN MOUSE GATE (the vanilla Screen the world
        // clicks never pass — the open chat the attack/use lanes keep
        // silent, the edge-state tracking the close's honesty keeps)
        if left_down && !app.chat_open {
            app.on_click(gmx, gmy);
        }
        // the LEFT press's rising edge routes the press WITH its
        // position (the vanilla MouseHandler's own dispatch): the chest
        // screen's press clicks the slot it landed on (the live
        // prediction + the wire echo), every other screen falls through
        // to the attack swing inside — minifb has no edge API, the
        // previous frame's state is tracked here
        if left_down && !was_left_down && !app.chat_open {
            app.on_press(gmx, gmy);
        }
        // THE CHAT SUGGESTIONS' OWN CLICK (the SuggestionsList
        // .mouseClicked walk — the open chat's rising edge the list's
        // contains gate answers: the contained row the apply speaks,
        // the outside the false keeps; the open chat's other lanes the
        // world gates above keep silent)
        if left_down && !was_left_down && app.chat_open {
            app.on_chat_click(gmx, gmy);
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
        if right_down && !was_right_down && !app.chat_open {
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
        // the vanilla's own pause law. The open chat the same freeze
        // wears (the ChatScreen a Screen the in-world keys never read,
        // the cursor the typing lane shows)
        let in_world =
            app.screen == crate::screens::ScreenKind::World && !app.paused && !app.chat_open;
        if in_world != was_in_world {
            window.set_cursor_visibility(!in_world);
            prev_mouse = None; // re-arm the delta stream (no jump on entry)
            was_in_world = in_world;
        }
        // THE CHAT KEYS' OWN LANE (the ChatScreen.keyPressed walk — the
        // open screen the Enter/Backspace/Arrows answers, the
        // isConfirmation the submit fires, the history the Up/Down
        // recall walks; the T key the CLOSED world answers — the
        // Gui.handleKeybinds keyChat lane the screen-open walk never
        // reads)
        if app.chat_open {
            if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
                app.on_chat_submit();
            }
            if window.is_key_pressed(Key::Backspace, minifb::KeyRepeat::No) {
                app.on_chat_backspace();
            }
            // THE ARROWS' OWN BRANCH (the vanilla keyPressed walk — the
            // shown suggestions the Up/Down cycle answers FIRST
            // (SuggestionsList.keyPressed's own arm), the hidden list
            // the history arrows recall): the App's one arrow walk the
            // two laws fold
            if window.is_key_pressed(Key::Up, minifb::KeyRepeat::No) {
                app.on_chat_arrow(true);
            }
            if window.is_key_pressed(Key::Down, minifb::KeyRepeat::No) {
                app.on_chat_arrow(false);
            }
            // THE TAB'S OWN ACCEPT (the vanilla isCycleFocus arm — the
            // shown list the first Tab applies the selected row to, the
            // tabCycles flag the next Tab's cycle speaks, the cycle the
            // useSuggestion/cycle walk keeps)
            if window.is_key_pressed(Key::Tab, minifb::KeyRepeat::No) {
                app.on_chat_tab_key();
            }
        } else if app.screen == crate::screens::ScreenKind::World
            && !app.paused
            && window.is_key_pressed(Key::T, minifb::KeyRepeat::No)
        {
            app.on_chat_key();
        }
        if in_world && app.body.is_none() && !app.world_is_network {
            app.spawn_offline_body();
        }
        // THE DEBUG MODIFIER'S OWN STATE (the keyDebugModifier held
        // state the chord walk reads — the binding hoisted OUT of the
        // in-world block: the release walk the open screen's own
        // keyReleased law serves must run while the SWITCHER is up, the
        // same law the F3+F4 open's screen-eats-input shape keeps)
        let f3_down = window.is_key_down(Key::F3);
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
            // THE Q DROP KEY (vanilla handleKeybinds's keyDrop lane —
            // the keybinds the overlay==null && screen==null gate reads,
            // the in_world shape keeps it): the rising edge throws from
            // the selected hotbar slot, the Ctrl modifier the whole
            // stack (hasControlDown the LocalPlayer.drop boolean speaks
            // — the same key the sprint gate reads, the vanilla's own
            // dual use)
            if window.is_key_pressed(Key::Q, minifb::KeyRepeat::No) {
                app.on_drop_key(window.is_key_down(Key::LeftCtrl));
            }
            // THE F1 HIDE-HUD TOGGLE (vanilla Gui.handleKeybinds's
            // keyToggleGui lane — the `Options.keyToggleGui
            // .consumeClick()` rising edge the `Hud.toggle()` flip
            // answers): the whole HUD layer the flag hides, the world
            // the bare frame the vanilla F1 look keeps. The in_world
            // gate the screen-null keybind law keeps (the menus and
            // the modal screens the vanilla lane never reads, the
            // flag the later frames still honor).
            if window.is_key_pressed(Key::F1, minifb::KeyRepeat::No) {
                app.on_toggle_gui_key();
            }
            // THE F2 SCREENSHOT (vanilla Minecraft.handleKeybinds's
            // keyScreenshot lane — the `Options.keyScreenshot
            // .consumeClick()` rising edge the Screenshot.grab walk
            // answers): the request the flag raises, the render site
            // below the grab answers (the fresh frame the PNG writes,
            // the chat message the consumer tail keeps). minifb has
            // no edge API, the press is read per frame.
            if window.is_key_pressed(Key::F2, minifb::KeyRepeat::No) {
                app.on_screenshot_key();
            }
            // THE F5 PERSPECTIVE (vanilla handleKeybinds's
            // keyTogglePerspective lane): the rising edge cycles the
            // three views — first person, third-person back, third-person
            // front (minifb has no edge API, the press is read per frame)
            if window.is_key_pressed(Key::F5, minifb::KeyRepeat::No) {
                app.on_perspective_key();
            }
            // THE F3+2 CHORD (the handleDebugKeys' keyDebugFpsCharts
            // lane — the toggle the getDebugOverlay().toggleFpsCharts()
            // walk answers); the F3+3 chord (the keyDebugNetworkCharts
            // lane — the toggleNetworkCharts walk the getDebugOverlay()
            // answers) the same gate reads: the raise arms the net
            // worker's ping monitor (the command the flag rides), the
            // ping chart the client's own RTT ring draws. The
            // F3+1/4 chords the chart-less client keeps silent (no
            // profiler pie, no lightmap viewer — the flags the
            // all-false constants stay, the rows the honest hidden
            // words wear)
            if f3_down && window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
                app.on_fps_charts_key();
            }
            if f3_down && window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
                app.on_network_charts_key();
            }
            // THE F3+G CHUNK BORDERS (the debug-entry toggle the audited
            // refreshRendererList gate serves — the enabled entry the
            // renderer list keeps independent of the overlay's own
            // visibility, the borders the world draw persists)
            if f3_down && window.is_key_pressed(Key::G, minifb::KeyRepeat::No) {
                app.on_chunk_borders_key();
            }
            // THE F3+F4 CHORD (the handleDebugKeys' keyDebugSwitchGameMode
            // lane — the `matches` arm the level/screen/canSwitchGameMode
            // chain guards opens the GameModeSwitcherScreen, the default
            // hover the getDefaultSelected law answers)
            if f3_down && window.is_key_pressed(Key::F4, minifb::KeyRepeat::No) {
                app.on_open_game_mode_switcher();
            }
            // THE MIDDLE-CLICK PICK (vanilla handleKeybinds's keyPickItem
            // lane — the middle button the keybind maps, the
            // `Minecraft.pickBlockOrEntity` walk the loop answers): the
            // rising edge picks the crosshair's block into the inventory
            // (the Ctrl modifier rides the packet's includeData slot —
            // the same key the sprint/Q gates read, the vanilla's own
            // dual use). minifb has no edge API, the press is tracked
            // like the left/right pair above
            let middle_down = window.get_mouse_down(minifb::MouseButton::Middle);
            if middle_down && !was_middle_down {
                // THE SPECTATOR'S ACTION KEY (javap Minecraft
                // .handleKeybinds pcs 617-653 — the keySpectatorHotbar
                // loop the `player.isSpectator()` verdict answers: the
                // key the Options <init> bytecode 3208-3226 binds MOUSE
                // button 2 — the MIDDLE button the keyPickItem maps
                // TOO —, the `spectatorGui.onHotbarActionKeyPressed()`
                // the loop speaks; the pick lane the spectator verdict
                // keeps silent — the handlePickItemFromBlock walk the
                // packet alone carries, the server the spectator's
                // refusal answers, the offline prediction the
                // spectator's own silence wears)
                if app.game_mode == crate::screens::GameMode::Spectator {
                    app.spectator_on_action_key();
                } else {
                    app.on_pick_block(window.is_key_down(Key::LeftCtrl));
                }
            }
            was_middle_down = middle_down;
        }
        // THE DEBUG MODIFIER'S OWN RELEASE (the keyDebugModifier
        // keyReleased law — hoisted out of the in-world block: the open
        // SWITCHER the walk eats first (the apply+close the screen's own
        // keyReleased arm serves), the in-world release the
        // overlay-toggle flag walk answers, and the OTHER screens the
        // vanilla Screen.keyReleased law keeps silent — the tracking the
        // stale flag the next in-world frame re-reads honestly)
        if f3_was_down && !f3_down
            && (in_world || app.screen == crate::screens::ScreenKind::GameModeSwitcher)
        {
            app.on_debug_modifier_released();
        }
        // THE SWITCHER'S OWN F4 CYCLE (the open screen's keyPressed walk
        // — the keyDebugSwitchGameMode match the getNext law cycles, the
        // open chord's rising edge the closed state alone consumes: the
        // two lanes the exclusive screen states never double-fire)
        if app.screen == crate::screens::ScreenKind::GameModeSwitcher
            && window.is_key_pressed(Key::F4, minifb::KeyRepeat::No)
        {
            app.on_game_mode_switcher_cycle();
        }
        f3_was_down = f3_down;
        // THE SCROLL WHEEL (vanilla `MouseHandler.onScroll`'s own law —
        // the event the frame thread answers EVERY frame, the SCREEN
        // arm the bytecode 121-195 keeps first): a CONTAINER screen's
        // wheel folds into the screen's own mouseScrolled walk
        // ([`crate::screens::App::on_container_scroll`] — the
        // hovered-slot + itemSlotMouseActions chain), the OTHER screens
        // (the pause, the menus) the dispatch's `pop` silences. The
        // SCREEN-NULL arm (the hotbar cycle — the MouseHandler's own
        // ScrollWheelHandler fold, bytecode 198+) only the in-world
        // screen rides: the accumulator the pause screen never
        // advances. The poll consumes
        // the event HERE every frame — the dropped delta the x11
        // backend's per-frame clear kept silent, the explicit poll keeps
        // the law honest (the screen-open wheel never leaks a stale
        // hotbar step).
        if let Some((sdx, sdy)) = window.get_scroll_wheel() {
            if app.chat_open {
                // THE CHAT SUGGESTIONS' OWN SCROLL (the
                // SuggestionsList.mouseScrolled walk — the contained
                // rect the offset window folds, the outside the false
                // keeps; the open chat the hotbar/container arms never
                // reach — the in_world gate the chat-open exclude
                // already keeps)
                app.on_chat_scroll(sdx as f64, sdy as f64, gmx, gmy);
            } else if in_world {
                app.on_hotbar_scroll(sdx as f64, sdy as f64);
            } else if matches!(
                app.screen,
                crate::screens::ScreenKind::Chest | crate::screens::ScreenKind::Inventory
            ) {
                app.on_container_scroll(sdx as f64, sdy as f64, gmx, gmy);
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
            // the chat line lane: the worker's chat intercept (the
            // system/player chat lines the pump carries) ships each
            // display line through — the frame thread's drain lands
            // every line into the App's display queue (the fade the
            // same 200-tick law rides), the view cadence never gates
            // the promptness the chat lane keeps
            let chat_tx = chat_tx.clone();
            let suggestion_tx = suggestion_tx.clone();
            let commands_tx = commands_tx.clone();
            std::thread::spawn(move || {
                run_join(
                    id,
                    host,
                    port,
                    tx,
                    live_tx,
                    click_rx,
                    chat_tx,
                    suggestion_tx,
                    commands_tx,
                    run,
                )
            });
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
                Ok((
                    world,
                    camera,
                    biome,
                    container,
                    held,
                    hotbar,
                    health,
                    mobs,
                    boats,
                    arrows,
                    voices,
                    world_voices,
                    arrow_syncs,
                    sidebar,
                    player_list,
                    ride,
                    inv_slots,
                    inv_bundles,
                    inv_state_id,
                    inv_carried,
                    ping_samples,
                    bw_samples,
                    tick_samples,
                    mode_word,
                )) => {
                    app.live_view_updated(world, camera, biome, container);
                    // THE GAME-MODE ECHO'S OWN BANK (the audited
                    // setLocalMode walk the installer wears — the
                    // previous bank, the mode bank, the abilities
                    // re-grant; the unchanged word the idempotent acmp
                    // keeps silent)
                    app.live_game_mode_updated(mode_word);
                    // the ping chart's lane beside the world's: the RTT
                    // snapshot the net worker's ring keeps lands verbatim
                    // (the chart's columns the newest belief re-draws)
                    app.live_ping_updated(ping_samples);
                    // the bandwidth chart's lane beside it: the
                    // bytes-per-tick snapshot the monitor's drain logs
                    // lands verbatim (the log-scale bars the wire's own
                    // traffic speaks)
                    app.live_bandwidth_updated(bw_samples);
                    // the TPS chart's lane beside them: the
                    // four-dimension tick snapshot the debug-sample
                    // lane's ring keeps lands verbatim (the stacked
                    // bars the wire's own tick re-draws)
                    app.live_tick_samples_updated(tick_samples);
                    // the ride's lane beside the world's: the carried
                    // position the vehicle's own snapshot keeps (the
                    // mount snap + the seat the projection walked)
                    app.live_ride_updated(ride);
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
                    // the boat lane beside the mob lane's: the hulls the
                    // SERVER spawned land verbatim (the wood-textured
                    // models the submit render draws — the ride lane's
                    // own vehicle the camera sits in)
                    app.live_boats_updated(boats);
                    // the net-arrow lane beside the boat lane's: the
                    // spawns and the despawns the wire's own population
                    // speaks (the fresh id the spawn word builds, the
                    // missing id drops — the sim the client's own tick
                    // owns between the refreshes)
                    app.live_net_arrows_updated(arrows);
                    // the arrows' sync lane beside it: the Entity
                    // Position Sync corrections re-pose the sim shots
                    // at the server's word (the near band the transit
                    // keeps, the far band the snap answers)
                    app.live_arrow_syncs_updated(arrow_syncs);
                    // the voice lane beside the mob lane's: the sounds
                    // the SERVER played for its mobs land at the named
                    // entities (the entity lookup's own gate — the
                    // voice the id doesn't name drops)
                    app.live_voices_updated(voices);
                    // the world's own voice lane beside the mob lane's:
                    // the GLOBAL sounds the server played at a world spot
                    // land at their own positions (the ear's distance law
                    // the packet's own spot feeds — no entity lookup)
                    app.live_world_voices_updated(world_voices);
                    // the scoreboard's lane beside the voices': the
                    // SIDEBAR slot's belief lands verbatim (the rows the
                    // projection sorted and capped — the HUD renders it
                    // while the slot speaks, nothing while it doesn't)
                    app.live_sidebar_updated(sidebar);
                    // the tab's lane beside the sidebar's: the TAB
                    // overlay's projection lands verbatim (the listed
                    // rows the comparator sorted and capped — the
                    // held-key gate renders it, nothing while the key
                    // sleeps)
                    app.live_player_list_updated(player_list);
                    // the window-0 store's lane beside the chest's: the
                    // player's own inventory the screen's live store
                    // projection rides (the slots the wire spoke, the
                    // bundle rows the patches carry — the ownership
                    // gate the chest's live window keeps); the
                    // revision and the cursor ride beside them — the
                    // click lane's own echo inputs
                    app.live_inventory_updated(
                        inv_slots,
                        inv_bundles,
                        inv_state_id,
                        inv_carried,
                    );
                }
                Err(reason) => app.live_session_lost(reason),
            }
        }
        // THE CHAT LINE DRAIN (the worker's intercept the dedicated
        // channel ships — each line the display queue's own addMessage
        // walk answers: the (text, addedTime) pair the 200-tick fade
        // law keeps, the wrap the draw re-walks. The SERVER's own echo
        // of the player's own sends lands through this lane too — the
        // vanilla client displays the server's copy, never its own.)
        while let Ok(line) = chat_rx.try_recv() {
            app.push_chat_message(line);
        }
        // THE SUGGESTION REPLY DRAIN (the frame thread's own walk — the
        // pump's CommandSuggestions lane ships each reply here, the App's
        // pending gate the matching-id answers, the stale ids the gate
        // drops)
        while let Ok(reply) = suggestion_rx.try_recv() {
            app.on_command_suggestions(reply);
        }
        // THE COMMANDS TREE DRAIN (the pump's Commands lane ships the
        // login-time Brigadier graph here — the App's local suggestion
        // lane the literals answer from now on)
        while let Ok(tree) = commands_rx.try_recv() {
            app.on_command_tree(tree);
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
        // THE F2 GRAB'S OWN ANSWER (the Screenshot.grab walk the
        // request's flag answers — the "screenshots" dir the
        // timestamp name fills, the PNG the fresh frame's own bytes
        // write, the chat message the consumer tail keeps: the
        // success the name speaks, the failure the error carries)
        if app.screenshot_request {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);
            let dir = std::path::Path::new("screenshots");
            let _ = std::fs::create_dir_all(dir);
            let name = crate::screens::screenshot_file_name(now, &|n| {
                dir.join(n).exists()
            });
            let outcome = crate::png_io::write_rgb_png(
                &dir.join(&name),
                width,
                height,
                &fb.color,
            )
            .map(|_| name)
            .map_err(|e| e.to_string());
            app.take_screenshot(outcome);
        }
        buf.iter_mut().zip(fb.to_argb()).for_each(|(o, c)| *o = c);
        window.update_with_buffer(&buf, width, height)?;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    Ok(())
}

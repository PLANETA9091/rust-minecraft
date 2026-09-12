//! GUI screens: the boot chain (Mojang Studios loading -> title -> menus),
//! driven entirely by GENERATED data: every label comes from the official
//! language table, every sprite from the official sprite inventory, every
//! splash from texts/splashes.txt.

use crate::gui::{Gui, GuiAssets};
use crate::math::{Mat4, Vec3};
use crate::raster::RenderCtx;

use mc_registry::generated::lang::tr;
use mc_registry::generated::splashes::SPLASHES;

pub const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
pub const YELLOW: [f32; 3] = [1.0, 1.0, 0.333];
pub const GRAY: [f32; 3] = [0.667, 0.667, 0.667];
/// Mojang Studios loading screen background (vanilla constant).
pub const MOJANG_RED: [f32; 3] = [239.0 / 255.0, 50.0 / 255.0, 61.0 / 255.0];

/// One interactive button (vanilla 200x20 nine-slice widget).
#[derive(Debug, Clone)]
pub struct Button {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub label: String,
    pub active: bool,
    pub id: &'static str,
}

impl Button {
    pub fn new(id: &'static str, label: String, cx: i32, y: i32, w: i32) -> Self {
        Button { x: cx - w / 2, y, w, h: 20, label, active: true, id }
    }

    pub fn hovered(&self, mx: i32, my: i32) -> bool {
        self.active && mx >= self.x && mx < self.x + self.w && my >= self.y && my < self.y + self.h
    }

    pub fn render(&self, g: &mut Gui, a: &mut GuiAssets, mx: i32, my: i32) {
        let sprite = if !self.active {
            "widget/button_disabled"
        } else if self.hovered(mx, my) {
            "widget/button_highlighted"
        } else {
            "widget/button"
        };
        g.sprite_9slice(a, sprite, self.x, self.y, self.w, self.h);
        let color = if self.active { WHITE } else { [0.5, 0.5, 0.5] };
        g.text_centered(a, self.x + self.w / 2, self.y + self.h / 2 - 4, &self.label, color, true);
    }
}

/// Server list entry (name, MOTD with § codes, ping ms).
#[derive(Debug, Clone)]
pub struct ServerEntry {
    pub name: String,
    pub motd: String,
    pub ping_ms: i32,
    pub online: bool,
    pub version: String,
    /// Real address ("host" or "host:port", vanilla default port 25565)
    /// the client can actually ping; `None` marks placeholder rows with
    /// no reachable endpoint (they always render offline).
    pub host: Option<String>,
}

/// One requested server-list ping: which row, which address. Append-only
/// request log — the window backend drains new entries (by cursor), runs
/// the blocking `mc_protocol::ping` on a worker thread and hands the
/// outcome back through `App::ping_finished`.
#[derive(Debug, Clone, PartialEq)]
pub struct PingRequest {
    pub id: u64,
    /// Index into `App::servers` this request belongs to.
    pub entry: usize,
    pub host: String,
    pub port: u16,
}

/// The slice of a real status response the server list renders.
#[derive(Debug, Clone, PartialEq)]
pub struct PingInfo {
    pub motd: String,
    pub version: String,
    pub ping_ms: i32,
}

/// One requested offline-mode join: which row, which address. Append-only
/// request log — the window backend drains new entries (by cursor), runs
/// the blocking `mc_protocol::connect_and_join_world` (login, the
/// configuration phase, then the play state pumped to the world-load
/// milestone) on a worker thread and hands the outcome back through
/// `App::login_finished`.
#[derive(Debug, Clone, PartialEq)]
pub struct LoginRequest {
    pub id: u64,
    /// Index into `App::servers` this request belongs to.
    pub entry: usize,
    pub host: String,
    pub port: u16,
}

/// The slice of a join outcome the UI renders — screens stay
/// protocol-agnostic (window.rs maps `mc_protocol::WorldOutcome` over).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginResult {
    /// The full join arc completed and the world-load milestone was hit:
    /// world info + the first position teleport + at least one decoded
    /// chunk. Every field below is REAL network data from the server —
    /// no placeholder values.
    Loaded {
        /// The offline profile name the client joined with.
        username: String,
        /// The client player's entity id for this session (play Login).
        entity_id: i32,
        /// The server's view distance for this session (play Login).
        view_distance: i32,
        /// Chunk columns decoded by the world-load milestone.
        chunks: usize,
        /// The materialized network world: the snapshot's chunks projected
        /// into the renderer grid, the window centered on the player's
        /// chunk — the World screen's terrain until a newer join replaces
        /// it. Built by the join worker off the frame thread.
        world: crate::world::World,
    },
    /// The server refused or kicked (login disconnect, config kick or a
    /// play-state disconnect), reason flattened from chat.
    Disconnected { reason: String },
}

/// The vanilla client joins with its launcher session name; this build has
/// no session service, so the offline profile "mc2rs" is the documented
/// stand-in (the profile UUID is the real vanilla offline derivation).
pub const PLAYER_NAME: &str = "mc2rs";

/// Split a vanilla server address ("host" or "host:port") — the port
/// defaults to 25565 exactly like the original server list.
fn parse_address(addr: &str) -> (String, u16) {
    match addr.rsplit_once(':') {
        Some((h, p)) if !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()) => {
            (h.to_string(), p.parse().unwrap_or(25565))
        }
        _ => (addr.to_string(), 25565),
    }
}

impl ServerEntry {
    pub fn ping_icon(&self) -> &'static str {
        match self.ping_ms {
            _ if !self.online => "server_list/unreachable",
            0..=40 => "server_list/ping_5",
            41..=80 => "server_list/ping_4",
            81..=120 => "server_list/ping_3",
            121..=200 => "server_list/ping_2",
            _ => "server_list/ping_1",
        }
    }
}

/// Top-level screen state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenKind {
    Loading,
    Title,
    Servers,
    Options,
    /// In-world 3D view with the vanilla HUD (Singleplayer).
    World,
    /// Vanilla ConnectScreen shown while a join is in flight (the blocking
    /// login runs off the frame thread; Escape/Cancel returns to the list).
    Connect,
    /// Vanilla DisconnectedScreen: the join outcome (server-side login
    /// disconnect reason, transport error, or the offline profile line).
    Disconnect,
}

pub struct App {
    pub screen: ScreenKind,
    pub boot_progress: f32,
    pub boot_stage: usize,
    pub selected_server: usize,
    pub servers: Vec<ServerEntry>,
    pub buttons: Vec<Button>,
    pub splash_index: usize,
    pub seed: u64,
    pub frame: u64,
    pub music: crate::music::MusicScheduler,
    /// In-game state: pause menu open (world frozen underneath).
    pub paused: bool,
    /// Meshed world (built lazily on entering the world screen).
    pub world_quads: Option<Vec<crate::mesher::Quad>>,
    pub world_size: i32,
    /// Eye height (top solid block y at spawn column) for the world camera.
    pub spawn_y: i32,
    /// Where the Options screen returns to (Title from menus, World from pause).
    pub options_return: ScreenKind,
    /// Active hotbar slot (vanilla HUD selection).
    pub selected_hotbar: usize,
    /// Base world tick for the day-night cycle (0 = vanilla dawn; the frame
    /// counter advances time 1 tick per frame like the vanilla clock).
    pub day_time_base: u64,
    /// UI one-shot sound log ("ui.button.click @<frame>"), mirroring the
    /// vanilla rule that EVERY activated button plays the click; consumed by
    /// the audio backend (audio.rs) like the music event log.
    pub ui_sounds: Vec<String>,
    /// Server-list ping requests (append-only log, same protocol as
    /// ui_sounds/music events): the headless logic records WHAT to ping,
    /// the window backend performs the blocking network call off the
    /// frame thread and delivers results via `ping_finished`.
    pub net_requests: Vec<PingRequest>,
    /// Monotonic id for the next ping request (pairs requests with results).
    pub next_ping_id: u64,
    /// Offline-mode login requests (append-only log, same protocol as the
    /// pings): recorded by the "join" button, run off the frame thread by
    /// the window backend, outcomes delivered via `login_finished`.
    pub login_requests: Vec<LoginRequest>,
    /// Monotonic id for the next login request.
    pub next_login_id: u64,
    /// DisconnectedScreen text (vanilla `disconnect.lost` title + the
    /// flattened reason line; the Success path shows the factual outcome).
    pub disconnect_title: String,
    pub disconnect_reason: String,
    /// The network world from the last successful join (the materialized
    /// snapshot window), kept for re-meshing when the id table / vertical
    /// layout resolve; dormant after Save and Quit until a newer join.
    pub net_world: Option<crate::world::World>,
    /// True while the World screen shows the NETWORK world (the quad mesh
    /// came from a join snapshot, not the procedural singleplayer world).
    pub world_is_network: bool,
}

/// World size for the in-game screen (chunks per side — same as the
/// headless golden world so the pixel test stays deterministic).
pub const WORLD_CHUNKS: usize = 5;

const HOTBAR_W: i32 = 182;
const HOTBAR_H: i32 = 22;
const HEART: i32 = 9;
const HEART_SPACING: i32 = 8;
pub const HEALTH: i32 = 20;

const BOOT_STAGES: [&str; 5] = [
    "mc2rs.boot.registry",
    "mc2rs.boot.fonts",
    "mc2rs.boot.sprites",
    "mc2rs.boot.sounds",
    "mc2rs.boot.worldgen",
];

impl App {
    /// Deterministic boot: splash selection from the seed (vanilla picks a
    /// random line; here the RNG is seeded for reproducible rendering).
    pub fn new(seed: u64) -> Self {
        let splash_index = if SPLASHES.is_empty() {
            0
        } else {
            (splitmix(seed) % SPLASHES.len() as u64) as usize
        };
        let servers = vec![
            ServerEntry {
                name: "mc2rs sandbox".to_string(),
                motd: "§aA Minecraft 26.2 client, generated in Rust".to_string(),
                ping_ms: 0,
                online: false,
                version: "".to_string(),
                // placeholder row: nothing reachable to ping yet
                host: None,
            },
            ServerEntry {
                name: "hypixel.net".to_string(),
                motd: "§bHypixel Network §7- §b1.8-1.21".to_string(),
                ping_ms: 34,
                online: true,
                version: "Requires MC 1.8/1.21".to_string(),
                host: Some("hypixel.net".to_string()),
            },
        ];
        Self {
            screen: ScreenKind::Loading,
            boot_progress: 0.0,
            boot_stage: 0,
            selected_server: 0,
            servers,
            buttons: Vec::new(),
            splash_index,
            seed,
            frame: 0,
            music: crate::music::MusicScheduler::new(seed),
            paused: false,
            world_quads: None,
            world_size: 0,
            spawn_y: 0,
            options_return: ScreenKind::Title,
            selected_hotbar: 0,
            day_time_base: 0,
            ui_sounds: Vec::new(),
            net_requests: Vec::new(),
            next_ping_id: 0,
            login_requests: Vec::new(),
            next_login_id: 0,
            disconnect_title: String::new(),
            disconnect_reason: String::new(),
            net_world: None,
            world_is_network: false,
        }
    }

    /// Advance deterministic boot progress; transitions to title.
    pub fn update(&mut self) {
        self.frame += 1;
        match self.screen {
            ScreenKind::Loading => {
                self.boot_progress += 0.05;
                self.boot_stage = (self.boot_progress * BOOT_STAGES.len() as f32) as usize;
                if self.boot_progress >= 1.0 {
                    self.boot_progress = 1.0;
                    self.screen = ScreenKind::Title;
                    self.music.on_menu_open();
                }
            }
            // the scheduler serves the current situation everywhere: menu
            // music on screens, music.game in-world (on_world_enter /
            // on_menu_open performed the vanilla-style switch)
            _ => self.music.update(),
        }
    }

    pub fn open(&mut self, kind: ScreenKind) {
        self.screen = kind;
        self.buttons.clear();
    }

    /// Enter the singleplayer world (3D view + HUD), building the mesh once.
    /// Vanilla situational music: the menu track is interrupted at once and
    /// the SPAWN BIOME's official background-music situation takes over
    /// (the seeded pick stands in for the future climate-driven biome
    /// source; the music data itself is fully official).
    pub fn enter_world(&mut self) {
        // rebuild when the current mesh belongs to a network world (a
        // stale procedural mesh is also rebuilt — the guard keeps each
        // terrain source out of the other's screen)
        if self.world_is_network || self.world_quads.is_none() {
            let world = crate::world::World::new(WORLD_CHUNKS, self.seed);
            self.world_size = world.size_blocks();
            self.spawn_y = top_solid_y(&world, self.world_size / 2, self.world_size / 2);
            self.world_quads = Some(crate::mesher::mesh_world(&world, None));
            self.world_is_network = false;
        }
        self.paused = false;
        self.music.on_world_enter_biome(spawn_biome(self.seed));
        self.open(ScreenKind::World);
    }

    /// Enter the world screen with the NETWORK world — the vanilla
    /// ConnectScreen → world handoff after a successful join. The screen
    /// machinery (sky, celestials, fog, clouds, HUD, pause) is shared
    /// with singleplayer; only the terrain source differs: the mesh comes
    /// from the materialized join snapshot. Camera: the window is centered
    /// on the player's chunk and VERTICALLY ANCHORED on the player's feet
    /// (`world_view::snapshot_y_base` — min_y + player.y, clamped at the
    /// world floor), so the grid's center column stands over the surface
    /// the player actually occupies; the exact player-follow camera
    /// (position + yaw/pitch from the teleport) is future work. Music:
    /// the spawn biome is unknown until section biomes decode, so the
    /// unknown-key lookup falls back to the generic music.game situation
    /// (no seeded guess about a network world).
    pub fn enter_network_world(&mut self) {
        let (size, spawn_y, quads) = {
            let Some(world) = self.net_world.as_ref() else { return };
            let size = world.size_blocks();
            let c = size / 2;
            (
                size,
                top_solid_y(world, c, c),
                crate::mesher::mesh_world(world, None),
            )
        };
        self.world_size = size;
        self.spawn_y = spawn_y;
        self.world_quads = Some(quads);
        self.world_is_network = true;
        self.paused = false;
        self.music.on_world_enter_biome("");
        self.open(ScreenKind::World);
    }

    /// Escape: toggles the pause menu while in game (vanilla behavior);
    /// leaves the connecting/outcome screens back to the server list
    /// (in-flight logins keep running: a late success after Escape is
    /// dropped with the cancelled screen, a late disconnect still reports).
    pub fn on_escape(&mut self) {
        if self.screen == ScreenKind::World {
            self.paused = !self.paused;
            self.buttons.clear();
        } else if matches!(self.screen, ScreenKind::Connect | ScreenKind::Disconnect) {
            self.open(ScreenKind::Servers);
        }
    }

    pub fn on_click(&mut self, mx: i32, my: i32) {
        let hit: Vec<&Button> = self.buttons.iter().filter(|b| b.hovered(mx, my)).collect();
        for b in hit {
            match b.id {
                "singleplayer" => self.enter_world(),
                "realms" => {}
                "multiplayer" => self.open(ScreenKind::Servers),
                "options" => {
                    self.options_return = if self.paused { ScreenKind::World } else { ScreenKind::Title };
                    self.open(ScreenKind::Options);
                }
                "quit" => {}
                "back" => {
                    let ret = self.options_return;
                    self.open(ret);
                }
                "backtogame" => {
                    self.paused = false;
                    self.buttons.clear();
                }
                "saveandquit" => {
                    self.paused = false;
                    if self.world_is_network {
                        // vanilla Save and Quit unloads the level: the
                        // network mesh drops (a newer join rematerializes
                        // it; singleplayer rebuilds its own world)
                        self.world_quads = None;
                        self.world_is_network = false;
                    }
                    self.open(ScreenKind::Title);
                    self.music.on_menu_open();
                }
                // vanilla Refresh: re-ping every hosted server row; the
                // blocking network work happens off the frame thread
                "refresh" => self.refresh_servers(),
                // vanilla Join Server: hosted rows fire the real offline-mode
                // login (ConnectScreen flow); placeholder rows keep the
                // click a no-op — there is nothing reachable to join
                "join" => self.join_selected_server(),
                // vanilla ConnectScreen Cancel: abandon the in-flight join
                // (late results carry unknown ids and are ignored)
                "cancel" => self.open(ScreenKind::Servers),
                // vanilla DisconnectedScreen back button: parent is the
                // server list the join came from
                "backtoservers" => self.open(ScreenKind::Servers),
                _ => {}
            }
            // vanilla AbstractButton.playDownSound: EVERY activated button
            // plays the UI click (disabled buttons can't be hit — hovered()
            // rejects them), even when the action itself is a no-op here
            // (quit/realms still click exactly like the original menus).
            self.ui_sounds.push(format!("ui.button.click @{}", self.frame));
            self.buttons.clear();
            return;
        }
        if self.screen == ScreenKind::Servers {
            // click a server row to select it
            for (i, _) in self.servers.iter().enumerate() {
                let y = 32 + i as i32 * 28;
                if my >= y && my < y + 24 && mx >= 20 && mx < self.buttons_row_cx() * 2 - 20 {
                    self.selected_server = i;
                    break;
                }
            }
        }
    }

    /// Vanilla "Refresh": request a real server-list ping (status request)
    /// for every entry that has an address. Rows without one have nothing
    /// reachable to ask and stay untouched. The actual network round-trip
    /// runs off the frame thread in the window backend — the headless
    /// logic only records the request (deterministic, like vanilla's
    /// async pinger from the renderer's point of view).
    pub fn refresh_servers(&mut self) {
        for (i, s) in self.servers.iter().enumerate() {
            let Some(addr) = &s.host else { continue };
            let (host, port) = parse_address(addr);
            self.net_requests.push(PingRequest {
                id: self.next_ping_id,
                entry: i,
                host,
                port,
            });
            self.next_ping_id += 1;
        }
    }

    /// Feed one completed server-list ping back into the list (vanilla
    /// updates the row in place: MOTD, version string, latency bars; an
    /// unreachable server renders the vanilla "Failed to connect" offline
    /// row). Unknown or already-consumed ids are ignored — the request
    /// log itself stays append-only.
    pub fn ping_finished(&mut self, id: u64, result: Result<PingInfo, String>) {
        let Some(req) = self.net_requests.iter().find(|r| r.id == id).cloned() else {
            return;
        };
        let Some(entry) = self.servers.get_mut(req.entry) else { return };
        match result {
            Ok(info) => {
                entry.online = true;
                entry.motd = info.motd;
                entry.version = info.version;
                entry.ping_ms = info.ping_ms.max(0);
            }
            Err(_) => {
                // vanilla marks the row offline: red connect.failed line
                // + the unreachable icon (render_servers)
                entry.online = false;
                entry.ping_ms = 0;
            }
        }
    }

    /// Vanilla "Join Server" on the selected row: only entries with a real
    /// address can actually connect (placeholder rows keep the click a no-op
    /// — there is nothing reachable to join). Records the join request in
    /// the append-only log and opens the vanilla ConnectScreen; the window
    /// backend runs the blocking `mc_protocol::connect_and_join_world` (the
    /// full login + configuration arc, then the play state pumped to the
    /// world-load milestone) off the frame thread and delivers the outcome
    /// through `login_finished`.
    pub fn join_selected_server(&mut self) {
        let Some(addr) = self
            .servers
            .get(self.selected_server)
            .and_then(|s| s.host.clone())
        else {
            return;
        };
        let (host, port) = parse_address(&addr);
        self.login_requests.push(LoginRequest {
            id: self.next_login_id,
            entry: self.selected_server,
            host,
            port,
        });
        self.next_login_id += 1;
        self.open(ScreenKind::Connect);
    }

    /// Feed one finished join back into the UI (vanilla flow: a login
    /// disconnect, config kick or play-state kick lands on the
    /// DisconnectedScreen with the flattened reason; a Loaded outcome means
    /// the client genuinely holds a network world — and just like the
    /// original, the completed join puts you IN it: the world screen renders
    /// the materialized snapshot window. A join whose ConnectScreen was
    /// already escaped drops the late success, exactly like the original's
    /// cancelled flow). Unknown or already-consumed ids are ignored — the
    /// request log stays append-only.
    pub fn login_finished(&mut self, id: u64, outcome: Result<LoginResult, String>) {
        if !self.login_requests.iter().any(|r| r.id == id) {
            return;
        }
        match outcome {
            Ok(LoginResult::Loaded { world, .. }) => {
                if self.screen == ScreenKind::Connect {
                    self.net_world = Some(world);
                    self.enter_network_world();
                }
            }
            Ok(LoginResult::Disconnected { reason }) => {
                self.disconnect_title =
                    tr("disconnect.lost").unwrap_or("Connection lost").to_string();
                self.disconnect_reason = reason;
                self.open(ScreenKind::Disconnect);
            }
            Err(e) => {
                self.disconnect_title =
                    tr("disconnect.lost").unwrap_or("Connection lost").to_string();
                self.disconnect_reason = e;
                self.open(ScreenKind::Disconnect);
            }
        }
    }

    fn buttons_row_cx(&self) -> i32 {
        100 // gui center for 160x90 (scale 2 @ 320x180); screens recenter dynamically
    }

    /// Render the current screen.
    pub fn render(&mut self, fb: &mut crate::raster::Framebuf, a: &mut GuiAssets) {
        let mut g = Gui::new(fb, Gui::auto_scale(fb.w, fb.h));
        let (w, h) = (g.w(), g.h());
        self.buttons.clear();
        match self.screen {
            ScreenKind::Loading => self.render_loading(&mut g, a),
            ScreenKind::Title => self.render_title(&mut g, a, w, h),
            ScreenKind::Servers => self.render_servers(&mut g, a, w, h),
            ScreenKind::Options => self.render_options(&mut g, a, w, h),
            ScreenKind::Connect => self.render_connect(&mut g, a, w, h),
            ScreenKind::Disconnect => self.render_disconnect(&mut g, a, w, h),
            ScreenKind::World => {
                // 3D world first (full framebuffer resolution), then the
                // HUD and pause overlay on top in GUI space.
                self.render_world_3d(fb);
                let mut g = Gui::new(fb, Gui::auto_scale(fb.w, fb.h));
                self.render_hud(&mut g, a);
                if self.paused {
                    let (gw, gh) = (g.w(), g.h());
                    self.render_pause(&mut g, a, gw, gh);
                }
            }
        }
    }

    /// The in-world 3D view: deterministic first-person camera at the spawn
    /// column facing east (+X — the direction the official 26.x sun rises),
    /// official day-night sky (timeline data + celestial textures), textured
    /// quads from the generated atlas, fog in the official color — the exact
    /// same raster path the headless pixel tests exercise.
    fn render_world_3d(&mut self, fb: &mut crate::raster::Framebuf) {
        self.ensure_world();
        let n = self.world_size as f32;
        // Creative-flight camera: 12 blocks above the spawn surface clears
        // the canopy so the official sky (sun/moon/stars) stays in frame.
        let eye = Vec3::new(n / 2.0, self.spawn_y as f32 + 12.0 + 1.62, n / 2.0);
        let cam = crate::camera::Camera {
            pos: eye,
            yaw: std::f32::consts::FRAC_PI_2, // face +X (east): sun visible at dawn
            pitch: -0.08,
        };
        let day_time = (self.day_time_base + self.frame) as i64;
        let sky = crate::sky::sample(day_time);
        let ctx = RenderCtx {
            view: cam.view(),
            proj: Mat4::perspective(
                70.0f32.to_radians(),
                fb.w as f32 / fb.h as f32,
                0.1,
                512.0,
            ),
            fog_color: sky.fog_color,
            fog_near: n * 0.55,
            fog_far: n * 1.7,
        };
        // Official render order: sky color -> sun/moon/stars -> terrain ->
        // clouds (depth-tested against terrain, the CLOUDS pass runs with
        // the main depth buffer in the game's frame graph).
        fb.clear(sky.sky_color, sky.fog_color);
        crate::sky::render_celestials(fb, &ctx, &cam, day_time);
        if let Some(quads) = &self.world_quads {
            fb.draw_quads(&ctx, quads);
        }
        crate::sky::render_clouds(fb, &ctx, &cam, day_time, 0.0);
    }

    fn ensure_world(&mut self) {
        if self.world_quads.is_none() {
            self.enter_world();
        }
    }

    /// Vanilla HUD: crosshair, hotbar + selection, hearts and hunger — all
    /// official `hud/*` sprites at vanilla positions and sizes.
    fn render_hud(&mut self, g: &mut Gui, a: &mut GuiAssets) {
        let (w, h) = (g.w(), g.h());
        // crosshair: centered 15x15
        g.sprite(a, "hud/crosshair", w / 2 - 7, h / 2 - 7);
        // hotbar: 182x22 centered at the bottom, selector around slot 0
        let (hx, hy) = hotbar_rect(w, h);
        g.sprite(a, "hud/hotbar", hx, hy);
        let sel_x = hx - 1 + self.selected_hotbar as i32 * (HEART_SPACING + 12);
        g.sprite(a, "hud/hotbar_selection", sel_x, hy - 1);
        // health: 10 full hearts above the hotbar's left half (health 20)
        let (hbx, hby) = hearts_origin(w, h);
        for i in 0..HEALTH / 2 {
            g.sprite(a, "hud/heart/full", hbx + i * HEART_SPACING, hby);
        }
        // hunger: 10 full drumsticks above the hotbar's right half,
        // right-aligned like vanilla
        let (fx, fy) = food_origin(w, h);
        for i in 0..HEALTH / 2 {
            g.sprite(a, "hud/food_full", fx - i * HEART_SPACING, fy);
        }
    }

    /// Vanilla pause menu (PauseScreen): "Game Paused" + Back to Game,
    /// Options..., Save and Quit to Title over a dimmed frozen world.
    fn render_pause(&mut self, g: &mut Gui, a: &mut GuiAssets, w: i32, h: i32) {
        g.fill(0, 0, w, h, [0.0, 0.0, 0.0, 0.35]);
        g.text_centered(a, w / 2, h / 4 - 20, tr("menu.paused").unwrap_or("Game Paused"), WHITE, true);
        let cx = w / 2;
        let y0 = h / 4 + 8;
        let half = 200 / 2 - 2;
        self.buttons.push(Button::new(
            "backtogame",
            tr("menu.returnToGame").unwrap_or("Back to Game").to_string(),
            cx,
            y0,
            200,
        ));
        self.buttons.push(Button::new(
            "options",
            tr("menu.options").unwrap_or("Options...").to_string(),
            cx - half / 2,
            y0 + 24,
            half,
        ));
        self.buttons.push(Button::new(
            "saveandquit",
            tr("menu.returnToMenu").unwrap_or("Save and Quit to Title").to_string(),
            cx + half / 2 + 2,
            y0 + 24,
            half,
        ));
        for b in &self.buttons.clone() {
            b.render(g, a, -1, -1);
        }
    }

    fn render_loading(&mut self, g: &mut Gui, a: &mut GuiAssets) {
        let (w, h) = (g.w(), g.h());
        g.fill(0, 0, w, h, [MOJANG_RED[0], MOJANG_RED[1], MOJANG_RED[2], 1.0]);
        // studio wordmark: the REAL Mojang Studios texture (title/mojangstudios,
        // 512x512 drawn at quarter scale = 128x128 like the vanilla loading
        // overlay); text fallback keeps the layout while assets are absent.
        let has_mark = a.sprite("title/mojangstudios").map(|s| s.w >= 16).unwrap_or(false);
        if has_mark {
            g.sprite_at(a, "title/mojangstudios", w / 2 - 64, h / 2 - 74, 128, 128);
        } else {
            let logo = "MOJANG STUDIOS";
            let lw = g.text_width(logo);
            g.text(a, w / 2 - lw / 2, h / 2 - 14, logo, WHITE, false);
        }
        // two progress bars along the bottom (vanilla loading layout)
        let bar_w = 70;
        let bar_y = h - 26;
        let track = [0.35, 0.08, 0.10, 1.0];
        g.fill(w / 2 - bar_w - 2, bar_y, bar_w, 5, track);
        g.fill(w / 2 + 2, bar_y, bar_w, 5, track);
        let p = self.boot_progress;
        let fill1 = ((p * 2.0).min(1.0) * bar_w as f32) as i32;
        let fill2 = (((p - 0.5).max(0.0) * 2.0) * bar_w as f32) as i32;
        let green = [0.06, 0.8, 0.15, 1.0];
        g.fill(w / 2 - bar_w - 2, bar_y, fill1, 5, [1.0, 1.0, 1.0, 1.0]);
        g.fill(w / 2 + 2, bar_y, fill2, 5, green);
        let stage = BOOT_STAGES.get(self.boot_stage).copied().unwrap_or(BOOT_STAGES[0]);
        let label = format!("{} ({})", tr("mc2rs.boot.loading").unwrap_or("Loading..."), stage);
        g.text_centered(a, w / 2, bar_y + 10, &label, WHITE, false);
    }

    fn render_title(&mut self, g: &mut Gui, a: &mut GuiAssets, w: i32, h: i32) {
        // backdrop: the REAL title panorama — six official cube faces sliding
        // horizontally (linear stand-in for the vanilla rotating cube). Flat
        // gradient fallback while the panorama assets are unavailable.
        let panorama = self.render_panorama(g, a, w, h);
        if !panorama {
            for y in 0..h {
                let t = y as f32 / h.max(1) as f32;
                let c = [0.12 + 0.05 * t, 0.16 + 0.07 * t, 0.25 + 0.10 * t];
                g.fill(0, y, w, 1, [c[0], c[1], c[2], 1.0]);
            }
        }
        // wordmark: the REAL Minecraft logo texture (title/minecraft, 1024x256
        // drawn at quarter scale = 256x64, vanilla LogoRenderer proportions);
        // text fallback when the texture is missing or the GUI is too small.
        let (logo_x, logo_w, logo_bottom) = if w >= 260 {
            let lw = 256;
            let lx = w / 2 - lw / 2;
            let ly = h / 4 - 30;
            g.sprite_at(a, "title/minecraft", lx, ly, lw, 64);
            (lx, lw, ly + 64)
        } else {
            let logo = "MINECRAFT";
            let lw = g.text_width(logo);
            let lx = w / 2 - lw / 2;
            let ly = h / 4 - 6;
            for k in 0..3 {
                g.text(a, lx, ly + k * 8, logo, WHITE, true);
            }
            (lx, lw, ly + 24)
        };
        // splash text (yellow, pulsing, deterministic pick) anchored at the
        // logo's bottom-right corner like vanilla
        let splash = SPLASHES.get(self.splash_index).copied().unwrap_or("Now in Rust!");
        let pulse = ((self.frame as f32 * 0.1).sin() * 0.5 + 0.5) * 2.0;
        let splash_scale = 1 + (pulse.round() as i32);
        let sw = g.text_width(splash);
        let sx = logo_x + logo_w - sw - 2;
        let sy = logo_bottom - 12;
        if splash_scale > 1 {
            // pulse emphasis: stamp twice at 1px offsets (cheap big mode)
            g.text(a, sx, sy, splash, YELLOW, true);
        }
        g.text(a, sx, sy, splash, YELLOW, true);

        // button grid (vanilla layout: two half-width rows + full-width row)
        let cx = w / 2;
        let bw = 200;
        let y0 = h / 4 + 44;
        let half = bw / 2 - 2;
        self.buttons.push(Button::new(
            "singleplayer",
            tr("menu.singleplayer").unwrap_or("Singleplayer").to_string(),
            cx - half / 2,
            y0,
            half,
        ));
        self.buttons.push(Button::new(
            "multiplayer",
            tr("menu.multiplayer").unwrap_or("Multiplayer").to_string(),
            cx + half / 2 + 2,
            y0,
            half,
        ));
        self.buttons.push(Button::new(
            "realms",
            tr("menu.online").unwrap_or("Minecraft Realms").to_string(),
            cx,
            y0 + 24,
            bw,
        ));
        self.buttons.push(Button::new(
            "options",
            format!("{}...", tr("menu.options").unwrap_or("Options")),
            cx - half / 2,
            y0 + 48,
            half,
        ));
        self.buttons.push(Button::new(
            "quit",
            tr("menu.quit").unwrap_or("Quit Game").to_string(),
            cx + half / 2 + 2,
            y0 + 48,
            half,
        ));
        for b in &self.buttons.clone() {
            b.render(g, a, -1, -1);
        }

        // version bottom-left, copyright bottom-right (vanilla overlay text)
        g.text(a, 2, h - 10, &format!("Minecraft {}", crate::build_info::MINECRAFT_VERSION), WHITE, true);
        let cp = "Copyright Mojang AB. Do not distribute!";
        let cw = g.text_width(cp);
        g.text(a, w - cw - 2, h - 10, cp, WHITE, true);
    }

    /// Draw the six official panorama faces as one horizontal strip sliding
    /// with `frame` (a linear stand-in for the vanilla cube rotation; the
    /// spherical warp is future work). Returns false when the real faces are
    /// unavailable — 1x1 jar placeholders don't count.
    fn render_panorama(&mut self, g: &mut Gui, a: &mut GuiAssets, w: i32, h: i32) -> bool {
        const FACES: [&str; 6] = [
            "title/background/panorama_0",
            "title/background/panorama_1",
            "title/background/panorama_2",
            "title/background/panorama_3",
            "title/background/panorama_4",
            "title/background/panorama_5",
        ];
        let face = match a.sprite(FACES[0]) {
            Some(s) if s.w >= 16 && s.h >= 16 => (s.w, s.h),
            _ => return false,
        };
        let face_w = (h as f32 * face.0 as f32 / face.1 as f32).max(1.0);
        let strip = face_w * FACES.len() as f32;
        let off = panorama_offset(self.frame, strip);
        let mut x = -off;
        let mut i = 0usize;
        while x < w as f32 {
            g.sprite_at(a, FACES[i % 6], x.round() as i32, 0, face_w.round() as i32, h);
            x += face_w;
            i += 1;
        }
        true
    }

    fn render_servers(&mut self, g: &mut Gui, a: &mut GuiAssets, w: i32, h: i32) {
        // dark list backdrop
        g.fill(0, 0, w, h, [0.06, 0.06, 0.06, 1.0]);
        g.text_centered(a, w / 2, 12, tr("multiplayer.title").unwrap_or("Play Multiplayer"), WHITE, true);
        let cx = w / 2;
        // server rows
        for (i, s) in self.servers.iter().enumerate() {
            let y = 34 + i as i32 * 28;
            let selected = i == self.selected_server;
            if selected {
                g.fill(cx - 100, y - 2, 200, 22, [1.0, 1.0, 1.0, 0.10]);
            }
            let name_color = if s.online { WHITE } else { [0.5, 0.5, 0.5] };
            g.text(a, cx - 98, y, &s.name, name_color, true);
            g.text(a, cx - 98, y + 10, &s.motd, GRAY, true);
            if s.online {
                let ver = format!("{} - {}", s.ping_ms, s.version);
                let vw = g.text_width(&ver);
                g.text(a, cx + 98 - vw, y, &ver, GRAY, true);
                g.sprite(a, s.ping_icon(), cx + 88, y + 10);
            } else {
                let msg = tr("connect.failed").unwrap_or("Failed to connect to the server");
                g.text(a, cx - 98, y + 10, msg, [0.667, 0.333, 0.333], true);
                g.sprite(a, s.ping_icon(), cx + 88, y + 10);
            }
        }
        // action buttons
        let bw = 100;
        let half = bw / 2 - 2;
        self.buttons.push(Button::new(
            "join",
            tr("selectServer.select").unwrap_or("Join Server").to_string(),
            cx - bw - half / 2,
            h - 28,
            bw,
        ));
        self.buttons.push(Button::new(
            "refresh",
            tr("selectServer.refresh").unwrap_or("Refresh").to_string(),
            cx,
            h - 28,
            bw,
        ));
        self.buttons.push(Button::new(
            "back",
            tr("gui.cancel").unwrap_or("Cancel").to_string(),
            cx + bw + half / 2,
            h - 28,
            bw,
        ));
        for b in &self.buttons.clone() {
            b.render(g, a, -1, -1);
        }
    }

    fn render_options(&mut self, g: &mut Gui, a: &mut GuiAssets, w: i32, h: i32) {
        for y in 0..h {
            let t = y as f32 / h.max(1) as f32;
            let c = [0.10 + 0.04 * t, 0.10 + 0.04 * t, 0.13 + 0.05 * t];
            g.fill(0, y, w, 1, [c[0], c[1], c[2], 1.0]);
        }
        g.text_centered(a, w / 2, 12, tr("options.title").unwrap_or("Options"), WHITE, true);
        let cx = w / 2;
        let bw = 140;
        // volume sliders rendered as labeled buttons with percentage fill
        let music = format!(
            "{}: {}%",
            tr("options.musicVolume").unwrap_or("Music & Sounds"),
            100
        );
        self.buttons.push(Button::new("opt_music", music, cx, h / 2 - 16, bw));
        let fov = format!("FOV: {}", 70);
        self.buttons.push(Button::new("opt_fov", fov, cx, h / 2 + 8, bw));
        self.buttons.push(Button::new(
            "back",
            tr("gui.done").unwrap_or("Done").to_string(),
            cx,
            h - 28,
            bw,
        ));
        for b in &self.buttons.clone() {
            b.render(g, a, -1, -1);
        }
    }
    /// Vanilla ConnectScreen: dark backdrop, the official "Connecting to
    /// server" line and a Cancel button (the blocking login runs off the
    /// frame thread — the screen itself is static exactly like the original).
    fn render_connect(&mut self, g: &mut Gui, a: &mut GuiAssets, w: i32, h: i32) {
        g.fill(0, 0, w, h, [0.06, 0.06, 0.06, 1.0]);
        let msg = tr("connect.connecting").unwrap_or("Connecting to server");
        g.text_centered(a, w / 2, h / 2 - 14, msg, WHITE, true);
        let bw = 100;
        self.buttons.push(Button::new(
            "cancel",
            tr("gui.cancel").unwrap_or("Cancel").to_string(),
            w / 2 - bw / 2,
            h - 28,
            bw,
        ));
        for b in &self.buttons.clone() {
            b.render(g, a, -1, -1);
        }
    }

    /// Vanilla DisconnectedScreen: the "Connection lost" title (official
    /// disconnect.lost), the wrapped reason lines and the Back button
    /// (the parent is the server list the join came from).
    fn render_disconnect(&mut self, g: &mut Gui, a: &mut GuiAssets, w: i32, h: i32) {
        g.fill(0, 0, w, h, [0.06, 0.06, 0.06, 1.0]);
        g.text_centered(a, w / 2, h / 2 - 20, &self.disconnect_title, WHITE, true);
        let lines = wrap_lines(g, &self.disconnect_reason, w - 20);
        for (i, line) in lines.iter().take(4).enumerate() {
            g.text_centered(a, w / 2, h / 2 - 6 + i as i32 * 10, line, GRAY, true);
        }
        let bw = 140;
        self.buttons.push(Button::new(
            "backtoservers",
            tr("gui.back").unwrap_or("Back").to_string(),
            w / 2 - bw / 2,
            h - 28,
            bw,
        ));
        for b in &self.buttons.clone() {
            b.render(g, a, -1, -1);
        }
    }
}

/// Minimal greedy word wrap (the vanilla Font splitter wraps the disconnect
/// reason the same way — one string per rendered row).
fn wrap_lines(g: &Gui, text: &str, max_w: i32) -> Vec<String> {
    let mut lines = Vec::new();
    for para in text.split('\n') {
        let mut cur = String::new();
        for word in para.split(' ') {
            let candidate = if cur.is_empty() {
                word.to_string()
            } else {
                format!("{cur} {word}")
            };
            if !cur.is_empty() && g.text_width(&candidate) as i32 > max_w {
                lines.push(std::mem::take(&mut cur));
                cur = word.to_string();
            } else {
                cur = candidate;
            }
        }
        lines.push(cur);
    }
    lines
}

/// Deterministic splitmix64 (same primitive as the world seed).
fn splitmix(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The world's spawn biome: a seeded pick from the OFFICIAL biome registry
/// (generated worldgen table). Until the terrain generator evaluates the
/// real climate source, this stands in as the situation's biome owner —
/// deterministic per seed, always an official id.
fn spawn_biome(seed: u64) -> &'static str {
    use mc_registry::generated::worldgen as wm;
    let i = (splitmix(seed) % wm::BIOMES.len() as u64) as usize;
    wm::BIOMES[i].name
}

/// Horizontal panorama offset for a frame — deterministic, wraps the strip
/// (vanilla pans the panorama slowly; we slide the cube strip at a similar
/// pace: one face every ~360 frames).
fn panorama_offset(frame: u64, strip_w: f32) -> f32 {
    (frame as f32 * 0.25) % strip_w.max(1.0)
}

/// Vanilla HUD layout: the hotbar (182x22) sits centered, its bottom edge
/// 1 GUI px above the screen bottom.
fn hotbar_rect(gui_w: i32, gui_h: i32) -> (i32, i32) {
    (gui_w / 2 - HOTBAR_W / 2, gui_h - HOTBAR_H - 1)
}

/// Health row origin: first (leftmost) heart, one px above the hotbar.
fn hearts_origin(gui_w: i32, gui_h: i32) -> (i32, i32) {
    let (hx, hy) = hotbar_rect(gui_w, gui_h);
    (hx, hy - HEART - 1)
}

/// Hunger row origin: first (rightmost) drumstick, aligned with the hearts,
/// right edge flush with the hotbar's right edge.
fn food_origin(gui_w: i32, gui_h: i32) -> (i32, i32) {
    let (hx, hy) = hotbar_rect(gui_w, gui_h);
    (hx + HOTBAR_W - HEART, hy - HEART - 1)
}

/// Top solid block y at (x, z) — the spawn column scan (CHUNK_Y when open).
fn top_solid_y(world: &crate::world::World, x: i32, z: i32) -> i32 {
    for y in (0..crate::world::CHUNK_Y as i32).rev() {
        if world.get(x, y, z) != crate::world::AIR {
            return y + 1;
        }
    }
    crate::world::CHUNK_Y as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splash_selection_is_deterministic_and_in_range() {
        let a = App::new(20260214);
        let b = App::new(20260214);
        assert_eq!(a.splash_index, b.splash_index);
        assert!(a.splash_index < SPLASHES.len(), "splash index in range");
        assert!(!SPLASHES.is_empty());
    }

    #[test]
    fn panorama_offset_is_deterministic_and_wraps() {
        let strip = 540.0;
        assert_eq!(panorama_offset(0, strip), 0.0);
        assert_eq!(panorama_offset(30, strip), 7.5);
        assert_eq!(panorama_offset(30, strip), panorama_offset(30, strip));
        // wraps: frame (strip/0.25) is exactly one full strip
        let wrap_frame = (strip / 0.25) as u64;
        assert!(panorama_offset(wrap_frame, strip) < 0.25);
        // degenerate strip never NaNs; offset stays within [0, strip)
        let o = panorama_offset(123, 0.0);
        assert!(o.is_finite() && (0.0..1.0).contains(&o));
    }

    #[test]
    fn boot_progress_transitions_to_title() {
        let mut app = App::new(7);
        assert_eq!(app.screen, ScreenKind::Loading);
        for _ in 0..30 {
            app.update();
        }
        assert_eq!(app.screen, ScreenKind::Title, "boot completes after 20 ticks");
        assert_eq!(app.boot_progress, 1.0);
    }

    #[test]
    fn singleplayer_click_enters_world_with_mesh() {
        let mut app = App::new(3);
        for _ in 0..30 {
            app.update();
        }
        assert_eq!(app.screen, ScreenKind::Title);
        app.render_title_buttons_for_test();
        let sp = app.buttons.iter().find(|b| b.id == "singleplayer").expect("singleplayer button").clone();
        app.on_click(sp.x + sp.w / 2, sp.y + sp.h / 2);
        assert_eq!(app.screen, ScreenKind::World);
        assert!(!app.paused);
        let quads = app.world_quads.as_ref().expect("world meshed on entry");
        assert!(!quads.is_empty(), "mesh produced faces");
        assert!(app.spawn_y > 0, "spawn eye above bedrock");
    }

    #[test]
    fn world_music_follows_the_official_spawn_biome() {
        // the situation is resolved from the seeded spawn biome through the
        // generated biome-music table: the playing event is either the
        // biome's official background-music event or the vanilla in-game
        // default for biomes without one; deterministic per seed either way
        use mc_registry::generated::worldgen as wm;
        let seed = 3;
        let biome = spawn_biome(seed);
        assert!(wm::biome(biome).is_some(), "{biome} must be an official biome");
        let expect = wm::biome_music(biome, "default")
            .map(|m| m.event)
            .unwrap_or("music.game");
        let mut app = App::new(seed);
        app.enter_world();
        assert_eq!(app.music.mode(), crate::music::MusicMode::Game);
        assert_eq!(
            app.music.game_tracks(),
            &[expect] as &[&str],
            "seed {seed} biome {biome} must resolve to {expect}"
        );
        // determinism: the same seed resolves the same situation
        let mut again = App::new(seed);
        again.enter_world();
        assert_eq!(again.music.game_tracks(), app.music.game_tracks());
    }

    #[test]
    fn escape_toggles_pause_over_world() {
        let mut app = App::new(3);
        app.enter_world();
        assert!(!app.paused);
        app.on_escape();
        assert!(app.paused);
        app.on_escape();
        assert!(!app.paused);
        // escape on menus is a no-op
        app.open(ScreenKind::Title);
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::Title);
    }

    #[test]
    fn pause_buttons_route_back_to_game_options_and_title() {
        let mut app = App::new(3);
        app.enter_world();
        app.paused = true;
        app.render_title_buttons_for_test(); // builds pause buttons
        let back = app.buttons.iter().find(|b| b.id == "backtogame").expect("backtogame").clone();
        app.on_click(back.x + back.w / 2, back.y + back.h / 2);
        assert!(!app.paused, "Back to Game resumes");
        assert_eq!(app.screen, ScreenKind::World);

        app.paused = true;
        app.render_title_buttons_for_test();
        let quit = app.buttons.iter().find(|b| b.id == "saveandquit").expect("saveandquit").clone();
        app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
        assert_eq!(app.screen, ScreenKind::Title, "Save and Quit returns to title");
        assert!(!app.paused);

        // options from pause returns to the paused world
        app.enter_world();
        app.paused = true;
        app.render_title_buttons_for_test();
        let opt = app.buttons.iter().find(|b| b.id == "options").expect("pauseoptions").clone();
        app.on_click(opt.x + opt.w / 2, opt.y + opt.h / 2);
        assert_eq!(app.screen, ScreenKind::Options);
        app.render_title_buttons_for_test();
        let back = app.buttons.iter().find(|b| b.id == "back").expect("back").clone();
        app.on_click(back.x + back.w / 2, back.y + back.h / 2);
        assert_eq!(app.screen, ScreenKind::World);
        assert!(app.paused, "returning from options stays paused");
    }

    #[test]
    fn hud_layout_matches_vanilla_geometry() {
        // vanilla: hotbar 182x22 centered, bottom edge 1px above screen;
        // hearts above left half, hunger above right half
        let (w, h) = (320, 180);
        assert_eq!(hotbar_rect(w, h), (320 / 2 - 91, 180 - 23));
        assert_eq!(hearts_origin(w, h), (320 / 2 - 91, 180 - 23 - 10));
        let (fx, _) = food_origin(w, h);
        assert_eq!(fx + HEART, 320 / 2 + 91, "hunger right edge flush with hotbar");
    }

    #[test]
    fn spawn_scan_finds_top_solid_block() {
        let world = crate::world::World::new(1, 42);
        let y = top_solid_y(&world, 0, 0);
        assert!(y >= 1 && y <= crate::world::CHUNK_Y as i32);
        assert_ne!(world.get(0, y - 1, 0), crate::world::AIR, "block below eye is solid");
    }

    #[test]
    fn ping_icons_follow_vanilla_thresholds() {
        let mut e = ServerEntry {
            name: "x".into(),
            motd: "y".into(),
            ping_ms: 10,
            online: true,
            version: String::new(),
            host: None,
        };
        assert_eq!(e.ping_icon(), "server_list/ping_5");
        e.ping_ms = 90;
        assert_eq!(e.ping_icon(), "server_list/ping_3");
        e.ping_ms = 999;
        assert_eq!(e.ping_icon(), "server_list/ping_1");
        e.online = false;
        assert_eq!(e.ping_icon(), "server_list/unreachable");
    }

    #[test]
    fn button_hover_and_hitbox() {
        let b = Button::new("test", "Hi".into(), 50, 40, 100);
        assert_eq!((b.x, b.y, b.w, b.h), (0, 40, 100, 20));
        assert!(b.hovered(50, 50));
        assert!(!b.hovered(150, 50));
        assert!(!b.hovered(50, 61));
    }

    #[test]
    fn gui_cancel_and_title_keys_exist_in_official_lang() {
        // The generated table must contain the keys our screens render.
        assert_eq!(tr("gui.cancel"), Some("Cancel"));
        assert_eq!(tr("menu.singleplayer"), Some("Singleplayer"));
        assert_eq!(tr("menu.multiplayer"), Some("Multiplayer"));
        assert_eq!(tr("menu.options"), Some("Options..."));
        assert_eq!(tr("menu.quit"), Some("Quit Game"));
        assert_eq!(tr("multiplayer.title"), Some("Play Multiplayer"));
        assert_eq!(tr("connect.failed"), Some("Failed to connect to the server"));
        assert_eq!(tr("selectServer.select"), Some("Join Server"));
        assert_eq!(tr("options.title"), Some("Options"));
        assert_eq!(tr("menu.online"), Some("Minecraft Realms"));
    }

    #[test]
    fn click_navigates_screens() {
        let mut app = App::new(3);
        for _ in 0..30 {
            app.update();
        }
        assert_eq!(app.screen, ScreenKind::Title);
        // render once to build the title buttons, then click Multiplayer
        app.render_title_buttons_for_test();
        let mp = app.buttons.iter().find(|b| b.id == "multiplayer").expect("multiplayer button").clone();
        app.on_click(mp.x + mp.w / 2, mp.y + mp.h / 2);
        assert_eq!(app.screen, ScreenKind::Servers);
    }

    #[test]
    fn button_click_records_the_vanilla_ui_sound() {
        let mut app = App::new(3);
        for _ in 0..30 {
            app.update();
        }
        app.render_title_buttons_for_test();
        let mp = app.buttons.iter().find(|b| b.id == "multiplayer").expect("multiplayer button").clone();
        app.on_click(mp.x + mp.w / 2, mp.y + mp.h / 2);
        // vanilla AbstractButton.playDownSound: exactly one ui.button.click
        // per activated button, stamped with the pressing frame
        assert_eq!(app.ui_sounds, vec![format!("ui.button.click @{}", app.frame)]);
        // a click on empty space (no button) is silent
        app.on_click(1, 1);
        assert_eq!(app.ui_sounds.len(), 1, "no button -> no click sound");
        // no-op actions (quit has no headless behavior) still play the click
        app.open(ScreenKind::Title);
        app.render_title_buttons_for_test();
        let quit = app.buttons.iter().find(|b| b.id == "quit").expect("quit button").clone();
        app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
        assert_eq!(app.ui_sounds.len(), 2, "quit still clicks, like vanilla");
        assert_eq!(app.screen, ScreenKind::Title);
    }

    // ---- server-list refresh: the REAL mc-protocol status ping ----

    #[test]
    fn server_addresses_parse_with_the_vanilla_default_port() {
        assert_eq!(parse_address("hypixel.net"), ("hypixel.net".to_string(), 25565));
        assert_eq!(parse_address("localhost:25500"), ("localhost".to_string(), 25500));
        // non-numeric / empty ports leave the whole string as the host
        assert_eq!(parse_address("host:notaport"), ("host:notaport".to_string(), 25565));
        assert_eq!(parse_address("mc.example.com:"), ("mc.example.com:".to_string(), 25565));
    }

    #[test]
    fn refresh_pings_only_rows_with_real_addresses() {
        let mut app = App::new(3);
        for _ in 0..30 {
            app.update();
        }
        app.open(ScreenKind::Servers);
        // render once to build the servers-screen buttons (the helper
        // renders whichever screen is active)
        app.render_title_buttons_for_test();
        let refresh = app
            .buttons
            .iter()
            .find(|b| b.id == "refresh")
            .expect("refresh button")
            .clone();
        app.on_click(refresh.x + refresh.w / 2, refresh.y + refresh.h / 2);
        // the sandbox row has no address and stays unpinged; hypixel.net
        // is requested with the vanilla default port
        assert_eq!(app.net_requests.len(), 1);
        let req = &app.net_requests[0];
        assert_eq!(req.entry, 1);
        assert_eq!(req.host, "hypixel.net");
        assert_eq!(req.port, 25565);
        assert_eq!(req.id, 0);
        // and the click still played the vanilla ui sound
        assert_eq!(app.ui_sounds.len(), 1);
    }

    #[test]
    fn ping_result_updates_the_row_in_place() {
        let mut app = App::new(3);
        app.open(ScreenKind::Servers);
        app.refresh_servers();
        let id = app.net_requests[0].id;
        app.ping_finished(
            id,
            Ok(PingInfo {
                motd: "A Minecraft Server".to_string(),
                version: "26.2".to_string(),
                ping_ms: 42,
            }),
        );
        let s = &app.servers[1];
        assert!(s.online);
        assert_eq!(s.motd, "A Minecraft Server");
        assert_eq!(s.version, "26.2");
        assert_eq!(s.ping_ms, 42);
        // unknown/already-consumed ids are ignored, the log stays intact
        app.ping_finished(999, Err("nope".into()));
        assert!(app.servers[1].online);
        assert_eq!(app.net_requests.len(), 1);
    }

    #[test]
    fn unreachable_ping_renders_the_vanilla_offline_row() {
        let mut app = App::new(3);
        app.open(ScreenKind::Servers);
        app.refresh_servers();
        let id = app.net_requests[0].id;
        app.servers[1].online = true; // cached list says online until pinged
        app.ping_finished(id, Err("tcp connect to 127.0.0.1:1".into()));
        let s = &app.servers[1];
        assert!(!s.online);
        assert_eq!(s.ping_ms, 0);
        assert_eq!(s.ping_icon(), "server_list/unreachable");
    }

    #[test]
    fn server_list_refresh_runs_a_real_status_ping() {
        // in-process vanilla-shaped status server (same dogfooding as the
        // mc-protocol login tests): handshake + request -> canned JSON
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            use mc_protocol::framing::PacketConn;
            use mc_protocol::varint::{write_string, write_varint};
            let (stream, _) = listener.accept().unwrap();
            let mut conn = PacketConn::new(stream);
            let hs = conn.read_packet().unwrap();
            assert_eq!(hs[0], 0x00, "handshake packet id");
            let rq = conn.read_packet().unwrap();
            assert_eq!(rq[0], 0x00, "status request packet id");
            let mut payload = Vec::new();
            write_varint(&mut payload, 0x00);
            write_string(
                &mut payload,
                r#"{"version":{"name":"26.2","protocol":776},"players":{"online":3,"max":42},"description":{"text":"mc2rs test server"}}"#,
            );
            conn.send_packet(&payload).unwrap();
        });

        let mut app = App::new(3);
        app.open(ScreenKind::Servers);
        // point the hosted row at the mock server
        app.servers[1].host = Some(format!("127.0.0.1:{port}"));
        app.refresh_servers();
        let req = app.net_requests[0].clone();
        assert_eq!(req.port, port, "explicit port parsed from the address");

        // the exact mapping the window worker performs
        let res = mc_protocol::ping(&req.host, req.port)
            .map(|r| PingInfo {
                motd: r.motd,
                version: r.version_name,
                ping_ms: r.latency_ms.min(i32::MAX as u128) as i32,
            })
            .map_err(|e| format!("{e:#}"));
        server.join().unwrap();
        app.ping_finished(req.id, res);

        let s = &app.servers[1];
        assert!(s.online, "the mock server answered the status request");
        assert_eq!(s.motd, "mc2rs test server");
        assert_eq!(s.version, "26.2");
        assert!(s.ping_ms >= 0);
        assert!(s.ping_icon().starts_with("server_list/ping_"));
    }

    #[test]
    fn join_click_fires_the_real_login_for_hosted_rows() {
        let mut app = App::new(3);
        app.open(ScreenKind::Servers);
        // select the hosted row (second row, vanilla click-to-select), then
        // click through the REAL join button
        app.render_title_buttons_for_test();
        app.on_click(80, 34 + 28 + 12);
        assert_eq!(app.selected_server, 1);
        app.render_title_buttons_for_test();
        let join = app
            .buttons
            .iter()
            .find(|b| b.id == "join")
            .expect("join button")
            .clone();
        app.on_click(join.x + join.w / 2, join.y + join.h / 2);
        // the vanilla ConnectScreen flow: one login request against the
        // hosted row's address, screen switched while the login is in flight
        assert_eq!(app.login_requests.len(), 1);
        let req = &app.login_requests[0];
        assert_eq!(req.entry, 1);
        assert_eq!(req.host, "hypixel.net");
        assert_eq!(req.port, 25565);
        assert_eq!(req.id, 0);
        assert_eq!(app.screen, ScreenKind::Connect);
        // and the click still played the vanilla ui sound
        assert_eq!(app.ui_sounds.len(), 1);
    }

    #[test]
    fn join_on_placeholder_rows_stays_a_noop_click() {
        let mut app = App::new(3);
        app.open(ScreenKind::Servers);
        app.render_title_buttons_for_test();
        // selected row 0 = the sandbox placeholder: nothing reachable to join
        let join = app
            .buttons
            .iter()
            .find(|b| b.id == "join")
            .expect("join button")
            .clone();
        app.on_click(join.x + join.w / 2, join.y + join.h / 2);
        assert!(app.login_requests.is_empty());
        assert_eq!(app.screen, ScreenKind::Servers);
        // but the button still activated, so the click sound still fired
        // (vanilla AbstractButton.playDownSound rule)
        assert_eq!(app.ui_sounds.len(), 1);
    }

    #[test]
    fn login_outcomes_route_like_vanilla() {
        let mut app = App::new(3);
        app.open(ScreenKind::Servers);
        app.selected_server = 1;
        app.join_selected_server();
        let id = app.login_requests[0].id;
        // server-side Login Disconnect: the official disconnect.lost title
        // + the flattened server reason
        app.login_finished(
            id,
            Ok(LoginResult::Disconnected { reason: "invalid session".into() }),
        );
        assert_eq!(app.screen, ScreenKind::Disconnect);
        assert_eq!(app.disconnect_title, tr("disconnect.lost").unwrap_or("Connection Lost"));
        assert_eq!(app.disconnect_reason, "invalid session");
        // transport errors surface the same vanilla way
        app.join_selected_server();
        let id2 = app.login_requests[1].id;
        app.login_finished(id2, Err("tcp connect to 127.0.0.1:1".into()));
        assert_eq!(app.screen, ScreenKind::Disconnect);
        assert_eq!(app.disconnect_reason, "tcp connect to 127.0.0.1:1");
        // Loaded means the client genuinely holds a network world: just
        // like the original, the completed join puts you IN it — the
        // world screen renders the materialized snapshot window
        app.join_selected_server();
        let id3 = app.login_requests[2].id;
        app.login_finished(
            id3,
            Ok(LoginResult::Loaded {
                username: "mc2rs".into(),
                entity_id: 17,
                view_distance: 8,
                chunks: 1,
                world: crate::world::World {
                    chunks_per_side: 2,
                    seed: 0,
                    chunks: vec![
                        vec![crate::world::AIR; crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z];
                        4
                    ],
                },
            }),
        );
        assert_eq!(app.screen, ScreenKind::World, "the join lands in the network world");
        assert!(app.world_is_network);
        assert_eq!(app.world_size, 32);
        // unknown/already-consumed ids are ignored, the log stays append-only
        app.login_finished(999, Err("nope".into()));
        assert_eq!(app.screen, ScreenKind::World, "an unknown id changes nothing");
        assert_eq!(app.login_requests.len(), 3);
    }

    #[test]
    fn connect_and_outcome_screens_return_to_the_server_list() {
        let mut app = App::new(3);
        app.open(ScreenKind::Servers);
        app.selected_server = 1;
        app.join_selected_server();
        assert_eq!(app.screen, ScreenKind::Connect);
        // Escape leaves the in-flight join (late results carry known ids and
        // still land — vanilla shows outcome screens whenever they arrive)
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::Servers);
        // the DisconnectedScreen's Back button returns to the list
        app.login_finished(
            app.login_requests[0].id,
            Ok(LoginResult::Disconnected { reason: "bye".into() }),
        );
        assert_eq!(app.screen, ScreenKind::Disconnect);
        app.render_title_buttons_for_test();
        let back = app
            .buttons
            .iter()
            .find(|b| b.id == "backtoservers")
            .expect("back button")
            .clone();
        app.on_click(back.x + back.w / 2, back.y + back.h / 2);
        assert_eq!(app.screen, ScreenKind::Servers);
    }

    #[test]
    fn server_join_runs_the_real_join_into_the_world() {
        // in-process vanilla-shaped join server (same dogfooding as the
        // mc-protocol config/play tests): handshake + login start ->
        // compression switch -> Login Success -> introductions -> probes +
        // finish -> the play state: world info, center chunk, chunk batch,
        // a REAL chunk (stone+dirt), keep-alive, ping, position teleport —
        // the worker pumps all the way to the world-load milestone
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = std::thread::spawn(move || {
            use mc_protocol::framing::PacketConn;
            use mc_protocol::varint::{read_string, read_varint, write_string, write_varint};
            let (stream, _) = listener.accept().unwrap();
            let mut conn = PacketConn::new(stream);
            let hs = conn.read_packet().unwrap();
            let mut c: &[u8] = &hs;
            assert_eq!(read_varint(&mut c).unwrap(), 0x00, "handshake packet id");
            assert_eq!(read_varint(&mut c).unwrap(), 776, "REAL protocol number");
            let _host = read_string(&mut c).unwrap();
            let mut port_b = [0u8; 2];
            std::io::Read::read_exact(&mut c, &mut port_b).unwrap();
            assert_eq!(read_varint(&mut c).unwrap(), 2, "handshake targets login");
            // login start BEFORE the compression switch (it travels
            // uncompressed — the client has not seen Set Compression yet)
            let start = conn.read_packet().unwrap();
            let mut c: &[u8] = &start;
            assert_eq!(read_varint(&mut c).unwrap(), 0x00, "login start id");
            let name = read_string(&mut c).unwrap();
            assert_eq!(name, crate::screens::PLAYER_NAME);
            conn.send_packet(&mc_protocol::login::encode_set_compression(256))
                .unwrap();
            conn.enable_compression(256);
            conn.send_packet(&mc_protocol::login::encode_login_success(
                &mc_protocol::login::offline_uuid(&name),
                &name,
            ))
            .unwrap();
            // the introductions: login acknowledged (0x03), client
            // information (0x00), plugin brand (0x02)
            for expected in [0x03, 0x00, 0x02] {
                let pkt = conn.read_packet().unwrap();
                let mut c: &[u8] = &pkt;
                assert_eq!(read_varint(&mut c).unwrap(), expected, "introduction id");
            }
            // Registry Data (a real vanilla server ALWAYS streams it in
            // the configuration phase): the shared mock fixture's
            // dimension_type registry (overworld -64/384) — the same
            // bytes the mc-protocol arc mock sends. Without it the join's
            // snapshot carries no layout knowledge and the world window
            // honestly falls back to the column-local copy.
            conn.send_packet(&mc_protocol::config::test_support::registry_packet(true))
                .unwrap();
            // probes + finish
            let mut ka = Vec::new();
            write_varint(&mut ka, 0x03); // config keep alive (clientbound)
            ka.extend_from_slice(&0x1122334455667788u64.to_be_bytes());
            conn.send_packet(&ka).unwrap();
            let mut pg = Vec::new();
            write_varint(&mut pg, 0x04); // config ping (clientbound)
            pg.extend_from_slice(&77i32.to_be_bytes());
            conn.send_packet(&pg).unwrap();
            let mut fin = Vec::new();
            write_varint(&mut fin, 0x02); // finish configuration
            conn.send_packet(&fin).unwrap();
            // responses: keep-alive echo (0x04), pong (0x05), ack (0x03)
            let resp = conn.read_packet().unwrap();
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), 0x04, "keep alive response id");
            assert_eq!(i64::from_be_bytes(c[..8].try_into().unwrap()), 0x1122334455667788);
            let resp = conn.read_packet().unwrap();
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), 0x05, "pong id");
            assert_eq!(i32::from_be_bytes(c[..4].try_into().unwrap()), 77);
            let resp = conn.read_packet().unwrap();
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), 0x03, "ack finish id");
            // ---- the play state: the world burst ----
            use mc_protocol::play::{clientbound, serverbound};
            let mut login_pkt = Vec::new();
            write_varint(&mut login_pkt, clientbound::LOGIN);
            login_pkt.extend_from_slice(&17i32.to_be_bytes()); // entity id
            login_pkt.push(0); // not hardcore
            write_varint(&mut login_pkt, 1); // one dimension
            write_string(&mut login_pkt, "minecraft:overworld");
            write_varint(&mut login_pkt, 20); // max players (dropped)
            write_varint(&mut login_pkt, 8); // chunk radius (view distance)
            write_varint(&mut login_pkt, 8); // simulation distance
            login_pkt.push(0); // reduced debug (dropped)
            login_pkt.push(1); // show death screen (dropped)
            login_pkt.push(0); // do limited crafting (dropped)
            // CommonPlayerSpawnInfo (vanilla 26.2): dimension_type id,
            // dimension, seed, game types, debug/flat, no death location,
            // portal cooldown, sea level
            write_varint(&mut login_pkt, 0);
            write_string(&mut login_pkt, "minecraft:overworld");
            login_pkt.extend_from_slice(&0i64.to_be_bytes()); // seed
            login_pkt.push(1); // game type
            login_pkt.push(1); // previous game type
            login_pkt.push(0); // is debug
            login_pkt.push(0); // is flat
            login_pkt.push(0); // no death location
            write_varint(&mut login_pkt, 300); // portal cooldown
            write_varint(&mut login_pkt, 63); // sea level
            login_pkt.push(1); // online mode (dropped)
            login_pkt.push(1); // enforces secure chat (dropped)
            conn.send_packet(&login_pkt).unwrap();
            let mut cc = Vec::new();
            write_varint(&mut cc, clientbound::CENTER_CHUNK);
            write_varint(&mut cc, 3);
            write_varint(&mut cc, -2);
            conn.send_packet(&cc).unwrap();
            // chunk batch start -> the pacing reply MUST arrive
            let mut bs = Vec::new();
            write_varint(&mut bs, clientbound::BATCH_START);
            conn.send_packet(&bs).unwrap();
            let resp = conn.read_packet().expect("chunk batch received");
            let mut c: &[u8] = &resp;
            assert_eq!(
                read_varint(&mut c).unwrap(),
                serverbound::CHUNK_BATCH_RECEIVED,
                "batch reply id"
            );
            let pace = f32::from_be_bytes(c[..4].try_into().unwrap());
            assert!(pace > 0.0, "server must be asked to keep streaming");
            // the real chunk (section 0): stone at (0,0,0), dirt at (1,2,3)
            // — the data array carries PALETTE INDICES, not global ids
            let palette = [0u32, 1, 10]; // air, stone, dirt
            let idx = |x: usize, y: usize, z: usize| (y << 8) | (z << 4) | x;
            let mut entries = vec![0u32; 4096];
            entries[idx(0, 0, 0)] = 1;
            entries[idx(3, 1, 2)] = 2;
            let mut buffer = Vec::new();
            buffer.extend_from_slice(&2i16.to_be_bytes()); // block count
            buffer.push(5); // bits per entry -> indirect palette
            write_varint(&mut buffer, palette.len() as i32);
            for id in palette {
                write_varint(&mut buffer, id as i32);
            }
            buffer.extend_from_slice(&mc_protocol::pack_entries(&entries, 5));
            buffer.push(0); // biomes: single valued
            write_varint(&mut buffer, 1);
            // a real vanilla join streams the FULL column (24 sections for
            // an overworld): 7 air sections below put the stone/dirt
            // section at index 7 — world y 48..63 with min_y -64, surface
            // terrain right under the teleport's feet y 64. The
            // player-anchored window (base 48 = 64 - 16) then shows the
            // stone at grid y 0 — a bottom-anchored copy would miss it.
            let mut air = Vec::new();
            air.extend_from_slice(&0i16.to_be_bytes()); // block count 0
            air.push(0); // bits per entry: single valued
            write_varint(&mut air, 0); // the single value: air
            air.push(0); // biomes: single valued
            write_varint(&mut air, 187);
            let mut column = Vec::new();
            for _ in 0..7 {
                column.extend_from_slice(&air); // sections arrive bottom-first
            }
            column.extend_from_slice(&buffer);
            let buffer = column;
            let mut cd = Vec::new();
            write_varint(&mut cd, clientbound::CHUNK_DATA);
            cd.extend_from_slice(&3i32.to_be_bytes());
            cd.extend_from_slice(&(-2i32).to_be_bytes());
            cd.push(0x0A);
            cd.push(0x00); // empty heightmaps compound
            write_varint(&mut cd, buffer.len() as i32);
            cd.extend_from_slice(&buffer);
            write_varint(&mut cd, 0); // no block entities
            cd.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // light masks
            conn.send_packet(&cd).unwrap();
            // keep alive -> echo (serverbound play 0x14)
            let mut kap = Vec::new();
            write_varint(&mut kap, clientbound::KEEP_ALIVE);
            kap.extend_from_slice(&(-5_000_000_000i64).to_be_bytes());
            conn.send_packet(&kap).unwrap();
            let resp = conn.read_packet().expect("play keep alive echo");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::KEEP_ALIVE);
            assert_eq!(i64::from_be_bytes(c[..8].try_into().unwrap()), -5_000_000_000);
            // ping -> pong (serverbound play 0x22)
            let mut pgp = Vec::new();
            write_varint(&mut pgp, clientbound::PING);
            pgp.extend_from_slice(&(-7i32).to_be_bytes());
            conn.send_packet(&pgp).unwrap();
            let resp = conn.read_packet().expect("play pong");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::PONG);
            assert_eq!(i32::from_be_bytes(c[..4].try_into().unwrap()), -7);
            // position teleport id 42 -> confirm teleportation (0x00).
            // The spawn sits INSIDE the centered chunk (3,-2) — a real
            // vanilla join burst is exactly this: the player position
            // within the view-distance center. floor(56.5)>>4 = 3,
            // floor(-27.25)>>4 = -2.
            let mut pos = Vec::new();
            write_varint(&mut pos, clientbound::SYNCHRONIZE_POSITION);
            pos.extend_from_slice(&56.5f64.to_be_bytes());
            pos.extend_from_slice(&64.0f64.to_be_bytes());
            pos.extend_from_slice(&(-27.25f64).to_be_bytes());
            pos.extend_from_slice(&90.0f32.to_be_bytes());
            pos.extend_from_slice(&0.0f32.to_be_bytes());
            pos.push(0x00); // all values absolute
            write_varint(&mut pos, 42);
            conn.send_packet(&pos).unwrap();
            let resp = conn.read_packet().expect("confirm teleportation");
            let mut c: &[u8] = &resp;
            assert_eq!(
                read_varint(&mut c).unwrap(),
                serverbound::CONFIRM_TELEPORTATION,
                "confirm id"
            );
            assert_eq!(read_varint(&mut c).unwrap(), 42, "teleport id verbatim");
        });

        let mut app = App::new(3);
        app.open(ScreenKind::Servers);
        // point the hosted row at the mock join server and join it
        app.servers[1].host = Some(format!("127.0.0.1:{port}"));
        app.selected_server = 1;
        app.join_selected_server();
        let req = app.login_requests[0].clone();
        assert_eq!(req.port, port, "explicit port parsed from the address");

        // the exact mapping the window worker performs: facts off the
        // snapshot, then the player-centered window materialized for the
        // world screen (the example mapper — the ids the mock streams;
        // the GUI resolves through the GENERATED registry table)
        let res = mc_protocol::connect_and_join_world(
            &req.host,
            req.port,
            crate::build_info::PROTOCOL_VERSION,
            crate::screens::PLAYER_NAME,
        )
        .map(|out| match out {
            mc_protocol::WorldOutcome::Loaded(snap) => {
                let (entity_id, view_distance, chunks) =
                    (snap.info.entity_id, snap.info.view_distance, snap.chunks.len());
                let world = crate::world_view::materialize_snapshot(
                    snap,
                    WORLD_CHUNKS,
                    |id| match id {
                        1 => crate::world::STONE,
                        10 => crate::world::DIRT,
                        _ => crate::world::AIR,
                    },
                );
                LoginResult::Loaded {
                    username: crate::screens::PLAYER_NAME.to_string(),
                    entity_id,
                    view_distance,
                    chunks,
                    world,
                }
            }
            mc_protocol::WorldOutcome::Disconnected { reason } => {
                LoginResult::Disconnected { reason }
            }
        })
        .map_err(|e| format!("{e:#}"));
        server.join().unwrap();
        app.login_finished(req.id, res);

        // the join landed IN the world (vanilla ConnectScreen → world):
        // the world screen renders the materialized network window, the
        // mock column (3,-2) sits at grid chunk (2,2) of the 5x5 window.
        // The stone at world y 48 landing at grid y 0 PINS the
        // player-anchored vertical placement (feet 64 - anchor 16 = base
        // 48): a bottom-anchored copy would show world -64..-16 — all air
        // this far above the one real section's world y 48.
        assert_eq!(app.screen, ScreenKind::World);
        assert!(app.world_is_network, "the world screen shows the network world");
        assert_eq!(app.world_size, (WORLD_CHUNKS * crate::world::CHUNK_X) as i32);
        let w = app.net_world.as_ref().expect("network world retained");
        assert_eq!(w.get(32, 0, 32), crate::world::STONE, "stone at world 48 = feet 64 - 16");
        assert_eq!(w.get(35, 1, 34), crate::world::DIRT, "dirt at world 49");
        assert_eq!(w.get(32, 16, 32), crate::world::AIR, "the feet row (world 64) is sky");
        assert!(
            !app.world_quads.as_ref().expect("meshed network world").is_empty(),
            "the network terrain meshes"
        );
        // the camera column (grid center) is air in this one-column world:
        // the spawn scan honestly reports the grid top
        assert_eq!(app.spawn_y, crate::world::CHUNK_Y as i32);
    }

    #[test]
    fn network_world_lands_in_the_world_screen_and_quits_cleanly() {
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        // an all-air network window: the state machine must not care
        app.net_world = Some(crate::world::World {
            chunks_per_side: 2,
            seed: 0,
            chunks: vec![
                vec![crate::world::AIR; crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z];
                4
            ],
        });
        app.enter_network_world();
        assert_eq!(app.screen, ScreenKind::World);
        assert!(app.world_is_network);
        assert_eq!(app.world_size, 32);
        assert_eq!(
            app.spawn_y,
            crate::world::CHUNK_Y as i32,
            "all-air window: the spawn scan reports the grid top"
        );
        assert!(
            app.world_quads.as_ref().unwrap().is_empty(),
            "an air world meshes to nothing"
        );

        // save and quit unloads the network level
        app.paused = true;
        app.render_title_buttons_for_test();
        let quit = app.buttons.iter().find(|b| b.id == "saveandquit").expect("saveandquit").clone();
        app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
        assert_eq!(app.screen, ScreenKind::Title);
        assert!(!app.world_is_network);
        assert!(app.world_quads.is_none(), "the network mesh is unloaded");

        // singleplayer afterwards rebuilds the PROCEDURAL world
        app.render_title_buttons_for_test();
        let sp = app.buttons.iter().find(|b| b.id == "singleplayer").expect("singleplayer").clone();
        app.on_click(sp.x + sp.w / 2, sp.y + sp.h / 2);
        assert_eq!(app.screen, ScreenKind::World);
        assert!(!app.world_is_network, "singleplayer renders the procedural world");
        assert!(!app.world_quads.unwrap().is_empty(), "procedural terrain meshes");
    }

    #[test]
    fn late_loaded_join_after_escape_is_dropped() {
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.on_escape(); // the user cancelled the join
        assert_eq!(app.screen, ScreenKind::Servers);
        app.login_requests.push(LoginRequest {
            id: 7,
            entry: 0,
            host: "127.0.0.1".to_string(),
            port: 1,
        });
        app.login_finished(
            7,
            Ok(LoginResult::Loaded {
                username: crate::screens::PLAYER_NAME.to_string(),
                entity_id: 1,
                view_distance: 8,
                chunks: 1,
                world: crate::world::World {
                    chunks_per_side: 2,
                    seed: 0,
                    chunks: vec![
                        vec![crate::world::AIR; crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z];
                        4
                    ],
                },
            }),
        );
        assert_eq!(
            app.screen,
            ScreenKind::Servers,
            "a cancelled join's late success is dropped"
        );
        assert!(app.net_world.is_none());
    }

    impl App {
        fn render_title_buttons_for_test(&mut self) {
            let mut fb = crate::raster::Framebuf::new(320, 180);
            let mut assets = GuiAssets::open("assets-cache/jar");
            self.render(&mut fb, &mut assets);
        }
    }
}

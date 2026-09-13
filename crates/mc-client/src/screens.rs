//! GUI screens: the boot chain (Mojang Studios loading -> title -> menus),
//! driven entirely by GENERATED data: every label comes from the official
//! language table, every sprite from the official sprite inventory, every
//! splash from texts/splashes.txt.

use crate::gui::{Gui, GuiAssets};
use crate::math::{Mat4, Vec3};
use crate::raster::RenderCtx;

use mc_registry::generated::lang::tr;
use mc_registry::generated::splashes::SPLASHES;
use mc_registry::generated::worldgen::Skybox;

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
#[derive(Debug, Clone, PartialEq)]
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
        /// The join teleport's EXACT pose in the grid frame (feet + the
        /// vanilla standing eye height, the server-directed yaw/pitch
        /// converted from the vanilla angle conventions) — the World
        /// screen renders FROM the player, the vanilla first person.
        /// `None` without dimension layout knowledge (the grid is
        /// column-local then and the player's world y means nothing in
        /// it): the deterministic flight camera stands instead.
        camera: Option<crate::camera::Camera>,
        /// The biome at the player's feet (the synced registry name,
        /// e.g. "minecraft:plains") — the situation key the biome music
        /// resolves through. `None` when the server streamed no biome
        /// registry, the section under the player is not loaded (yet),
        /// or the id is one the server never named: the generic
        /// `music.game` situation stands instead, never a guess.
        spawn_biome: Option<String>,
        /// The skin model the login profile named (`true` = slim) — the
        /// profile-metadata fact the first-person arm renders with; the
        /// offline/no-metadata shape is the classic wide default.
        slim_arm: bool,
        /// The current dimension's `DimensionType$Skybox` — the render
        /// state the sky pipeline gates the celestials on (the vanilla
        /// extract pins `dimensionType.skybox()` on the SkyRenderState at
        /// world entry). Resolved through the model the snapshot adopts
        /// into (the synced registry's own skybox, the codec default, or
        /// the built-in table — see `WorldModel::skybox`).
        skybox: Skybox,
        /// The dimension's STATED fog distances `(start, end)` in blocks
        /// (the nether's real 10/96) — `None` when no pair is stated,
        /// the bytecode defaults' shape (0/1024) whose far view is the
        /// documented window treatment's domain (see
        /// `WorldModel::fog_distances`).
        fog: Option<(f32, f32)>,
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
    /// The END sky's flash state (javap `ClientLevel.endFlashState`):
    /// ticked every frame on the client's game-time clock — the same
    /// `day_time` the celestials ride — and read by the END branch of
    /// [`crate::sky::render_celestials`]. Deterministic in game time
    /// (see [`crate::end_flash`]); an Overworld/Nether sky never reads it.
    pub end_flash: crate::end_flash::EndFlashState,
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
    /// The join teleport's exact camera pose in the grid frame (shipped
    /// with the world by the join worker); `None` = no pose known (a
    /// hand-installed world, or a join without layout knowledge) — the
    /// deterministic flight camera stands. Cleared with the level.
    pub net_camera: Option<crate::camera::Camera>,
    /// True while the World screen shows the NETWORK world (the quad mesh
    /// came from a join snapshot, not the procedural singleplayer world).
    pub world_is_network: bool,
    /// The current dimension's skybox — the vanilla SkyRenderState.skybox
    /// discipline: pinned at world entry from the dimension's
    /// `DimensionType$Skybox`, the sky pipeline gates the celestials on
    /// it (NONE renders no sky at all, END no celestials — the extract
    /// returns before the disc/sun/moon/stars state exists).
    pub world_skybox: Skybox,
    /// The dimension's STATED fog distances `(start, end)` (blocks) —
    /// pinned at level entry like the skybox, the join's resolved pair;
    /// `None` = no pair stated (the bytecode defaults 0/1024 are beyond
    /// this client's 5x5-chunk window, where the documented window
    /// treatment renders).
    pub world_fog: Option<(f32, f32)>,
    /// The first-person hand (the empty main-hand arm from the official
    /// player skin), loaded lazily from the runtime asset root on the first
    /// world frame; `None` = the skin asset is absent or failed its sha1
    /// verification and the hand is not drawn.
    pub arm: Option<crate::hand::ArmModel>,
    /// Which player-skin variant the first-person arm uses — vanilla picks
    /// it from the profile's texture metadata; this client has no profile,
    /// so it is an explicit selection (`--slim-arm`), defaulting to the
    /// wide classic the pixel goldens render.
    pub slim_arm: bool,
    /// The attack-swing animation state (LivingEntity's swing fields,
    /// verbatim); the deterministic headless scenes never swing, so the
    /// pixel goldens always see the rest pose.
    pub swing: crate::hand::SwingState,
    /// The held item (the first-person consumer of the display arc), set
    /// explicitly via `--held-item <model_id>` — the deterministic
    /// stand-in for inventory state (the client has no gameplay
    /// inventory). `None` = the empty main hand (the default scenes),
    /// which draws the arm; `Some` draws the item INSTEAD of the arm —
    /// vanilla's submitArmWithItem branch, never both.
    pub held_item: Option<crate::hand::HeldItem>,
    /// The last biome the network world's music situation resolved to
    /// (the synced registry name at the player's feet). `None` until a
    /// join resolves one; cleared with the level (Save and Quit, a kick).
    /// Live updates fire the situation change ONLY when this differs —
    /// the vanilla biome-music behavior, not a restart per refresh.
    pub net_biome: Option<String>,
    /// Live-session stop flags by login-request id (the frame thread
    /// attaches one per dispatched join; the worker checks its flag
    /// between packets). The App closes every session it no longer
    /// wants: a cancelled join's late success, a superseded join, and
    /// the network world's own session on Save and Quit — a ghost
    /// client would linger on the server otherwise. A fully idle
    /// connection keeps the worker blocked until the next packet
    /// arrives (vanilla servers always stream keep-alives, so a quit
    /// closes within seconds).
    pub live_runs: Vec<(u64, std::sync::Arc<std::sync::atomic::AtomicBool>)>,
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
            end_flash: crate::end_flash::EndFlashState::new(),
            ui_sounds: Vec::new(),
            net_requests: Vec::new(),
            next_ping_id: 0,
            login_requests: Vec::new(),
            next_login_id: 0,
            disconnect_title: String::new(),
            disconnect_reason: String::new(),
            net_world: None,
            net_camera: None,
            world_is_network: false,
            world_skybox: Skybox::Overworld,
            world_fog: None,
            net_biome: None,
            live_runs: Vec::new(),
            arm: None,
            slim_arm: false,
            swing: crate::hand::SwingState::default(),
            held_item: None,
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
        // the swing clock ticks every game tick like the player entity's
        // (at rest it pins swingTime to 0 and attackAnim to 0 — no-op)
        self.swing.tick();
    }

    /// The vanilla attack input (left mouse press in-world): starts the
    /// empty-hand swing. No-op outside the world or while paused (the
    /// pause menu freezes the game — vanilla has no attack there).
    pub fn on_attack(&mut self) {
        if self.screen == ScreenKind::World && !self.paused {
            self.swing.start();
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
            // the procedural world is an overworld world — the vanilla
            // extract pins the dimension's skybox on entering the level
            self.world_skybox = Skybox::Overworld;
            self.world_fog = None;
        }
        self.paused = false;
        self.music.on_world_enter_biome(spawn_biome(self.seed));
        self.open(ScreenKind::World);
    }

    /// Enter the world screen with the NETWORK world — the vanilla
    /// ConnectScreen → world handoff after a successful join. The screen
    /// machinery (sky, celestials, fog, clouds, HUD, pause) is shared
    /// with singleplayer; only the terrain source differs: the mesh comes
    /// from the materialized join snapshot. Camera: the join teleport's
    /// EXACT pose (position + yaw/pitch, the vanilla first person — see
    /// [`App::world_camera`]) when the worker shipped one; without it the
    /// deterministic flight camera stands. Music: the SPAWN BIOME the
    /// worker resolved from the decoded section biomes + the synced
    /// registry names drives the situation (the biome's official
    /// background-music event); without one (no registry streamed, the
    /// section under the player not loaded) the unknown-key lookup falls
    /// back to the generic `music.game` situation — no seeded guess
    /// about a network world.
    pub fn enter_network_world(&mut self, spawn_biome: Option<String>, skybox: Skybox, fog: Option<(f32, f32)>) {
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
        // the vanilla extract's render state: the dimension's skybox pins
        // at level entry — resolved through the live model (the synced
        // registry's own skybox, the codec default, or the built-in table;
        // see `WorldModel::skybox`)
        self.world_skybox = skybox;
        // the fog pair pins with it: the dimension's stated distances are
        // the level-entry render state (None = the window treatment)
        self.world_fog = fog;
        self.net_biome = spawn_biome.clone();
        self.paused = false;
        self.music
            .on_world_enter_biome(spawn_biome.as_deref().unwrap_or(""));
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
                        // it; singleplayer rebuilds its own world) — and
                        // the teleport pose rides along with it. The live
                        // session closes too: the socket goes down like
                        // the original's, no ghost client lingers.
                        self.world_quads = None;
                        self.world_is_network = false;
                        self.net_camera = None;
                        self.net_biome = None;
                        self.close_all_live_runs();
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
            Ok(LoginResult::Loaded { world, camera, spawn_biome, slim_arm, skybox, fog, .. }) => {
                if self.screen == ScreenKind::Connect {
                    // the profile's skin model rides the join: the first
                    // world frame's lazy arm load reads THIS flag (set
                    // before the screen switches, so the load sees it)
                    self.slim_arm = slim_arm;
                    self.net_world = Some(world);
                    self.net_camera = camera;
                    // the live view's stream: this join's session is THE
                    // one — every other in-flight join's session closes
                    // now (superseded joins never haunt the server)
                    self.prune_live_runs(Some(id));
                    self.enter_network_world(spawn_biome, skybox, fog);
                } else {
                    // a late success after Escape: the join is cancelled —
                    // close its session exactly like the vanilla client
                    // closes the connection on a cancelled join
                    self.close_live_run(id);
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

    /// The frame thread registers a dispatched join's live-session stop
    /// flag (the worker checks it between packets and closes the socket
    /// when it flips).
    pub fn attach_live_run(&mut self, id: u64, run: std::sync::Arc<std::sync::atomic::AtomicBool>) {
        self.live_runs.push((id, run));
    }

    /// Close every live session EXCEPT `keep`'s (the adopted join):
    /// cancelled or superseded joins' workers stop between packets and
    /// their sockets close — a ghost client never lingers on the server.
    fn prune_live_runs(&mut self, keep: Option<u64>) {
        self.live_runs.retain(|(id, run)| {
            if Some(*id) == keep {
                true
            } else {
                run.store(false, std::sync::atomic::Ordering::Relaxed);
                false
            }
        });
    }

    /// Close one join's live session by request id (a cancelled join's
    /// late success): that worker stops between packets and its socket
    /// closes; every other in-flight join keeps running.
    fn close_live_run(&mut self, id: u64) {
        self.live_runs.retain(|(rid, run)| {
            if *rid == id {
                run.store(false, std::sync::atomic::Ordering::Relaxed);
                false
            } else {
                true
            }
        });
    }

    /// Close EVERY live session (Save and Quit unloads the network
    /// world; a live kick ends it): all workers stop between packets,
    /// the sockets go down like the original's.
    fn close_all_live_runs(&mut self) {
        for (_, run) in self.live_runs.drain(..) {
            run.store(false, std::sync::atomic::Ordering::Relaxed);
        }
    }

    /// The live view refreshed: the play session's stream landed in the
    /// SAME model the screen renders and the worker shipped the new
    /// window — swap the network world in place (a re-sent chunk
    /// replaces its column, a new teleport re-anchors the grid and the
    /// pose). The shipped biome AT THE PLAYER re-resolves the music
    /// situation on CHANGE only (the vanilla biome-music behavior — a
    /// same-biome refresh never restarts the track); an unresolvable
    /// biome (None: outside the loaded columns, an unnamed id) keeps
    /// what is playing. Updates for a world the UI no longer shows
    /// (Save and Quit, a cancelled join) drop honestly.
    pub fn live_view_updated(
        &mut self,
        world: crate::world::World,
        camera: Option<crate::camera::Camera>,
        biome: Option<String>,
    ) {
        if !self.world_is_network {
            return;
        }
        self.net_world = Some(world);
        self.net_camera = camera;
        if let Some(b) = biome {
            if self.net_biome.as_deref() != Some(b.as_str()) {
                self.net_biome = Some(b.clone());
                self.music.on_world_enter_biome(&b);
            }
        }
        if self.screen == ScreenKind::World {
            // re-mesh while the screen shows: the terrain refreshes under
            // the player's eyes exactly like the original's chunk updates
            if let Some(w) = self.net_world.as_ref() {
                self.world_quads = Some(crate::mesher::mesh_world(w, None));
            }
        }
    }

    /// The live session ended (a server kick or a transport error) while
    /// the client was IN the network world: the vanilla kick lands on
    /// the DisconnectedScreen and the level unloads exactly like Save
    /// and Quit. A dead session nobody adopted (a cancelled join's
    /// transport error) drops silently — there is no world to lose.
    pub fn live_session_lost(&mut self, reason: String) {
        if !self.world_is_network {
            return;
        }
        self.disconnect_title =
            tr("disconnect.lost").unwrap_or("Connection lost").to_string();
        self.disconnect_reason = reason;
        self.world_quads = None;
        self.world_is_network = false;
        self.net_camera = None;
        self.net_biome = None;
        self.close_all_live_runs();
        self.open(ScreenKind::Disconnect);
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

    /// The World screen camera. NETWORK world with a shipped pose: the
    /// join teleport's EXACT pose — the vanilla first person at the
    /// player's feet + the standing eye height, looking the way the
    /// server directed (`world_view::snapshot_camera` converted the
    /// vanilla angle conventions at materialization time, from the SAME
    /// window decisions that placed the grid — pose and terrain can never
    /// disagree about where the player stands). PROCEDURAL world (and any
    /// network world without a pose): the deterministic creative-flight
    /// camera above the spawn surface facing east (+X — the direction the
    /// official 26.x sun rises), the exact camera the headless pixel
    /// tests pin.
    fn world_camera(&self) -> crate::camera::Camera {
        if self.world_is_network {
            if let Some(pose) = self.net_camera {
                return pose;
            }
        }
        let n = self.world_size as f32;
        // Creative-flight camera: 12 blocks above the spawn surface clears
        // the canopy so the official sky (sun/moon/stars) stays in frame.
        crate::camera::Camera {
            pos: Vec3::new(n / 2.0, self.spawn_y as f32 + 12.0 + crate::world_view::PLAYER_EYE_HEIGHT, n / 2.0),
            yaw: std::f32::consts::FRAC_PI_2, // face +X (east): sun visible at dawn
            pitch: -0.08,
        }
    }

    /// The in-world 3D view: first-person camera (the network join's
    /// exact teleport pose, or the deterministic flight camera for the
    /// procedural world), official day-night sky (timeline data +
    /// celestial textures), textured quads from the generated atlas, fog
    /// in the official color — the exact same raster path the headless
    /// pixel tests exercise.
    fn render_world_3d(&mut self, fb: &mut crate::raster::Framebuf) {
        self.ensure_world();
        let n = self.world_size as f32;
        let cam = self.world_camera();
        let day_time = (self.day_time_base + self.frame) as i64;
        // the END sky's flash state ticks on the client's game-time clock
        // (vanilla ticks ClientLevel.endFlashState on the level's game
        // time); tick() is idempotent per tick, so the per-frame call is
        // the vanilla cadence under the frame == tick model
        self.end_flash.tick(day_time);
        let sky = crate::sky::sample(day_time);
        // the fog: the dimension's STATED pair (the nether's 10/96 — the
        // fogged walls) when one rides the level entry, else the window
        // treatment (the bytecode defaults 0/1024 are beyond the window)
        let (fog_near, fog_far) = self.world_fog.unwrap_or((n * 0.55, n * 1.7));
        let ctx = RenderCtx {
            view: cam.view(),
            proj: Mat4::perspective(
                70.0f32.to_radians(),
                fb.w as f32 / fb.h as f32,
                0.1,
                512.0,
            ),
            fog_color: sky.fog_color,
            fog_near,
            fog_far,
        };
        // Official render order: sky color -> sun/moon/stars -> terrain ->
        // clouds (depth-tested against terrain, the CLOUDS pass runs with
        // the main depth buffer in the game's frame graph) -> the HAND
        // (vanilla renders the first-person arm after the level with a
        // cleared depth buffer, before any HUD).
        fb.clear(sky.sky_color, sky.fog_color);
        crate::sky::render_celestials(fb, &ctx, &cam, day_time, self.world_skybox, &self.end_flash);
        if let Some(quads) = &self.world_quads {
            fb.draw_quads(&ctx, quads);
        }
        crate::sky::render_clouds(fb, &ctx, &cam, day_time, 0.0);
        if self.arm.is_none() {
            let kind = if self.slim_arm {
                crate::hand::ArmKind::Slim
            } else {
                crate::hand::ArmKind::Wide
            };
            self.arm = crate::hand::load_kind(std::path::Path::new("assets-cache/jar"), kind);
        }
        // vanilla submitArmWithItem: a non-empty main hand draws the ITEM
        // instead of the arm; the empty hand draws the arm. Exactly one of
        // the two per frame.
        if let Some(item) = &self.held_item {
            item.draw(fb, ctx.proj, self.swing.get_attack_anim(0.5));
        } else if let Some(arm) = &self.arm {
            arm.draw(fb, ctx.proj, self.swing.get_attack_anim(0.5));
        }
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
    fn the_login_handshake_speaks_the_generated_manifest() {
        // the wire protocol number the window worker passes to the login
        // is `mc_protocol::generated::pack_claim::PROTOCOL_VERSION` (the
        // manifest emitter REFUSES a jar that does not state the number —
        // it can never be the -1 "unknown" the build_info stamp may
        // legally carry). build_info.rs is a SECOND generated surface for
        // the same jar fact (the full `generate` flow's stamp, from the
        // v0.2.0 era), so the two are CROSS-PINNED here: if either
        // generator flow is ever re-run against a different jar or the
        // emissions drift, this test fails loudly instead of the wire
        // quietly speaking two truths (the timeline-snapshot pattern).
        assert_eq!(
            crate::build_info::PROTOCOL_VERSION,
            mc_protocol::generated::pack_claim::PROTOCOL_VERSION,
            "the two generated surfaces for the jar's protocol_version agree"
        );
        // the manifest's number is a REAL stated one, not a fallback
        assert!(
            mc_protocol::generated::pack_claim::PROTOCOL_VERSION > 0,
            "the manifest emitter refuses unstated numbers — this is 26.2's 776"
        );
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
                    light: None,
                    day_tick: None, // test scenes stay day
                    biomes: None, // test scenes carry no biome layer
                    variants: None, // test scenes carry no state variants
            ambient_light: None, // no dimension opinion — the black bytecode default
                },
                camera: None,
                spawn_biome: Some("minecraft:plains".into()),
                slim_arm: false,
                skybox: Skybox::Overworld,
                fog: None,
            }),
        );
        assert_eq!(app.screen, ScreenKind::World, "the join lands in the network world");
        assert!(app.world_is_network);
        assert_eq!(app.world_size, 32);
        assert_eq!(app.net_biome.as_deref(), Some("minecraft:plains"), "the spawn biome rides the adoption");
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
            // the introductions: login acknowledged, client information,
            // plugin brand — ids from the GENERATED tables (all three
            // kept their wiki-era numbers; now they're pinned to the
            // runtime truth instead of remembered constants)
            for expected in [
                mc_protocol::generated::packet_ids::LSB_LOGIN_ACKNOWLEDGED,
                mc_protocol::generated::packet_ids::CSB_CLIENT_INFORMATION,
                mc_protocol::generated::packet_ids::CSB_CUSTOM_PAYLOAD,
            ] {
                let pkt = conn.read_packet().unwrap();
                let mut c: &[u8] = &pkt;
                assert_eq!(read_varint(&mut c).unwrap(), expected, "introduction id");
            }
            // Registry Data (a real vanilla server ALWAYS streams it in
            // the configuration phase): the shared mock fixture's
            // dimension_type registry (overworld -64/384) — the same
            // bytes the mc-protocol arc mock sends. Without it the join's
            // snapshot carries no layout knowledge and the world window
            // honestly falls back to the column-local copy. The BIOME
            // registry rides alongside (one entry: swamp, id 0) — the
            // synced name table the chunk biome palettes resolve through.
            conn.send_packet(&mc_protocol::config::test_support::registry_packet(true))
                .unwrap();
            conn.send_packet(&mc_protocol::config::test_support::biome_registry_packet(
                "minecraft:swamp",
            ))
            .unwrap();
            // probes + finish — ids from the GENERATED configuration
            // table (4/5/3 on the real 26.2 wire; the wiki-era literals
            // these replaced were one low)
            let mut ka = Vec::new();
            write_varint(&mut ka, mc_protocol::config::clientbound::KEEP_ALIVE);
            ka.extend_from_slice(&0x1122334455667788u64.to_be_bytes());
            conn.send_packet(&ka).unwrap();
            let mut pg = Vec::new();
            write_varint(&mut pg, mc_protocol::config::clientbound::PING);
            pg.extend_from_slice(&77i32.to_be_bytes());
            conn.send_packet(&pg).unwrap();
            let mut fin = Vec::new();
            write_varint(&mut fin, mc_protocol::config::clientbound::FINISH);
            conn.send_packet(&fin).unwrap();
            // responses: keep-alive echo, pong, ack — ids from the same
            // generated truth
            let resp = conn.read_packet().unwrap();
            let mut c: &[u8] = &resp;
            assert_eq!(
                read_varint(&mut c).unwrap(),
                mc_protocol::config::serverbound::KEEP_ALIVE,
                "keep alive response id"
            );
            assert_eq!(i64::from_be_bytes(c[..8].try_into().unwrap()), 0x1122334455667788);
            let resp = conn.read_packet().unwrap();
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), mc_protocol::config::serverbound::PONG, "pong id");
            assert_eq!(i32::from_be_bytes(c[..4].try_into().unwrap()), 77);
            let resp = conn.read_packet().unwrap();
            let mut c: &[u8] = &resp;
            assert_eq!(
                read_varint(&mut c).unwrap(),
                mc_protocol::config::serverbound::ACKNOWLEDGE_FINISH,
                "ack finish id"
            );
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
            write_varint(&mut buffer, 0); // the synced biome: swamp (id 0)
            // a real vanilla join streams the FULL column (24 sections for
            // an overworld): 7 air sections below put the stone/dirt
            // section at index 7 — world y 48..63 with min_y -64, surface
            // terrain right under the teleport's feet y 64. The
            // player-anchored window (base 48 = 64 - 16) then shows the
            // stone at grid y 0 — a bottom-anchored copy would miss it.
            // An 8th air section (index 8, world y 64..79) sits UNDER THE
            // PLAYER: its biome cells are what the music situation reads.
            let mut air = Vec::new();
            air.extend_from_slice(&0i16.to_be_bytes()); // block count 0
            air.push(0); // bits per entry: single valued
            write_varint(&mut air, 0); // the single value: air
            air.push(0); // biomes: single valued
            write_varint(&mut air, 0); // the synced biome: swamp (id 0)
            let mut column = Vec::new();
            for _ in 0..7 {
                column.extend_from_slice(&air); // sections arrive bottom-first
            }
            column.extend_from_slice(&buffer);
            column.extend_from_slice(&air); // section 8: the player's own
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
            // floor(-27.25)>>4 = -2. The javap-proven 26.2 shape:
            // teleport id FIRST, then the PositionMoveRotation (position
            // + delta movement Vec3s + yaw/pitch), then the int flags.
            let mut pos = Vec::new();
            write_varint(&mut pos, clientbound::SYNCHRONIZE_POSITION);
            write_varint(&mut pos, 42);
            pos.extend_from_slice(&56.5f64.to_be_bytes());
            pos.extend_from_slice(&64.0f64.to_be_bytes());
            pos.extend_from_slice(&(-27.25f64).to_be_bytes());
            pos.extend_from_slice(&0.0f64.to_be_bytes()); // delta movement
            pos.extend_from_slice(&0.0f64.to_be_bytes());
            pos.extend_from_slice(&0.0f64.to_be_bytes());
            pos.extend_from_slice(&90.0f32.to_be_bytes());
            pos.extend_from_slice(&0.0f32.to_be_bytes());
            pos.extend_from_slice(&0i32.to_be_bytes()); // all values absolute
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

        // the exact mapping the window worker performs: the milestone
        // adopts into the LIVE view and the first refresh IS the one-shot
        // snapshot view (the example mapper — the ids the mock streams;
        // the GUI resolves through the GENERATED registry table) — plus
        // the teleport's exact camera pose from the same decisions
        let res = mc_protocol::connect_and_join_world(
            &req.host,
            req.port,
            mc_protocol::generated::pack_claim::PROTOCOL_VERSION,
            crate::screens::PLAYER_NAME,
        )
        .map(|out| match out {
            mc_protocol::WorldOutcome::Loaded(snap) => {
                let (entity_id, view_distance, chunks) =
                    (snap.info.entity_id, snap.info.view_distance, snap.chunks.len());
                // the worker's exact resolution order: the skybox resolves
                // through the model the snapshot adopts into (the synced
                // registry's own skybox is the server's truth — the wire
                // decode; the codec default when the element states
                // nothing; the built-in table on a skip-sync)
                let view = crate::world_view::LiveView::from_snapshot(snap, WORLD_CHUNKS);
                let skybox = view.model.skybox();
                let fog = view.model.fog_distances();
                let first = view.refresh(|id| match id {
                    1 => crate::world::STONE,
                    10 => crate::world::DIRT,
                    _ => crate::world::AIR,
                });
                LoginResult::Loaded {
                    username: crate::screens::PLAYER_NAME.to_string(),
                    entity_id,
                    view_distance,
                    chunks,
                    world: first.world,
                    camera: first.camera,
                    spawn_biome: first.biome,
                    slim_arm: false,
                    skybox,
                    fog,
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
        // the camera is the teleport's EXACT pose in the grid frame:
        // feet (56.5, 64, -27.25) with the window origin (1,-4) and base
        // 48 -> grid (40.5, 16, 36.75) + the standing eye height; the
        // server's yaw 90 (facing west) converts to camera yaw 90+180;
        // pitch 0 stays 0. The world screen renders FROM the player.
        let cam = app.net_camera.expect("the join shipped the teleport pose");
        assert_eq!(cam.pos.x, 40.5);
        assert_eq!(cam.pos.y, 64.0 - 48.0 + crate::world_view::PLAYER_EYE_HEIGHT);
        assert_eq!(cam.pos.z, 36.75);
        assert_eq!(cam.yaw, 90.0f32.to_radians() + std::f32::consts::PI);
        assert_eq!(cam.pitch, 0.0);
        assert_eq!(
            app.world_camera(),
            cam,
            "the world screen renders FROM the teleport pose"
        );
        // the REAL spawn biome: the synced registry (one entry, swamp) +
        // the section under the player resolve to the situation's key,
        // and the music is that biome's official background-music event
        assert_eq!(
            app.net_biome.as_deref(),
            Some("minecraft:swamp"),
            "the biome at the player's feet, decoded from the sections"
        );
        use mc_registry::generated::worldgen as wm;
        let expect = wm::biome_music("minecraft:swamp", "default")
            .map(|m| m.event)
            .unwrap_or("music.game");
        assert_eq!(
            app.music.game_tracks(),
            [expect],
            "the join's music situation is the spawn biome's own event"
        );
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
            light: None,
            day_tick: None, // test scenes stay day
            biomes: None, // test scenes carry no biome layer
            variants: None, // test scenes carry no state variants
            ambient_light: None, // no dimension opinion — the black bytecode default
        });
        app.enter_network_world(None, Skybox::Overworld, None);
        assert_eq!(app.screen, ScreenKind::World);
        assert!(app.world_is_network);
        assert_eq!(app.world_size, 32);
        assert!(app.net_biome.is_none(), "no biome shipped, no situation claimed");
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
        assert!(app.net_camera.is_none(), "the teleport pose unloads with the level");
        assert!(app.net_biome.is_none(), "the biome situation unloads with the level");

        // singleplayer afterwards rebuilds the PROCEDURAL world
        app.render_title_buttons_for_test();
        let sp = app.buttons.iter().find(|b| b.id == "singleplayer").expect("singleplayer").clone();
        app.on_click(sp.x + sp.w / 2, sp.y + sp.h / 2);
        assert_eq!(app.screen, ScreenKind::World);
        assert!(!app.world_is_network, "singleplayer renders the procedural world");
        assert!(!app.world_quads.unwrap().is_empty(), "procedural terrain meshes");
    }

    #[test]
    fn world_camera_follows_the_teleport_and_falls_back_to_the_flight_camera() {
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        // a network world installed WITHOUT a pose (no join carried it):
        // the deterministic flight camera stands — never a guess about
        // where the player is
        app.net_world = Some(crate::world::World {
            chunks_per_side: 2,
            seed: 0,
            chunks: vec![
                vec![crate::world::AIR; crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z];
                4
            ],
            light: None,
            day_tick: None, // test scenes stay day
            biomes: None, // test scenes carry no biome layer
            variants: None, // test scenes carry no state variants
            ambient_light: None, // no dimension opinion — the black bytecode default
        });
        app.enter_network_world(None, Skybox::Overworld, None);
        let flight = app.world_camera();
        let n = app.world_size as f32;
        assert_eq!(
            flight.pos,
            Vec3::new(
                n / 2.0,
                app.spawn_y as f32 + 12.0 + crate::world_view::PLAYER_EYE_HEIGHT,
                n / 2.0
            ),
            "the flight camera over the spawn surface, eye 1.62 up"
        );
        assert_eq!(flight.yaw, std::f32::consts::FRAC_PI_2, "facing east at dawn");
        assert_eq!(flight.pitch, -0.08);
        // a REAL pose (what the join worker ships) takes over: the world
        // screen renders from the player, not from the stand-in
        let pose = crate::camera::Camera {
            pos: Vec3::new(40.5, 17.62, 36.75),
            yaw: 5.5,
            pitch: 0.1,
        };
        app.net_camera = Some(pose);
        assert_eq!(app.world_camera(), pose, "the teleport pose drives the camera");
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
                    light: None,
                    day_tick: None, // test scenes stay day
                    biomes: None, // test scenes carry no biome layer
                    variants: None, // test scenes carry no state variants
            ambient_light: None, // no dimension opinion — the black bytecode default
                },
                camera: None,
                spawn_biome: None,
                slim_arm: false,
                skybox: Skybox::Overworld,
                fog: None,
            }),
        );
        assert_eq!(
            app.screen,
            ScreenKind::Servers,
            "a cancelled join's late success is dropped"
        );
        assert!(app.net_world.is_none());
    }

    /// An all-air network window of `side` chunks per side (the state
    /// machine must not care about content) with a distinguishable seed.
    fn air_net_world(side: usize, seed: u64) -> crate::world::World {
        crate::world::World {
            chunks_per_side: side,
            seed,
            chunks: vec![
                vec![crate::world::AIR; crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z];
                side * side
            ],
            light: None,
            day_tick: None, // test scenes stay day
            biomes: None, // test scenes carry no biome layer
            variants: None, // test scenes carry no state variants
            ambient_light: None, // no dimension opinion — the black bytecode default
        }
    }

    #[test]
    fn live_view_updates_swap_the_network_world_in_place() {
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        // the join landed (an all-air window: the state machine must not care)
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None);
        assert!(app.world_is_network);
        assert!(app.world_quads.as_ref().unwrap().is_empty(), "air meshes to nothing");
        // no biome known: the vanilla in-game default situation
        assert_eq!(app.music.game_tracks(), ["music.game"]);

        // a live update ships a world WITH content + a pose + the biome
        // at the player: the swap is IN PLACE — net_world/net_camera
        // refresh, the showing screen re-meshes (the terrain changes
        // under the player's eyes, the vanilla chunk-update behavior)
        // and the music situation resolves to the biome's own event
        let mut streamed = air_net_world(2, 7);
        streamed.chunks[0][0] = crate::world::STONE;
        let pose = crate::camera::Camera {
            pos: Vec3::new(1.0, 2.0, 3.0),
            yaw: 0.5,
            pitch: -0.1,
        };
        app.live_view_updated(streamed, Some(pose), Some("minecraft:swamp".into()));
        let w = app.net_world.as_ref().expect("the streamed world swapped in");
        assert_eq!(w.seed, 7, "the update REPLACED the join world (newest wins)");
        assert_eq!(w.get(0, 0, 0), crate::world::STONE, "the streamed block shows");
        assert_eq!(app.net_camera, Some(pose), "the refreshed pose drives the camera");
        assert_eq!(app.world_camera(), pose, "world_is_network renders FROM the pose");
        assert!(
            !app.world_quads.as_ref().expect("re-meshed on the live update").is_empty(),
            "the streamed terrain re-meshed under the player"
        );
        // the biome change re-resolved the music situation — the swamp's
        // official background-music event, from the GENERATED table
        assert_eq!(app.net_biome.as_deref(), Some("minecraft:swamp"));
        use mc_registry::generated::worldgen as wm;
        let swamp_event = wm::biome_music("minecraft:swamp", "default")
            .map(|m| m.event)
            .unwrap_or("music.game");
        assert_eq!(app.music.game_tracks(), [swamp_event]);
        // a SAME-biome refresh never restarts the situation (the vanilla
        // behavior — the scheduler is not touched, its event log frozen)
        let events_before = app.music.events.len();
        app.live_view_updated(air_net_world(2, 8), Some(pose), Some("minecraft:swamp".into()));
        assert_eq!(app.net_biome.as_deref(), Some("minecraft:swamp"));
        assert_eq!(app.music.events.len(), events_before, "no situation restart");
        // an UNRESOLVABLE biome (None) keeps what is playing — no
        // knowledge is not a change
        app.live_view_updated(air_net_world(2, 9), Some(pose), None);
        assert_eq!(app.net_biome.as_deref(), Some("minecraft:swamp"));
        assert_eq!(app.music.events.len(), events_before);
        // and a real change re-fires once
        app.live_view_updated(air_net_world(2, 10), Some(pose), Some("minecraft:desert".into()));
        assert_eq!(app.net_biome.as_deref(), Some("minecraft:desert"));
        let desert_event = wm::biome_music("minecraft:desert", "default")
            .map(|m| m.event)
            .unwrap_or("music.game");
        assert_eq!(app.music.game_tracks(), [desert_event]);
    }

    #[test]
    fn live_updates_drop_after_save_and_quit() {
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(Some("minecraft:swamp".into()), Skybox::Overworld, None);
        assert_eq!(app.net_biome.as_deref(), Some("minecraft:swamp"));
        // Save and Quit unloads the level AND closes the live session
        let run = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        app.attach_live_run(1, std::sync::Arc::clone(&run));
        app.paused = true;
        app.render_title_buttons_for_test();
        let quit = app.buttons.iter().find(|b| b.id == "saveandquit").expect("saveandquit").clone();
        app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
        assert_eq!(app.screen, ScreenKind::Title);
        assert!(!run.load(std::sync::atomic::Ordering::Relaxed), "the session's flag flipped");
        assert!(app.live_runs.is_empty(), "the closed session's entry is gone");

        // a LATE live update (the worker raced its final refresh): the
        // world the UI no longer shows — dropped honestly, nothing
        // re-meshes, the title screen stays
        let mut late = air_net_world(2, 9);
        late.chunks[0][0] = crate::world::STONE;
        app.live_view_updated(
            late,
            Some(crate::camera::Camera {
                pos: Vec3::new(1.0, 2.0, 3.0),
                yaw: 0.0,
                pitch: 0.0,
            }),
            Some("minecraft:plains".into()),
        );
        assert_eq!(app.screen, ScreenKind::Title, "the late update never opens a screen");
        assert_eq!(
            app.net_world.as_ref().expect("net_world dormant, not cleared").seed,
            0,
            "the stale join world is untouched by the dropped update"
        );
        assert!(app.world_quads.is_none(), "no mesh reappeared");
        assert!(app.net_camera.is_none(), "no pose reappeared");
        assert!(app.net_biome.is_none(), "no situation reappeared for a dead world");
        // a late session death drops equally silently
        app.live_session_lost("late kick".to_string());
        assert_eq!(app.screen, ScreenKind::Title, "no world to lose — no disconnect screen");
    }

    #[test]
    fn live_session_lost_kicks_to_the_disconnect_screen() {
        // IN the network world: a session death is the vanilla kick
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(Some("minecraft:swamp".into()), Skybox::Overworld, None);
        let run = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        app.attach_live_run(1, run);
        app.live_session_lost("Server closed".to_string());
        assert_eq!(app.screen, ScreenKind::Disconnect);
        assert_eq!(app.disconnect_reason, "Server closed");
        assert_eq!(
            app.disconnect_title,
            tr("disconnect.lost").unwrap_or("Connection lost"),
            "the vanilla lost-connection title"
        );
        assert!(!app.world_is_network, "the level unloaded with the session");
        assert!(app.world_quads.is_none(), "the mesh dropped with the level");
        assert!(app.net_camera.is_none(), "the pose dropped with the level");
        assert!(app.net_biome.is_none(), "the biome situation dropped with the level");
    }

    #[test]
    fn the_world_skybox_pins_at_level_entry_and_rides_the_dimension() {
        // the vanilla SkyRenderState.skybox discipline: the extract pins
        // the dimension's skybox at level entry, the sky pipeline gates
        // on it, and re-entering the PROCEDURAL world re-pins Overworld
        let mut app = App::new(3);
        assert_eq!(app.world_skybox, Skybox::Overworld, "the App default");
        assert_eq!(app.world_fog, None, "no fog pair: the window treatment");
        // a nether join ships Skybox::None: the sky pipeline renders no
        // sky state at all there (the extract returns before any exists)
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        // the nether's real 10/96 pins with the join (the fogged walls)
        app.enter_network_world(None, Skybox::None, Some((10.0, 96.0)));
        assert!(app.world_is_network);
        assert_eq!(app.world_skybox, Skybox::None, "the nether pin");
        assert_eq!(app.world_fog, Some((10.0, 96.0)), "the stated pair pins");
        // a procedural re-entry re-pins the overworld sky AND drops the
        // fog pair (the extract re-runs on every level entry — a network
        // nether never haunts the singleplayer world's sky or fog)
        app.enter_world();
        assert!(!app.world_is_network);
        assert_eq!(app.world_skybox, Skybox::Overworld, "the procedural re-pin");
        assert_eq!(app.world_fog, None, "the pair drops with the level");
        // the resolution discipline itself lives in the model: the
        // synced registry's own skybox, the codec default, or the
        // built-in table (WorldModel::skybox — pinned by the
        // world_model tests, the built-in verdicts included)
    }

    #[test]
    fn live_run_flags_close_on_cancel_supersede_and_quit() {
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        // two joins dispatched (the frame thread attaches a flag each)
        let run_a = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let run_b = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        app.attach_live_run(1, std::sync::Arc::clone(&run_a));
        app.attach_live_run(2, std::sync::Arc::clone(&run_b));
        // join A's success arrives while the Connect screen shows: A is
        // THE session, B (superseded) closes
        app.login_requests.push(LoginRequest { id: 1, entry: 0, host: "h".into(), port: 1 });
        app.login_requests.push(LoginRequest { id: 2, entry: 0, host: "h".into(), port: 2 });
        app.login_finished(
            1,
            Ok(LoginResult::Loaded {
                username: crate::screens::PLAYER_NAME.to_string(),
                entity_id: 1,
                view_distance: 8,
                chunks: 1,
                world: air_net_world(2, 0),
                camera: None,
                spawn_biome: None,
                slim_arm: true,
                skybox: Skybox::Overworld,
                fog: None,
            }),
        );
        assert!(app.world_is_network, "A adopted");
        // the profile-metadata consumer: the join's skin model IS the
        // first-person arm's selection (the wide --slim-arm stand-in is
        // overridden by a real login's textures metadata)
        assert!(app.slim_arm, "the slim profile rides the join onto the app");
        assert!(
            run_a.load(std::sync::atomic::Ordering::Relaxed),
            "the adopted session keeps running"
        );
        assert!(
            !run_b.load(std::sync::atomic::Ordering::Relaxed),
            "the superseded session closed"
        );
        assert_eq!(app.live_runs.len(), 1, "only the adopted flag remains");

        // Save and Quit closes the adopted session too
        app.paused = true;
        app.render_title_buttons_for_test();
        let quit = app.buttons.iter().find(|b| b.id == "saveandquit").expect("saveandquit").clone();
        app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
        assert!(
            !run_a.load(std::sync::atomic::Ordering::Relaxed),
            "the quit closed the network session"
        );

        // a cancelled join's late success closes ITS session (C), and
        // nothing else is disturbed
        let run_c = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        app.attach_live_run(3, std::sync::Arc::clone(&run_c));
        app.open(ScreenKind::Connect);
        app.on_escape(); // the user cancelled while C was in flight
        assert_eq!(app.screen, ScreenKind::Servers);
        app.login_requests.push(LoginRequest { id: 3, entry: 0, host: "h".into(), port: 3 });
        app.login_finished(
            3,
            Ok(LoginResult::Loaded {
                username: crate::screens::PLAYER_NAME.to_string(),
                entity_id: 3,
                view_distance: 8,
                chunks: 1,
                world: air_net_world(2, 3),
                camera: None,
                spawn_biome: None,
                slim_arm: false,
                skybox: Skybox::Overworld,
                fog: None,
            }),
        );
        assert!(
            !run_c.load(std::sync::atomic::Ordering::Relaxed),
            "the cancelled join's session closed"
        );
        assert_eq!(app.screen, ScreenKind::Servers, "the late success is still dropped");
        assert_eq!(
            app.net_world.as_ref().expect("dormant from the earlier join").seed,
            0,
            "nothing adopted from a cancelled join (the old world stays dormant)"
        );
    }

    impl App {
        fn render_title_buttons_for_test(&mut self) {
            let mut fb = crate::raster::Framebuf::new(320, 180);
            let mut assets = GuiAssets::open("assets-cache/jar");
            self.render(&mut fb, &mut assets);
        }
    }
}

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
        /// The dimension's STATED cloud-fog end (blocks) — `None` when
        /// no dimension states one, the consumer's 2048.0 bytecode
        /// default's shape (see `WorldModel::cloud_fog_end_distance`).
        cloud_fog_end: Option<f32>,
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
    /// The container GUI's SCREEN round: the vanilla GenericContainerScreen
    /// (generic_9x3) over the frozen world — the 9x3 grid mapping, the real
    /// `container/slot` insets, the flat item icons, the vanilla counts.
    /// The OFFLINE deterministic stand-in; the networked store wiring is
    /// the next round's own line.
    Chest,
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
    /// The LIVE store's hotbar row — the HUD's icons and counts are
    /// the store's projection (the vanilla renderHotbar's own lane,
    /// `USE_ROW_SLOT_START..USE_ROW_SLOT_END` verbatim, the `(0, 0)`
    /// slots the emptiness the wire itself named). The OFFLINE world
    /// carries no store — the row stays empty and the hotbar renders
    /// its chrome only (the honest shape the goldens pin).
    pub hotbar_slots: [(i32, i32); 9],
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
    /// THE BLOCK-SOUND LANE (sound.rs): the deterministic one-shot engine
    /// the interaction lanes speak — the dig beat's hit, the completion's
    /// break, the placement, the footsteps — the vanilla folds (the
    /// javap-quoted volume/pitch constants) over the generated SoundType
    /// table, the weighted official file pick, the event log the audio
    /// backend mirrors ("block.stone.hit @<tick> #<file> v<vol> p<pitch>").
    pub sounds: crate::sound::BlockSounds,
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
    /// The dimension's STATED cloud-fog end (blocks) — pinned at level
    /// entry like the fog pair; `None` = no dimension states one (the
    /// cloud fog rides the 2048.0 bytecode default, `sky::DEFAULT_CLOUD_FOG_END`).
    pub world_cloud_fog_end: Option<f32>,
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
    /// The live USE arc (the vanilla LocalPlayer.isUsingItem's own
    /// shape): the item the use started with, the elapsed ticks (the
    /// game-tick clock the swing lane already speaks) and the loaded
    /// state index (the reload fires only when the charge's selected
    /// state changes — the same no-op-reload law live_held_updated
    /// speaks). `None` = not using (the resting payload renders).
    pub using_item: Option<UseActive>,
    /// The held stack's CHARGED_PROJECTILES component — the vanilla
    /// ItemStack's own component state, predicted locally (the client
    /// has no server component stream for the offline lane; the same
    /// deterministic stand-in `--held-item` speaks for inventory
    /// state). `Some(kind)` = the crossbow carries a loaded projectile
    /// (the resting render rides the charge_type select's case — the
    /// crossbow_arrow / crossbow_firework bolt); `None` = the plain
    /// resting def. Cleared when the held item changes (the component
    /// belongs to the STACK, never the slot).
    pub held_charge: Option<mc_registry::ChargeKind>,
    /// The OFFLINE procedural world the world screen owns — the
    /// singleplayer lane's own terrain (the App used to keep only its
    /// mesh; the block-use lane's placement prediction needs the VOXELS
    /// to edit). `None` = the networked world owns the screen (its
    /// placement is the server's own chunk refresh — the wire echo's
    /// authority), or no world is entered at all.
    pub offline_world: Option<crate::world::World>,
    /// The OFFLINE held stack's wire id — the `--held-item` model id's
    /// own route into the interaction domain (the press lanes read the
    /// unified held id; the OFFLINE stand-in for the store's selected
    /// hotbar stack). `None` = the empty selection.
    pub offline_held: Option<i32>,
    /// The OFFLINE world screen's own camera pose — the look-control
    /// lane's seam (the net side rides the teleport's net_camera; the
    /// offline side's pose lands HERE, the flight camera stays the
    /// computed fallback until a pose arrives). The placement lane's
    /// clip aims through it.
    pub offline_camera: Option<crate::camera::Camera>,
    /// THE FIRST-PERSON BODY — the vanilla player's own look, input,
    /// travel and collision lane (body.rs, the javap laws).
    /// `None` = the deterministic camera discipline (the headless
    /// scenes and the pixel goldens never feed a body — the flight
    /// camera stands untouched). The WINDOWED app materializes it when
    /// the world screen stands (the offline spawn pose, or the join
    /// teleport's pose through live_view_updated's re-anchor), feeds
    /// the mouse deltas + the movement keys per frame, and the tick
    /// rides update()'s frame==tick model — the wire echoing through
    /// the same command channel the use arc rides.
    pub body: Option<crate::body::PlayerBody>,
    /// THE F5 PERSPECTIVE (the vanilla `options.cameraType`): the F5
    /// key cycles it (`Minecraft.handleKeybinds`'s own
    /// `keyTogglePerspective.consumeClick()` lane) and the camera +
    /// render gates speak it — the first-person hand and the
    /// crosshair sleep in the third-person views, the AVATAR (the
    /// body's own model) draws instead (the LevelRenderer's detached
    /// camera entity law). The default is the vanilla default
    /// FIRST_PERSON — the deterministic scenes and the pixel goldens
    /// never press F5, so every first-person frame stays bit-for-bit.
    pub perspective: crate::camera::CameraType,
    /// THE THIRD-PERSON AVATAR (the body's own render — avatar.rs, the
    /// vanilla HumanoidModel/PlayerModel laws), loaded lazily from the
    /// runtime asset root on the first third-person frame; `None` =
    /// the skin asset is absent or failed its sha1 verification and
    /// the third-person body draws nothing.
    pub avatar: Option<crate::avatar::AvatarModel>,
    /// THE DIG ARC's state machine — the vanilla `MultiPlayerGameMode`'s
    /// destroy half (the survival lane: the destroy fields, the progress
    /// law, the START/ABORT/STOP sends). The left mouse speaks it: the
    /// press arms (startDestroyBlock), the hold mines
    /// (continueDestroyBlock per tick), the release aborts
    /// (stopDestroyBlock) — the crack overlay and the wire ride along.
    pub dig: crate::dig::DigState,
    /// The left mouse button's HOLD state (the vanilla
    /// `options.keyAttack.isDown()`): the rising edge set by on_press,
    /// the falling edge cleared by on_left_release — the dig tick reads
    /// it (continueAttack(true) runs while held).
    pub dig_held: bool,
    /// The vanilla `missTime` — the 10-tick anti-spam window an AIR
    /// swing arms (survival's hasMissTime): both the press and the hold
    /// return while it counts (startAttack/continueAttack's own gates).
    pub miss_time: i32,
    /// The frame's crosshair pick — the vanilla `hitResult` (the
    /// GameRenderer.pick every frame speaks). The dig tick re-clips it
    /// per tick (the frame==tick model), the attack press reads it, and
    /// the world render draws the hit outline from it. `None` = the
    /// pick spoke MISS (nothing solid within the 4.5-block reach).
    pub frame_pick: Option<crate::world::ClipHit>,
    /// The chest screen's 27 container slots as (item_id, count) — the
    /// wire's own stack shape (count <= 0 is the empty stack), the FIXED
    /// deterministic loadout the pixel test pins. Filled by open_chest
    /// (the OFFLINE stand-in) and by the live container lane (the
    /// networked store's projection, the menu's own numbering).
    pub chest_slots: Vec<(i32, i32)>,
    /// The chest screen's player-inventory mirror (36 slots as
    /// (item_id, count)) — the networked screen's rows 27..63 of the
    /// menu's own numbering (27 main + 9 hotbar, the generic_54
    /// canvas's layout). The OFFLINE stand-in keeps it EMPTY (the
    /// chrome-only rows the golden pins); the live lane fills it from
    /// the same store projection the grid rides.
    pub chest_mirror: Vec<(i32, i32)>,
    /// Whether the chest screen currently shows the NETWORKED store
    /// (the live lane opened it) — the offline deterministic stand-in
    /// (open_chest, the pixel test's own scene) is never closed by the
    /// live lane's container-less updates, the two flows stay separate.
    pub chest_is_live: bool,
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
    /// The container command lane into the LIVE session's worker: the
    /// frame thread's slot clicks and the player's own close echo ride
    /// this channel (the worker drains it each loop and speaks the
    /// wire). Attached per dispatched join, replaced by the next one,
    /// cleared with the session (Save and Quit, a kick) — a command
    /// with no lane dies honestly on the frame thread.
    pub live_clicks: Option<std::sync::mpsc::Sender<crate::world_view::ContainerCommand>>,
    /// The LIVE window's identity the clicks echo against: (window id,
    /// the revision the client last saw). `Some` while a chest menu
    /// from the store is up, `None` with the offline stand-in and the
    /// closed lanes — a click without a window is the honest no-op.
    pub chest_live_window: Option<(i32, i32)>,
    /// The chest slot hit boxes the LAST render laid out: (x, y, the
    /// MENU's own slot number) — the render-time geometry the clicks
    /// hit-test against (the buttons' own pattern: the frame renders
    /// the layout, the press finds it). The 16x16 inner box only (the
    /// vanilla Slot's own hit shape, the inset sprite's inside).
    pub chest_slot_rects: Vec<(i32, i32, i16)>,
    /// The client's BELIEVED cursor stack as (item_id, count) — the
    /// local prediction's own lane. The authoritative carried (the
    /// store's projection) lands it on every container update; the
    /// player's clicks mutate it between corrections (the vanilla
    /// pickup prediction — see [`crate::world_view::predict_pickup`]);
    /// the close/drop resets it. The (0, 0) sentinel is the emptiness.
    pub chest_carried: (i32, i32),
    /// The mouse position in GUI units the LAST loop tick tracked —
    /// the carried stack renders here (the vanilla cursor-follow).
    /// `None` wherever no live mouse has moved (the headless scenes:
    /// the pixel goldens never see the render).
    pub mouse_pos: Option<(i32, i32)>,
    /// The keyboard lane's shift state the LAST loop tick tracked —
    /// the vanilla `Screen.hasShiftDown()` query: the press's rising
    /// edge reads it to route the shift-click's QUICK_MOVE lane. Fed
    /// per-frame by the windowed loop; the headless scenes never feed
    /// it (the goldens never see the lane).
    pub shift_down: bool,
    /// The live held-item lane's currently-LOADED wire id (the reload
    /// key): the store's selected hotbar stack projects the
    /// first-person hand, and the model reloads only when the id
    /// changes (the count is the GUI's business — the hand renders
    /// the model alone, vanilla renders no count in first person).
    /// `None` = the bare arm (the empty selection, or an id outside
    /// the bake domain).
    pub live_held: Option<i32>,
    /// THE DROP LANE — the survival break's item entities (the vanilla
    /// `ItemEntity` arc, drops.rs): the popResource spawn, the physics
    /// tick, the neighbour merge, the 6000-tick despawn, the pickup
    /// scan. The OFFLINE lane's own authority (the networked world's
    /// entities stay the server's — the voxel prediction's own split).
    pub drops: Vec<crate::drops::ItemDrop>,
    /// The OFFLINE gameplay inventory (the vanilla `Inventory`'s own 36
    /// slots — the hotbar 0..8, the main 9..35; the (id, count) stack
    /// shape the HUD's projection speaks). The pickups land here; the
    /// placement consumes the selected stack; the networked world keeps
    /// the store's own projection (this lane sleeps).
    pub inventory: crate::drops::Inventory,
    /// The drop lane's own deterministic generator (the spawn spreads,
    /// the loot rolls — drops.rs's LCG, the audio lane's own
    /// architecture).
    pub drop_rng: crate::drops::Lcg,
    /// The offline entity id counter (the settle law's phase partner).
    pub next_drop_id: i32,
    /// THE HURT/HEALTH ARC — the vanilla player's own vitals (the
    /// javap laws, the hurt round): the health float the hearts speak
    /// (the entity data's clamp 0..=max), the food/saturation the
    /// Set Health packet carries, and the LivingEntity hurt fields
    /// verbatim (`hurtTime`/`hurtDuration` the flash drives,
    /// `invulnerableTime` the 20-tick cooldown folds, `lastHurt` the
    /// cooldown's own comparison — the vanilla hurtServer's own
    /// pipeline, the LOCAL prediction's voice).
    pub health: f32,
    /// THE FOOD LANE (javap FoodData, the food.rs module's own laws):
    /// the bar/saturation/exhaustion/timer quartet — the wire's Set
    /// Health lands the server's truth, the OFFLINE lane's stand-in
    /// server ticks it (the fold, the double-lane regen, the starve).
    pub food_data: crate::food::FoodData,
    /// The Hud.random's stand-in (the food row's saturation shimmy
    /// draws — the free-running RandomSource the vanilla speaks, the
    /// client's deterministic Lcg the audio/drop lanes' own
    /// architecture).
    pub food_rng: crate::drops::Lcg,
    /// THE EAT LANE (the consumable's own hold — the vanilla's
    /// `useItem`/`useItemRemaining` narrowed to the food): opened by
    /// the right press's canConsume gate, ticked by the use clock (the
    /// chew bursts at the cadence's own stride), completed at zero or
    /// cancelled by the release. Mutually exclusive with the pull lane
    /// — the vanilla's one `isUsingItem` at a time.
    pub eating: Option<crate::eat::EatActive>,
    /// THE ITEM PIECES (the eat arc's BreakingItemParticles — the
    /// mouth's own debris, the spawnItemParticles' law the eat lane
    /// speaks; the pieces tick with the world, the lifetime's removal
    /// prunes).
    pub pieces: Vec<crate::eat::ItemPiece>,
    pub hurt_time: i32,
    pub hurt_duration: i32,
    pub invulnerable_time: i32,
    pub last_hurt: f32,
    /// THE STATUS-EFFECT STORE (the effects.rs module's own laws — the
    /// add/update/compare chain the wire and the eat arc ride, the
    /// absorption value the white hearts row speaks): the OFFLINE lane's
    /// stand-in server applies the eat's payloads and ticks the
    /// server-side arc (the regen beat, the tick-down, the expiry's
    /// removal); the NETWORKED lane mirrors the model's own-effect
    /// store (the wire's Update/Remove Mob Effect packets' filtered
    /// truth) and ticks the client copy's durations the blink law reads.
    pub effects: crate::effects::EffectsStore,
    /// The Hurt Animation packet's tilt yaw (the flash-and-lean's own
    /// channel; the camera roll is the future's consumer).
    pub hurt_yaw: f32,
    /// The HUD's health-row state (the Hud's own fields, the tick
    /// clock the frame==tick model speaks): `healthBlinkTime` the
    /// 20-tick flash window arms, `lastHealth`/`displayHealth` the
    /// lerp pair (the display snaps after the 1000ms idle, folded to
    /// 20 ticks), `lastHealthTime` the idle clock.
    pub health_blink_time: u64,
    pub last_health: i32,
    pub display_health: i32,
    pub last_health_time: u64,
    /// The death screen's state: the vanilla shows the DeathScreen
    /// when the health reaches zero (the Gui.tick's own auto-open) —
    /// `dead` gates the HUD off and the gradient + the Respawn
    /// buttons on; the message the Combat Kill wire named (the local
    /// lane's fall death rides the client's own title).
    pub dead: bool,
    pub death_message: Option<String>,
    /// The spawn pose the body materialized at (the offline spawn law's
    /// own feet/angles, the network teleport's pose) — the respawn's
    /// own re-anchor (the vanilla respawn teleports the player to the
    /// spawn point and the body re-baselines there).
    pub spawn_pose: Option<(f64, f64, f64, f32, f32)>,
}

/// One active USE — the vanilla LocalPlayer's own using fields (the
/// used item, the elapsed ticks). `state` is the client's reload key:
/// the using branch's selected state index.
#[derive(Debug, Clone, PartialEq)]
pub struct UseActive {
    pub item: i32,
    pub ticks: u32,
    pub state: usize,
}

/// World size for the in-game screen (chunks per side — same as the
/// headless golden world so the pixel test stays deterministic).
pub const WORLD_CHUNKS: usize = 5;

const HOTBAR_W: i32 = 182;
const HOTBAR_H: i32 = 22;
const HEART: i32 = 9;
const HEART_SPACING: i32 = 8;
pub const HEALTH: i32 = 20;
/// `Attributes.MAX_HEALTH`'s default — the setHealth clamp's own
/// ceiling (javap LivingEntity.setHealth: `Mth.clamp(health, 0,
/// maxHealth)`).
pub const MAX_HEALTH: f32 = 20.0;

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
            sounds: crate::sound::BlockSounds::new(seed),
            paused: false,
            world_quads: None,
            world_size: 0,
            spawn_y: 0,
            options_return: ScreenKind::Title,
            selected_hotbar: 0,
            hotbar_slots: [(0, 0); 9],
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
            world_cloud_fog_end: None,
            net_biome: None,
            live_runs: Vec::new(),
            arm: None,
            slim_arm: false,
            swing: crate::hand::SwingState::default(),
            held_item: None,
            using_item: None,
            held_charge: None,
            offline_world: None,
            offline_held: None,
            offline_camera: None,
            body: None,
            perspective: crate::camera::CameraType::FirstPerson,
            avatar: None,
            dig: crate::dig::DigState::new(),
            dig_held: false,
            miss_time: 0,
            frame_pick: None,
            chest_slots: Vec::new(),
            chest_mirror: Vec::new(),
            chest_is_live: false,
            live_clicks: None,
            chest_live_window: None,
            chest_slot_rects: Vec::new(),
            chest_carried: (0, 0),
            mouse_pos: None,
            shift_down: false,
            live_held: None,
            drops: Vec::new(),
            inventory: crate::drops::Inventory::default(),
            drop_rng: crate::drops::Lcg::new(seed),
            food_rng: crate::drops::Lcg::new(seed),
            eating: None,
            pieces: Vec::new(),
            next_drop_id: 1,
            health: 20.0,
            food_data: crate::food::FoodData::new(),
            hurt_time: 0,
            hurt_duration: 0,
            invulnerable_time: 0,
            last_hurt: 0.0,
            hurt_yaw: 0.0,
            health_blink_time: 0,
            effects: crate::effects::EffectsStore::new(),
            last_health: 20,
            display_health: 20,
            last_health_time: 0,
            dead: false,
            death_message: None,
            spawn_pose: None,
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
        // THE USE CLOCK: the elapsed ticks advance while using (the
        // vanilla useDuration's own tick lane); the held item reloads
        // only when the charge's selected state index changes (the
        // bow's pulling_0 -> pulling_1 -> pulling_2 progression, the
        // crossbow's own) — the same no-op-reload law the hand lane
        // speaks. The charge math is the registry's (the javap-pinned
        // property laws).
        let mut reload: Option<(i32, usize)> = None;
        if let Some(active) = self.using_item.as_mut() {
            active.ticks += 1;
            if let Some(def) = mc_registry::item_using_def(active.item) {
                let idx = mc_registry::using_state_index(
                    def,
                    mc_registry::using_charge(def, active.ticks),
                );
                if idx != active.state {
                    active.state = idx;
                    reload = Some((active.item, idx));
                }
            }
        }
        if let Some((id, idx)) = reload {
            self.load_held_payload(id, Some(idx));
        }
        // THE EAT CLOCK (the consumable's own hold — the vanilla
        // updateUsingItem's own order: the burst check at the
        // remaining, the decrement, the completion at zero): the chew
        // bursts spawn the 5-piece shower + the voice, the completion
        // fires the 16-piece finisher + the gulp + the FoodData fold +
        // THE BURP + the survival consume. The drain slots carry the
        // events past the field borrow (the body's own pattern). The
        // pause freezes the hold (the vanilla pause's own law — the
        // singleplayer tick sleeps).
        let mut burst: Option<crate::eat::EatActive> = None;
        let mut completed: Option<crate::eat::EatActive> = None;
        if !self.paused {
            if let Some(eat) = self.eating.as_mut() {
                if eat.burst_due() {
                    burst = Some(eat.clone());
                }
                eat.ticks += 1;
                if eat.complete_due() {
                    completed = self.eating.take();
                }
            }
        }
        if let Some(eat) = burst {
            let (volume, pitch) = self.spawn_chew_burst(&eat);
            let event = eat.sound.trim_start_matches("minecraft:");
            self.sounds.play_voice(self.frame, event, volume, pitch);
        }
        if let Some(eat) = completed {
            self.complete_eat(&eat);
        }
        // THE PIECE TICK (the BreakingItemParticle's own law: the age,
        // the lifetime's removal, the move, the 0.98 friction)
        self.pieces.retain_mut(crate::eat::ItemPiece::tick);
        // THE BODY TICK (the frame==tick model the swing/use clocks
        // speak): the vanilla travel on the ACTIVE world's voxels, the
        // wire echoing through the same command channel the use arc
        // rides. The pause freezes the body (the vanilla pause's own
        // law); the headless scenes never feed one (the goldens stay).
        if self.screen == ScreenKind::World && !self.paused {
            // the drain slot the body's borrow fills (the hurt pipeline
            // runs after it ends — the disjoint-borrow split)
            let mut fall_event: Option<i32> = None;
            // the exhaustion sources' own drain slot (the body's borrow
            // fills it — the FoodData fold runs after the pipeline)
            let mut exhaustion_cost = 0.0f32;
            // THE HURT CLOCKS (LivingEntity.tick's own decrements: the
            // hurtTime flash and the invulnerableTime cooldown walk
            // down one per tick)
            if self.hurt_time > 0 {
                self.hurt_time -= 1;
            }
            if self.invulnerable_time > 0 {
                self.invulnerable_time -= 1;
            }
            // THE DEATH GATE (the DeathScreen's own input law): a dead
            // player's impulses stay zero and the dig sleeps — the
            // body's tick keeps its physics (the corpse settles) but
            // the input lanes carry nothing.
            if self.dead {
                if let Some(b) = self.body.as_mut() {
                    b.forward = 0.0;
                    b.left = 0.0;
                    b.jumping = false;
                    b.sprinting = false;
                }
                self.dig_held = false;
            }
            let using = self.using_item.is_some();
            // the disjoint field borrows: the body mutates while the
            // level reads (the borrow checker splits the fields)
            if let Some(b) = self.body.as_mut() {
                let world: Option<&crate::world::World> = if self.world_is_network {
                    self.net_world.as_ref()
                } else {
                    self.offline_world.as_ref()
                };
                if let Some(w) = world {
                    b.tick(w, using);
                    // THE FOOTSTEP: the body's own cadence law fired on a
                    // solid block under the feet — the block's step voice
                    // at the SoundType's *0.15 fold (Entity.playStepSound)
                    if b.step_due {
                        let name = crate::world::NAME_OF[w.get(
                            b.x.floor() as i32,
                            (b.y - 0.2).floor() as i32,
                            b.z.floor() as i32,
                        ) as usize];
                        self.sounds.play(self.frame, name, crate::sound::Lane::Step);
                        b.step_due = false;
                    }
                    if let Some(cmd) = b.take_move_wire().map(Self::move_wire_command) {
                        if let Some(sink) = self.live_clicks.as_ref() {
                            let _ = sink.send(cmd);
                        }
                    }
                    // THE FALL EVENT (the landing's damage the body's
                    // law folded this tick — drained like the step_due
                    // lane it sits beside; the hurt pipeline runs after
                    // the body's borrow ends)
                    fall_event = b.fall_event.take();
                    // THE EXHAUSTION SOURCES' own drain (the jump's
                    // 0.05/0.2 and the movement's cm law the body's
                    // tick banked — the networked lane's exhaustion is
                    // the server's own observation, the client folds
                    // nothing there)
                    exhaustion_cost = std::mem::take(&mut b.exhaustion_event);
                }
            }
            if let Some(dmg) = fall_event {
                self.apply_fall_damage(dmg);
            }
            // THE FOOD TICK (the stand-in server's own FoodData.tick at
            // the ServerPlayer.tick's head position — BEFORE the
            // movement's aiStep the bytecode's own order speaks, so the
            // sources of THIS tick bank for the NEXT fold): the
            // exhaustion fold, the double-lane regen (the heal the
            // Entity.heal law folds — only a living body heals, the
            // setHealth clamp rides), the starve lane's bite through
            // the hurt pipeline (the hurtServer(starve, 1.0) the
            // cooldown chain speaks). The NETWORKED lane's FoodData is
            // the server's own — the wire's Set Health speaks its
            // truth, the client folds nothing.
            if !self.world_is_network {
                let ctx = crate::food::TickContext {
                    health: self.health,
                    max_health: MAX_HEALTH,
                    difficulty: crate::food::Difficulty::Normal,
                    natural_regen: true,
                };
                let events = self.food_data.tick(&ctx);
                if events.heal > 0.0 && self.health > 0.0 {
                    self.health = (self.health + events.heal).min(MAX_HEALTH);
                }
                if events.starve {
                    self.hurt(crate::food::STARVE_DAMAGE);
                }
                // THIS tick's sources bank for the next fold (the
                // causeFoodExhaustion's own !isClientSide gate — the
                // offline lane is the server here)
                self.food_data.add_exhaustion(exhaustion_cost);
                // THE STATUS-EFFECTS' OFFLINE TICK (the stand-in
                // server's MobEffectInstance.tickServer arc at the
                // LivingEntity.tick position — the effects' beat AFTER
                // the FoodData's, the aiStep order the bytecode
                // speaks): the regeneration's heal through the
                // Entity.heal clamp (only a living body heals), the
                // durations' tick-down, the expiry's same-tick removal
                // (the absorption value rides past the expiry — the
                // 26.2's own no-removal-hook law)
                if self.health > 0.0 {
                    let heals = self.effects.tick_offline();
                    for h in heals.heals {
                        self.health = (self.health + h).min(MAX_HEALTH);
                    }
                }
            } else if self.health > 0.0 {
                // THE NETWORKED LANE'S CLIENT COPY (the LocalPlayer's
                // own tickClient — the durations tick down so the HUD's
                // blink alpha walks the vanilla law; the wire's next
                // Update/Remove is the truth, the mirror lands it)
                self.effects.tick_client();
            }
            // THE DIG TICK (the vanilla Minecraft.tick's own frame):
            // missTime counts down, the crosshair re-picks (the
            // GameRenderer.pick every frame), and the HOLD mines —
            // continueAttack(true)'s own gates and beat: the using hands
            // return early, the BLOCK hit feeds continueDestroyBlock (a
            // TRUE beat swings the arm), the MISS/air tail speaks the
            // stop (the armed dig aborts when the aim wanders off).
            if self.miss_time > 0 {
                self.miss_time -= 1;
            }
            self.frame_pick = self.clip_crosshair();
            if self.dig_held && !using {
                // the pick's own tuple copied out (the borrow ends before
                // the machine's borrows and the sends begin)
                let pick = self
                    .frame_pick
                    .as_ref()
                    .map(|h| (h.pos, h.face));
                let mut sends: Vec<crate::dig::DigSend> = Vec::new();
                let mut beat = false;
                let mut broke = false;
                let mut broke_kind: u16 = 0;
                let mut broke_pos: Option<(i32, i32, i32)> = None;
                let mut hit_kind: Option<u16> = None;
                // THE HIT BEAT (sound.rs's law): the vanilla checks
                // destroyTicks % 4.0F == 0.0F BEFORE the machine's tick
                // increments — captured here on the SAME-DESTROYING-TARGET
                // state (a fresh arm plays nothing — startDestroyBlock is
                // silent, the first hit rides the first continue tick),
                // folded after the call (the vanished-cell path never
                // speaks; the retarget path is excluded by the target
                // moving — the tail-called start resets nothing here)
                let target_before = self.dig.pos;
                let hit_beat = self.dig.is_destroying
                    && self.dig.delay == 0
                    && crate::sound::beat_due(self.dig.ticks);
                match pick {
                    Some((pos, face)) => {
                        let kind = self
                            .active_world()
                            .map(|w| w.get(pos.0, pos.1, pos.2))
                            .unwrap_or(crate::world::AIR);
                        if kind != crate::world::AIR {
                            let held = self.held_id();
                            let (on_ground, eye_water) = self.body_state();
                            // the direct field split (the method's own
                            // &mut self borrow would live too long)
                            let world_opt = if self.world_is_network {
                                self.net_world.as_mut()
                            } else {
                                self.offline_world.as_mut()
                            };
                            let dig = &mut self.dig;
                            if let Some(world) = world_opt {
                                let was_solid =
                                    world.get(pos.0, pos.1, pos.2) != crate::world::AIR;
                                let (s, b) = dig.continue_destroy_block(
                                    pos, face, held, world, on_ground, eye_water,
                                );
                                sends = s;
                                beat = b;
                                broke = b
                                    && was_solid
                                    && world.get(pos.0, pos.1, pos.2) == crate::world::AIR;
                                if broke {
                                    broke_kind = kind;
                                    broke_pos = Some(pos);
                                }
                                hit_kind = Some(kind);
                            }
                        } else {
                            // the aimed cell vanished (a neighbor's dig):
                            // the state disarms, no beat, no swing
                            self.dig.is_destroying = false;
                        }
                    }
                    None => {
                        // the aim left the blocks: the stop tail (the
                        // vanilla's own stopDestroyBlock call)
                        sends = self.dig.stop_destroy_block();
                    }
                }
                self.enqueue_dig(&sends);
                if broke {
                    // the completion's local break: the mesh re-speaks
                    // (the placement's own rhythm); the 2001 event's own
                    // voice — the broken block's break sound at the
                    // (v+1)/2, p*0.8 fold (LevelEventHandler case 2001)
                    self.remesh_offline();
                    let name = crate::world::NAME_OF[broke_kind as usize];
                    self.sounds.play(self.frame, name, crate::sound::Lane::Break);
                    // THE REWARD HALF (the vanilla destroyBlock's own
                    // dropResources): the loot roll, the item entities
                    // spawning by the popResource law
                    if let Some(pos) = broke_pos {
                        self.spawn_drops_for(broke_kind, pos);
                    }
                }
                if beat {
                    // the mining beat's own swing (the vanilla's
                    // continueAttack tail — every TRUE beat swings; the
                    // SwingState's re-trigger law keeps the arc honest
                    // and LocalPlayer.swing echoes the wire)
                    self.swing.start();
                    if let Some(sink) = self.live_clicks.as_ref() {
                        let _ = sink.send(crate::world_view::ContainerCommand::Swing);
                    }
                    // THE DIG BEAT's hit voice: the same-target mining
                    // tick whose %4 beat is due — the block's hit sound
                    // at the (v+1)/8, p*0.5 fold (the pre-tick block kind
                    // names the voice — the completion tick folds too
                    // when the beat lands there, the vanilla bytecode's
                    // own order: sound, then the completion; the retarget
                    // and vanished paths never reach here)
                    if hit_beat && self.dig.pos == target_before {
                        if let Some(kind) = hit_kind {
                            let name = crate::world::NAME_OF[kind as usize];
                            self.sounds.play(self.frame, name, crate::sound::Lane::Hit);
                        }
                    }
                }
            }
            // THE DROP TICK (the vanilla Minecraft.tick's entity lane, the
            // OFFLINE world's own authority): the item entities live their
            // ItemEntity tick (the physics, the clocks), the neighbour
            // merge rides each entity's own %40 cadence (inside
            // merge_pass), and the player's aiStep scan speaks the pickup
            // — the inventory add, the take voice, the stack's honest
            // discard. The NETWORKED world's entities stay the server's
            // (the voxel prediction's own split).
            if !self.world_is_network && !self.drops.is_empty() {
                {
                    let world = self.offline_world.as_mut();
                    if let Some(world) = world {
                        for d in self.drops.iter_mut() {
                            d.tick(world);
                        }
                    }
                }
                crate::drops::merge_pass(&mut self.drops);
                if let Some(b) = self.body.as_ref() {
                    let selected = self.selected_hotbar;
                    let mut picked = false;
                    for d in self.drops.iter_mut() {
                        if crate::drops::pickup_scan(b, d) {
                            if let crate::drops::Touch::Picked { .. } =
                                crate::drops::player_touch(d, &mut self.inventory, selected)
                            {
                                picked = true;
                            }
                        }
                    }
                    if picked {
                        // the take voice (handleTakeItemEntity's own law —
                        // the packet's sound beat the offline lane speaks
                        // directly)
                        self.sounds.play_pickup(self.frame);
                        // the hotbar's projection refreshes with the take
                        self.hotbar_slots = self.inventory.hotbar();
                    }
                }
                self.drops.retain(|d| d.count > 0);
            }
        }
    }

    /// The left press's attack lane — the vanilla `Minecraft.startAttack`'s
    /// own law, verbatim: the missTime gate (the anti-spam window an air
    /// swing armed), the busy-hands gate (the using item's own), the
    /// BLOCK hit that arms the dig (startDestroyBlock — the state machine
    /// speaks its own ABORT/START), the MISS that arms the 10-tick
    /// missTime (survival's hasMissTime), and the UNCONDITIONAL swing
    /// (the vanilla swings on every branch that got past the early
    /// returns — the arm moves whether or not a block stood there).
    pub fn on_attack(&mut self) {
        // the button state rides the edge (the OS's own isDown — the
        // hold continues even while the gates refuse the swing)
        self.dig_held = true;
        if self.screen != ScreenKind::World || self.paused {
            return;
        }
        // THE DEATH GATE: a dead player's hands sleep (the DeathScreen
        // owns the input — the same law the update loop's gate speaks)
        if self.dead {
            return;
        }
        if self.miss_time > 0 {
            return;
        }
        if self.using_item.is_some() {
            return; // the vanilla isHandsBusy gate
        }
        // the BLOCK case: the dig arms through the state machine (the
        // held id, the body's ground state and the eye's water state all
        // ride — the vanilla computes the insta-break from the LIVE
        // player). The clip re-runs HERE (the press may land between the
        // ticks; the vanilla's hitResult is the last pick's own — the
        // fresh clip is the same information, one frame fresher).
        let hit = self.clip_crosshair();
        let kind = hit
            .as_ref()
            .and_then(|h| self.active_world())
            .map(|w| {
                let h = hit.as_ref().unwrap();
                w.get(h.pos.0, h.pos.1, h.pos.2)
            })
            .unwrap_or(crate::world::AIR);
        let mut dug = false;
        let mut dug_pos: Option<(i32, i32, i32)> = None;
        let mut sends: Vec<crate::dig::DigSend> = Vec::new();
        if let Some(h) = &hit {
            if kind != crate::world::AIR {
                let held = self.held_id();
                let (on_ground, eye_water) = self.body_state();
                // the disjoint field split: the world option and the dig
                // state borrow separately (the checker's own verdict)
                let world_opt = if self.world_is_network {
                    self.net_world.as_mut()
                } else {
                    self.offline_world.as_mut()
                };
                let dig = &mut self.dig;
                if let Some(world) = world_opt {
                    let was_solid = world.get(h.pos.0, h.pos.1, h.pos.2) != crate::world::AIR;
                    sends = dig.start_destroy_block(
                        h.pos, h.face, held, world, on_ground, eye_water,
                    );
                    dug = was_solid
                        && world.get(h.pos.0, h.pos.1, h.pos.2) == crate::world::AIR;
                    if dug {
                        dug_pos = Some(h.pos);
                    }
                }
            } else {
                self.miss_time = 10; // the vanilla hasMissTime branch
            }
        } else {
            self.miss_time = 10; // the pick spoke MISS
        }
        self.enqueue_dig(&sends);
        if dug {
            self.remesh_offline();
            // THE INSTA-BREAK's own completion (the vanilla startDestroyBlock
            // lambda's destroyBlock): the drop roll speaks here too
            if let Some(pos) = dug_pos {
                self.spawn_drops_for(kind, pos);
            }
        }
        // THE UNCONDITIONAL SWING (the vanilla startAttack's own tail:
        // the BLOCK case, the MISS case — every branch that reached the
        // switch's end swings; LocalPlayer.swing echoes the wire)
        self.swing.start();
        if let Some(sink) = self.live_clicks.as_ref() {
            let _ = sink.send(crate::world_view::ContainerCommand::Swing);
        }
    }

    /// The left press's falling edge — the vanilla `continueAttack(false)`
    /// tail: the missTime clears, and the armed dig ABORTs (the crack
    /// overlay clears, the wire speaks the Player Action packet). The
    /// using-item gate returns BEFORE the abort (the vanilla's own
    /// order — a bow release does not speak a dig abort).
    pub fn on_left_release(&mut self) {
        self.dig_held = false;
        if self.screen != ScreenKind::World || self.paused {
            return;
        }
        self.miss_time = 0;
        if self.using_item.is_some() {
            return;
        }
        let sends = self.dig.stop_destroy_block();
        self.enqueue_dig(&sends);
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
            let mut world = crate::world::World::new(WORLD_CHUNKS, self.seed);
            self.world_size = world.size_blocks();
            self.spawn_y = top_solid_y(&world, self.world_size / 2, self.world_size / 2);
            self.world_quads = Some(crate::mesher::mesh_world(&world, None));
            self.world_is_network = false;
            // the App OWNS the procedural voxels now (the block-use
            // lane's placement prediction edits them in place)
            self.offline_world = Some(world);
            // the dig lane predicts locally on the offline terrain (the
            // singleplayer behavior — the machine's own default re-armed)
            self.dig.apply_locally = true;
            self.dig_held = false;
            // the procedural world is an overworld world — the vanilla
            // extract pins the dimension's skybox on entering the level
            self.world_skybox = Skybox::Overworld;
            self.world_fog = None;
            self.world_cloud_fog_end = None;
        }
        self.paused = false;
        self.music.on_world_enter_biome(spawn_biome(self.seed));
        self.open(ScreenKind::World);
    }

    /// Opens the deterministic chest screen — the container GUI's SCREEN
    /// round. The vanilla GenericContainerScreen (generic_9x3) geometry:
    /// the 176x166 panel centered, the 9x3 container grid at (8, 18)
    /// with the 18-pitch, the player mirror at (8, 84) and the hotbar
    /// row at (8, 142) — the generic_54 canvas's own numbers. The
    /// loadout is a FIXED representative table (no RNG: a constant pins
    /// the pixel test hardest), the ids the ITEM registry order's own —
    /// the same table the wire's holder-VarInt decodes ride. The world
    /// underneath is the deterministic procedural one (the pause
    /// screen's own flow).
    pub fn open_chest(&mut self) {
        self.enter_world();
        // the OFFLINE stand-in: the live lane's flag stays down — a
        // container-less network update never closes this screen, the
        // two flows stay separate (the golden pins this scene's pixels)
        self.chest_is_live = false;
        self.chest_mirror.clear();
        const LOADOUT: [(usize, i32, i32); 8] = [
            (4, 926, 12),  // diamond
            (13, 932, 64), // iron_ingot
            (22, 936, 7),  // gold_ingot
            (9, 921, 5),   // apple
            (17, 981, 3),  // bread
            (0, 62, 27),   // cobblestone
            (26, 63, 16),  // oak_planks
            (10, 924, 41), // coal
        ];
        let mut slots = vec![(0i32, 0i32); 27];
        for (i, id, count) in LOADOUT {
            slots[i] = (id, count);
        }
        self.chest_slots = slots;
        // the offline scene carries no cursor stack (the deterministic
        // scene the golden pins — a live session's held stack never
        // leaks into it)
        self.chest_carried = (0, 0);
        self.open(ScreenKind::Chest);
    }

    /// The chest screen render: the frozen world dimmed underneath (the
    /// container background's own treatment), the classic gray panel
    /// with its white/0x555555 bevels, the real `container/slot` insets,
    /// the flat item icons through the generated texture table, the
    /// vanilla count rendering (white, shadowed, right-aligned at the
    /// slot's bottom edge), and the lang title in the label color
    /// WITHOUT the shadow (the AbstractContainerScreen's own draw call).
    /// The player mirror rows render their chrome always and their
    /// CONTENTS when the live lane filled them (the networked store's
    /// rows 27..63 — the offline stand-in keeps the chrome-only shape
    /// the golden pins). The render also lays out the slot HIT BOXES
    /// (the 16x16 inner cells with their menu slot numbers) for the
    /// press lane — the buttons' own render-then-hit pattern, zero
    /// pixel surface of its own.
    fn render_chest(&mut self, g: &mut Gui, a: &mut GuiAssets, w: i32, h: i32) {
        g.fill(0, 0, w, h, [0.0, 0.0, 0.0, 0.35]);
        let (px, py) = chest_panel_origin(w, h);
        // the hit boxes: the container grid's 0..27 then the mirror's
        // 27..63 — the MENU's own numbering, the click's slot lane
        self.chest_slot_rects = (0..27)
            .map(|i| {
                let (sx, sy) = chest_container_slot((px, py), i);
                (sx + 1, sy + 1, i as i16)
            })
            .chain((0..36).map(|i| {
                let (sx, sy) = chest_mirror_slot((px, py), i);
                (sx + 1, sy + 1, (27 + i) as i16)
            }))
            .collect();
        let face = [0.776, 0.776, 0.776, 1.0]; // 0xC6C6C6
        let light = [1.0, 1.0, 1.0, 1.0];
        let dark = [0.333, 0.333, 0.333, 1.0]; // 0x555555
        g.fill(px + 1, py + 1, 174, 164, face);
        g.fill(px, py, 176, 1, light); // top bevel
        g.fill(px, py, 1, 166, light); // left bevel
        g.fill(px, py + 165, 176, 1, dark); // bottom bevel
        g.fill(px + 175, py, 1, 166, dark); // right bevel
        let label = [0.251, 0.251, 0.251]; // 0x404040
        g.text(
            a,
            px + 8,
            py + 6,
            tr("container.chest").unwrap_or("Chest"),
            label,
            false,
        );
        for i in 0..27 {
            let (sx, sy) = chest_container_slot((px, py), i);
            g.sprite(a, "container/slot", sx, sy);
        }
        for (i, (id, count)) in self.chest_slots.iter().enumerate() {
            if *count <= 0 {
                continue; // the wire's own emptiness sentinel
            }
            let (sx, sy) = chest_container_slot((px, py), i);
            draw_item_icon(g, *id, sx + 1, sy + 1);
            if *count > 1 {
                let s = count.to_string();
                let tw = g.text_width(&s);
                g.text(a, sx + 17 - tw, sy + 9, &s, WHITE, true);
            }
        }
        for i in 0..36 {
            let (sx, sy) = chest_mirror_slot((px, py), i);
            g.sprite(a, "container/slot", sx, sy);
        }
        // the live mirror's contents (the networked store's rows 27..63):
        // the same icon/count treatment the grid rides over the mirror's
        // own geometry — the offline stand-in leaves this list empty
        for (i, (id, count)) in self.chest_mirror.iter().enumerate().take(36) {
            if *count <= 0 {
                continue; // the wire's own emptiness sentinel
            }
            let (sx, sy) = chest_mirror_slot((px, py), i);
            draw_item_icon(g, *id, sx + 1, sy + 1);
            if *count > 1 {
                let s = count.to_string();
                let tw = g.text_width(&s);
                g.text(a, sx + 17 - tw, sy + 9, &s, WHITE, true);
            }
        }
        // THE HELD STACK follows the cursor (the vanilla carried render,
        // AbstractContainerScreen's post-panel pass): the believed
        // stack's icon centered under the hotspot, the same icon/count
        // treatment the slots ride, above everything the panel drew.
        // The headless scenes track no mouse (`mouse_pos` is None) —
        // the pixel goldens never see this pass.
        if self.chest_is_live && self.chest_carried.1 > 0 {
            if let Some((mx, my)) = self.mouse_pos {
                draw_item_icon(g, self.chest_carried.0, mx - 8, my - 8);
                if self.chest_carried.1 > 1 {
                    let s = self.chest_carried.1.to_string();
                    let tw = g.text_width(&s);
                    g.text(a, mx - 8 + 17 - tw, my - 8 + 9, &s, WHITE, true);
                }
            }
        }
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
    pub fn enter_network_world(&mut self, spawn_biome: Option<String>, skybox: Skybox, fog: Option<(f32, f32)>, cloud_fog_end: Option<f32>) {
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
        // the networked world owns the terrain from here — the offline
        // voxels drop (the placement prediction is the server's own
        // chunk refresh on this lane)
        self.offline_world = None;
        // the dig lane's prediction flag follows the world's authority
        // (the networked terrain is the server's own — the placement
        // lane's verdict the dig speaks too)
        self.dig.apply_locally = false;
        self.dig_held = false;
        // the vanilla extract's render state: the dimension's skybox pins
        // at level entry — resolved through the live model (the synced
        // registry's own skybox, the codec default, or the built-in table;
        // see `WorldModel::skybox`)
        self.world_skybox = skybox;
        // the fog pair pins with it: the dimension's stated distances are
        // the level-entry render state (None = the window treatment)
        self.world_fog = fog;
        self.world_cloud_fog_end = cloud_fog_end;
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
            // THE DEATH GATE (the vanilla death screen's own modal law):
            // Escape never pauses while dead — the DeathScreen owns the
            // input, the Respawn/Title buttons are the only exits
            if self.dead {
                return;
            }
            self.paused = !self.paused;
            self.buttons.clear();
        } else if self.screen == ScreenKind::Chest {
            // the container's own close (vanilla Escape/E on a container
            // screen): back to the world. The player's OWN close echoes
            // the serverbound Container Close packet through the live
            // session's send lane (the offline stand-in has no window
            // and no session — its close stays a local act); the
            // server's OWN close (the clientbound packet) closes the
            // live screen through the container lane regardless.
            if self.chest_is_live {
                if let (Some((window_id, _)), Some(sink)) =
                    (self.chest_live_window, self.live_clicks.as_ref())
                {
                    let _ = sink.send(crate::world_view::ContainerCommand::Close { window_id });
                }
            }
            self.chest_is_live = false;
            self.chest_live_window = None;
            // the player's OWN close returns the held stack to the
            // inventory server-side (vanilla containerClose): the local
            // belief resets with the window
            self.chest_carried = (0, 0);
            self.open(ScreenKind::World);
        } else if matches!(self.screen, ScreenKind::Connect | ScreenKind::Disconnect) {
            self.open(ScreenKind::Servers);
        }
    }

    /// THE F5 LANE (vanilla `Minecraft.handleKeybinds`):
    /// `keyTogglePerspective.consumeClick()` →
    /// `options.setCameraType(getCameraType().cycle())` — the three-view
    /// cycle the CameraType bytecode speaks.
    pub fn on_perspective_key(&mut self) {
        self.perspective = self.perspective.cycle();
    }

    /// The LEFT press's rising edge (the vanilla MouseHandler's own
    /// press routing): a container screen's press clicks the slot it
    /// landed on (button 0); every other screen falls through to the
    /// attack swing (the world's left press — the guard inside keeps
    /// the menus silent).
    pub fn on_press(&mut self, mx: i32, my: i32) {
        if self.screen == ScreenKind::Chest {
            self.click_chest_slot(mx, my, 0);
            return;
        }
        self.on_attack();
    }

    /// The RIGHT press's rising edge (the vanilla MouseHandler's own
    /// routing, the button byte's other lane): a container screen's
    /// press clicks the slot with button 1 — the split-stack pickup /
    /// place-one table (`predict_pickup_right`). The WORLD's right
    /// press is the USE lane (the vanilla rightClickStart's own shape)
    /// with the BLOCK branch FIRST (the vanilla startUseItem's own
    /// order): the crosshair clip speaks the useItemOn echo and the
    /// block item's placement CONSUMES; the PASS results fall through
    /// to the item lanes — the CHARGED crossbow fires INSTANTLY (javap
    /// CrossbowItem.use: the CHARGED_PROJECTILES component non-empty ->
    /// performShooting + CONSUME — no pull), the charge is consumed
    /// (the shot's own component clear, the same prediction the swing
    /// lane speaks) and the UseItem echo rides the live session's
    /// command channel; the OTHER items whose trees carry the
    /// `minecraft:using_item` gate start a use (the registry's own
    /// census — the bow, the crossbow, the goat horn, the shield, the
    /// trident), the hold renders the using branch. Returns whether a
    /// use started (the tests' honest probe).
    pub fn on_right_press(&mut self, mx: i32, my: i32) -> bool {
        if self.screen == ScreenKind::Chest {
            self.click_chest_slot(mx, my, 1);
            return false;
        }
        if self.screen == ScreenKind::World && !self.paused {
            // THE DESTROY GATE (the vanilla startUseItem's own first
            // check: `if gameMode.isDestroying() return` — the right
            // press refuses while the left hand mines)
            if self.dig.is_destroying {
                return false;
            }
            // THE BLOCK LANE FIRST (the vanilla startUseItem's own
            // order): the crosshair clip + the useItemOn echo + the
            // block item's placement CONSUME; the PASS results fall
            // through to the item's use lane below
            if self.on_right_press_block_lane() {
                return true;
            }
            // the item lane reads the UNIFIED held id (the networked
            // store's selection, the offline `--held-item` stand-in —
            // the offline client's use animations are the vanilla's own
            // singleplayer behavior)
            if let Some(id) = self.held_id() {
                // THE EAT LANE FIRST (the vanilla startUseItem's own
                // order — the item's use() speaks: the CONSUMABLE's
                // startConsuming opens the hold; the pull lanes ride
                // after): the canConsume gate — a FOOD-less consumable
                // (the potion) consumes unconditionally, the FOOD's
                // own lane speaks `canEat(canAlwaysEat)`.
                if let Some(def) = self.food_def_of(id) {
                    let consumes = !def.has_food()
                        || crate::eat::can_eat(
                            def.can_always_eat,
                            false,
                            self.food_data.needs_food(),
                        );
                    if !consumes {
                        return false; // the canConsume's own FAIL
                    }
                    self.using_item = None; // one use at a time
                    self.eating = crate::eat::EatActive::start(id, def);
                    // the wire echo: the eat's use-item click (the
                    // networked lane's own command the pull lane rides)
                    let (yaw, pitch) = crate::world_view::wire_angles(&self.world_camera());
                    if let Some(sink) = self.live_clicks.as_ref() {
                        let _ = sink.send(crate::world_view::ContainerCommand::UseItem {
                            yaw,
                            pitch,
                        });
                    }
                    return true;
                }
                // THE LOADED CROSSBOW: the instant shot — the use
                // consumes the charge (vanilla's CONSUME branch), the
                // resting render returns to the plain crossbow, and
                // the wire echo rides (the shot is a use-item click)
                if self.held_charge.is_some() && mc_registry::item_charge_def(id).is_some() {
                    self.held_charge = None;
                    self.load_held_payload(id, None);
                    let (yaw, pitch) = crate::world_view::wire_angles(&self.world_camera());
                    if let Some(sink) = self.live_clicks.as_ref() {
                        let _ = sink.send(crate::world_view::ContainerCommand::UseItem {
                            yaw,
                            pitch,
                        });
                    }
                    return true;
                }
                if let Some(def) = mc_registry::item_using_def(id) {
                    let state = mc_registry::using_state_index(def, 0.0);
                    self.using_item = Some(UseActive { item: id, ticks: 0, state });
                    self.load_held_payload(id, Some(state));
                    // the wire echo: the camera's angles in the vanilla
                    // degrees (the snapshot conversion's own inverse)
                    let (yaw, pitch) = crate::world_view::wire_angles(&self.world_camera());
                    if let Some(sink) = self.live_clicks.as_ref() {
                        let _ = sink.send(crate::world_view::ContainerCommand::UseItem {
                            yaw,
                            pitch,
                        });
                    }
                    return true;
                }
            }
        }
        false
    }

    /// The RIGHT press's falling edge (the vanilla rightClickStop's own
    /// shape): the active use ends, the held item returns to the
    /// resting payload, and the release's serverbound echo rides the
    /// command channel (the RELEASE_USE_ITEM action). The CROSSBOW's
    /// load law rides the same edge (javap CrossbowItem.releaseUsing:
    /// the power for the elapsed ticks >= 1.0 && !isCharged ->
    /// tryLoadProjectiles sets the CHARGED_PROJECTILES component — the
    /// pull LOADED the weapon, it did not fire): a full-charge release
    /// sets the local component (the arrow — the inventory's first
    /// projectile, the deterministic stand-in the offline lane speaks)
    /// and the resting render switches to the charged bolt. A no-op
    /// when no use is active.
    pub fn on_right_release(&mut self) {
        // THE EAT'S CANCEL (javap releaseUsing's own shape for the
        // consumable: the hold ends silently — no completion, no
        // sound, no penalty; the pieces already spawned keep their
        // flight). The eat checked first — the one-use-at-a-time law
        // keeps the lanes exclusive.
        if let Some(eat) = self.eating.take() {
            self.load_held_payload(eat.item, None);
            return;
        }
        let Some(active) = self.using_item.take() else {
            return;
        };
        // THE LOAD LAW: the full charge loads the projectile (the
        // crossbow's own gate — the charge def names the component
        // select's carrier; the bow has no charge lane)
        if active.ticks >= mc_registry::CROSSBOW_CHARGE_TICKS
            && mc_registry::item_charge_def(active.item).is_some()
        {
            self.held_charge = Some(mc_registry::ChargeKind::Arrow);
        }
        self.load_held_payload(active.item, None);
        if let Some(sink) = self.live_clicks.as_ref() {
            let _ = sink.send(crate::world_view::ContainerCommand::ReleaseUse);
        }
    }

    /// The held-item payload loader — the USING arc's reload lane over
    /// live_held_updated's resting law. `Some(index)` loads the using
    /// branch's state at that index (the special payloads through the
    /// dump-backed lane, the model layers through the layer tables);
    /// `None` = the resting resolution (the specials first, then the
    /// layer tables — the disjoint lanes' routing). A state that binds
    /// nothing drops to the bare arm (the honest unknown — the same
    /// law every other lane speaks).
    fn load_held_payload(&mut self, id: i32, using_state: Option<usize>) {
        self.held_item = match using_state {
            Some(idx) => match mc_registry::item_using_state(id, idx) {
                Some(mc_registry::UsingStateRender::Special(_)) => {
                    crate::hand::load_special_using_item(id)
                }
                Some(mc_registry::UsingStateRender::Layers(layers)) => {
                    crate::hand::load_item_layers(&layers)
                }
                None => None,
            },
            None => {
                // THE CHARGE LANE first: the loaded crossbow's resting
                // render rides the charge_type select's case (the
                // CHARGED_PROJECTILES component's own branch) — the
                // uncharged stack falls through to the plain routing
                if let Some(kind) = self.held_charge {
                    if mc_registry::item_charge_def(id).is_some() {
                        if let Some(held) = crate::hand::load_charge_item(id, kind) {
                            self.held_item = Some(held);
                            return;
                        }
                    }
                }
                if mc_registry::special_item_renderer(id).is_some() {
                    crate::hand::load_special_item(id)
                } else {
                    mc_registry::item_model_layers(id)
                        .and_then(|layers| crate::hand::load_item_layers(&layers))
                }
            }
        };
    }

    /// The live mouse position (the windowed loop's per-frame feed, the
    /// vanilla MouseHandler's own stream): the carried stack's
    /// cursor-follow render reads it. `None`-clearing is the window's
    /// own honesty — minifb's Discard mode reports no position when the
    /// cursor leaves, and the carried render follows it out.
    pub fn on_mouse_move(&mut self, mx: i32, my: i32) {
        self.mouse_pos = Some((mx, my));
    }

    /// The keyboard lane's shift state (the windowed loop's per-frame
    /// feed, the vanilla `hasShiftDown()` query's own shape): the next
    /// press's rising edge routes the shift-click through it. The
    /// headless scenes never feed it — the QUICK_MOVE lane stays off
    /// (the goldens' deterministic scenes).
    pub fn on_shift_down(&mut self, down: bool) {
        self.shift_down = down;
    }

    /// The number keys' hotbar selection (the vanilla keyPress's own
    /// lane): the HUD's selection box moves with the key, and the LIVE
    /// session's belief rides the command channel — the worker moves
    /// the store's selection (the local prediction: the hand follows
    /// the keys through the store's own projection, v0.3.137's lane)
    /// and speaks the serverbound Set Carried Item (the dump's
    /// short-shaped slot). The OFFLINE stand-in keeps the move
    /// cosmetic (no session, no wire — the deterministic scenes stay
    /// silent). The SELECTION-relative 0..8 gate drops the
    /// out-of-domain and the no-op repeat (the vanilla
    /// `Inventory.selected`'s own range).
    pub fn on_number_key(&mut self, selection: usize) {
        if selection > 8 || selection == self.selected_hotbar {
            return; // the gate: out-of-domain or the repeat
        }
        self.selected_hotbar = selection;
        // the held model follows the selection (the offline inventory's
        // own lane — the empty selection falls through to the stand-in,
        // the goldens' scenes never press keys)
        let id = self.held_id();
        self.load_held_payload_from_id(id);
        if let Some(sink) = self.live_clicks.as_ref() {
            let _ = sink.send(crate::world_view::ContainerCommand::SelectHotbar {
                selection: selection as u8,
            });
        }
    }

    /// The chest screen's slot click (the button byte's own lane: 0 =
    /// the left table, 1 = the right table; the SHIFT state overrides
    /// either table with the QUICK_MOVE lane — the vanilla
    /// `hasShiftDown()`'s own routing): the press finds the 16x16
    /// hit box the last render laid out (the vanilla Slot's own hit
    /// shape), and the LIVE window's identity (id + revision) rides the
    /// command to the session's send lane. THE LOCAL PREDICTION runs
    /// the vanilla table against the believed state (the screen's own
    /// slots + the believed cursor — [`crate::world_view::predict_pickup`]
    /// for the left, [`crate::world_view::predict_pickup_right`] for
    /// the right), the screen's slot lists and cursor mutate IN PLACE
    /// (the server's authoritative update replaces them wholesale — the
    /// correction is the next projection), and the post-click belief
    /// rides the command as the changed-slot delta + the carried cursor
    /// (the wire lanes `HashedStack::item` speaks). The OFFLINE
    /// stand-in has no window and no session: its presses are the
    /// honest no-op (the golden's deterministic scene).
    fn click_chest_slot(&mut self, mx: i32, my: i32, button: i8) {
        if !self.chest_is_live {
            return; // the offline stand-in: no window, no click
        }
        let Some((window_id, state_id)) = self.chest_live_window else {
            return; // no window identity — nothing to echo against
        };
        let Some(sink) = self.live_clicks.as_ref() else {
            return; // no live session — the command dies on the frame thread
        };
        let Some(&(_, _, slot)) = self
            .chest_slot_rects
            .iter()
            .find(|&&(x, y, _)| mx >= x && mx < x + 16 && my >= y && my < y + 16)
        else {
            return; // the press landed between the slots — no click
        };
        // the believed slot value: the grid's 0..27, the mirror's
        // 27..63 — the menu's own numbering both lanes index
        let believed = if (slot as usize) < 27 {
            self.chest_slots[slot as usize]
        } else {
            self.chest_mirror[slot as usize - 27]
        };
        // THE QUICK_MOVE LANE (the shift-click): the whole stack moves
        // to the OTHER section — the vanilla doClick's third branch
        // (`hasShiftDown()` overrides the click type for the left AND
        // the right press; the button byte keeps its press's own
        // value). The full believed menu feeds the prediction, the
        // multi-slot deltas mutate the screen in place and ride the
        // command; the cursor is NOT the quick-move's lane.
        if self.shift_down {
            let snapshot: Vec<(i32, i32)> = self
                .chest_slots
                .iter()
                .chain(self.chest_mirror.iter())
                .copied()
                .collect();
            let changed = crate::world_view::predict_quick_move(&snapshot, slot as usize);
            for (i, pair) in &changed {
                if *i < 27 {
                    self.chest_slots[*i] = *pair;
                } else {
                    self.chest_mirror[*i - 27] = *pair;
                }
            }
            let _ = sink.send(crate::world_view::ContainerCommand::Click(
                crate::world_view::ContainerClick {
                    window_id,
                    state_id,
                    slot,
                    button,
                    input: crate::world_view::ContainerInput::QuickMove,
                    changed: changed.iter().map(|(i, p)| (*i as i16, *p)).collect(),
                    carried: self.chest_carried,
                },
            ));
            return;
        }
        let (new_slot, new_carried) = if button == 0 {
            crate::world_view::predict_pickup(believed, self.chest_carried)
        } else {
            crate::world_view::predict_pickup_right(believed, self.chest_carried)
        };
        if (slot as usize) < 27 {
            self.chest_slots[slot as usize] = new_slot;
        } else {
            self.chest_mirror[slot as usize - 27] = new_slot;
        }
        self.chest_carried = new_carried;
        let _ = sink.send(crate::world_view::ContainerCommand::Click(
            crate::world_view::ContainerClick {
                window_id,
                state_id,
                slot,
                button,
                input: crate::world_view::ContainerInput::Pickup,
                changed: vec![(slot, new_slot)],
                carried: new_carried,
            },
        ));
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
                // the DeathScreen's own pair: Respawn resets the vitals,
                // re-anchors the body to the spawn and echoes the Client
                // Command's PERFORM_RESPAWN; the Title Screen button is
                // the vanilla's exit lane (the client's own title menu)
                "respawn" => {
                    self.respawn();
                    self.buttons.clear();
                }
                "deathtitle" => {
                    self.dead = false;
                    self.open(ScreenKind::Title);
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
                        // the body drops with the level (the next join's
                        // teleport pose materializes a fresh one)
                        self.body = None;
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
            Ok(LoginResult::Loaded { world, camera, spawn_biome, slim_arm, skybox, fog, cloud_fog_end, .. }) => {
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
                    self.enter_network_world(spawn_biome, skybox, fog, cloud_fog_end);
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
    /// the sockets go down like the original's. The container command
    /// lane dies with the session — a stored sender without a worker
    /// would be a ghost lane (a click would silently vanish instead of
    /// dying on the frame thread).
    fn close_all_live_runs(&mut self) {
        for (_, run) in self.live_runs.drain(..) {
            run.store(false, std::sync::atomic::Ordering::Relaxed);
        }
        self.live_clicks = None;
        self.chest_live_window = None;
    }

    /// Attach the container command lane of the freshly dispatched
    /// join's worker (the frame thread's clicks and the player's own
    /// close echo ride it into the session's send path). Replaces any
    /// previous lane — a superseded join's sender drops with its
    /// worker's stop flag.
    pub fn attach_live_clicks(
        &mut self,
        clicks: std::sync::mpsc::Sender<crate::world_view::ContainerCommand>,
    ) {
        self.live_clicks = Some(clicks);
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
    ///
    /// The CONTAINER lane rides the same update: `Some(window)` opens
    /// (or updates in place) the networked chest screen — the vanilla
    /// `MenuScreens.create` show lands the server's window on the
    /// player; `None` (the store dropped — the server's Container
    /// Close or a fresh join) closes a LIVE chest screen back to the
    /// world (the vanilla closeContainer's own return). The offline
    /// deterministic chest (open_chest) is never touched: the live
    /// flag gates the close, the flows stay separate.
    pub fn live_view_updated(
        &mut self,
        world: crate::world::World,
        camera: Option<crate::camera::Camera>,
        biome: Option<String>,
        container: Option<crate::world_view::LiveContainer>,
    ) {
        if !self.world_is_network {
            return;
        }
        // THE BODY RE-ANCHOR (the vanilla SynchronizePlayerPosition's
        // own authority): the shipped pose is the server's teleport
        // lane — when it CHANGES (or the body never stood) the body
        // re-baselines at it; the body's own walking never rides this
        // (the server does not echo plain moves — the prediction
        // stands until the server corrects).
        let teleport = self.net_camera.is_none() || self.net_camera != camera;
        self.net_camera = camera;
        if let Some(cam) = camera {
            if teleport || self.body.is_none() {
                let (yaw, pitch) = crate::world_view::wire_angles(&cam);
                self.body = Some(crate::body::PlayerBody::new(
                    cam.pos.x as f64,
                    (cam.pos.y - crate::world_view::PLAYER_EYE_HEIGHT) as f64,
                    cam.pos.z as f64,
                    yaw,
                    pitch,
                ));
                // the teleport's pose IS the spawn (the server's respawn
                // stream re-teleports through the same lane)
                self.spawn_pose = Some((
                    cam.pos.x as f64,
                    (cam.pos.y - crate::world_view::PLAYER_EYE_HEIGHT) as f64,
                    cam.pos.z as f64,
                    yaw,
                    pitch,
                ));
            }
        }
        self.net_world = Some(world);
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
        self.live_container_updated(container);
    }

    /// The OFFLINE world screen's camera pose — the look-control lane's
    /// own set (the vanilla teleport/pose shape the net side speaks: the
    /// pose lands, the world renders from it). The placement lane's clip
    /// aims through the same pose.
    pub fn offline_camera_updated(&mut self, cam: crate::camera::Camera) {
        self.offline_camera = Some(cam);
    }
    /// THE OFFLINE BODY SPAWN (the windowed loop's own call once the
    /// world screen stands): the feet on the procedural surface at the
    /// spawn column's center, the flight camera's east-facing angles —
    /// the body then OWNS the windowed pose (the vanilla
    /// singleplayer's own spawn discipline). No-op when a body already
    /// stands or the screen shows the network world (its body lands
    /// through live_view_updated's re-anchor). The headless scenes
    /// never call this — the goldens' flight camera stays.
    pub fn spawn_offline_body(&mut self) {
        if self.body.is_some() || self.world_is_network || self.world_size <= 0 {
            return;
        }
        let n = self.world_size as f64;
        // THE SPAWN LAW (the vanilla getSpawnPos's own discipline): the
        // feet stand on the first SOLID cell below the recorded spawn
        // height — the fluid surface never spawns a body (it sinks).
        // The scan walks the world-center column down from spawn_y.
        let (cx, cz) = (self.world_size / 2, self.world_size / 2);
        let mut feet_y = self.spawn_y;
        if let Some(w) = self.offline_world.as_ref() {
            let mut y = self.spawn_y - 1;
            while y >= 0 {
                let id = w.get(cx, y, cz);
                if id != crate::world::AIR && id != crate::world::WATER {
                    feet_y = y + 1;
                    break;
                }
                y -= 1;
            }
        }
        let (yaw, pitch) = crate::world_view::wire_angles(&self.world_camera());
        self.body = Some(crate::body::PlayerBody::new(
            n / 2.0,
            feet_y as f64,
            n / 2.0,
            yaw,
            pitch,
        ));
        // THE SPAWN POSE (the respawn's own re-anchor: the vanilla
        // respawn teleports the player to the spawn point — the pose
        // the body materialized at is the pose the respawn restores)
        self.spawn_pose = Some((n / 2.0, feet_y as f64, n / 2.0, yaw, pitch));
    }

    /// The look deltas (the windowed loop's per-frame feed — the mouse
    /// stream the vanilla MouseHandler speaks): the body turns through
    /// the turnPlayer curve (the sensitivity 0.5 slider, the famous
    /// 0.15 deg/px). No-op without a body (the headless scenes) or
    /// while paused (the pause freezes the look — the vanilla's own
    /// pause law).
    pub fn on_look_delta(&mut self, dx: f64, dy: f64) {
        if self.screen == ScreenKind::World && !self.paused {
            if let Some(b) = self.body.as_mut() {
                b.turn(dx, dy, crate::body::LOOK_SENSITIVITY_DEFAULT);
            }
        }
    }

    /// The movement keys (the windowed loop's per-frame feed): the
    /// impulses (the KeyboardInput's own ±1 shape) and the state keys
    /// the body's tick consumes. The SPRINT gate is the vanilla
    /// canStartSprinting's own core: the forward impulse past 0.8, the
    /// crouch refused, and the FOOD GATE the
    /// hasEnoughFoodToDoExhaustiveManoeuvres speaks (the bar over
    /// SPRINT_LEVEL 6 — a starving body walks).
    pub fn on_movement_input(
        &mut self,
        forward: f32,
        left: f32,
        jump: bool,
        crouch: bool,
        sprint: bool,
    ) {
        if let Some(b) = self.body.as_mut() {
            b.forward = forward;
            b.left = left;
            b.jumping = jump;
            b.want_crouch = crouch;
            b.sprinting = sprint
                && forward > 0.8
                && !crouch
                && self.food_data.has_enough_food();
        }
    }

    /// The body's sendPosition dispatch -> the wire command (the
    /// container channel's own ride).
    fn move_wire_command(w: crate::body::MoveWire) -> crate::world_view::ContainerCommand {
        match w {
            crate::body::MoveWire::PosRot {
                x,
                y,
                z,
                yaw,
                pitch,
                on_ground,
                horizontal_collision,
            } => crate::world_view::ContainerCommand::MovePosRot {
                x,
                y,
                z,
                yaw,
                pitch,
                on_ground,
                horizontal_collision,
            },
            crate::body::MoveWire::Pos {
                x,
                y,
                z,
                on_ground,
                horizontal_collision,
            } => crate::world_view::ContainerCommand::MovePos {
                x,
                y,
                z,
                on_ground,
                horizontal_collision,
            },
            crate::body::MoveWire::Rot {
                yaw,
                pitch,
                on_ground,
                horizontal_collision,
            } => crate::world_view::ContainerCommand::MoveRot {
                yaw,
                pitch,
                on_ground,
                horizontal_collision,
            },
            crate::body::MoveWire::StatusOnly {
                on_ground,
                horizontal_collision,
            } => crate::world_view::ContainerCommand::MoveStatus {
                on_ground,
                horizontal_collision,
            },
        }
    }

    /// The OFFLINE held-item lane — the `--held-item` model id's own
    /// route into the interaction domain: the wire id resolves through
    /// the registry's reverse lookup (the namespace strip routes the
    /// model id's `item/<name>` vocabulary), the payload loads through
    /// the SAME routing the networked store lane speaks. `None` = the
    /// empty selection (the bare arm). The windowed interactive client's
    /// own lane — the headless scenes never press keys.
    pub fn offline_held_updated(&mut self, model: Option<&str>) {
        let id = model
            .and_then(|m| m.strip_prefix("item/").or(Some(m)))
            .and_then(mc_registry::item_id_by_name);
        self.offline_held = id;
        self.load_held_payload_from_id(id);
    }

    /// The unified held id — the networked store's selection first, the
    /// offline `--held-item` stand-in second (the lanes are exclusive:
    /// a live session clears the offline stand-in's authority by owning
    /// the screen). The press lanes' own key.
    fn held_id(&self) -> Option<i32> {
        // the live lane's store selection speaks first (the networked
        // world's own)
        if let Some(id) = self.live_held {
            return Some(id);
        }
        // the offline gameplay arc: the inventory's selected stack (the
        // pickups' own lane) speaks before the `--held-item` stand-in —
        // the golden scenes' empty inventory falls through, the stand-in
        // lane stays the goldens' own
        let sel = self.inventory.items[self.selected_hotbar];
        if sel.1 > 0 {
            return Some(sel.0);
        }
        self.offline_held
    }

    /// The block-use lane's world — the screen's OWN terrain: the
    /// networked world first (the wire echo's authority), the offline
    /// procedural voxels second (the placement prediction's own lane).
    fn active_world(&self) -> Option<&crate::world::World> {
        if self.world_is_network {
            self.net_world.as_ref()
        } else {
            self.offline_world.as_ref()
        }
    }

    /// The dig lane's mutable world accessor (the ACTIVE world's own
    /// voxels — the machine reads them either way; the writes ride the
    /// apply_locally flag the entry lanes set).
    fn active_world_mut(&mut self) -> Option<&mut crate::world::World> {
        if self.world_is_network {
            self.net_world.as_mut()
        } else {
            self.offline_world.as_mut()
        }
    }

    /// The crosshair pick — the vanilla GameRenderer.pick's own shape:
    /// the active camera's eye along the view direction through the
    /// BLOCK_INTERACTION_RANGE reach (the dig and the placement speak
    /// the same clip).
    fn clip_crosshair(&self) -> Option<crate::world::ClipHit> {
        self.active_world().and_then(|w| {
            let cam = self.world_camera();
            let f = cam.forward();
            w.clip(
                (cam.pos.x as f64, cam.pos.y as f64, cam.pos.z as f64),
                (f.x as f64, f.y as f64, f.z as f64),
                crate::world::BLOCK_INTERACTION_RANGE,
            )
        })
    }

    /// The body state the dig speed law speaks: (on ground, the eye in
    /// water). The body-less scenes speak the honest defaults (grounded,
    /// dry) — the flight camera's own belief; the eye samples the cell
    /// the camera stands in (the vanilla isEyeInFluid's own query).
    fn body_state(&self) -> (bool, bool) {
        let cam = self.world_camera();
        let on_ground = self.body.as_ref().map(|b| b.on_ground).unwrap_or(true);
        let eye_water = self
            .active_world()
            .map(|w| {
                w.get(
                    cam.pos.x.floor() as i32,
                    cam.pos.y.floor() as i32,
                    cam.pos.z.floor() as i32,
                ) == crate::world::WATER
            })
            .unwrap_or(false);
        (on_ground, eye_water)
    }

    /// The dig sends ride the live session's own command channel (the
    /// offline lane has no session — the sends die honestly on the
    /// frame thread, exactly like the placement's echo).
    fn enqueue_dig(&mut self, sends: &[crate::dig::DigSend]) {
        if sends.is_empty() {
            return;
        }
        if let Some(sink) = self.live_clicks.as_ref() {
            for s in sends {
                let _ = sink.send(crate::world_view::ContainerCommand::DigAction {
                    action: s.action,
                    pos: s.pos,
                    face: s.face,
                });
            }
        }
    }

    /// The offline mesh re-speaks the edited voxels (the placement's own
    /// rhythm — the networked world's mesh is the server's refresh).
    fn remesh_offline(&mut self) {
        if !self.world_is_network {
            if let Some(world) = self.offline_world.as_ref() {
                self.world_quads = Some(crate::mesher::mesh_world(world, None));
            }
        }
    }

    /// THE REWARD HALF — `Block.dropResources`'s own call the completion
    /// speaks: the loot roll (the generated hand-lane table, the TOOL
    /// GATE riding block_dig's own requiresCorrectToolForDrops flag —
    /// the bare hand's stone refuses everything), then one item entity
    /// per drop by the popResource law (the ±0.25 spread, the 0.125
    /// lift, the 10-tick pickup delay, the render seed). The OFFLINE
    /// lane's own authority: the networked world's entities are the
    /// server's own stream.
    fn spawn_drops_for(&mut self, kind: u16, pos: (i32, i32, i32)) {
        if self.world_is_network {
            return;
        }
        // THE MINE'S OWN COST (javap Block.playerDestroy: the
        // awardStat(BLOCK_MINED) tail's inlined 0.005 — the
        // causeFoodExhaustion the destroyBlock's own reward lane
        // speaks; the offline gate rides the drops' own)
        self.food_data.add_exhaustion(crate::food::EXHAUSTION_MINE);
        for (item, count) in crate::drops::roll_drops(kind, &mut self.drop_rng) {
            let id = self.next_drop_id;
            self.next_drop_id += 1;
            self.drops.push(crate::drops::ItemDrop::spawn(
                id, item, count, pos.0, pos.1, pos.2, &mut self.drop_rng,
            ));
        }
    }

    /// THE BLOCK-USE LANE — the vanilla startUseItem's block branch:
    /// the crosshair clip (the eye along the view direction, the
    /// BLOCK_INTERACTION_RANGE's 4.5), the useItemOn echo riding EVERY
    /// block right-click (the offline stand-in has no session — no
    /// echo), then the placement: the block item's own CONSUME branch
    /// (the held stack's block lands in the hit face's adjacent cell —
    /// the vanilla canPlace's own laws: the cell replaceable (the air
    /// and the fluid), the placed shape not intersecting the player's
    /// own bounding box). The OFFLINE world predicts the voxel in place
    /// (the mesh re-speaks); the NETWORKED world's placement is the
    /// server's own chunk refresh. Returns whether the placement
    /// consumed the press (the item's use lane falls through on the
    /// PASS — the vanilla's own order).
    fn on_right_press_block_lane(&mut self) -> bool {
        let Some(hit) = self.active_world().and_then(|w| {
            let cam = self.world_camera();
            let f = cam.forward();
            w.clip(
                (cam.pos.x as f64, cam.pos.y as f64, cam.pos.z as f64),
                (f.x as f64, f.y as f64, f.z as f64),
                crate::world::BLOCK_INTERACTION_RANGE,
            )
        }) else {
            return false;
        };
        // THE ECHO: the useItemOn packet rides every block right-click
        // (the cursor = the hit location minus the block origin, the
        // double math the wire's writeBlockHitResult speaks)
        let cursor = (
            (hit.location.0 - hit.pos.0 as f64) as f32,
            (hit.location.1 - hit.pos.1 as f64) as f32,
            (hit.location.2 - hit.pos.2 as f64) as f32,
        );
        if let Some(sink) = self.live_clicks.as_ref() {
            let _ = sink.send(crate::world_view::ContainerCommand::UseItemOn {
                pos: hit.pos,
                face: hit.face,
                cursor,
                inside: hit.inside,
            });
        }
        // THE PLACEMENT: the block item's CONSUME branch (the eye-inside
        // hit never places — the vanilla's own guard)
        if hit.inside {
            return false;
        }
        let Some(id) = self.held_id() else {
            return false;
        };
        let Some(block) = mc_registry::item_name(id)
            .and_then(crate::world::World::placeable_block)
        else {
            return false; // the PASS result — the item's use lane follows
        };
        let normal = crate::world::World::DIRECTION_NORMAL[hit.face as usize];
        let cell = (
            hit.pos.0 + normal.0,
            hit.pos.1 + normal.1,
            hit.pos.2 + normal.2,
        );
        // the replaceable-cell law: the air and the fluid give way (the
        // vanilla state.canBeReplaced's own set for our vocabulary)
        let replaceable = matches!(
            self.active_world().map(|w| w.get(cell.0, cell.1, cell.2)),
            Some(crate::world::AIR) | Some(crate::world::WATER)
        );
        if !replaceable {
            return false;
        }
        // the entity law: the placed shape must not intersect the
        // player's own bounding box (the feet ±0.3, the height 1.8 —
        // the vanilla noCollision's own check, the strict edges)
        let eye = self.world_camera().pos;
        let feet = (eye.x as f64, (eye.y - 1.62) as f64, eye.z as f64);
        let intersect = |pmin: f64, pmax: f64, clo: i32| {
            (clo as f64) < pmax && ((clo + 1) as f64) > pmin
        };
        let hits_player = intersect(feet.0 - 0.3, feet.0 + 0.3, cell.0)
            && intersect(feet.1, feet.1 + 1.8, cell.1)
            && intersect(feet.2 - 0.3, feet.2 + 0.3, cell.2);
        if hits_player {
            return false; // the vanilla refuses the placement into itself
        }
        // THE OFFLINE PREDICTION: the voxel lands in the App's own
        // terrain and the mesh re-speaks (the networked world's
        // placement rides the server's chunk refresh — no local edit)
        if let (Some(world), false) = (self.offline_world.as_mut(), self.world_is_network) {
            world.set(cell.0, cell.1, cell.2, block);
            self.world_quads = Some(crate::mesher::mesh_world(world, None));
            // THE SURVIVAL CONSUME (BlockItem.place's own shrink): the
            // held stack came from the inventory's selection — the count
            // rides down, the empty slot clears. The `--held-item`
            // stand-in (the golden scenes' lane) speaks no stack: the
            // consume is the honest no-op it always was.
            let sel = self.selected_hotbar;
            let stack = self.inventory.items[sel];
            if stack.1 > 0 && stack.0 == id {
                self.inventory.items[sel].1 -= 1;
                if self.inventory.items[sel].1 <= 0 {
                    self.inventory.items[sel] = (0, 0);
                }
                self.hotbar_slots = self.inventory.hotbar();
            }
        }
        // THE PLACE SOUND: BlockItem.place's own fold — the placed block's
        // SoundType through (v+1)/2, p*0.8. The networked prediction
        // speaks too (the client hears its own placement, the server's
        // echo never doubles it — the vanilla's own singleplayer shape).
        let name = crate::world::NAME_OF[block as usize];
        self.sounds.play(self.frame, name, crate::sound::Lane::Place);
        true
    }

    /// The networked held-item lane: the store's selected hotbar stack
    /// drives the first-person hand exactly like the vanilla
    /// renderItemInHand's own projection — the id's RESTING-HAND layers
    /// load on change (the plain roots as their single layer, the
    /// complex trees through the generated resolver — condition
    /// on_false, select fallback, threshold-0 dispatch, the composite
    /// children with their transformations), and the emptiness (a
    /// count-0 slot, a slot the store never spoke) or an id outside the
    /// bake domain (the special block-entity renderers — no model JSON
    /// exists) drops back to the bare arm. The count is not the hand's
    /// lane (vanilla renders no count in first person); the reload
    /// fires only when the id changes — the store's refresh cadence
    /// does not re-bake. The OFFLINE headless `--held-item` keeps
    /// owning its own surface (the goldens' lane — this method is the
    /// live session's only, the headless scenes never call it).
    pub fn live_held_updated(&mut self, held: Option<(i32, i32)>) {
        let id = held.filter(|&(_, count)| count > 0).map(|(id, _)| id);
        if self.live_held == id {
            return; // no change — the loaded model stands
        }
        self.live_held = id;
        // the item changed: the use ends (the vanilla releaseUsingItem
        // on the item switch — the hold cannot outlive its own stack)
        // and the charge goes with the STACK it belonged to (the
        // component is the loaded crossbow's own, never the slot's)
        self.using_item = None;
        // the charge clears with the old stack (the early return above
        // guarded the equal case — here the id always changed)
        self.held_charge = None;
        // the resolution order: the 63 SPECIAL renderer payloads first
        // (the dump-backed geometry lane), then the resting-hand layer
        // tables (the plain defs + the resolved trees) — the lanes are
        // disjoint (the registry's own census tests), so the order is a
        // routing, never a shadowing
        self.load_held_payload_from_id(id);
    }

    /// The resting payload by wire id (the routing live_held_updated
    /// spoke before the using arc existed — the specials' disjoint
    /// lanes first, then the layer tables).
    fn load_held_payload_from_id(&mut self, id: Option<i32>) {
        self.held_item = id.and_then(|id| {
            if mc_registry::special_item_renderer(id).is_some() {
                return crate::hand::load_special_item(id);
            }
            mc_registry::item_model_layers(id)
                .and_then(|layers| crate::hand::load_item_layers(&layers))
        });
    }

    /// Whether a use is active (the vanilla isUsingItem's own query —
    /// the tests' honest probe).
    pub fn is_using(&self) -> bool {
        self.using_item.is_some()
    }

    /// The networked HUD lane: the store's WHOLE hotbar row lands
    /// verbatim (the vanilla renderHotbar's projection — the icons and
    /// the counts are the store's business, the emptiness `(0, 0)` the
    /// wire itself named, never a guessed stack). The replacement is
    /// wholesale: the store's every refresh re-speaks the row, so the
    /// HUD's belief never outlives it — exactly like the chest's slots.
    /// The OFFLINE world carries no store (the row stays empty, the
    /// hotbar renders chrome only — the headless scenes never feed
    /// this lane, the goldens' pixels stand).
    pub fn live_hotbar_updated(&mut self, hotbar: [(i32, i32); 9]) {
        self.hotbar_slots = hotbar;
    }

    /// The vitals' lane (the Set Health / Hurt Animation / Combat Kill
    /// wire's projection — the HealthView the refresh ships): the
    /// store's truth lands verbatim, the death gate follows the
    /// message's arrival (the vanilla handlePlayerCombatKill's own
    /// setScreen(new DeathScreen)).
    pub fn live_health_updated(&mut self, health: crate::world_view::HealthView) {
        self.health = health.health;
        // THE FOOD LANE'S LANDING (handleSetHealth's own assignments:
        // foodData.setFoodLevel(food); foodData.setSaturation(sat) —
        // the packet carries no exhaustion, the bank keeps its own)
        self.food_data.set_food_level(health.food);
        self.food_data.set_saturation(health.saturation);
        self.hurt_time = health.hurt_time;
        self.hurt_yaw = health.hurt_yaw;
        self.death_message = health.death_message.clone();
        self.dead = self.death_message.is_some() || (self.dead && self.health <= 0.0);
        // THE EFFECTS' MIRROR (the model's own-effect store's snapshot —
        // the wire's Update/Remove Mob Effect packets' filtered truth;
        // the store's add/update laws ride the same code the offline
        // lane speaks, the absorption's lift rides the application)
        self.effects
            .sync_from_wire(&health.effects.iter().map(|e| (e.effect_id, crate::world_model::MobEffectState {
                amplifier: e.amplifier,
                duration: e.duration,
                flags: e.flags,
            })).collect::<Vec<_>>());
    }

    /// THE FALL DAMAGE PIPELINE (javap `LivingEntity.causeFallDamage`'s
    /// own order): the fall voice first — the FIXED 1.0/1.0
    /// `getFallDamageSound(damage)` (the small/big boundary at damage
    /// 5), then the hurt through the amount `calculateFallDamage`
    /// spoke.
    /// The held item's FOOD/CONSUMABLE row — the registry's item name
    /// through the generated foods table (the probe's 47-row verdict).
    fn food_def_of(&self, id: i32) -> Option<&'static mc_registry::generated::foods::FoodDef> {
        let name = mc_registry::item_name(id)?;
        mc_registry::generated::foods::food_def_by_name(name)
    }

    /// THE CHEW BURST (javap emitParticlesAndSounds + spawnItemParticles
    /// verbatim): the voice's four draws FIRST (the boolean, the two
    /// triangle floats, the randomBetween float — the entity random's
    /// own order), then the 5 pieces' draws (the offset/velocity pair
    /// per piece, the parent's jitter fold, the lifetime, the
    /// texture-slice pair). Returns the (volume, pitch) the voice logs
    /// with — the engine's file pick rides the caller.
    fn spawn_chew_burst(&mut self, eat: &crate::eat::EatActive) -> (f32, f32) {
        let r_bool = self.drop_rng.next_f32() >= 0.5;
        let r1 = self.drop_rng.next_f32();
        let r2 = self.drop_rng.next_f32();
        let r3 = self.drop_rng.next_f32();
        let (volume, pitch) = crate::eat::chew_voice(eat.animation, r_bool, r1, r2, r3);
        if eat.has_consume_particles {
            self.spawn_pieces(eat.item, crate::eat::CHEW_BURST_COUNT);
        }
        (volume, pitch)
    }

    /// THE PIECE SPAWNER (javap spawnItemParticles): `count` pieces at
    /// the eye + the rotated offset, riding the rotated mouthward
    /// velocity through the Particle + BreakingItemParticle folds.
    fn spawn_pieces(&mut self, item: i32, count: u32) {
        let cam = self.world_camera();
        let (yaw, pitch) = (cam.yaw, cam.pitch);
        let eye = [cam.pos.x as f64, cam.pos.y as f64, cam.pos.z as f64];
        for _ in 0..count {
            // the offset's two draws, the velocity's two
            let r_ox = self.drop_rng.next_f32();
            let r_oy = self.drop_rng.next_f32();
            let r_vx = self.drop_rng.next_f32();
            let r_vy = self.drop_rng.next_f32();
            let (off, vel) =
                crate::eat::piece_offset_and_velocity(r_ox, r_oy, r_vx, r_vy, yaw, pitch);
            // the parent's jitter fold (six draws) + the normalize's
            // d0 (two draws)
            let jx = self.drop_rng.next_f32();
            let jy = self.drop_rng.next_f32();
            let jz = self.drop_rng.next_f32();
            let r1 = self.drop_rng.next_f32();
            let r2 = self.drop_rng.next_f32();
            let parent = crate::eat::parent_velocity(
                vel[0] as f32,
                vel[1] as f32,
                vel[2] as f32,
                jx,
                jy,
                jz,
                r1,
                r2,
            );
            let final_v = crate::eat::breaking_item_velocity(parent, vel[0] as f32, vel[1] as f32, vel[2] as f32);
            // the lifetime's draw, the texture slice's two
            let life = crate::eat::particle_lifetime(self.drop_rng.next_f32());
            let uo = self.drop_rng.next_f32() * 3.0;
            let vo = self.drop_rng.next_f32() * 3.0;
            self.pieces.push(crate::eat::ItemPiece {
                x: eye[0] + off[0],
                y: eye[1] + off[1],
                z: eye[2] + off[2],
                vx: final_v[0],
                vy: final_v[1],
                vz: final_v[2],
                age: 0,
                lifetime: life,
                item,
                uo,
                vo,
            });
        }
    }

    /// THE COMPLETION (javap Consumable.onConsume then
    /// FoodProperties.onConsume verbatim): the FINAL 16-piece burst
    /// with the chew's own voice law, the gulp at the fixed 1.0 /
    /// triangle(1.0, 0.4), the OFFLINE lane's FoodData.eat (the
    /// networked lane's food is the server's own — the Set Health
    /// packet speaks it, the client folds nothing), THE BURP at 0.5 /
    /// randomBetween(0.9, 1.0), and the survival's consume(1) with the
    /// USE_REMAINDER's return lane. The game event and the
    /// server-side effect list ride the honest absences (no
    /// GameEvent channel, no effect store — the effects' rows already
    /// speak in the table).
    fn complete_eat(&mut self, eat: &crate::eat::EatActive) {
        // 1. the finisher: 16 pieces + the chew voice's fold
        let (volume, pitch) = self.spawn_chew_burst(eat);
        let event = eat.sound.trim_start_matches("minecraft:");
        self.sounds.play_voice(self.frame, event, volume, pitch);
        // 2. THE FOOD LISTENER's lane (only FOOD rows carry it — the
        // potion's completion never speaks here)
        if eat.has_food() {
            // the gulp: the fixed volume, the ±0.4 triangle (two draws)
            let g1 = self.drop_rng.next_f32();
            let g2 = self.drop_rng.next_f32();
            let gulp_pitch = crate::eat::triangle(g1, g2, 1.0, crate::eat::FINISH_GULP_DEVIATION);
            self.sounds.play_voice(self.frame, event, crate::eat::FINISH_GULP_VOLUME, gulp_pitch);
            // 3. THE OFFLINE FOLD: FoodData.eat(props) — the ABSOLUTE
            // saturation straight into the add (the 26.2 record's own
            // verdict). The networked lane folds nothing (the
            // server's Set Health speaks its truth).
            if !self.world_is_network {
                self.food_data.eat_props(eat.nutrition, eat.saturation);
            }
            // 4. THE BURP (javap FoodProperties.onConsume's tail):
            // volume 0.5, pitch randomBetween(0.9, 1.0) — one draw
            let b = self.drop_rng.next_f32();
            let burp_pitch = crate::eat::random_between(
                b,
                crate::eat::BURP_PITCH_MIN,
                crate::eat::BURP_PITCH_MAX,
            );
            self.sounds.play_voice(
                self.frame,
                "entity.player.burp",
                crate::eat::BURP_VOLUME,
                burp_pitch,
            );
        }
        // 3c. THE STATUS-EFFECTS' PAYLOAD ARC (javap
        // `ApplyStatusEffectsConsumeEffect.apply`: ONE roll gates
        // the whole payload — `nextFloat() < probability` — then
        // every row rides `addEffect` verbatim, the copy
        // constructor's own lane; the random is the entity's own
        // `getRandom()`, the stand-in the drop lane's Lcg. The
        // listener stream drew FIRST (the gulp's two, the burp's
        // one), the effect list applies SECOND — the onConsume
        // chain's own order. The networked lane folds nothing (the
        // server applies, the wire's Update Mob Effect speaks).
        if !self.world_is_network {
            let def = mc_registry::item_name(eat.item)
                .and_then(mc_registry::generated::foods::food_def_by_name);
            if let Some(def) = def {
                for ce in def.effects {
                    match ce.kind {
                        mc_registry::generated::foods::EffectKind::ApplyStatusEffects => {
                            let roll = self.drop_rng.next_f32();
                            if roll < ce.probability {
                                for row in ce.rows {
                                    if let Some(eid) =
                                        crate::effects::effect_id_by_name(row.effect)
                                    {
                                        self.effects.add_effect(
                                            eid,
                                            row.amplifier,
                                            row.duration,
                                            crate::effects::DEFAULT_EFFECT_FLAGS,
                                        );
                                    }
                                }
                            }
                        }
                        mc_registry::generated::foods::EffectKind::ClearAllStatusEffects => {
                            self.effects.remove_all();
                        }
                        // the other kinds' client surfaces — the
                        // chorus fruit's teleport (the world's own
                        // lane), the play-sound's one-shot — stay
                        // their documented absences (no wire, no
                        // world surface in the offline arc yet)
                        _ => {}
                    }
                }
            }
        }
        // 5. THE SURVIVAL CONSUME (javap ItemStack.consume(1, entity):
        // the infinite-materials gate off — the client's survival
        // lane) + the USE_REMAINDER's return (the milk's bucket, the
        // bottles): a single stack BECOMES the remainder, a larger
        // stack shrinks and the remainder rides the inventory's own
        // add. The `--held-item` stand-in (no stack) stays the honest
        // no-op.
        if !self.world_is_network {
            let sel = self.selected_hotbar;
            let stack = self.inventory.items[sel];
            if stack.1 > 0 && stack.0 == eat.item {
                match eat.use_remainder {
                    Some(rem_name) => {
                        let rem_id = mc_registry::item_id_by_name(
                            rem_name.trim_start_matches("minecraft:"),
                        )
                        .unwrap_or(0);
                        if stack.1 == 1 {
                            self.inventory.items[sel] = (rem_id, 1);
                        } else {
                            self.inventory.items[sel].1 -= 1;
                            let leftover = self.inventory.add(sel, rem_id, 1);
                            if leftover > 0 {
                                // the full inventory's honesty: the
                                // remainder drops at the body (the
                                // handleExtraItemsCreatedOnUse's own
                                // lane) — the spawn the drops module
                                // already speaks
                                self.spawn_remainder_drop(rem_id);
                            }
                        }
                    }
                    None => {
                        self.inventory.items[sel].1 -= 1;
                        if self.inventory.items[sel].1 <= 0 {
                            self.inventory.items[sel] = (0, 0);
                        }
                    }
                }
                self.hotbar_slots = self.inventory.hotbar();
            }
        }
        self.eating = None;
    }

    /// The full-inventory remainder's drop (the
    /// handleExtraItemsCreatedOnUse's own lane — the entity spawns at
    /// the body with the pickup delay the ItemEntity law speaks).
    fn spawn_remainder_drop(&mut self, item: i32) {
        if let Some(b) = self.body.as_ref() {
            let id = self.next_drop_id;
            self.next_drop_id += 1;
            self.drops.push(crate::drops::ItemDrop::spawn(
                id,
                item,
                1,
                b.x.floor() as i32,
                b.y.floor() as i32,
                b.z.floor() as i32,
                &mut self.drop_rng,
            ));
        }
    }

    fn apply_fall_damage(&mut self, damage: i32) {
        self.sounds.play_fall_voice(self.frame, damage);
        // THE DAMAGE SOURCE'S OWN FOOD COST (javap Player.hurt's tail:
        // `causeFoodExhaustion(source.getFoodExhaustion())` — the
        // damage type's data-driven field; the fall type's own JSON
        // verdict 0.0, the lane's visibility the law keeps)
        self.food_data
            .add_exhaustion(crate::food::FALL_DAMAGE_EXHAUSTION);
        self.hurt(damage as f32);
    }

    /// THE HURT LAW (javap `hurtServer`'s own pipeline, the LOCAL
    /// prediction's voice): the 20-tick cooldown's own comparison — a
    /// hurt inside the 10-tick window must EXCEED the last hurt and
    /// lands for the difference only; a fresh hurt re-arms the window,
    /// the 10-tick flash (`hurtTime = hurtDuration`), and the health
    /// drops through actuallyHurt (the armor/absorption lanes' honest
    /// absence — the bare player's own path).
    pub fn hurt(&mut self, amount: f32) {
        if amount <= 0.0 || self.dead {
            return;
        }
        if (self.invulnerable_time as f32) > 10.0 {
            if amount <= self.last_hurt {
                return;
            }
            self.actually_hurt(amount - self.last_hurt);
            self.last_hurt = amount;
        } else {
            self.last_hurt = amount;
            self.invulnerable_time = 20;
            self.hurt_duration = 10;
            self.hurt_time = self.hurt_duration;
            self.actually_hurt(amount);
            // the hurt voice (handleDamageEvent's own call — the
            // client's own law speaks the (r - r) * 0.2 + 1.0 pitch)
            self.sounds.play_hurt_voice(self.frame);
        }
    }

    /// THE ACTUAL HURT (javap `actuallyHurt`'s bare-player path — no
    /// armor, the full amount rides the ABSORPTION LANE first — and the
    /// death's own tail: the health at zero arms the death screen (the
    /// Gui.tick's auto-open) and speaks the death voice (the hurtServer
    /// tail's `makeSound(getDeathSound())`).
    /// THE ABSORPTION'S TWO DRAINS (the 26.2 bytecode's own verdict): the
    /// health-lane share clamps at zero (`max(0, amount - absorption)`),
    /// the absorption absorbs the remainder (drain one), and the health's
    /// own damage drains the absorption AGAIN (drain two — the second
    /// `setAbsorptionAmount(getAbsorptionAmount() - amount)` the
    /// bytecode speaks). A fully-absorbed hit never touches the health
    /// and never reaches the combat/death tail (the `amount == 0`
    /// return).
    fn actually_hurt(&mut self, amount: f32) {
        // THE HURT FLINCH (LivingEntity.handleDamageEvent's own opening
        // move): the walk speed kicks to 1.5 — the third-person
        // avatar's limbs flail through the hurt tick
        if let Some(b) = &mut self.body {
            b.walk.set_speed(crate::body::HURT_WALK_SPEED);
        }
        let total = amount;
        let health_share = (total - self.effects.absorption()).max(0.0);
        self.effects
            .set_absorption(self.effects.absorption() - (total - health_share));
        if health_share <= 0.0 {
            return;
        }
        self.health = (self.health - health_share).clamp(0.0, MAX_HEALTH);
        self.effects
            .set_absorption(self.effects.absorption() - health_share);
        if self.health <= 0.0 && !self.dead {
            self.dead = true;
            self.sounds.play_death_voice(self.frame);
        }
    }

    /// THE RESPAWN (the DeathScreen's Respawn button's own action): the
    /// vitals reset (the health back to max, the hurt fields zero), the
    /// body re-anchors to the spawn pose (the vanilla respawn's own
    /// teleport), and the live session echoes the Client Command's
    /// PERFORM_RESPAWN (the server re-streams position and vitals).
    pub fn respawn(&mut self) {
        self.dead = false;
        self.death_message = None;
        self.health = MAX_HEALTH;
        // THE FOOD LANE'S OWN RESET (the server's respawn re-creates
        // the player's stats — the fresh FoodData the constructor
        // speaks: the full bar, the 5.0 saturation, the zero bank)
        self.food_data = crate::food::FoodData::new();
        // THE STATUS-EFFECTS' OWN RESET (the death's removal — the
        // vanilla death clears the active effects, the fresh respawn's
        // body carries none, the absorption value rides the dead
        // body's history)
        self.effects = crate::effects::EffectsStore::new();
        self.hurt_time = 0;
        self.hurt_duration = 0;
        self.invulnerable_time = 0;
        self.last_hurt = 0.0;
        if let (Some(b), Some((x, y, z, yaw, pitch))) = (self.body.as_mut(), self.spawn_pose) {
            b.x = x;
            b.y = y;
            b.z = z;
            b.yaw = yaw;
            b.pitch = pitch;
            b.vx = 0.0;
            b.vy = 0.0;
            b.vz = 0.0;
            b.fall_distance = 0.0;
            b.fall_event = None;
            // THE WALK STORE'S OWN RESET (the vanilla respawn
            // re-creates the entity — the fresh WalkAnimationState the
            // constructor speaks)
            b.walk.stop();
        }
        if let Some(sink) = self.live_clicks.as_ref() {
            let _ = sink.send(crate::world_view::ContainerCommand::Respawn);
        }
    }

    /// The networked container lane: the live store's projection drives
    /// the chest screen exactly like the vanilla client's — the Open
    /// Screen packet SHOWS the window over the world (from the World
    /// screen; a window arriving under other screens — pause, chat —
    /// waits honestly, no yank), the Set Content / Set Slot packets
    /// update its slots IN PLACE while it shows, and the store's drop
    /// (the Container Close packet) returns to the world. Menu types
    /// other than the chest (generic_9x3 = 2) are honest drops for
    /// now — their screens are their own rounds' lines; the store
    /// itself keeps them (the model lane is menu-agnostic).
    fn live_container_updated(
        &mut self,
        container: Option<crate::world_view::LiveContainer>,
    ) {
        match container {
            Some(c) => {
                if c.menu_type_id != crate::world_model::MENU_TYPE_CHEST {
                    return; // not the wired menu — the honest drop
                }
                // the menu's own numbering: 0..27 the container grid,
                // the tail the player mirror (27 main + 9 hotbar) —
                // padded to the layout's full shape, the wire's own
                // (0, 0) emptiness riding verbatim
                let mut grid = vec![(0i32, 0i32); 27];
                for (i, &(id, count)) in c.slots.iter().take(27).enumerate() {
                    grid[i] = (id, count);
                }
                let mut mirror = vec![(0i32, 0i32); 36];
                for (i, &(id, count)) in c.slots.iter().skip(27).take(36).enumerate() {
                    mirror[i] = (id, count);
                }
                self.chest_slots = grid;
                self.chest_mirror = mirror;
                self.chest_is_live = true;
                // the window's identity rides in: the clicks echo
                // against THIS id and revision (the stateId lane)
                self.chest_live_window = Some((c.id, c.state_id));
                // the AUTHORITATIVE carried lands too: the server's own
                // cursor replaces whatever the prediction believed (the
                // correction is the wholesale replacement — the local
                // belief never outlives the store's projection)
                self.chest_carried = c.carried;
                if self.screen == ScreenKind::World {
                    self.open(ScreenKind::Chest);
                }
            }
            None => {
                // the store dropped: a LIVE chest closes back to the
                // world (the vanilla closeContainer's return); the
                // offline stand-in is not the live lane's to close
                if self.screen == ScreenKind::Chest && self.chest_is_live {
                    self.chest_is_live = false;
                    self.open(ScreenKind::World);
                }
                self.chest_live_window = None;
                self.chest_carried = (0, 0); // no window, no held stack
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
        self.body = None; // the level's body drops with it
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
                // HUD and pause overlay on top in GUI space. THE DEATH
                // GATE (the vanilla death screen's own shape): a dead
                // player's HUD sleeps and the DeathScreen renders
                // instead — the world stays beneath the red gradient.
                self.render_world_3d(fb);
                let mut g = Gui::new(fb, Gui::auto_scale(fb.w, fb.h));
                if self.dead {
                    let (gw, gh) = (g.w(), g.h());
                    self.render_death(&mut g, a, gw, gh);
                } else {
                    self.render_hud(&mut g, a);
                    if self.paused {
                        let (gw, gh) = (g.w(), g.h());
                        self.render_pause(&mut g, a, gw, gh);
                    }
                }
            }
            ScreenKind::Chest => {
                // the container screen covers the HUD (the panel draws
                // over the hotbar's ground): world underneath, dimmed,
                // then the generic_9x3 chrome — the pause flow's shape.
                self.render_world_3d(fb);
                let mut g = Gui::new(fb, Gui::auto_scale(fb.w, fb.h));
                let (gw, gh) = (g.w(), g.h());
                self.render_chest(&mut g, a, gw, gh);
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
        // THE BODY's own pose first — the first-person body the windowed
        // app feeds is the camera's highest authority (the vanilla
        // client's own position); the deterministic scenes never own a
        // body and keep the disciplines below bit for bit
        if let Some(body) = &self.body {
            let base = body.camera();
            // THE DETACHED CAMERA (Camera.setup's third-person branch):
            // a non-first-person perspective backs the camera off along
            // its own view (the mirrored type flips first), clipped by
            // the same `getMaxZoom` raycasts the vanilla speaks — over
            // the screen's own world (the offline procedural terrain,
            // or the networked world's voxels)
            if !self.perspective.is_first_person() {
                let world = if self.world_is_network {
                    self.net_world.as_ref()
                } else {
                    self.offline_world.as_ref()
                };
                return crate::camera::detached(
                    base,
                    self.perspective.is_mirrored(),
                    &mut |from, to| world.and_then(|w| crate::body::clip_ray(w, from, to)),
                );
            }
            return base;
        }
        if self.world_is_network {
            if let Some(pose) = self.net_camera {
                return pose;
            }
        } else if let Some(pose) = self.offline_camera {
            // the offline look-control pose (the seam the placement
            // lane's clip aims through)
            return pose;
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
        // THE DROP LANE's entities (the vanilla LevelRenderer's entity
        // pass: after the terrain, before the clouds) — the item
        // entities bob and spin at their world positions
        if !self.drops.is_empty() {
            self.render_drops(fb, &ctx);
        }
        // THE THIRD-PERSON AVATAR (the entity pass's own camera-entity
        // render: the body the detached camera watches — nothing in
        // first person)
        self.render_avatar(fb, &ctx, &cam);
        // THE EAT ARC's pieces (the BreakingItemParticle pass — the
        // vanilla's own particle layer rides the entity pass): the
        // mouth's debris as camera-facing quads
        if !self.pieces.is_empty() {
            self.render_pieces(fb, &ctx, &cam);
        }
        crate::sky::render_clouds(fb, &ctx, &cam, day_time, 0.0, self.world_cloud_fog_end);
        // THE DIG LANE's frame render (the vanilla LevelRenderer's own
        // pair, both after the terrain and before the hand): the hit
        // outline (the black wireframe on the crosshair's block — every
        // BLOCK pick speaks it, mining or not) and the breaking overlay
        // (the destroy_stage crack on the armed dig's cell, the progress
        // law's own stage index — the vanilla's destroyBlockProgress
        // state the level renderer draws)
        if let Some(hit) = &self.frame_pick {
            fb.draw_hit_outline(&ctx, hit.pos);
        }
        let stage = self.dig.destroy_stage();
        if (0..10).contains(&stage) {
            let tile = mc_registry::generated::atlas::DESTROY_TILES[stage as usize];
            let quads = crate::mesher::destroy_overlay_quads(self.dig.pos, tile);
            fb.draw_destroy_overlay(&ctx, &quads);
        }
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
        // the two per frame. The EAT POSE rides the item's draw (the
        // consumable's hold: the pull-in + the bite bob + the rotations —
        // the remaining the pose reads at the frame's own 0.5 partial).
        let eat_pose = self.eating.as_ref().map(|eat| {
            crate::eat::apply_eat_transform(
                eat.remaining(),
                0.5,
                eat.total,
                1.0, // the main hand's side (the right-handed view)
            )
        });
        if self.perspective.is_first_person() {
            if let Some(item) = &self.held_item {
                item.draw_with_pose(fb, ctx.proj, self.swing.get_attack_anim(0.5), eat_pose);
            } else if let Some(arm) = &self.arm {
                arm.draw(fb, ctx.proj, self.swing.get_attack_anim(0.5));
            }
        }
    }

    /// THE THIRD-PERSON AVATAR's frame render — the vanilla
    /// LevelRenderer's detached-camera entity pass: the camera entity's
    /// OWN model draws (after the terrain's depth, before the clouds),
    /// posed by the walk/hurt/crouch/swing state the body already
    /// speaks. First person draws nothing (the hand overlays take the
    /// lane — the same vanilla gate that hides this pass there).
    fn render_avatar(
        &mut self,
        fb: &mut crate::raster::Framebuf,
        ctx: &RenderCtx,
        cam: &crate::camera::Camera,
    ) {
        if self.perspective.is_first_person() {
            return; // the vanilla first-person law: no own-body render
        }
        let Some(body) = &self.body else {
            return; // the deterministic scenes own no body — nothing to watch
        };
        if self.avatar.is_none() {
            self.avatar =
                crate::avatar::AvatarModel::load(std::path::Path::new("assets-cache/jar"));
        }
        let Some(model) = &self.avatar else {
            return; // the skin asset failed its sha1 — the renderer stays honest
        };
        // THE RENDER STATE (LivingEntityRenderer.extractRenderState's own
        // sampling, the 0.5 partial the camera lane speaks)
        let state = crate::avatar::AvatarState {
            yaw_deg: body.yaw,
            pitch_deg: body.pitch,
            walk_pos: body.walk.position_at(crate::avatar::RENDER_PARTIAL),
            walk_speed: body.walk.speed_at(crate::avatar::RENDER_PARTIAL),
            crouching: body.crouching,
            attack_time: self.swing.get_attack_anim(0.5),
            attack_right: true, // the client's own right main hand
            age_ticks: self.frame as f32 + 0.5,
        };
        let feet = Vec3::new(body.x as f32, body.y as f32, body.z as f32);
        let quads = model.quads(&state, feet, cam.yaw, cam.pitch);
        let sampled: Vec<crate::raster::SampledQuad> = quads
            .iter()
            .map(|q| crate::raster::SampledQuad {
                pts: q.pts,
                uv: q.uv,
                tex: "",
                tint: [q.shade, q.shade, q.shade],
            })
            .collect();
        // the depth-tested entity draw — the skin sampler walks the
        // model's own verified pixels (the alpha-cutout law rides the
        // raster: the hat's empty region shows the world)
        fb.draw_sampled_quads(ctx.proj.mul(&ctx.view), &sampled, &mut |_tex, u, v| {
            let [r, g, b, a] = model.pixel(u, v);
            Some([r, g, b, a])
        });
    }

    /// THE DROP LANE's render — the vanilla `ItemEntityRenderer`'s own
    /// pose chain (javap submit): the entity origin carries
    /// `translate(0, bob + lift, 0)` and the `rotY(spin)` about it, the
    /// GROUND display transform speaks inside (the registry's posed
    /// chain — scale 0.5, the [0,3,0]/16 translation), and the model's
    /// own base lift rides `-minY + 1/16` over the posed quads' actual
    /// minimum. The face shade folds per quad (`Level.getShade`'s own
    /// table — the offline world carries no light grid, the same
    /// full-bright lane the procedural scenes mesh in). The count
    /// cluster rides the single-model lane (the HUD's count number
    /// speaks the stack size; the cluster's random spread polys are the
    /// count>1 render's future own).
    fn render_drops(&mut self, fb: &mut crate::raster::Framebuf, ctx: &RenderCtx) {
        use mc_registry::element_quads::Direction;
        // the face shade law (colors.rs's own table, the Cull enum order)
        let ground_shade = |d: Direction| match d {
            Direction::Up => 1.0f32,
            Direction::Down => 0.5,
            Direction::North | Direction::South => 0.8,
            Direction::West | Direction::East => 0.6,
        };
        let mut sampled: Vec<crate::raster::SampledQuad> = Vec::new();
        for d in &self.drops {
            let Some(layers) = mc_registry::item_model_layers(d.item) else {
                continue;
            };
            // the posed quads with their layer's tint jsons (the layer
            // boundary the tint resolution rides)
            let mut parts: Vec<(mc_registry::element_quads::Quad, &'static [&'static str])> =
                Vec::new();
            let mut ok = true;
            for layer in &layers {
                match mc_registry::display::posed_item_layer_quads(
                    layer.model,
                    layer.transform,
                    mc_registry::display::GROUND,
                ) {
                    Some(Ok(quads)) => {
                        for q in quads {
                            parts.push((q, layer.tints));
                        }
                    }
                    _ => {
                        ok = false;
                        break;
                    }
                }
            }
            if !ok || parts.is_empty() {
                continue; // the honest unknown renders nothing
            }
            // the model's own base lift over the posed quads' minimum
            let min_y = parts
                .iter()
                .flat_map(|(q, _)| q.vertices.iter().map(|v| v[1]))
                .fold(f32::INFINITY, f32::min);
            let age = d.age as f32;
            let m = Mat4::translation(d.x as f32, d.y as f32, d.z as f32)
                .mul(&Mat4::rot_y(crate::drops::spin(age, d.bob_offs)))
                .mul(&Mat4::translation(
                    0.0,
                    crate::drops::bob(age, d.bob_offs) + crate::drops::base_lift(min_y),
                    0.0,
                ));
            for (q, tints) in &parts {
                let base = icon_tint(tints);
                let sh = if q.shade { ground_shade(q.direction) } else { 1.0 };
                sampled.push(crate::raster::SampledQuad {
                    pts: q.vertices.map(|v| {
                        let p = m.transform_point(Vec3::new(v[0], v[1], v[2]));
                        [p[0], p[1], p[2]]
                    }),
                    uv: q.uvs,
                    tex: q.texture,
                    tint: [base[0] * sh, base[1] * sh, base[2] * sh],
                });
            }
        }
        if sampled.is_empty() {
            return;
        }
        let vp = ctx.proj.mul(&ctx.view);
        fb.draw_sampled_quads(vp, &sampled, &mut |tex, u, v| {
            match mc_registry::sprite_uv::sample_uv(tex, u, v) {
                mc_registry::sprite_uv::Sample::Pixel(c) => Some(c),
                mc_registry::sprite_uv::Sample::Missing(c) => Some(c),
                mc_registry::sprite_uv::Sample::Unknown => None,
            }
        });
    }

    /// THE EAT ARC's piece render — the vanilla SingleQuadParticle's
    /// own camera-facing quad (the roll stays 0, the corners ±1 scaled
    /// by the quad size 0.1), the BreakingItemParticle's 4x4-pixel
    /// texture slice riding the uo/vo law over the item's first sprite.
    /// The slice resolves per frame (the pieces are few, the layer
    /// fetch is the drops' own cost) — the unresolvable item renders
    /// nothing (the honest unknown).
    fn render_pieces(
        &mut self,
        fb: &mut crate::raster::Framebuf,
        ctx: &RenderCtx,
        cam: &crate::camera::Camera,
    ) {
        // the camera basis (the billboard's own axes — the vanilla folds
        // the camera rotation quaternion; the roll-0 default speaks it)
        let sy = cam.yaw.sin();
        let cy = cam.yaw.cos();
        let sp = cam.pitch.sin();
        let cp = cam.pitch.cos();
        let right = [cy, 0.0, sy];
        let up = [-sy * sp, cp, cy * sp];
        let mut sampled: Vec<crate::raster::SampledQuad> = Vec::new();
        for p in &self.pieces {
            let Some((tex, rect)) = piece_sprite(p.item) else {
                continue;
            };
            // the 4x4-pixel slice over the sprite's 16-px law: the
            // vanilla getU((uo + u) * 4 / 16) over the sprite's own
            // coords — the rect carries the quad's uv span (the block
            // models' 0..16 or the named sprite's own)
            let lerp = |lo: f32, hi: f32, f: f32| lo + (hi - lo) * f;
            let u0 = lerp(rect[0], rect[1], p.uo * 4.0 / 16.0);
            let u1 = lerp(rect[0], rect[1], (p.uo + 1.0) * 4.0 / 16.0);
            let v0 = lerp(rect[2], rect[3], p.vo * 4.0 / 16.0);
            let v1 = lerp(rect[2], rect[3], (p.vo + 1.0) * 4.0 / 16.0);
            let half = 0.1f32;
            let corner = |sx: f32, syp: f32| {
                [
                    p.x as f32 + (right[0] * sx + up[0] * syp) * half,
                    p.y as f32 + (right[1] * sx + up[1] * syp) * half,
                    p.z as f32 + (right[2] * sx + up[2] * syp) * half,
                ]
            };
            // the vanilla avector3f order: (-1,-1), (-1,1), (1,1), (1,-1)
            sampled.push(crate::raster::SampledQuad {
                pts: [corner(-1.0, -1.0), corner(-1.0, 1.0), corner(1.0, 1.0), corner(1.0, -1.0)],
                uv: [[u0, v1], [u0, v0], [u1, v0], [u1, v1]],
                tex,
                tint: [1.0, 1.0, 1.0],
            });
        }
        if sampled.is_empty() {
            return;
        }
        let vp = ctx.proj.mul(&ctx.view);
        fb.draw_sampled_quads(vp, &sampled, &mut |tex, u, v| {
            match mc_registry::sprite_uv::sample_uv(tex, u, v) {
                mc_registry::sprite_uv::Sample::Pixel(c) => Some(c),
                mc_registry::sprite_uv::Sample::Missing(c) => Some(c),
                mc_registry::sprite_uv::Sample::Unknown => None,
            }
        });
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
        // crosshair: centered 15x15 — ONLY in first person (the vanilla
        // renderCrosshair's own cameraType gate: a detached camera has no
        // crosshair)
        if self.perspective.is_first_person() {
            g.sprite(a, "hud/crosshair", w / 2 - 7, h / 2 - 7);
        }
        // hotbar: 182x22 centered at the bottom, selector around slot 0
        let (hx, hy) = hotbar_rect(w, h);
        g.sprite(a, "hud/hotbar", hx, hy);
        let sel_x = hx - 1 + self.selected_hotbar as i32 * (HEART_SPACING + 12);
        g.sprite(a, "hud/hotbar_selection", sel_x, hy - 1);
        // the LIVE store's row over the sprite (the vanilla
        // renderHotbarAndDecorations): each slot's icon at the vanilla
        // origin — 3px inset, the 20px pitch — and the count white and
        // shadowed at the slot's bottom-right, hidden while a single
        // item rides (the chest screen's own treatment). The offline
        // world's row is empty — the chrome-only shape the goldens pin.
        for (i, &(id, count)) in self.hotbar_slots.iter().enumerate() {
            if count <= 0 {
                continue; // the wire's own emptiness sentinel
            }
            let (ix, iy) = (hx + 3 + i as i32 * 20, hy + 3);
            draw_item_icon(g, id, ix, iy);
            if count > 1 {
                let s = count.to_string();
                let tw = g.text_width(&s);
                g.text(a, ix + 17 - tw, iy + 9, &s, WHITE, true);
            }
        }
        // THE HEALTH ROW (javap Hud.extractPlayerHealth + extractHearts
        // — the REAL law, the static 10-full render it replaces): the
        // Hud's own state machine (the blink arm, the display lerp)
        // runs per frame, then the RIGHT-TO-LEFT heart walk renders the
        // CONTAINER under every slot first, then the fill above the
        // health's own — the ABSORBING hearts (the white
        // `hud/heart/absorbing_*` sprites, the ceil(absorption/2)
        // slots the walk extends to) — then the highlight fill (the
        // _blinking sprites, the real health's own), then the normal
        // fill (the display health's own, the half boundary
        // `n + 1 == display`). The low-health jitter rides the
        // Java-Random law the `tickCount * 312871` seed speaks and its
        // gate counts the absorption (`health + absorption <= 4`); the
        // REGENERATION's lift (the `-2y` bounce on the
        // `tickCount % ceil(maxScale + 5)` heart) rides the effects
        // store's truth.
        let (hbx, hby) = hearts_origin(w, h);
        let health_i = self.health.ceil() as i32;
        let absorption_i = self.effects.absorption().ceil() as i32;
        // the blink flag (the Hud's own: the 20-tick window's /3 %2
        // rhythm)
        let blink = self.health_blink_time > self.frame
            && (self.health_blink_time - self.frame) / 3 % 2 == 1;
        // the lerp state (the Hud's own: the health moving while
        // invulnerable arms the flash; the 1000ms idle snaps the
        // display — folded to 20 ticks, the frame==tick model)
        if health_i < self.last_health && self.invulnerable_time > 0 {
            self.last_health_time = self.frame;
            self.health_blink_time = self.frame + 20;
        } else if health_i > self.last_health && self.invulnerable_time > 0 {
            self.last_health_time = self.frame;
            self.health_blink_time = self.frame + 20;
        }
        if self.frame.saturating_sub(self.last_health_time) > 20 {
            self.display_health = health_i;
            self.last_health_time = self.frame;
        }
        self.last_health = health_i;
        let display = self.display_health;
        let max_display = MAX_HEALTH.max(display as f32).max(health_i as f32);
        let health_hearts = (max_display / 2.0).ceil() as i32;
        let absorption_hearts = ((absorption_i as f32) / 2.0).ceil() as i32;
        // THE BLINK RANDOM (vanilla `random.setSeed(tickCount *
        // 312871)` + `nextInt(2)` per jittered heart — the Java
        // Random law verbatim: the 0x5DEECE66D scramble, the 48-bit
        // mask, the 31-bit draw, the power-of-two bound's own
        // arithmetic)
        let mut rng = ((self.frame as i64).wrapping_mul(312871) ^ 0x5DEECE66D)
            & ((1i64 << 48) - 1);
        let mut next_jitter = |rng: &mut i64| -> i32 {
            *rng = rng.wrapping_mul(0x5DEECE66D).wrapping_add(0xB) & ((1i64 << 48) - 1);
            let next31 = (*rng >> 17) as u32;
            ((2u64.wrapping_mul(next31 as u64)) >> 31) as i32
        };
        // the row height (the Hud's own: max(10 - (rows - 2), 3) — the
        // multi-row compression; the rows COUNT the absorption
        // (`ceil((maxScale + absorption) / 2 / 10)`), the law the
        // extractPlayerHealth call speaks)
        let rows = ((max_display + absorption_i as f32) / 2.0 / 10.0).ceil() as i32;
        let row_height = (10 - (rows - 2)).max(3);
        // the REGEN LIFT (the Hud's own: the heart index
        // `tickCount % ceil(maxScale + 5.0)` bounces -2y while the
        // REGENERATION effect rides — the store's truth, the
        // maxScale's own sum)
        let regen_lift = if self.effects.has_effect(crate::effects::MOB_EFFECT_REGENERATION) {
            (self.frame as i64 % (max_display as i64 + 5)) as i32
        } else {
            -1
        };
        for m in (0..health_hearts + absorption_hearts).rev() {
            let row = m / 10;
            let col = m % 10;
            let x = hbx + col * HEART_SPACING;
            let mut y = hby - row * row_height;
            // the LOW-HEALTH JITTER (the Hud's own: health + absorption
            // <= 4 shakes every heart — the absorption's own count)
            if health_i + absorption_i <= 4 {
                y += next_jitter(&mut rng);
            }
            // the REGEN BOUNCE (the extractHearts' own: only a health
            // slot, only the lifted index — `i17 < i14 && i17 == i6`)
            if m < health_hearts && m == regen_lift {
                y -= 2;
            }
            // the CONTAINER under every slot (blinking during the flash)
            g.sprite(
                a,
                if blink {
                    "hud/heart/container_blinking"
                } else {
                    "hud/heart/container"
                },
                x,
                y,
            );
            let n = m * 2;
            // THE ABSORBING FILL (the slot above the health's own — the
            // walk's `i17 >= i14` territory, the half-heart index
            // `n - healthHearts*2` against the absorption value, the
            // white `absorbing_*` sprites; the blink arg rides false —
            // the bytecode's own constants, the WITHERED swap the
            // honest absence the wither's lane never speaks here)
            if m >= health_hearts {
                let a_idx = n - health_hearts * 2;
                if a_idx < absorption_i {
                    g.sprite(
                        a,
                        if a_idx + 1 == absorption_i {
                            "hud/heart/absorbing_half"
                        } else {
                            "hud/heart/absorbing_full"
                        },
                        x,
                        y,
                    );
                }
            }
            // the HIGHLIGHT fill (the _blinking sprites over the LERPED
            // display — the white flash marks the lost hearts the
            // display still holds, half at `n + 1 == display`)
            if blink && n < display {
                g.sprite(
                    a,
                    if n + 1 == display {
                        "hud/heart/half_blinking"
                    } else {
                        "hud/heart/full_blinking"
                    },
                    x,
                    y,
                );
            }
            // the NORMAL fill (the REAL health's own — the red drop is
            // instant, half at `n + 1 == health`)
            if n < health_i {
                g.sprite(
                    a,
                    if n + 1 == health_i {
                        "hud/heart/half"
                    } else {
                        "hud/heart/full"
                    },
                    x,
                    y,
                );
            }
        }
        // THE HUNGER ROW (javap Hud.extractFood, the vanilla's own
        // law): the 10 drumsticks RIGHT-TO-LEFT from width/2 + 91 (the
        // -9 sprite offset the food_origin folds), the EMPTY under
        // every slot, the FULL at `i * 2 + 1 < food`, the HALF at
        // `i * 2 + 1 == food`, and the SATURATION-DRIVEN SHIMMY — the
        // saturation at zero shakes the row on the
        // `tickCount % (food * 3 + 1)` beat, each drumstick its own
        // `nextInt(3) - 1` draw (the Hud.random's stand-in: the
        // client's own Lcg, the deterministic draw the goldens speak)
        let (fx, fy) = food_origin(w, h);
        let food = self.food_data.food_level();
        let shake = self.food_data.saturation() <= 0.0
            && (self.frame as i64) % (food as i64 * 3 + 1) == 0;
        for i in 0..10 {
            let mut y = fy;
            if shake {
                y += self.food_rng.next_int_inclusive(-1, 1);
            }
            let x = fx - i * HEART_SPACING;
            g.sprite(a, "hud/food_empty", x, y);
            if i * 2 + 1 < food {
                g.sprite(a, "hud/food_full", x, y);
            } else if i * 2 + 1 == food {
                g.sprite(a, "hud/food_half", x, y);
            }
        }
        // THE EFFECT ICONS (javap Hud.extractEffects — the 26.2 row
        // law, verbatim): the icons render whenever the store carries
        // effects — the base Screen's `showsActiveEffects` is a flat
        // false and the client has no inventory screen of its own yet
        // (the chest's AbstractContainerScreen never overrides it: the
        // icons ride OVER the container GUI, the vanilla's own
        // treatment); the SORT is the reverse natural order
        // (the MobEffectInstance.compareTo's reversal); the beneficial
        // icons ride the top line at y=1 and the harmful ones the
        // second line 26px below, each row growing leftward from the
        // right edge at the 25px stride (`guiWidth - 25 * count`); the
        // 24x24 background (the _ambient variant for the ambient
        // effect), then the 18x18 icon at (+3, +3) — the
        // `getMobEffectSprite` path the GUI atlas's own directory
        // source stitches — under the expiring effect's blink alpha
        // (the `endsWithin(200)` cos fold the effects.rs law carries).
        if !self.effects.is_empty() {
            let beneficial_first = self.effects.sorted_for_hud();
            let (mut top_count, mut harm_count) = (0i32, 0i32);
            for (id, inst) in beneficial_first {
                if !inst.show_icon() {
                    continue;
                }
                let (beneficial, _) = mc_registry::generated::mob_effects::MOB_EFFECT_META
                    .get(id as usize)
                    .copied()
                    .unwrap_or((true, 0));
                let (count, y) = if beneficial {
                    top_count += 1;
                    (top_count, 1)
                } else {
                    harm_count += 1;
                    (harm_count, 27)
                };
                let x = w - 25 * count;
                g.sprite(
                    a,
                    if inst.is_ambient() {
                        "hud/effect_background_ambient"
                    } else {
                        "hud/effect_background"
                    },
                    x,
                    y,
                );
                let alpha = if inst.ends_within(200) {
                    crate::effects::icon_alpha(inst.duration)
                } else {
                    1.0
                };
                if let Some(icon) = crate::effects::effect_icon_sprite(id) {
                    g.sprite_at_tinted(a, icon, x + 3, y + 3, 18, 18, [1.0, 1.0, 1.0, alpha]);
                }
            }
        }
    }

    /// THE DEATH SCREEN (javap DeathScreen, the vanilla's own layout):
    /// the red gradient over the whole screen (`fillGradient(0, 0, w,
    /// h, 0x60500000, 0xA0703030)` — the dark-red top, the deep
    /// blood-red bottom, the constants the bytecode's ldc_w pair
    /// speaks), the "You Died!" title at scale 2 (the visitText law's
    /// y 30 in the scaled space), the cause of death line at y 85 when
    /// the wire named one, and the two buttons — Respawn at
    /// (w/2 - 100, h/4 + 72, 200x20), Title Screen at (w/2 - 100,
    /// h/4 + 96, 200x20) — the init() bounds verbatim.
    fn render_death(&mut self, g: &mut Gui, a: &mut GuiAssets, w: i32, h: i32) {
        // the gradient: fillGradient's linear top->bottom alpha walk
        // (the two ARGB constants decoded per row)
        let (top, bottom) = (0x60500000u32, 0xA0703030u32);
        let (tr_, tg, tb, ta) = (
            ((top >> 16) & 0xFF) as f32,
            ((top >> 8) & 0xFF) as f32,
            (top & 0xFF) as f32,
            ((top >> 24) & 0xFF) as f32,
        );
        let (br_, bg, bb, ba) = (
            ((bottom >> 16) & 0xFF) as f32,
            ((bottom >> 8) & 0xFF) as f32,
            (bottom & 0xFF) as f32,
            ((bottom >> 24) & 0xFF) as f32,
        );
        let hh = (h - 1).max(1) as f32;
        for y in 0..h {
            let t = y as f32 / hh;
            let c = [
                (tr_ + (br_ - tr_) * t) / 255.0,
                (tg + (bg - tg) * t) / 255.0,
                (tb + (bb - tb) * t) / 255.0,
                (ta + (ba - ta) * t) / 255.0,
            ];
            g.fill(0, y, w, 1, c);
        }
        // the title: scale 2, the visitText law's CENTER accept at
        // (width/4, 30) in the scaled space — the screen center, the
        // y 60 unscaled, the glyphs doubled
        let title = tr("deathScreen.title").unwrap_or("You Died!");
        let tw = g.text_width(title) * 2;
        g.text_scaled(
            a,
            w / 2 - tw / 2,
            60,
            title,
            WHITE,
            true,
            2,
        );
        // the cause of death (the Combat Kill wire's message — the
        // local fall's own lane rides the client's title only)
        if let Some(msg) = self.death_message.as_deref() {
            if !msg.is_empty() {
                g.text_centered(a, w / 2, 85, msg, WHITE, true);
            }
        }
        // the buttons: the init() bounds verbatim (h/4 + 72 and
        // h/4 + 96, both 200x20 centered)
        self.buttons.push(Button::new(
            "respawn",
            tr("deathScreen.respawn").unwrap_or("Respawn").to_string(),
            w / 2,
            h / 4 + 72,
            200,
        ));
        self.buttons.push(Button::new(
            "deathtitle",
            tr("deathScreen.titleScreen")
                .unwrap_or("Title screen")
                .to_string(),
            w / 2,
            h / 4 + 96,
            200,
        ));
        for b in &self.buttons.clone() {
            b.render(g, a, -1, -1);
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

/// The chest panel's GUI-space origin — the vanilla generic_54 canvas
/// (176x166) centered in the GUI workspace.
fn chest_panel_origin(gui_w: i32, gui_h: i32) -> (i32, i32) {
    ((gui_w - 176) / 2, (gui_h - 166) / 2)
}

/// The 9x3 container grid's slot origin (the sprite's top-left): the
/// generic_54 layout starts the container rows at panel-relative
/// (8, 18), the slot pitch is 18 (the row-major indexing the wire's
/// menu numbering mirrors for the generic_9x3 menu).
fn chest_container_slot(panel: (i32, i32), index: usize) -> (i32, i32) {
    (
        panel.0 + 8 + (index % 9) as i32 * 18,
        panel.1 + 18 + (index / 9) as i32 * 18,
    )
}

/// The player-inventory mirror's slot origin: the three rows at
/// panel-relative y 84, 102, 120 and the hotbar row at 142 — the
/// generic_54 canvas's own numbers, the gap between the grids kept.
fn chest_mirror_slot(panel: (i32, i32), index: usize) -> (i32, i32) {
    let y = if index < 27 {
        84 + (index / 9) as i32 * 18
    } else {
        142
    };
    (panel.0 + 8 + (index % 9) as i32 * 18, panel.1 + y)
}

/// One flat item icon, 16x16 GUI pixels, through the generated texture
/// table (`mc_registry::item_icon` — the `item/<name>` GUI layer).
/// Unresolvable ids and the pinned missing materials draw nothing —
/// the honest unknown. The texels land one alpha-blended GUI pixel at
/// a time (the public fill primitive at unit size).
/// The per-quad direction -> the FACE_SHADE index (the mesher's own
/// order: [east, west, top, bottom, south, north]).
fn shade_for_direction(q: &mc_registry::element_quads::Quad) -> f32 {
    use mc_registry::generated::elements::Cull;
    let idx = match q.direction {
        Cull::East => 0,
        Cull::West => 1,
        Cull::Up => 2,
        Cull::Down => 3,
        Cull::South => 4,
        Cull::North => 5,
    };
    crate::colors::FACE_SHADE[idx]
}

/// The eat piece's sprite probe — the item's FIRST model layer's
/// first quad: the texture name the sampler resolves and the quad's
/// uv span (the 4x4-pixel slice rides inside it). The unresolvable
/// item (no layers, no quads) is None — the honest unknown renders
/// nothing.
fn piece_sprite(item: i32) -> Option<(&'static str, [f32; 4])> {
    let layers = mc_registry::item_model_layers(item)?;
    let layer = layers.first()?;
    let quads = mc_registry::display::posed_item_layer_quads(
        layer.model,
        layer.transform,
        mc_registry::display::GROUND,
    )?
    .ok()?;
    let q = quads.first()?;
    let (mut u_lo, mut u_hi, mut v_lo, mut v_hi) =
        (f32::INFINITY, f32::NEG_INFINITY, f32::INFINITY, f32::NEG_INFINITY);
    for uv in q.uvs {
        u_lo = u_lo.min(uv[0]);
        u_hi = u_hi.max(uv[0]);
        v_lo = v_lo.min(uv[1]);
        v_hi = v_hi.max(uv[1]);
    }
    Some((q.texture, [u_lo, u_hi, v_lo, v_hi]))
}

/// The ItemColors law for the GUI icons: the first tint json of the
/// layer, the constant value verbatim (0xRRGGBB int or [r,g,b] floats),
/// the grass/foliage tints at the vanilla's own no-level climate
/// (0.5, 1.0) through the audited colormap. Untinted layers = white.
fn icon_tint(tints: &[&str]) -> [f32; 3] {
    for t in tints {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(t) else {
            continue;
        };
        match v.get("type").and_then(|t| t.as_str()) {
            Some("minecraft:constant") => {
                let rgb = match v.get("value") {
                    Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(0xFFFFFF) as u32,
                    Some(serde_json::Value::Array(a)) => {
                        let f = |i: usize| {
                            a.get(i).and_then(|x| x.as_f64()).unwrap_or(1.0) as f32
                        };
                        let (r, g, b) = (f(0), f(1), f(2));
                        ((r * 255.0).round() as u32) << 16
                            | ((g * 255.0).round() as u32) << 8
                            | (b * 255.0).round() as u32
                    }
                    _ => 0xFFFFFF,
                };
                return crate::tint::unpack(rgb & 0xFFFFFF);
            }
            Some("minecraft:grass") => {
                let c = crate::colormap::grass_from_climate(0.5, 1.0).unwrap_or(0x8EB971);
                return crate::tint::unpack(c);
            }
            Some("minecraft:foliage") => {
                let c = crate::colormap::foliage_from_climate(0.5, 1.0).unwrap_or(0x48B518);
                return crate::tint::unpack(c);
            }
            _ => {}
        }
    }
    [1.0, 1.0, 1.0]
}

/// The 3D icon renderer — the GuiItemAtlas drawToSlot law over ONE slot:
/// the pose = translate(slot center px, 0) . scale(s, -s, s) with
/// s = 16 GUI px (the slot's own pixel size; the atlas slots render at
/// the gui resolution and ours rasterizes straight into the framebuffer,
/// single-pass — no atlas, no reuse), the ortho depth carried by the
/// projection's w row (nearer = smaller, the raster's own convention),
/// the per-slot depth clear (the atlas clears each slot's depth too).
/// The quads ride the registry's gui chain with the per-quad tint x
/// shade folded into the SampledQuad multiplier. Returns false when the
/// item has no gui quads (the caller falls back to the flat blit).
fn draw_item_icon_3d(g: &mut Gui, item_id: i32, x: i32, y: i32) -> bool {
    let Some(Ok(layers)) = mc_registry::display::item_gui_quads(item_id) else {
        return false;
    };
    if layers.is_empty() {
        return false;
    }
    let scale = g.scale as f32;
    let (w_fb, h_fb) = (g.fb.w as f32, g.fb.h as f32);
    let cx = (x as f32 + 8.0) * scale;
    let cy = (y as f32 + 8.0) * scale;
    let s = 16.0 * scale;
    // translate(slot center) . scale(s, -s, s) as the NDC projection:
    // x_ndc = 2(x_px)/w - 1 with x_px = cx + bx*s; y_px = cy - by*s (the
    // atlas's own y-flip) -> y_ndc = 1 - 2 y_px/h; the w row carries the
    // depth (nearer = smaller): w = 1.5 - bz
    let mut proj = crate::math::Mat4::IDENT;
    proj.0[0] = 2.0 * s / w_fb;
    proj.0[5] = 2.0 * s / h_fb;
    proj.0[11] = -1.0;
    proj.0[12] = 2.0 * cx / w_fb - 1.0;
    proj.0[13] = 1.0 - 2.0 * cy / h_fb;
    proj.0[15] = 1.5;
    g.fb.clear_depth();
    let sampled: Vec<crate::raster::SampledQuad> = layers
        .iter()
        .map(|(q, shaded, tints)| {
            let tint = if *shaded {
                let sh = shade_for_direction(q);
                let t = icon_tint(tints);
                [t[0] * sh, t[1] * sh, t[2] * sh]
            } else {
                icon_tint(tints)
            };
            crate::raster::SampledQuad {
                pts: q.vertices,
                uv: q.uvs,
                tex: q.texture,
                tint,
            }
        })
        .collect();
    g.fb.draw_sampled_quads(proj, &sampled, &mut |tex, u, v| {
        match mc_registry::sprite_uv::sample_uv(tex, u, v) {
            mc_registry::sprite_uv::Sample::Pixel(c) => Some(c),
            mc_registry::sprite_uv::Sample::Missing(c) => Some(c),
            mc_registry::sprite_uv::Sample::Unknown => None,
        }
    });
    true
}

fn draw_item_icon(g: &mut Gui, item_id: i32, x: i32, y: i32) {
    // THE 3D ICON PATH: every item's GUI icon is the vanilla atlas arc's
    // own render — the gui-context quads (the block models' isometric
    // geometry, the flat sprites' centered extrusion, the 63 specials'
    // dump polygons) projected into the 16x16 slot by the GuiItemAtlas
    // drawToSlot law (the slot-center translate, the scale(s, -s, s)
    // y-flip, the ortho depth). The flat blit below stays as the
    // defensive fallback for ids the quad path refuses.
    if draw_item_icon_3d(g, item_id, x, y) {
        return;
    }
    let Some((rgba, w, h)) = mc_registry::item_icon(item_id) else {
        return;
    };
    for yy in 0..h as i32 {
        for xx in 0..w as i32 {
            let o = ((yy * w as i32 + xx) * 4) as usize;
            let alpha = rgba[o + 3] as f32 / 255.0;
            if alpha <= 0.0 {
                continue;
            }
            g.fill(
                x + xx,
                y + yy,
                1,
                1,
                [
                    rgba[o] as f32 / 255.0,
                    rgba[o + 1] as f32 / 255.0,
                    rgba[o + 2] as f32 / 255.0,
                    alpha,
                ],
            );
        }
    }
}

/// Health row origin: the vanilla layout law (javap
/// Hud.extractPlayerHealth).
fn hearts_origin(gui_w: i32, gui_h: i32) -> (i32, i32) {
    // THE VANILLA LAYOUT (javap Hud.extractPlayerHealth): the row's
    // left edge at width/2 - 91, the row's top at height - 39 — the
    // y leaves the experience bar's own slot between the hearts and
    // the hotbar, exactly like the original's HUD.
    (gui_w / 2 - 91, gui_h - 39)
}

/// Hunger row origin: first (rightmost) drumstick, aligned with the hearts,
/// right edge flush with the hotbar's right edge.
fn food_origin(gui_w: i32, gui_h: i32) -> (i32, i32) {
    // the same vanilla line the hearts speak (the food row shares the
    // hearts' y), the right edge at width/2 + 91 (the row's own edge)
    (gui_w / 2 + 91 - HEART, gui_h - 39)
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
    fn chest_geometry_carries_the_generic_54_layout() {
        // the 176x166 canvas centers in the 320x180 GUI workspace
        assert_eq!(chest_panel_origin(320, 180), (72, 7));
        // the 9x3 container grid: row-major from (8, 18), the 18 pitch,
        // the row starts at the panel-relative x 8 and y 18 / 36 / 54
        let p = (0, 0);
        assert_eq!(chest_container_slot(p, 0), (8, 18));
        assert_eq!(chest_container_slot(p, 8), (8 + 8 * 18, 18));
        assert_eq!(chest_container_slot(p, 9), (8, 36));
        assert_eq!(chest_container_slot(p, 26), (8 + 8 * 18, 54));
        // the player mirror: three rows at 84 / 102 / 120, the hotbar
        // row at 142 — the gap between the grids kept (the vanilla
        // canvas's own numbers)
        assert_eq!(chest_mirror_slot(p, 0), (8, 84));
        assert_eq!(chest_mirror_slot(p, 8), (8 + 8 * 18, 84));
        assert_eq!(chest_mirror_slot(p, 26), (8 + 8 * 18, 120));
        assert_eq!(chest_mirror_slot(p, 27), (8, 142));
        assert_eq!(chest_mirror_slot(p, 35), (8 + 8 * 18, 142));
        // the panel bottom keeps the mirror's last row inside: 142 + 18
        // slot height + the bevel lands exactly on the 166 canvas
        assert_eq!(chest_mirror_slot(p, 27).1 + 18 + 6, 166);
    }

    #[test]
    fn the_chest_loadout_is_fixed_and_uses_registry_ids() {
        let mut app = App::new(20260214);
        app.open_chest();
        // the screen rode open; the world underneath came with it
        assert_eq!(app.screen, ScreenKind::Chest);
        assert_eq!(app.chest_slots.len(), 27);
        // the loadout is a CONSTANT (no RNG): two opens, one loadout
        let again = {
            let mut a2 = App::new(7); // a DIFFERENT seed — the loadout holds
            a2.open_chest();
            a2.chest_slots.clone()
        };
        assert_eq!(app.chest_slots, again);
        // the pins: the ids resolve through the registry's own order
        // and the counts ride the wire's shape (positive, the rest
        // empty via the count<=0 sentinel)
        assert_eq!(app.chest_slots[4], (926, 12)); // diamond x12
        assert_eq!(app.chest_slots[13], (932, 64)); // iron_ingot x64
        assert_eq!(mc_registry::item_name(926), Some("diamond"));
        assert_eq!(mc_registry::item_name(932), Some("iron_ingot"));
        let filled = app.chest_slots.iter().filter(|(_, c)| *c > 0).count();
        assert_eq!(filled, 8);
    }

    #[test]
    fn the_hud_row_lands_verbatim_from_the_store() {
        // the offline world carries no store: the row is the all-empty
        // default, the hotbar renders its chrome only (the honest shape
        // the goldens pin)
        let mut app = App::new(3);
        assert_eq!(app.hotbar_slots, [(0, 0); 9]);
        // the live row lands WHOLESALE: every slot verbatim, the (0, 0)
        // slots the wire's own emptiness riding untouched — the HUD
        // skips them at draw time (the count<=0 sentinel)
        let row = [
            (0, 0),
            (921, 5),   // apple x5
            (0, 0),
            (926, 1),   // diamond x1 — the count's own hiding rule
            (62, 27),   // cobblestone x27
            (0, 0),
            (0, 0),
            (932, 64),  // iron_ingot x64
            (0, 0),
        ];
        app.live_hotbar_updated(row);
        assert_eq!(app.hotbar_slots, row);
        // the replacement is wholesale: a fresh row replaces the old
        // belief entirely (the store's refresh re-speaks every slot —
        // a stale icon never outlives its store)
        app.live_hotbar_updated([(0, 0); 9]);
        assert_eq!(app.hotbar_slots, [(0, 0); 9]);
        // the ids resolve through the registry's own order (the same
        // table the flat icon textures walk)
        assert_eq!(mc_registry::item_name(921), Some("apple"));
        assert_eq!(mc_registry::item_name(926), Some("diamond"));
    }

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
        // hearts above left half at height - 39 (the Hud's own law, the
        // XP bar's slot between the hearts and the hotbar), hunger on
        // the same line, right edge flush with the hotbar's
        let (w, h) = (320, 180);
        assert_eq!(hotbar_rect(w, h), (320 / 2 - 91, 180 - 23));
        assert_eq!(hearts_origin(w, h), (320 / 2 - 91, 180 - 39));
        assert_eq!(food_origin(w, h), (320 / 2 + 91 - HEART, 180 - 39));
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
            block_light_tint: None, // no dimension opinion — the bytecode default tint
            sky_light_factor: None, // no dimension opinion — the bytecode default 1.0
            sky_light_color: None, // no dimension opinion — the -1 default's white identity
            night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
            night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
                },
                camera: None,
                spawn_biome: Some("minecraft:plains".into()),
                slim_arm: false,
                skybox: Skybox::Overworld,
                fog: None,
                cloud_fog_end: None,
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
                let cloud_fog_end = view.model.cloud_fog_end_distance();
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
                    cloud_fog_end,
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
            block_light_tint: None, // no dimension opinion — the bytecode default tint
            sky_light_factor: None, // no dimension opinion — the bytecode default 1.0
            sky_light_color: None, // no dimension opinion — the -1 default's white identity
            night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
            night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
        });
        app.enter_network_world(None, Skybox::Overworld, None, None);
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
            block_light_tint: None, // no dimension opinion — the bytecode default tint
            sky_light_factor: None, // no dimension opinion — the bytecode default 1.0
            sky_light_color: None, // no dimension opinion — the -1 default's white identity
            night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
            night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
        });
        app.enter_network_world(None, Skybox::Overworld, None, None);
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
            block_light_tint: None, // no dimension opinion — the bytecode default tint
            sky_light_factor: None, // no dimension opinion — the bytecode default 1.0
            sky_light_color: None, // no dimension opinion — the -1 default's white identity
            night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
            night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
                },
                camera: None,
                spawn_biome: None,
                slim_arm: false,
                skybox: Skybox::Overworld,
                fog: None,
                cloud_fog_end: None,
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
            block_light_tint: None, // no dimension opinion — the bytecode default tint
            sky_light_factor: None, // no dimension opinion — the bytecode default 1.0
            sky_light_color: None, // no dimension opinion — the -1 default's white identity
            night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
            night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
        }
    }

    #[test]
    fn live_view_updates_swap_the_network_world_in_place() {
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        // the join landed (an all-air window: the state machine must not care)
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
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
        app.live_view_updated(streamed, Some(pose), Some("minecraft:swamp".into()), None);
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
        app.live_view_updated(air_net_world(2, 8), Some(pose), Some("minecraft:swamp".into()), None);
        assert_eq!(app.net_biome.as_deref(), Some("minecraft:swamp"));
        assert_eq!(app.music.events.len(), events_before, "no situation restart");
        // an UNRESOLVABLE biome (None) keeps what is playing — no
        // knowledge is not a change
        app.live_view_updated(air_net_world(2, 9), Some(pose), None, None);
        assert_eq!(app.net_biome.as_deref(), Some("minecraft:swamp"));
        assert_eq!(app.music.events.len(), events_before);
        // and a real change re-fires once
        app.live_view_updated(air_net_world(2, 10), Some(pose), Some("minecraft:desert".into()), None);
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
        app.enter_network_world(Some("minecraft:swamp".into()), Skybox::Overworld, None, None);
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
            None,
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
    fn the_live_container_drives_the_chest_screen() {
        use crate::world_view::LiveContainer;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        assert_eq!(app.screen, ScreenKind::World);

        // the server opens a chest (Open Screen → the store, the Set
        // Content fills it — here the store's PROJECTION arrives): the
        // screen SHOWS over the world, the slots ride the menu's own
        // numbering into the grid and the mirror
        let chest = LiveContainer {
            id: 5,
            menu_type_id: crate::world_model::MENU_TYPE_CHEST,
            slots: {
                let mut s = vec![(0i32, 0i32); 63];
                s[4] = (926, 12); // the container's slot 4: diamond x12
                s[27] = (921, 5); // the mirror's first main row slot: apple x5
                s[54] = (932, 64); // the mirror's hotbar head: iron_ingot x64
                s
            },
            carried: (0, 0),
            state_id: 7,
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(chest.clone()));
        assert_eq!(app.screen, ScreenKind::Chest, "the window shows over the world");
        assert!(app.chest_is_live, "the live lane owns the screen now");
        assert_eq!(app.chest_slots.len(), 27);
        assert_eq!(app.chest_slots[4], (926, 12), "the container grid rode in");
        assert_eq!(app.chest_mirror.len(), 36);
        assert_eq!(app.chest_mirror[0], (921, 5), "the mirror's main row slot 0 = menu slot 27");
        assert_eq!(app.chest_mirror[27], (932, 64), "the mirror's hotbar head = menu slot 54");

        // a NEWER snapshot of the same window updates IN PLACE (the
        // Set Content packet's replacement — no re-open flicker)
        let mut chest2 = chest.clone();
        chest2.slots[4] = (0, 0); // the diamond was taken out
        chest2.slots[9] = (981, 3);
        app.live_view_updated(air_net_world(2, 2), None, None, Some(chest2));
        assert_eq!(app.screen, ScreenKind::Chest, "the update is in place");
        assert_eq!(app.chest_slots[4], (0, 0), "the taken stack emptied");
        assert_eq!(app.chest_slots[9], (981, 3), "the new stack landed");

        // a NON-chest menu is the honest drop: the store keeps it, the
        // screen machinery does not move (its screen is its own round)
        let hopper = LiveContainer { id: 6, menu_type_id: 15, slots: vec![], carried: (0, 0), state_id: 0 };
        app.live_view_updated(air_net_world(2, 3), None, None, Some(hopper));
        assert_eq!(app.screen, ScreenKind::Chest, "the wired screen stays");
        assert_eq!(app.chest_slots[9], (981, 3), "the hopper touched nothing");

        // the store DROPPED (the server's Container Close): the live
        // chest returns to the world — the vanilla closeContainer's own
        app.live_view_updated(air_net_world(2, 4), None, None, None);
        assert_eq!(app.screen, ScreenKind::World, "the close returns to the world");
        assert!(!app.chest_is_live, "the live flag dropped with the window");

        // a container-less update with NO chest up: nothing happens
        app.live_view_updated(air_net_world(2, 5), None, None, None);
        assert_eq!(app.screen, ScreenKind::World);

        // the OFFLINE stand-in is never the live lane's to close: the
        // deterministic chest stays up (the app went PROCEDURAL with
        // the offline scene — the world guard drops the live updates)
        app.open_chest();
        assert_eq!(app.screen, ScreenKind::Chest);
        assert!(!app.chest_is_live);
        assert!(app.chest_mirror.is_empty(), "the offline mirror is chrome-only");
        app.live_view_updated(air_net_world(2, 6), None, None, None);
        assert_eq!(app.screen, ScreenKind::Chest, "the offline chest survives");
        // and the lane re-arms on a NEW join: the network world back,
        // the live window opens over it (the fresh menu's slots — the
        // vanilla open is a fresh screen)
        app.net_world = Some(air_net_world(2, 8));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        assert_eq!(app.screen, ScreenKind::World);
        app.live_view_updated(air_net_world(2, 9), None, None, Some(chest));
        assert_eq!(app.screen, ScreenKind::Chest);
        assert!(app.chest_is_live, "the live lane took the screen over");
        assert_eq!(app.chest_slots[4], (926, 12), "the live slots replaced the loadout");
        assert_eq!(app.chest_mirror[0], (921, 5), "the live mirror filled in");
    }

    /// The slot click's whole arc: the live chest's press rides the
    /// send lane with the window's own identity (id + revision), the
    /// between-slots press is the honest no-op, and the offline
    /// stand-in never speaks (no window, no session).
    #[test]
    fn the_slot_click_rides_the_send_lane() {
        use crate::world_view::{ContainerCommand, ContainerInput};
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let chest = crate::world_view::LiveContainer {
            id: 5,
            menu_type_id: crate::world_model::MENU_TYPE_CHEST,
            slots: {
                let mut s = vec![(0i32, 0i32); 63];
                s[4] = (926, 12); // the diamond x12 in the container's slot 4
                s
            },
            carried: (0, 0),
            state_id: 7,
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
        assert_eq!(app.screen, ScreenKind::Chest);
        assert!(app.chest_is_live);
        assert_eq!(app.chest_live_window, Some((5, 7)), "the window's identity rode in");

        // the render lays out the hit boxes (the buttons' own pattern);
        // the press at the box's center clicks the MENU's slot number
        app.render_title_buttons_for_test();
        assert_eq!(app.chest_slot_rects.len(), 63, "27 grid + 36 mirror boxes");
        let &(sx, sy, slot) = app
            .chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 4)
            .expect("the container's slot 4 box");
        app.on_press(sx + 8, sy + 8);
        // THE PREDICTION: the pickup ran locally — the slot emptied and
        // the believed cursor holds the stack
        assert_eq!(app.chest_slots[4], (0, 0), "the slot's belief emptied");
        assert_eq!(app.chest_carried, (926, 12), "the cursor's belief holds the stack");
        match click_rx.try_recv().expect("the click rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 5, "the live window's id");
                assert_eq!(c.state_id, 7, "the revision the client last saw");
                assert_eq!(c.slot, 4, "the MENU's own slot number");
                assert_eq!(c.button, 0, "the left press");
                assert_eq!(c.input, ContainerInput::Pickup);
                assert_eq!(
                    c.changed,
                    vec![(4, (0, 0))],
                    "the post-click delta rides the command"
                );
                assert_eq!(c.carried, (926, 12), "the believed cursor rides");
            }
            other => panic!("expected a click, got {other:?}"),
        }

        // the MIRROR's box speaks the menu's tail numbering (the main
        // row's first slot is the menu's 27) — the held stack PLACES
        // into the empty mirror slot (the pickup table's own arc)
        let &(mx, my, _) = app
            .chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 27)
            .expect("the mirror's first box");
        app.on_press(mx + 8, my + 8);
        assert_eq!(app.chest_mirror[0], (926, 12), "the held stack placed");
        assert_eq!(app.chest_carried, (0, 0), "the cursor emptied");
        match click_rx.try_recv().expect("the mirror click rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 27, "the menu's tail numbering");
                assert_eq!(c.changed, vec![(27, (926, 12))]);
                assert_eq!(c.carried, (0, 0));
            }
            other => panic!("expected a click, got {other:?}"),
        }

        // a press BETWEEN the boxes dies on the frame thread (no click
        // rides the lane)
        app.on_press(0, 0);
        assert!(click_rx.try_recv().is_err(), "no slot, no click");

        // the OFFLINE stand-in: the same screen, no window, no session
        // — its presses are the honest no-op (the golden's scene)
        app.live_view_updated(air_net_world(2, 2), None, None, None);
        assert_eq!(app.screen, ScreenKind::World, "the live chest closed");
        app.open_chest();
        assert_eq!(app.screen, ScreenKind::Chest);
        assert!(!app.chest_is_live, "the offline flag stays down");
        assert_eq!(app.chest_live_window, None, "no window identity");
        app.render_title_buttons_for_test();
        let &(ox, oy, _) = app
            .chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 4)
            .expect("the offline grid's own slot 4 box");
        app.on_press(ox + 8, oy + 8);
        assert!(click_rx.try_recv().is_err(), "the offline chest never speaks");
    }

    /// The player's OWN close echoes the serverbound Container Close
    /// packet through the send lane (the live chest only — the offline
    /// stand-in's close stays a local act).
    #[test]
    fn escape_echoes_the_close_packet() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let chest = crate::world_view::LiveContainer {
            id: 2,
            menu_type_id: crate::world_model::MENU_TYPE_CHEST,
            slots: vec![(926, 1)],
            carried: (0, 0),
            state_id: 3,
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
        assert_eq!(app.screen, ScreenKind::Chest);
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::World, "the container closed");
        assert!(!app.chest_is_live);
        assert_eq!(app.chest_live_window, None, "the window's identity dropped");
        match click_rx.try_recv().expect("the close echo rode the lane") {
            ContainerCommand::Close { window_id } => assert_eq!(window_id, 2),
            other => panic!("expected a close, got {other:?}"),
        }

        // the OFFLINE chest's Escape: the same close, no echo (no
        // window to close on the wire)
        app.open_chest();
        assert_eq!(app.screen, ScreenKind::Chest);
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::World);
        assert!(click_rx.try_recv().is_err(), "the offline close never speaks");
    }

    /// The prediction's own arcs over the believed state: the same-item
    /// MERGE (the remainder rides), the different-item SWAP, the
    /// authoritative correction's wholesale replacement (the server's
    /// projection overrules every local belief), and the Escape reset.
    #[test]
    fn the_prediction_believes_and_the_correction_overrules() {
        use crate::world_view::{ContainerCommand, ContainerInput};
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let chest = crate::world_view::LiveContainer {
            id: 9,
            menu_type_id: crate::world_model::MENU_TYPE_CHEST,
            slots: {
                let mut s = vec![(0i32, 0i32); 63];
                s[8] = (926, 60); // diamond x60 in the grid
                s[9] = (926, 10); // diamond x10 beside it
                s[10] = (921, 3); // apple x3
                s
            },
            carried: (0, 0),
            state_id: 11,
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
        assert_eq!(app.screen, ScreenKind::Chest);
        app.render_title_buttons_for_test();
        let box_of = |app: &App, want: i16| {
            app.chest_slot_rects
                .iter()
                .find(|&&(_, _, s)| s == want)
                .copied()
                .expect("the box")
        };

        // MERGE: pick up the x60, drop it on the x10 — the slot takes
        // what fits (to the ceiling), the remainder rides the cursor
        let (bx, by, _) = box_of(&app, 8);
        app.on_press(bx + 8, by + 8);
        assert_eq!(app.chest_carried, (926, 60));
        let (bx, by, _) = box_of(&app, 9);
        app.on_press(bx + 8, by + 8);
        assert_eq!(app.chest_slots[9], (926, 64), "the merge filled the slot");
        assert_eq!(app.chest_carried, (926, 6), "the remainder rides");
        let _ = click_rx.try_recv(); // the first click
        match click_rx.try_recv().expect("the merge's click") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.changed, vec![(9, (926, 64))]);
                assert_eq!(c.carried, (926, 6));
            }
            other => panic!("expected a click, got {other:?}"),
        }

        // SWAP: the held diamonds over the apple — the stacks trade
        let (bx, by, _) = box_of(&app, 10);
        app.on_press(bx + 8, by + 8);
        assert_eq!(app.chest_slots[10], (926, 6), "the swap landed");
        assert_eq!(app.chest_carried, (921, 3), "the apple rides");
        match click_rx.try_recv().expect("the swap's click") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.changed, vec![(10, (926, 6))]);
                assert_eq!(c.carried, (921, 3));
            }
            other => panic!("expected a click, got {other:?}"),
        }

        // THE CORRECTION: the server's authoritative projection replaces
        // the whole belief — slots, cursor, window identity alike
        let truth = crate::world_view::LiveContainer {
            id: 9,
            menu_type_id: crate::world_model::MENU_TYPE_CHEST,
            slots: {
                let mut s = vec![(0i32, 0i32); 63];
                s[8] = (926, 60); // the server never processed the clicks
                s
            },
            carried: (0, 0),
            state_id: 12,
        };
        app.live_view_updated(air_net_world(2, 2), None, None, Some(truth));
        assert_eq!(app.chest_slots[9], (0, 0), "the prediction's merge is gone");
        assert_eq!(app.chest_slots[10], (0, 0), "the prediction's swap is gone");
        assert_eq!(app.chest_carried, (0, 0), "the server's cursor is the truth");
        assert_eq!(app.chest_live_window, Some((9, 12)), "the revision advanced");
        // the next click echoes the NEW revision
        let (bx, by, _) = box_of(&app, 8);
        app.on_press(bx + 8, by + 8);
        match click_rx.try_recv().expect("the post-correction click") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.state_id, 12, "the correction's revision rides");
                assert_eq!(c.input, ContainerInput::Pickup);
            }
            other => panic!("expected a click, got {other:?}"),
        }

        // ESCAPE resets the belief with the window (the held stack
        // returns to the inventory server-side)
        assert_eq!(app.chest_carried, (926, 60), "the held stack rides out");
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::World);
        assert_eq!(app.chest_carried, (0, 0), "no window, no held stack");
        let _ = click_rx.try_recv(); // the close echo
    }

    /// The RIGHT press's own lane: the split-stack pickup (the ceiling
    /// half), the place-one, and the button byte riding the command —
    /// the same PICKUP input mode, the wire's other button value. The
    /// offline stand-in stays silent under the right press too.
    #[test]
    fn the_right_press_splits_and_places_one() {
        use crate::world_view::{ContainerCommand, ContainerInput};
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let chest = crate::world_view::LiveContainer {
            id: 4,
            menu_type_id: crate::world_model::MENU_TYPE_CHEST,
            slots: {
                let mut s = vec![(0i32, 0i32); 63];
                s[6] = (926, 5); // diamond x5 — the odd split's own number
                s
            },
            carried: (0, 0),
            state_id: 8,
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
        assert_eq!(app.screen, ScreenKind::Chest);
        app.render_title_buttons_for_test();
        let (bx, by, _) = app
            .chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 6)
            .copied()
            .expect("the box");

        // the right press takes the CEILING half (5 -> 3 taken, 2 left)
        app.on_right_press(bx + 8, by + 8);
        assert_eq!(app.chest_slots[6], (926, 2), "the odd split left two");
        assert_eq!(app.chest_carried, (926, 3), "the ceiling half rides");
        match click_rx.try_recv().expect("the right press's click") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.button, 1, "the right press's own button byte");
                assert_eq!(c.input, ContainerInput::Pickup, "the same PICKUP mode");
                assert_eq!(c.changed, vec![(6, (926, 2))]);
                assert_eq!(c.carried, (926, 3));
            }
            other => panic!("expected a click, got {other:?}"),
        }

        // the held half places ONE back per right press
        let (ex, ey, _) = app
            .chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 7)
            .copied()
            .expect("the empty box");
        app.on_right_press(ex + 8, ey + 8);
        assert_eq!(app.chest_slots[7], (926, 1), "one placed");
        assert_eq!(app.chest_carried, (926, 2), "the cursor paid one");
        match click_rx.try_recv().expect("the place-one's click") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.button, 1);
                assert_eq!(c.changed, vec![(7, (926, 1))]);
                assert_eq!(c.carried, (926, 2));
            }
            other => panic!("expected a click, got {other:?}"),
        }

        // the offline stand-in: the right press never speaks
        app.on_escape();
        let _ = click_rx.try_recv(); // the close echo
        app.open_chest();
        app.render_title_buttons_for_test();
        let (ox, oy, _) = app
            .chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 6)
            .copied()
            .expect("the offline box");
        app.on_right_press(ox + 8, oy + 8);
        assert!(click_rx.try_recv().is_err(), "the offline right press is silent");
    }

    /// The shift-click's whole arc: the keyboard lane's shift state
    /// routes the press through the QUICK_MOVE table — the whole stack
    /// moves to the OTHER section (the merge, the remainder's hotbar
    /// fill), the multi-slot belief rides the command, the cursor is
    /// NOT the lane, and the offline stand-in stays silent.
    #[test]
    fn the_shift_click_moves_the_whole_stack() {
        use crate::world_view::{ContainerCommand, ContainerInput};
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        let chest = crate::world_view::LiveContainer {
            id: 5,
            menu_type_id: crate::world_model::MENU_TYPE_CHEST,
            slots: {
                let mut s = vec![(0i32, 0i32); 63];
                s[4] = (926, 30); // the chest grid's diamond stack
                s[50] = (926, 40); // the mirror's same-item merge target
                s
            },
            carried: (921, 3), // a held stack: QUICK_MOVE never touches it
            state_id: 7,
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
        assert_eq!(app.screen, ScreenKind::Chest);
        app.render_title_buttons_for_test();
        let &(sx, sy, slot) = app
            .chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 4)
            .expect("the container's slot 4 box");

        // the SHIFT+left press: the merge fills the mirror's stack to
        // the ceiling, the remainder rides the hotbar's last slot
        app.on_shift_down(true);
        app.on_press(sx + 8, sy + 8);
        assert_eq!(app.chest_slots[4], (0, 0), "the source emptied");
        assert_eq!(app.chest_mirror[23], (926, 64), "the merge filled (menu 50)");
        assert_eq!(app.chest_mirror[35], (926, 6), "the remainder rides (menu 62)");
        assert_eq!(app.chest_carried, (921, 3), "the cursor is not the QUICK_MOVE's lane");
        match click_rx.try_recv().expect("the shift click rode the lane") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.window_id, 5);
                assert_eq!(c.state_id, 7);
                assert_eq!(c.slot, 4);
                assert_eq!(c.button, 0, "the left press's own button byte");
                assert_eq!(c.input, ContainerInput::QuickMove);
                assert_eq!(
                    c.changed,
                    vec![(4, (0, 0)), (50, (926, 64)), (62, (926, 6))],
                    "the multi-slot belief rides the command"
                );
                assert_eq!(c.carried, (921, 3), "the carried lane untouched");
            }
            other => panic!("expected a click, got {other:?}"),
        }

        // the SHIFT+right press on the MIRROR moves the full stack back
        // to the grid (slot 0 forward) — the vanilla hasShiftDown
        // overrides the right button too
        let &(mx, my, mslot) = app
            .chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 50)
            .expect("the mirror's menu-50 box");
        assert_eq!(mslot, 50);
        app.on_right_press(mx + 8, my + 8);
        assert_eq!(app.chest_mirror[23], (0, 0), "the mirror's stack moved out");
        assert_eq!(app.chest_slots[0], (926, 64), "the grid's slot 0 took it");
        match click_rx.try_recv().expect("the mirror's shift click rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.slot, 50);
                assert_eq!(c.button, 1, "the right press's own button byte");
                assert_eq!(c.input, ContainerInput::QuickMove);
                assert_eq!(c.changed, vec![(50, (0, 0)), (0, (926, 64))]);
            }
            other => panic!("expected a click, got {other:?}"),
        }

        // the shift RELEASE restores the pickup table: the same spot's
        // plain left press now PLACES the held stack into the empty
        // slot (the pickup table's own place law — the shift's
        // whole-move routing is gone)
        app.on_shift_down(false);
        app.on_press(sx + 8, sy + 8);
        assert_eq!(app.chest_slots[4], (921, 3), "the held stack placed");
        assert_eq!(app.chest_carried, (0, 0), "the cursor emptied");
        match click_rx.try_recv().expect("the plain click rode") {
            ContainerCommand::Click(c) => {
                assert_eq!(c.input, ContainerInput::Pickup, "the plain press");
                assert_eq!(c.changed, vec![(4, (921, 3))]);
                assert_eq!(c.carried, (0, 0));
            }
            other => panic!("expected a click, got {other:?}"),
        }

        // the offline stand-in: the shift press never speaks
        app.on_escape();
        let _ = click_rx.try_recv(); // the close echo
        app.open_chest();
        app.render_title_buttons_for_test();
        let (ox, oy, _) = app
            .chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == 6)
            .copied()
            .expect("the offline box");
        app.on_shift_down(true);
        app.on_press(ox + 8, oy + 8);
        assert!(click_rx.try_recv().is_err(), "the offline shift press is silent");
    }

    /// The live held-item lane's laws: the store's selected stack
    /// loads its first-person model on change, the count is not the
    /// hand's business, the emptiness and the out-of-bake ids drop to
    /// the bare arm, and the wire id is the reload key.
    #[test]
    fn the_live_held_lane_loads_the_selected_model() {
        let mut app = App::new(3);
        assert!(app.held_item.is_none(), "the bare arm boots");
        assert_eq!(app.live_held, None);
        // the diamond's stack rides: the model loads
        app.live_held_updated(Some((926, 12)));
        assert_eq!(app.live_held, Some(926));
        assert!(app.held_item.is_some(), "the diamond's model loaded");
        // the same id, another count: the loaded model stands (the
        // count is the GUI's business, the hand renders the model)
        app.live_held_updated(Some((926, 5)));
        assert_eq!(app.live_held, Some(926));
        assert!(app.held_item.is_some());
        // the slot empties: the bare arm returns
        app.live_held_updated(Some((926, 0)));
        assert_eq!(app.live_held, None);
        assert!(app.held_item.is_none(), "the emptiness drops to the arm");
        // an id outside the bake domain (past the wire table): the
        // honest arm — no model, no lie; the id stays the store's truth
        app.live_held_updated(Some((5000, 3)));
        assert_eq!(app.live_held, Some(5000));
        assert!(app.held_item.is_none(), "no model, no lie");
        // back to the diamond: the model reloads on the id's change
        app.live_held_updated(Some((926, 1)));
        assert_eq!(app.live_held, Some(926));
        assert!(app.held_item.is_some(), "the model reloads");
    }

    /// The GUI icons' 3D law: the block items render their isometric
    /// geometry (non-blank, the tint law rides), the flat sprites fill
    /// the slot exactly, the specials' payloads land — the fallback
    /// blit covers only what the quad path refuses.
    #[test]
    fn the_item_icons_render_the_gui_quads() {
        let lit_in_slot = |g: &Gui| -> usize {
            let mut lit = 0;
            for py in 16..48 {
                for px in 16..48 {
                    let o = (py * 64 + px) * 3;
                    if g.fb.color[o] != 0 || g.fb.color[o + 1] != 0 || g.fb.color[o + 2] != 0 {
                        lit += 1;
                    }
                }
            }
            lit
        };
        let wire = |name: &str| -> i32 {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect(name) as i32
        };
        // the cobblestone icon: the element cube's isometric render —
        // the slot area carries pixels now (it was blank before this
        // round), the shaded faces riding the FACE_SHADE multipliers
        let mut fb = crate::raster::Framebuf::new(64, 64);
        let mut g = Gui::new(&mut fb, 2);
        draw_item_icon(&mut g, wire("cobblestone"), 8, 8);
        let lit = lit_in_slot(&g);
        assert!(lit > 100, "the cobblestone icon renders ({lit} px)");
        // the diamond: the flat sprite item — non-blank, centered
        let mut fb = crate::raster::Framebuf::new(64, 64);
        let mut g = Gui::new(&mut fb, 2);
        draw_item_icon(&mut g, wire("diamond"), 8, 8);
        let lit = lit_in_slot(&g);
        assert!(lit > 100, "the diamond icon renders ({lit} px)");
        // the grass block: the tint law rides — the icon's pixels lean
        // GREEN (the vanilla no-level climate's grass tint through the
        // audited colormap)
        let mut fb = crate::raster::Framebuf::new(64, 64);
        let mut g = Gui::new(&mut fb, 2);
        draw_item_icon(&mut g, wire("grass_block"), 8, 8);
        let mut green_lean = 0;
        for py in 16..48 {
            for px in 16..48 {
                let o = (py * 64 + px) * 3;
                let (r, gr, b) = (g.fb.color[o], g.fb.color[o + 1], g.fb.color[o + 2]);
                let painted = r != 0 || gr != 0 || b != 0;
                if painted && gr > r && gr >= b {
                    green_lean += 1;
                }
            }
        }
        assert!(green_lean > 20, "the grass icon tints green ({green_lean} px)");
        // the shield: the SPECIAL payload in the slot (the dump's
        // geometry, the base model's gui stanza)
        let mut fb = crate::raster::Framebuf::new(64, 64);
        let mut g = Gui::new(&mut fb, 2);
        draw_item_icon(&mut g, wire("shield"), 8, 8);
        let lit = lit_in_slot(&g);
        assert!(lit > 50, "the shield icon renders ({lit} px)");
    }

    /// The live held lane's SPECIAL payloads: the 63 block-entity /
    /// entity-model items route through the dump-backed geometry lane —
    /// the shield's 12 quads (2 parts x 1 cube x 6 faces) with the
    /// nopattern base texture, the white banner's tinted flag variant —
    /// and the emptiness law is the same honest arm as the model lane.
    #[test]
    fn the_live_held_lane_routes_the_special_payloads() {
        let mut app = App::new(3);
        let shield = mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == "shield")
            .expect("shield rides the wire") as i32;
        let banner = mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == "white_banner")
            .expect("white_banner rides the wire") as i32;
        // the shield's stack rides: the SPECIAL geometry loads
        app.live_held_updated(Some((shield, 1)));
        assert_eq!(app.live_held, Some(shield));
        let held = app.held_item.as_ref().expect("the shield loads");
        assert_eq!(held.quad_len(), 12);
        assert!(held.binds_texture("entity/shield/shield_base_nopattern"));
        // the banner: two layers (the rod + the tinted flag), the
        // flag's pixels riding the PER-COLOR pre-tinted variant
        app.live_held_updated(Some((banner, 1)));
        let held = app.held_item.as_ref().expect("the banner loads");
        assert!(held.binds_texture("entity/banner/banner_base_white"));
        // the emptiness law holds on the special lane too
        app.live_held_updated(Some((banner, 0)));
        assert!(app.held_item.is_none(), "the arm returns");
    }

    /// THE USE ARC: the right press raises the shield into its blocking
    /// pose (the using branch's own payload — the shield_blocking base
    /// poses the geometry, the mirror transform moves the centroid), the
    /// bow's hold pulls through the charge states (the javap-pinned
    /// charge law: 13 ticks -> pulling_1, 18 -> pulling_2), the release
    /// returns the resting payload, and the wire echo rides the command
    /// channel (the offline stand-in without a sink stays cosmetic).
    #[test]
    fn the_use_arc_raises_the_shield_and_pulls_the_bow() {
        use crate::world_view::ContainerCommand;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);

        // THE SHIELD: the resting special loads first
        app.live_held_updated(Some((wire("shield"), 1)));
        let resting = app.held_item.as_ref().expect("the resting shield loads");
        assert_eq!(resting.quad_len(), 12);
        let resting_centroid = resting.centroid();
        assert!(app.on_right_press(0, 0), "the shield starts a use");
        assert!(app.is_using(), "the use is active");
        let blocking = app.held_item.as_ref().expect("the blocking payload loads");
        assert_eq!(blocking.quad_len(), 12, "the same geometry — the same 12 quads");
        assert!(blocking.binds_texture("entity/shield/shield_base_nopattern"));
        let blocking_centroid = blocking.centroid();
        assert!(
            (blocking_centroid[0] - resting_centroid[0]).abs() > 0.01
                || (blocking_centroid[1] - resting_centroid[1]).abs() > 0.01,
            "the blocking stanza MOVES the payload ({resting_centroid:?} -> {blocking_centroid:?})"
        );
        // the use echo rode the command channel: the camera's angles in
        // the vanilla degrees (the flight camera faces east — yaw −90)
        match click_rx.try_recv().expect("the use echo rode the lane") {
            ContainerCommand::UseItem { yaw, pitch } => {
                assert_eq!(yaw, -90.0, "the vanilla yRot");
                assert!((pitch - 4.583).abs() < 0.01, "the vanilla xRot");
            }
            other => panic!("expected a UseItem echo, got {other:?}"),
        }
        // the shield has no charge lane: the ticks advance, the state
        // holds (no reload churn)
        for _ in 0..30 {
            app.update();
        }
        assert_eq!(app.using_item.as_ref().unwrap().state, 0, "the pure gate holds");
        // THE RELEASE: the falling edge returns the resting payload
        app.on_right_release();
        assert!(!app.is_using());
        let back = app.held_item.as_ref().expect("the resting payload returns");
        let back_centroid = back.centroid();
        assert!(
            (back_centroid[0] - resting_centroid[0]).abs() < 1e-4
                && (back_centroid[1] - resting_centroid[1]).abs() < 1e-4,
            "the rest pose is the one the arc started from"
        );
        assert!(matches!(click_rx.try_recv(), Ok(ContainerCommand::ReleaseUse)));

        // THE BOW: the hold pulls through the charge states — 13 ticks
        // cross 0.65, 18 cross 0.9 (the UseDuration x scale law)
        app.live_held_updated(Some((wire("bow"), 1)));
        assert!(!app.is_using(), "the item switch ended the old use");
        assert!(app.on_right_press(0, 0), "the bow starts a use");
        match click_rx.try_recv().expect("the bow's use echo rode the lane") {
            ContainerCommand::UseItem { .. } => {}
            other => panic!("expected a UseItem echo, got {other:?}"),
        }
        let pulling0 = app.held_item.as_ref().expect("pulling_0 loads");
        assert!(pulling0.binds_texture("item/bow_pulling_0"), "the bottom lane");
        for _ in 0..13 {
            app.update();
        }
        let pulling1 = app.held_item.as_ref().expect("pulling_1 loads");
        assert!(pulling1.binds_texture("item/bow_pulling_1"), "13 * 0.05 = 0.65");
        for _ in 0..5 {
            app.update();
        }
        let pulling2 = app.held_item.as_ref().expect("pulling_2 loads");
        assert!(pulling2.binds_texture("item/bow_pulling_2"), "18 * 0.05 = 0.90");
        app.on_right_release();
        let resting_bow = app.held_item.as_ref().expect("the resting bow returns");
        assert!(resting_bow.binds_texture("item/bow"), "the condition's on_false");
        assert!(matches!(click_rx.try_recv(), Ok(ContainerCommand::ReleaseUse)));

        // A PLAIN ITEM: no using lane — the press is the honest no-op
        app.live_held_updated(Some((wire("diamond"), 1)));
        assert!(!app.on_right_press(0, 0), "the diamond cannot start a use");
        assert!(!app.is_using());
        assert!(click_rx.try_recv().is_err(), "no echo either");
    }

    /// THE CROSSBOW'S COMPONENT ARC — the charge_type select's consumer:
    /// the full-charge release LOADS the projectile (javap
    /// releaseUsing: power >= 1.0 -> tryLoadProjectiles sets the
    /// CHARGED_PROJECTILES component — the pull loaded, it did not
    /// fire), the resting render switches to the charged bolt (the
    /// select's arrow case), the next right press fires INSTANTLY
    /// (javap use: the component non-empty -> performShooting + CONSUME
    /// — no pull), and the component belongs to the STACK (the item
    /// switch drops it).
    #[test]
    fn the_crossbow_loads_on_the_full_charge_and_fires_instantly() {
        use crate::world_view::ContainerCommand;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);

        // THE UNCHARGED PULL: the fresh stack carries no component, the
        // press starts the pull like the bow's
        app.live_held_updated(Some((wire("crossbow"), 1)));
        assert!(app.held_charge.is_none(), "the fresh stack carries no component");
        assert!(app.on_right_press(0, 0), "the uncharged crossbow starts the pull");
        match click_rx.try_recv().expect("the pull's use echo rode the lane") {
            ContainerCommand::UseItem { .. } => {}
            other => panic!("expected a UseItem echo, got {other:?}"),
        }
        let pulling = app.held_item.as_ref().expect("pulling_0 loads");
        assert!(pulling.binds_texture("item/crossbow_pulling_0"));
        // 24 ticks then the release: under the charge duration — the
        // power < 1.0 loads nothing (the javap releaseUsing gate)
        for _ in 0..24 {
            app.update();
        }
        app.on_right_release();
        assert!(app.held_charge.is_none(), "24 < 25 ticks — no load");
        let resting = app.held_item.as_ref().expect("the resting crossbow returns");
        assert!(resting.binds_texture("item/crossbow_standby"), "the model's own layer0");
        assert!(matches!(click_rx.try_recv(), Ok(ContainerCommand::ReleaseUse)));

        // THE FULL CHARGE: 25 ticks then the release — the LOAD law and
        // the resting render's switch to the loaded bolt (the select's
        // arrow case rides the crossbow_arrow texture)
        assert!(app.on_right_press(0, 0), "the pull starts again");
        match click_rx.try_recv().expect("the second pull's echo rode the lane") {
            ContainerCommand::UseItem { .. } => {}
            other => panic!("expected a UseItem echo, got {other:?}"),
        }
        for _ in 0..25 {
            app.update();
        }
        app.on_right_release();
        assert_eq!(
            app.held_charge,
            Some(mc_registry::ChargeKind::Arrow),
            "the full charge LOADED the arrow (the component set)"
        );
        let loaded = app.held_item.as_ref().expect("the charged payload loads");
        assert!(loaded.binds_texture("item/crossbow_arrow"), "the charge_type select's arrow case");
        assert!(matches!(click_rx.try_recv(), Ok(ContainerCommand::ReleaseUse)));

        // THE INSTANT SHOT: the right press on the charged crossbow
        // consumes the charge — no pull starts (the vanilla CONSUME
        // branch), the plain crossbow renders again, and the shot's
        // UseItem echo rides the wire lane
        assert!(app.on_right_press(0, 0), "the charged crossbow answers the press");
        assert!(app.held_charge.is_none(), "the shot consumed the charge");
        assert!(!app.is_using(), "the shot is instant — no pull started");
        let plain = app.held_item.as_ref().expect("the plain crossbow returns");
        assert!(plain.binds_texture("item/crossbow_standby"), "the standby texture returns");
        match click_rx.try_recv().expect("the shot's use echo rode the lane") {
            ContainerCommand::UseItem { .. } => {}
            other => panic!("expected a UseItem echo, got {other:?}"),
        }

        // THE ROCKET SEED: the component lane's other case (the CLI
        // seed's own domain) renders the firework bolt
        app.held_charge = Some(mc_registry::ChargeKind::Rocket);
        app.load_held_payload(wire("crossbow"), None);
        let rocket = app.held_item.as_ref().expect("the rocket payload loads");
        assert!(rocket.binds_texture("item/crossbow_firework"));

        // THE STACK SWITCH: the component belongs to the stack, never
        // the slot — the item switch drops it
        app.live_held_updated(Some((wire("diamond"), 1)));
        assert!(app.held_charge.is_none(), "the switch dropped the charge");
    }

    /// THE BLOCK-USE LANE's OFFLINE ARC — the App owns the procedural
    /// voxels, the `--held-item` stand-in holds dirt, the look seam aims
    /// the camera straight down at the spawn column, and the placement
    /// lands in the hit face's adjacent cell (the mesh re-speaks). The
    /// reach ends the lane honestly (the sky miss consumes nothing) and
    /// the vanilla refuses the placement into the player's own box.
    #[test]
    fn the_block_use_lane_places_the_held_block_offline() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/dirt"));
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        assert_eq!(app.offline_held, Some(wire("dirt")), "the reverse lookup route");
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        // top_solid_y speaks the STANDING cell (the surface's top solid
        // Y + 1) — the placement cell the straight-down ray fills
        let top = app.spawn_y;
        // THE AIM: the eye 3.0 above the standing cell's floor, straight
        // down — the hit voxel is the surface block (top-1), the face
        // UP, the adjacent cell = top itself
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::AIR,
            "the placement cell starts empty"
        );
        let quads_before = app.world_quads.as_ref().unwrap().len();
        assert!(app.on_right_press(0, 0), "the placement consumed the press");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::DIRT,
            "the held dirt landed in the adjacent cell"
        );
        assert_ne!(
            app.world_quads.as_ref().unwrap().len(),
            quads_before,
            "the mesh re-spoke with the placed voxel"
        );
        // THE REACH MISS: 40 above the surface — the clip ends in the air,
        // the press consumes nothing (the vanilla's own no-hit use)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 40.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(!app.on_right_press(0, 0), "the miss does not consume");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top + 2, c),
            crate::world::AIR,
            "no voxel landed on the miss"
        );
        // THE PLAYER'S OWN BOX: the feet standing ON the placed dirt (the
        // eye 1.62 above the feet cell's floor) — the next placement cell
        // is the feet's own cell and the vanilla refuses it
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 2.0 + 0.62, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(!app.on_right_press(0, 0), "the vanilla refuses the self-intersecting placement");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top + 2, c),
            crate::world::AIR,
            "the refused cell stays air"
        );
    }

    /// THE BLOCK-USE LANE's NETWORKED ARC — the clip runs over the net
    /// world, the UseItemOn echo rides the command channel (the
    /// worker's serverbound packet), and the LOCAL voxels stay
    /// untouched (the server's own chunk refresh is the authority on
    /// this lane).
    #[test]
    fn the_block_use_lane_echoes_the_networked_world() {
        use crate::world_view::ContainerCommand;
        let wire = |name: &str| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .iter()
                .position(|&n| n == name)
                .expect("the item rides the wire") as i32
        };
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        // the net world with a stone floor under the spawn column
        let mut net = air_net_world(2, 0);
        let c = net.size_blocks() / 2;
        let floor_y = net.size_blocks() / 4;
        for x in 0..net.size_blocks() {
            for z in 0..net.size_blocks() {
                net.set(x, floor_y, z, crate::world::STONE);
            }
        }
        app.net_world = Some(net);
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.live_held_updated(Some((wire("dirt"), 1)));
        // the camera: the eye 3.0 above the floor plane, straight down
        app.net_camera = Some(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, floor_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the block lane answers the press");
        match click_rx.try_recv().expect("the useItemOn echo rode the lane") {
            ContainerCommand::UseItemOn { pos, face, cursor, inside } => {
                assert_eq!(pos, (c, floor_y, c), "the hit voxel");
                assert_eq!(face, 1, "UP — the floor's entry face");
                // the cursor rides the float cast's own tail (the
                // vanilla's double math + f32 wire cast speaks the same
                // epsilon) — the near-exact face-centered offsets
                assert!((cursor.0 - 0.5).abs() < 1e-3, "cursor x {cursor:?}");
                assert!((cursor.1 - 1.0).abs() < 1e-3, "cursor y {cursor:?}");
                assert!((cursor.2 - 0.5).abs() < 1e-3, "cursor z {cursor:?}");
                assert!(!inside);
            }
            other => panic!("expected a UseItemOn echo, got {other:?}"),
        }
        // the local voxels are untouched — the server's refresh owns the
        // networked terrain
        assert_eq!(
            app.net_world.as_ref().unwrap().get(c, floor_y + 1, c),
            crate::world::AIR,
            "no local prediction on the networked lane"
        );
    }


    /// THE DIG LANE's OFFLINE ARC — the left press arms the destroy
    /// (startDestroyBlock), the hold mines on the frame==tick model
    /// (continueDestroyBlock's own beat), the completion breaks the cell
    /// and the mesh re-speaks (the placement's own rhythm), the release
    /// mid-dig aborts, and the air press arms the missTime anti-spam
    /// window the vanilla's survival speaks.
    #[test]
    fn the_dig_lane_mines_the_offline_world() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1; // the top SOLID cell the ray hits
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        let kind_before = app.offline_world.as_ref().unwrap().get(c, surface, c);
        assert_ne!(kind_before, crate::world::AIR, "the surface cell is solid");
        let mesh_before = app.world_quads.as_ref().unwrap().clone();
        // THE PRESS: the dig arms, the swing beats (the vanilla
        // startAttack's own tail)
        app.on_press(0, 0);
        assert!(app.dig.is_destroying, "the press armed the destroy");
        assert_eq!(app.dig.pos, (c, surface, c), "the armed cell");
        assert!(app.dig_held, "the hold flag rides the edge");
        assert!(app.swing.swinging, "the press swung the arm");
        // THE HOLD: the ticks mine (the frame==tick model), the crack
        // stage climbs, the completion breaks the cell
        let mut ticks = 0;
        while app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::AIR {
            app.update();
            ticks += 1;
            assert!(ticks < 400, "the dig completed in time");
        }
        assert!(ticks > 1, "the dig took real beats ({ticks})");
        assert!(!app.dig.is_destroying, "the completion disarmed");
        assert_eq!(app.dig.delay, 5, "the completion's cooldown armed");
        assert_ne!(
            *app.world_quads.as_ref().unwrap(),
            mesh_before,
            "the mesh re-spoke with the broken cell (the quad COUNT may tie on a sloped cell — the faces move)"
        );
        // THE SOUND ARC (sound.rs): the hit beats rode the %4 law (the dig
        // ticks 0, 4, 8, ...), the completion spoke the 2001 break — the
        // folds' own numbers ride the entries
        use mc_registry::generated::block_sounds as bs;
        let name = crate::world::NAME_OF[kind_before as usize];
        let idx = bs::sound_index(name).expect("the surface block in the sound table");
        let (hit_ev, break_ev) = (bs::SOUND_HIT[idx], bs::SOUND_BREAK[idx]);
        let hits = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with(&format!("{hit_ev} @")))
            .count();
        assert_eq!(hits, (ticks + 3) / 4, "the beat count for {ticks} dig ticks");
        let hit_entry = app
            .sounds
            .events
            .iter()
            .find(|e| e.starts_with(&format!("{hit_ev} @")))
            .expect("the first beat rode the FIRST continue tick (dig tick 0)");
        assert!(
            hit_entry.contains(" v0.250000 p0.500000"),
            "the hit fold (v+1)/8, p*0.5: {hit_entry}"
        );
        let break_entry = app
            .sounds
            .events
            .iter()
            .find(|e| e.starts_with(&format!("{break_ev} @")))
            .expect("the completion's break voice");
        assert!(
            break_entry.contains(" v1.000000 p0.800000"),
            "the break fold (v+1)/2, p*0.8: {break_entry}"
        );
        // THE COOLDOWN + THE COLUMN CONTINUATION (the vanilla dig-down
        // rhythm): the eye drops to reach the column's next cell — the
        // hold ticks the delay away (each cooldown tick still beats the
        // swing), then the retarget arms the next cell through
        // startDestroyBlock (continueDestroyBlock's own tail call)
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 2.5, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(app.dig.delay, 0, "the cooldown ticked away on the hold");
        assert!(!app.dig.is_destroying, "the dig stays disarmed through the cooldown");
        app.update();
        assert!(
            app.dig.is_destroying && app.dig.pos == (c, surface - 1, c),
            "the column's next cell armed through the retarget"
        );
        // THE RELEASE MID-DIG: two beats into the new cell, the falling
        // edge aborts (the crack clears, the cell survives)
        app.update();
        app.update();
        app.on_left_release();
        assert!(!app.dig.is_destroying, "the release aborted");
        assert_eq!(app.dig.progress, 0.0, "the crack progress cleared");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, surface - 1, c),
            crate::world::DIRT,
            "the aborted cell survives the abort"
        );
        // THE AIR MISS: aimed at the sky, the press arms the missTime
        // window (survival's hasMissTime) and the dig never arms
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: std::f32::consts::FRAC_PI_2,
        });
        app.on_press(0, 0);
        assert!(!app.dig.is_destroying, "the air press never armed");
        assert_eq!(app.miss_time, 10, "the anti-spam window armed");
    }

    /// THE BLOCK-SOUND ARC's PLACE VOICE: the right-press placement
    /// speaks the placed block's place sound at BlockItem.place's own
    /// fold — (v+1)/2 volume, p*0.8 pitch — one entry per press, the
    /// dirt's GRAVEL SoundType row naming the event.
    #[test]
    fn the_block_placement_speaks_the_place_voice() {
        let mut app = App::new(3);
        app.enter_world();
        app.offline_held_updated(Some("item/dirt"));
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::AIR,
            "the placement cell starts empty"
        );
        let before = app.sounds.events.len();
        assert!(app.on_right_press(0, 0), "the placement consumed the press");
        assert_eq!(app.sounds.events.len(), before + 1, "exactly one voice");
        let entry = &app.sounds.events[before];
        assert!(
            entry.starts_with("block.gravel.place @")
                && entry.contains(" v1.000000 p0.800000"),
            "the place fold on the dirt's GRAVEL row: {entry}"
        );
        // THE MISS PLACES NOTHING: the sky press consumes nothing and
        // adds no voice
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: std::f32::consts::FRAC_PI_2,
        });
        let before = app.sounds.events.len();
        assert!(!app.on_right_press(0, 0), "the sky miss never places");
        assert_eq!(app.sounds.events.len(), before, "the miss is silent");
    }

    /// THE DROP LANE'S REWARD ARC: the dirt break spawns its item entity
    /// by the popResource law (the spread, the 10-tick delay), the body
    /// standing in the hole picks it up through the aiStep scan — the
    /// inventory add, the take voice, the hotbar's projection, the
    /// entity's honest discard.
    #[test]
    fn the_drop_lane_rewards_the_dig() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        let kind_before = app.offline_world.as_ref().unwrap().get(c, surface, c);
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.drops.is_empty(), "the lane starts empty");
        // THE DIG: the hold mines the surface cell to air
        app.on_press(0, 0);
        let mut ticks = 0;
        while app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::AIR {
            app.update();
            ticks += 1;
            assert!(ticks < 400, "the dig completed in time");
        }
        // THE DROP: exactly one item entity (the hand-lane row's own
        // single entry), the popResource laws on the fields
        assert_eq!(app.drops.len(), 1, "the break spawned its drop");
        let drop_item = app.drops[0].item;
        // the item id resolves through the generated table's own row
        let name = crate::world::NAME_OF[kind_before as usize];
        let pools = mc_registry::generated::block_drops::drops(name)
            .expect("the broken block's loot row exists");
        let expected = mc_registry::item_id_by_name(pools[0].entries[0].item).unwrap();
        assert_eq!(drop_item, expected, "the drop rides the table's own row");
        assert_eq!(app.drops[0].count, 1);
        // the spawn update's own drop tick already drained one (the dig
        // completion and the entity tick share that update)
        assert_eq!(app.drops[0].pickup_delay, 9, "the popResource delay, one tick spoken");
        assert_eq!(app.drops[0].id, 1, "the offline id counter");
        assert!(
            (app.drops[0].x - (c as f64 + 0.5)).abs() <= 0.25
                && (app.drops[0].z - (c as f64 + 0.5)).abs() <= 0.25,
            "the ±0.25 spread around the block center"
        );
        // THE HOLD ENDS: the release clears the dig (no re-arm while the
        // test's body materializes in the hole)
        app.on_left_release();
        // THE BODY in the hole: the feet on the broken cell's floor — the
        // scan's inflate reaches the resting item
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            surface as f64,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        // THE TICKS: the item's delay drains, the scan speaks the pickup
        for _ in 0..20 {
            app.update();
        }
        assert!(app.drops.is_empty(), "the entity discarded after the take");
        assert_eq!(
            app.inventory.items[0],
            (drop_item, 1),
            "the hotbar's first slot holds the take"
        );
        assert_eq!(app.hotbar_slots[0], (drop_item, 1), "the HUD's projection");
        let pickup_entry = app
            .sounds
            .events
            .iter()
            .find(|e| e.starts_with("entity.item.pickup @"))
            .expect("the take voice spoke");
        assert!(
            pickup_entry.contains(" v0.200000 p"),
            "the pickup fold's own volume: {pickup_entry}"
        );
    }

    /// THE SURVIVAL CONSUME: the placement rides the selected stack's
    /// count down (BlockItem.place's own shrink), the HUD's projection
    /// follows, and the empty slot clears (the held lane falls back).
    #[test]
    fn the_place_lane_consumes_the_selected_stack() {
        let mut app = App::new(3);
        app.enter_world();
        let dirt_id = mc_registry::item_id_by_name("dirt").unwrap();
        // the selected slot carries 3 dirt — the pickups' own lane
        app.inventory.items[0] = (dirt_id, 3);
        app.hotbar_slots = app.inventory.hotbar();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let top = app.spawn_y;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::AIR,
            "the placement cell starts empty"
        );
        assert!(app.on_right_press(0, 0), "the placement consumed the press");
        assert_eq!(
            app.offline_world.as_ref().unwrap().get(c, top, c),
            crate::world::DIRT,
            "the voxel landed"
        );
        assert_eq!(
            app.inventory.items[0],
            (dirt_id, 2),
            "the count rode down one"
        );
        assert_eq!(app.hotbar_slots[0], (dirt_id, 2), "the projection followed");
        // THE SECOND PLACE: the crosshair now rides the placed dirt's own
        // UP face — the next cell lands on top, the count rides to 1
        assert!(app.on_right_press(0, 0), "the second place spoke");
        assert_eq!(app.inventory.items[0], (dirt_id, 1), "the count rode to 1");
        // THE REFUSAL: the next cell (the column's third) intersects the
        // camera's own standing box — the vanilla's own guard, the count
        // keeps its 1
        assert!(!app.on_right_press(0, 0), "the placement into the player refuses");
        assert_eq!(app.inventory.items[0], (dirt_id, 1), "the refused place consumes nothing");
        // THE CAMERA LIFTS: the same column's next cell no longer
        // intersects — the last stack places, the slot clears
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 6.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        assert!(app.on_right_press(0, 0), "the third place spoke");
        assert_eq!(app.inventory.items[0], (0, 0), "the empty slot cleared");
        assert!(!app.on_right_press(0, 0), "the empty selection never places");
    }

    /// THE FALL DAMAGE ARC (the hurt round's own headline): the body
    /// spawned ten blocks above the surface falls, the landing folds
    /// damage 7 through calculateFallDamage, the pipeline speaks the
    /// fall voice (the FIXED 1.0/1.0 lane), the hurt voice (the
    /// (r-r)*0.2+1.0 fold), the health drops 20 -> 13, the flash arms
    /// (hurtTime 10) and the cooldown re-arms (invulnerableTime 20).
    #[test]
    fn the_fall_damage_hurts_the_player() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        // the body ten blocks above the spawn surface (the feet on the
        // top solid cell's surface, ten up)
        let spawn_feet = app.spawn_y;
        app.body = Some(crate::body::PlayerBody::new(
            c as f64 + 0.5,
            (spawn_feet + 10) as f64,
            c as f64 + 0.5,
            0.0,
            0.0,
        ));
        assert_eq!(app.health, 20.0, "the untouched player starts full");
        // the ticks until the landing speaks
        let mut landed = false;
        for _ in 0..60 {
            app.update();
            if app.health != 20.0 {
                landed = true;
                break;
            }
        }
        assert!(landed, "the landing spoke inside the ticks");
        // THE CLAMP'S OWN ARITHMETIC: the 10-block descent banks
        // 10.0 minus the rising clamp's 0.1435 (the
        // checkFallDistanceAccumulation law) — the landing folds 6
        // (floor(9.8565 + 1e-6 - 3.0)), the health rides to 14
        assert_eq!(app.health, 14.0, "the clamp-loss verdict, the bytecode's own");
        assert_eq!(app.hurt_time, 10, "the flash armed at its full window");
        assert_eq!(app.hurt_duration, 10);
        assert_eq!(app.invulnerable_time, 20, "the cooldown re-armed");
        assert!(!app.dead, "six damage never kills");
        // THE VOICES: the fall's own fixed lane, then the hurt's pitched
        // one (causeFallDamage's own order)
        let fall = app
            .sounds
            .events
            .iter()
            .position(|e| e.contains("_fall @"))
            .expect("the fall voice spoke");
        assert!(
            app.sounds.events[fall].contains(" v1.000000 p1.000000"),
            "the fall's fixed fold: {}",
            app.sounds.events[fall]
        );
        let hurt = app
            .sounds
            .events
            .iter()
            .position(|e| e.starts_with("entity.player.hurt @"))
            .expect("the hurt voice spoke");
        assert!(hurt > fall, "the hurt voice follows the fall's own");
        // THE COOLDOWN'S ECHO: a second landing inside the window refuses
        // the smaller repeat (the lastHurt comparison) — the health stays
        app.hurt(5.0);
        assert_eq!(
            app.health, 14.0,
            "the 5-damage repeat inside the window refuses (5 <= lastHurt 6)"
        );
    }

    /// THE HURT COOLDOWN LAW (javap hurtServer's own comparison chain):
    /// the fresh hurt re-arms the 20-tick window; a bigger hurt inside
    /// the 10-tick sub-window lands for the DIFFERENCE only; a smaller
    /// one refuses; a hurt after the window re-lands whole.
    #[test]
    fn the_hurt_cooldown_walks_the_javap_chain() {
        let mut app = App::new(3);
        app.enter_world();
        // the regen lane silenced for the walk's own arithmetic (the
        // saturation empty — the slow lane's 80-tick beat never fires
        // inside the 25-tick walk)
        app.food_data.set_saturation(0.0);
        // the fresh hurt: the whole amount lands
        app.hurt(2.0);
        assert_eq!(app.health, 18.0);
        assert_eq!(app.invulnerable_time, 20);
        assert_eq!(app.hurt_time, 10);
        assert_eq!(app.last_hurt, 2.0);
        // five ticks: the window still holds (15 > 10)
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(app.invulnerable_time, 15);
        // the BIGGER hurt lands the difference
        app.hurt(3.0);
        assert_eq!(app.health, 17.0, "3 - lastHurt 2 = 1 damage");
        assert_eq!(app.invulnerable_time, 15, "the window never re-arms");
        assert_eq!(app.hurt_time, 5, "the flash never re-arms either");
        // the SMALLER hurt refuses outright
        app.hurt(2.0);
        assert_eq!(app.health, 17.0, "2 <= lastHurt 3 refused");
        // the window drains: the fresh hurt re-lands whole
        for _ in 0..20 {
            app.update();
        }
        assert_eq!(app.invulnerable_time, 0);
        app.hurt(2.0);
        assert_eq!(app.health, 15.0, "the fresh hurt lands whole");
        assert_eq!(app.invulnerable_time, 20, "re-armed");
        assert_eq!(app.hurt_time, 10, "the flash re-armed");
    }

    /// THE DEATH ARC AND THE RESPAWN (the vanilla's own beat pair): the
    /// killing hurt speaks the death voice, arms the death screen (the
    /// HUD sleeps, the gradient owns the world), zeroes the input; the
    /// Respawn resets the vitals and re-anchors the body to the spawn
    /// pose with the fall state cleared.
    #[test]
    fn the_death_arc_respawns_at_the_spawn() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        let spawn = app.spawn_pose.expect("the spawn pose recorded");
        // THE KILLING HURT
        app.hurt(20.0);
        assert_eq!(app.health, 0.0, "the health clamped at zero");
        assert!(app.dead, "the death screen armed");
        app.sounds
            .events
            .iter()
            .find(|e| e.starts_with("entity.player.death @"))
            .expect("the death voice spoke");
        // further hurts refuse (the isDeadOrDying gate)
        app.hurt(5.0);
        assert_eq!(app.health, 0.0);
        // THE INPUT GATE: the dead body's impulses zero through update
        if let Some(b) = app.body.as_mut() {
            b.forward = 1.0;
            b.jumping = true;
        }
        app.update();
        if let Some(b) = app.body.as_ref() {
            assert_eq!(b.forward, 0.0, "the corpse's impulses zero");
            assert!(!b.jumping, "the corpse never jumps");
        }
        // THE ATTACK GATE: the press never arms a dig while dead
        app.on_attack();
        assert!(!app.dig.is_destroying, "the dead hand never mines");
        // THE RESPAWN: the vitals reset, the body re-anchors to the spawn
        // pose (the vanilla respawn's own teleport), the fall clears
        if let Some(b) = app.body.as_mut() {
            b.x += 3.0;
            b.fall_distance = 4.0;
        }
        app.respawn();
        assert!(!app.dead, "the death screen closed");
        assert_eq!(app.health, 20.0, "the vitals back to max");
        assert_eq!(app.invulnerable_time, 0);
        let b = app.body.as_ref().expect("the body stands");
        assert_eq!(b.x, spawn.0, "the feet re-anchored to the spawn pose");
        assert_eq!(b.y, spawn.1);
        assert_eq!(b.z, spawn.2);
        assert_eq!(b.fall_distance, 0.0, "the fall state cleared");
    }

    /// THE WIRE'S VITALS LANE (the Set Health / Hurt Animation / Combat
    /// Kill projections through live_health_updated): the store's truth
    /// lands verbatim, the death message arms the death gate, the
    /// respawn wire rides the command channel.
    #[test]
    fn the_wire_vitals_land_and_the_respawn_echoes() {
        let mut app = App::new(3);
        app.enter_world();
        // the vitals snapshot lands verbatim
        app.live_health_updated(crate::world_view::HealthView {
            health: 6.5,
            food: 18,
            saturation: 5.0,
            hurt_time: 10,
            hurt_yaw: -45.5,
            death_message: None,
            effects: Vec::new(),
        });
        assert_eq!(app.health, 6.5, "the half heart's own float");
        assert_eq!(app.food_data.food_level(), 18);
        assert_eq!(app.food_data.saturation(), 5.0);
        assert_eq!(app.hurt_time, 10, "the flash window rides");
        assert_eq!(app.hurt_yaw, -45.5, "the tilt yaw rides");
        assert!(!app.dead);
        // the death message arms the gate (the wire's own death)
        app.live_health_updated(crate::world_view::HealthView {
            health: 0.0,
            food: 0,
            saturation: 0.0,
            hurt_time: 0,
            hurt_yaw: 0.0,
            death_message: Some("Player fell from a high place".to_string()),
            effects: Vec::new(),
        });
        assert!(app.dead, "the message armed the death gate");
        assert_eq!(
            app.death_message.as_deref(),
            Some("Player fell from a high place"),
            "the cause line's own text"
        );
        // the respawn echo rides the command channel when a session
        // exists (the offline lane's channel is none — the no-op)
        let (tx, rx) = std::sync::mpsc::channel();
        app.live_clicks = Some(tx);
        app.respawn();
        match rx.try_recv().expect("the respawn echoed") {
            crate::world_view::ContainerCommand::Respawn => {}
            other => panic!("the wrong command rode the lane: {other:?}"),
        }
        assert!(!app.dead);
    }

    /// THE BLOCK-SOUND ARC'S FOOTSTEPS: the walking body's steps ride
    /// Entity.move's own cadence — moveDist grows by the horizontal
    /// movement × 0.6, a step fires per whole moveDist unit over a solid
    /// block under the feet — and the step voice speaks the block under
    /// at Entity.playStepSound's own fold (v*0.15, the pitch untouched).
    #[test]
    fn the_walking_body_speaks_the_footstep_cadence() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.on_movement_input(1.0, 0.0, false, false, false);
        for _ in 0..40 {
            app.update();
        }
        let b = app.body.as_ref().unwrap();
        assert!(
            b.move_dist > 2.0,
            "the walk accumulated the vanilla law ({})",
            b.move_dist
        );
        let steps: Vec<&String> = app
            .sounds
            .events
            .iter()
            .filter(|e| e.contains(".step @"))
            .collect();
        // the walk crosses the lakebed's own slope (the honest seed-3
        // terrain): the airborne crossings stay silent — the vanilla
        // isAir gate — so the count never EXCEEDS the whole units
        assert!(
            !steps.is_empty() && steps.len() <= b.move_dist as i32 as usize,
            "the steps rode the cadence ({} steps for moveDist {})",
            steps.len(),
            b.move_dist
        );
        for s in &steps {
            assert!(
                s.contains(" v0.150000 p1.000000"),
                "the step fold on every entry: {s}"
            );
        }
        // the events the steps named are real generated sound events
        use mc_registry::generated::sounds as snd;
        for s in &steps {
            let ev = s.split(" @").next().unwrap();
            assert!(snd::event(ev).is_some(), "the step event exists: {ev}");
        }
        // the body rested on solid ground the whole walk (the lakebed
        // spawn's own honest floor) — no floating step possible
        assert!(b.on_ground, "the walk stayed on the terrain");
    }

    /// THE DIG LANE's NETWORKED ARC — the destroy state arms and mines
    /// over the net world, the START/ABORT echoes ride the command
    /// channel (the worker's serverbound Player Action packets), the
    /// SWING rides its own command, and the LOCAL voxels stay untouched
    /// (the server's own chunk refresh is the authority on this lane).
    #[test]
    fn the_dig_lane_echoes_the_networked_world() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        // the net world with a stone floor under the spawn column
        let mut net = air_net_world(2, 0);
        let c = net.size_blocks() / 2;
        let floor_y = net.size_blocks() / 4;
        for x in 0..net.size_blocks() {
            for z in 0..net.size_blocks() {
                net.set(x, floor_y, z, crate::world::STONE);
            }
        }
        app.net_world = Some(net);
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);
        app.net_camera = Some(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, floor_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        // THE PRESS: the START + the swing ride the lane
        app.on_press(0, 0);
        assert!(app.dig.is_destroying, "the net lane armed the destroy");
        match click_rx.try_recv().expect("the START rode the lane") {
            ContainerCommand::DigAction { action, pos, face } => {
                assert_eq!(action, crate::dig::ACTION_START, "the START ordinal");
                assert_eq!(pos, (c, floor_y, c), "the hit voxel");
                assert_eq!(face, 1, "UP — the floor's entry face");
            }
            other => panic!("expected a DigAction START, got {other:?}"),
        }
        match click_rx.try_recv().expect("the swing rode the lane") {
            ContainerCommand::Swing => {}
            other => panic!("expected a Swing, got {other:?}"),
        }
        // THE HOLD: the progress ticks (the hand-on-stone 100 divisor),
        // the local voxels stay the server's own
        for _ in 0..3 {
            app.update();
        }
        assert!(app.dig.progress > 0.0, "the net lane's progress ticks");
        // the mining beats' swings queued on the lane; drain them (the
        // vanilla's own per-beat echo)
        let mut beats = 0;
        while let Ok(cmd) = click_rx.try_recv() {
            match cmd {
                ContainerCommand::Swing => beats += 1,
                other => panic!("unexpected lane traffic mid-dig: {other:?}"),
            }
        }
        assert!(beats >= 1, "the mining beats swung");
        assert_eq!(
            app.net_world.as_ref().unwrap().get(c, floor_y, c),
            crate::world::STONE,
            "no local prediction on the networked lane"
        );
        // THE RELEASE: the ABORT rides DOWN (the vanilla's own constant)
        app.on_left_release();
        match click_rx.try_recv().expect("the ABORT rode the lane") {
            ContainerCommand::DigAction { action, pos, face } => {
                assert_eq!(action, crate::dig::ACTION_ABORT, "the ABORT ordinal");
                assert_eq!(pos, (c, floor_y, c), "the held cell");
                assert_eq!(face, 0, "DOWN — the release's own direction");
            }
            other => panic!("expected a DigAction ABORT, got {other:?}"),
        }
        assert!(!app.dig.is_destroying);
    }

    /// The number keys' whole arc: the HUD's selection moves with the
    /// key, the LIVE session's belief rides the command channel (the
    /// SelectHotbar command the worker translates to the wire AND the
    /// store's own move), the out-of-domain and the repeat drop, and
    /// the offline stand-in keeps the move cosmetic.
    #[test]
    fn the_number_key_moves_the_selection_and_echoes() {
        use crate::world_view::ContainerCommand;
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
        app.attach_live_clicks(click_tx);

        // the key moves the HUD's selection and rides the command
        app.on_number_key(4);
        assert_eq!(app.selected_hotbar, 4, "the HUD's own belief moved");
        match click_rx.try_recv().expect("the selection rode the lane") {
            ContainerCommand::SelectHotbar { selection } => {
                assert_eq!(selection, 4, "the SELECTION-relative slot");
            }
            other => panic!("expected a selection, got {other:?}"),
        }

        // the repeat naming the held slot is the honest silence
        app.on_number_key(4);
        assert!(click_rx.try_recv().is_err(), "the repeat sends nothing");

        // the out-of-domain drop (the gate: 0..8, the vanilla
        // Inventory.selected's own range)
        app.on_number_key(9);
        assert_eq!(app.selected_hotbar, 4, "the gate held");
        assert!(click_rx.try_recv().is_err(), "no command either");

        // the chest closes back to the world (the container-less
        // update) — the session LIVES: the key still echoes (the
        // vanilla client's selection is the session's own, not the
        // chest screen's)
        app.live_view_updated(air_net_world(2, 1), None, None, None);
        assert_eq!(app.screen, ScreenKind::World, "the live chest closed");
        app.on_number_key(2);
        assert_eq!(app.selected_hotbar, 2, "the world-screen key moved too");
        match click_rx.try_recv().expect("the world-screen key echoes") {
            ContainerCommand::SelectHotbar { selection } => assert_eq!(selection, 2),
            other => panic!("expected a selection, got {other:?}"),
        }

        // the OFFLINE stand-in: a fresh app in the procedural world —
        // no session, no lane: the selection still moves (the HUD's own
        // lane), the wire stays silent (the live channel above never
        // hears from it)
        let mut offline = App::new(3);
        offline.enter_world();
        offline.on_number_key(2);
        assert_eq!(offline.selected_hotbar, 2, "the offline HUD moves");
        assert!(click_rx.try_recv().is_err(), "the offline wire is silent");
    }

    #[test]
    fn escape_closes_the_chest_back_to_the_world() {
        // the LIVE chest: Escape is the player's own close — back to the
        // world (the serverbound echo rides the send lane, pinned by the
        // escape test above; the flag drops so the next container-less
        // update is a no-op)
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(None, Skybox::Overworld, None, None);
        let chest = crate::world_view::LiveContainer {
            id: 2,
            menu_type_id: crate::world_model::MENU_TYPE_CHEST,
            slots: vec![(926, 1)],
            carried: (0, 0),
            state_id: 3,
        };
        app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
        assert_eq!(app.screen, ScreenKind::Chest);
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::World, "the container closed");
        assert!(!app.chest_is_live);
        // the following container-less update does not re-open anything
        app.live_view_updated(air_net_world(2, 2), None, None, None);
        assert_eq!(app.screen, ScreenKind::World);

        // the OFFLINE chest closes the same way (the deterministic scene
        // is a screen too — Escape is its close, the world is under it)
        app.open_chest();
        assert_eq!(app.screen, ScreenKind::Chest);
        app.on_escape();
        assert_eq!(app.screen, ScreenKind::World, "the offline chest closed too");
    }

    #[test]
    fn live_session_lost_kicks_to_the_disconnect_screen() {
        // IN the network world: a session death is the vanilla kick
        let mut app = App::new(3);
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        app.enter_network_world(Some("minecraft:swamp".into()), Skybox::Overworld, None, None);
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
        assert_eq!(
            app.world_cloud_fog_end,
            None,
            "no stated cloud-fog end: the 2048.0 default's shape"
        );
        // a nether join ships Skybox::None: the sky pipeline renders no
        // sky state at all there (the extract returns before any exists)
        app.open(ScreenKind::Connect);
        app.net_world = Some(air_net_world(2, 0));
        // the nether's real 10/96 pins with the join (the fogged walls)
        app.enter_network_world(None, Skybox::None, Some((10.0, 96.0)), Some(512.0));
        assert!(app.world_is_network);
        assert_eq!(app.world_skybox, Skybox::None, "the nether pin");
        assert_eq!(app.world_fog, Some((10.0, 96.0)), "the stated pair pins");
        assert_eq!(
            app.world_cloud_fog_end,
            Some(512.0),
            "the stated cloud-fog end pins with the join"
        );
        // a procedural re-entry re-pins the overworld sky AND drops the
        // fog pair (the extract re-runs on every level entry — a network
        // nether never haunts the singleplayer world's sky or fog)
        app.enter_world();
        assert!(!app.world_is_network);
        assert_eq!(app.world_skybox, Skybox::Overworld, "the procedural re-pin");
        assert_eq!(app.world_fog, None, "the pair drops with the level");
        assert_eq!(
            app.world_cloud_fog_end,
            None,
            "the cloud-fog end drops with the level"
        );
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
                cloud_fog_end: None,
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
                cloud_fog_end: None,
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

    /// THE FIRST-PERSON BODY: the offline spawn, the look curve through
    /// the App's own feed, the walk tick on the procedural voxels, and
    /// the pause freeze (the vanilla pause's own law).
    #[test]
    fn the_body_spawns_walks_and_freezes_with_the_pause() {
        let mut app = App::new(3);
        app.enter_world();
        assert!(app.body.is_none(), "the headless discipline: no body until fed");
        app.spawn_offline_body();
        let body = app.body.as_ref().expect("the offline body spawned");
        // the feet ON the procedural surface at the spawn column's
        // center, facing the flight camera's east
        let n = app.world_size as f64;
        assert_eq!(body.x, n / 2.0);
        assert_eq!(body.z, n / 2.0);
        assert!(
            body.y <= app.spawn_y as f64,
            "the feet at or below the recorded spawn height (the lake law)"
        );
        // THE LOOK: the App feed turns the body through the curve
        let yaw_before = app.body.as_ref().unwrap().yaw;
        app.on_look_delta(10.0, 0.0);
        let turned = app.body.as_ref().unwrap().yaw - yaw_before;
        assert!(
            (turned - 1.5).abs() < 1e-4,
            "10 px at the default sensitivity = 1.5 degrees, got {}",
            turned
        );
        // THE WALK: the forward key feeds, the update ticks the travel,
        // the camera moves south-ish (yaw east -> the forward is +X)
        app.on_movement_input(1.0, 0.0, false, false, false);
        let x_before = app.body.as_ref().unwrap().x;
        for _ in 0..12 {
            app.update();
        }
        let b = app.body.as_ref().unwrap();
        assert!(b.x > x_before, "the east-facing walk moved +X ({x_before} -> {})", b.x);
        assert!(b.on_ground, "the walk stayed on the terrain");
        // THE PAUSE: the freeze (no tick, no move)
        app.paused = true;
        let (x, y, z) = (b.x, b.y, b.z);
        app.on_movement_input(1.0, 0.0, true, false, false);
        for _ in 0..10 {
            app.update();
        }
        let b = app.body.as_ref().unwrap();
        assert_eq!(b.x, x, "the paused body never moves (x)");
        assert_eq!(b.y, y, "the paused body never moves (y)");
        assert_eq!(b.z, z, "the paused body never moves (z)");
    }

    /// THE NETWORKED BODY: the join's teleport pose materializes the
    /// body; the plain refreshes (the same pose re-shipped) never
    /// re-anchor it; a CHANGED pose (the server's correction) does.
    #[test]
    fn the_body_reanchors_on_the_teleport_and_holds_the_prediction() {
        let mut app = App::new(3);
        app.net_world = Some(crate::world::World {
            chunks_per_side: 2,
            seed: 0,
            chunks: vec![
                vec![crate::world::AIR; crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z];
                4
            ],
            light: None,
            day_tick: None,
            biomes: None,
            variants: None,
            ambient_light: None,
            block_light_tint: None,
            sky_light_factor: None,
            sky_light_color: None,
            night_vision_color: None,
            night_vision_factor: 0.0,
        });
        app.net_world.as_mut().unwrap().set(8, 0, 8, crate::world::STONE);
        app.enter_network_world(None, Skybox::Overworld, None, None);
        assert!(app.body.is_none(), "no body until the first pose ships");
        let cam = crate::camera::Camera {
            pos: Vec3::new(8.5, 1.0 + crate::world_view::PLAYER_EYE_HEIGHT, 8.5),
            yaw: 0.0,
            pitch: 0.0,
        };
        app.live_view_updated(
            app.net_world.clone().unwrap(),
            Some(cam),
            None,
            None,
        );
        let b = app.body.as_ref().expect("the teleport pose materialized the body");
        assert_eq!(b.x, 8.5);
        assert!(
            (b.y - 1.0).abs() < 1e-4,
            "the feet anchor below the 1.62 eye (the f32 camera cast)"
        );
        assert_eq!(b.z, 8.5);
        // the prediction: walk forward (the camera yaw 0 = the vanilla
        // -180 = facing -Z); the same-pose refresh holds it
        app.on_movement_input(1.0, 0.0, false, false, false);
        for _ in 0..10 {
            app.update();
        }
        let walked_z = app.body.as_ref().unwrap().z;
        assert!(
            walked_z < 8.5,
            "the body walked its own prediction (z {})",
            walked_z
        );
        app.live_view_updated(
            app.net_world.clone().unwrap(),
            Some(cam),
            None,
            None,
        );
        assert_eq!(
            app.body.as_ref().unwrap().z,
            walked_z,
            "the plain refresh never re-anchors (the server does not echo plain moves)"
        );
        // the CHANGED pose: the server's correction lands
        let corrected = crate::camera::Camera {
            pos: Vec3::new(20.5, 2.0 + crate::world_view::PLAYER_EYE_HEIGHT, 20.5),
            yaw: 1.0,
            pitch: 0.0,
        };
        app.live_view_updated(
            app.net_world.clone().unwrap(),
            Some(corrected),
            None,
            None,
        );
        let b = app.body.as_ref().unwrap();
        assert_eq!(b.x, 20.5, "the correction re-baselines the body");
        assert!(
            (b.y - 2.0).abs() < 1e-4,
            "the corrected feet anchor (the f32 cast): {}",
            b.y
        );
    }

    /// THE FOOD TICK'S REGEN ARC (the stand-in server's own FoodData):
    /// a hurt body on a FULL bar + saturation heals through the fast
    /// lane's 10-tick beat, pays the exhaustion, and the fed body's
    /// hurt-clock lane (the else tail) resets the timer.
    #[test]
    fn the_food_tick_heals_the_hurt_body_off_the_saturation() {
        let mut app = App::new(3);
        app.enter_world();
        // the hurt body on the full bar: the fast lane's own setup
        app.health = 14.0;
        app.food_data.set_saturation(6.0);
        for _ in 0..9 {
            app.update();
            assert_eq!(
                app.health, 14.0,
                "the beat walks before the 10th tick speaks"
            );
        }
        app.update();
        assert_eq!(app.health, 15.0, "min(6, 6) / 6 healed the body");
        assert_eq!(
            app.food_data.exhaustion(),
            6.0,
            "the heal's own 6.0 exhaustion cost"
        );
        // the exhaustion over the fold's threshold: the fold drains the
        // saturation 1.0 at the next tick's head (the banked 6.0 > 4.0)
        app.update();
        assert_eq!(app.food_data.saturation(), 5.0, "the fold's own drain");
        assert_eq!(app.food_data.exhaustion(), 2.0, "the 4.0 banked");
        // the second beat (ticks 12..20): the SHARED timer continues —
        // the heal rides the DRAINED saturation (min(5, 6) / 6)
        for _ in 0..8 {
            app.update();
        }
        app.update();
        assert!(
            (app.health - (15.0 + 5.0 / 6.0)).abs() < 1e-4,
            "the second beat's 5/6 heal: {}",
            app.health
        );
        assert_eq!(app.food_data.exhaustion(), 7.0, "the 2.0 bank + the 5.0 cost");
    }

    /// THE FOOD TICK'S STARVE ARC: an EMPTY bar's 80-tick beat bites
    /// 1.0 through the hurt pipeline (the cooldown chain, the flash,
    /// the voice), the NORMAL difficulty stops at the last heart.
    #[test]
    fn the_food_tick_starves_the_empty_bar_on_its_own_beat() {
        let mut app = App::new(3);
        app.enter_world();
        // the starving setup: the empty bar, the empty saturation
        app.health = 20.0;
        app.food_data.set_food_level(0);
        app.food_data.set_saturation(0.0);
        // the exhaustion fold can't drop the bar further; the starve
        // lane's timer walks 80 ticks
        for _ in 0..79 {
            app.update();
            assert!(!app.dead);
        }
        app.update();
        assert_eq!(app.health, 19.0, "the starve bite's own 1.0");
        // the clocks armed THIS tick (the decrement rides the next
        // update's head — the food tick runs after the clock lane)
        assert_eq!(app.hurt_time, 10, "the flash armed");
        assert_eq!(app.invulnerable_time, 20, "the cooldown armed");
        let hurt_entry = app
            .sounds
            .events
            .iter()
            .filter(|e| e.contains("hurt"))
            .count();
        assert!(hurt_entry >= 1, "the hurt voice spoke");
        // the cooldown window: the next bite 80 ticks later lands only
        // when the window's own 20 folded — the beats' honest cadence
        for _ in 0..80 {
            app.update();
        }
        assert_eq!(app.health, 18.0, "the second bite");
    }

    /// THE MINE'S OWN COST: every break's reward lane banks the 0.005
    /// the Block.playerDestroy tail speaks — the dig's completion folds
    /// into the exhaustion bank.
    #[test]
    fn the_dig_banks_the_mine_exhaustion() {
        let mut app = App::new(3);
        app.enter_world();
        let size = app.offline_world.as_ref().unwrap().size_blocks();
        let c = size / 2;
        let surface = app.spawn_y - 1;
        app.offline_camera_updated(crate::camera::Camera {
            pos: crate::math::Vec3::new(c as f32 + 0.5, app.spawn_y as f32 + 4.0, c as f32 + 0.5),
            yaw: 0.0,
            pitch: -std::f32::consts::FRAC_PI_2,
        });
        let banked_before = app.food_data.exhaustion();
        app.on_press(0, 0);
        let mut ticks = 0;
        while app.offline_world.as_ref().unwrap().get(c, surface, c) != crate::world::AIR {
            app.update();
            ticks += 1;
            assert!(ticks < 400, "the dig completed in time");
        }
        app.on_left_release();
        assert!(
            app.food_data.exhaustion() > banked_before,
            "the break banked the mine's own 0.005"
        );
    }

    /// THE SPRINT'S FOOD GATE (the hasEnoughFoodToDoExhaustiveManoeuvres
    /// law): the bar over 6 sprints, the starving bar walks.
    #[test]
    fn the_sprint_gate_refuses_the_starving_body() {
        let mut app = App::new(3);
        app.enter_world();
        app.spawn_offline_body();
        app.on_movement_input(1.0, 0.0, false, false, true);
        assert!(
            app.body.as_ref().unwrap().sprinting,
            "the fed body sprints"
        );
        // the bar drops under the gate (food 6 — `> 6` refuses)
        app.food_data.set_food_level(6);
        app.on_movement_input(1.0, 0.0, false, false, true);
        assert!(
            !app.body.as_ref().unwrap().sprinting,
            "the starving body walks"
        );
    }

    /// THE FOOD ROW'S SHIMMY GATE (javap Hud.extractFood): the
    /// saturation at zero shakes on the `tickCount % (food * 3 + 1)`
    /// beat — the saturation fed keeps the row still. The sprites'
    /// own half/full law rides the row's walk.
    #[test]
    fn the_food_row_gate_speaks_the_shimmy_beat() {
        let mut app = App::new(3);
        app.enter_world();
        // the beat's own arithmetic: the full bar shakes at the 61st
        // (and every 61st), the 7-bar at the 22nd, the empty bar at
        // every tick (the modulus 1)
        for (food, frame, fires) in [
            (20u32, 61u64, true),
            (20, 100, false),
            (7, 22, true),
            (0, 1, true),
        ] {
            let beat = food as u64 * 3 + 1;
            assert_eq!(frame % beat == 0, fires);
        }
        // the default vitals (saturation 5.0) keep the goldens' row
        // still — the shake's own gate never fires
        assert!(
            app.food_data.saturation() > 0.0,
            "the spawn saturation shakes nothing"
        );
        // the drained saturation arms the gate
        app.food_data.set_saturation(0.0);
        app.food_data.set_food_level(20);
        app.frame = 61;
        let fires = app.frame % (app.food_data.food_level() as u64 * 3 + 1) == 0;
        assert!(fires, "the 61st tick shakes the full bar's row");
    }

    /// THE RESPAWN'S FOOD RESET: the fresh FoodData the constructor
    /// speaks — the full bar, the 5.0 saturation, the zero bank, the
    /// zero timer.
    #[test]
    fn the_respawn_resets_the_food_lane() {
        let mut app = App::new(3);
        app.enter_world();
        app.food_data.set_food_level(4);
        app.food_data.set_saturation(0.0);
        app.food_data.add_exhaustion(10.0);
        app.respawn();
        assert_eq!(app.food_data.food_level(), 20);
        assert_eq!(app.food_data.saturation(), 5.0);
        assert_eq!(app.food_data.exhaustion(), 0.0);
        assert!(app.food_data.has_enough_food(), "the fresh bar sprints");
    }

    /// THE EAT ARC — the full 32-tick hold over an apple: the six chew
    /// bursts (5 pieces + a voice each), the completion's 16-piece
    /// finisher, the gulp, the FoodData.eat fold (the ABSOLUTE 2.4
    /// saturation), THE BURP, and the survival's consume. The item
    /// pieces spawned and ticked (the BreakingItemParticle lane).
    #[test]
    fn the_eat_arc_feeds_the_hungry_body() {
        let mut app = App::new(3);
        app.enter_world();
        let apple_id = mc_registry::item_id_by_name("apple").unwrap();
        app.inventory.items[app.selected_hotbar] = (apple_id, 3);
        app.hotbar_slots = app.inventory.hotbar();
        // the hungry body: the bar under 20 (the canEat's own gate)
        app.food_data.set_food_level(10);
        app.food_data.set_saturation(0.0);
        let pieces_before = app.pieces.len();
        // the press opens the hold
        assert!(app.on_right_press(0, 0), "the hungry body starts the eat");
        let eat = app.eating.as_ref().expect("the hold opened");
        assert_eq!(eat.total, 32, "the apple's 1.6s");
        assert_eq!(eat.item, apple_id);
        // walk the 32 ticks: the bursts at the cadence's own stride,
        // the completion at the 32nd
        for _ in 0..32 {
            app.update();
        }
        assert!(app.eating.is_none(), "the hold completed");
        // THE FOOD DATA ATE: the bar 10 + 4, the saturation 0 + 2.4
        // (the 26.2 record's ABSOLUTE field)
        assert_eq!(app.food_data.food_level(), 14);
        assert!((app.food_data.saturation() - 2.4).abs() < 1e-4);
        // THE SURVIVAL CONSUME: the stack 3 -> 2
        assert_eq!(app.inventory.items[app.selected_hotbar], (apple_id, 2));
        // THE VOICES: six chews + the finisher + the gulp (all
        // entity.generic.eat) + the burp
        let eat_events = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.generic.eat "))
            .count();
        assert_eq!(eat_events, 8, "6 chews + the finisher + the gulp: {}", app.sounds.events.iter().filter(|e| e.contains("eat")).count());
        let burp_events = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.player.burp "))
            .count();
        assert_eq!(burp_events, 1, "the burp's own voice");
        // THE PIECES SPAWNED: 6 chews x 5 + the finisher's 16 = 46 over
        // the arc; some already expired (the lifetime's own law) — the
        // spawner's own floor: the earliest pieces at least RAN
        assert!(
            app.pieces.len() + pieces_before <= 46,
            "the spawner spoke its 46"
        );
    }

    /// THE FULL BAR'S REFUSAL: the canEat gate — the plain apple on a
    /// full bar refuses (the press returns false, the lane stays
    /// shut); the golden apple's canAlwaysEat speaks anyway.
    #[test]
    fn the_can_eat_gate_refuses_the_full_bar() {
        let mut app = App::new(3);
        app.enter_world();
        let apple_id = mc_registry::item_id_by_name("apple").unwrap();
        let gapple_id = mc_registry::item_id_by_name("golden_apple").unwrap();
        // the FULL bar: the plain apple refused
        app.inventory.items[app.selected_hotbar] = (apple_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(!app.on_right_press(0, 0), "the full bar refuses the apple");
        assert!(app.eating.is_none(), "the lane stayed shut");
        // the golden apple's canAlwaysEat: the full bar eats it
        app.inventory.items[app.selected_hotbar] = (gapple_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press(0, 0), "the golden apple always");
        let eat = app.eating.as_ref().expect("the hold opened");
        assert_eq!(eat.total, 32);
        app.on_right_release();
        assert!(app.eating.is_none(), "the release cancelled");
    }

    /// THE GOLDEN APPLE'S STATUS EFFECTS LAND (javap
    /// `ApplyStatusEffectsConsumeEffect.apply` through the eat arc's
    /// payload lane): the completion applies the Regeneration II
    /// (100t) + the Absorption (2400t) — the rows verbatim — and the
    /// absorption's `onEffectStarted` lifts the value to 8.0.
    #[test]
    fn the_golden_apple_lands_its_effects() {
        let mut app = App::new(3);
        app.enter_world();
        let gapple_id = mc_registry::item_id_by_name("golden_apple").unwrap();
        app.inventory.items[app.selected_hotbar] = (gapple_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        app.food_data.set_food_level(20); // the canAlwaysEat lane
        assert!(app.on_right_press(0, 0), "the golden apple always eats");
        for _ in 0..32 {
            app.update();
        }
        assert!(app.eating.is_none(), "the hold completed");
        // the store speaks both effects at the payload's own numbers —
        // the durations already walked their FIRST tick (the eat's
        // completion lands mid-update, the body tick's tick_offline
        // runs the same update: the beat fired at duration 100, the
        // tick-down followed)
        let regen = app.effects.get(crate::effects::MOB_EFFECT_REGENERATION).expect("the regen rides");
        assert_eq!(regen.amplifier, 1, "Regeneration II");
        assert_eq!(regen.duration, 99);
        let absorption = app.effects.get(crate::effects::MOB_EFFECT_ABSORPTION).expect("the absorption rides");
        assert_eq!(absorption.amplifier, 0);
        assert_eq!(absorption.duration, 2399);
        // the onEffectStarted's lift: 4 * (1 + 0) = 4.0 — two white
        // hearts the vanilla golden apple speaks
        assert!((app.effects.absorption() - 4.0).abs() < 1e-5);
    }

    /// THE ABSORPTION'S TWO DRAINS (javap `actuallyHurt`'s 26.2
    /// verdict): a hit inside the absorption never reaches the health
    /// (the fully-absorbed return); a hit beyond it takes the
    /// remainder off the health AND drains the absorption AGAIN — the
    /// second `setAbsorptionAmount` the bytecode speaks.
    #[test]
    fn the_absorption_drains_and_the_health_lives() {
        let mut app = App::new(3);
        app.enter_world();
        app.effects.add_effect(
            crate::effects::MOB_EFFECT_ABSORPTION,
            1,
            2400,
            crate::effects::DEFAULT_EFFECT_FLAGS,
        );
        assert!((app.effects.absorption() - 8.0).abs() < 1e-5);
        // the small hit: fully absorbed, the health untouched
        app.hurt(4.0);
        assert_eq!(app.health, MAX_HEALTH, "the white hearts took it");
        assert!((app.effects.absorption() - 4.0).abs() < 1e-5);
        // the cooldown's own law rides (the 10-tick window would only
        // land the difference — the test resets the pipeline for the
        // clean second hit)
        app.invulnerable_time = 0;
        app.last_hurt = 0.0;
        // the big hit: 10 total against 4 absorption — the health share
        // is 6, the drain one takes the 4, the drain two takes the 6
        // again (the absorption floors at the clamp's own zero)
        app.hurt(10.0);
        assert!((app.health - (MAX_HEALTH - 6.0)).abs() < 1e-5, "the remainder hit the health");
        assert_eq!(app.effects.absorption(), 0.0, "the double drain's floor");
    }

    /// THE REGENERATION'S BEAT HEALS (the offline stand-in server's
    /// `tickServer` arc): the golden apple's 100t Regeneration II
    /// heals 1.0 at the 25-tick period's four beats — the FoodData's
    /// own lanes silent (the bar under the regen gates).
    #[test]
    fn the_regen_beat_heals_through_the_world_tick() {
        let mut app = App::new(3);
        app.enter_world();
        let gapple_id = mc_registry::item_id_by_name("golden_apple").unwrap();
        app.inventory.items[app.selected_hotbar] = (gapple_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        // the damaged body, the bar under the food regen's gates
        app.food_data.set_food_level(14);
        app.food_data.set_saturation(0.0);
        app.health = 14.0;
        assert!(app.on_right_press(0, 0));
        for _ in 0..32 {
            app.update();
        }
        // the eat landed: the FIRST regen beat fired inside the
        // completion tick itself (the store's duration 100 beat, then
        // the tick-down) — the body climbed to 15.0
        let after_eat = app.health;
        assert_eq!(after_eat, 15.0, "the first beat healed in the walk");
        // the regen's remaining three beats over the 100-tick span
        for _ in 0..100 {
            app.update();
        }
        // the beats at durations 75/50/25 healed +3.0 (the food's slow
        // lane may add its own 80-tick beat — the ceiling the clamp
        // holds); the claim: the regen's beats RAN
        assert!(
            app.health >= after_eat + 3.0,
            "the regen's beats healed: {after_eat} -> {}",
            app.health
        );
        assert!(
            app.health <= MAX_HEALTH,
            "the Entity.heal clamp held"
        );
    }

    /// THE WIRE'S EFFECTS MIRROR (the networked lane): the Update Mob
    /// Effect packets' filtered truth lands through the vitals view —
    /// the store's add/update laws ride, the absorption's lift rides,
    /// the icons' data speaks.
    #[test]
    fn the_wire_effects_mirror_lands() {
        let mut app = App::new(3);
        app.enter_world();
        app.live_health_updated(crate::world_view::HealthView {
            health: 20.0,
            food: 20,
            saturation: 5.0,
            hurt_time: 0,
            hurt_yaw: 0.0,
            death_message: None,
            effects: vec![
                crate::world_model::MobEffectWireState {
                    effect_id: crate::effects::MOB_EFFECT_ABSORPTION,
                    amplifier: 0,
                    duration: 2400,
                    flags: crate::effects::DEFAULT_EFFECT_FLAGS,
                },
            ],
        });
        assert!(app.effects.has_effect(crate::effects::MOB_EFFECT_ABSORPTION));
        assert!((app.effects.absorption() - 4.0).abs() < 1e-5);
        // the update re-speaks: the amplifier's upgrade rides
        app.live_health_updated(crate::world_view::HealthView {
            health: 20.0,
            food: 20,
            saturation: 5.0,
            hurt_time: 0,
            hurt_yaw: 0.0,
            death_message: None,
            effects: vec![
                crate::world_model::MobEffectWireState {
                    effect_id: crate::effects::MOB_EFFECT_ABSORPTION,
                    amplifier: 2,
                    duration: 2400,
                    flags: crate::effects::DEFAULT_EFFECT_FLAGS,
                },
            ],
        });
        assert_eq!(
            app.effects.get(crate::effects::MOB_EFFECT_ABSORPTION).unwrap().amplifier,
            2,
            "the wire's upgrade landed"
        );
        assert!((app.effects.absorption() - 12.0).abs() < 1e-5);
        // the silence removes (the Remove packet's own verdict)
        app.live_health_updated(crate::world_view::HealthView {
            health: 20.0,
            food: 20,
            saturation: 5.0,
            hurt_time: 0,
            hurt_yaw: 0.0,
            death_message: None,
            effects: Vec::new(),
        });
        assert!(app.effects.is_empty());
    }

    /// THE RESPAWN CLEARS THE STORE (the death's removal — the fresh
    /// body carries no effects, the absorption rides the dead body's
    /// history).
    #[test]
    fn the_respawn_clears_the_effects() {
        let mut app = App::new(3);
        app.enter_world();
        app.effects.add_effect(
            crate::effects::MOB_EFFECT_REGENERATION,
            0,
            100,
            crate::effects::DEFAULT_EFFECT_FLAGS,
        );
        app.effects.set_absorption(8.0);
        app.dead = true;
        app.respawn();
        assert!(app.effects.is_empty(), "the death's removal");
        assert_eq!(app.effects.absorption(), 0.0);
    }

    /// THE MILK CLEARS THE STATUS EFFECTS (the ClearAllStatusEffects
    /// payload's own lane — the FOOD-LESS consumable's arc: no roll,
    /// no listener, the store empties and the bucket returns).
    #[test]
    fn the_milk_clears_the_status_effects() {
        let mut app = App::new(3);
        app.enter_world();
        let milk_id = mc_registry::item_id_by_name("milk_bucket").unwrap();
        app.effects.add_effect(
            crate::effects::MOB_EFFECT_REGENERATION,
            1,
            100,
            crate::effects::DEFAULT_EFFECT_FLAGS,
        );
        app.effects.set_absorption(8.0);
        app.inventory.items[app.selected_hotbar] = (milk_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        assert!(app.on_right_press(0, 0), "the milk drinks");
        for _ in 0..32 {
            app.update();
        }
        assert!(app.eating.is_none(), "the drink completed");
        assert!(app.effects.is_empty(), "the ClearAll's own verdict");
        // the 26.2's own honesty: the removal hook's absence — the
        // absorption VALUE persists until damage drains it (the
        // AbsorptionMobEffect carries no removeAttributeModifiers)
        assert_eq!(app.effects.absorption(), 8.0, "the value persists");
    }

    /// THE MILK DRINK: no FOOD component (the potion's own split) —
    /// the full bar consumes unconditionally, the completion folds NO
    /// FoodData (no listener, no burp), and the USE_REMAINDER returns
    /// the bucket.
    #[test]
    fn the_milk_drink_returns_the_bucket() {
        let mut app = App::new(3);
        app.enter_world();
        let milk_id = mc_registry::item_id_by_name("milk_bucket").unwrap();
        let bucket_id = mc_registry::item_id_by_name("bucket").unwrap();
        app.inventory.items[app.selected_hotbar] = (milk_id, 2);
        app.hotbar_slots = app.inventory.hotbar();
        app.food_data.set_food_level(20); // the FULL bar — milk doesn't care
        assert!(app.on_right_press(0, 0), "the consumable without food consumes");
        for _ in 0..32 {
            app.update();
        }
        assert!(app.eating.is_none(), "the drink completed");
        // the FoodData untouched (no FOOD listener)
        assert_eq!(app.food_data.food_level(), 20);
        // the stack 2 -> 1 and the BUCKET returned to the slot's place
        // (the add lane's first-empty walks the selected first — the
        // 2-stack left the slot occupied, the bucket rides the next
        // free slot)
        assert_eq!(app.inventory.items[app.selected_hotbar].0, milk_id);
        assert_eq!(app.inventory.items[app.selected_hotbar].1, 1);
        let bucket_slots = app
            .inventory
            .items
            .iter()
            .filter(|s| s.0 == bucket_id && s.1 >= 1)
            .count();
        assert_eq!(bucket_slots, 1, "the remainder's own return");
        // no burp without the FOOD listener
        let burp_events = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.player.burp "))
            .count();
        assert_eq!(burp_events, 0, "the potion's split: no listener, no burp");
    }

    /// THE CANCELLED HOLD: the release mid-hold ends the eat silently
    /// — no completion, no voices, no consume, the stack untouched.
    #[test]
    fn the_release_cancels_the_hold_silently() {
        let mut app = App::new(3);
        app.enter_world();
        let apple_id = mc_registry::item_id_by_name("apple").unwrap();
        app.inventory.items[app.selected_hotbar] = (apple_id, 2);
        app.hotbar_slots = app.inventory.hotbar();
        app.food_data.set_food_level(10);
        assert!(app.on_right_press(0, 0));
        for _ in 0..10 {
            app.update();
        }
        let pieces_mid = app.pieces.len();
        assert!(pieces_mid > 0, "the chews already spoke their pieces");
        app.on_right_release();
        assert!(app.eating.is_none(), "the release cancelled");
        // the walk continues: no completion fires (the lane's gone)
        for _ in 0..32 {
            app.update();
        }
        assert_eq!(app.food_data.food_level(), 10, "the FoodData never ate");
        assert_eq!(app.inventory.items[app.selected_hotbar], (apple_id, 2), "the stack untouched");
        let eat_events = app
            .sounds
            .events
            .iter()
            .filter(|e| e.starts_with("entity.generic.eat "))
            .count();
        assert!(eat_events <= 2, "the cancelled hold's chews only: {eat_events}");
    }

    /// THE PAUSE FREEZES THE HOLD: the vanilla pause's own law — the
    /// eat clock sleeps, no burst, no completion while paused.
    #[test]
    fn the_pause_freezes_the_eat_clock() {
        let mut app = App::new(3);
        app.enter_world();
        let apple_id = mc_registry::item_id_by_name("apple").unwrap();
        app.inventory.items[app.selected_hotbar] = (apple_id, 1);
        app.hotbar_slots = app.inventory.hotbar();
        app.food_data.set_food_level(10);
        assert!(app.on_right_press(0, 0));
        app.paused = true;
        for _ in 0..40 {
            app.update();
        }
        assert!(app.eating.is_some(), "the hold survived the pause");
        assert_eq!(app.eating.as_ref().unwrap().ticks, 0, "the clock slept");
        app.paused = false;
        for _ in 0..32 {
            app.update();
        }
        assert!(app.eating.is_none(), "the hold resumed and completed");
    }
}

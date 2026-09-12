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
            },
            ServerEntry {
                name: "hypixel.net".to_string(),
                motd: "§bHypixel Network §7- §b1.8-1.21".to_string(),
                ping_ms: 34,
                online: true,
                version: "Requires MC 1.8/1.21".to_string(),
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
            // in-game: the menu music scheduler is suspended
            ScreenKind::World => {}
            _ => self.music.update(),
        }
    }

    pub fn open(&mut self, kind: ScreenKind) {
        self.screen = kind;
        self.buttons.clear();
    }

    /// Enter the singleplayer world (3D view + HUD), building the mesh once.
    pub fn enter_world(&mut self) {
        if self.world_quads.is_none() {
            let world = crate::world::World::new(WORLD_CHUNKS, self.seed);
            self.world_size = world.size_blocks();
            self.spawn_y = top_solid_y(&world, self.world_size / 2, self.world_size / 2);
            self.world_quads = Some(crate::mesher::mesh_world(&world, None));
        }
        self.paused = false;
        self.open(ScreenKind::World);
    }

    /// Escape: toggles the pause menu while in game (vanilla behavior);
    /// no-op on other screens.
    pub fn on_escape(&mut self) {
        if self.screen == ScreenKind::World {
            self.paused = !self.paused;
            self.buttons.clear();
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
                    self.open(ScreenKind::Title);
                    self.music.on_menu_open();
                }
                _ => {}
            }
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
        let eye = Vec3::new(n / 2.0, self.spawn_y as f32 + 1.62, n / 2.0);
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
        // Official render order: sky color -> sun/moon/stars -> terrain.
        fb.clear(sky.sky_color, sky.fog_color);
        crate::sky::render_celestials(fb, &ctx, &cam, day_time);
        if let Some(quads) = &self.world_quads {
            fb.draw_quads(&ctx, quads);
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
}

/// Deterministic splitmix64 (same primitive as the world seed).
fn splitmix(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
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

    impl App {
        fn render_title_buttons_for_test(&mut self) {
            let mut fb = crate::raster::Framebuf::new(320, 180);
            let mut assets = GuiAssets::open("assets-cache/jar");
            self.render(&mut fb, &mut assets);
        }
    }
}

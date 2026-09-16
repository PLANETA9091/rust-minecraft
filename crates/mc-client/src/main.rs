//! mc-client — Minecraft (Rust) reference client.
//!
//! Modes:
//!   default         windowed fly-camera (falls back to headless w/o display)
//!   --headless      deterministic offscreen render -> PNG (pixel CI)
//!   --window        force windowed mode
//!   --gui           windowed GUI app (boot -> title -> menus), generated data
//!   --screen S      headless GUI screenshot: loading|title|servers|options|world|worldnight|pause|chest

mod avatar;
mod audio;
mod body;
mod build_info;
mod camera;
mod colors;
mod colormap;
mod dig;
mod drops;
mod eat;
mod effects;
mod end_flash;
mod mob;
mod particles;
mod food;
mod gpu;
mod gui;
mod hand;
mod headless;
mod math;
mod mesher;
mod net_fixture;
mod music;
mod png_io;
mod raster;
mod screens;
mod sky;
mod sound;
mod textures;
mod timeline;
mod tint;
mod world;
mod world_model;
mod world_view;
mod xp;

#[cfg(not(target_arch = "wasm32"))]
mod window;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "mc-client", version, about = "Minecraft client (Rust reference renderer)")]
struct Cli {
    /// Headless deterministic render (saves PNG, exits).
    #[arg(long)]
    headless: bool,
    /// Force windowed mode.
    #[arg(long)]
    window: bool,
    /// Windowed GUI application (boot chain + menus from generated data).
    #[arg(long)]
    gui: bool,
    /// Headless GUI screen render: loading|title|servers|options|world|pause|chest.
    #[arg(long)]
    screen: Option<String>,
    /// UI frame to capture (drives splash pulse / boot progress).
    #[arg(long, default_value_t = 60)]
    ui_frame: u64,
    #[arg(long, default_value_t = 640)]
    width: usize,
    #[arg(long, default_value_t = 360)]
    height: usize,
    #[arg(long, default_value_t = 1)]
    frames: u32,
    #[arg(long, default_value_t = 20260214)]
    seed: u64,
    /// World size in chunks per side.
    #[arg(long, default_value_t = 5)]
    chunks: usize,
    /// Render the NETWORK world fixture (real packet bytes -> parse ->
    /// model -> materialize, light layer active) instead of the
    /// procedural scene — the golden path for the wire view.
    #[arg(long)]
    net_world: bool,
    /// Hold an item in the first-person hand on the GUI world screens
    /// (the model id, e.g. `item/iron_sword`) — the deterministic
    /// stand-in for inventory state; the item is drawn INSTEAD of the
    /// empty-hand arm (vanilla's submitArmWithItem branch). The default
    /// scenes hold nothing, so the pixel goldens never see this.
    #[arg(long)]
    held_item: Option<String>,
    /// The held stack's CHARGED_PROJECTILES component state (the
    /// deterministic stand-in for the loaded crossbow's own component):
    /// `arrow` or `rocket`. Requires `--held-item item/crossbow` — the
    /// resting render rides the charge_type select's case (the loaded
    /// bolt). The default scenes carry no charge, so the pixel goldens
    /// never see this.
    #[arg(long)]
    held_charge: Option<String>,
    /// Freeze the attack-swing clock at this progress (getAttackAnim's
    /// domain [0, 1], 0 = rest) on the world screens — the deterministic
    /// stand-in for the swing clock (the client has no gameplay input in
    /// the headless scenes). The default scenes never swing, so the
    /// pre-existing goldens never see a nonzero value.
    #[arg(long)]
    swing_progress: Option<f32>,
    /// Use the slim (Alex) player-skin variant for the empty-hand arm —
    /// the deterministic stand-in for the profile's texture metadata.
    /// The default (wide classic) is what the pixel goldens render.
    #[arg(long)]
    slim_arm: bool,
    // ---- THE SHOTS LANE (the shots/ gallery's own options — the
    // deterministic scene seeds the world shots speak; every flag
    // absent = the goldens' own shape, the pixel matrix untouched) ----
    /// The F5 camera state for the world shots: `first`, `third-back`
    /// or `third-front` (the CameraType cycle's own states).
    #[arg(long)]
    perspective: Option<String>,
    /// Spawn the deterministic player body at the spawn column's
    /// surface facing east (the third-person avatar's camera entity).
    #[arg(long)]
    body: bool,
    /// The body's facing override (the vanilla yaw degrees, e.g. `90`
    /// faces west) — the shots stage the encounter the facing names.
    #[arg(long)]
    body_yaw: Option<f32>,
    /// The body's look pitch override (the vanilla degrees, positive
    /// up, e.g. `-20` looks down) — the detached camera rides it.
    #[arg(long)]
    body_pitch: Option<f32>,
    /// The body's mid-stride walk seed "speed,position" (e.g.
    /// "0.9,5.0") — the WalkAnimationState's own pair the avatar's
    /// limb cosines speak.
    #[arg(long)]
    body_walk: Option<String>,
    /// The resolved crouch pose (rides the walk ticks when they run).
    #[arg(long)]
    body_crouch: bool,
    /// Spawn one deterministic zombie approaching the body (the
    /// walkable-approach scan's own cell, the mid-stride walk).
    #[arg(long)]
    zombie: bool,
    /// The zombie's remainingFireTicks (the igniteForSeconds(8) law
    /// speaks 160).
    #[arg(long)]
    zombie_fire: Option<i32>,
    /// The zombie's deathTime (0..20 — the death-flip lane the render
    /// state's deathTime+partial read speaks).
    #[arg(long)]
    zombie_death: Option<i32>,
    /// The zombie's aggressive raised-arms pose
    /// (ZombieRenderState.isAggressive).
    #[arg(long)]
    zombie_aggro: bool,
    /// The zombie's hurtTime (the OverlayTexture's 0.7 mix).
    #[arg(long)]
    zombie_hurt: Option<i32>,
    /// Game ticks with the forward impulse held (the walk state
    /// evolves through the body's own update, the body travels its
    /// facing).
    #[arg(long, default_value_t = 0)]
    walk_ticks: u32,
    /// Input-less game ticks after the walk (the AI chase, the burn
    /// roll, the idle bob live through these).
    #[arg(long, default_value_t = 0)]
    settle_ticks: u32,
    /// Showcase zombies placed ahead of the body along its facing
    /// (the spawn law's own ground scan; requires --body).
    #[arg(long, default_value_t = 0)]
    mobs: usize,
    /// Ignite every placed zombie for this many seconds (the
    /// igniteForSeconds law).
    #[arg(long, default_value_t = 0.0)]
    ignite: f32,
    /// The player body's remainingFireTicks (the flame feature's own
    /// render clock — the deterministic stand-in for the live fire).
    #[arg(long)]
    body_fire: Option<i32>,
    /// Carve a shallow lava pool ahead of the body (the 3x3 shelf at
    /// four cells out along the facing) — the walk ticks carry the
    /// body INTO it and the REAL fire laws speak (the ignite, the
    /// 4.0, the burn).
    #[arg(long)]
    lava_pool: bool,
    /// Spawn the XP orbs ahead of the body — the zombie kill's own
    /// [3, 1, 1] award walk at chest height, the flight the follow
    /// law owns (the camera-facing quads, the pulse, the icon bands).
    #[arg(long)]
    orbs: bool,
    /// Stage the item drops ahead of the body — the dig's own dirt and
    /// the kill's own rotten flesh riding the popResource physics (the
    /// mid-bounce the gravity law owns, the fold the render's lerp
    /// walks at the partial frames).
    #[arg(long)]
    drops: bool,
    /// THE WIRE'S OWN HOUR: stage the world screen as a NETWORK world
    /// carrying the server's Set Time opinion (the day tick the wire
    /// speaks) — the day clock re-bases through the live lane's own
    /// fold (live_view_updated's align_day_clock_to_wire), the sky
    /// renders the server's hour, the client's dawn never shows.
    #[arg(long)]
    net_day: Option<i64>,
    /// THE WIRE'S OWN MOBS: stage N wire zombies into the network
    /// world's mob lane (the live_mobs_updated entry the session's
    /// refresh rides — the lerpTo law walks their pose). Requires the
    /// network arm to be armed (--net-day or --net-mobs alone works).
    #[arg(long, default_value = "0")]
    net_mobs: usize,
    /// THE KILL'S OWN WORD: stage the wire mob's metadata health 0.0
    /// (the Set Entity Data accessor the death clock speaks) and walk
    /// N ticks of the flip — the frame freezes mid-fall (N=10 half
    /// over, N=19 nearly flat). Requires --net-mobs.
    #[arg(long)]
    net_death: Option<u32>,
    /// THE HIT'S OWN PACKET: stage the wire mob's Hurt Animation arm
    /// (the 10-tick flash window the mob lane's own tick counts down)
    /// and walk N ticks of the countdown — the frame freezes
    /// mid-window (the 0.7 shade the render's mix speaks). Requires
    /// --net-mobs.
    #[arg(long)]
    net_hurt: Option<u32>,
    /// THE BURNING WORD: stage the wire mob's shared-flags accessor
    /// bit 0 (Entity.isOnFire's own query the metadata carries) — the
    /// flame rings the render draws at the server's word, N ticks
    /// walked. Requires --net-mobs.
    #[arg(long)]
    net_fire: Option<u32>,
    /// THE CROUCH WORD: stage the wire mob's shared-flags accessor
    /// bit 1 (Entity.isCrouching's own query the metadata carries) —
    /// the avatar's crouch pose the render bends through, N ticks
    /// walked. Requires --net-mobs.
    #[arg(long)]
    net_crouch: Option<u32>,
    /// Carve a 3x3 pool AT the body and sink it to the basin floor —
    /// the eye sits under the water surface and the REAL water-screen
    /// gate speaks (the underwater overlay, no scripted blue).
    #[arg(long)]
    water_pool: bool,
    /// Seed the water vision clock (javap `LocalPlayer.waterVisionTime`,
    /// the 26.2 jar) — the deterministic stand-in for the soak time the
    /// live drift can't hold (the water's own push walks the body out
    /// of a staged pool). 0 = the fresh dive's densest murk (the 0.25
    /// floor's end 24.0), 600 = the fully soaked eye's open fog (end
    /// 96.0) — the getWaterVision ramp's own stations.
    #[arg(long, default_value_t = 0)]
    water_vision: i32,
    /// Seed the body's air supply (javap `Entity.getAirSupply`, the 26.2
    /// jar) — the deterministic stand-in for the mid-drain frames the
    /// live dive can't hold. The HUD's bubble row reads the seeded
    /// store (the extract law's own gate). Clamped [-20, 300].
    #[arg(long)]
    air_supply: Option<i32>,
    /// Carve a 3x3 LAVA pool AT the body and sink it to the basin
    /// floor — the eye sits IN the lava and the REAL fog + contact laws
    /// speak (the lava murk, the ignite, the 4.0).
    #[arg(long)]
    lava_dive: bool,
    /// BURY THE EYE: fill the eye's own cell with DIRT — the
    /// getViewBlockingState walk's eight samples all read the solid,
    /// the head-in-block screen arm speaks (the block particle sprite
    /// the 0xFF191919 tint carries).
    #[arg(long)]
    bury: bool,
    /// Seed the experience stat "level,progress" (javap `Player`, the
    /// 26.2 jar) — the deterministic stand-in for the mining grants.
    /// The HUD's contextual bar reads the seeded store (the extract
    /// law's own gate).
    #[arg(long)]
    xp: Option<String>,
    /// Stage a particle scene "burst|poof|crit" (the lane's own
    /// shots — the broken cell's debris, the death cloud, the crit
    /// emitter's stars). Anything else refuses loudly.
    #[arg(long)]
    particles: Option<String>,
    /// THE FIXED CLOCK's render remainder override (0.0..=1.0): the
    /// partial tick the staged frame's interpolation folds ride — the
    /// windowed loop's own value the headless default 1.0 pins (the
    /// current tick position bit for bit, the goldens' contract).
    #[arg(long)]
    partial: Option<f32>,
    #[arg(long, default_value = "render.png")]
    out: PathBuf,
    /// Path to official asset index json (enables real texture-average colors).
    #[arg(long)]
    asset_index: Option<PathBuf>,
    /// Root of downloaded assets (contains objects/).
    #[arg(long, default_value = "assets-cache")]
    assets_root: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    println!(
        "mc2rs client — target Minecraft {} (jar sha1 {}..., obfuscated: {})",
        build_info::MINECRAFT_VERSION,
        &build_info::CLIENT_SHA1[..8.min(build_info::CLIENT_SHA1.len())],
        build_info::OBFUSCATED
    );

    // ---- headless GUI screenshots (boot chain / menus) ----
    if let Some(screen) = &cli.screen {
        // THE SHOTS LANE's parse (the loud refuse — a guessed name
        // would render a guessed view/pose):
        let perspective = cli
            .perspective
            .as_deref()
            .map(crate::headless::parse_perspective)
            .transpose()?;
        let body_walk = cli
            .body_walk
            .as_deref()
            .map(|s| {
                crate::headless::parse_walk_seed(s)
                    .ok_or_else(|| anyhow::anyhow!("--body-walk must be \"speed,position\" (the WalkAnimationState pair), got {s:?}"))
            })
            .transpose()?;
        let xp = cli
            .xp
            .as_deref()
            .map(|s| {
                crate::headless::parse_xp_seed(s).ok_or_else(|| {
                    anyhow::anyhow!("--xp must be \"level,progress\" (level 0..=24791, progress 0.0..=1.0), got {s:?}")
                })
            })
            .transpose()?;
        let opts = crate::headless::SceneOpts {
            perspective,
            body: cli.body,
            body_yaw: cli.body_yaw,
            body_pitch: cli.body_pitch,
            body_walk,
            body_crouch: cli.body_crouch,
            zombie: cli.zombie,
            zombie_fire: cli.zombie_fire,
            zombie_death: cli.zombie_death,
            zombie_aggro: cli.zombie_aggro,
            zombie_hurt: cli.zombie_hurt,
            walk_ticks: cli.walk_ticks,
            settle_ticks: cli.settle_ticks,
            mobs: cli.mobs,
            ignite_secs: cli.ignite,
            body_fire: cli.body_fire,
            lava_pool: cli.lava_pool,
            orbs: cli.orbs,
            drops: cli.drops,
            net_day: cli.net_day,
            net_mobs: cli.net_mobs,
            net_death: cli.net_death,
            net_hurt: cli.net_hurt,
            net_fire: cli.net_fire,
            net_crouch: cli.net_crouch,
            water_pool: cli.water_pool,
            water_vision: cli.water_vision,
            air_supply: cli.air_supply,
            lava_dive: cli.lava_dive,
            bury: cli.bury,
            xp,
            particles: cli.particles,
            partial: cli.partial,
        };
        headless::render_gui_screen_opts(
            screen,
            cli.ui_frame,
            cli.seed,
            cli.width,
            cli.height,
            cli.held_item.as_deref(),
            cli.held_charge.as_deref(),
            cli.swing_progress,
            cli.slim_arm,
            &opts,
            &cli.out,
        )?;
        return Ok(());
    }

    let world = if cli.net_world {
        net_fixture::fixture_world(cli.chunks)
    } else {
        world::World::new(cli.chunks, cli.seed)
    };
    println!(
        "world: {}x{} chunks ({}x{} blocks), {} blocks total",
        cli.chunks,
        cli.chunks,
        world.size_blocks(),
        world.size_blocks(),
        world.size_blocks() as usize * world.size_blocks() as usize
    );

    let lut = load_lut(&cli).map(|l| {
        println!("real colors: {}", l.describe());
        l
    });

    if cli.gui && !cli.headless {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match window::run_gui_app(cli.seed, cli.width, cli.height, cli.held_item.as_deref()) {
                Ok(()) => return Ok(()),
                Err(e) => eprintln!("gui mode unavailable ({e}); falling back to headless"),
            }
        }
    }

    if cli.window || (!cli.headless && !cfg!(target_os = "linux")) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match window::run_windowed(&world, lut.as_ref(), cli.width, cli.height) {
                Ok(()) => return Ok(()),
                Err(e) => {
                    eprintln!("window mode unavailable ({e}); falling back to headless");
                }
            }
        }
    }

    let timings = headless::render_frames(&world, lut.as_ref(), cli.width, cli.height, cli.frames.max(1), &cli.out)?;
    if timings.len() > 1 {
        let avg = timings.iter().sum::<f64>() / timings.len() as f64;
        println!("avg render: {avg:.2} ms/frame ({:.0} fps)", 1000.0 / avg);
    }
    if cfg!(feature = "gpu") {
        println!("{}", gpu::gpu_path_status());
    }
    Ok(())
}

fn load_lut(cli: &Cli) -> Option<textures::TextureLut> {
    let index_path: PathBuf = match &cli.asset_index {
        Some(p) => p.clone(),
        None => {
            let p = PathBuf::from("assets-cache")
                .join("indexes")
                .join(format!("{}.json", build_info::MINECRAFT_VERSION));
            if !p.is_file() {
                return None;
            }
            p
        }
    };
    match textures::TextureLut::load(&index_path, &cli.assets_root) {
        Ok(l) => Some(l),
        Err(e) => {
            eprintln!("real colors unavailable: {e}");
            None
        }
    }
}

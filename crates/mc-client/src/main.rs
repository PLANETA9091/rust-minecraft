//! mc-client — Minecraft (Rust) reference client.
//!
//! Modes:
//!   default         windowed fly-camera (falls back to headless w/o display)
//!   --headless      deterministic offscreen render -> PNG (pixel CI)
//!   --window        force windowed mode
//!   --gui           windowed GUI app (boot -> title -> menus), generated data
//!   --screen S      headless GUI screenshot: loading|title|servers|options|world|worldnight|pause|chest

mod audio;
mod body;
mod build_info;
mod camera;
mod colors;
mod colormap;
mod dig;
mod drops;
mod eat;
mod end_flash;
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
        headless::render_gui_screen(
            screen,
            cli.ui_frame,
            cli.seed,
            cli.width,
            cli.height,
            cli.held_item.as_deref(),
            cli.held_charge.as_deref(),
            cli.swing_progress,
            cli.slim_arm,
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

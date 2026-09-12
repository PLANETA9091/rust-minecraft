//! mc-client — Minecraft (Rust) reference client.
//!
//! Modes:
//!   default         windowed fly-camera (falls back to headless w/o display)
//!   --headless      deterministic offscreen render -> PNG (pixel CI)
//!   --window        force windowed mode
//!   --gui           windowed GUI app (boot -> title -> menus), generated data
//!   --screen S      headless GUI screenshot: loading|title|servers|options|world|worldnight|pause

mod audio;
mod build_info;
mod camera;
mod colors;
mod gpu;
mod gui;
mod headless;
mod math;
mod mesher;
mod net_fixture;
mod music;
mod png_io;
mod raster;
mod screens;
mod sky;
mod textures;
mod timeline;
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
    /// Headless GUI screen render: loading|title|servers|options|world|pause.
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
        headless::render_gui_screen(screen, cli.ui_frame, cli.seed, cli.width, cli.height, &cli.out)?;
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
            match window::run_gui_app(cli.seed, cli.width, cli.height) {
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

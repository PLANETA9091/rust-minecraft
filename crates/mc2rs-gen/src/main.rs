//! mc2rs-gen — the version-agnostic Minecraft → Rust generator CLI.

mod atlas;
mod codegen;
mod jar;
mod manifest;
mod util;
mod version;

use std::path::Path;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "mc2rs-gen",
    version,
    about = "Generate a Rust Minecraft client workspace for ANY official version"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// List every version Mojang publishes.
    Manifest {
        /// Print full table (default: last 15).
        #[arg(long)]
        all: bool,
    },
    /// Show details for one version (downloads, obfuscation status, asset index).
    Info {
        #[arg(long)]
        version: String,
    },
    /// THE pipeline: download jar -> analyze -> regenerate the workspace.
    Generate {
        #[arg(long)]
        version: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
    },
    /// Extract the REAL texture atlas from the official jar into
    /// mc-registry/src/generated/atlas.rs (blockstates -> models -> PNGs).
    Atlas {
        #[arg(long)]
        version: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
    },
    /// Download official game assets into a local cache (requires owning MC).
    Assets {
        #[arg(long)]
        version: String,
        #[arg(long, default_value = "assets-cache")]
        dir: String,
    },
    /// Ping a server using the generated protocol layer (smoke test).
    Ping {
        host: String,
        #[arg(long, default_value_t = 25565)]
        port: u16,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let agent = util::agent();
    let cache = util::default_cache();

    match cli.cmd {
        Cmd::Manifest { all } => {
            let m = manifest::load(&agent)?;
            println!(
                "latest release: {} | latest snapshot: {} | total: {}",
                m.latest_release(),
                m.latest.snapshot,
                m.versions.len()
            );
            let list: Vec<_> = if all {
                m.versions.iter().collect()
            } else {
                m.versions.iter().take(15).collect()
            };
            for v in list {
                println!("{:<22} {:<9} {}", v.id, v.kind, v.release_time);
            }
        }
        Cmd::Info { version } => {
            let m = manifest::load(&agent)?;
            let entry = m
                .find(&version)
                .with_context(|| format!("version {version} not in manifest"))?
                .clone();
            let vj = version::resolve(&agent, &cache, &entry.url, entry.sha1.as_deref())?;
            println!("id:             {}", vj.id);
            println!("type:           {}", vj.kind);
            println!("releaseTime:    {}", vj.release_time);
            println!("java:           {:?}", vj.java_version.as_ref().map(|j| j.major_version));
            println!("client jar:     {} bytes, sha1 {}", vj.downloads.client.size, vj.downloads.client.sha1.as_deref().unwrap_or("?"));
            println!("client maps:    {}", vj.downloads.client_mappings.as_ref().map(|m| m.url.as_str()).unwrap_or("none (unobfuscated)"));
            if let Some(ai) = &vj.asset_index {
                println!("asset index:    {} ({})", ai.id, ai.url);
            }
            println!("libraries:      {}", vj.libraries.len());
        }
        Cmd::Generate { version, out } => {
            generate(&agent, &cache, &version, Path::new(&out))?;
        }
        Cmd::Atlas { version, out } => {
            atlas_cmd(&agent, &cache, &version, Path::new(&out))?;
        }
        Cmd::Assets { version, dir } => {
            let m = manifest::load(&agent)?;
            let entry = m
                .find(&version)
                .with_context(|| format!("version {version} not in manifest"))?
                .clone();
            let vj = version::resolve(&agent, &cache, &entry.url, entry.sha1.as_deref())?;
            let ai = vj
                .asset_index
                .context("version json has no asset index")?;
            let idx_bytes = cache.fetch(&agent, &format!("asset-index:{}", ai.sha1.as_deref().unwrap_or(&ai.id)), &ai.url)?;
            util::verify(&idx_bytes, ai.sha1.as_deref(), "asset index")?;
            let index = mc_assets::AssetIndex::parse(&idx_bytes)?;
            std::fs::create_dir_all(&dir)?;
            let t0 = std::time::Instant::now();
            mc_assets::download_all(&index, Path::new(&dir), &agent, |done, total| {
                println!("  assets {done}/{total}");
            })?;
            println!(
                "downloaded/verified {} objects ({:.1}s) -> {}",
                index.object_count(),
                t0.elapsed().as_secs_f32(),
                dir
            );
        }
        Cmd::Ping { host, port } => {
            let r = mc_protocol::ping(&host, port)?;
            println!(
                "{}:{} — {} (protocol {}) | {}/{} players | {} ms | MOTD: {}",
                host,
                port,
                r.version_name,
                r.protocol,
                r.players_online,
                r.players_max,
                r.latency_ms,
                r.motd.lines().next().unwrap_or("")
            );
        }
    }
    Ok(())
}

fn generate(agent: &ureq::Agent, cache: &util::Cache, version: &str, out: &Path) -> Result<()> {
    let t0 = std::time::Instant::now();
    let workspace_root = out
        .canonicalize()
        .with_context(|| format!("workspace root {} not found", out.display()))?;

    let m = manifest::load(agent)?;
    let entry = m
        .find(version)
        .with_context(|| {
            format!(
                "version {version} not in manifest (latest release is {})",
                m.latest_release()
            )
        })?
        .clone();
    println!("[1/5] version {version} found ({} from {})", entry.kind, entry.release_time);

    let vj = version::resolve(agent, cache, &entry.url, entry.sha1.as_deref())?;
    let dl = &vj.downloads.client;
    println!("[2/5] fetching official client jar ({:.1} MiB)...", dl.size as f64 / (1024.0 * 1024.0));
    let jar_bytes = cache.fetch(
        agent,
        &format!("client-jar:{}", dl.sha1.as_deref().unwrap_or(vj.id.as_str())),
        &dl.url,
    )?;
    util::verify(&jar_bytes, dl.sha1.as_deref(), "client jar")?;
    let client_sha1 = util::sha1_hex(&jar_bytes);

    println!("[3/5] analyzing jar (obfuscation detection, registry extraction)...");
    let report = jar::analyze(&jar_bytes)?;
    println!(
        "      obfuscated: {} (ratio {:.2}), classes: {}, net/minecraft classes: {}",
        report.obfuscated, report.obfuscation_ratio, report.class_count, report.net_minecraft_classes
    );

    println!("[4/6] generating registry + build info...");
    let report_path = codegen::emit(&workspace_root, version, &client_sha1, &report)?;

    println!("[5/6] extracting REAL texture atlas (blockstates -> models -> PNGs)...");
    write_atlas(&workspace_root, version, &client_sha1, &jar_bytes)?;

    println!("[6/6] done in {:.1}s", t0.elapsed().as_secs_f32());
    println!("      blocks: {} | items: {} | textures: {} | sounds: {}",
        report.blockstate_count, report.item_count, report.texture_count, report.sound_count);
    println!("      report: {}", report_path.display());
    println!("      next: cargo test -p mc-registry && cargo run -p mc-client --release -- --headless");
    Ok(())
}

/// Fetch the jar (cache-first), resolve the atlas, write generated/atlas.rs.
fn atlas_cmd(agent: &ureq::Agent, cache: &util::Cache, version: &str, out: &Path) -> Result<()> {
    let t0 = std::time::Instant::now();
    let workspace_root = out
        .canonicalize()
        .with_context(|| format!("workspace root {} not found", out.display()))?;
    let m = manifest::load(agent)?;
    let entry = m
        .find(version)
        .with_context(|| format!("version {version} not in manifest"))?
        .clone();
    let vj = version::resolve(agent, cache, &entry.url, entry.sha1.as_deref())?;
    let dl = &vj.downloads.client;
    let jar_bytes = cache.fetch(
        agent,
        &format!("client-jar:{}", dl.sha1.as_deref().unwrap_or(vj.id.as_str())),
        &dl.url,
    )?;
    util::verify(&jar_bytes, dl.sha1.as_deref(), "client jar")?;
    let sha = util::sha1_hex(&jar_bytes);
    write_atlas(&workspace_root, version, &sha, &jar_bytes)?;
    println!("atlas written in {:.1}s (jar {})", t0.elapsed().as_secs_f32(), sha[..8].to_string());
    Ok(())
}

/// Resolve + write crates/mc-registry/src/generated/atlas.rs.
fn write_atlas(workspace_root: &Path, version: &str, client_sha1: &str, jar_bytes: &[u8]) -> Result<()> {
    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let src = atlas::build(jar_bytes, version, client_sha1)
        .with_context(|| format!("atlas build for {version}"))?;
    let path = reg_dir.join("atlas.rs");
    std::fs::write(&path, &src)?;
    let tiles = src.matches("static TILE_").count();
    println!("      atlas: {} tiles -> {}", tiles, path.display());
    Ok(())
}

/// UTC timestamp without pulling chrono in.
#[allow(dead_code)]
fn chrono_now_iso() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = now / 86_400;
    let secs = now % 86_400;
    let (y, mo, d) = civil_from_days(days as i64);
    format!("{y:04}-{mo:02}-{d:02}T{:02}:{:02}:{:02}Z", secs / 3600, (secs % 3600) / 60, secs % 60)
}

///_days-since-epoch helper kept for potential future use.
#[allow(dead_code)]
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

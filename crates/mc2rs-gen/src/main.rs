//! mc2rs-gen — the version-agnostic Minecraft → Rust generator CLI.

mod atlas;
mod codegen;
mod extract;
mod fontgen;
mod jar;
mod langgen;
mod manifest;
mod soundgen;
mod splashgen;
mod uigen;
mod util;
mod version;
mod worldgen;

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
        /// Comma-separated object-key prefixes to download (empty = all).
        /// Example: "font/,gui/,texts/"
        #[arg(long, default_value = "")]
        filter: String,
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
        Cmd::Assets { version, dir, filter } => {
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
            let prefixes: Vec<String> = filter
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();
            let filtered = if prefixes.is_empty() {
                index
            } else {
                let objects: std::collections::BTreeMap<String, mc_assets::AssetObject> = index
                    .objects
                    .iter()
                    .filter(|(k, _)| prefixes.iter().any(|p| k.starts_with(p.as_str())))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                mc_assets::AssetIndex { map_to_resources: index.map_to_resources, objects }
            };
            println!(
                "asset index: {} object(s), filter {:?} -> {} to download",
                index.object_count(),
                prefixes,
                filtered.object_count()
            );
            std::fs::create_dir_all(&dir)?;
            let t0 = std::time::Instant::now();
            mc_assets::download_all(&filtered, Path::new(&dir), &agent, |done, total| {
                println!("  assets {done}/{total}");
            })?;
            println!(
                "downloaded/verified {} objects ({:.1}s) -> {}",
                filtered.object_count(),
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

    println!("[3/7] analyzing jar (obfuscation detection, registry extraction)...");
    let report = jar::analyze(&jar_bytes)?;
    println!(
        "      obfuscated: {} (ratio {:.2}), classes: {}, net/minecraft classes: {}",
        report.obfuscated, report.obfuscation_ratio, report.class_count, report.net_minecraft_classes
    );

    println!("[4/7] extracting official data (font chain, lang, splashes, GUI sprites, sounds, worldgen)...");
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(jar_bytes.as_slice()))
        .context("open client jar as zip")?;
    let font = fontgen::generate(&mut zip)?;
    let lang = langgen::generate(&mut zip)?;
    let splashes = splashgen::generate(&mut zip)?;
    let ui = uigen::generate(&mut zip)?;
    let worldgen = worldgen::generate(&mut zip)?;
    let sounds_json = match fetch_sounds_json(agent, cache, vj) {
        Ok(b) => soundgen::generate(&b)?,
        Err(e) => {
            eprintln!("      warning: sounds.json unavailable ({e}); emitting empty sound table");
            soundgen::SoundData::default()
        }
    };
    println!(
        "      glyphs: {} ({} textures) | lang: {} | splashes: {} | sprites: {} | sound events: {} | biomes: {}",
        font.glyphs.len(),
        font.textures.len(),
        lang.map.len(),
        splashes.lines.len(),
        ui.sprites.len(),
        sounds.events.len(),
        worldgen.biomes.len()
    );

    println!("[5/7] generating registry + build info...");
    let gen = codegen::GenAll { font, lang, splashes, ui, sounds: sounds_json, worldgen };
    let report_path = codegen::emit(&workspace_root, version, &client_sha1, &report, &gen)?;

    println!("[6/7] extracting REAL texture atlas (blockstates -> models -> PNGs)...");
    write_atlas(&workspace_root, version, &client_sha1, &jar_bytes)?;

    let extracted = codegen::extract_runtime_assets(&mut zip, &workspace_root, &gen.ui, &gen.font)?;
    println!("[7/7] done in {:.1}s ({} runtime asset files -> assets-cache/jar)", t0.elapsed().as_secs_f32(), extracted);
    println!(
        "      blocks: {} | items: {} | textures: {} | sounds: {}",
        report.blockstate_count, report.item_count, report.texture_count, report.sound_count
    );
    println!("      report: {}", report_path.display());
    println!("      next: cargo test -p mc-registry && cargo run -p mc-client --release -- --headless");
    Ok(())
}

/// Fetch the official sounds.json through the asset index (it ships as an
/// index object, not inside the client jar).
fn fetch_sounds_json(
    agent: &ureq::Agent,
    cache: &util::Cache,
    vj: &crate::version::VersionJson,
) -> Result<Vec<u8>> {
    let ai = vj
        .asset_index
        .as_ref()
        .context("version json has no asset index")?;
    let idx_bytes = cache.fetch(
        agent,
        &format!("asset-index:{}", ai.sha1.as_deref().unwrap_or(&ai.id)),
        &ai.url,
    )?;
    util::verify(&idx_bytes, ai.sha1.as_deref(), "asset index")?;
    let index = mc_assets::AssetIndex::parse(&idx_bytes)?;
    let obj = index
        .objects
        .get("minecraft/sounds.json")
        .context("sounds.json not in asset index")?
        .clone();
    let url = format!("https://resources.download.minecraft.net/{}/{}", &obj.hash[..2], obj.hash);
    let bytes = cache.fetch(agent, &format!("sounds-json:{}", obj.hash), &url)?;
    util::verify(&bytes, Some(obj.hash.as_str()), "sounds.json")?;
    Ok(bytes)
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

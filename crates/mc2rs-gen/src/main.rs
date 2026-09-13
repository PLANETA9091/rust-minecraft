//! mc2rs-gen — the version-agnostic Minecraft → Rust generator CLI.

mod atlas;
mod codegen;
mod extract;
mod fontgen;
mod itemgen;
mod jar;
mod langgen;
mod manifest;
mod modelgen;
mod pack_claim;
mod packet_dump;
mod runtime_dump;
mod skygen;
mod skingen;
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
    /// Generate the bytecode-true global block state id table: boot the
    /// REAL vanilla registries from the official jar + libraries (all
    /// sha1-verified) with the checked-in runtime dumper and emit
    /// mc-registry/src/generated/block_ids.rs.
    DumpBlockStates {
        #[arg(long)]
        version: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
        /// Explicit java binary (>= the version's java requirement from
        /// the manifest). Default: `java` on PATH.
        #[arg(long)]
        java: Option<String>,
    },
    /// Generate the bytecode-true packet id tables for EVERY phase: bind
    /// the REAL vanilla play, login, configuration, status and handshake
    /// protocols from the official jar (all sha1-verified) with the
    /// checked-in runtime dumper and emit
    /// mc-protocol/src/generated/packet_ids.rs.
    DumpPacketIds {
        #[arg(long)]
        version: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
        /// Explicit java binary (>= the version's java requirement from
        /// the manifest). Default: `java` on PATH.
        #[arg(long)]
        java: Option<String>,
    },
    /// Dump EVERY built-in registry's entry order (the vanilla
    /// registration = wire order) from the REAL bootstrapped registries:
    /// boot the official jar + libraries (all sha1-verified) with the
    /// checked-in dumper and save the `registries.txt` evidence file.
    DumpRegistries {
        #[arg(long)]
        version: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
        /// Explicit java binary (>= the version's java requirement from
        /// the manifest). Default: `java` on PATH.
        #[arg(long)]
        java: Option<String>,
    },
    /// Re-emit mc-registry/src/generated/registry_order.rs from a SAVED
    /// dump (the `.mc2rs/classpath/registries.txt` the dumper wrote) —
    /// the offline half of the registry-order arc: no java, no network.
    RegistryOrder {
        #[arg(long)]
        version: String,
        /// The saved dump text (`registry|id|entry_name` per line).
        #[arg(long)]
        dump: String,
        /// The client jar sha1 the dump came from (baked into the header).
        #[arg(long)]
        client_sha1: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
    },
    /// Re-emit mc-registry/src/generated/block_ids.rs from a SAVED dump
    /// (the `.mc2rs/classpath/block_states.txt` the dumper wrote) — the
    /// offline half of the dump-block-states arc: no java, no network.
    BlockIds {
        #[arg(long)]
        version: String,
        /// The saved dump text (`id|name|props` per line).
        #[arg(long)]
        dump: String,
        /// The client jar sha1 the dump came from (baked into the header).
        #[arg(long)]
        client_sha1: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
    },
    /// Re-emit mc-protocol/src/generated/pack_claim.rs (the Select Known
    /// Packs reply's version constants) from a LOCAL client jar — the
    /// offline half of the pack-version claim arc: no network, no java.
    PackClaim {
        #[arg(long)]
        version: String,
        /// Path to the official client jar (e.g. tools/client.jar).
        #[arg(long)]
        jar: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
    },
    /// Re-emit mc-registry/src/generated/skin.rs (the player skin's integrity
    /// sha1s + the vanilla arm-box geometry — the first-person hand's data
    /// half) from a LOCAL client jar: no network, no java.
    Skin {
        #[arg(long)]
        version: String,
        /// Path to the official client jar (e.g. .mc2rs/classpath/client.jar).
        #[arg(long)]
        jar: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
    },
    /// Item model definitions (assets/minecraft/items/) -> item_models.rs.
    ItemModels {
        #[arg(long)]
        version: String,
        /// Path to the official client jar (e.g. .mc2rs/classpath/client.jar).
        #[arg(long)]
        jar: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
    },
    /// Block/item model files (assets/minecraft/models/) -> models.rs: the
    /// parent chains, texture + display stanzas, the element skip list.
    Models {
        #[arg(long)]
        version: String,
        /// Path to the official client jar (e.g. .mc2rs/classpath/client.jar).
        #[arg(long)]
        jar: String,
        /// Workspace root (contains crates/).
        #[arg(long, default_value = ".")]
        out: String,
    },
    /// Re-emit mc-registry/src/generated/worldgen.rs (biomes + the built-in
    /// dimension_type/timeline/timeline-tag registry tables) from a LOCAL
    /// client jar — the offline half of the worldgen arc: no network, no java.
    Worldgen {
        #[arg(long)]
        version: String,
        /// Path to the official client jar (e.g. .mc2rs/classpath/client.jar).
        #[arg(long)]
        jar: String,
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
        Cmd::DumpBlockStates { version, out, java } => {
            runtime_dump::dump_block_states(&agent, &cache, &version, Path::new(&out), java.as_deref())?;
        }
        Cmd::DumpPacketIds { version, out, java } => {
            packet_dump::dump_packet_ids(&agent, &cache, &version, Path::new(&out), java.as_deref())?;
        }
        Cmd::DumpRegistries { version, out, java } => {
            let path = runtime_dump::dump_registries(&agent, &cache, &version, Path::new(&out), java.as_deref())?;
            println!("registries dump: {}", path.display());
        }
        Cmd::RegistryOrder { version, dump, client_sha1, out } => {
            let text = std::fs::read_to_string(&dump)
                .with_context(|| format!("read dump {dump}"))?;
            let root = Path::new(&out)
                .canonicalize()
                .with_context(|| format!("workspace root {out} not found"))?;
            let path = runtime_dump::emit_registry_order(&root, &version, &client_sha1, &text)?;
            println!("registry-order: {}", path.display());
        }
        Cmd::BlockIds { version, dump, client_sha1, out } => {
            let text = std::fs::read_to_string(&dump)
                .with_context(|| format!("read dump {dump}"))?;
            let root = Path::new(&out)
                .canonicalize()
                .with_context(|| format!("workspace root {out} not found"))?;
            let path = runtime_dump::emit_block_ids(&root, &version, &client_sha1, &text)?;
            println!("block-ids: {}", path.display());
        }
        Cmd::PackClaim { version, jar, out } => {
            let bytes = std::fs::read(&jar).with_context(|| format!("read jar {jar}"))?;
            let root = Path::new(&out)
                .canonicalize()
                .with_context(|| format!("workspace root {out} not found"))?;
            let path = write_pack_claim(&root, &bytes)?;
            println!("pack-claim ({version}): {}", path.display());
        }
        Cmd::Worldgen { version, jar, out } => {
            let bytes = std::fs::read(&jar).with_context(|| format!("read jar {jar}"))?;
            let root = Path::new(&out)
                .canonicalize()
                .with_context(|| format!("workspace root {out} not found"))?;
            let path = write_worldgen(&root, &bytes)?;
            println!("worldgen ({version}): {}", path.display());
        }
        Cmd::Skin { version, jar, out } => {
            let bytes = std::fs::read(&jar).with_context(|| format!("read jar {jar}"))?;
            let root = Path::new(&out)
                .canonicalize()
                .with_context(|| format!("workspace root {out} not found"))?;
            let path = write_skin(&root, &bytes)?;
            println!("skin ({version}): {}", path.display());
        }
        Cmd::Models { version, jar, out } => {
            let bytes = std::fs::read(&jar).with_context(|| format!("read jar {jar}"))?;
            let root = std::path::PathBuf::from(&out);
            let path = write_models(&root, &bytes)?;
            println!("models ({version}): {}", path.display());
        }
        Cmd::ItemModels { version, jar, out } => {
            let bytes = std::fs::read(&jar).with_context(|| format!("read jar {jar}"))?;
            let root = Path::new(&out)
                .canonicalize()
                .with_context(|| format!("workspace root {out} not found"))?;
            let path = write_item_models(&root, &bytes)?;
            println!("item-models ({version}): {}", path.display());
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
            let total_objects = index.object_count();
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
                    .map(|(k, v)| (k.as_str().to_string(), (*v).clone()))
                    .collect();
                mc_assets::AssetIndex { map_to_resources: index.map_to_resources, objects }
            };
            println!(
                "asset index: {} object(s), filter {:?} -> {} to download",
                total_objects,
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
    let mut ui = uigen::generate(&mut zip)?;
    let worldgen = worldgen::generate(&mut zip)?;
    // The 26.x day-night cycle is data: timeline tracks + dimension colors +
    // real sun/moon-phase textures.
    let sky = skygen::generate(&mut zip)?;
    // The asset index drives sounds.json AND the real title panorama
    // (the 26.2 jar ships 1x1 panorama placeholders in its place).
    let index = match fetch_asset_index(agent, cache, &vj) {
        Ok(i) => Some(i),
        Err(e) => {
            eprintln!("      warning: asset index unavailable ({e}); sounds.json + panorama skipped");
            None
        }
    };
    let sounds_json = match &index {
        Some(i) => match fetch_sounds_json(agent, cache, i) {
            Ok(b) => soundgen::generate(&b)?,
            Err(e) => {
                eprintln!("      warning: sounds.json unavailable ({e}); emitting empty sound table");
                soundgen::SoundData::default()
            }
        },
        None => soundgen::SoundData::default(),
    };
    // Real title panorama faces (1024x1024, sha1-verified) into the runtime
    // asset root; then let their real sizes replace the 1x1 jar stubs.
    let panorama_faces = match &index {
        Some(i) => fetch_title_panorama(agent, cache, i, &workspace_root).unwrap_or_else(|e| {
            eprintln!("      warning: title panorama fetch failed ({e}); keeping jar stubs");
            0
        }),
        None => 0,
    };
    // Menu music + in-game music + UI sounds + biome music: the real .ogg
    // files stream as asset-index objects (the jar ships only sounds.json).
    // Fetch every `music.menu` track, the `music.game` pool, the vanilla UI
    // click AND every music event the official biome data references into
    // the runtime asset root so the client decodes + plays them.
    let biome_events: Vec<String> = {
        let mut v: Vec<String> = worldgen
            .biome_music
            .iter()
            .map(|m| m.event.clone())
            .collect();
        v.sort();
        v.dedup();
        v
    };
    let (music_files, game_files, ui_sound_files, biome_sound_files) = match &index {
        Some(i) => {
            fetch_sound_events(agent, cache, i, &sounds_json, &biome_events, &workspace_root)
                .unwrap_or_else(|e| {
                    eprintln!("      warning: game audio fetch failed ({e}); client falls back to silence");
                    (0, 0, 0, 0)
                })
        }
        None => (0, 0, 0, 0),
    };
    let pano_root = workspace_root.join("assets-cache/jar");
    let pano_names: Vec<(String, String)> = (0..6)
        .map(|i| {
            (
                format!("title/background/panorama_{i}"),
                format!("gui/title/background/panorama_{i}.png"),
            )
        })
        .collect();
    uigen::merge_from_disk(&mut ui, &pano_root, &pano_names, true)?;
    println!(
        "      glyphs: {} ({} textures) | lang: {} | splashes: {} | sprites: {} | sound events: {} | biomes: {} | panorama faces: {} | menu music: {} | game music: {} | ui sounds: {} | biome music: {} files / {} events",
        font.glyphs.len(),
        font.textures.len(),
        lang.map.len(),
        splashes.lines.len(),
        ui.sprites.len(),
        sounds_json.events.len(),
        worldgen.biomes.len(),
        panorama_faces,
        music_files,
        game_files,
        ui_sound_files,
        biome_sound_files,
        biome_events.len()
    );
    println!(
        "      sky: day period {} ticks, {} moon phases, celestial textures {}x{}",
        sky.day_period,
        sky.moon_phase_names.len(),
        sky.sun_rgba.0,
        sky.sun_rgba.1
    );

    println!("[5/7] generating registry + build info...");
    let gen = codegen::GenAll { font, lang, splashes, ui, sounds: sounds_json, worldgen, sky };
    let report_path = codegen::emit(&workspace_root, version, &client_sha1, &report, &gen)?;

    println!("[6/7] extracting REAL texture atlas (blockstates -> models -> PNGs)...");
    write_atlas(&workspace_root, version, &client_sha1, &jar_bytes)?;
    let skin_path = write_skin(&workspace_root, &jar_bytes)?;
    println!("      skin table: {}", skin_path.display());
    let item_models_path = write_item_models(&workspace_root, &jar_bytes)?;
    println!("      item-models table: {}", item_models_path.display());
    let models_path = write_models(&workspace_root, &jar_bytes)?;
    println!("      models table: {}", models_path.display());

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

/// Fetch + parse the official asset index for a version.
fn fetch_asset_index(
    agent: &ureq::Agent,
    cache: &util::Cache,
    vj: &crate::version::VersionJson,
) -> Result<mc_assets::AssetIndex> {
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
    mc_assets::AssetIndex::parse(&idx_bytes)
}

/// Fetch the official sounds.json through the asset index (it ships as an
/// index object, not inside the client jar).
fn fetch_sounds_json(
    agent: &ureq::Agent,
    cache: &util::Cache,
    index: &mc_assets::AssetIndex,
) -> Result<Vec<u8>> {
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

/// The 26.2 jar ships the title-screen panorama as 1x1 placeholder PNGs; the
/// real 1024x1024 faces stream as asset-index objects. Download + sha1-verify
/// all six faces into the runtime asset root (assets-cache/jar) so the client
/// renders the identical backdrop. Returns the number of faces fetched.
fn fetch_title_panorama(
    agent: &ureq::Agent,
    cache: &util::Cache,
    index: &mc_assets::AssetIndex,
    workspace_root: &Path,
) -> Result<usize> {
    let mut n = 0usize;
    for i in 0..6 {
        let key = format!("minecraft/textures/gui/title/background/panorama_{i}.png");
        let Some(obj) = index.objects.get(&key) else { continue };
        let url = format!("https://resources.download.minecraft.net/{}/{}", &obj.hash[..2], obj.hash);
        let bytes = cache.fetch(agent, &format!("asset:panorama-{i}:{}", obj.hash), &url)?;
        util::verify(&bytes, Some(obj.hash.as_str()), &key)?;
        let dest = workspace_root
            .join("assets-cache/jar/gui/title/background")
            .join(format!("panorama_{i}.png"));
        std::fs::create_dir_all(dest.parent().unwrap())?;
        std::fs::write(&dest, &bytes)?;
        n += 1;
    }
    Ok(n)
}

/// Sound events the client plays beyond menu music, by exact id. The vanilla
/// UI button press sound and the in-game overworld pool live here: every GUI
/// button click plays `ui.button.click` (26.2: one file,
/// `random/click_stereo`), a loaded world plays `music.game` (26.2: the
/// 32-file weighted soundtrack pool). Biome-referenced music events
/// (`music.overworld.*`, `music.nether.*`, …) come in through the parsed
/// biome-music table, not this constant.
const EXTRA_SOUND_EVENTS: [&str; 2] = ["ui.button.click", "music.game"];

/// Fetch the real menu music + in-game music + UI sound + biome music .ogg
/// files through the asset index. sounds.json names are relative to
/// `assets/minecraft/sounds/` (extension optional — the vanilla sound engine
/// appends `.ogg`); the asset index keys include the `minecraft/` prefix.
/// Every file is sha1-verified against the index object. Returns
/// (menu music, game music, ui sound, biome music) files fetched
/// (duplicates across events deduped).
fn fetch_sound_events(
    agent: &ureq::Agent,
    cache: &util::Cache,
    index: &mc_assets::AssetIndex,
    sounds: &soundgen::SoundData,
    biome_events: &[String],
    workspace_root: &Path,
) -> Result<(usize, usize, usize, usize)> {
    // 26.2 ships one `music.menu` event with many weighted files (older
    // versions had per-track `music.menu.menu1` children) — match both.
    let wanted = |id: &str| {
        id.starts_with("music.menu")
            || EXTRA_SOUND_EVENTS.contains(&id)
            || biome_events.binary_search(&id.to_string()).is_ok()
    };
    let mut menu = 0usize;
    let mut game = 0usize;
    let mut ui = 0usize;
    let mut biome = 0usize;
    let mut seen = std::collections::BTreeSet::new();
    for ev in sounds.events.values().filter(|e| wanted(&e.id)) {
        let mut have = 0usize;
        for f in &ev.files {
            let name = if f.name.ends_with(".ogg") {
                f.name.clone()
            } else {
                format!("{}.ogg", f.name)
            };
            let dest = workspace_root
                .join("assets-cache/jar/assets/minecraft/sounds")
                .join(&name);
            // several events share files (26.2: five music/game tracks are
            // also in the menu pool) — fetch each DISK file once, but count
            // it for every event that references it (disk truth, not fetch
            // order)
            if seen.insert(name.clone()) {
                let key = format!("minecraft/sounds/{name}");
                let Some(obj) = index.objects.get(&key) else { continue };
                let url = format!(
                    "https://resources.download.minecraft.net/{}/{}",
                    &obj.hash[..2],
                    obj.hash
                );
                let bytes = cache.fetch(agent, &format!("asset:music:{}", obj.hash), &url)?;
                util::verify(&bytes, Some(obj.hash.as_str()), &key)?;
                std::fs::create_dir_all(dest.parent().unwrap())?;
                std::fs::write(&dest, &bytes)?;
            }
            if dest.is_file() {
                have += 1;
            }
        }
        if ev.id.starts_with("music.menu") {
            menu = have;
        } else if ev.id.starts_with("music.game") {
            game = have;
        } else if biome_events.binary_search(&ev.id).is_ok() {
            biome += have;
        } else {
            ui = have;
        }
    }
    Ok((menu, game, ui, biome))
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

/// Read the jar's version.json and write
/// crates/mc-protocol/src/generated/pack_claim.rs (the claim's version
/// constants) from it.
fn write_pack_claim(workspace_root: &Path, jar_bytes: &[u8]) -> Result<std::path::PathBuf> {
    let mut zip =
        zip::ZipArchive::new(std::io::Cursor::new(jar_bytes)).context("open client jar as zip")?;
    let claim = pack_claim::generate(&mut zip)?;
    let proto_dir = workspace_root.join("crates/mc-protocol/src/generated");
    std::fs::create_dir_all(&proto_dir)?;
    let path = proto_dir.join("pack_claim.rs");
    std::fs::write(&path, pack_claim::emit(&claim)?)?;
    println!(
        "      claim: (minecraft, core, {}) | world {} | protocol {} | pack {}.{}.{}.{}",
        claim.id,
        claim.world_version,
        claim.protocol_version,
        claim.resource_major,
        claim.resource_minor,
        claim.data_major,
        claim.data_minor
    );
    Ok(path)
}

/// Resolve + write crates/mc-registry/src/generated/worldgen.rs (biomes +
/// the built-in dimension_type/timeline registry tables) from a local jar.
fn write_worldgen(workspace_root: &Path, jar_bytes: &[u8]) -> Result<std::path::PathBuf> {
    let mut zip =
        zip::ZipArchive::new(std::io::Cursor::new(jar_bytes)).context("open client jar as zip")?;
    let data = worldgen::generate(&mut zip)?;
    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("worldgen.rs");
    std::fs::write(&path, worldgen::emit(&data)?)?;
    println!(
        "      biomes: {} | dimension types: {} | timelines: {} | timeline tags: {} | tint extras: {} | world clocks: {} | biome tags: {}",
        data.biomes.len(),
        data.dimension_types.len(),
        data.timelines.len(),
        data.timeline_tags.len(),
        data.biome_tint_extras.len(),
        data.world_clocks.len(),
        data.biome_tags.len()
    );
    Ok(path)
}

/// Resolve + write crates/mc-registry/src/generated/skin.rs (the player
/// skin's integrity sha1s + the arm-box geometry) from a local jar.
fn write_skin(workspace_root: &Path, jar_bytes: &[u8]) -> Result<std::path::PathBuf> {
    let mut zip =
        zip::ZipArchive::new(std::io::Cursor::new(jar_bytes)).context("open client jar as zip")?;
    let data = skingen::generate(&mut zip)?;
    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("skin.rs");
    std::fs::write(&path, skingen::emit(&data)?)?;
    println!(
        "      skin: wide steve {}x{} (png {}…, rgba {}…) -> {}",
        data.side,
        data.side,
        &data.png_sha1[..8],
        &data.rgba_sha1[..8],
        path.display()
    );
    Ok(path)
}

/// Resolve + write crates/mc-registry/src/generated/models.rs (the block/item
/// model files: parent chains, texture + display stanzas) from a local jar.
fn write_models(workspace_root: &Path, jar_bytes: &[u8]) -> Result<std::path::PathBuf> {
    let mut zip =
        zip::ZipArchive::new(std::io::Cursor::new(jar_bytes)).context("open client jar as zip")?;
    let data = modelgen::generate(&mut zip)?;
    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("models.rs");
    std::fs::write(&path, modelgen::emit(&data)?)?;
    println!(
        "      models: {} files ({} block / {} item), {} carry elements -> {}",
        data.models.len(),
        data.blocks,
        data.items,
        data.element_models.len(),
        path.display()
    );
    Ok(path)
}

/// Resolve + write crates/mc-registry/src/generated/item_models.rs (the
/// 1.21.4+ item model definitions) from a local jar.
fn write_item_models(workspace_root: &Path, jar_bytes: &[u8]) -> Result<std::path::PathBuf> {
    let mut zip =
        zip::ZipArchive::new(std::io::Cursor::new(jar_bytes)).context("open client jar as zip")?;
    let data = itemgen::generate(&mut zip)?;
    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("item_models.rs");
    std::fs::write(&path, itemgen::emit(&data)?)?;
    println!(
        "      item models: {} plain defs | {} complex trees | {:?} root types -> {}",
        data.defs.len(),
        data.complex.len(),
        data.root_types,
        path.display()
    );
    Ok(path)
}

/// Resolve + write crates/mc-registry/src/generated/{atlas,colormap}.rs.
fn write_atlas(workspace_root: &Path, version: &str, client_sha1: &str, jar_bytes: &[u8]) -> Result<()> {
    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let src = atlas::build(jar_bytes, version, client_sha1)
        .with_context(|| format!("atlas build for {version}"))?;
    let path = reg_dir.join("atlas.rs");
    std::fs::write(&path, &src)?;
    let tiles = src.matches("static TILE_").count();
    println!("      atlas: {} tiles -> {}", tiles, path.display());
    let cmap = atlas::build_colormap(jar_bytes, version, client_sha1)
        .with_context(|| format!("colormap build for {version}"))?;
    let cmap_path = reg_dir.join("colormap.rs");
    std::fs::write(&cmap_path, &cmap)?;
    println!(
        "      colormap: {}x{} grass/foliage grids -> {}",
        atlas::COLORMAP_SIDE,
        atlas::COLORMAP_SIDE,
        cmap_path.display()
    );
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

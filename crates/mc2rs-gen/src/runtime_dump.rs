//! The runtime-dumper arc: generate the bytecode-true global block state
//! id table by booting the REAL vanilla registries from the official jar.
//!
//! Steps (all cache-first, all sha1-verified — the project's discipline):
//! 1. resolve the version json (piston-meta) and its java requirement;
//! 2. fetch the client jar + every linux-relevant library artifact;
//! 3. materialize them as classpath FILES under `.mc2rs/classpath/`;
//! 4. resolve a java binary whose class version >= the requirement
//!    (`--java` override, else `java` on PATH — Mojang's own runtime-
//!    manifest endpoints are gone (BlobNotFound), so the tool names the
//!    requirement and uses a provided runtime; a Java {req} JDK fits via
//!    Adoptium, the requirement itself comes from the official manifest);
//! 5. run the checked-in single-file dumper (`dump/DumpBlockStates.java`,
//!    our clean-room tool) through the source launcher — it calls
//!    `Bootstrap.bootStrap()` and prints `id|name|props` per global state;
//! 6. emit `crates/mc-registry/src/generated/block_ids.rs` — the mapper
//!    table the client renders network terrain through.
//!
//! NO Mojang code is read into the repository: only the emitted id/name
//! structure, the same class of data as the blockstate JSONs.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::util::{self, Cache};
use crate::version::{DirectDownload, VersionJson};

/// The linux classpath artifacts of a version (vanilla launcher rule
/// semantics, main artifact only).
pub fn linux_classpath(vj: &VersionJson) -> Vec<(&str, &DirectDownload)> {
    vj.libraries
        .iter()
        .filter(|l| l.allowed_on("linux"))
        .filter_map(|l| l.artifact().map(|a| (l.name.as_str(), a)))
        .collect()
}

/// Parse a `java -version` output (first line) into the major version
/// (legacy 1.8 scheme included).
pub fn parse_java_version_output(text: &str) -> u32 {
    let line = text.lines().next().unwrap_or("");
    let quoted = line.split('"').nth(1).unwrap_or("");
    let mut parts = quoted.split(['.', '-', '_']);
    let first = parts.next().unwrap_or("0");
    if first == "1" {
        parts.next().unwrap_or("0").parse().unwrap_or(0)
    } else {
        first.parse().unwrap_or(0)
    }
}

/// Probe a java binary's major version (`java -version` stderr).
pub fn java_major(java: &Path) -> Result<u32> {
    let out = Command::new(java)
        .arg("-version")
        .output()
        .with_context(|| format!("run {} -version", java.display()))?;
    Ok(parse_java_version_output(&String::from_utf8_lossy(&out.stderr)))
}

/// Pick the java binary: explicit override, else `java` on PATH; fail with
/// the requirement (from the version json) when the probe is too old.
pub fn resolve_java(explicit: Option<&str>, required_major: u32) -> Result<PathBuf> {
    let candidate = match explicit {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from("java"),
    };
    let major = java_major(&candidate)?;
    if required_major > 0 && major < required_major {
        bail!(
            "{} reports java {major}, but this Minecraft version requires \
             class version {required_major}; pass --java with a newer JDK \
             (e.g. Adoptium: https://api.adoptium.net/v3/binary/latest/\
             {required_major}/ga/linux/x64/jdk/hotspot/normal/eclipse)",
            candidate.display()
        );
    }
    Ok(candidate)
}

/// Materialize one artifact as a classpath FILE (cache-first: the bytes
/// already sit in the content-addressed cache; the file only appears when
/// its sha1 matches).
pub(crate) fn materialize(
    agent: &ureq::Agent,
    cache: &Cache,
    cp_dir: &Path,
    file_name: &str,
    dl: &DirectDownload,
) -> Result<PathBuf> {
    let bytes = cache.fetch(
        agent,
        &format!("cp:{}", dl.sha1.as_deref().unwrap_or(file_name)),
        &dl.url,
    )?;
    util::verify(&bytes, dl.sha1.as_deref(), file_name)?;
    let path = cp_dir.join(file_name);
    if !path.exists() || std::fs::metadata(&path)?.len() != bytes.len() as u64 {
        std::fs::write(&path, &bytes)
            .with_context(|| format!("materialize {}", path.display()))?;
    }
    Ok(path)
}

/// One entry of the dump: `id|block_name|props` — the raw `k=v,k=v`
/// property string rides along (`None` when the block has no properties);
/// the property-assignment table is generated from it, the client's
/// per-state rendering (leaf_litter's facing x segment_amount, the
/// fourteenth audit) decodes through it.
fn parse_dump(text: &str) -> Result<Vec<(u32, String, Option<String>)>> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let mut it = line.splitn(3, '|');
        let id: u32 = it
            .next()
            .unwrap_or("")
            .parse()
            .with_context(|| format!("dump line {}: bad id", i + 1))?;
        let name = it
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        if name.is_empty() {
            bail!("dump line {}: missing block name", i + 1);
        }
        // the raw props string ("" -> None); kept VERBATIM — the values
        // are the jar's own words, never remembered
        let props = it.next().map(str::trim).filter(|s| !s.is_empty()).map(str::to_string);
        out.push((id, name, props));
    }
    out.sort_unstable_by_key(|(id, _, _)| *id);
    // density check: ids must tile 0..n exactly (the registry is dense)
    for (i, (id, _, _)) in out.iter().enumerate() {
        if *id != i as u32 {
            bail!("dump ids not dense at index {i} (id {id})");
        }
    }
    Ok(out)
}

/// Collapse per-state entries into per-block ranges.
fn block_ranges(dump: &[(u32, String, Option<String>)]) -> Vec<(u32, u32, &str)> {
    let mut ranges: Vec<(u32, u32, &str)> = Vec::new();
    for (id, name, _) in dump {
        match ranges.last_mut() {
            Some((_, hi, last)) if *last == name.as_str() => *hi = id + 1,
            _ => ranges.push((*id, id + 1, name.as_str())),
        }
    }
    ranges
}

/// Collapse per-state property assignments into runs: adjacent states with
/// IDENTICAL props merge (no-property runs merge across blocks too). The
/// result tiles [0, STATE_COUNT) exactly, mirroring BLOCK_RANGES.
fn props_ranges(dump: &[(u32, String, Option<String>)]) -> Vec<(u32, u32, &str)> {
    let mut ranges: Vec<(u32, u32, &str)> = Vec::new();
    for (id, _, props) in dump {
        let p = props.as_deref().unwrap_or("");
        match ranges.last_mut() {
            Some((_, hi, last)) if *last == p => *hi = id + 1,
            _ => ranges.push((*id, id + 1, p)),
        }
    }
    ranges
}

/// Generate crates/mc-registry/src/generated/block_ids.rs from dump text.
pub fn emit_block_ids(workspace_root: &Path, version: &str, client_sha1: &str, dump_text: &str) -> Result<PathBuf> {
    let dump = parse_dump(dump_text)?;
    let ranges = block_ranges(&dump);
    let props = props_ranges(&dump);
    let state_count = dump.len() as u32;
    let block_count = ranges.len();

    let mut src = String::with_capacity(3 * 1024 * 1024);
    src.push_str(&format!(
        "// @generated by `mc2rs-gen dump-block-states` for Minecraft {version} — DO NOT EDIT BY HAND.\n\
         // Source of truth: the official client jar's REAL bootstrapped runtime registry\n\
         // (client sha1 {client_sha1}); dumped in vanilla registration order.\n\
         // Regenerate: `cargo run -p mc2rs-gen -- dump-block-states --version {version}`\n\n\
         /// Global block state count (ids are dense: 0..STATE_COUNT).\n\
         pub const STATE_COUNT: u32 = {state_count};\n\n\
         /// (start_id, end_id_exclusive, block name) — one range per block,\n\
         /// in vanilla registry order; ids are dense so the ranges tile\n\
         /// [0, STATE_COUNT) exactly.\n\
         pub const BLOCK_RANGES: &[(u32, u32, &str)] = &[\n"
    ));
    for (lo, hi, name) in &ranges {
        src.push_str(&format!("    ({lo}, {hi}, \"{name}\"),\n"));
    }
    src.push_str(
        "];\n\n\
         /// The block name that owns a global state id (binary search over\n\
         /// the generated ranges); `None` past the table.\n\
         pub fn block_name_of(state_id: u32) -> Option<&'static str> {\n\
         \x20   BLOCK_RANGES\n\
         \x20       .binary_search_by(|(lo, hi, _)| {\n\
         \x20           if state_id < *lo {\n\
         \x20               std::cmp::Ordering::Greater\n\
         \x20           } else if state_id >= *hi {\n\
         \x20               std::cmp::Ordering::Less\n\
         \x20           } else {\n\
         \x20               std::cmp::Ordering::Equal\n\
         \x20           }\n\
         \x20       })\n\
         \x20       .ok()\n\
         \x20       .map(|i| BLOCK_RANGES[i].2)\n\
         }\n",
    );

    // The per-state property-assignment runs, APPENDED after the name
    // table: everything above stays byte-identical to the pre-props
    // generation (the determinism check diffs this prefix against HEAD).
    src.push_str(
        "\n\n\
         /// (start_id, end_id_exclusive, props) — one range per DISTINCT\n\
         /// property-assignment run, tiling [0, STATE_COUNT) exactly:\n\
         /// adjacent states with identical properties collapse (no-property\n\
         /// runs merge across blocks); an empty string = the block has no\n\
         /// properties. The values are the jar dumper's own words\n\
         /// Regenerate offline: `mc2rs-gen block-ids --dump <saved\
\
         /// block_states.txt> --client-sha1 <sha> --out <workspace>`\n\
         /// (`k=v,k=v`, vanilla registration order).\n\
         pub const STATE_PROPS: &[(u32, u32, &str)] = &[\n"
    );
    for (lo, hi, p) in &props {
        src.push_str(&format!("    ({lo}, {hi}, \"{p}\"),\n"));
    }
    src.push_str(
        "];\n\n\
         /// The raw `k=v,k=v` property string of a global state id (binary\n\
         /// search over the generated runs); `None` past the table,\n\
         /// `Some(\"\")` when the block has no properties.\n\
         pub fn props_of(state_id: u32) -> Option<&'static str> {\n\
         \x20   STATE_PROPS\n\
         \x20       .binary_search_by(|(lo, hi, _)| {\n\
         \x20           if state_id < *lo {\n\
         \x20               std::cmp::Ordering::Greater\n\
         \x20           } else if state_id >= *hi {\n\
         \x20               std::cmp::Ordering::Less\n\
         \x20           } else {\n\
         \x20               std::cmp::Ordering::Equal\n\
         \x20           }\n\
         \x20       })\n\
         \x20       .ok()\n\
         \x20       .map(|i| STATE_PROPS[i].2)\n\
         }\n",
    );

    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("block_ids.rs");
    std::fs::write(&path, src).context("write block_ids.rs")?;
    println!(
        "block_ids.rs written: {state_count} states / {block_count} blocks / {} props runs -> {}",
        props.len(),
        path.display()
    );
    Ok(path)
}

/// The whole arc: version -> classpath files -> java -> dumper -> table.
pub fn dump_block_states(
    agent: &ureq::Agent,
    cache: &Cache,
    version: &str,
    workspace_root: &Path,
    java_override: Option<&str>,
) -> Result<PathBuf> {
    let t0 = std::time::Instant::now();
    let (java, cp, client_sha1) = vanilla_runtime(agent, cache, version, java_override)?;
    let cp_dir = Path::new(".mc2rs").join("classpath");
    let dump_src = workspace_root.join("crates/mc2rs-gen/dump/DumpBlockStates.java");
    let out_txt = cp_dir.join("block_states.txt");
    run_dumper(&java, &cp, &dump_src, &out_txt)?;
    let dump_text = std::fs::read_to_string(&out_txt).context("read dumper output")?;
    println!("[4/4] registry bootstrapped and dumped in {:.1}s", t0.elapsed().as_secs_f32());
    emit_block_ids(workspace_root, version, &client_sha1, &dump_text)
}

/// The whole arc for the REGISTRY-ORDER dumper: version -> classpath files
/// -> java -> dumper -> the saved `registries.txt` (the evidence file; the
/// offline `registry-order` emitter turns it into the generated table).
pub fn dump_registries(
    agent: &ureq::Agent,
    cache: &Cache,
    version: &str,
    workspace_root: &Path,
    java_override: Option<&str>,
) -> Result<PathBuf> {
    let t0 = std::time::Instant::now();
    let (java, cp, _) = vanilla_runtime(agent, cache, version, java_override)?;
    let cp_dir = Path::new(".mc2rs").join("classpath");
    let dump_src = workspace_root.join("crates/mc2rs-gen/dump/DumpRegistries.java");
    let out_txt = cp_dir.join("registries.txt");
    run_dumper(&java, &cp, &dump_src, &out_txt)?;
    println!(
        "[4/4] registries bootstrapped and dumped in {:.1}s -> {}",
        t0.elapsed().as_secs_f32(),
        out_txt.display()
    );
    Ok(out_txt)
}

/// The whole arc for the DIG-LANE dumper: version -> classpath files ->
/// java -> the block dig table (the destroy arc's own data: the default
/// state's destroySpeed + requiresCorrectToolForDrops per block, the REAL
/// bootstrapped registry's order) emitted straight into
/// `mc-registry/src/generated/block_dig.rs`.
pub fn dump_block_dig(
    agent: &ureq::Agent,
    cache: &Cache,
    version: &str,
    workspace_root: &Path,
    java_override: Option<&str>,
) -> Result<PathBuf> {
    let t0 = std::time::Instant::now();
    let (java, cp, client_sha1) = vanilla_runtime(agent, cache, version, java_override)?;
    let cp_dir = Path::new(".mc2rs").join("classpath");
    let dump_src = workspace_root.join("crates/mc2rs-gen/dump/DumpBlockDig.java");
    let out_txt = cp_dir.join("block_dig.txt");
    run_dumper(&java, &cp, &dump_src, &out_txt)?;
    let dump_text = std::fs::read_to_string(&out_txt).context("read dumper output")?;
    println!("[4/4] block dig data bootstrapped and dumped in {:.1}s", t0.elapsed().as_secs_f32());
    emit_block_dig(workspace_root, version, &client_sha1, &dump_text)
}

/// Materialize the dumper's `name|destroySpeed|requiresTool` lines into the
/// generated dig table. The lines ride the vanilla registration order (the
/// BuiltInRegistries.BLOCK iterator's own order); the emitter keeps that
/// order verbatim and adds a binary-search name lookup.
pub fn emit_block_dig(
    workspace_root: &Path,
    version: &str,
    client_sha1: &str,
    dump_text: &str,
) -> Result<PathBuf> {
    let mut names: Vec<String> = Vec::new();
    let mut speeds: Vec<String> = Vec::new();
    let mut requires: Vec<bool> = Vec::new();
    for line in dump_text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('|');
        let name = parts
            .next()
            .context("dig dump line missing the block name")?;
        let speed = parts
            .next()
            .context("dig dump line missing the destroy speed")?;
        let requires_tool = parts
            .next()
            .context("dig dump line missing the requires-tool flag")?;
        if parts.next().is_some() {
            bail!("dig dump line {name:?}: too many fields");
        }
        let speed_f: f32 = speed
            .parse()
            .with_context(|| format!("dig dump line {name:?}: bad destroy speed {speed:?}"))?;
        let flag = match requires_tool {
            "true" => true,
            "false" => false,
            other => bail!("dig dump line {name:?}: bad requires-tool flag {other:?}"),
        };
        // the registry keys speak the `minecraft:` namespace; the client's
        // own block vocabulary is the bare name — strip the prefix here so
        // the table's lookups match the palette's words directly
        names.push(
            name.strip_prefix("minecraft:")
                .unwrap_or(name)
                .to_string(),
        );
        // {:?} keeps the f32 literal's decimal point (a bare `0` is an
        // integer literal — the f32 array refuses it)
        speeds.push(format!("{:?}", speed_f));
        requires.push(flag);
    }
    anyhow::ensure!(!names.is_empty(), "the dig dump named no blocks");

    let mut src = String::with_capacity(names.len() * 48 + 2048);
    src.push_str(&format!(
        "// @generated by `mc2rs-gen dump-block-dig` for Minecraft {version} — DO NOT EDIT BY HAND.\n\
         // Source of truth: the official client jar's REAL bootstrapped runtime registry\n\
         // (client sha1 {client_sha1}); the default state's own queries, vanilla registration order.\n\
         // Regenerate: `cargo run -p mc2rs-gen -- dump-block-dig --version {version}`\n\n\
         /// The block count (the full BLOCK registry, not the render palette).\n\
         pub const DIG_BLOCK_COUNT: usize = {};\n\n\
         /// The block names in vanilla registration order (parallel to the\n\
         /// two data tables).\n\
         pub const DIG_NAMES: [&str; DIG_BLOCK_COUNT] = [\n",
        names.len()
    ));
    for n in &names {
        src.push_str(&format!("    \"{n}\",\n"));
    }
    src.push_str("];\n\n/// The default state's destroySpeed (the hardness the progress law divides by; -1.0 = unbreakable, 0.0 = insta-break).\n");
    src.push_str("pub const DIG_DESTROY_SPEED: [f32; DIG_BLOCK_COUNT] = [\n");
    for s in &speeds {
        src.push_str(&format!("    {s},\n"));
    }
    src.push_str("];\n\n/// The default state's requiresCorrectToolForDrops (the 30/100 divisor pick).\n");
    src.push_str("pub const DIG_REQUIRES_TOOL: [bool; DIG_BLOCK_COUNT] = [\n");
    for r in &requires {
        src.push_str(&format!("    {r},\n"));
    }
    // THE NAME LOOKUP's own lane: DIG_NAMES stays in vanilla registration
    // order (the evidence's own order), the lookup rides a SORTED index
    // table (the binary search demands sorted data — the registration
    // order is not alphabetical, the plain search misses)
    let mut sorted: Vec<usize> = (0..names.len()).collect();
    sorted.sort_by(|a, b| names[*a].cmp(&names[*b]));
    src.push_str(
        "];\n\n\
         /// The registry indices sorted by NAME (the binary search's own\n\
         /// lane — DIG_NAMES itself stays in vanilla registration order).\n\
         pub const DIG_SORTED: [u16; DIG_BLOCK_COUNT] = [\n",
    );
    for i in &sorted {
        src.push_str(&format!("    {i},\n"));
    }
    src.push_str(
        "];\n\n\
         /// The dig row of a block name (binary search over the sorted\n\
         /// index, the data through the registration order); `None` when\n\
         /// the registry does not speak the name.\n\
         pub fn dig_row(name: &str) -> Option<(f32, bool)> {\n\
         \x20   let pos = DIG_SORTED\n\
         \x20       .binary_search_by(|probe| DIG_NAMES[*probe as usize].cmp(name))\n\
         \x20       .ok()? as usize;\n\
         \x20   let idx = DIG_SORTED[pos] as usize;\n\
         \x20   Some((DIG_DESTROY_SPEED[idx], DIG_REQUIRES_TOOL[idx]))\n\
         }\n",
    );

    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("block_dig.rs");
    std::fs::write(&path, src).context("write block_dig.rs")?;
    println!(
        "block_dig.rs written: {} blocks -> {}",
        names.len(),
        path.display()
    );
    Ok(path)
}

/// The whole arc for the SOUND-LANE dumper: version -> classpath files ->
/// java -> the block sound table (the interaction arc's own data: the
/// default state's SoundType — volume, pitch and the break/step/place/hit/
/// fall sound-event ids — per block, the REAL bootstrapped registry's
/// order) emitted straight into `mc-registry/src/generated/block_sounds.rs`.
pub fn dump_block_sounds(
    agent: &ureq::Agent,
    cache: &Cache,
    version: &str,
    workspace_root: &Path,
    java_override: Option<&str>,
) -> Result<PathBuf> {
    let t0 = std::time::Instant::now();
    let (java, cp, client_sha1) = vanilla_runtime(agent, cache, version, java_override)?;
    let cp_dir = Path::new(".mc2rs").join("classpath");
    let dump_src = workspace_root.join("crates/mc2rs-gen/dump/DumpBlockSounds.java");
    let out_txt = cp_dir.join("block_sounds.txt");
    run_dumper(&java, &cp, &dump_src, &out_txt)?;
    let dump_text = std::fs::read_to_string(&out_txt).context("read dumper output")?;
    println!("[4/4] block sounds bootstrapped and dumped in {:.1}s", t0.elapsed().as_secs_f32());
    emit_block_sounds(workspace_root, version, &client_sha1, &dump_text)
}

/// Materialize the dumper's `name|volume|pitch|break|step|place|hit|fall`
/// lines into the generated sound table. The lines ride the vanilla
/// registration order (the BuiltInRegistries.BLOCK iterator's own order);
/// the emitter keeps that order verbatim and adds a binary-search name
/// lookup (the same shape the dig table speaks).
pub fn emit_block_sounds(
    workspace_root: &Path,
    version: &str,
    client_sha1: &str,
    dump_text: &str,
) -> Result<PathBuf> {
    let mut names: Vec<String> = Vec::new();
    let mut rows: Vec<(String, String, String, String, String, String, String)> = Vec::new();
    for line in dump_text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('|');
        let name = parts
            .next()
            .context("sound dump line missing the block name")?;
        let volume = parts
            .next()
            .context("sound dump line missing the volume")?;
        let pitch = parts
            .next()
            .context("sound dump line missing the pitch")?;
        let break_ev = parts
            .next()
            .context("sound dump line missing the break event")?;
        let step_ev = parts
            .next()
            .context("sound dump line missing the step event")?;
        let place_ev = parts
            .next()
            .context("sound dump line missing the place event")?;
        let hit_ev = parts
            .next()
            .context("sound dump line missing the hit event")?;
        let fall_ev = parts
            .next()
            .context("sound dump line missing the fall event")?;
        if parts.next().is_some() {
            bail!("sound dump line {name:?}: too many fields");
        }
        let volume_f: f32 = volume
            .parse()
            .with_context(|| format!("sound dump line {name:?}: bad volume {volume:?}"))?;
        let pitch_f: f32 = pitch
            .parse()
            .with_context(|| format!("sound dump line {name:?}: bad pitch {pitch:?}"))?;
        // the registry keys speak the `minecraft:` namespace; the client's
        // own block vocabulary is the bare name — strip the prefix here so
        // the table's lookups match the palette's words directly
        names.push(
            name.strip_prefix("minecraft:")
                .unwrap_or(name)
                .to_string(),
        );
        // {:?} keeps the f32 literal's decimal point (a bare `0` is an
        // integer literal — the f32 array refuses it)
        rows.push((
            format!("{:?}", volume_f),
            format!("{:?}", pitch_f),
            break_ev.to_string(),
            step_ev.to_string(),
            place_ev.to_string(),
            hit_ev.to_string(),
            fall_ev.to_string(),
        ));
    }
    anyhow::ensure!(!names.is_empty(), "the sound dump named no blocks");

    let mut src = String::with_capacity(names.len() * 160 + 4096);
    src.push_str(&format!(
        "// @generated by `mc2rs-gen dump-block-sounds` for Minecraft {version} — DO NOT EDIT BY HAND.\n\
         // Source of truth: the official client jar's REAL bootstrapped runtime registry\n\
         // (client sha1 {client_sha1}); the default state's SoundType, vanilla registration order.\n\
         // Regenerate: `cargo run -p mc2rs-gen -- dump-block-sounds --version {version}`\n\n\
         /// The block count (the full BLOCK registry, not the render palette).\n\
         pub const SOUND_BLOCK_COUNT: usize = {};\n\n\
         /// The block names in vanilla registration order (parallel to the\n\
         /// data tables).\n\
         pub const SOUND_NAMES: [&str; SOUND_BLOCK_COUNT] = [\n",
        names.len()
    ));
    for n in &names {
        src.push_str(&format!("    \"{n}\",\n"));
    }
    src.push_str("];\n\n/// The per-block SoundType base volume (the vanilla sound laws fold it).\n");
    src.push_str("pub const SOUND_VOLUME: [f32; SOUND_BLOCK_COUNT] = [\n");
    for r in &rows {
        src.push_str(&format!("    {},\n", r.0));
    }
    src.push_str("];\n\n/// The per-block SoundType base pitch.\n");
    src.push_str("pub const SOUND_PITCH: [f32; SOUND_BLOCK_COUNT] = [\n");
    for r in &rows {
        src.push_str(&format!("    {},\n", r.1));
    }
    src.push_str("];\n\n/// The break-sound event id (the completion's 2001 event plays it).\n");
    src.push_str("pub const SOUND_BREAK: [&str; SOUND_BLOCK_COUNT] = [\n");
    for r in &rows {
        src.push_str(&format!("    \"{}\",\n", r.2));
    }
    src.push_str("];\n\n/// The step-sound event id (the footsteps play it).\n");
    src.push_str("pub const SOUND_STEP: [&str; SOUND_BLOCK_COUNT] = [\n");
    for r in &rows {
        src.push_str(&format!("    \"{}\",\n", r.3));
    }
    src.push_str("];\n\n/// The place-sound event id (the placement plays it).\n");
    src.push_str("pub const SOUND_PLACE: [&str; SOUND_BLOCK_COUNT] = [\n");
    for r in &rows {
        src.push_str(&format!("    \"{}\",\n", r.4));
    }
    src.push_str("];\n\n/// The hit-sound event id (the dig beat plays it).\n");
    src.push_str("pub const SOUND_HIT: [&str; SOUND_BLOCK_COUNT] = [\n");
    for r in &rows {
        src.push_str(&format!("    \"{}\",\n", r.5));
    }
    src.push_str("];\n\n/// The fall-sound event id (the damaging landing plays it).\n");
    src.push_str("pub const SOUND_FALL: [&str; SOUND_BLOCK_COUNT] = [\n");
    for r in &rows {
        src.push_str(&format!("    \"{}\",\n", r.6));
    }
    // THE NAME LOOKUP's own lane: SOUND_NAMES stays in vanilla registration
    // order (the evidence's own order), the lookup rides a SORTED index
    // table (the binary search demands sorted data — the registration
    // order is not alphabetical, the plain search misses)
    let mut sorted: Vec<usize> = (0..names.len()).collect();
    sorted.sort_by(|a, b| names[*a].cmp(&names[*b]));
    src.push_str(
        "];\n\n\
         /// The registry indices sorted by NAME (the binary search's own\n\
         /// lane — SOUND_NAMES itself stays in vanilla registration order).\n\
         pub const SOUND_SORTED: [u16; SOUND_BLOCK_COUNT] = [\n",
    );
    for i in &sorted {
        src.push_str(&format!("    {i},\n"));
    }
    src.push_str(
        "];\n\n\
         /// The registration index of a block name (binary search over the\n\
         /// sorted lane); the parallel arrays index through it. `None`\n\
         /// when the registry does not speak the name.\n\
         pub fn sound_index(name: &str) -> Option<usize> {\n\
         \x20   SOUND_SORTED\n\
         \x20       .binary_search_by(|probe| SOUND_NAMES[*probe as usize].cmp(name))\n\
         \x20       .ok()\n\
         \x20       .map(|pos| SOUND_SORTED[pos] as usize)\n\
         }\n",
    );

    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("block_sounds.rs");
    std::fs::write(&path, src).context("write block_sounds.rs")?;
    println!(
        "block_sounds.rs written: {} blocks -> {}",
        names.len(),
        path.display()
    );
    Ok(path)
}

/// The shared vanilla-boot plumbing: resolve the version json, materialize
/// the client jar + every linux-relevant library as classpath FILES, and
/// resolve the java binary. Returns (java, classpath, client sha1).
fn vanilla_runtime(
    agent: &ureq::Agent,
    cache: &Cache,
    version: &str,
    java_override: Option<&str>,
) -> Result<(PathBuf, std::ffi::OsString, String)> {
    let m = crate::manifest::load(agent)?;
    let entry = m
        .find(version)
        .with_context(|| format!("version {version} not in manifest"))?
        .clone();
    let vj = crate::version::resolve(agent, cache, &entry.url, entry.sha1.as_deref())?;
    let required_java = vj.java_version.as_ref().map(|j| j.major_version).unwrap_or(0);
    println!("[1/4] version {version}: needs java {required_java}");

    let cp_dir = Path::new(".mc2rs").join("classpath");
    std::fs::create_dir_all(&cp_dir)?;

    let dl = &vj.downloads.client;
    let jar_bytes = cache.fetch(
        agent,
        &format!("client-jar:{}", dl.sha1.as_deref().unwrap_or(vj.id.as_str())),
        &dl.url,
    )?;
    util::verify(&jar_bytes, dl.sha1.as_deref(), "client jar")?;
    let client_sha1 = util::sha1_hex(&jar_bytes);
    let client_file = materialize(agent, cache, &cp_dir, "client.jar", dl)?;
    println!(
        "[2/4] client jar materialized ({:.1} MiB, sha1 {})",
        dl.size as f64 / (1024.0 * 1024.0),
        &client_sha1[..8]
    );

    let libs = linux_classpath(&vj);
    let mut cp = client_file.clone().into_os_string();
    for (name, art) in &libs {
        let file_name = art.url.rsplit('/').next().unwrap_or(name);
        let path = materialize(agent, cache, &cp_dir, file_name, art)?;
        cp.push(":");
        cp.push(path.into_os_string());
    }
    println!("[3/4] classpath: {} libraries under {}", libs.len(), cp_dir.display());

    let java = resolve_java(java_override, required_java)?;
    Ok((java, cp, client_sha1))
}

/// Run one single-file dumper through the java source launcher.
fn run_dumper(java: &Path, cp: &std::ffi::OsStr, dump_src: &Path, out_txt: &Path) -> Result<()> {
    let status = Command::new(java)
        .arg("-Xmx2G")
        .arg("-cp")
        .arg(cp)
        .arg(dump_src)
        .arg(out_txt)
        .output()
        .with_context(|| format!("run the dumper with {}", java.display()))?;
    if !status.status.success() {
        let err = String::from_utf8_lossy(&status.stderr);
        let tail: Vec<&str> = err.lines().collect::<Vec<_>>();
        let tail = &tail[tail.len().saturating_sub(12)..];
        bail!("dumper failed:\n{}", tail.join("\n"));
    }
    Ok(())
}

/// The built-in registries whose WIRE ORDER the client's runtime tables
/// must mirror once the pack-version claim stands: a vanilla server
/// assigns synced-registry ids in this built-in registration order, so
/// these are the orders the generated tables pin.
pub const ORDERED_REGISTRIES: [&str; 4] = [
    "minecraft:dimension_type",
    "minecraft:worldgen/biome",
    "minecraft:timeline",
    "minecraft:world_clock",
];

/// Parse the DumpRegistries output (`registry|id|entry_name` lines) into
/// per-registry ordered name lists. Ids must be dense per registry (the
/// built-in registries are dense) and names unique within one.
pub fn parse_registry_dump(text: &str) -> Result<BTreeMap<String, Vec<String>>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let mut it = line.splitn(3, '|');
        let reg = it
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        let id: u32 = it
            .next()
            .unwrap_or("")
            .trim()
            .parse()
            .with_context(|| format!("registry dump line {}: bad id", i + 1))?;
        let name = it
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        if reg.is_empty() || name.is_empty() {
            bail!("registry dump line {}: missing registry or entry name", i + 1);
        }
        let entries = out.entry(reg.clone()).or_default();
        if id != entries.len() as u32 {
            bail!(
                "registry {reg}: ids not dense at index {} (id {id})",
                entries.len()
            );
        }
        if entries.contains(&name) {
            bail!("registry {reg}: duplicate entry {name}");
        }
        entries.push(name);
    }
    Ok(out)
}

/// Generate crates/mc-registry/src/generated/registry_order.rs from the
/// DumpRegistries output: per synced registry, the entry NAMES in vanilla
/// registration (= wire) order — the array index IS the wire id a server
/// that skipped the sync would use. Every ORDERED_REGISTRIES member must
/// appear in the dump; other dumped registries ride the evidence file only.
pub fn emit_registry_order(
    workspace_root: &Path,
    version: &str,
    client_sha1: &str,
    dump_text: &str,
) -> Result<PathBuf> {
    let dump = parse_registry_dump(dump_text)?;
    let mut src = String::with_capacity(16 * 1024);
    src.push_str(&format!(
        "// @generated by `mc2rs-gen registry-order` for Minecraft {version} — DO NOT EDIT BY HAND.\n\
         // Source of truth: the official client jar's REAL bootstrapped built-in registries\n\
         // (client sha1 {client_sha1}), dumped in vanilla registration order by the checked-in\n\
         // DumpRegistries.java. The array index IS the wire id: a vanilla server assigns\n\
         // synced-registry ids in this same built-in order, so these tables are the pack-version\n\
         // claim's evidence layer (a skip-sync server and this client agree on them by construction).\n\
         // Regenerate: `cargo run -p mc2rs-gen -- dump-registries --version {version}` (online)\n\
         // or `mc2rs-gen registry-order --dump <saved registries.txt> --client-sha1 <sha>` (offline).\n\n"
    ));
    for reg in ORDERED_REGISTRIES {
        let Some(names) = dump.get(reg) else {
            bail!("the dump carries no {reg} registry (the built-in boot must have it)");
        };
        let ident = match reg {
            "minecraft:dimension_type" => "DIMENSION_TYPE_ORDER",
            "minecraft:worldgen/biome" => "BIOME_ORDER",
            "minecraft:timeline" => "TIMELINE_ORDER",
            "minecraft:world_clock" => "WORLD_CLOCK_ORDER",
            other => bail!("unmapped registry {other}"),
        };
        let doc = match ident {
            "DIMENSION_TYPE_ORDER" => "The built-in `minecraft:dimension_type` registry in vanilla registration (= wire) order.",
            "BIOME_ORDER" => "The built-in `minecraft:worldgen/biome` registry in vanilla registration (= wire) order.",
            "TIMELINE_ORDER" => "The built-in `minecraft:timeline` registry in vanilla registration (= wire) order.",
            _ => "The built-in `minecraft:world_clock` registry in vanilla registration (= wire) order.",
        };
        src.push_str(&format!("/// {doc}\n"));
        src.push_str(&format!("pub static {ident}: [&str; {}] = [", names.len()));
        for (i, name) in names.iter().enumerate() {
            if i > 0 {
                src.push_str(", ");
            }
            src.push_str(&format!("\"{name}\""));
        }
        src.push_str("];\n\n");
    }
    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("registry_order.rs");
    std::fs::write(&path, src).context("write registry_order.rs")?;
    let total: usize = ORDERED_REGISTRIES
        .iter()
        .filter_map(|r| dump.get(*r).map(|v| v.len()))
        .sum();
    println!(
        "registry_order.rs written: {} registries / {total} entries -> {}",
        ORDERED_REGISTRIES.len(),
        path.display()
    );
    Ok(path)
}

/// The whole arc for the MOB-EFFECT dumper: version -> classpath files ->
/// java -> dumper -> the saved `mob_effects.txt` (the evidence file; the
/// offline `mob-effects` emitter turns it into the generated table). The
/// mob-effect registry is NOT synchronized — the client resolves it
/// locally, so the wire ids are THIS boot's 0-based built-in order.
pub fn dump_mob_effects(
    agent: &ureq::Agent,
    cache: &Cache,
    version: &str,
    workspace_root: &Path,
    java_override: Option<&str>,
) -> Result<PathBuf> {
    let t0 = std::time::Instant::now();
    let (java, cp, _) = vanilla_runtime(agent, cache, version, java_override)?;
    let cp_dir = Path::new(".mc2rs").join("classpath");
    let dump_src = workspace_root.join("crates/mc2rs-gen/dump/DumpMobEffects.java");
    let out_txt = cp_dir.join("mob_effects.txt");
    run_dumper(&java, &cp, &dump_src, &out_txt)?;
    println!(
        "[4/4] mob effects bootstrapped and dumped in {:.1}s -> {}",
        t0.elapsed().as_secs_f32(),
        out_txt.display()
    );
    Ok(out_txt)
}

/// Parse the DumpMobEffects output (`id|name` lines) into the ordered name
/// list. Ids must be dense from 0 (the built-in registries are dense) and
/// names unique — the same honesty checks the registry-order parser runs.
pub fn parse_mob_effect_dump(text: &str) -> Result<Vec<String>> {
    parse_dense_id_name_dump(text, "mob effect")
}

/// The shared `id|name` dump grammar (the mob-effect and item-order
/// lanes' identical wire shape): ids dense 0..n, names unique and
/// non-empty, at least one entry. `what` names the lane in the errors.
pub fn parse_dense_id_name_dump(text: &str, what: &str) -> Result<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let mut it = line.splitn(2, '|');
        let id: u32 = it
            .next()
            .unwrap_or("")
            .trim()
            .parse()
            .with_context(|| format!("{what} dump line {}: bad id", i + 1))?;
        let name = it.next().unwrap_or("").trim().to_string();
        if name.is_empty() {
            bail!("{what}s: missing entry name at line {}", i + 1);
        }
        if id != out.len() as u32 {
            bail!("{what}s: ids not dense at index {} (id {id})", out.len());
        }
        if out.contains(&name) {
            bail!("{what}s: duplicate entry {name}");
        }
        out.push(name);
    }
    anyhow::ensure!(!out.is_empty(), "the dump carries no {what}s");
    Ok(out)
}

/// Generate crates/mc-registry/src/generated/mob_effects.rs from the
/// DumpMobEffects output: the mob-effect NAMES in vanilla built-in
/// registration (= wire) order — the array index IS the 0-based registry
/// id the `holderRegistry(Registries.MOB_EFFECT)` serializers carry. The
/// legacy 1-based wiki ids are not the wire; this table is the evidence.
pub fn emit_mob_effects(
    workspace_root: &Path,
    version: &str,
    client_sha1: &str,
    dump_text: &str,
) -> Result<PathBuf> {
    let names = parse_mob_effect_dump(dump_text)?;
    let mut src = String::with_capacity(4 * 1024);
    src.push_str(&format!(
        "// @generated by `mc2rs-gen mob-effects` for Minecraft {version} — DO NOT EDIT BY HAND.\n\
         // Source of truth: the official client jar's REAL bootstrapped MOB_EFFECT built-in\n\
         // registry (client sha1 {client_sha1}), dumped in vanilla registration order by the\n\
         // checked-in DumpMobEffects.java. The array index IS the 0-based wire id: the\n\
         // holderRegistry(Registries.MOB_EFFECT) serializers (the effect packets' effect\n\
         // lane) ride this built-in order — a negative id throws, the legacy 1-based wiki\n\
         // ids are NOT the wire. NOT a synchronized registry: the client resolves it\n\
         // locally, so the order is this boot's, not the server's sync.\n\
         // Regenerate: `cargo run -p mc2rs-gen -- dump-mob-effects --version {version}` (online)\n\
         // or `mc2rs-gen mob-effects --dump <saved mob_effects.txt> --client-sha1 <sha>` (offline).\n\n"
    ));
    src.push_str(&format!(
        "/// The built-in `minecraft:mob_effect` registry in vanilla registration (= wire) order.\n\
         pub static MOB_EFFECT_ORDER: [&str; {}] = [",
        names.len()
    ));
    for (i, name) in names.iter().enumerate() {
        if i > 0 {
            src.push_str(", ");
        }
        src.push_str(&format!("\"{name}\""));
    }
    src.push_str("];\n");
    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("mob_effects.rs");
    std::fs::write(&path, src).context("write mob_effects.rs")?;
    println!(
        "mob_effects.rs written: {} entries -> {}",
        names.len(),
        path.display()
    );
    Ok(path)
}

/// Generate crates/mc-registry/src/generated/item_order.rs from the
/// DumpItemOrder output: the item NAMES in vanilla built-in registration
/// (= wire) order — the array index IS the wire id the container lanes'
/// item stacks ride. The names ride the PLAIN form (the `minecraft:`
/// prefix stripped — the ITEMS table's own vocabulary, the texture lane's
/// `item/<name>` keys); the dump's full identifiers live in the evidence
/// file this table was emitted from.
pub fn emit_item_order(
    workspace_root: &Path,
    version: &str,
    client_sha1: &str,
    dump_text: &str,
) -> Result<PathBuf> {
    let names = parse_dense_id_name_dump(dump_text, "item")?;
    let mut src = String::with_capacity(24 * 1024);
    src.push_str(&format!(
        "// @generated by `mc2rs-gen item-order` for Minecraft {version} — DO NOT EDIT BY HAND.\n\
         // Source of truth: the official client jar's REAL bootstrapped ITEM built-in\n\
         // registry (client sha1 {client_sha1}), dumped in vanilla registration order by the\n\
         // checked-in DumpItemOrder.java. The array index IS the 0-based wire id: the\n\
         // holderRegistry(Registries.ITEM) serializers (the container lanes' item stacks,\n\
         // the equipment lane) ride this built-in order — NOT the assets table's\n\
         // alphabetical order (that is the item model JSONs' TEXTURE vocabulary).\n\
         // Names ride the PLAIN form (the `minecraft:` prefix stripped — the ITEMS\n\
         // table's own vocabulary; the dump's full identifiers live in the evidence file).\n\
         // NOT a synchronized registry: the client resolves it locally, so the order is\n\
         // this boot's, not the server's sync.\n\
         // Regenerate: `cargo run -p mc2rs-gen -- dump-item-order --version {version}` (online)\n\
         // or `mc2rs-gen item-order --dump <saved item_order.txt> --client-sha1 <sha>` (offline).\n\n"
    ));
    src.push_str(&format!(
        "/// The built-in `minecraft:item` registry in vanilla registration (= wire) order,\n\
         /// the names plain (the `minecraft:` prefix stripped).\n\
         pub static ITEM_WIRE_ORDER: [&str; {}] = [",
        names.len()
    ));
    for (i, full) in names.iter().enumerate() {
        if i > 0 {
            src.push_str(", ");
        }
        let plain = full.strip_prefix("minecraft:").unwrap_or(full);
        src.push_str(&format!("\"{plain}\""));
    }
    src.push_str("];\n");
    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("item_order.rs");
    std::fs::write(&path, src).context("write item_order.rs")?;
    println!(
        "item_order.rs written: {} entries -> {}",
        names.len(),
        path.display()
    );
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn java_version_output_parses_modern_and_legacy() {
        assert_eq!(
            parse_java_version_output("openjdk version \"21.0.12.1\" 2026-08-18\nmore"),
            21
        );
        assert_eq!(parse_java_version_output("openjdk version \"25.0.4.1\""), 25);
        assert_eq!(
            parse_java_version_output("openjdk version \"1.8.0_412\""),
            8,
            "legacy 1.x scheme maps to the second component"
        );
        assert_eq!(parse_java_version_output("garbage"), 0);
    }

    #[test]
    fn library_rules_follow_vanilla_launcher_semantics() {
        use crate::version::{LibraryOs, LibraryRef, LibraryRule};
        let lib = |rules: Option<Vec<LibraryRule>>| LibraryRef {
            name: "lib".into(),
            url: None,
            rules,
            downloads: None,
        };
        let rule = |action: &str, os: Option<&str>| LibraryRule {
            action: action.to_string(),
            os: os.map(|n| LibraryOs { name: Some(n.into()) }),
        };
        // no rules -> unconditional
        assert!(lib(None).allowed_on("linux"));
        // rules present but nothing matches -> rejected
        assert!(!lib(Some(vec![rule("allow", Some("osx"))])).allowed_on("linux"));
        // matching allow -> accepted (os-scoped and global)
        assert!(lib(Some(vec![rule("allow", Some("linux"))])).allowed_on("linux"));
        assert!(lib(Some(vec![rule("allow", None)])).allowed_on("linux"));
        // the LAST matching rule wins
        assert!(
            !lib(Some(vec![
                rule("allow", Some("linux")),
                rule("disallow", Some("linux"))
            ]))
            .allowed_on("linux")
        );
        assert!(
            lib(Some(vec![
                rule("disallow", Some("linux")),
                rule("allow", Some("linux"))
            ]))
            .allowed_on("linux")
        );
        // a non-matching rule never overrides
        assert!(
            lib(Some(vec![
                rule("allow", Some("linux")),
                rule("disallow", Some("osx"))
            ]))
            .allowed_on("linux")
        );
    }

    #[test]
    fn dump_parses_dense_and_collapses_into_block_ranges() {
        let dump = parse_dump(
            "0|minecraft:air|\n1|minecraft:stone|\n2|minecraft:grass_block|snowy=true\n\
             3|minecraft:grass_block|snowy=false\n4|minecraft:dirt|\n",
        )
        .expect("parses");
        assert_eq!(dump.len(), 5);
        // props ride along verbatim (the jar's own words; empty -> None)
        assert_eq!(dump[2].2.as_deref(), Some("snowy=true"));
        assert_eq!(dump[0].2, None);
        let ranges = block_ranges(&dump);
        assert_eq!(
            ranges,
            vec![
                (0, 1, "minecraft:air"),
                (1, 2, "minecraft:stone"),
                (2, 4, "minecraft:grass_block"),
                (4, 5, "minecraft:dirt"),
            ]
        );
        // property runs: adjacent identical props collapse — the prop-less
        // air and stone states merge ACROSS blocks into one run
        let props = props_ranges(&dump);
        assert_eq!(
            props,
            vec![
                (0, 2, ""),
                (2, 3, "snowy=true"),
                (3, 4, "snowy=false"),
                (4, 5, ""),
            ]
        );
        // non-dense input (a gap) is refused — the registry is dense
        assert!(parse_dump("0|minecraft:air|\n5|minecraft:stone|").is_err());
        assert!(parse_dump("x|minecraft:air|").is_err());
        assert!(parse_dump("0||").is_err(), "empty name refused");
    }

    #[test]
    fn emitted_table_answers_binary_search_queries() {
        let dir = std::env::temp_dir().join(format!("mc2rs-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let ws = dir.join("ws");
        let path = emit_block_ids(
            &ws,
            "26.2",
            "abc123",
            "0|minecraft:air|\n1|minecraft:stone|\n2|minecraft:granite|\n",
        )
        .expect("emits");
        let src = std::fs::read_to_string(&path).unwrap();
        assert!(src.contains("pub const STATE_COUNT: u32 = 3;"));
        assert!(src.contains("(1, 2, \"minecraft:stone\")"));
        assert!(src.contains("pub const STATE_PROPS"));
        assert!(src.contains("pub fn props_of"));
        assert!(src.contains("pub fn block_name_of"));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn registry_dump_parses_dense_and_emits_the_order_tables() {
        let dump = "minecraft:dimension_type|0|minecraft:overworld\n\
                    minecraft:dimension_type|1|minecraft:the_nether\n\
                    minecraft:worldgen/biome|0|minecraft:plains\n\
                    minecraft:worldgen/biome|1|minecraft:desert\n\
                    minecraft:world_clock|0|minecraft:overworld\n";
        let parsed = parse_registry_dump(dump).expect("parses");
        assert_eq!(
            parsed["minecraft:dimension_type"],
            vec!["minecraft:overworld", "minecraft:the_nether"]
        );
        // non-dense ids and duplicates are refused — the built-in
        // registries are dense and unique
        assert!(parse_registry_dump("minecraft:timeline|0|a\nminecraft:timeline|2|b\n").is_err());
        assert!(parse_registry_dump("minecraft:timeline|0|a\nminecraft:timeline|1|a\n").is_err());
        assert!(parse_registry_dump("minecraft:timeline|x|a\n").is_err());
        assert!(parse_registry_dump("minecraft:timeline|0|\n").is_err());

        let dir = std::env::temp_dir().join(format!("mc2rs-order-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let ws = dir.join("ws");
        // the timeline registry is in ORDERED_REGISTRIES but missing from
        // this fixture — the emitter must REFUSE (every ordered registry
        // must be present in the evidence)
        assert!(emit_registry_order(&ws, "26.2", "abc123", dump).is_err());
        let full = format!("{dump}minecraft:timeline|0|minecraft:day\n");
        let path = emit_registry_order(&ws, "26.2", "abc123", &full).expect("emits");
        let src = std::fs::read_to_string(&path).unwrap();
        assert!(src.contains("pub static DIMENSION_TYPE_ORDER: [&str; 2]"));
        assert!(src.contains("\"minecraft:overworld\", \"minecraft:the_nether\""));
        assert!(src.contains("pub static BIOME_ORDER: [&str; 2] = [\"minecraft:plains\", \"minecraft:desert\"]"));
        assert!(src.contains("pub static TIMELINE_ORDER: [&str; 1] = [\"minecraft:day\"]"));
        assert!(src.contains("pub static WORLD_CLOCK_ORDER: [&str; 1] = [\"minecraft:overworld\"]"));
        assert!(src.contains("client sha1 abc123"));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn mob_effect_dump_parses_dense_and_emits_the_wire_table() {
        // the round-120 probe's verdict, as a fixture: speed 0 .. night_vision 15
        let dump = "0|minecraft:speed\n1|minecraft:slowness\n15|PLACEHOLDER\n";
        // non-dense (a gap) is refused — the built-in registry is dense
        assert!(parse_mob_effect_dump(dump).is_err());
        assert!(parse_mob_effect_dump("x|minecraft:speed\n").is_err());
        assert!(parse_mob_effect_dump("0|\n").is_err(), "empty name refused");
        assert!(parse_mob_effect_dump("0|a\n1|a\n").is_err(), "duplicate refused");
        assert!(parse_mob_effect_dump("").is_err(), "empty dump refused");

        let dense = "0|minecraft:speed\n1|minecraft:slowness\n2|minecraft:haste\n";
        let parsed = parse_mob_effect_dump(dense).expect("parses");
        assert_eq!(
            parsed,
            vec!["minecraft:speed", "minecraft:slowness", "minecraft:haste"]
        );

        let dir = std::env::temp_dir().join(format!("mc2rs-mfx-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let ws = dir.join("ws");
        let path = emit_mob_effects(&ws, "26.2", "abc123", dense).expect("emits");
        let src = std::fs::read_to_string(&path).unwrap();
        assert!(src.contains("pub static MOB_EFFECT_ORDER: [&str; 3]"));
        assert!(src.contains(
            "\"minecraft:speed\", \"minecraft:slowness\", \"minecraft:haste\""
        ));
        assert!(src.contains("client sha1 abc123"));
        assert!(src.contains("NOT a synchronized registry"));
        std::fs::remove_dir_all(&dir).ok();
    }
}

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

use std::io::Write;
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
fn materialize(
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

/// One entry of the dump: `id|block_name|props`.
fn parse_dump(text: &str) -> Result<Vec<(u32, String)>> {
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
        // props (third field) are not needed for the name mapping — the
        // whole state range of a block collapses into one entry
        out.push((id, name));
    }
    out.sort_unstable_by_key(|(id, _)| *id);
    // density check: ids must tile 0..n exactly (the registry is dense)
    for (i, (id, _)) in out.iter().enumerate() {
        if *id != i as u32 {
            bail!("dump ids not dense at index {i} (id {id})");
        }
    }
    Ok(out)
}

/// Collapse per-state entries into per-block ranges.
fn block_ranges(dump: &[(u32, String)]) -> Vec<(u32, u32, &str)> {
    let mut ranges: Vec<(u32, u32, &str)> = Vec::new();
    for (id, name) in dump {
        match ranges.last_mut() {
            Some((_, hi, last)) if *last == name.as_str() => *hi = id + 1,
            _ => ranges.push((*id, id + 1, name.as_str())),
        }
    }
    ranges
}

/// Generate crates/mc-registry/src/generated/block_ids.rs from dump text.
pub fn emit_block_ids(workspace_root: &Path, version: &str, client_sha1: &str, dump_text: &str) -> Result<PathBuf> {
    let dump = parse_dump(dump_text)?;
    let ranges = block_ranges(&dump);
    let state_count = dump.len() as u32;
    let block_count = ranges.len();

    let mut src = String::with_capacity(128 * 1024);
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

    let reg_dir = workspace_root.join("crates/mc-registry/src/generated");
    std::fs::create_dir_all(&reg_dir)?;
    let path = reg_dir.join("block_ids.rs");
    std::fs::write(&path, src).context("write block_ids.rs")?;
    println!(
        "block_ids.rs written: {state_count} states / {block_count} blocks -> {}",
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
    let dump_src = workspace_root.join("crates/mc2rs-gen/dump/DumpBlockStates.java");
    let out_txt = cp_dir.join("block_states.txt");
    let status = Command::new(&java)
        .arg("-Xmx2G")
        .arg("-cp")
        .arg(&cp)
        .arg(&dump_src)
        .arg(&out_txt)
        .output()
        .with_context(|| format!("run the dumper with {}", java.display()))?;
    if !status.status.success() {
        let err = String::from_utf8_lossy(&status.stderr);
        bail!("dumper failed:\n{}", err.lines().rev().take(12).rev().collect::<Vec<_>>().join("\n"));
    }
    let dump_text = std::fs::read_to_string(&out_txt).context("read dumper output")?;
    println!("[4/4] registry bootstrapped and dumped in {:.1}s", t0.elapsed().as_secs_f32());

    emit_block_ids(workspace_root, version, &client_sha1, &dump_text)
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
            "0|minecraft:air|\n1|minecraft:stone|\n8|minecraft:grass_block|snowy=true\n\
             9|minecraft:grass_block|snowy=false\n10|minecraft:dirt|\n",
        )
        .expect("parses");
        assert_eq!(dump.len(), 5);
        let ranges = block_ranges(&dump);
        assert_eq!(
            ranges,
            vec![
                (0, 1, "minecraft:air"),
                (1, 2, "minecraft:stone"),
                (8, 10, "minecraft:grass_block"),
                (10, 11, "minecraft:dirt"),
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
        assert!(src.contains("pub fn block_name_of"));
        std::fs::remove_dir_all(&dir).ok();
    }
}

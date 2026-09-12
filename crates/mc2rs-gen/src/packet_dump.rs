//! Packet id table generation from the official jar's REAL bootstrapped
//! protocol — for EVERY protocol phase, not just play. The same arc as
//! `dump_block_states`: version json -> client jar (sha1) -> linux
//! libraries -> classpath files -> java source-launcher run of the
//! checked-in dumper — but the dumper (`dump/DumpPacketIds.java`) binds
//! all nine vanilla flows at runtime (play/login/configuration/status
//! clientbound+serverbound, handshake serverbound) and prints each packet
//! type's LIVE wire id from the built dispatch codec's id map. The
//! emitted tables replace the hand-kept wiki-era constants, which were
//! stale for the 26.2 protocol in BOTH waves: the play ids drifted
//! (login 0x27 -> 49) and the configuration clientbound ids drift +1
//! (registry_data 0x06 -> 0x07 — the wiki-era table forgot
//! cookie_request's head slot).

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::util::{self, Cache};

#[derive(Debug, Clone, PartialEq)]
pub struct FlowTable {
    pub prefix: &'static str,
    /// (id, "minecraft:name") — ids dense 0..n, vanilla registration order.
    pub entries: Vec<(u32, String)>,
}

/// Every protocol phase's wire tables, exactly what the runtime dumper
/// emits: play/login/configuration/status as (clientbound, serverbound)
/// pairs and handshake as its single serverbound flow.
#[derive(Debug, Clone, PartialEq)]
pub struct PacketDump {
    pub play: (FlowTable, FlowTable),
    pub login: (FlowTable, FlowTable),
    pub config: (FlowTable, FlowTable),
    pub status: (FlowTable, FlowTable),
    pub handshake: FlowTable,
}

/// The dump line vocabulary -> (field, prefix): the dumper's flow token,
/// where it lands in [`PacketDump`], and the Rust const prefix it emits.
const FLOWS: &[(&str, u8, &str)] = &[
    ("cb", 0, "CB"),
    ("sb", 1, "SB"),
    ("login-cb", 2, "LCB"),
    ("login-sb", 3, "LSB"),
    ("config-cb", 4, "CCB"),
    ("config-sb", 5, "CSB"),
    ("status-cb", 6, "TCB"),
    ("status-sb", 7, "TSB"),
    ("handshake-sb", 8, "HSB"),
];

/// Parse dumper lines "<flow> <id> <name>" (flow: cb/sb = play, plus the
/// login-/config-/status-/handshake- prefixed flows); each flow's ids
/// must be unique, names unique, and dense 0..n (the wire dispatch has no
/// holes), and EVERY flow must be present (the runtime dumper always
/// binds all nine).
pub fn parse_packet_dump(text: &str) -> Result<PacketDump> {
    let mut buckets: Vec<Vec<(u32, String)>> = vec![Vec::new(); FLOWS.len()];
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let mut it = line.splitn(3, ' ');
        let flow = it.next().unwrap_or("");
        let id: u32 = it
            .next()
            .unwrap_or("")
            .parse()
            .with_context(|| format!("dump line {}: bad id", i + 1))?;
        let name = it.next().unwrap_or("").trim().to_string();
        if !name.starts_with("minecraft:") {
            bail!("dump line {}: name {name:?} is not a minecraft identifier", i + 1);
        }
        let Some((_, slot, _)) = FLOWS.iter().find(|(tok, _, _)| *tok == flow) else {
            bail!("dump line {}: unknown flow {flow:?}", i + 1);
        };
        buckets[*slot as usize].push((id, name));
    }
    let mut table = |slot: usize, prefix: &'static str| -> Result<FlowTable> {
        let t = FlowTable { prefix, entries: std::mem::take(&mut buckets[slot]) };
        check_flow(&t)?;
        Ok(t)
    };
    let dump = PacketDump {
        play: (table(0, "CB")?, table(1, "SB")?),
        login: (table(2, "LCB")?, table(3, "LSB")?),
        config: (table(4, "CCB")?, table(5, "CSB")?),
        status: (table(6, "TCB")?, table(7, "TSB")?),
        handshake: table(8, "HSB")?,
    };
    Ok(dump)
}

fn check_flow(table: &FlowTable) -> Result<()> {
    if table.entries.is_empty() {
        bail!("{} flow table is empty — the dumper binds all nine flows", table.prefix);
    }
    let mut ids: Vec<u32> = table.entries.iter().map(|(id, _)| *id).collect();
    ids.sort_unstable();
    for (i, id) in ids.iter().enumerate() {
        if *id != i as u32 {
            bail!(
                "{} flow ids not dense at index {i} (id {id}) — the wire \
                 dispatch has no holes",
                table.prefix
            );
        }
    }
    let mut names: Vec<&str> = table.entries.iter().map(|(_, n)| n.as_str()).collect();
    names.sort_unstable();
    let n = names.len();
    names.dedup();
    if names.len() != n {
        bail!("{} flow has duplicate packet names", table.prefix);
    }
    Ok(())
}

/// "minecraft:chunk_batch_start" + prefix "CB" -> "CB_CHUNK_BATCH_START".
/// Any character outside [A-Z0-9_] (e.g. the `/` in debug/block_value)
/// folds to `_` so every name is a valid Rust const.
fn const_name(prefix: &str, name: &str) -> String {
    let tail = name.trim_start_matches("minecraft:").to_uppercase();
    let safe: String = tail
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    format!("{prefix}_{safe}")
}

/// Generate crates/mc-protocol/src/generated/packet_ids.rs from dump text.
pub fn emit_packet_ids(
    workspace_root: &Path,
    version: &str,
    client_sha1: &str,
    dump_text: &str,
) -> Result<PathBuf> {
    let dump = parse_packet_dump(dump_text)?;
    let phases: [(&str, [(&str, &FlowTable); 2]); 4] = [
        ("play", [("CLIENTBOUND", &dump.play.0), ("SERVERBOUND", &dump.play.1)]),
        ("login", [("CLIENTBOUND", &dump.login.0), ("SERVERBOUND", &dump.login.1)]),
        ("config", [("CLIENTBOUND", &dump.config.0), ("SERVERBOUND", &dump.config.1)]),
        ("status", [("CLIENTBOUND", &dump.status.0), ("SERVERBOUND", &dump.status.1)]),
    ];

    let mut src = String::with_capacity(64 * 1024);
    src.push_str(&format!(
        "// @generated by `mc2rs-gen dump-packet-ids` for Minecraft {version} — DO NOT EDIT BY HAND.\n\
         // Source of truth: the official client jar's REAL bootstrapped\n\
         // protocol (client sha1 {client_sha1}); all nine phase flows dumped\n\
         // from the live dispatch codecs' id maps by the checked-in runtime\n\
         // dumper.\n\
         // Regenerate: `cargo run -p mc2rs-gen -- dump-packet-ids --version {version}`\n\n"
    ));

    let mut flow_list: Vec<(&str, &FlowTable)> = Vec::new();
    for (phase, flows) in phases {
        for (flow, table) in flows {
            flow_list.push((flow, table));
            let count = table.entries.len();
            src.push_str(&format!(
                "/// The {phase} phase's {flow_label} flow: (id, packet name), dense 0..{count}.\n\
                 pub static {phase}_{flow_label}: &[(u32, &str)] = &[\n",
                flow_label = flow.to_uppercase(),
                phase = phase.to_uppercase()
            ));
            for (id, name) in &table.entries {
                src.push_str(&format!("    ({id}, \"{name}\"),\n"));
            }
            src.push_str("];\n\n");
        }
    }
    flow_list.push(("SERVERBOUND", &dump.handshake));
    let count = dump.handshake.entries.len();
    src.push_str(&format!(
        "/// The handshake phase's single SERVERBOUND flow: (id, packet name), dense 0..{count}.\n\
         pub static HANDSHAKE_SERVERBOUND: &[(u32, &str)] = &[\n"
    ));
    for (id, name) in &dump.handshake.entries {
        src.push_str(&format!("    ({id}, \"{name}\"),\n"));
    }
    src.push_str("];\n\n");

    src.push_str("/// Named wire ids (sanitized vanilla names), the compile-time mirror\n");
    src.push_str("/// of the tables above.\n");
    let mut seen_names: std::collections::BTreeMap<String, String> = Default::default();
    for (_, table) in &flow_list {
        for (id, name) in &table.entries {
            let cname = const_name(table.prefix, name);
            if let Some(prev) = seen_names.insert(cname.clone(), name.clone()) {
                bail!(
                    "const name collision: {cname} for both {prev:?} and {name:?}"
                );
            }
            src.push_str(&format!("pub const {}: i32 = {id}; // {name}\n", cname));
        }
        src.push('\n');
    }

    let proto_dir = workspace_root.join("crates/mc-protocol/src/generated");
    std::fs::create_dir_all(&proto_dir)?;
    let path = proto_dir.join("packet_ids.rs");
    std::fs::write(&path, src).context("write packet_ids.rs")?;
    println!(
        "packet_ids.rs written: play {}/{}, login {}/{}, config {}/{}, status {}/{}, handshake {} -> {}",
        dump.play.0.entries.len(), dump.play.1.entries.len(),
        dump.login.0.entries.len(), dump.login.1.entries.len(),
        dump.config.0.entries.len(), dump.config.1.entries.len(),
        dump.status.0.entries.len(), dump.status.1.entries.len(),
        dump.handshake.entries.len(),
        path.display()
    );
    Ok(path)
}

/// The whole arc: version -> classpath files -> java -> dumper -> table.
pub fn dump_packet_ids(
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
    let client_file = crate::runtime_dump::materialize(agent, cache, &cp_dir, "client.jar", dl)?;

    let libs = crate::runtime_dump::linux_classpath(&vj);
    let mut cp = client_file.clone().into_os_string();
    for (name, art) in &libs {
        let file_name = art.url.rsplit('/').next().unwrap_or(name);
        let path = crate::runtime_dump::materialize(agent, cache, &cp_dir, file_name, art)?;
        cp.push(":");
        cp.push(path.into_os_string());
    }
    println!("[3/4] classpath: {} libraries under {}", libs.len(), cp_dir.display());

    let java = crate::runtime_dump::resolve_java(java_override, required_java)?;
    let dump_src = workspace_root.join("crates/mc2rs-gen/dump/DumpPacketIds.java");
    let out_txt = cp_dir.join("packet_ids.txt");
    let status = std::process::Command::new(&java)
        .arg("-Xmx2G")
        .arg("-cp")
        .arg(&cp)
        .arg(&dump_src)
        .arg(&out_txt)
        .output()
        .with_context(|| format!("run the dumper with {}", java.display()))?;
    if !status.status.success() {
        let err = String::from_utf8_lossy(&status.stderr);
        let tail: Vec<&str> = err.lines().collect::<Vec<_>>();
        let tail = &tail[tail.len().saturating_sub(12)..];
        bail!("dumper failed:\n{}", tail.join("\n"));
    }
    let dump_text = std::fs::read_to_string(&out_txt).context("read dumper output")?;
    println!("[4/4] all nine phase protocols bound and dumped in {:.1}s", t0.elapsed().as_secs_f32());

    emit_packet_ids(workspace_root, version, &client_sha1, &dump_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    // the fixture is DENSE (ids tile 0..n per flow) — exactly the shape
    // real dumps carry; a sparse sample must be rejected by the check
    const SAMPLE: &str = "\
cb 0 minecraft:bundle_delimiter
cb 1 minecraft:add_entity
cb 2 minecraft:chunk_batch_start
cb 3 minecraft:level_chunk_with_light
cb 4 minecraft:login
cb 5 minecraft:respawn
sb 0 minecraft:accept_teleportation
sb 1 minecraft:pong
login-cb 0 minecraft:login_disconnect
login-cb 1 minecraft:hello
login-cb 2 minecraft:login_finished
login-cb 3 minecraft:login_compression
login-sb 0 minecraft:hello
login-sb 1 minecraft:key
login-sb 2 minecraft:custom_query_answer
login-sb 3 minecraft:login_acknowledged
config-cb 0 minecraft:cookie_request
config-cb 1 minecraft:custom_payload
config-cb 2 minecraft:disconnect
config-cb 3 minecraft:finish_configuration
config-cb 4 minecraft:keep_alive
config-cb 5 minecraft:ping
config-cb 6 minecraft:reset_chat
config-cb 7 minecraft:registry_data
config-sb 0 minecraft:client_information
config-sb 1 minecraft:cookie_response
config-sb 2 minecraft:custom_payload
config-sb 3 minecraft:finish_configuration
status-cb 0 minecraft:status_response
status-cb 1 minecraft:pong_response
status-sb 0 minecraft:status_request
status-sb 1 minecraft:ping_request
handshake-sb 0 minecraft:intention
";

    #[test]
    fn dump_parses_into_dense_phase_tables() {
        let d = parse_packet_dump(SAMPLE).expect("parses");
        assert_eq!(d.play.0.entries.len(), 6);
        assert_eq!(d.play.1.entries.len(), 2);
        assert_eq!(d.login.0.entries.len(), 4);
        assert_eq!(d.login.1.entries.len(), 4);
        assert_eq!(d.config.0.entries.len(), 8);
        assert_eq!(d.config.1.entries.len(), 4);
        assert_eq!(d.status.0.entries.len(), 2);
        assert_eq!(d.status.1.entries.len(), 2);
        assert_eq!(d.handshake.entries.len(), 1);
        assert_eq!(d.play.0.entries[5], (5, "minecraft:respawn".to_string()));
        assert_eq!(d.login.1.entries[3], (3, "minecraft:login_acknowledged".to_string()));
        assert_eq!(d.config.0.entries[7], (7, "minecraft:registry_data".to_string()));
        assert_eq!(d.handshake.entries[0], (0, "minecraft:intention".to_string()));
        assert_eq!(d.config.0.prefix, "CCB");
        assert_eq!(d.status.0.prefix, "TCB");
    }

    #[test]
    fn const_names_sanitize_vanilla_names() {
        assert_eq!(const_name("CB", "minecraft:chunk_batch_start"), "CB_CHUNK_BATCH_START");
        assert_eq!(const_name("SB", "minecraft:pong"), "SB_PONG");
        assert_eq!(const_name("CCB", "minecraft:registry_data"), "CCB_REGISTRY_DATA");
        assert_eq!(const_name("HSB", "minecraft:intention"), "HSB_INTENTION");
    }

    #[test]
    fn sparse_or_duplicate_or_partial_flows_are_rejected() {
        let sparse = SAMPLE.replace("cb 1 ", "cb 3 ");
        assert!(parse_packet_dump(&sparse).is_err(), "density check");
        let dup = format!("{SAMPLE}cb 46 minecraft:level_chunk_with_light\n");
        assert!(parse_packet_dump(&dup).is_err(), "name uniqueness");
        let bad_flow = format!("{SAMPLE}xx 5 minecraft:stone\n");
        assert!(parse_packet_dump(&bad_flow).is_err(), "flow whitelist");
        let empty = "";
        assert!(parse_packet_dump(empty).is_err(), "empty dump");
        // a missing flow (the dumper always binds all nine) is rejected
        let partial: String = SAMPLE.lines().filter(|l| !l.starts_with("status-sb")).collect::<Vec<_>>().join("\n");
        assert!(parse_packet_dump(&partial).is_err(), "every flow must be present");
    }

    #[test]
    fn emitted_source_carries_all_phase_tables_and_consts() {
        let dir = std::env::temp_dir().join(format!("mc2rs-pids-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = emit_packet_ids(&dir, "26.2", "deadbeef", SAMPLE).expect("emits");
        let src = std::fs::read_to_string(&path).unwrap();
        for stat in [
            "PLAY_CLIENTBOUND", "PLAY_SERVERBOUND", "LOGIN_CLIENTBOUND", "LOGIN_SERVERBOUND",
            "CONFIG_CLIENTBOUND", "CONFIG_SERVERBOUND", "STATUS_CLIENTBOUND",
            "STATUS_SERVERBOUND", "HANDSHAKE_SERVERBOUND",
        ] {
            assert!(src.contains(&format!("pub static {stat}: &[(u32, &str)]")), "{stat} static");
        }
        assert!(src.contains("pub const CB_RESPAWN: i32 = 5; // minecraft:respawn"));
        assert!(src.contains("pub const LSB_LOGIN_ACKNOWLEDGED: i32 = 3; // minecraft:login_acknowledged"));
        assert!(src.contains("pub const CCB_REGISTRY_DATA: i32 = 7; // minecraft:registry_data"));
        assert!(src.contains("pub const TSB_PING_REQUEST: i32 = 1; // minecraft:ping_request"));
        assert!(src.contains("pub const HSB_INTENTION: i32 = 0; // minecraft:intention"));
        assert!(src.contains("client sha1 deadbeef"));
        std::fs::remove_dir_all(&dir).ok();
    }
}

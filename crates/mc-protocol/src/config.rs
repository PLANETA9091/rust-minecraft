//! Configuration (protocol state 3): the phase both sides enter after
//! Login Success (via the serverbound Login Acknowledged). The vanilla
//! client introduces itself (Client Information + the `minecraft:brand`
//! plugin message), answers Keep Alive / Ping probes, Select Known Packs
//! and Cookie Requests, skips the bulk feature/resource-pack traffic and
//! acknowledges the server's Finish Configuration — then the play state
//! begins.
//!
//! Runtime-true shapes for 1.20.5+ / 26.x (the wire IDS are GENERATED —
//! `mc2rs-gen dump-packet-ids` binds the vanilla ConfigurationProtocols
//! and dumps the live dispatch codec's id map; the wiki-era table they
//! replaced had drifted: every clientbound id from disconnect onward was
//! one LOW, the wiki table forgetting cookie_request's head slot —
//! registry_data is 0x07 on the real 26.2 wire, not 0x06):
//! - Login Acknowledged (serverbound login 0x03): empty payload — from the
//!   NEXT packet on, both sides speak configuration.
//! - Client Information (serverbound config 0x00): javap-proven 26.2
//!   shape — ServerboundClientInformationPacket delegates to
//!   ClientInformation.write: Locale (String 16), View Distance (BYTE,
//!   not a VarInt), Chat Mode (VarInt enum 0 = enabled), Chat Colors
//!   (Bool), Displayed Skin Parts (Byte 0x7F = all), Main Hand (VarInt
//!   1 = right), Disable Text Filtering (Bool), Allow Server Listings
//!   (Bool), Particle Status (VarInt enum 0 = full) — the wiki-era
//!   encoder was one field SHORT and used the wrong primitive for the
//!   view distance.
//! - Registry Data (clientbound 0x07, one registry per packet):
//!   javap-proven 26.2 shape — ClientboundRegistryDataPacket is the
//!   record (ResourceKey registry, List<PackedRegistryEntry>):
//!   Identifier + VarInt count (max 256) + per entry { Identifier id,
//!   optional network NBT tag }. The wiki-era "Identifier + whole NBT"
//!   shape would have read the entry count as an NBT root and FAILED on
//!   a real server — the same self-consistent-mock mask the id drift
//!   had. The `minecraft:dimension_type` registry is DECODED — every
//!   entry's optional tag IS the dimension element (min_y, height at
//!   the top level, the name from the entry id); every other registry
//!   (biomes, worldgen, banners...) is walked and skipped.
//! - Plugin Message (serverbound config 0x02): Channel (String) + Data —
//!   the brand channel carries one UTF string ("mc2rs").
//! - Keep Alive (clientbound config 0x04 / serverbound 0x04): i64 id,
//!   echoed verbatim.
//! - Ping/Pong (clientbound 0x05 / serverbound 0x05): i32 id, echoed.
//! - Disconnect (clientbound config 0x02): chat JSON reason.
//! - Finish Configuration (clientbound 0x03) → Acknowledge Finish
//!   Configuration (serverbound 0x03): both empty; after the ack the play
//!   state begins.
//! - Select Known Packs (clientbound 0x0E → serverbound 0x07, both
//!   GENERATED): javap-proven 26.2 shape — a list of KnownPack records,
//!   each THREE UTF STRINGS (namespace, id, version — the version is a
//!   string, not a VarInt; `ByteBufCodecs.list(64)` caps the serverbound
//!   reply). The vanilla server BLOCKS on this reply before finishing
//!   configuration — an unanswered packet stalls a real join until the
//!   30s timeout kicks it. The client replies with what it truly knows:
//!   an EMPTY list (claiming a pack version we have not extracted would
//!   make the server skip registry traffic we never received; empty
//!   means the server streams its registries, which the Registry Data
//!   parser already walks).
//! - Cookie Request (clientbound 0x00 → serverbound Cookie Response 0x01,
//!   both GENERATED): javap-proven 26.2 shapes — the request is one
//!   Identifier key; the response is the key echoed + a nullable
//!   byte-array payload (`ByteBufCodecs.byteArray(5120)` under the
//!   nullable). The vanilla client ALWAYS answers — `Map.get` returning
//!   null encodes as the absent payload — and the client stores no
//!   server cookies, so the reply is key + Bool 0.
//! - Everything else (Feature Flags, Resource Pack Push/Pop, Custom
//!   Report Details, Server Links) is read and SKIPPED — the client has
//!   no data-pack or resource-pack surface yet; the vanilla server
//!   finishes configuration fine without answering those.

use std::io::{Read, Write};

use anyhow::{Context, Result};

use crate::framing::{encode_handshake, PacketConn};
use crate::login::{login_conn, LoginOutcome};
use crate::nbt::{read_network_nbt, Nbt};
use crate::varint::{read_string, read_varint, write_string, write_varint};
use crate::{DEFAULT_TIMEOUT_SECS, NEXT_STATE_LOGIN};

/// Serverbound login packet id (the state switch happens at the server),
/// GENERATED from the official jar's real login protocol.
pub mod loginbound {
    pub use crate::generated::packet_ids::LSB_LOGIN_ACKNOWLEDGED as LOGIN_ACKNOWLEDGED;
}

/// Serverbound configuration packet ids, GENERATED from the official
/// jar's real configuration protocol (see the module notes for the drift
/// story). The vanilla names moved on (plugin_message → custom_payload,
/// acknowledge_finish → finish_configuration); the public names are the
/// client's own vocabulary, aliased onto the generated truth.
pub mod serverbound {
    pub use crate::generated::packet_ids::{
        CSB_CLIENT_INFORMATION as CLIENT_INFORMATION,
        CSB_COOKIE_RESPONSE as COOKIE_RESPONSE,
        CSB_CUSTOM_PAYLOAD as PLUGIN_MESSAGE,
        CSB_FINISH_CONFIGURATION as ACKNOWLEDGE_FINISH, CSB_KEEP_ALIVE as KEEP_ALIVE,
        CSB_PONG as PONG, CSB_SELECT_KNOWN_PACKS as SELECT_KNOWN_PACKS,
    };
}

/// Clientbound configuration packet ids, GENERATED from the official
/// jar's real configuration protocol — the ids the wiki-era table had
/// ALL one low (disconnect 0x01 → 0x02, finish 0x02 → 0x03, keep alive
/// 0x03 → 0x04, ping 0x04 → 0x05, registry data 0x06 → 0x07): the
/// vanilla flow grew cookie_request at its head and everything after it
/// shifted. A real 26.2 join would have failed to recognize ANY of
/// these packets; the e2e mocks were self-consistent with the old table
/// and masked it (the same mask the play-id drift had).
pub mod clientbound {
    pub use crate::generated::packet_ids::{
        CCB_COOKIE_REQUEST as COOKIE_REQUEST, CCB_DISCONNECT as DISCONNECT,
        CCB_FINISH_CONFIGURATION as FINISH, CCB_KEEP_ALIVE as KEEP_ALIVE,
        CCB_PING as PING, CCB_REGISTRY_DATA as REGISTRY_DATA,
        CCB_SELECT_KNOWN_PACKS as SELECT_KNOWN_PACKS,
    };
}

/// One registered dimension type: the world's vertical layout, decoded
/// from the `minecraft:dimension_type` registry the server streams during
/// configuration (a real vanilla server sends overworld -64/384, nether
/// 0/256, end 0/256).
#[derive(Debug, Clone, PartialEq)]
pub struct DimensionTypeEntry {
    /// Registry key (e.g. `minecraft:overworld`).
    pub name: String,
    /// World Y of the bottom section.
    pub min_y: i32,
    /// Total height in blocks (a multiple of 16).
    pub height: i32,
}

/// The registry knowledge the client retained from the configuration
/// phase. `Default` is the honest empty table — a server that never sent
/// Registry Data yields no entries.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RegistryData {
    pub dimension_types: Vec<DimensionTypeEntry>,
    /// The `minecraft:worldgen/biome` entries in ARRIVAL ORDER — the
    /// index IS the network id the chunk section biome palettes
    /// reference (the vanilla synced-registry contract, the same order
    /// the `dimension_type_id` index already relies on through
    /// CommonPlayerSpawnInfo). Names only: the tags (temperature,
    /// effects, carvers...) are walked and skipped — nothing consumes
    /// them yet.
    pub biomes: Vec<String>,
}

impl RegistryData {
    /// The vertical layout of one dimension, by registry key.
    pub fn dimension(&self, name: &str) -> Option<&DimensionTypeEntry> {
        self.dimension_types.iter().find(|d| d.name == name)
    }

    /// The registry name of one synced biome id (the palette values in
    /// a chunk section resolve through this) — `None` for an id the
    /// server never streamed (the honest "unknown biome").
    pub fn biome_name(&self, id: u32) -> Option<&str> {
        self.biomes.get(id as usize).map(|s| s.as_str())
    }

    /// Absorb one decoded Registry Data packet (vanilla streams one
    /// registry per packet; entries append in arrival order).
    fn merge(&mut self, other: RegistryData) {
        self.dimension_types.extend(other.dimension_types);
        self.biomes.extend(other.biomes);
    }
}

/// One pack identity the server offers and the client may claim to know:
/// the javap-proven 26.2 `KnownPack` record — THREE UTF STRINGS on the
/// wire (the version is a string like "26.2", NOT a VarInt).
#[derive(Debug, Clone, PartialEq)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

/// The brand the vanilla server reports through its player list and logs
/// (the vanilla client sends "vanilla", modded clients their loader name).
pub const BRAND: &str = "mc2rs";

/// How the configuration phase ended.
#[derive(Debug, Clone, PartialEq)]
pub enum PlayOutcome {
    /// Finish Configuration acknowledged — the play state begins.
    Joined {
        /// The server-assigned profile UUID (equals the requested one in
        /// offline mode).
        uuid: [u8; 16],
        username: String,
        /// The registry knowledge retained during the phase (the
        /// dimension-type vertical layout table).
        registry: RegistryData,
    },
    /// The server refused the login (Login Disconnect) or kicked the
    /// client during configuration (config Disconnect); carries the
    /// flattened chat-component reason.
    Disconnected { reason: String },
}

/// The vanilla-shaped client introduction (Client Information defaults a
/// fresh vanilla install sends: en_us locale, all skin parts, right hand).
pub struct ClientInfo {
    pub locale: String,
    /// Wire type: a single BYTE (vanilla writeByte).
    pub view_distance: i32,
    /// 0 = enabled, 1 = commands only, 2 = hidden.
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub skin_parts: u8,
    /// 0 = left, 1 = right.
    pub main_hand: i32,
    pub disable_text_filtering: bool,
    pub allow_server_listings: bool,
    /// 0 = all, 1 = decreased, 2 = minimal — the ninth ClientInformation
    /// field the wiki-era shape never knew about.
    pub particle_status: i32,
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self {
            locale: "en_us".to_string(),
            view_distance: 8,
            chat_mode: 0,
            chat_colors: true,
            skin_parts: 0x7F,
            main_hand: 1,
            disable_text_filtering: false,
            allow_server_listings: true,
            particle_status: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// packet builders
// ---------------------------------------------------------------------------

/// Login Acknowledged (serverbound LOGIN 0x03): empty payload.
pub fn encode_login_acknowledged() -> Vec<u8> {
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, loginbound::LOGIN_ACKNOWLEDGED);
    p
}

/// Client Information (serverbound config 0x00), the javap-proven 26.2
/// field order and primitives: locale String, view distance BYTE, chat
/// mode VarInt enum, chat colors Bool, skin parts Byte, main hand VarInt
/// enum, text filtering Bool, server listings Bool, particle status
/// VarInt enum.
pub fn encode_client_information(info: &ClientInfo) -> Vec<u8> {
    let mut p = Vec::with_capacity(24);
    write_varint(&mut p, serverbound::CLIENT_INFORMATION);
    write_string(&mut p, &info.locale);
    p.push(info.view_distance as u8); // writeByte, NOT a varint
    write_varint(&mut p, info.chat_mode);
    p.push(u8::from(info.chat_colors));
    p.push(info.skin_parts);
    write_varint(&mut p, info.main_hand);
    p.push(u8::from(info.disable_text_filtering));
    p.push(u8::from(info.allow_server_listings));
    write_varint(&mut p, info.particle_status);
    p
}

/// Plugin Message (serverbound config 0x02) — the brand channel carries a
/// single UTF string payload.
pub fn encode_plugin_brand(brand: &str) -> Vec<u8> {
    let mut p = Vec::with_capacity(24 + brand.len());
    write_varint(&mut p, serverbound::PLUGIN_MESSAGE);
    write_string(&mut p, "minecraft:brand");
    // the brand payload is itself a length-prefixed string
    write_string(&mut p, brand);
    p
}

/// Keep Alive response (serverbound config 0x04): the i64 id verbatim.
pub fn encode_config_keep_alive_response(id: i64) -> Vec<u8> {
    let mut p = Vec::with_capacity(10);
    write_varint(&mut p, serverbound::KEEP_ALIVE);
    p.extend_from_slice(&id.to_be_bytes());
    p
}

/// Pong (serverbound config 0x05): the i32 id verbatim.
pub fn encode_config_pong(id: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(6);
    write_varint(&mut p, serverbound::PONG);
    p.extend_from_slice(&id.to_be_bytes());
    p
}

/// Acknowledge Finish Configuration (serverbound config 0x03): empty.
pub fn encode_config_ack() -> Vec<u8> {
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, serverbound::ACKNOWLEDGE_FINISH);
    p
}

/// Select Known Packs reply (serverbound config 0x07 — generated): the
/// javap-proven 26.2 shape — VarInt count (`ByteBufCodecs.list(64)`, the
/// vanilla server's own cap) + per pack THREE UTF strings. The honest
/// client answer is `&[]`: we claim nothing, so the server streams its
/// registries instead of assuming we have them (claiming an unverified
/// pack version would make a real server skip traffic we never got).
pub fn encode_select_known_packs_reply(known: &[KnownPack]) -> Result<Vec<u8>> {
    anyhow::ensure!(
        known.len() <= 64,
        "{} known packs exceeds the vanilla serverbound cap of 64",
        known.len()
    );
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, serverbound::SELECT_KNOWN_PACKS);
    write_varint(&mut p, known.len() as i32);
    for k in known {
        write_string(&mut p, &k.namespace);
        write_string(&mut p, &k.id);
        write_string(&mut p, &k.version);
    }
    Ok(p)
}

/// Cookie Response (serverbound config 0x01 — generated): the javap-proven
/// 26.2 shape — Identifier key + a NULLABLE byte-array payload
/// (`ByteBufCodecs.byteArray(5120)` under the nullable). `None` is what
/// the vanilla client sends when it has no stored cookie for the key —
/// `Map.get` returning null encodes as Bool false — and the client
/// stores no server cookies, so the driver always passes `None`.
pub fn encode_cookie_response(key: &str, payload: Option<&[u8]>) -> Result<Vec<u8>> {
    if let Some(bytes) = payload {
        anyhow::ensure!(
            bytes.len() <= 5120,
            "cookie payload of {} bytes exceeds the vanilla cap of 5120",
            bytes.len()
        );
    }
    let mut p = Vec::with_capacity(4 + key.len());
    write_varint(&mut p, serverbound::COOKIE_RESPONSE);
    write_string(&mut p, key);
    match payload {
        None => p.push(0), // nullable absent — the vanilla no-cookie reply
        Some(bytes) => {
            p.push(1);
            write_varint(&mut p, bytes.len() as i32);
            p.extend_from_slice(bytes);
        }
    }
    Ok(p)
}

// ---------------------------------------------------------------------------
// packet parsers (full payload including the packet id)
// ---------------------------------------------------------------------------

/// Parse Keep Alive (clientbound config 0x04 — generated): i64 id.
pub fn parse_config_keep_alive(payload: &[u8]) -> Result<i64> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::KEEP_ALIVE,
        "not a config keep-alive packet (0x{id:02X})"
    );
    anyhow::ensure!(c.len() >= 8, "config keep-alive truncated");
    Ok(i64::from_be_bytes(c[..8].try_into()?))
}

/// Parse Ping (clientbound config 0x05 — generated): i32 id.
pub fn parse_config_ping(payload: &[u8]) -> Result<i32> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::PING, "not a config ping packet (0x{id:02X})");
    anyhow::ensure!(c.len() >= 4, "config ping truncated");
    Ok(i32::from_be_bytes(c[..4].try_into()?))
}

/// Parse Disconnect (clientbound config 0x02 — generated): chat JSON
/// reason, flattened to plain text (raw string kept when the JSON does
/// not parse).
pub fn parse_config_disconnect(payload: &[u8]) -> Result<String> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::DISCONNECT,
        "not a config disconnect packet (0x{id:02X})"
    );
    let reason = read_string(&mut c)?;
    let flat = serde_json::from_str::<serde_json::Value>(&reason)
        .map(|v| crate::status::extract_text(&v))
        .unwrap_or(reason);
    Ok(flat)
}

/// Parse Registry Data (clientbound config 0x07 — generated): the
/// javap-proven 26.2 shape — registry Identifier + VarInt entry count
/// (vanilla readCount max 256) + per entry { Identifier id, optional
/// network NBT tag }. `minecraft:dimension_type` is decoded (the entry
/// tag IS the dimension element: min_y/height at the top level, the
/// name from the entry id) and `minecraft:worldgen/biome` is RETAINED
/// as the id -> name table the chunk biome palettes resolve through
/// (the entry tags are walked and skipped); every other registry's
/// entries are walked and dropped. Strict on the decoded registry: an
/// entry WITH a tag missing min_y/height is a protocol drift worth
/// erroring on, the same way vanilla's own registry loader refuses
/// malformed data; an entry with NO tag carries nothing decodable and
/// is skipped.
pub fn parse_registry_data(payload: &[u8]) -> Result<RegistryData> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::REGISTRY_DATA,
        "not a registry data packet (0x{id:02X})"
    );
    let registry_name = read_string(&mut c)?;
    let count = read_varint(&mut c)?;
    anyhow::ensure!(
        count <= 256,
        "registry {registry_name} carries {count} entries (vanilla max 256)"
    );
    let mut out = RegistryData::default();
    for _ in 0..count {
        let entry_id = read_string(&mut c)?;
        anyhow::ensure!(
            !c.is_empty(),
            "registry {registry_name} entry {entry_id} truncated (tag flag)"
        );
        let present = c[0] != 0;
        c = &c[1..];
        if registry_name == "minecraft:worldgen/biome" {
            // EVERY biome entry occupies one id slot — record the name
            // in arrival order whether or not a tag rode along (a tag
            // is walked and skipped: nothing consumes it yet)
            out.biomes.push(entry_id);
            if present {
                crate::play::skip_network_nbt(&mut c)
                    .with_context(|| format!("skip {registry_name} entry nbt"))?;
            }
            continue;
        }
        if !present {
            // an entry without data: nothing to decode, honestly skipped
            continue;
        }
        if registry_name != "minecraft:dimension_type" {
            crate::play::skip_network_nbt(&mut c)
                .with_context(|| format!("skip {registry_name} entry {entry_id} nbt"))?;
            continue;
        }
        let element = read_network_nbt(&mut c)
            .with_context(|| format!("decode dimension_type {entry_id} element nbt"))?;
        let min_y = element
            .get("min_y")
            .and_then(Nbt::as_i32)
            .with_context(|| format!("dimension_type {entry_id} without min_y"))?;
        let height = element
            .get("height")
            .and_then(Nbt::as_i32)
            .with_context(|| format!("dimension_type {entry_id} without height"))?;
        out.dimension_types.push(DimensionTypeEntry { name: entry_id, min_y, height });
    }
    anyhow::ensure!(
        c.is_empty(),
        "trailing bytes after the {registry_name} registry entries"
    );
    Ok(out)
}

/// Parse Select Known Packs (clientbound config 0x0E — generated): the
/// javap-proven 26.2 shape — VarInt pack count + per pack THREE UTF
/// strings (namespace, id, version — the version is a STRING, not a
/// VarInt). Vanilla's clientbound list uses the default collection cap
/// (no small bound in the codec); real configuration flows carry a
/// handful, so anything past 1024 is treated as the drift it is rather
/// than allocated for.
pub fn parse_select_known_packs(payload: &[u8]) -> Result<Vec<KnownPack>> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SELECT_KNOWN_PACKS,
        "not a select-known-packs packet (0x{id:02X})"
    );
    let count = read_varint(&mut c)?;
    anyhow::ensure!(
        (0..=1024).contains(&count),
        "select-known-packs carries {count} packs (past any sane flow)"
    );
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let namespace = read_string(&mut c)?;
        let id = read_string(&mut c)?;
        let version = read_string(&mut c)?;
        out.push(KnownPack { namespace, id, version });
    }
    anyhow::ensure!(c.is_empty(), "trailing bytes after the known packs");
    Ok(out)
}

/// Parse Cookie Request (clientbound config 0x00 — generated): the
/// javap-proven 26.2 shape — one Identifier key, nothing else.
pub fn parse_cookie_request(payload: &[u8]) -> Result<String> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::COOKIE_REQUEST,
        "not a cookie request packet (0x{id:02X})"
    );
    let key = read_string(&mut c)?;
    anyhow::ensure!(c.is_empty(), "trailing bytes after the cookie key");
    Ok(key)
}

// ---------------------------------------------------------------------------
// the configuration driver
// ---------------------------------------------------------------------------

/// Answer the server's configuration probes until Finish Configuration is
/// acknowledged. Registry Data is decoded for the dimension-type vertical
/// layout (everything else in it — other registries — is skipped); Select
/// Known Packs gets the honest empty reply and Cookie Requests the
/// no-cookie one (both BLOCK a real vanilla server otherwise);
/// feature/resource-pack traffic is skipped wholesale; a config
/// Disconnect surfaces as `PlayOutcome::Disconnected`.
fn run_configuration<S: Read + Write>(
    conn: &mut PacketConn<S>,
    uuid: [u8; 16],
    username: String,
) -> Result<PlayOutcome> {
    let mut registry = RegistryData::default();
    loop {
        let pkt = conn.read_packet().context("read configuration packet")?;
        anyhow::ensure!(!pkt.is_empty(), "empty configuration packet");
        let mut c: &[u8] = &pkt;
        let id = read_varint(&mut c)?;
        match id {
            clientbound::KEEP_ALIVE => {
                let probe = parse_config_keep_alive(&pkt)?;
                conn.send_packet(&encode_config_keep_alive_response(probe))?;
            }
            clientbound::PING => {
                let probe = parse_config_ping(&pkt)?;
                conn.send_packet(&encode_config_pong(probe))?;
            }
            clientbound::REGISTRY_DATA => {
                registry.merge(parse_registry_data(&pkt)?);
            }
            clientbound::SELECT_KNOWN_PACKS => {
                // validated, then answered with what we truly know:
                // nothing beyond our own built-in (an empty list keeps
                // the server streaming its registries — the safe path)
                parse_select_known_packs(&pkt)?;
                conn.send_packet(&encode_select_known_packs_reply(&[])?)?;
            }
            clientbound::COOKIE_REQUEST => {
                let key = parse_cookie_request(&pkt)?;
                // the vanilla client answers unconditionally; with no
                // stored cookie the payload is the absent one (Bool 0)
                conn.send_packet(&encode_cookie_response(&key, None)?)?;
            }
            clientbound::FINISH => {
                conn.send_packet(&encode_config_ack())?;
                return Ok(PlayOutcome::Joined { uuid, username, registry });
            }
            clientbound::DISCONNECT => {
                let reason = parse_config_disconnect(&pkt)?;
                return Ok(PlayOutcome::Disconnected { reason });
            }
            // feature flags / resource packs / report details /
            // server links — read and skip
            _ => {}
        }
    }
}

/// Handshake + offline login + full configuration phase on an ALREADY
/// open packet connection (play.rs continues on the same wire after the
/// finish-configuration ack — the vanilla client never reopens the
/// socket between the phases).
pub fn join_play_conn<S: Read + Write>(
    conn: &mut PacketConn<S>,
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<PlayOutcome> {
    conn.send_packet(&encode_handshake(protocol_version, host, port, NEXT_STATE_LOGIN))?;
    let uuid = crate::login::offline_uuid(username);
    match login_conn(conn, username, &uuid)? {
        LoginOutcome::Disconnected { reason } => Ok(PlayOutcome::Disconnected { reason }),
        LoginOutcome::Success { .. } => {
            // switch both sides to configuration
            conn.send_packet(&encode_login_acknowledged())?;
            // the vanilla client's mandatory introductions
            conn.send_packet(&encode_client_information(&ClientInfo::default()))?;
            conn.send_packet(&encode_plugin_brand(BRAND))?;
            run_configuration(conn, uuid, username.to_string())
        }
    }
}

/// Handshake + offline login + full configuration phase on one connection
/// (the vanilla client never reopens the socket between the phases).
pub fn join_play<S: Read + Write>(
    stream: S,
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<PlayOutcome> {
    let mut conn = PacketConn::new(stream);
    join_play_conn(&mut conn, host, port, protocol_version, username)
}

/// Connect + full join (login + configuration) with the vanilla offline
/// profile UUID for `username`.
pub fn connect_and_join_play(
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<PlayOutcome> {
    let addr = format!("{host}:{port}");
    let timeout = std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS);
    let stream = std::net::TcpStream::connect(&addr).with_context(|| format!("tcp connect to {addr}"))?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    join_play(stream, host, port, protocol_version, username)
}

/// Shared e2e mock fixtures. The same-crate mock servers reuse these,
/// and downstream crates' mock servers do too (the GUI join e2e streams
/// the SAME registry bytes the mc-protocol arc mock does — one truth for
/// the mock registry, not hand-duplicated NBT in every mock).
pub mod test_support {
    use super::clientbound;
    use crate::nbt::Nbt;
    use crate::varint::{write_string, write_varint};

    /// A REAL `minecraft:worldgen/biome` registry data packet: ONE entry
    /// (the named biome, id 0 in the chunk biome palettes), with the
    /// realistic tag shape (temperature + effects compound) the client
    /// walks and skips. Mock servers that exercise biome music stream
    /// this alongside [`registry_packet`] — one truth for the mock
    /// registries, not hand-duplicated bytes in every mock.
    pub fn biome_registry_packet(biome: &str) -> Vec<u8> {
        biome_registry_packet_multi(&[biome])
    }

    /// The MULTI-entry form: a real vanilla server streams the WHOLE
    /// `minecraft:worldgen/biome` registry in one packet — both
    /// dimensions' biomes together, the ARRIVAL ORDER being the network
    /// id the chunk biome palettes reference. Mock servers that exercise
    /// dimension crossings stream two entries so the new dimension's
    /// chunks can carry a DIFFERENT biome id (the crossing re-fires the
    /// music only when the resolved name actually changes).
    pub fn biome_registry_packet_multi(names: &[&str]) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REGISTRY_DATA);
        write_string(&mut p, "minecraft:worldgen/biome");
        write_varint(&mut p, names.len() as i32);
        for biome in names {
            write_string(&mut p, biome);
            p.push(1); // tag present
            let tag = Nbt::Compound(vec![
                ("temperature".into(), Nbt::Float(0.8)),
                (
                    "effects".into(),
                    Nbt::Compound(vec![("fog_color".into(), Nbt::Int(0xC0D8FF))]),
                ),
            ]);
            crate::nbt::write_network_nbt(&tag, &mut p).expect("biome element writes");
        }
        p
    }

    /// A REAL registry data packet: the javap-proven 26.2 shape —
    /// registry Identifier + VarInt entry count + per entry { Identifier
    /// id, optional network NBT tag }. `dimension` selects between the
    /// decoded `minecraft:dimension_type` registry (overworld -64/384,
    /// nether 0/256 — the vanilla values; the entry tag IS the element)
    /// and a non-decoded one (biomes, with a richly nested compound
    /// inside).
    pub fn registry_packet(dimension: bool) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REGISTRY_DATA);
        if dimension {
            write_string(&mut p, "minecraft:dimension_type");
            write_varint(&mut p, 2); // two entries
            // entry 1: overworld — the optional tag IS the element
            write_string(&mut p, "minecraft:overworld");
            p.push(1); // tag present
            let ow = Nbt::Compound(vec![
                ("min_y".into(), Nbt::Int(-64)),
                ("height".into(), Nbt::Int(384)),
                ("has_skylight".into(), Nbt::Byte(1)),
            ]);
            crate::nbt::write_network_nbt(&ow, &mut p).expect("overworld element writes");
            // entry 2: nether
            write_string(&mut p, "minecraft:the_nether");
            p.push(1);
            let nether = Nbt::Compound(vec![
                ("min_y".into(), Nbt::Int(0)),
                ("height".into(), Nbt::Int(256)),
                ("fixed_time".into(), Nbt::Long(18_000)),
            ]);
            crate::nbt::write_network_nbt(&nether, &mut p).expect("nether element writes");
        } else {
            write_string(&mut p, "minecraft:worldgen/biome");
            write_varint(&mut p, 1); // one entry
            write_string(&mut p, "minecraft:plains");
            p.push(1); // tag present — walked and skipped by the client
            let plains = Nbt::Compound(vec![
                ("temperature".into(), Nbt::Float(0.8)),
                (
                    "effects".into(),
                    Nbt::Compound(vec![("fog_color".into(), Nbt::Int(0xC0D8FF))]),
                ),
            ]);
            crate::nbt::write_network_nbt(&plains, &mut p).expect("biome element writes");
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_support::registry_packet;
    use crate::login::{encode_login_disconnect, encode_login_success, encode_set_compression};
    use crate::varint::write_varint;
    use std::net::{TcpListener, TcpStream};
    use std::thread::JoinHandle;

    #[test]
    fn packet_shapes() {
        assert_eq!(encode_login_acknowledged(), vec![0x03]);
        assert_eq!(encode_config_ack(), vec![0x03]);
        // client information: the javap-proven 26.2 shape — locale "en_us",
        // vd 8 (a BYTE, not a varint), chat 0, colors 1, parts 0x7F, hand
        // 1, filters 0, listings 1, particle status 0 (the ninth field)
        let p = encode_client_information(&ClientInfo::default());
        assert_eq!(p[0], 0x00);
        assert_eq!(p[1], 5, "string length prefix");
        assert_eq!(&p[2..7], b"en_us");
        assert_eq!(p[7], 8, "view distance is a raw byte");
        assert_eq!(
            &p[p.len() - 6..],
            &[0x01, 0x7F, 0x01, 0x00, 0x01, 0x00],
            "skin parts .. particle status"
        );
        // brand: channel + nested string payload
        let b = encode_plugin_brand("mc2rs");
        assert_eq!(b[0], 0x02);
        assert!(b.windows(15).any(|w| w == b"minecraft:brand"), "channel");
        assert!(b.windows(5).any(|w| w == b"mc2rs"), "nested payload");
        // keep alive / pong roundtrips
        assert_eq!(encode_config_keep_alive_response(0x0123456789ABCDEF), {
            let mut p = vec![0x04];
            p.extend_from_slice(&0x0123456789ABCDEFi64.to_be_bytes());
            p
        });
        // parse expects the CLIENTBOUND id (4 on the real 26.2 wire —
        // generated), so build that shape here
        let mut ka = Vec::new();
        write_varint(&mut ka, clientbound::KEEP_ALIVE);
        ka.extend_from_slice(&(-42i64).to_be_bytes());
        assert_eq!(parse_config_keep_alive(&ka).unwrap(), -42);
        // the real wire has the clientbound ping and the serverbound pong
        // at the SAME id (both 5, generated) — different flows, no wire
        // ambiguity; the parser distinguishes within ONE flow: a
        // keep-alive id (4) is rejected by the ping parser (5)
        assert_eq!(
            clientbound::PING,
            crate::generated::packet_ids::CSB_PONG,
            "cross-flow id coincidence, the real 26.2 wire"
        );
        assert!(parse_config_ping(&encode_config_keep_alive_response(777)).is_err());
        let ok = {
            let mut p = Vec::new();
            write_varint(&mut p, clientbound::PING);
            p.extend_from_slice(&777i32.to_be_bytes());
            p
        };
        assert_eq!(parse_config_ping(&ok).unwrap(), 777);
        // disconnect flattening
        let mut d = Vec::new();
        write_varint(&mut d, clientbound::DISCONNECT);
        write_string(&mut d, r#"{"text":"shut","extra":[{"text":"down"}]}"#);
        assert_eq!(parse_config_disconnect(&d).unwrap(), "shutdown");
    }

    /// Mock server: login (with compression) -> configuration -> play.
    /// Asserts the client's introductions and answers server-side, then
    /// returns the client-visible log of serverbound packet ids it read.
    fn spawn_join_mock() -> (u16, JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let port = listener.local_addr().unwrap().port();
        let handle = std::thread::spawn(move || {
            let (stream, _) = listener.accept().expect("accept");
            let mut srv = PacketConn::new(stream);
            // handshake
            let hs = srv.read_packet().expect("handshake");
            let mut c: &[u8] = &hs;
            assert_eq!(read_varint(&mut c).unwrap(), 0x00);
            assert_eq!(read_varint(&mut c).unwrap(), 776, "REAL protocol number");
            let _host = read_string(&mut c).unwrap();
            let mut port_b = [0u8; 2];
            std::io::Read::read_exact(&mut c, &mut port_b).unwrap();
            assert_eq!(read_varint(&mut c).unwrap(), NEXT_STATE_LOGIN);
            // login start FIRST (the client has not seen Set Compression
            // yet — it travels uncompressed), then the compression switch
            let start = srv.read_packet().expect("login start");
            let mut c: &[u8] = &start;
            assert_eq!(read_varint(&mut c).unwrap(), 0x00);
            let name = read_string(&mut c).unwrap();
            srv.send_packet(&encode_set_compression(256)).unwrap();
            srv.enable_compression(256);
            srv.send_packet(&encode_login_success(
                &crate::login::offline_uuid(&name),
                &name,
            ))
            .unwrap();
            let mut log = vec![format!("login:{name}")];
            // the introductions: login acknowledged (still login state),
            // client information, plugin brand
            for expected in ["ack:0x03", "info:0x00", "brand:0x02"] {
                let pkt = srv.read_packet().expect(expected);
                let mut c: &[u8] = &pkt;
                log.push(format!("{expected}:0x{:02X}", read_varint(&mut c).unwrap()));
            }
            // probes + bulk traffic + finish
            let mut ka = Vec::new();
            write_varint(&mut ka, clientbound::KEEP_ALIVE);
            ka.extend_from_slice(&0x1122334455667788u64.to_be_bytes());
            srv.send_packet(&ka).unwrap();
            let mut pg = Vec::new();
            write_varint(&mut pg, clientbound::PING);
            pg.extend_from_slice(&77i32.to_be_bytes());
            srv.send_packet(&pg).unwrap();
            // replies arrive inline in read order: keep-alive echo + pong
            let resp = srv.read_packet().expect("keep alive response");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::KEEP_ALIVE);
            assert_eq!(i64::from_be_bytes(c[..8].try_into().unwrap()), 0x1122334455667788);
            let resp = srv.read_packet().expect("pong");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::PONG);
            assert_eq!(i32::from_be_bytes(c[..4].try_into().unwrap()), 77);
            // the BLOCKING probes: select known packs (the vanilla server
            // waits for the reply before finishing configuration — the
            // vanilla-era skip would stall a real join for 30s) + a cookie
            // request (the vanilla client answers unconditionally)
            let mut skp = Vec::new();
            write_varint(&mut skp, clientbound::SELECT_KNOWN_PACKS);
            write_varint(&mut skp, 1); // one offered pack
            write_string(&mut skp, "minecraft");
            write_string(&mut skp, "core");
            write_string(&mut skp, "26.2");
            srv.send_packet(&skp).unwrap();
            let mut crq = Vec::new();
            write_varint(&mut crq, clientbound::COOKIE_REQUEST);
            write_string(&mut crq, "minecraft:join_cookie");
            srv.send_packet(&crq).unwrap();
            let resp = srv.read_packet().expect("select known packs reply");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::SELECT_KNOWN_PACKS);
            assert_eq!(read_varint(&mut c).unwrap(), 0, "the honest empty claim");
            assert!(c.is_empty(), "nothing after the empty list");
            let resp = srv.read_packet().expect("cookie response");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::COOKIE_RESPONSE);
            assert_eq!(read_string(&mut c).unwrap(), "minecraft:join_cookie");
            assert_eq!(c, &[0x00], "no stored cookie -> the absent payload");
            // bulk registry traffic: the dimension_type registry is
            // DECODED by the client, a second registry is walked+skipped
            srv.send_packet(&registry_packet(true)).unwrap();
            srv.send_packet(&registry_packet(false)).unwrap();
            let mut fin = Vec::new();
            write_varint(&mut fin, clientbound::FINISH);
            srv.send_packet(&fin).unwrap();
            // the finish ack closes the phase
            let resp = srv.read_packet().expect("ack");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::ACKNOWLEDGE_FINISH);
            assert!(c.is_empty(), "ack payload must be empty");
            log
        });
        (port, handle)
    }


    #[test]
    fn registry_data_decodes_dimensions_keeps_biomes_and_skips_other_registries() {
        // the decoded registry: both dimensions land with their layouts
        let data = parse_registry_data(&registry_packet(true)).expect("dimension_type parses");
        assert_eq!(data.dimension_types.len(), 2);
        let ow = data.dimension("minecraft:overworld").expect("overworld");
        assert_eq!((ow.min_y, ow.height), (-64, 384));
        let nether = data.dimension("minecraft:the_nether").expect("nether");
        assert_eq!((nether.min_y, nether.height), (0, 256));
        assert!(data.dimension("minecraft:the_end").is_none());
        // the biome registry: names RETAINED in arrival order (the id ->
        // name table the chunk biome palettes resolve through), the tag
        // walked and skipped without a trailing-bytes error
        let biomes = parse_registry_data(&registry_packet(false)).expect("biome parses");
        assert_eq!(biomes.biomes, vec!["minecraft:plains".to_string()]);
        assert_eq!(biomes.biome_name(0), Some("minecraft:plains"));
        assert_eq!(biomes.biome_name(1), None, "an id the server never streamed");
        assert!(biomes.dimension_types.is_empty(), "no dimension knowledge rides along");
        // wrong packet id is rejected, not guessed
        let mut wrong = Vec::new();
        write_varint(&mut wrong, 0x99);
        assert!(parse_registry_data(&wrong).is_err());
        // a dimension entry WITH a tag but no min_y is a hard error
        let mut broken = Vec::new();
        write_varint(&mut broken, clientbound::REGISTRY_DATA);
        write_string(&mut broken, "minecraft:dimension_type");
        write_varint(&mut broken, 1);
        write_string(&mut broken, "minecraft:overworld");
        broken.push(1); // tag present
        let broken_tree = Nbt::Compound(vec![]);
        crate::nbt::write_network_nbt(&broken_tree, &mut broken).unwrap();
        assert!(parse_registry_data(&broken).is_err());
        // an entry with NO tag is skipped (nothing decodable, not drift)
        let mut absent = Vec::new();
        write_varint(&mut absent, clientbound::REGISTRY_DATA);
        write_string(&mut absent, "minecraft:dimension_type");
        write_varint(&mut absent, 2);
        write_string(&mut absent, "minecraft:overworld");
        absent.push(1);
        let ow = Nbt::Compound(vec![
            ("min_y".into(), Nbt::Int(-64)),
            ("height".into(), Nbt::Int(384)),
        ]);
        crate::nbt::write_network_nbt(&ow, &mut absent).unwrap();
        write_string(&mut absent, "minecraft:the_end");
        absent.push(0); // NO tag — the honest skip
        let data = parse_registry_data(&absent).expect("absent-tag entry skipped");
        assert_eq!(data.dimension_types.len(), 1, "only the tagged entry");
        assert_eq!(data.dimension("minecraft:overworld").unwrap().min_y, -64);
        // a truncated entry stream (count says 2, one entry) errors
        let mut short = Vec::new();
        write_varint(&mut short, clientbound::REGISTRY_DATA);
        write_string(&mut short, "minecraft:dimension_type");
        write_varint(&mut short, 2);
        write_string(&mut short, "minecraft:overworld");
        short.push(0);
        assert!(parse_registry_data(&short).is_err(), "count lies -> error");
    }

    #[test]
    fn biome_registries_retain_the_id_to_name_table_in_arrival_order() {
        // three entries: the FIRST with no tag (the id slot exists anyway
        // — arrival order is the network id, tag presence is irrelevant
        // to the numbering), the second tagged, the third again bare —
        // all three resolve through their arrival-order ids
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REGISTRY_DATA);
        write_string(&mut p, "minecraft:worldgen/biome");
        write_varint(&mut p, 3);
        write_string(&mut p, "minecraft:the_void");
        p.push(0); // NO tag — the id slot is still real
        write_string(&mut p, "minecraft:swamp");
        p.push(1);
        let tag = Nbt::Compound(vec![("temperature".into(), Nbt::Float(0.8))]);
        crate::nbt::write_network_nbt(&tag, &mut p).unwrap();
        write_string(&mut p, "minecraft:plains");
        p.push(0);
        let data = parse_registry_data(&p).expect("three biomes parse");
        assert_eq!(
            data.biomes,
            vec![
                "minecraft:the_void".to_string(),
                "minecraft:swamp".to_string(),
                "minecraft:plains".to_string(),
            ]
        );
        assert_eq!(data.biome_name(0), Some("minecraft:the_void"));
        assert_eq!(data.biome_name(1), Some("minecraft:swamp"));
        assert_eq!(data.biome_name(2), Some("minecraft:plains"));
        assert_eq!(data.biome_name(3), None, "past the streamed ids: unknown");
        // a biome entry with a MALFORMED tag is a hard error, not a
        // silent swallow — the walk must stay strict
        let mut broken = Vec::new();
        write_varint(&mut broken, clientbound::REGISTRY_DATA);
        write_string(&mut broken, "minecraft:worldgen/biome");
        write_varint(&mut broken, 1);
        write_string(&mut broken, "minecraft:swamp");
        broken.push(1);
        broken.extend_from_slice(&[0xFF, 0xFF]); // garbage tag bytes
        assert!(parse_registry_data(&broken).is_err(), "broken tag -> error");
        // a LYING count (two entries, one sent) errors the same way
        let mut short = Vec::new();
        write_varint(&mut short, clientbound::REGISTRY_DATA);
        write_string(&mut short, "minecraft:worldgen/biome");
        write_varint(&mut short, 2);
        write_string(&mut short, "minecraft:plains");
        short.push(0);
        assert!(parse_registry_data(&short).is_err(), "count lies -> error");
    }

    #[test]
    fn configuration_reaches_the_play_state() {
        let (port, srv) = spawn_join_mock();
        let stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        stream
            .set_write_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let out = join_play(stream, "127.0.0.1", port, 776, "mc2rs").expect("join play");
        match out {
            PlayOutcome::Joined { username, uuid, registry } => {
                assert_eq!(username, "mc2rs");
                assert_eq!(uuid, crate::login::offline_uuid("mc2rs"));
                // the registry knowledge survived the phase: both
                // dimension layouts decoded from the streamed NBT
                let ow = registry.dimension("minecraft:overworld").expect("overworld");
                assert_eq!((ow.min_y, ow.height), (-64, 384));
                let nether = registry.dimension("minecraft:the_nether").expect("nether");
                assert_eq!((nether.min_y, nether.height), (0, 256));
                // the biome registry rode the same phase: the id -> name
                // table survives into the play state (the merge path)
                assert_eq!(registry.biomes, vec!["minecraft:plains".to_string()]);
                assert_eq!(registry.biome_name(0), Some("minecraft:plains"));
            }
            other => panic!("expected Joined, got {other:?}"),
        }
        let log = srv.join().unwrap();
        assert_eq!(log.len(), 4, "full vanilla introduction order: {log:?}");
        assert_eq!(log[0], "login:mc2rs");
        assert_eq!(log[1], "ack:0x03:0x03");
        assert_eq!(log[2], "info:0x00:0x00");
        assert_eq!(log[3], "brand:0x02:0x02");
    }

    #[test]
    fn select_known_packs_and_cookies_roundtrip_with_the_proven_bounds() {
        // clientbound select known packs: two packs, THREE strings each
        // (the version is a string, not a VarInt — javap-proven)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SELECT_KNOWN_PACKS);
        write_varint(&mut p, 2);
        write_string(&mut p, "minecraft");
        write_string(&mut p, "core");
        write_string(&mut p, "26.2");
        write_string(&mut p, "example_mod");
        write_string(&mut p, "example");
        write_string(&mut p, "7");
        let packs = parse_select_known_packs(&p).expect("packs parse");
        assert_eq!(
            packs,
            vec![
                KnownPack {
                    namespace: "minecraft".into(),
                    id: "core".into(),
                    version: "26.2".into()
                },
                KnownPack {
                    namespace: "example_mod".into(),
                    id: "example".into(),
                    version: "7".into()
                }
            ]
        );
        // drift guards: wrong id, lying count, trailing bytes
        let mut wrong = Vec::new();
        write_varint(&mut wrong, 0x77);
        assert!(parse_select_known_packs(&wrong).is_err());
        let mut liar = Vec::new();
        write_varint(&mut liar, clientbound::SELECT_KNOWN_PACKS);
        write_varint(&mut liar, 3);
        write_string(&mut liar, "minecraft");
        write_string(&mut liar, "core");
        write_string(&mut liar, "26.2");
        assert!(parse_select_known_packs(&liar).is_err(), "count lies -> error");
        let mut trailing = p.clone();
        trailing.push(0xFF);
        assert!(parse_select_known_packs(&trailing).is_err());
        // the honest serverbound reply: EMPTY (claims nothing) — id 7 + 0
        assert_eq!(encode_select_known_packs_reply(&[]).unwrap(), vec![0x07, 0x00]);
        // a non-empty reply is the same three-string shape
        let reply = encode_select_known_packs_reply(&packs).unwrap();
        assert_eq!(reply[0], serverbound::SELECT_KNOWN_PACKS as u8);
        assert_eq!(reply[1], 2, "pack count");
        assert!(reply.windows(4).any(|w| w == b"26.2"), "version is a plain string");
        // the vanilla serverbound cap of 64 is enforced
        let mut too_many = Vec::new();
        for _ in 0..65 {
            too_many.push(KnownPack {
                namespace: "minecraft".into(),
                id: "core".into(),
                version: "26.2".into(),
            });
        }
        assert!(encode_select_known_packs_reply(&too_many).is_err());
        // cookie request: one Identifier key, nothing else
        let mut crq = Vec::new();
        write_varint(&mut crq, clientbound::COOKIE_REQUEST);
        write_string(&mut crq, "minecraft:join_cookie");
        assert_eq!(parse_cookie_request(&crq).unwrap(), "minecraft:join_cookie");
        crq.push(0x01);
        assert!(parse_cookie_request(&crq).is_err(), "trailing byte -> error");
        let mut wrong_key = Vec::new();
        write_varint(&mut wrong_key, 0x66);
        write_string(&mut wrong_key, "minecraft:nope");
        assert!(parse_cookie_request(&wrong_key).is_err());
        // cookie response: the key echoed + the absent payload (Bool 0) —
        // exactly the vanilla client's no-cookie answer
        assert_eq!(
            encode_cookie_response("minecraft:join_cookie", None).unwrap(),
            {
                let mut p = vec![0x01];
                write_string(&mut p, "minecraft:join_cookie");
                p.push(0);
                p
            }
        );
        // a payload rides the nullable form; 5120 is the vanilla cap
        let with_payload = encode_cookie_response("minecraft:join_cookie", Some(&[9, 9])).unwrap();
        assert_eq!(
            &with_payload[with_payload.len() - 4..],
            &[0x01, 0x02, 9, 9],
            "present flag, length, bytes"
        );
        let mut big = vec![0u8; 5121];
        big[5120] = 1;
        assert!(encode_cookie_response("minecraft:join_cookie", Some(&big)).is_err());
    }

    #[test]
    fn configuration_disconnect_surfaces_the_reason() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let srv = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut conn = PacketConn::new(stream);
            let _hs = conn.read_packet().unwrap();
            let _start = conn.read_packet().unwrap();
            conn.send_packet(&encode_set_compression(256)).unwrap();
            conn.enable_compression(256);
            conn.send_packet(&encode_login_success(&[9u8; 16], "mc2rs")).unwrap();
            // read the introductions, then kick during configuration
            for _ in 0..3 {
                conn.read_packet().unwrap();
            }
            let mut d = Vec::new();
            write_varint(&mut d, clientbound::DISCONNECT);
            write_string(&mut d, r#"{"text":"You are ","extra":[{"text":"banned","color":"red"}]}"#);
            conn.send_packet(&d).unwrap();
        });
        let stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let out = join_play(stream, "127.0.0.1", port, 776, "mc2rs").expect("join flow");
        assert_eq!(
            out,
            PlayOutcome::Disconnected { reason: "You are banned".to_string() }
        );
        srv.join().unwrap();
    }

    #[test]
    fn login_refusal_surfaces_through_join_play() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let srv = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut conn = PacketConn::new(stream);
            let _hs = conn.read_packet().unwrap();
            let _start = conn.read_packet().unwrap();
            conn.send_packet(&encode_login_disconnect(r#"{"text":"invalid session"}"#))
                .unwrap();
        });
        let stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let out = join_play(stream, "127.0.0.1", port, 776, "mc2rs").expect("join flow");
        assert_eq!(out, PlayOutcome::Disconnected { reason: "invalid session".to_string() });
        srv.join().unwrap();
    }

    #[test]
    fn connect_and_join_play_end_to_end() {
        let (port, srv) = spawn_join_mock();
        let out = connect_and_join_play("127.0.0.1", port, 776, "Steve").expect("connect");
        assert!(matches!(out, PlayOutcome::Joined { ref username, .. } if username == "Steve"));
        srv.join().unwrap();
    }

    #[test]
    fn pre_play_ids_are_the_generated_runtime_facts() {
        // The configuration/login tables are GENERATED from the official
        // jar's own bootstrapped protocols (`mc2rs-gen dump-packet-ids`
        // binds LoginProtocols + ConfigurationProtocols + the rest and
        // dumps the live dispatch codecs' id maps). These pins come from
        // the real 26.2 runtime dump — the wiki-era constants they
        // replaced had EVERY clientbound config id one low (registry
        // data was 0x06 there, 0x07 on the real wire): a real join would
        // have failed to recognize any of these packets, and the e2e
        // mocks were self-consistent with the old table and masked it.
        use crate::generated::packet_ids as gen;
        // clientbound configuration: the +1 drift wave
        assert_eq!(clientbound::DISCONNECT, gen::CCB_DISCONNECT);
        assert_eq!(clientbound::DISCONNECT, 2, "disconnect — the wiki-era 0x01 was one low");
        assert_eq!(clientbound::FINISH, 3, "finish_configuration — wiki-era 0x02");
        assert_eq!(clientbound::KEEP_ALIVE, 4, "wiki-era 0x03");
        assert_eq!(clientbound::PING, 5, "wiki-era 0x04");
        assert_eq!(clientbound::REGISTRY_DATA, 7, "registry_data — wiki-era 0x06");
        // the answered configuration probes (this run's javap proofs)
        assert_eq!(clientbound::SELECT_KNOWN_PACKS, 14, "minecraft:select_known_packs");
        assert_eq!(clientbound::COOKIE_REQUEST, 0, "minecraft:cookie_request — the flow head");
        assert_eq!(serverbound::SELECT_KNOWN_PACKS, 7, "minecraft:select_known_packs");
        assert_eq!(serverbound::COOKIE_RESPONSE, 1, "minecraft:cookie_response");
        // serverbound configuration: numerically stable, names moved
        assert_eq!(serverbound::CLIENT_INFORMATION, 0);
        assert_eq!(serverbound::PLUGIN_MESSAGE, 2, "minecraft:custom_payload");
        assert_eq!(serverbound::ACKNOWLEDGE_FINISH, 3, "minecraft:finish_configuration");
        assert_eq!(serverbound::KEEP_ALIVE, 4);
        assert_eq!(serverbound::PONG, 5);
        // login acknowledged lives in the LOGIN serverbound flow
        assert_eq!(loginbound::LOGIN_ACKNOWLEDGED, gen::LSB_LOGIN_ACKNOWLEDGED);
        assert_eq!(loginbound::LOGIN_ACKNOWLEDGED, 3);
        // the full flows are dense: the last id is the entry count - 1
        for (flow, count) in [
            (&gen::CONFIG_CLIENTBOUND, 20),
            (&gen::CONFIG_SERVERBOUND, 10),
            (&gen::LOGIN_CLIENTBOUND, 6),
            (&gen::LOGIN_SERVERBOUND, 5),
            (&gen::STATUS_CLIENTBOUND, 2),
            (&gen::STATUS_SERVERBOUND, 2),
            (&gen::HANDSHAKE_SERVERBOUND, 1),
        ] {
            assert_eq!(flow.last().unwrap().0, (flow.len() - 1) as u32, "dense flow");
            assert_eq!(flow.len(), count, "flow size pinned from the 26.2 dump");
        }
        // the status/handshake facts the framers now reference
        assert_eq!(gen::TCB_STATUS_RESPONSE, 0);
        assert_eq!(gen::TSB_STATUS_REQUEST, 0);
        assert_eq!(gen::HSB_INTENTION, 0);
        assert_eq!(gen::LSB_HELLO, 0, "login start is hello");
    }
}

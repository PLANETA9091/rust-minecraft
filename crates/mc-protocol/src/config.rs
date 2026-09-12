//! Configuration (protocol state 3): the phase both sides enter after
//! Login Success (via the serverbound Login Acknowledged). The vanilla
//! client introduces itself (Client Information + the `minecraft:brand`
//! plugin message), answers Keep Alive / Ping probes, skips the bulk
//! registry/feature/resource-pack traffic and acknowledges the server's
//! Finish Configuration — then the play state begins.
//!
//! Clean-room wiki.vg shapes for 1.20.5+ / 26.x:
//! - Login Acknowledged (serverbound login 0x03): empty payload — from the
//!   NEXT packet on, both sides speak configuration.
//! - Client Information (serverbound config 0x00): Locale (String 16),
//!   View Distance (VarInt), Chat Mode (VarInt 0 = enabled), Chat Colors
//!   (Bool), Displayed Skin Parts (UByte 0x7F = all), Main Hand (VarInt
//!   1 = right), Disable Text Filtering (Bool), Allow Server Listings
//!   (Bool).
//! - Registry Data (clientbound 0x06, one registry per packet):
//!   Identifier + network NBT. The `minecraft:dimension_type` registry is
//!   DECODED — every registered dimension's vertical layout (min_y,
//!   height) — the first retained NBT in the client (crate::nbt); every
//!   other registry (biomes, worldgen, banners...) is walked and skipped.
//! - Plugin Message (serverbound config 0x02): Channel (String) + Data —
//!   the brand channel carries one UTF string ("mc2rs").
//! - Keep Alive (clientbound config 0x03 / serverbound 0x04): i64 id,
//!   echoed verbatim.
//! - Ping/Pong (clientbound 0x04 / serverbound 0x05): i32 id, echoed.
//! - Disconnect (clientbound config 0x01): chat JSON reason.
//! - Finish Configuration (clientbound 0x02) → Acknowledge Finish
//!   Configuration (serverbound 0x03): both empty; after the ack the play
//!   state begins.
//! - Everything else (Cookie Request, Feature Flags, Resource Pack
//!   Push/Pop, Select Known Packs, Custom Report Details, Server Links) is
//!   read and SKIPPED — the client has no data-pack or resource-pack
//!   surface yet; the vanilla server finishes configuration fine without
//!   answering any of them.

use std::io::{Read, Write};

use anyhow::{Context, Result};

use crate::framing::{encode_handshake, PacketConn};
use crate::login::{login_conn, LoginOutcome};
use crate::nbt::{read_network_nbt, Nbt};
use crate::varint::{read_string, read_varint, write_string, write_varint};
use crate::{DEFAULT_TIMEOUT_SECS, NEXT_STATE_LOGIN};

/// Serverbound login packet id (the state switch happens at the server).
mod loginbound {
    pub const LOGIN_ACKNOWLEDGED: i32 = 0x03;
}

/// Serverbound configuration packet ids.
mod serverbound {
    pub const CLIENT_INFORMATION: i32 = 0x00;
    pub const PLUGIN_MESSAGE: i32 = 0x02;
    pub const ACKNOWLEDGE_FINISH: i32 = 0x03;
    pub const KEEP_ALIVE: i32 = 0x04;
    pub const PONG: i32 = 0x05;
}

/// Clientbound configuration packet ids.
mod clientbound {
    pub const DISCONNECT: i32 = 0x01;
    pub const FINISH: i32 = 0x02;
    pub const KEEP_ALIVE: i32 = 0x03;
    pub const PING: i32 = 0x04;
    pub const REGISTRY_DATA: i32 = 0x06;
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
}

impl RegistryData {
    /// The vertical layout of one dimension, by registry key.
    pub fn dimension(&self, name: &str) -> Option<&DimensionTypeEntry> {
        self.dimension_types.iter().find(|d| d.name == name)
    }

    /// Absorb one decoded Registry Data packet (vanilla streams one
    /// registry per packet; entries append in arrival order).
    fn merge(&mut self, other: RegistryData) {
        self.dimension_types.extend(other.dimension_types);
    }
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
    pub view_distance: i32,
    /// 0 = enabled, 1 = commands only, 2 = hidden.
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub skin_parts: u8,
    /// 0 = left, 1 = right.
    pub main_hand: i32,
    pub disable_text_filtering: bool,
    pub allow_server_listings: bool,
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

/// Client Information (serverbound config 0x00).
pub fn encode_client_information(info: &ClientInfo) -> Vec<u8> {
    let mut p = Vec::with_capacity(24);
    write_varint(&mut p, serverbound::CLIENT_INFORMATION);
    write_string(&mut p, &info.locale);
    write_varint(&mut p, info.view_distance);
    write_varint(&mut p, info.chat_mode);
    p.push(u8::from(info.chat_colors));
    p.push(info.skin_parts);
    write_varint(&mut p, info.main_hand);
    p.push(u8::from(info.disable_text_filtering));
    p.push(u8::from(info.allow_server_listings));
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

// ---------------------------------------------------------------------------
// packet parsers (full payload including the packet id)
// ---------------------------------------------------------------------------

/// Parse Keep Alive (clientbound config 0x03): i64 id.
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

/// Parse Ping (clientbound config 0x04): i32 id.
pub fn parse_config_ping(payload: &[u8]) -> Result<i32> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::PING, "not a config ping packet (0x{id:02X})");
    anyhow::ensure!(c.len() >= 4, "config ping truncated");
    Ok(i32::from_be_bytes(c[..4].try_into()?))
}

/// Parse Disconnect (clientbound config 0x01): chat JSON reason, flattened
/// to plain text (raw string kept when the JSON does not parse).
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

/// Parse Registry Data (clientbound config 0x06): one registry per
/// packet — Identifier + network NBT. Only `minecraft:dimension_type` is
/// decoded (the vertical layout per entry); every other registry's NBT is
/// walked and dropped. Strict on the decoded registry: an entry missing
/// name/element/min_y/height is a protocol drift worth erroring on, the
/// same way vanilla's own registry loader refuses malformed data.
pub fn parse_registry_data(payload: &[u8]) -> Result<RegistryData> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::REGISTRY_DATA,
        "not a registry data packet (0x{id:02X})"
    );
    let registry_name = read_string(&mut c)?;
    if registry_name != "minecraft:dimension_type" {
        crate::play::skip_network_nbt(&mut c)
            .with_context(|| format!("skip {registry_name} registry nbt"))?;
        return Ok(RegistryData::default());
    }
    let nbt = read_network_nbt(&mut c)
        .with_context(|| "decode minecraft:dimension_type registry nbt")?;
    anyhow::ensure!(
        c.is_empty(),
        "trailing bytes after the dimension_type registry nbt"
    );
    let values = nbt
        .get("value")
        .and_then(Nbt::as_list)
        .with_context(|| "dimension_type nbt has no value list")?;
    let mut out = RegistryData::default();
    for entry in values {
        let name = entry
            .get("name")
            .and_then(Nbt::as_str)
            .with_context(|| "dimension_type entry without a name")?
            .to_string();
        let element = entry
            .get("element")
            .with_context(|| format!("dimension_type {name} without an element"))?;
        let min_y = element
            .get("min_y")
            .and_then(Nbt::as_i32)
            .with_context(|| format!("dimension_type {name} without min_y"))?;
        let height = element
            .get("height")
            .and_then(Nbt::as_i32)
            .with_context(|| format!("dimension_type {name} without height"))?;
        out.dimension_types.push(DimensionTypeEntry { name, min_y, height });
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// the configuration driver
// ---------------------------------------------------------------------------

/// Answer the server's configuration probes until Finish Configuration is
/// acknowledged. Registry Data is decoded for the dimension-type vertical
/// layout (everything else in it — other registries — is skipped);
/// feature/resource-pack/cookie traffic is skipped wholesale; a config
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
            clientbound::FINISH => {
                conn.send_packet(&encode_config_ack())?;
                return Ok(PlayOutcome::Joined { uuid, username, registry });
            }
            clientbound::DISCONNECT => {
                let reason = parse_config_disconnect(&pkt)?;
                return Ok(PlayOutcome::Disconnected { reason });
            }
            // cookie request / feature flags / resource packs / report
            // details / server links — read and skip
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

    /// A REAL registry data packet: identifier + network NBT. `dimension`
    /// selects between the decoded `minecraft:dimension_type` registry
    /// (overworld -64/384, nether 0/256 — the vanilla values) and a
    /// non-decoded one (biomes, with a richly nested compound inside).
    pub fn registry_packet(dimension: bool) -> Vec<u8> {
        let (name, nbt) = if dimension {
            (
                "minecraft:dimension_type",
                Nbt::Compound(vec![
                    ("type".into(), Nbt::String("minecraft:dimension_type".into())),
                    (
                        "value".into(),
                        Nbt::List(vec![
                            Nbt::Compound(vec![
                                ("name".into(), Nbt::String("minecraft:overworld".into())),
                                ("id".into(), Nbt::Int(0)),
                                (
                                    "element".into(),
                                    Nbt::Compound(vec![
                                        ("min_y".into(), Nbt::Int(-64)),
                                        ("height".into(), Nbt::Int(384)),
                                        ("has_skylight".into(), Nbt::Byte(1)),
                                    ]),
                                ),
                            ]),
                            Nbt::Compound(vec![
                                ("name".into(), Nbt::String("minecraft:the_nether".into())),
                                ("id".into(), Nbt::Int(2)),
                                (
                                    "element".into(),
                                    Nbt::Compound(vec![
                                        ("min_y".into(), Nbt::Int(0)),
                                        ("height".into(), Nbt::Int(256)),
                                        ("fixed_time".into(), Nbt::Long(18_000)),
                                    ]),
                                ),
                            ]),
                        ]),
                    ),
                ]),
            )
        } else {
            (
                "minecraft:worldgen/biome",
                Nbt::Compound(vec![
                    ("type".into(), Nbt::String("minecraft:worldgen/biome".into())),
                    (
                        "value".into(),
                        Nbt::List(vec![Nbt::Compound(vec![
                            ("name".into(), Nbt::String("minecraft:plains".into())),
                            (
                                "element".into(),
                                Nbt::Compound(vec![
                                    ("temperature".into(), Nbt::Float(0.8)),
                                    ("effects".into(), Nbt::Compound(vec![
                                        ("fog_color".into(), Nbt::Int(0xC0D8FF)),
                                    ])),
                                ]),
                            ),
                        ])]),
                    ),
                ]),
            )
        };
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REGISTRY_DATA);
        write_string(&mut p, name);
        crate::nbt::write_network_nbt(&nbt, &mut p).expect("registry nbt writes");
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
        // client information: locale "en_us", vd 8, chat 0, colors 1,
        // parts 0x7F, hand 1, filters 0, listings 1
        let p = encode_client_information(&ClientInfo::default());
        assert_eq!(p[0], 0x00);
        assert_eq!(p[1], 5, "string length prefix");
        assert_eq!(&p[2..7], b"en_us");
        assert_eq!(p[7], 8);
        assert_eq!(&p[p.len() - 5..], &[0x01, 0x7F, 0x01, 0x00, 0x01]);
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
        // parse expects the CLIENTBOUND id (0x03), so build that shape here
        let mut ka = Vec::new();
        write_varint(&mut ka, clientbound::KEEP_ALIVE);
        ka.extend_from_slice(&(-42i64).to_be_bytes());
        assert_eq!(parse_config_keep_alive(&ka).unwrap(), -42);
        // the pong encoder (0x05) must be REJECTED by the ping parser (0x04)
        assert!(parse_config_ping(&encode_config_pong(777)).is_err());
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
            // bulk registry traffic: the dimension_type registry is
            // DECODED by the client, a second registry is walked+skipped
            srv.send_packet(&registry_packet(true)).unwrap();
            srv.send_packet(&registry_packet(false)).unwrap();
            let mut fin = Vec::new();
            write_varint(&mut fin, clientbound::FINISH);
            srv.send_packet(&fin).unwrap();
            // responses: keep-alive echo, pong, acknowledge finish
            let resp = srv.read_packet().expect("keep alive response");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::KEEP_ALIVE);
            assert_eq!(i64::from_be_bytes(c[..8].try_into().unwrap()), 0x1122334455667788);
            let resp = srv.read_packet().expect("pong");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::PONG);
            assert_eq!(i32::from_be_bytes(c[..4].try_into().unwrap()), 77);
            let resp = srv.read_packet().expect("ack");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::ACKNOWLEDGE_FINISH);
            assert!(c.is_empty(), "ack payload must be empty");
            log
        });
        (port, handle)
    }


    #[test]
    fn registry_data_decodes_the_dimension_layout_and_skips_other_registries() {
        // the decoded registry: both dimensions land with their layouts
        let data = parse_registry_data(&registry_packet(true)).expect("dimension_type parses");
        assert_eq!(data.dimension_types.len(), 2);
        let ow = data.dimension("minecraft:overworld").expect("overworld");
        assert_eq!((ow.min_y, ow.height), (-64, 384));
        let nether = data.dimension("minecraft:the_nether").expect("nether");
        assert_eq!((nether.min_y, nether.height), (0, 256));
        assert!(data.dimension("minecraft:the_end").is_none());
        // any other registry: walked, dropped, no entries
        let empty = parse_registry_data(&registry_packet(false)).expect("biome parses");
        assert_eq!(empty, RegistryData::default());
        // wrong packet id is rejected, not guessed
        let mut wrong = Vec::new();
        write_varint(&mut wrong, 0x99);
        assert!(parse_registry_data(&wrong).is_err());
        // a dimension entry without min_y is a hard error (protocol drift)
        let mut broken = Vec::new();
        write_varint(&mut broken, clientbound::REGISTRY_DATA);
        write_string(&mut broken, "minecraft:dimension_type");
        let broken_tree = Nbt::Compound(vec![(
            "value".into(),
            Nbt::List(vec![Nbt::Compound(vec![
                ("name".into(), Nbt::String("minecraft:overworld".into())),
                ("element".into(), Nbt::Compound(vec![])),
            ])]),
        )]);
        crate::nbt::write_network_nbt(&broken_tree, &mut broken).unwrap();
        assert!(parse_registry_data(&broken).is_err());
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
}

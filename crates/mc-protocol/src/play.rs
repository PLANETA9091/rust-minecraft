//! Play (protocol state 4): the first real world from the network — the
//! state both sides enter after Finish Configuration is acknowledged. The
//! session pump reads one clientbound packet per call, answers the
//! server's probes (keep-alive, ping, chunk batch, position teleport) and
//! decodes chunk data into resolved block/biome ids — the raw material a
//! client world needs.
//!
//! Clean-room wiki.vg shapes for 1.20.5+ / 26.x (play state):
//! - Login (clientbound 0x27): Entity ID (Int), Is Hardcore (Bool),
//!   Dimension Count (VarInt) + names (String), Max Players (VarInt,
//!   unused by vanilla), View Distance (VarInt), Simulation Distance
//!   (VarInt), Reduced Debug (Bool), Enable Respawn Screen (Bool), Do
//!   Limited Crafting (Bool), Has Death Location (Bool [+ dimension name
//!   + 8-byte position]), Portal Cooldown (VarInt).
//! - Synchronize Player Position (clientbound 0x3C): X/Y/Z (Double),
//!   Yaw/Pitch (Float), Relativity Flags (UByte), Teleport ID (VarInt) —
//!   answered with Confirm Teleportation (serverbound 0x00, id verbatim).
//! - Chunk Data and Update Light (clientbound 0x23): Chunk X/Z (Int),
//!   Heightmaps (network NBT — walked and skipped), chunk buffer (VarInt
//!   length) of sections back to back, block entities (skipped), light
//!   masks (ignored — the sky-light layer is future work).
//!   One section = Block Count (Short) + block states + biomes, each a
//!   palette + packed data array in VANILLA LONG PACKING: entries fill
//!   64-bit longs LSB-first and NEVER span two longs (valuesPerLong =
//!   floor(64 / bitsPerEntry)). Block index = y*256 + z*16 + x (YZX,
//!   4096 entries); biome index = y*16 + z*4 + x (4×4×4, 64 entries).
//!   Bits per entry: block states 0 = single valued, 4..=8 indirect,
//!   9..=15 direct; biomes 0 = single, 1..=3 indirect, 4..=15 direct.
//!   Decoded arrays are fully RESOLVED global ids (flat Vec<u32>) —
//!   compact client-side storage is the future mesher's concern.
//! - Set Center Chunk (clientbound 0x4E): VarInt chunk X/Z.
//! - Unload Chunk (clientbound 0x1D): Long ((z << 32) | (x & 0xFFFFFFFF)).
//! - Keep Alive (clientbound 0x22 / serverbound 0x14): i64 verbatim.
//! - Ping (clientbound 0x31 / serverbound 0x22): i32 verbatim.
//! - Chunk Batch Start (clientbound 0x0D) → Chunk Batch Received
//!   (serverbound 0x07, f32 desired chunks per tick) — a vanilla server
//!   PAUSES chunk streaming until this reply arrives.
//! - Disconnect (clientbound 0x1A): chat JSON reason, flattened.
//! - Everything else is read and SKIPPED (entity metadata, time, sounds,
//!   scoreboard... — no gameplay surface yet; unknown ids never error).

use std::io::{Read, Write};

use anyhow::{Context, Result};

use crate::config::{join_play_conn, PlayOutcome, RegistryData};
use crate::framing::PacketConn;
use crate::varint::{read_string, read_varint, write_varint};
use crate::DEFAULT_TIMEOUT_SECS;

/// Clientbound play packet ids (wiki.vg play state). Public protocol
/// facts — mock servers / tooling assert against them.
pub mod clientbound {
    pub const BATCH_START: i32 = 0x0D;
    pub const DISCONNECT: i32 = 0x1A;
    pub const UNLOAD_CHUNK: i32 = 0x1D;
    pub const KEEP_ALIVE: i32 = 0x22;
    pub const CHUNK_DATA: i32 = 0x23;
    pub const LOGIN: i32 = 0x27;
    pub const PING: i32 = 0x31;
    pub const SYNCHRONIZE_POSITION: i32 = 0x3C;
    pub const CENTER_CHUNK: i32 = 0x4E;
}

/// Serverbound play packet ids (wiki.vg play state). Public protocol
/// facts — mock servers / tooling assert against them.
pub mod serverbound {
    pub const CONFIRM_TELEPORTATION: i32 = 0x00;
    pub const CHUNK_BATCH_RECEIVED: i32 = 0x07;
    pub const KEEP_ALIVE: i32 = 0x14;
    pub const PONG: i32 = 0x22;
}

/// Fixed chunk-streaming pace requested in Chunk Batch Received. The
/// vanilla client adapts this from measured batch latencies; a fixed mid
/// value keeps real servers streaming without pretending to have a batch
/// timing model yet.
pub const DESIRED_CHUNKS_PER_TICK: f32 = 50.0;

/// Entries per section side: 16 for blocks, 4 for biomes.
const SECTION_BLOCKS: usize = 4096;
const SECTION_BIOMES: usize = 64;

// ---------------------------------------------------------------------------
// world model
// ---------------------------------------------------------------------------

/// The play-state Login packet: the world's static description.
#[derive(Debug, Clone, PartialEq)]
pub struct WorldInfo {
    /// The client player's entity id for this session.
    pub entity_id: i32,
    pub hardcore: bool,
    /// Dimension names the server registered (e.g. `minecraft:overworld`).
    pub dimensions: Vec<String>,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub portal_cooldown: i32,
}

/// Synchronize Player Position: where the server says the player is.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    /// Relativity flags (0x01 X, 0x02 Y, 0x04 Z, 0x08 yaw, 0x10 pitch,
    /// higher bits velocity/deltas in newer shapes) — kept raw.
    pub flags: u8,
    pub teleport_id: i32,
}

/// One 16×16×16 chunk section, fully decoded.
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkSection {
    /// Non-air blocks in the section (vanilla counts fluids too).
    pub block_count: i16,
    /// Resolved global block state ids, vanilla YZX order:
    /// index = y*256 + z*16 + x.
    pub blocks: Vec<u32>,
    /// Resolved biome ids, index = y*16 + z*4 + x (64 entries).
    pub biomes: Vec<u32>,
}

/// One chunk column as sent by Chunk Data and Update Light.
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkColumn {
    pub x: i32,
    pub z: i32,
    /// Sections in buffer order — bottom of the world first.
    pub sections: Vec<ChunkSection>,
}

/// The "loading screen finished" milestone: world info, the first
/// position teleport and at least one decoded chunk — plus the
/// configuration registry knowledge (the dimension-type vertical layout
/// table the server streamed before the play state began).
#[derive(Debug, Clone, PartialEq)]
pub struct WorldSnapshot {
    pub info: WorldInfo,
    pub player: PlayerPosition,
    pub chunks: Vec<ChunkColumn>,
    pub registry: RegistryData,
}

// ---------------------------------------------------------------------------
// packed data arrays (vanilla long packing)
// ---------------------------------------------------------------------------

/// Pack palette entries into the vanilla data-array layout: 64-bit longs,
/// entries LSB-first, never spanning two longs. Big-endian longs on the
/// wire. Protocol tooling + mock-server helper (the mirror of
/// `read_packed`).
pub fn pack_entries(entries: &[u32], bits: u32) -> Vec<u8> {
    let bits = bits.max(1);
    let per_long = 64 / bits as usize;
    let longs = entries.len().div_ceil(per_long).max(1);
    let mut packed = vec![0u64; longs];
    for (i, e) in entries.iter().enumerate() {
        let mask = (1u64 << bits) - 1;
        packed[i / per_long] |= ((*e as u64) & mask) << ((i % per_long) * bits as usize);
    }
    let mut out = Vec::with_capacity(longs * 8);
    for l in packed {
        out.extend_from_slice(&l.to_be_bytes());
    }
    out
}

/// Unpack `count` entries of `bits` bits from the vanilla data-array
/// layout (the mirror of `pack_entries`).
fn read_packed(r: &mut &[u8], bits: u32, count: usize) -> std::io::Result<Vec<u32>> {
    let per_long = 64 / bits as usize;
    let longs = count.div_ceil(per_long);
    if r.len() < longs * 8 {
        return Err(corrupt(&format!(
            "packed data array truncated: need {longs} longs, have {}",
            r.len() / 8
        )));
    }
    let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
    let mut out = Vec::with_capacity(count);
    for li in 0..longs {
        let mut b = [0u8; 8];
        r.read_exact(&mut b)?;
        let long = u64::from_be_bytes(b);
        // the last long may hold fewer valid entries (padding above)
        let entries = (((li + 1) * per_long).min(count) - li * per_long).min(per_long);
        for k in 0..entries {
            let off = k * bits as usize;
            out.push(((long >> off) & mask) as u32);
        }
    }
    Ok(out)
}

/// Bits-per-entry thresholds for one palettized array kind.
struct PaletteSpec {
    indirect_min: u32,
    indirect_max: u32,
    direct_max: u32,
}

/// Block states: 0 = single, 4..=8 indirect, 9..=15 direct.
const BLOCK_PALETTE: PaletteSpec = PaletteSpec { indirect_min: 4, indirect_max: 8, direct_max: 15 };
/// Biomes: 0 = single, 1..=3 indirect, 4..=15 direct.
const BIOME_PALETTE: PaletteSpec = PaletteSpec { indirect_min: 1, indirect_max: 3, direct_max: 15 };

/// Parse one palettized array (block states or biomes) and resolve every
/// entry to its global id.
fn parse_palettized(
    r: &mut &[u8],
    count: usize,
    spec: &PaletteSpec,
    kind: &str,
) -> std::io::Result<Vec<u32>> {
    let bits = read_u8(r)? as u32;
    match bits {
        0 => {
            let single = read_varint(r)? as u32;
            Ok(vec![single; count])
        }
        b if spec.indirect_min <= b && b <= spec.indirect_max => {
            let len = read_varint(r)?;
            if len < 0 || len as usize > count {
                return Err(corrupt(&format!("{kind}: palette length {len} for {count} entries")));
            }
            let mut palette = Vec::with_capacity(len as usize);
            for _ in 0..len {
                palette.push(read_varint(r)? as u32);
            }
            let raw = read_packed(r, b, count)?;
            let mut out = Vec::with_capacity(count);
            for idx in raw {
                let id = palette
                    .get(idx as usize)
                    .ok_or_else(|| corrupt(&format!("{kind}: palette index {idx} out of range")))?;
                out.push(*id);
            }
            Ok(out)
        }
        b if b > spec.indirect_max && b <= spec.direct_max => read_packed(r, b, count),
        b => Err(corrupt(&format!("{kind}: unsupported bits per entry {b}"))),
    }
}

fn parse_section(r: &mut &[u8]) -> std::io::Result<ChunkSection> {
    let block_count = read_i16(r)?;
    let blocks = parse_palettized(r, SECTION_BLOCKS, &BLOCK_PALETTE, "block states")?;
    let biomes = parse_palettized(r, SECTION_BIOMES, &BIOME_PALETTE, "biomes")?;
    Ok(ChunkSection { block_count, blocks, biomes })
}

// ---------------------------------------------------------------------------
// network NBT (1.20.5+ anonymous root) — walked, never retained
// ---------------------------------------------------------------------------

/// Walk one network-NBT value without retaining it (root compound must be
/// anonymous per the 1.20.5+ wire format). Big-endian, all 13 tag types.
pub fn skip_network_nbt(r: &mut &[u8]) -> std::io::Result<()> {
    let root = read_u8(r)?;
    if root != 0x0A {
        return Err(corrupt(&format!("expected an NBT root compound (0x{root:02X})")));
    }
    skip_compound_contents(r)
}

fn skip_compound_contents(r: &mut &[u8]) -> std::io::Result<()> {
    loop {
        let tag = read_u8(r)?;
        if tag == 0x00 {
            return Ok(());
        }
        let name_len = read_u16(r)? as usize;
        if r.len() < name_len {
            return Err(corrupt("nbt: truncated tag name"));
        }
        *r = &r[name_len..];
        skip_nbt_payload(r, tag)?;
    }
}

fn skip_nbt_payload(r: &mut &[u8], tag: u8) -> std::io::Result<()> {
    match tag {
        0x01 => skip_bytes(r, 1),
        0x02 => skip_bytes(r, 2),
        0x03 | 0x05 => skip_bytes(r, 4),
        0x04 | 0x06 => skip_bytes(r, 8),
        0x07 => {
            // byte array
            let n = read_i32(r)?;
            skip_bytes(r, n as i64)
        }
        0x08 => {
            // string (u16 length)
            let n = read_u16(r)? as i64;
            skip_bytes(r, n)
        }
        0x09 => {
            // list: inner tag id + count + payloads
            let inner = read_u8(r)?;
            let n = read_i32(r)?;
            if n < 0 {
                return Err(corrupt("nbt: negative list length"));
            }
            for _ in 0..n {
                skip_nbt_payload(r, inner)?;
            }
            Ok(())
        }
        0x0A => skip_compound_contents(r),
        0x0B => {
            // int array
            let n = read_i32(r)?;
            skip_bytes(r, n as i64 * 4)
        }
        0x0C => {
            // long array (heightmaps live here)
            let n = read_i32(r)?;
            skip_bytes(r, n as i64 * 8)
        }
        other => Err(corrupt(&format!("nbt: unknown tag id 0x{other:02X}"))),
    }
}

/// Advance the cursor by `n` bytes, rejecting negative/huge lengths.
fn skip_bytes(r: &mut &[u8], n: i64) -> std::io::Result<()> {
    if n < 0 || n as usize > r.len() {
        return Err(corrupt(&format!("nbt: payload of {n} bytes overflows the packet")));
    }
    *r = &r[n as usize..];
    Ok(())
}

// ---------------------------------------------------------------------------
// packet parsers (full payload including the packet id)
// ---------------------------------------------------------------------------

/// Parse the play-state Login (clientbound 0x27).
pub fn parse_world_info(payload: &[u8]) -> Result<WorldInfo> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::LOGIN, "not a play login packet (0x{id:02X})");
    let entity_id = read_i32(&mut c)?;
    let hardcore = read_bool(&mut c)?;
    let dim_count = read_varint(&mut c)?;
    anyhow::ensure!((0..=4096).contains(&dim_count), "nonsensical dimension count {dim_count}");
    let mut dimensions = Vec::with_capacity(dim_count as usize);
    for _ in 0..dim_count {
        dimensions.push(read_string(&mut c)?);
    }
    let _max_players = read_varint(&mut c)?;
    let view_distance = read_varint(&mut c)?;
    let simulation_distance = read_varint(&mut c)?;
    let _reduced_debug = read_bool(&mut c)?;
    let _respawn_screen = read_bool(&mut c)?;
    let _limited_crafting = read_bool(&mut c)?;
    let has_death_location = read_bool(&mut c)?;
    if has_death_location {
        let _dim = read_string(&mut c)?;
        let mut pos = [0u8; 8];
        c.read_exact(&mut pos)?;
    }
    let portal_cooldown = read_varint(&mut c)?;
    Ok(WorldInfo {
        entity_id,
        hardcore,
        dimensions,
        view_distance,
        simulation_distance,
        portal_cooldown,
    })
}

/// Parse Chunk Data and Update Light (clientbound 0x23) into a decoded
/// column. Heightmaps and block entities are walked and skipped; the
/// light masks after them are ignored (sky-light is future work).
pub fn parse_chunk_data(payload: &[u8]) -> Result<ChunkColumn> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::CHUNK_DATA, "not a chunk data packet (0x{id:02X})");
    let x = read_i32(&mut c)?;
    let z = read_i32(&mut c)?;
    skip_network_nbt(&mut c)?;
    let size = read_varint(&mut c)?;
    anyhow::ensure!(
        size >= 0 && size as usize <= c.len(),
        "chunk buffer size {size} exceeds the remaining packet ({})",
        c.len()
    );
    let mut buf = &c[..size as usize];
    let mut sections = Vec::new();
    while !buf.is_empty() {
        sections.push(parse_section(&mut buf)?);
    }
    let mut rest = &c[size as usize..];
    let block_entities = read_varint(&mut rest)?;
    anyhow::ensure!(block_entities >= 0, "negative block entity count {block_entities}");
    for _ in 0..block_entities {
        // packed xz byte + y short + type varint + NBT data — skipped
        let mut header = [0u8; 3];
        rest.read_exact(&mut header)?;
        let _ty = read_varint(&mut rest)?;
        skip_network_nbt(&mut rest)?;
    }
    Ok(ChunkColumn { x, z, sections })
}

/// Parse Keep Alive (clientbound play 0x22): i64 id.
pub fn parse_play_keep_alive(payload: &[u8]) -> Result<i64> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::KEEP_ALIVE, "not a play keep-alive packet (0x{id:02X})");
    anyhow::ensure!(c.len() >= 8, "play keep-alive truncated");
    Ok(i64::from_be_bytes(c[..8].try_into()?))
}

/// Parse Ping (clientbound play 0x31): i32 id.
pub fn parse_play_ping(payload: &[u8]) -> Result<i32> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::PING, "not a play ping packet (0x{id:02X})");
    anyhow::ensure!(c.len() >= 4, "play ping truncated");
    Ok(i32::from_be_bytes(c[..4].try_into()?))
}

/// Parse Synchronize Player Position (clientbound play 0x3C).
pub fn parse_synchronize_position(payload: &[u8]) -> Result<PlayerPosition> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SYNCHRONIZE_POSITION,
        "not a position synchronization packet (0x{id:02X})"
    );
    anyhow::ensure!(c.len() >= 8 * 3 + 4 * 2 + 1, "position synchronization truncated");
    let x = read_f64(&mut c)?;
    let y = read_f64(&mut c)?;
    let z = read_f64(&mut c)?;
    let yaw = read_f32(&mut c)?;
    let pitch = read_f32(&mut c)?;
    let flags = read_u8(&mut c)?;
    let teleport_id = read_varint(&mut c)?;
    Ok(PlayerPosition { x, y, z, yaw, pitch, flags, teleport_id })
}

/// Parse Set Center Chunk (clientbound play 0x4E): VarInt chunk X/Z.
pub fn parse_center_chunk(payload: &[u8]) -> Result<(i32, i32)> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::CENTER_CHUNK, "not a center-chunk packet (0x{id:02X})");
    let x = read_varint(&mut c)?;
    let z = read_varint(&mut c)?;
    Ok((x, z))
}

/// Parse Unload Chunk (clientbound play 0x1D): the packed coordinate long.
pub fn parse_unload_chunk(payload: &[u8]) -> Result<(i32, i32)> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::UNLOAD_CHUNK, "not an unload-chunk packet (0x{id:02X})");
    anyhow::ensure!(c.len() >= 8, "unload chunk truncated");
    let packed = i64::from_be_bytes(c[..8].try_into()?);
    Ok(((packed >> 32) as i32, (packed & 0xFFFF_FFFF) as i32))
}

/// Parse Disconnect (clientbound play 0x1A): chat JSON reason, flattened
/// to plain text (raw string kept when the JSON does not parse).
pub fn parse_play_disconnect(payload: &[u8]) -> Result<String> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::DISCONNECT, "not a play disconnect packet (0x{id:02X})");
    let reason = read_string(&mut c)?;
    let flat = serde_json::from_str::<serde_json::Value>(&reason)
        .map(|v| crate::status::extract_text(&v))
        .unwrap_or(reason);
    Ok(flat)
}

// ---------------------------------------------------------------------------
// serverbound replies
// ---------------------------------------------------------------------------

/// Confirm Teleportation (serverbound play 0x00): the teleport id verbatim.
pub fn encode_confirm_teleportation(teleport_id: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(6);
    write_varint(&mut p, serverbound::CONFIRM_TELEPORTATION);
    write_varint(&mut p, teleport_id);
    p
}

/// Chunk Batch Received (serverbound play 0x07): desired chunks per tick.
pub fn encode_chunk_batch_received(desired_per_tick: f32) -> Vec<u8> {
    let mut p = Vec::with_capacity(5);
    write_varint(&mut p, serverbound::CHUNK_BATCH_RECEIVED);
    p.extend_from_slice(&desired_per_tick.to_be_bytes());
    p
}

/// Keep Alive response (serverbound play 0x14): the i64 id verbatim.
pub fn encode_play_keep_alive_response(id: i64) -> Vec<u8> {
    let mut p = Vec::with_capacity(9);
    write_varint(&mut p, serverbound::KEEP_ALIVE);
    p.extend_from_slice(&id.to_be_bytes());
    p
}

/// Pong (serverbound play 0x22): the i32 id verbatim.
pub fn encode_play_pong(id: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(5);
    write_varint(&mut p, serverbound::PONG);
    p.extend_from_slice(&id.to_be_bytes());
    p
}

// ---------------------------------------------------------------------------
// the session pump
// ---------------------------------------------------------------------------

/// One clientbound play packet, handled.
#[derive(Debug, Clone, PartialEq)]
pub enum PlayEvent {
    /// Play-state Login: the world's static description.
    WorldInfo(WorldInfo),
    /// Synchronize Player Position (the teleport is confirmed
    /// automatically inside the pump).
    Position(PlayerPosition),
    /// A decoded chunk column.
    Chunk(ChunkColumn),
    /// The server dropped a chunk column.
    UnloadChunk { x: i32, z: i32 },
    /// The view center moved.
    CenterChunk { x: i32, z: i32 },
    /// Keep-alive probe answered.
    KeepAlive(i64),
    /// Ping probe answered.
    Ping(i32),
    /// Chunk batch started; the pacing reply was sent.
    ChunkBatch,
    /// The server closed the play session (flattened reason).
    Disconnected { reason: String },
    /// Any packet without a client surface yet (skipped silently).
    Skipped,
}

/// A live play-state session on an established connection (login and
/// configuration already finished on the SAME wire). Carries the registry
/// knowledge the configuration phase retained (dimension-type layouts).
pub struct PlaySession<S: Read + Write> {
    conn: PacketConn<S>,
    pub registry: RegistryData,
}

impl<S: Read + Write> PlaySession<S> {
    /// Read ONE clientbound play packet; answer the server's probes
    /// (keep-alive, ping, chunk batch, position teleport) in place.
    /// Unknown packet ids are skipped — the vanilla server sends plenty
    /// of gameplay traffic this client has no surface for yet.
    pub fn pump(&mut self) -> Result<PlayEvent> {
        let pkt = self.conn.read_packet().context("read play packet")?;
        anyhow::ensure!(!pkt.is_empty(), "empty play packet");
        let id = {
            let mut c: &[u8] = &pkt;
            read_varint(&mut c)?
        };
        match id {
            clientbound::LOGIN => Ok(PlayEvent::WorldInfo(parse_world_info(&pkt)?)),
            clientbound::CHUNK_DATA => Ok(PlayEvent::Chunk(parse_chunk_data(&pkt)?)),
            clientbound::KEEP_ALIVE => {
                let probe = parse_play_keep_alive(&pkt)?;
                self.conn.send_packet(&encode_play_keep_alive_response(probe))?;
                Ok(PlayEvent::KeepAlive(probe))
            }
            clientbound::PING => {
                let probe = parse_play_ping(&pkt)?;
                self.conn.send_packet(&encode_play_pong(probe))?;
                Ok(PlayEvent::Ping(probe))
            }
            clientbound::BATCH_START => {
                self.conn
                    .send_packet(&encode_chunk_batch_received(DESIRED_CHUNKS_PER_TICK))?;
                Ok(PlayEvent::ChunkBatch)
            }
            clientbound::SYNCHRONIZE_POSITION => {
                let pos = parse_synchronize_position(&pkt)?;
                self.conn
                    .send_packet(&encode_confirm_teleportation(pos.teleport_id))?;
                Ok(PlayEvent::Position(pos))
            }
            clientbound::CENTER_CHUNK => {
                let (x, z) = parse_center_chunk(&pkt)?;
                Ok(PlayEvent::CenterChunk { x, z })
            }
            clientbound::UNLOAD_CHUNK => {
                let (x, z) = parse_unload_chunk(&pkt)?;
                Ok(PlayEvent::UnloadChunk { x, z })
            }
            clientbound::DISCONNECT => {
                Ok(PlayEvent::Disconnected { reason: parse_play_disconnect(&pkt)? })
            }
            _ => Ok(PlayEvent::Skipped),
        }
    }
}

/// How the play state began.
pub enum PlayStart<S: Read + Write> {
    /// Finish Configuration acknowledged — the play state began.
    Session(PlaySession<S>),
    /// Login/config refusal or kick; carries the flattened reason.
    Disconnected { reason: String },
}

/// Handshake + offline login + configuration + the play-state switch on
/// one connection (the vanilla client never reopens the socket). The
/// returned session is ready to pump world packets.
pub fn begin_play<S: Read + Write>(
    stream: S,
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<PlayStart<S>> {
    let mut conn = PacketConn::new(stream);
    match join_play_conn(&mut conn, host, port, protocol_version, username)? {
        PlayOutcome::Joined { registry, .. } => Ok(PlayStart::Session(PlaySession {
            conn,
            registry,
        })),
        PlayOutcome::Disconnected { reason } => Ok(PlayStart::Disconnected { reason }),
    }
}

/// How a world load ended.
#[derive(Debug, Clone, PartialEq)]
pub enum WorldOutcome {
    /// World info + the first position teleport + at least one decoded
    /// chunk — the vanilla "loading screen finished" milestone.
    Loaded(WorldSnapshot),
    /// The server kicked during the world load; flattened reason.
    Disconnected { reason: String },
}

/// Pump the session until the world-load milestone (info + position +
/// first chunk) or a play disconnect. Chunks arriving before the first
/// teleport are collected; further chunks stay in the stream for the
/// caller's own pump loop.
pub fn join_world<S: Read + Write>(session: &mut PlaySession<S>) -> Result<WorldOutcome> {
    let mut info: Option<WorldInfo> = None;
    let mut player: Option<PlayerPosition> = None;
    let mut chunks: Vec<ChunkColumn> = Vec::new();
    loop {
        match session.pump()? {
            PlayEvent::WorldInfo(w) => info = Some(w),
            PlayEvent::Position(p) => {
                if player.is_none() {
                    player = Some(p);
                }
            }
            PlayEvent::Chunk(col) => chunks.push(col),
            PlayEvent::Disconnected { reason } => {
                return Ok(WorldOutcome::Disconnected { reason })
            }
            _ => {}
        }
        if info.is_some() && player.is_some() && !chunks.is_empty() {
            return Ok(WorldOutcome::Loaded(WorldSnapshot {
                info: info.unwrap(),
                player: player.unwrap(),
                chunks,
                registry: session.registry.clone(),
            }));
        }
    }
}

/// Connect + full vanilla join (login, configuration, play) with the
/// offline profile UUID for `username`, pumped to the world-load
/// milestone.
pub fn connect_and_join_world(
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<WorldOutcome> {
    let addr = format!("{host}:{port}");
    let timeout = std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS);
    let stream =
        std::net::TcpStream::connect(&addr).with_context(|| format!("tcp connect to {addr}"))?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    match begin_play(stream, host, port, protocol_version, username)? {
        PlayStart::Session(mut session) => join_world(&mut session),
        PlayStart::Disconnected { reason } => Ok(WorldOutcome::Disconnected { reason }),
    }
}

// ---------------------------------------------------------------------------
// little-endian-free cursor helpers (everything on the wire is big-endian)
// ---------------------------------------------------------------------------

fn corrupt(msg: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, msg.to_string())
}

fn read_u8(r: &mut &[u8]) -> std::io::Result<u8> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b)?;
    Ok(b[0])
}

fn read_u16(r: &mut &[u8]) -> std::io::Result<u16> {
    let mut b = [0u8; 2];
    r.read_exact(&mut b)?;
    Ok(u16::from_be_bytes(b))
}

fn read_i16(r: &mut &[u8]) -> std::io::Result<i16> {
    let mut b = [0u8; 2];
    r.read_exact(&mut b)?;
    Ok(i16::from_be_bytes(b))
}

fn read_i32(r: &mut &[u8]) -> std::io::Result<i32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(i32::from_be_bytes(b))
}

fn read_f32(r: &mut &[u8]) -> std::io::Result<f32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(f32::from_be_bytes(b))
}

fn read_f64(r: &mut &[u8]) -> std::io::Result<f64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(f64::from_be_bytes(b))
}

fn read_bool(r: &mut &[u8]) -> std::io::Result<bool> {
    Ok(read_u8(r)? != 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::login::{encode_login_success, encode_set_compression};
    use crate::varint::write_string;
    use std::net::TcpListener;
    use std::thread::JoinHandle;

    #[test]
    fn palette_packing_roundtrips_without_spanning_longs() {
        // 5-bit entries: 12 per long — the entry stream must never
        // straddle a long boundary (vanilla packing rule).
        let entries: Vec<u32> = (0..4096).map(|i| ((i * 7 + i / 13) % 3) as u32).collect();
        let bytes = pack_entries(&entries, 5);
        let mut cursor: &[u8] = &bytes;
        let got = read_packed(&mut cursor, 5, 4096).unwrap();
        assert_eq!(got, entries);
        assert!(cursor.is_empty(), "array consumed exactly");
        // partial last long: 10 entries at 5 bits = 1 long with padding
        let tail = pack_entries(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5);
        assert_eq!(tail.len(), 8);
        let mut cursor: &[u8] = &tail;
        assert_eq!(read_packed(&mut cursor, 5, 10).unwrap().len(), 10);
        // the wiki index conventions land where they say
        let idx = |x: usize, y: usize, z: usize| (y << 8) | (z << 4) | x;
        assert_eq!(idx(0, 0, 0), 0);
        assert_eq!(idx(3, 1, 2), 291, "y*256 + z*16 + x");
        assert_eq!(idx(15, 15, 15), 4095);
        let bidx = |x: usize, y: usize, z: usize| (y << 4) | (z << 2) | x;
        assert_eq!(bidx(3, 1, 2), 27, "y*16 + z*4 + x");
        assert_eq!(bidx(3, 3, 3), 63);
    }

    #[test]
    fn single_valued_and_direct_sections_decode() {
        // an all-air section: block_count 0, single-valued states+biomes
        let mut s = Vec::new();
        s.extend_from_slice(&0i16.to_be_bytes());
        s.push(0);
        write_varint(&mut s, 0); // air
        s.push(0);
        write_varint(&mut s, 187); // biomes: single valued
        let sec = parse_section(&mut s.as_slice()).unwrap();
        assert_eq!(sec.block_count, 0);
        assert_eq!(sec.blocks, vec![0u32; 4096]);
        assert_eq!(sec.biomes, vec![187u32; 64]);
        // direct palette: 15-bit global state ids straight from the array
        let ids: Vec<u32> = (0..4096).map(|i| (i % 1000) as u32 + 20_000).collect();
        let mut d = Vec::new();
        d.extend_from_slice(&4096i16.to_be_bytes());
        d.push(15);
        d.extend_from_slice(&pack_entries(&ids, 15));
        d.push(2); // biome bits 2 -> indirect palette of 3
        write_varint(&mut d, 3);
        for b in [5u32, 6, 7] {
            write_varint(&mut d, b as i32);
        }
        d.extend_from_slice(&pack_entries(&vec![0u32; 64], 2));
        let sec = parse_section(&mut d.as_slice()).unwrap();
        assert_eq!(sec.block_count, 4096);
        assert_eq!(sec.blocks, ids);
        assert_eq!(sec.biomes, vec![5u32; 64]);
        // corrupt: 3 bits per entry is not a legal block-state palette
        let mut bad = Vec::new();
        bad.extend_from_slice(&0i16.to_be_bytes());
        bad.push(3);
        assert!(parse_section(&mut bad.as_slice()).is_err());
    }

    #[test]
    fn nbt_skipper_walks_heightmaps() {
        // a heightmaps-shaped compound: two long arrays + a nested
        // compound (int + int list + string) — all skipped exactly
        let mut nbt = vec![0x0A];
        nbt.push(0x0C); // long array
        nbt.extend_from_slice(&15u16.to_be_bytes()); // len("MOTION_BLOCKING")
        nbt.extend_from_slice(b"MOTION_BLOCKING");
        nbt.extend_from_slice(&37i32.to_be_bytes());
        nbt.extend_from_slice(&[0u8; 37 * 8]);
        nbt.push(0x0C);
        nbt.extend_from_slice(&13u16.to_be_bytes());
        nbt.extend_from_slice(b"WORLD_SURFACE");
        nbt.extend_from_slice(&37i32.to_be_bytes());
        nbt.extend_from_slice(&[0u8; 37 * 8]);
        nbt.push(0x0A); // nested compound
        nbt.extend_from_slice(&5u16.to_be_bytes());
        nbt.extend_from_slice(b"stats");
        nbt.push(0x03);
        nbt.extend_from_slice(&1u16.to_be_bytes());
        nbt.extend_from_slice(b"n");
        nbt.extend_from_slice(&7i32.to_be_bytes());
        nbt.push(0x09); // list of ints
        nbt.extend_from_slice(&3u16.to_be_bytes());
        nbt.extend_from_slice(b"ids");
        nbt.push(0x03);
        nbt.extend_from_slice(&2i32.to_be_bytes());
        nbt.extend_from_slice(&5i32.to_be_bytes());
        nbt.extend_from_slice(&6i32.to_be_bytes());
        nbt.push(0x00); // end of nested
        nbt.push(0x08); // string tag: name length + name + PAYLOAD length + bytes
        nbt.extend_from_slice(&3u16.to_be_bytes()); // name length
        nbt.extend_from_slice(b"log");
        nbt.extend_from_slice(&3u16.to_be_bytes()); // payload length
        nbt.extend_from_slice(b"hi!");
        nbt.push(0x00); // end of root
        let mut cursor: &[u8] = &nbt;
        skip_network_nbt(&mut cursor).unwrap();
        assert!(cursor.is_empty(), "whole value consumed");
        // garbage root -> error
        let mut bad: &[u8] = &[0x03, 0x00];
        assert!(skip_network_nbt(&mut bad).is_err());
        // truncated long array -> error
        let mut cut = vec![0x0A, 0x0C];
        cut.extend_from_slice(&4u16.to_be_bytes());
        cut.extend_from_slice(b"abcd");
        cut.extend_from_slice(&37i32.to_be_bytes());
        cut.extend_from_slice(&[0u8; 10]);
        let mut cursor: &[u8] = &cut;
        assert!(skip_network_nbt(&mut cursor).is_err());
    }

    /// One section with an indirect 5-bit palette: stone at (0,0,0),
    /// dirt at (1,2,3) — the shape the e2e mock reuses. The data array
    /// carries PALETTE INDICES; the parser resolves them to global ids.
    fn build_stone_dirt_section() -> Vec<u8> {
        let palette = [0u32, 1, 10]; // air, stone, dirt (global state ids)
        let idx = |x: usize, y: usize, z: usize| (y << 8) | (z << 4) | x;
        let mut entries = vec![0u32; 4096]; // palette index 0 = air
        entries[idx(0, 0, 0)] = 1; // stone
        entries[idx(3, 1, 2)] = 2; // dirt
        let mut section = Vec::new();
        section.extend_from_slice(&2i16.to_be_bytes()); // block count
        section.push(5); // bits per entry -> indirect
        write_varint(&mut section, palette.len() as i32);
        for id in palette {
            write_varint(&mut section, id as i32);
        }
        section.extend_from_slice(&pack_entries(&entries, 5));
        section.push(0); // biomes: single valued
        write_varint(&mut section, 1);
        section
    }

    #[test]
    fn chunk_packet_decodes_palettized_sections() {
        let buffer = build_stone_dirt_section();
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CHUNK_DATA);
        p.extend_from_slice(&3i32.to_be_bytes()); // chunk x
        p.extend_from_slice(&(-2i32).to_be_bytes()); // chunk z
        p.push(0x0A); // heightmaps: empty anonymous compound
        p.push(0x00);
        write_varint(&mut p, buffer.len() as i32);
        p.extend_from_slice(&buffer);
        write_varint(&mut p, 0); // no block entities
        p.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // light masks — ignored
        let col = parse_chunk_data(&p).unwrap();
        assert_eq!(col.x, 3);
        assert_eq!(col.z, -2);
        assert_eq!(col.sections.len(), 1);
        let s = &col.sections[0];
        assert_eq!(s.block_count, 2);
        assert_eq!(s.blocks[0], 1, "stone at (0,0,0), resolved from palette index 1");
        assert_eq!(s.blocks[291], 10, "dirt at (1,2,3), resolved from palette index 2");
        assert_eq!(s.blocks[1], 0, "air next to it");
        assert_eq!(s.biomes, vec![1u32; 64]);
        // truncated buffer is rejected, not misparsed (cut deep into the
        // section data array — the declared chunk-buffer size no longer
        // fits the packet)
        let mut cut = p.clone();
        cut.truncate(cut.len() - 300);
        assert!(parse_chunk_data(&cut).is_err());
    }

    /// Mock server: login -> configuration -> play (the FULL vanilla
    /// arc), then a real world burst: login, center chunk, chunk batch,
    /// chunk data, keep-alive, ping, junk, unload, position teleport.
    /// Asserts every client reply server-side and returns nothing (the
    /// client-side snapshot is asserted by the caller).
    fn spawn_world_mock() -> (u16, JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let port = listener.local_addr().unwrap().port();
        let handle = std::thread::spawn(move || {
            let (stream, _) = listener.accept().expect("accept");
            let mut srv = PacketConn::new(stream);
            // ---- login (deep shapes asserted by the config.rs mock) ----
            let _hs = srv.read_packet().expect("handshake");
            let _start = srv.read_packet().expect("login start");
            srv.send_packet(&encode_set_compression(256)).unwrap();
            srv.enable_compression(256);
            srv.send_packet(&encode_login_success(&[7u8; 16], "mc2rs")).unwrap();
            for _ in 0..3 {
                srv.read_packet().unwrap(); // ack, info, brand
            }
            // probes + bulk traffic + finish (configuration)
            let mut ka = Vec::new();
            write_varint(&mut ka, 0x03); // clientbound config keep alive
            ka.extend_from_slice(&0x1122334455667788u64.to_be_bytes());
            srv.send_packet(&ka).unwrap();
            let mut pg = Vec::new();
            write_varint(&mut pg, 0x04); // clientbound config ping
            pg.extend_from_slice(&77i32.to_be_bytes());
            srv.send_packet(&pg).unwrap();
            // bulk registry traffic (configuration): the dimension_type
            // registry the client DECODES + a second registry it skips
            srv.send_packet(&config_tests::registry_packet(true)).unwrap();
            srv.send_packet(&config_tests::registry_packet(false)).unwrap();
            let mut fin = Vec::new();
            write_varint(&mut fin, 0x02); // clientbound config finish
            srv.send_packet(&fin).unwrap();
            for (id, tail) in [(0x04, 8i64), (0x05, 77i64), (0x03, -1)] {
                let resp = srv.read_packet().expect("config reply");
                let mut c: &[u8] = &resp;
                assert_eq!(read_varint(&mut c).unwrap(), id, "config reply id");
                if tail >= 0 {
                    let n = if id == 0x05 { 4 } else { 8 };
                    assert_eq!(c.len(), n, "config reply size");
                } else {
                    assert!(c.is_empty(), "ack empty");
                }
            }
            // ---- play ----
            let mut login_pkt = Vec::new();
            write_varint(&mut login_pkt, clientbound::LOGIN);
            login_pkt.extend_from_slice(&17i32.to_be_bytes()); // entity id
            login_pkt.push(0); // not hardcore
            write_varint(&mut login_pkt, 1); // one dimension
            write_string(&mut login_pkt, "minecraft:overworld");
            write_varint(&mut login_pkt, 20); // max players
            write_varint(&mut login_pkt, 8); // view distance
            write_varint(&mut login_pkt, 8); // simulation distance
            login_pkt.push(0); // reduced debug
            login_pkt.push(1); // respawn screen
            login_pkt.push(0); // limited crafting
            login_pkt.push(0); // no death location
            write_varint(&mut login_pkt, 300); // portal cooldown
            srv.send_packet(&login_pkt).unwrap();
            let mut cc = Vec::new();
            write_varint(&mut cc, clientbound::CENTER_CHUNK);
            write_varint(&mut cc, 3);
            write_varint(&mut cc, -2);
            srv.send_packet(&cc).unwrap();
            // chunk batch start -> the pacing reply MUST arrive
            let mut bs = Vec::new();
            write_varint(&mut bs, clientbound::BATCH_START);
            srv.send_packet(&bs).unwrap();
            let resp = srv.read_packet().expect("chunk batch received");
            let mut c: &[u8] = &resp;
            assert_eq!(
                read_varint(&mut c).unwrap(),
                serverbound::CHUNK_BATCH_RECEIVED,
                "batch reply id"
            );
            let pace = f32::from_be_bytes(c[..4].try_into().unwrap());
            assert!(pace > 0.0, "server must be asked to keep streaming");
            // chunk data (the stone+dirt section)
            let buffer = build_stone_dirt_section();
            let mut cd = Vec::new();
            write_varint(&mut cd, clientbound::CHUNK_DATA);
            cd.extend_from_slice(&3i32.to_be_bytes());
            cd.extend_from_slice(&(-2i32).to_be_bytes());
            cd.push(0x0A);
            cd.push(0x00); // empty heightmaps compound
            write_varint(&mut cd, buffer.len() as i32);
            cd.extend_from_slice(&buffer);
            write_varint(&mut cd, 0); // no block entities
            cd.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // light masks
            srv.send_packet(&cd).unwrap();
            // keep alive -> echo (serverbound play 0x14)
            let mut kap = Vec::new();
            write_varint(&mut kap, clientbound::KEEP_ALIVE);
            kap.extend_from_slice(&(-5_000_000_000i64).to_be_bytes());
            srv.send_packet(&kap).unwrap();
            let resp = srv.read_packet().expect("play keep alive echo");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::KEEP_ALIVE);
            assert_eq!(i64::from_be_bytes(c[..8].try_into().unwrap()), -5_000_000_000);
            // ping -> pong (serverbound play 0x22)
            let mut pgp = Vec::new();
            write_varint(&mut pgp, clientbound::PING);
            pgp.extend_from_slice(&(-7i32).to_be_bytes());
            srv.send_packet(&pgp).unwrap();
            let resp = srv.read_packet().expect("play pong");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::PONG);
            assert_eq!(i32::from_be_bytes(c[..4].try_into().unwrap()), -7);
            // junk packet (unknown id) — must be skipped without a reply
            let mut junk = vec![0x55, 1, 2, 3];
            write_string(&mut junk, "gameplay surface the client has no use for");
            srv.send_packet(&junk).unwrap();
            // unload chunk — no reply
            let mut un = Vec::new();
            write_varint(&mut un, clientbound::UNLOAD_CHUNK);
            un.extend_from_slice(&(((-2i64) << 32) | 3).to_be_bytes());
            srv.send_packet(&un).unwrap();
            // position teleport id 42 -> confirm teleportation (0x00)
            let mut pos = Vec::new();
            write_varint(&mut pos, clientbound::SYNCHRONIZE_POSITION);
            pos.extend_from_slice(&8.5f64.to_be_bytes());
            pos.extend_from_slice(&64.0f64.to_be_bytes());
            pos.extend_from_slice(&(-3.25f64).to_be_bytes());
            pos.extend_from_slice(&90.0f32.to_be_bytes());
            pos.extend_from_slice(&0.0f32.to_be_bytes());
            pos.push(0x00); // all values absolute
            write_varint(&mut pos, 42);
            srv.send_packet(&pos).unwrap();
            let resp = srv.read_packet().expect("confirm teleportation");
            let mut c: &[u8] = &resp;
            assert_eq!(
                read_varint(&mut c).unwrap(),
                serverbound::CONFIRM_TELEPORTATION,
                "confirm id"
            );
            assert_eq!(read_varint(&mut c).unwrap(), 42, "teleport id verbatim");
        });
        (port, handle)
    }

    #[test]
    fn play_loop_reaches_a_real_world() {
        let (port, srv) = spawn_world_mock();
        let out = connect_and_join_world("127.0.0.1", port, 776, "mc2rs").expect("join world");
        match out {
            WorldOutcome::Loaded(snap) => {
                assert_eq!(snap.info.entity_id, 17);
                assert_eq!(snap.info.dimensions, vec!["minecraft:overworld".to_string()]);
                assert_eq!(snap.info.view_distance, 8);
                assert_eq!(snap.info.simulation_distance, 8);
                assert_eq!(snap.info.portal_cooldown, 300);
                assert!(!snap.info.hardcore);
                assert_eq!(snap.player.teleport_id, 42);
                assert!((snap.player.x - 8.5).abs() < 1e-9);
                assert!((snap.player.y - 64.0).abs() < 1e-9);
                assert!((snap.player.z + 3.25).abs() < 1e-9);
                assert!((snap.player.yaw - 90.0).abs() < 1e-6);
                assert_eq!(snap.chunks.len(), 1, "center-chunk junk and unload are not chunks");
                assert_eq!(snap.chunks[0].x, 3);
                assert_eq!(snap.chunks[0].z, -2);
                assert_eq!(snap.chunks[0].sections[0].blocks[291], 10, "the decoded dirt");
                // the configuration registry knowledge rode along: the
                // dimension-type vertical layouts decoded pre-play
                let ow = snap.registry.dimension("minecraft:overworld").expect("overworld");
                assert_eq!((ow.min_y, ow.height), (-64, 384));
                let nether = snap.registry.dimension("minecraft:the_nether").expect("nether");
                assert_eq!((nether.min_y, nether.height), (0, 256));
            }
            other => panic!("expected Loaded, got {other:?}"),
        }
        srv.join().unwrap();
    }

    #[test]
    fn play_disconnect_surfaces_the_reason() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let srv = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut conn = PacketConn::new(stream);
            let _hs = conn.read_packet().unwrap();
            let _start = conn.read_packet().unwrap();
            conn.send_packet(&encode_set_compression(256)).unwrap();
            conn.enable_compression(256);
            conn.send_packet(&encode_login_success(&[3u8; 16], "mc2rs")).unwrap();
            for _ in 0..3 {
                conn.read_packet().unwrap();
            }
            let mut fin = Vec::new();
            write_varint(&mut fin, 0x02); // config finish
            conn.send_packet(&fin).unwrap();
            let _ack = conn.read_packet().unwrap();
            // kick during the world load (clientbound play disconnect)
            let mut d = Vec::new();
            write_varint(&mut d, clientbound::DISCONNECT);
            write_string(&mut d, r#"{"text":"You died!"}"#);
            conn.send_packet(&d).unwrap();
        });
        let out = connect_and_join_world("127.0.0.1", port, 776, "mc2rs").expect("join flow");
        assert_eq!(out, WorldOutcome::Disconnected { reason: "You died!".to_string() });
        srv.join().unwrap();
    }
}

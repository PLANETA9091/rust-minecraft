//! Play (protocol state 4): the first real world from the network — the
//! state both sides enter after Finish Configuration is acknowledged. The
//! session pump reads one clientbound packet per call, answers the
//! server's probes (keep-alive, ping, chunk batch, position teleport) and
//! decodes chunk data into resolved block/biome ids — the raw material a
//! client world needs.
//!
//! Clean-room javap-proven vanilla 26.2 shapes (play state):
//! - Login (clientbound: generated id 49): Player ID (Int), Hardcore (Bool),
//!   Levels (VarInt count + strings), Max Players (VarInt, dropped),
//!   Chunk Radius (VarInt — the view distance), Simulation Distance
//!   (VarInt), Reduced Debug / Show Death Screen / Do Limited Crafting
//!   (Bool, dropped), CommonPlayerSpawnInfo, Online Mode + Enforces
//!   Secure Chat (Bool, dropped).
//! - CommonPlayerSpawnInfo (shared by Login and Respawn): Dimension Type
//!   (VarInt registry id), Dimension (ResourceKey string), Seed (Long,
//!   dropped), Game Type + Previous Game Type (UByte, dropped), Is Debug
//!   + Is Flat (Bool, dropped), Has Death Location (Bool [+ dimension
//!   string + 8-byte position], dropped), Portal Cooldown (VarInt), Sea
//!   Level (VarInt, dropped). The CURRENT dimension and the portal
//!   cooldown are kept; the login pins the dimension, Respawn updates it.
//! - Respawn (clientbound: generated id 82): CommonPlayerSpawnInfo + a
//!   dataToKeep byte (stays in the framed payload, safely skipped).
//! - Synchronize Player Position (clientbound: generated id 72): the
//!   FIFTH javap drift wave — the 26.2 codec is the composite
//!   (VAR_INT teleportId, PositionMoveRotation, Relative.SET): the
//!   teleport id comes FIRST (the wiki-era tail), the position rides a
//!   PositionMoveRotation record — X/Y/Z (Double x3) + a DELTA MOVEMENT
//!   Vec3 (Double x3) the wiki-era shape never knew — then Yaw/Pitch
//!   (Float x2) and the relativity flags as a full INT bitmask (the
//!   1.21.2+ Relative enum: X, Y, Z, Y_ROT, X_ROT, DELTA_X/Y/Z,
//!   ROTATE_DELTA — 9 bits, not the wiki-era byte). Answered with
//!   Confirm Teleportation (serverbound 0x00, the id verbatim).
//! - Chunk Data and Update Light (clientbound: generated id 45): Chunk X/Z (Int),
//!   Heightmaps (network NBT — walked and skipped), chunk buffer (VarInt
//!   length) of sections back to back, block entities (skipped), then the
//!   light tail — javap-proven 26.2 (`ClientboundLightUpdatePacketData`):
//!   four BitSets (sky/block lit masks + empty masks), then the sky array
//!   list, then the block array list, each `readList(byteArray(2048))` —
//!   one 2048-byte nibble array per section in the lit mask, bottom to
//!   top, the count == the mask's popcount. The light sections span ONE
//!   EXTRA LAYER below and above the block sections (javap:
//!   `getLightSectionCount() = getSectionsCount() + 2`,
//!   `getMinLightSection() = minSectionY - 1`), so mask bit 0 is the
//!   layer under the column's bottom section and bit len+1 the layer
//!   above its top.
//!   One section = Block Count (Short) + block states + biomes, each a
//!   palette + packed data array in VANILLA LONG PACKING: entries fill
//!   64-bit longs LSB-first and NEVER span two longs (valuesPerLong =
//!   floor(64 / bitsPerEntry)). Block index = y*256 + z*16 + x (YZX,
//!   4096 entries); biome index = y*16 + z*4 + x (4×4×4, 64 entries).
//!   Bits per entry: block states 0 = single valued, 4..=8 indirect,
//!   9..=15 direct; biomes 0 = single, 1..=3 indirect, 4..=15 direct.
//!   Decoded arrays are fully RESOLVED global ids (flat Vec<u32>) —
//!   compact client-side storage is the future mesher's concern.
//! - Update Light (clientbound: generated id 48): VarInt chunk X/Z
//!   (NOT the chunk packet's Ints) + the SAME light tail — the
//!   standalone sibling, the column's full-replace light state on the
//!   live wire (see `parse_update_light`).
//! - Set Center Chunk (clientbound: generated id 94): VarInt chunk X/Z.
//! - Unload Chunk (clientbound: generated id 37): Long ((z << 32) | (x & 0xFFFFFFFF)).
//! - Keep Alive (clientbound: generated id 44 / serverbound: generated id 28): i64 verbatim.
//! - Ping (clientbound: generated id 61 / serverbound: generated id 45): i32 verbatim.
//! - Chunk Batch Start (clientbound: generated id 12) → Chunk Batch Received
//!   (serverbound: generated id 11, f32 desired chunks per tick) — a vanilla server
//!   PAUSES chunk streaming until this reply arrives.
//! - Disconnect (clientbound: generated id 32): chat JSON reason, flattened.
//! - Everything else is read and SKIPPED (entity metadata, time, sounds,
//!   scoreboard... — no gameplay surface yet; unknown ids never error).

use std::io::{Read, Write};

use anyhow::{Context, Result};
use std::net::TcpStream;

use crate::config::{join_play_conn, PlayOutcome, RegistryData};
use crate::framing::PacketConn;
use crate::nbt::{read_bare_nbt, write_bare_nbt, Nbt};
use crate::varint::{
    read_string, read_varint, read_varlong, write_string, write_varint, write_varlong,
};
use crate::DEFAULT_TIMEOUT_SECS;

/// Clientbound play packet ids, GENERATED from the official jar's REAL
/// bootstrapped play protocol (`mc2rs-gen dump-packet-ids` binds the
/// vanilla flows and dumps the live dispatch codec's id map). The previous
/// hand-kept wiki-era table was stale for the 26.2 protocol — every id
/// below had drifted (login 0x27 -> 49, chunk data 0x23 -> 45, ...); the
/// e2e mocks were self-consistent with the old table and masked it.
/// Runtime facts: login = 49, respawn = 82, chunk data = 45.
pub mod clientbound {
    pub use crate::generated::packet_ids::{
        CB_ADD_ENTITY as ADD_ENTITY, CB_BLOCK_UPDATE as BLOCK_UPDATE,
        CB_CHUNK_BATCH_START as BATCH_START, CB_DISCONNECT as DISCONNECT,
        CB_ENTITY_POSITION_SYNC as ENTITY_POSITION_SYNC,
        CB_FORGET_LEVEL_CHUNK as UNLOAD_CHUNK, CB_KEEP_ALIVE as KEEP_ALIVE,
        CB_LEVEL_CHUNK_WITH_LIGHT as CHUNK_DATA, CB_LIGHT_UPDATE as UPDATE_LIGHT,
        CB_LOGIN as LOGIN, CB_MOVE_ENTITY_POS as MOVE_ENTITY_POS,
        CB_MOVE_ENTITY_POS_ROT as MOVE_ENTITY_POS_ROT, CB_MOVE_ENTITY_ROT as MOVE_ENTITY_ROT,
        CB_PING as PING, CB_PLAYER_POSITION as SYNCHRONIZE_POSITION, CB_ROTATE_HEAD as ROTATE_HEAD,
        CB_REMOVE_ENTITIES as REMOVE_ENTITIES, CB_REMOVE_MOB_EFFECT as REMOVE_MOB_EFFECT,
        CB_RESPAWN as RESPAWN, CB_SECTION_BLOCKS_UPDATE as SECTION_BLOCKS_UPDATE,
        CB_SET_CHUNK_CACHE_CENTER as CENTER_CHUNK, CB_SET_ENTITY_DATA as SET_ENTITY_DATA,
        CB_SET_EQUIPMENT as SET_EQUIPMENT, CB_CONTAINER_SET_CONTENT as CONTAINER_SET_CONTENT,
        CB_CONTAINER_SET_SLOT as CONTAINER_SET_SLOT, CB_SET_HELD_SLOT as SET_HELD_SLOT,
        CB_OPEN_SCREEN as OPEN_SCREEN, CB_CONTAINER_CLOSE as CONTAINER_CLOSE,
        CB_SET_TIME as SET_TIME,
        CB_UPDATE_MOB_EFFECT as UPDATE_MOB_EFFECT,
    };
}

/// Serverbound play packet ids, GENERATED from the official jar's REAL
/// bootstrapped play protocol (see `clientbound` for the full story).
/// Runtime facts: accept teleportation = 0, chunk batch received = 11,
/// keep alive = 28, pong = 45.
pub mod serverbound {
    pub use crate::generated::packet_ids::{
        SB_ACCEPT_TELEPORTATION as CONFIRM_TELEPORTATION,
        SB_CHUNK_BATCH_RECEIVED as CHUNK_BATCH_RECEIVED, SB_KEEP_ALIVE as KEEP_ALIVE,
        SB_PONG as PONG,
    };
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

/// The play-state Login packet: the world's static description. The wire
/// shape is the PROVEN vanilla 26.2 order (javap of the real record):
/// playerId, hardcore, levels, maxPlayers (dropped), chunkRadius,
/// simulationDistance, 3 bools (dropped), CommonPlayerSpawnInfo,
/// onlineMode + enforcesSecureChat (dropped).
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
    /// The CURRENT dimension, from CommonPlayerSpawnInfo (vanilla carries
    /// it at login since 1.20.5; Respawn updates it on dimension change).
    pub current_dimension: String,
}

/// The spawn description vanilla shares between Login (initial) and
/// Respawn (dimension change) — CommonPlayerSpawnInfo, proven by javap:
/// dimensionType (registry id varint), dimension (ResourceKey = string),
/// seed (i64, dropped), gameType + previousGameType (u8, dropped),
/// isDebug + isFlat (bool, dropped), lastDeathLocation (optional GlobalPos
/// = [string, i64], dropped), portalCooldown (varint), seaLevel (dropped).
#[derive(Debug, Clone, PartialEq)]
pub struct SpawnInfo {
    /// Index into the configuration-phase dimension_type registry.
    pub dimension_type_id: i32,
    /// The dimension this spawn lives in (e.g. `minecraft:overworld`).
    pub dimension: String,
    pub portal_cooldown: i32,
}

/// Synchronize Player Position: where the server says the player is —
/// the javap-proven 26.2 field order (teleport id FIRST, position +
/// delta movement as the PositionMoveRotation record, int flags LAST).
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerPosition {
    /// The VarInt the serverbound Accept Teleportation must echo —
    /// first on the 26.2 wire.
    pub teleport_id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// The deltaMovement Vec3 the teleport carries (the relative
    /// velocity component the wiki-era shape never knew).
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub yaw: f32,
    pub pitch: f32,
    /// Relativity flags — a full INT bitmask in 26.2 (the Relative
    /// enum: bit 0 X, 1 Y, 2 Z, 3 Y_ROT, 4 X_ROT, 5 DELTA_X, 6 DELTA_Y,
    /// 7 DELTA_Z, 8 ROTATE_DELTA) — kept raw.
    pub flags: i32,
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

/// One light section's server-side truth.
#[derive(Debug, Clone, PartialEq)]
pub enum LightData {
    /// The server marked the section KNOWN-EMPTY — light 0 everywhere
    /// (the vanilla writer sets the empty bit for an all-default layer,
    /// javap: `prepareSectionData` for a `DataLayer.isEmpty()` section).
    Empty,
    /// The server sent the 2048-byte nibble array — 4096 4-bit values,
    /// vanilla YZX order (index y*256 + z*16 + x, javap:
    /// `DataLayer.getIndex(x,y,z) = (y << 8) | (z << 4) | x`), the even
    /// index in the LOW nibble (javap: getByteIndex = i >> 1,
    /// getNibbleIndex = i & 1).
    Data(Vec<u8>),
}

/// The light tail of one chunk column, decoded. Indexed by LIGHT
/// section — the vanilla engine keeps one extra layer BELOW and one
/// ABOVE the block sections (see the module doc), so index 0 is the
/// layer under the column's bottom section, 1..=len are the column's
/// own sections and len+1 the layer above; each Vec is len+2 long. A
/// `None` slot means the server sent NOTHING for that layer — the
/// honest unknown, never a guess.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChunkLight {
    pub sky: Vec<Option<LightData>>,
    pub block: Vec<Option<LightData>>,
}

/// One chunk column as sent by Chunk Data and Update Light.
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkColumn {
    pub x: i32,
    pub z: i32,
    /// Sections in buffer order — bottom of the world first.
    pub sections: Vec<ChunkSection>,
    /// The decoded light tail — sky/block data per light section.
    pub light: ChunkLight,
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

/// Parse CommonPlayerSpawnInfo — the shared Login/Respawn tail (the
/// layout is javap-proven; see `SpawnInfo`).
fn parse_common_spawn_info(c: &mut &[u8]) -> Result<SpawnInfo> {
    let dimension_type_id = read_varint(c)?;
    let dimension = read_string(c)?;
    let _seed = read_i64(c)?;
    let _game_type = read_u8(c)?;
    let _previous_game_type = read_u8(c)?;
    let _is_debug = read_bool(c)?;
    let _is_flat = read_bool(c)?;
    let has_death_location = read_bool(c)?;
    if has_death_location {
        let _dim = read_string(c)?;
        let mut pos = [0u8; 8];
        c.read_exact(&mut pos)?;
    }
    let portal_cooldown = read_varint(c)?;
    let _sea_level = read_varint(c)?;
    Ok(SpawnInfo { dimension_type_id, dimension, portal_cooldown })
}

/// Parse the play-state Login (clientbound: generated id 49) — the
/// PROVEN vanilla 26.2 field order.
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
    let _show_death_screen = read_bool(&mut c)?;
    let _limited_crafting = read_bool(&mut c)?;
    let spawn = parse_common_spawn_info(&mut c)?;
    let _online_mode = read_bool(&mut c)?;
    let _enforces_secure_chat = read_bool(&mut c)?;
    Ok(WorldInfo {
        entity_id,
        hardcore,
        dimensions,
        view_distance,
        simulation_distance,
        portal_cooldown: spawn.portal_cooldown,
        current_dimension: spawn.dimension,
    })
}

/// Parse Respawn (clientbound: generated id 82) — a dimension change.
/// The payload is CommonPlayerSpawnInfo + a dataToKeep byte; the fields
/// the model doesn't need stay in the framed payload (skipped safely).
pub fn parse_respawn(payload: &[u8]) -> Result<SpawnInfo> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::RESPAWN, "not a respawn packet (0x{id:02X})");
    parse_common_spawn_info(&mut c)
}

/// Parse Chunk Data and Update Light (clientbound: generated id 45) into a decoded
/// column. Heightmaps and block entities are walked and skipped; the
/// light tail after them is decoded into per-light-section sky/block
/// data (see [`ChunkLight`]) — and it is the packet's END: trailing
/// bytes after the light lists are the drift they look like.
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
    // the light tail — javap-proven 26.2 shape (four BitSets, then the
    // sky array list, then the block array list; everything after the
    // block entities IS light data, and it is the packet's end), shared
    // with the standalone Update Light packet
    let light = parse_light_tail(&mut rest, Some(sections.len() + 2))?;
    anyhow::ensure!(
        rest.is_empty(),
        "{} trailing bytes after the chunk light data",
        rest.len()
    );
    Ok(ChunkColumn { x, z, sections, light })
}

/// The light tail shared by Chunk Data and the standalone Update Light
/// (javap: both carry the exact same `ClientboundLightUpdatePacketData`):
/// four BitSets (sky/block lit masks + empty masks), then the sky array
/// list, then the block list. `total` is the column's light-section
/// count when the caller knows it (the chunk packet: sections + 2);
/// `None` (the standalone packet — the wire carries no world height
/// there) sizes the result to the masks' own span, where the range
/// checks below pass vacuously and the MODEL validates the bits against
/// its stored column.
fn parse_light_tail(r: &mut &[u8], total: Option<usize>) -> Result<ChunkLight> {
    let sky_mask = read_bitset(r)?;
    let block_mask = read_bitset(r)?;
    let empty_sky_mask = read_bitset(r)?;
    let empty_block_mask = read_bitset(r)?;
    let size = total.unwrap_or_else(|| {
        light_span(&[&sky_mask, &block_mask, &empty_sky_mask, &empty_block_mask])
    });
    let mut light = ChunkLight::default();
    light.sky = parse_light_arrays(r, &sky_mask, size, "sky light")?;
    light.block = parse_light_arrays(r, &block_mask, size, "block light")?;
    apply_empty_mask(&mut light.sky, &empty_sky_mask, size, "sky light")?;
    apply_empty_mask(&mut light.block, &empty_block_mask, size, "block light")?;
    Ok(light)
}

/// The span (max documented bit + 1) across a set of masks — the
/// standalone Update Light parser's section count.
fn light_span(masks: &[&[u64]]) -> usize {
    masks
        .iter()
        .flat_map(|m| bitset_bits(m))
        .max()
        .map_or(0, |b| b + 1)
}

/// Parse Update Light (clientbound: generated id 48) — the standalone
/// sibling of the chunk packet's light tail: VarInt chunk X/Z (VarInts
/// here, NOT the chunk packet's Ints — javap of the real constructor)
/// followed by the SAME light data. The update carries the column's
/// COMPLETE light state (the vanilla writer serializes the engine's
/// current storage), so the model REPLACES the stored light with it;
/// the model also validates the mask bits against the column's real
/// range — the wire has no world height for that check.
pub fn parse_update_light(payload: &[u8]) -> Result<(i32, i32, ChunkLight)> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::UPDATE_LIGHT,
        "not an update light packet (0x{id:02X})"
    );
    let x = read_varint(&mut c)?;
    let z = read_varint(&mut c)?;
    let light = parse_light_tail(&mut c, None)?;
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the update light data",
        c.len()
    );
    Ok((x, z, light))
}

/// Parse Set Time (clientbound play, generated id 113). The javap-proven
/// 26.2 shape: `LONG gameTime` then `ByteBufCodecs.map(IntFunction,
/// WorldClock.STREAM_CODEC, ClockNetworkState.STREAM_CODEC)` — a VarInt
/// count then that many (VarInt clock id, VAR_LONG totalTicks,
/// FLOAT partialTick, FLOAT rate) entries. The count is validated
/// against the remaining bytes BEFORE any allocation — each entry is at
/// least 10 bytes on the wire (min 1-byte VarInt id, 1-byte VarLong,
/// 4 + 4 floats), so a lying count cannot reserve memory the packet
/// cannot back.
pub fn parse_set_time(payload: &[u8]) -> Result<TimeSync> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::SET_TIME, "not a set time packet (0x{id:02X})");
    let game_time = read_i64(&mut c)?;
    let count = read_varint(&mut c)?;
    if count < 0 || count as usize > c.len() / 10 {
        return Err(anyhow::anyhow!(
            "absurd clock count {count} for {} remaining bytes",
            c.len()
        ));
    }
    let mut clocks = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let id = read_varint(&mut c)?;
        let total_ticks = read_varlong(&mut c)?;
        let partial_tick = read_f32(&mut c)?;
        let rate = read_f32(&mut c)?;
        clocks.push(ClockUpdate { id, total_ticks, partial_tick, rate });
    }
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the set time data",
        c.len()
    );
    Ok(TimeSync { game_time, clocks })
}

/// Read a protocol BitSet: VarInt long count then that many big-endian
/// longs (javap: `FriendlyByteBuf.readBitSet`). The long count is
/// validated BEFORE any allocation — a lying VarInt must not reserve
/// gigabytes for a packet that cannot back it.
fn read_bitset(r: &mut &[u8]) -> std::io::Result<Vec<u64>> {
    let longs = read_varint(r)?;
    if !(0..=4096).contains(&longs) {
        return Err(corrupt(&format!("absurd bitset long count {longs}")));
    }
    let mut out = Vec::new();
    for _ in 0..longs {
        let mut b = [0u8; 8];
        r.read_exact(&mut b)?;
        out.push(u64::from_be_bytes(b));
    }
    Ok(out)
}

/// The set-bit positions of a BitSet, low to high — the order the
/// vanilla writer emits the arrays in.
fn bitset_bits(mask: &[u64]) -> Vec<usize> {
    let mut out = Vec::new();
    for (li, l) in mask.iter().enumerate() {
        for b in 0..64 {
            if (l >> b) & 1 == 1 {
                out.push(li * 64 + b);
            }
        }
    }
    out
}

/// Parse one layer's array list (javap: `readList(byteArray(2048))`):
/// a VarInt count that MUST equal the lit mask's popcount (the vanilla
/// writer emits exactly one array per set bit — anything else is
/// drift), then one 2048-byte array per set bit, bottom to top, placed
/// at the bit's light-section index. Bits the mask never sets stay
/// `None` — the server said nothing about them.
fn parse_light_arrays(
    r: &mut &[u8],
    mask: &[u64],
    total: usize,
    kind: &str,
) -> std::io::Result<Vec<Option<LightData>>> {
    let count = read_varint(r)?;
    let bits = bitset_bits(mask);
    if count < 0 || count as usize != bits.len() {
        return Err(corrupt(&format!(
            "{kind}: {count} light arrays for {} mask bits",
            bits.len()
        )));
    }
    let mut out: Vec<Option<LightData>> = vec![None; total];
    for bit in bits {
        if bit >= total {
            return Err(corrupt(&format!(
                "{kind}: mask bit {bit} outside the column's light range ({total})"
            )));
        }
        let len = read_varint(r)?;
        if len != 2048 {
            return Err(corrupt(&format!(
                "{kind}: light array length {len}, expected 2048"
            )));
        }
        let mut arr = vec![0u8; 2048];
        r.read_exact(&mut arr)?;
        out[bit] = Some(LightData::Data(arr));
    }
    Ok(out)
}

/// Apply an EMPTY mask — the server's known-zero sections (the vanilla
/// writer marks an all-default layer empty and NEVER also sends an
/// array for it). A section both lit and empty is drift; so is a bit
/// outside the column's light range.
fn apply_empty_mask(
    layer: &mut [Option<LightData>],
    mask: &[u64],
    total: usize,
    kind: &str,
) -> std::io::Result<()> {
    for bit in bitset_bits(mask) {
        if bit >= total {
            return Err(corrupt(&format!(
                "{kind}: empty-mask bit {bit} outside the column's light range ({total})"
            )));
        }
        if layer[bit].is_some() {
            return Err(corrupt(&format!(
                "{kind}: light section {bit} marked both lit and empty"
            )));
        }
        layer[bit] = Some(LightData::Empty);
    }
    Ok(())
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

/// Parse Synchronize Player Position (clientbound: generated id 72) —
/// the javap-proven 26.2 composite (VAR_INT teleportId,
/// PositionMoveRotation.STREAM_CODEC, Relative.SET_STREAM_CODEC):
/// VarInt teleport id, X/Y/Z + delta-movement (six doubles), yaw/pitch
/// (two floats), the relativity flags as a raw i32 bitmask. Strict:
/// trailing bytes are the drift they look like.
pub fn parse_synchronize_position(payload: &[u8]) -> Result<PlayerPosition> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SYNCHRONIZE_POSITION,
        "not a position synchronization packet (0x{id:02X})"
    );
    anyhow::ensure!(
        c.len() >= 1 + 8 * 6 + 4 * 2 + 4,
        "position synchronization truncated"
    );
    let teleport_id = read_varint(&mut c)?;
    let x = read_f64(&mut c)?;
    let y = read_f64(&mut c)?;
    let z = read_f64(&mut c)?;
    let dx = read_f64(&mut c)?;
    let dy = read_f64(&mut c)?;
    let dz = read_f64(&mut c)?;
    let yaw = read_f32(&mut c)?;
    let pitch = read_f32(&mut c)?;
    let flags = read_i32(&mut c)?;
    anyhow::ensure!(
        c.is_empty(),
        "trailing bytes after the position synchronization fields"
    );
    Ok(PlayerPosition { teleport_id, x, y, z, dx, dy, dz, yaw, pitch, flags })
}

/// Parse Set Center Chunk (clientbound: generated id 94): VarInt chunk X/Z.
pub fn parse_center_chunk(payload: &[u8]) -> Result<(i32, i32)> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::CENTER_CHUNK, "not a center-chunk packet (0x{id:02X})");
    let x = read_varint(&mut c)?;
    let z = read_varint(&mut c)?;
    Ok((x, z))
}

/// Parse Unload Chunk (clientbound: generated id 37): the packed coordinate long.
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

/// Decode the wire's packed block position long — javap of the real 26.2
/// (`FriendlyByteBuf.readBlockPos` -> `BlockPos.of(long)`; the constants
/// from `BlockPos.<clinit>`): `PACKED_HORIZONTAL_LENGTH` = 26 bits
/// (`log2(smallestEncompassingPowerOfTwo(30000000)) + 1`),
/// `PACKED_Y_LENGTH` = 12 (`64 - 2 * 26`), so the layout is
/// `((x & 0x3FFFFFF) << 38) | (y & 0xFFF) | ((z & 0x3FFFFFF) << 12)` and
/// the getters sign-extend with the two-shift ARITHMETIC walk
/// (`getX = (packed << 0) >> 38`, `getY = (packed << 52) >> 52`,
/// `getZ = (packed << 26) >> 38` — `lshr`, not the logical shift).
fn decode_block_pos(packed: i64) -> (i32, i32, i32) {
    let x = ((packed << 0) >> 38) as i32;
    let y = ((packed << 52) >> 52) as i32;
    let z = ((packed << 26) >> 38) as i32;
    (x, y, z)
}

/// Decode the wire's packed section position long — javap of the real
/// 26.2 (`SectionPos.STREAM_CODEC` = `ByteBufCodecs.LONG` mapped through
/// `SectionPos.of(long)`; the layout from `SectionPos.asLong(III)`):
/// `((x & 0x3FFFFF) << 42) | (y & 0xFFFFF) | ((z & 0x3FFFFF) << 20)` —
/// the SECTION coordinates (a section is 16 blocks), sign-extended the
/// same two-shift way (`x = packed >> 42`, `y = (packed << 44) >> 44`,
/// `z = (packed << 22) >> 42`).
fn decode_section_pos(packed: i64) -> (i32, i32, i32) {
    let sx = ((packed << 0) >> 42) as i32;
    let sy = ((packed << 44) >> 44) as i32;
    let sz = ((packed << 22) >> 42) as i32;
    (sx, sy, sz)
}

/// Parse Block Update (clientbound play, generated id 8 —
/// `minecraft:block_update`). javap of the 26.2 STREAM_CODEC:
/// `BlockPos.STREAM_CODEC` (a packed position LONG, [`decode_block_pos`])
/// then `ByteBufCodecs.idMapper(BLOCK_STATE_REGISTRY)` — a VarInt global
/// block state id, the SAME id space the chunk decoder resolves. One
/// block changed: the world's own placement/breaking/fluid traffic.
pub fn parse_block_update(payload: &[u8]) -> Result<(i32, i32, i32, u32)> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::BLOCK_UPDATE,
        "not a block update packet (0x{id:02X})"
    );
    let packed = read_i64(&mut c)?;
    let state = read_varint(&mut c)?;
    anyhow::ensure!(state >= 0, "negative block state id {state}");
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the block update",
        c.len()
    );
    let (x, y, z) = decode_block_pos(packed);
    Ok((x, y, z, state as u32))
}

/// Section Blocks Update (clientbound play, generated id 84 —
/// `minecraft:section_blocks_update`): a batch of block changes inside
/// ONE 16³ section (the vanilla writer's multi-change shape — piston
/// moves, explosions, fluid sweeps).
#[derive(Debug, Clone, PartialEq)]
pub struct SectionBlocks {
    /// Section x (world x / 16, arithmetic shift — negative half-plane
    /// honest).
    pub sx: i32,
    /// Section y — the WORLD section coordinate (the column's bottom
    /// section is `min_y / 16`, which is NEGATIVE for the overworld's
    /// -64 floor).
    pub sy: i32,
    /// Section z (world z / 16, arithmetic shift).
    pub sz: i32,
    /// The changed blocks: SECTION-RELATIVE x/y/z (0..=15 each) plus
    /// the global block state id (the chunk decoder's id space).
    pub blocks: Vec<(i32, i32, i32, u32)>,
}

/// Parse Section Blocks Update. javap of the 26.2 constructor:
/// `SectionPos.STREAM_CODEC` (a packed section LONG, [`decode_section_pos`])
/// then a VarInt count then that many VarLong entries —
/// `position = packed & 4095` (the LOW 12 bits, javap `ldc2_w 4095l`),
/// `state id = packed >>> 12` (LOGICAL shift, javap `lushr 12`), and the
/// 12-bit local packs `(x << 8) | (z << 4) | y` (javap
/// `sectionRelativeX/Y/Z`: x = short `>>> 8 & 15`, z = `>>> 4 & 15`,
/// y = `& 15`). The count is validated against the remaining bytes
/// BEFORE any allocation — each entry is at least one wire byte, so a
/// lying count cannot reserve memory the packet cannot back.
pub fn parse_section_blocks(payload: &[u8]) -> Result<SectionBlocks> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SECTION_BLOCKS_UPDATE,
        "not a section blocks update packet (0x{id:02X})"
    );
    let packed = read_i64(&mut c)?;
    let (sx, sy, sz) = decode_section_pos(packed);
    let count = read_varint(&mut c)?;
    if count < 0 || count as usize > c.len() {
        return Err(anyhow::anyhow!(
            "absurd block count {count} for {} remaining bytes",
            c.len()
        ));
    }
    let mut blocks = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let entry = read_varlong(&mut c)?;
        let local = (entry & 0xFFF) as u16;
        let state = ((entry as u64) >> 12) as u32;
        let lx = ((local >> 8) & 0xF) as i32;
        let ly = (local & 0xF) as i32;
        let lz = ((local >> 4) & 0xF) as i32;
        blocks.push((lx, ly, lz, state));
    }
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the section blocks update",
        c.len()
    );
    Ok(SectionBlocks { sx, sy, sz, blocks })
}

// ---------------------------------------------------------------------------
// entities: the world's own population traffic
// ---------------------------------------------------------------------------

/// Decode the wire's low-precision Vec3 — javap of the real 26.2
/// (`Vec3.LP_STREAM_CODEC` delegates to `net.minecraft.network.LpVec3`,
/// NOT the wiki-era 3-shorts-times-8000 layout). The wire is 6 bytes
/// packed as a 48-bit big-endian integer:
/// * byte 0 carries the scale's low 2 bits (bits 0-1), the continuation
///   flag (bit 2, `hasContinuationBit` = `(i & 4) == 4`) and the data's
///   low 5 bits — a zero FIRST byte is the ZERO sentinel (nothing else
///   is read),
/// * the remaining 43 data bits split into three 15-bit lanes at
///   shifts 3 / 18 / 33 (`read`: `packed >>> 3 / 18 / 33`),
/// * each lane `unpack(v) = min(v & 32767, 32766) * 2 / 32766 - 1`
///   (javap: the 15-bit value CLAMPS at 32766, scales to [-1, 1]),
/// * the value is multiplied by the scale (a double): the low 2 bits
///   ride byte 0; when the continuation flag is set the scale's high
///   bits follow as one VarInt (`scale |= (varint & 0xFFFFFFFF) << 2`).
fn decode_lp_vec3(r: &mut &[u8]) -> std::io::Result<[f64; 3]> {
    let b0 = read_u8(r)?;
    if b0 == 0 {
        return Ok([0.0; 3]);
    }
    let b1 = read_u8(r)?;
    let tail = read_u32(r)? as u64;
    let packed = (tail << 16) | ((b1 as u64) << 8) | (b0 as u64);
    let mut scale = (b0 & 3) as u64;
    if b0 & 4 != 0 {
        scale |= (read_varint(r)? as u32 as u64) << 2;
    }
    let unpack = |shift: u32| {
        let v = ((packed >> shift) & 0x7FFF) as f64;
        (v.min(32766.0) * 2.0 / 32766.0) - 1.0
    };
    Ok([
        unpack(3) * scale as f64,
        unpack(18) * scale as f64,
        unpack(33) * scale as f64,
    ])
}

/// The unpacked Add Entity payload (clientbound play, generated id 1 —
/// `minecraft:add_entity`). javap of the 26.2 STREAM_CODEC reading
/// constructor, in wire order: VarInt entity id, UUID (two BIG-ENDIAN
/// longs: most then least significant), VarInt entity TYPE registry id
/// (`ByteBufCodecs.registry(Registries.ENTITY_TYPE)` — the same registry
/// the configuration phase syncs), three BIG-ENDIAN doubles (position),
/// the low-precision movement vector ([`decode_lp_vec3`]), three angle
/// BYTES (x/y/yHead — `Mth.unpackDegrees(b) = b * 360 / 256`, the
/// sign-extended byte's [-180, 180) degrees), and the VarInt data
/// (entity-type specific: the minecart's display block, the
/// projectile's owner shard, ...). The world's population: the mob that
/// walked in, the item that dropped, the arrow in flight.
#[derive(Debug, Clone, PartialEq)]
pub struct EntitySpawn {
    /// The entity's numeric id (the key every later move/remove names).
    pub id: i32,
    /// The entity's UUID — the 128 wire bits packed most-long first.
    pub uuid: u128,
    /// The entity type's registry id (resolved by the synced registry).
    pub type_id: i32,
    /// World position (full-precision doubles).
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// The low-precision movement vector ([`decode_lp_vec3`]).
    pub movement: [f64; 3],
    /// Pitch in degrees (the byte unpacked the vanilla way).
    pub x_rot: f32,
    /// Yaw in degrees (the byte unpacked the vanilla way).
    pub y_rot: f32,
    /// Head yaw in degrees (the byte unpacked the vanilla way).
    pub y_head_rot: f32,
    /// The entity-type-specific data int (verbatim).
    pub data: i32,
}

/// Parse Add Entity (clientbound play, generated id 1). The count
/// checks are per-field (each read fails on exhaustion) and trailing
/// bytes are drift.
pub fn parse_add_entity(payload: &[u8]) -> Result<EntitySpawn> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::ADD_ENTITY, "not an add entity packet (0x{id:02X})");
    let entity_id = read_varint(&mut c)?;
    let msb = read_i64(&mut c)? as u64;
    let lsb = read_i64(&mut c)? as u64;
    let uuid = ((msb as u128) << 64) | (lsb as u128);
    let type_id = read_varint(&mut c)?;
    let x = read_f64(&mut c)?;
    let y = read_f64(&mut c)?;
    let z = read_f64(&mut c)?;
    let movement = decode_lp_vec3(&mut c)?;
    let unpack_degrees = |b: i8| (b as i32 * 360) as f32 / 256.0;
    let x_rot = unpack_degrees(read_i8(&mut c)?);
    let y_rot = unpack_degrees(read_i8(&mut c)?);
    let y_head_rot = unpack_degrees(read_i8(&mut c)?);
    let data = read_varint(&mut c)?;
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the add entity packet",
        c.len()
    );
    Ok(EntitySpawn {
        id: entity_id,
        uuid,
        type_id,
        x,
        y,
        z,
        movement,
        x_rot,
        y_rot,
        y_head_rot,
        data,
    })
}

/// Parse Remove Entities (clientbound play, generated id 77 —
/// `minecraft:remove_entities`). javap of the 26.2 reading constructor:
/// `FriendlyByteBuf.readIntIdList` — a VarInt count then that many
/// VarInt entity ids. The count is validated against the remaining
/// bytes BEFORE any allocation (each id is at least one wire byte — a
/// lying count cannot reserve memory the packet cannot back).
pub fn parse_remove_entities(payload: &[u8]) -> Result<Vec<i32>> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::REMOVE_ENTITIES,
        "not a remove entities packet (0x{id:02X})"
    );
    let count = read_varint(&mut c)?;
    if count < 0 || count as usize > c.len() {
        return Err(anyhow::anyhow!(
            "absurd entity count {count} for {} remaining bytes",
            c.len()
        ));
    }
    let mut ids = Vec::with_capacity(count as usize);
    for _ in 0..count {
        ids.push(read_varint(&mut c)?);
    }
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the remove entities packet",
        c.len()
    );
    Ok(ids)
}

/// The unpacked Entity Position Sync payload (clientbound play,
/// generated id 35 — `minecraft:entity_position_sync`). javap of the
/// 26.2 STREAM_CODEC: `StreamCodec.composite(VAR_INT, VAR_INT, ...)` of
/// the entity id, `PositionMoveRotation.STREAM_CODEC`, and BOOL — the
/// record is `(int id, PositionMoveRotation values, boolean onGround)`
/// and the rotation record composes `Vec3.STREAM_CODEC` (position,
/// FULL-precision doubles — not the movement lane), `Vec3.STREAM_CODEC`
/// (deltaMovement), FLOAT yRot, FLOAT xRot.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityPositionSync {
    /// The entity's numeric id.
    pub id: i32,
    /// World position (full-precision doubles).
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// The entity's velocity (full-precision doubles).
    pub delta: [f64; 3],
    /// Yaw in degrees.
    pub y_rot: f32,
    /// Pitch in degrees.
    pub x_rot: f32,
    /// The entity's ground contact flag (verbatim).
    pub on_ground: bool,
}

/// Parse Entity Position Sync — the server's authoritative "this entity
/// is HERE now" teleport/position correction.
pub fn parse_entity_position_sync(payload: &[u8]) -> Result<EntityPositionSync> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::ENTITY_POSITION_SYNC,
        "not an entity position sync packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let x = read_f64(&mut c)?;
    let y = read_f64(&mut c)?;
    let z = read_f64(&mut c)?;
    let dx = read_f64(&mut c)?;
    let dy = read_f64(&mut c)?;
    let dz = read_f64(&mut c)?;
    let y_rot = read_f32(&mut c)?;
    let x_rot = read_f32(&mut c)?;
    let on_ground = read_u8(&mut c)? != 0;
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the entity position sync packet",
        c.len()
    );
    Ok(EntityPositionSync {
        id: entity_id,
        x,
        y,
        z,
        delta: [dx, dy, dz],
        y_rot,
        x_rot,
        on_ground,
    })
}

/// The unpacked Move Entity Pos payload (clientbound play, generated
/// id 53 — `minecraft:move_entity_pos`). javap of the 26.2
/// `ClientboundMoveEntityPacket$Pos.read`: a VarInt entity id, THREE
/// SHORT deltas and a BOOL ground flag. The deltas ride RAW 1/4096
/// grid units — the client's `VecDeltaCodec` holds the base and does
/// the quantize-and-add walk, so this side of the wire is exactly the
/// shorts the server wrote (no scaling, no absolute positions).
#[derive(Debug, Clone, PartialEq)]
pub struct EntityDeltaPos {
    /// The entity's numeric id.
    pub id: i32,
    /// The raw x delta (1/4096 grid units; the consumer quantizes the
    /// base and adds — see the model's `delta_axis`).
    pub dx: i16,
    /// The raw y delta (1/4096 grid units).
    pub dy: i16,
    /// The raw z delta (1/4096 grid units).
    pub dz: i16,
    /// The entity's ground contact flag (verbatim).
    pub on_ground: bool,
}

/// Parse Move Entity Pos — the per-tick walk the sync's teleports
/// don't cover.
pub fn parse_move_entity_pos(payload: &[u8]) -> Result<EntityDeltaPos> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::MOVE_ENTITY_POS,
        "not a move entity pos packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let dx = read_i16(&mut c)?;
    let dy = read_i16(&mut c)?;
    let dz = read_i16(&mut c)?;
    let on_ground = read_u8(&mut c)? != 0;
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the move entity pos packet",
        c.len()
    );
    Ok(EntityDeltaPos {
        id: entity_id,
        dx,
        dy,
        dz,
        on_ground,
    })
}

/// The unpacked Move Entity Pos and Rotation payload (clientbound
/// play, generated id 54 — `minecraft:move_entity_pos_rot`). javap of
/// the 26.2 `ClientboundMoveEntityPacket$PosRot.read`: the Pos layout
/// plus TWO angle BYTES (yaw then pitch, each `Mth.unpackDegrees(b) =
/// b * 360 / 256` — the Add Entity angles' own quantization).
#[derive(Debug, Clone, PartialEq)]
pub struct EntityDeltaPosRot {
    /// The entity's numeric id.
    pub id: i32,
    /// The raw x delta (1/4096 grid units).
    pub dx: i16,
    /// The raw y delta (1/4096 grid units).
    pub dy: i16,
    /// The raw z delta (1/4096 grid units).
    pub dz: i16,
    /// Yaw in degrees (the byte unpacked the vanilla way).
    pub y_rot: f32,
    /// Pitch in degrees (the byte unpacked the vanilla way).
    pub x_rot: f32,
    /// The entity's ground contact flag (verbatim).
    pub on_ground: bool,
}

/// Parse Move Entity Pos and Rotation — the walk plus the turn.
pub fn parse_move_entity_pos_rot(payload: &[u8]) -> Result<EntityDeltaPosRot> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::MOVE_ENTITY_POS_ROT,
        "not a move entity pos rot packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let dx = read_i16(&mut c)?;
    let dy = read_i16(&mut c)?;
    let dz = read_i16(&mut c)?;
    let unpack_degrees = |b: i8| (b as i32 * 360) as f32 / 256.0;
    let y_rot = unpack_degrees(read_i8(&mut c)?);
    let x_rot = unpack_degrees(read_i8(&mut c)?);
    let on_ground = read_u8(&mut c)? != 0;
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the move entity pos rot packet",
        c.len()
    );
    Ok(EntityDeltaPosRot {
        id: entity_id,
        dx,
        dy,
        dz,
        y_rot,
        x_rot,
        on_ground,
    })
}

/// The unpacked Move Entity Rot payload (clientbound play, generated
/// id 56 — `minecraft:move_entity_rot`). javap of the 26.2
/// `ClientboundMoveEntityPacket$Rot.read`: a VarInt entity id, TWO
/// angle BYTES and a BOOL — no position lane (the body turns in
/// place). NOTE the generated table: 55 is the minecart, 56 is this
/// packet — the wiki-era numbering would have shipped the wrong id.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityDeltaRot {
    /// The entity's numeric id.
    pub id: i32,
    /// Yaw in degrees (the byte unpacked the vanilla way).
    pub y_rot: f32,
    /// Pitch in degrees (the byte unpacked the vanilla way).
    pub x_rot: f32,
    /// The entity's ground contact flag (verbatim).
    pub on_ground: bool,
}

/// Parse Move Entity Rot — rotation-only, position untouched.
pub fn parse_move_entity_rot(payload: &[u8]) -> Result<EntityDeltaRot> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::MOVE_ENTITY_ROT,
        "not a move entity rot packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let unpack_degrees = |b: i8| (b as i32 * 360) as f32 / 256.0;
    let y_rot = unpack_degrees(read_i8(&mut c)?);
    let x_rot = unpack_degrees(read_i8(&mut c)?);
    let on_ground = read_u8(&mut c)? != 0;
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the move entity rot packet",
        c.len()
    );
    Ok(EntityDeltaRot {
        id: entity_id,
        y_rot,
        x_rot,
        on_ground,
    })
}

/// The unpacked Rotate Head payload (clientbound play, generated id
/// 83 — `minecraft:rotate_head`). javap of the 26.2
/// `ClientboundRotateHeadPacket`: the read is the private
/// constructor — a VarInt entity id and ONE angle BYTE (`yHeadRot`).
/// The head yaw is its own channel: a mob's body can face one way
/// while its head tracks the player, so the packet family rides
/// alongside the move packets, never inside them.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityHeadRot {
    /// The entity's numeric id.
    pub id: i32,
    /// Head yaw in degrees (the byte unpacked the vanilla way).
    pub y_head_rot: f32,
}

/// Parse Rotate Head — the head's own wire channel.
pub fn parse_rotate_head(payload: &[u8]) -> Result<EntityHeadRot> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::ROTATE_HEAD,
        "not a rotate head packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let unpack_degrees = |b: i8| (b as i32 * 360) as f32 / 256.0;
    let y_head_rot = unpack_degrees(read_i8(&mut c)?);
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the rotate head packet",
        c.len()
    );
    Ok(EntityHeadRot {
        id: entity_id,
        y_head_rot,
    })
}

// ---------------------------------------------------------------------------
// entity metadata (Set Entity Data)
// ---------------------------------------------------------------------------

/// The Set Entity Data serializer table. The wire ids are the 26.2
/// `EntityDataSerializers.<clinit>` REGISTRATION order — each
/// `registerSerializer` call appends into a
/// `CrudeIncrementalIntIdentityHashBiMap`, so the id IS the call
/// sequence — and that sequence is NOT the class's field declaration
/// order: VILLAGER_DATA (18) and OPTIONAL_GLOBAL_POS (33) trade places
/// against the wiki-era table, and the animal variant/sound-variant
/// Holders fill 21-32 in clinit sequence. The wiki-era guess would
/// have shipped VILLAGER_DATA at 19 and OPTIONAL_GLOBAL_POS at 18 —
/// both wrong (the tests pin the refusal at the real ids).
pub mod entity_data_serializers {
    /// `ByteBufCodecs.BYTE` — one signed byte.
    pub const BYTE: i32 = 0;
    /// `ByteBufCodecs.VAR_INT`.
    pub const INT: i32 = 1;
    /// `ByteBufCodecs.VAR_LONG`.
    pub const LONG: i32 = 2;
    /// `ByteBufCodecs.FLOAT` — a big-endian f32 (the sync floats' own
    /// codec).
    pub const FLOAT: i32 = 3;
    /// `ByteBufCodecs.STRING_UTF8` — `stringUtf8(32767)`: a VarInt
    /// length + UTF-8, capped at 32767 (not the wiki-era 65536).
    pub const STRING: i32 = 4;
    /// `ComponentSerialization.TRUSTED_STREAM_CODEC` =
    /// `fromCodecWithRegistriesTrusted` = `tagCodec` + the NBT-ops
    /// adapter: ONE BARE network NBT tag on the wire (a TAG_String for
    /// plain text, a TAG_Compound for the structured form) — decoded
    /// with the bare-tag reader ("trusted" = the accounter is
    /// unlimited, the server's own packet).
    pub const COMPONENT: i32 = 5;
    /// The optional op over COMPONENT — a boolean flag, then the bare
    /// tag only if present (the CustomName surface's own lane).
    pub const OPTIONAL_COMPONENT: i32 = 6;
    /// `ItemStack.OPTIONAL_STREAM_CODEC` — the object-graph lane.
    /// The javap walk (three hops): `ItemStack$1.decode` = a VarInt
    /// COUNT (`ifgt` — count <= 0 is `ItemStack.EMPTY`, NOTHING else
    /// on the wire), then `Item.STREAM_CODEC` =
    /// `holderRegistry(Registries.ITEM)` (a VarInt registry id, the
    /// `ByteBufCodecs$29` byIdOrThrow the villager round pinned), then
    /// `DataComponentPatch.STREAM_CODEC` = `createStreamCodec($1)` =
    /// `DataComponentPatch$3.decode` — a VarInt ADDED count, a VarInt
    /// REMOVED count (both zero = the EMPTY patch, the early return at
    /// 10-21), then `added` × {type id, the type's OWN value} and
    /// `removed` × {type id}. The component type itself rides
    /// `DataComponentType.STREAM_CODEC` = `StreamCodec.recursive(..)` =
    /// `ByteBufCodecs.registry(DATA_COMPONENT_TYPE)` — the SAME $29
    /// VarInt byIdOrThrow, and the value codec is the type's own
    /// `streamCodec()`: UNFRAMED (no length prefix), so a value's byte
    /// length is only knowable through the type's codec. The decodable
    /// slice is [`data_component_types`]; every other type refuses —
    /// an unknown id exactly where vanilla's byIdOrThrow throws, a
    /// known-but-unimplemented one because there is no honest skip.
    pub const ITEM_STACK: i32 = 7;
    /// `ByteBufCodecs.BOOL` — one byte.
    pub const BOOLEAN: i32 = 8;
    /// `Rotations.STREAM_CODEC` — three big-endian f32s.
    pub const ROTATIONS: i32 = 9;
    /// `BlockPos.STREAM_CODEC` — the packed position long (the block
    /// update's own 26/12/26-bit layout).
    pub const BLOCK_POS: i32 = 10;
    /// `BlockPos.STREAM_CODEC` through `ByteBufCodecs.optional` — a
    /// boolean flag, then the long only if present.
    pub const OPTIONAL_BLOCK_POS: i32 = 11;
    /// `Direction.STREAM_CODEC` = `ByteBufCodecs.idMapper(BY_ID,
    /// Direction::ordinal)` where `BY_ID` = `ByIdMap.continuous(..,
    /// OutOfBoundsStrategy.WRAP)`: a VarInt registry id (0 = down ..
    /// 5 = east) that NEVER refuses — out-of-range keys floorMod
    /// into [0, 6) (a negative -1 wraps to 5 = up). The wire ordinal
    /// is retained verbatim; the wrap is the consumer's resolution.
    pub const DIRECTION: i32 = 12;
    /// `EntityReference.streamCodec()` optional — the javap walk
    /// pins it THREE hops deep: `STREAM_CODEC` = `UUIDUtil
    /// .STREAM_CODEC.map(wrap-left, unpack-uuid)` — the wire carries
    /// ONLY the referenced entity's UUID (the Either's RIGHT, a
    /// resolved entity, is never serialized; the decode wraps the
    /// UUID in Either.left and the encode unpacks via getUUID);
    /// `UUIDUtil$1` = `FriendlyByteBuf.readUUID` = `new UUID(
    /// readLong(), readLong())` — 16 bytes, msb long then lsb long,
    /// both big-endian; and the serializer wraps the whole codec in
    /// the ONE-FLAG-BYTE optional operation (the same invokedynamic
    /// bsm BlockPos.optional and GlobalPos.optional ride). The
    /// resolution (which living entity the UUID points at) is the
    /// client's act — the port retains the UUID verbatim.
    pub const OPTIONAL_LIVING_ENTITY_REFERENCE: i32 = 13;
    /// `ByteBufCodecs.idMapper(BLOCK_STATE_REGISTRY)` — a VarInt in
    /// the chunk decoder's own state-id space.
    pub const BLOCK_STATE: i32 = 14;
    /// The optional companion of BLOCK_STATE — a flag, then the
    /// VarInt state id.
    pub const OPTIONAL_BLOCK_STATE: i32 = 15;
    /// `ParticleTypes.STREAM_CODEC` — the object-graph lane.
    pub const PARTICLE: i32 = 16;
    /// The list operation over PARTICLE.
    pub const PARTICLES: i32 = 17;
    /// `VillagerData.STREAM_CODEC` — REGISTRATION id 18, NOT the
    /// wiki-era 19 (the clinit order is the wire truth). The javap
    /// walk pins the composite: `holderRegistry(VILLAGER_TYPE)` +
    /// `holderRegistry(VILLAGER_PROFESSION)` + `ByteBufCodecs.VAR_INT`
    /// — THREE VarInt registry ids, and `ByteBufCodecs$29.decode` is
    /// `VarInt.read` -> `IdMap.byIdOrThrow` (a NUMERIC registry id,
    /// not a name; an id the registry does not know throws, failing
    /// the packet read).
    pub const VILLAGER_DATA: i32 = 18;
    /// The hand-rolled `OptionalInt` codec (the anonymous
    /// `EntityDataSerializers$3`): a VarInt where 0 = empty and a
    /// present value arrives BIAS-BY-ONE — wire n means n - 1 (the
    /// decode is `OptionalInt.of(n - 1)`, the encode
    /// `optional.orElse(-1) + 1`). The ARITHMETIC never refuses: a
    /// negative wire n is `OptionalInt.of(n - 1)` — a NEGATIVE
    /// stored value — and the i32 wrap (wire MIN_VALUE stores
    /// MAX_VALUE) is the isub/iadd's own two's-complement truth.
    /// The field NAME says unsigned; the codec's TYPE is the signed
    /// `OptionalInt` — the name-vs-type divergence the port's store
    /// follows (the i32 truth, not the wiki name).
    pub const OPTIONAL_UNSIGNED_INT: i32 = 19;
    /// `Pose.STREAM_CODEC` — `idMapper` over the enum's own id field:
    /// a VarInt. The map is `ByIdMap.continuous(..,
    /// OutOfBoundsStrategy.ZERO)` — NEVER refuses (out-of-range keys
    /// fill to values[0] = STANDING in the consumer's resolution).
    pub const POSE: i32 = 20;
    /// `CatVariant.STREAM_CODEC` = `ByteBufCodecs.holderRegistry
    /// (Registries.CAT_VARIANT)` — the FIRST of the twelve variant
    /// holders (ids 21-32), all the same lane: a VarInt registry id
    /// (the anonymous `ByteBufCodecs$29` = `VarInt.read` ->
    /// `IdMap.byIdOrThrow`, the villager-lane family). byIdOrThrow
    /// throws on a negative AND an unknown id; the port refuses the
    /// negative half (arithmetic-only) and retains the non-negative
    /// unknown verbatim (no variant-registry tables client-side; the
    /// consumer resolves or refuses). REGISTRATION ids 21-32 — the
    /// loop's holder order DIVERGES from the field-init order (the
    /// autopush-115 lesson, again): the loop runs CAT, CAT_SOUND,
    /// COW, COW_SOUND, WOLF, WOLF_SOUND, FROG, PIG, PIG_SOUND,
    /// CHICKEN, CHICKEN_SOUND, ZOMBIE_NAUTILUS, where the field-init
    /// block had the CHICKEN pair third and the COW pair fifth —
    /// only the registration loop is the wire.
    pub const CAT_VARIANT: i32 = 21;
    /// `CatSoundVariant.STREAM_CODEC` = `holderRegistry
    /// (Registries.CAT_SOUND_VARIANT)` — the family lane.
    pub const CAT_SOUND_VARIANT: i32 = 22;
    /// `CowVariant.STREAM_CODEC` = `holderRegistry(Registries
    /// .COW_VARIANT)` — wire 23, where a field-order pin would have
    /// shipped CHICKEN_VARIANT (the loop-vs-init swap).
    pub const COW_VARIANT: i32 = 23;
    /// `CowSoundVariant.STREAM_CODEC` = `holderRegistry(Registries
    /// .COW_SOUND_VARIANT)` — the family lane.
    pub const COW_SOUND_VARIANT: i32 = 24;
    /// `WolfVariant.STREAM_CODEC` = `holderRegistry(Registries
    /// .WOLF_VARIANT)` — the family lane.
    pub const WOLF_VARIANT: i32 = 25;
    /// `WolfSoundVariant.STREAM_CODEC` = `holderRegistry(Registries
    /// .WOLF_SOUND_VARIANT)` — the family lane.
    pub const WOLF_SOUND_VARIANT: i32 = 26;
    /// `FrogVariant.STREAM_CODEC` = `holderRegistry(Registries
    /// .FROG_VARIANT)` — the family lane.
    pub const FROG_VARIANT: i32 = 27;
    /// `PigVariant.STREAM_CODEC` = `holderRegistry(Registries
    /// .PIG_VARIANT)` — the family lane.
    pub const PIG_VARIANT: i32 = 28;
    /// `PigSoundVariant.STREAM_CODEC` = `holderRegistry(Registries
    /// .PIG_SOUND_VARIANT)` — the family lane.
    pub const PIG_SOUND_VARIANT: i32 = 29;
    /// `ChickenVariant.STREAM_CODEC` = `holderRegistry(Registries
    /// .CHICKEN_VARIANT)` — wire 30, TENTH of the holders where the
    /// field-init block declared it third (the loop-vs-init swap).
    pub const CHICKEN_VARIANT: i32 = 30;
    /// `ChickenSoundVariant.STREAM_CODEC` = `holderRegistry
    /// (Registries.CHICKEN_SOUND_VARIANT)` — the family lane.
    pub const CHICKEN_SOUND_VARIANT: i32 = 31;
    /// `ZombieNautilusVariant.STREAM_CODEC` = `holderRegistry
    /// (Registries.ZOMBIE_NAUTILUS_VARIANT)` — the last holder, the
    /// loop and the field-init agreeing here.
    pub const ZOMBIE_NAUTILUS_VARIANT: i32 = 32;
    /// `GlobalPos.STREAM_CODEC` under the ONE-FLAG-BYTE optional
    /// operation (the same invokedynamic bsm #149 BlockPos.optional
    /// and EntityReference.optional ride — the family's fourth
    /// member). The composite is TWO fields: the dimension key rides
    /// `ResourceKey.streamCodec(Registries.DIMENSION)` =
    /// `Identifier.STREAM_CODEC.map(create, toString)` — the wire is
    /// the plain identifier STRING ("minecraft:overworld",
    /// STRING_UTF8's own 32767 cap), NOT a registry id — and the
    /// position is `BlockPos.STREAM_CODEC` (the packed i64 the chunk
    /// path already decodes). A name-shaped trap: the villager ids
    /// ride VarInts a few serializers earlier while this dimension
    /// rides a string.
    pub const OPTIONAL_GLOBAL_POS: i32 = 33;
    /// `PaintingVariant.STREAM_CODEC` = `ByteBufCodecs.holder
    /// (Registries.PAINTING_VARIANT, DIRECT_STREAM_CODEC)` — the
    /// anonymous `ByteBufCodecs$30`, the ONLY bias-by-one member of
    /// the holder family: `n = VarInt.read`; `n == 0` -> the INLINE
    /// record payload (the direct holder), `n >= 1` ->
    /// `byIdOrThrow(n - 1)` (the registry reference, wire n means
    /// registry id n - 1), `n < 0` -> byIdOrThrow throws (the port
    /// refuses). The direct payload is the 5-field composite: width
    /// (VarInt), height (VarInt), assetId (the identifier string),
    /// title and author (each `ComponentSerialization
    /// .TRUSTED_OPTIONAL_STREAM_CODEC` — the flag byte, then the bare
    /// NBT tag only if present). Twelve sibling serializers ride the
    /// bias-zero `holderRegistry`; the painting rides `holder` — the
    /// family resemblance that would have flattened the 0-sentinel.
    pub const PAINTING_VARIANT: i32 = 34;
    /// `Sniffer$State.STREAM_CODEC` = `ByteBufCodecs.idMapper(BY_ID,
    /// ordinal)` with `BY_ID = ByIdMap.continuous(.., ZERO)` — a
    /// VarInt ordinal that NEVER refuses (out-of-range keys fill to
    /// values[0] = IDLING in the consumer's resolution). Seven
    /// states in declaration order: IDLING, FEELING_HAPPY, SCENTING,
    /// SNIFFING, SEARCHING, DIGGING, RISING. Wire 35 — BEFORE the
    /// armadillo, where the field-init order had the armadillo first
    /// (the loop-vs-init swap, the third family of this round).
    pub const SNIFFER_STATE: i32 = 35;
    /// `Armadillo$ArmadilloState.STREAM_CODEC` — the idMapper ZERO
    /// lane; four states: IDLE, ROLLING, SCARED, UNROLLING.
    pub const ARMADILLO_STATE: i32 = 36;
    /// `CopperGolemState.STREAM_CODEC` — the idMapper ZERO lane; a
    /// TOP-LEVEL class (not an inner enum); five states: IDLE,
    /// GETTING_ITEM, GETTING_NO_ITEM, DROPPING_ITEM,
    /// DROPPING_NO_ITEM.
    pub const COPPER_GOLEM_STATE: i32 = 37;
    /// `WeatheringCopper$WeatherState.STREAM_CODEC` — the idMapper
    /// lane with `OutOfBoundsStrategy.CLAMP`, the family's only
    /// non-ZERO member: the decode map CLAMPS out-of-range keys into
    /// [0, 3] (a wire 9 decodes to OXIDIZED in vanilla). The port
    /// retains the wire ordinal verbatim and the clamp is the
    /// consumer's act (the DIRECTION/POSE retention line); never
    /// refuses. Four states: UNAFFECTED, EXPOSED, WEATHERED,
    /// OXIDIZED.
    pub const WEATHERING_COPPER_STATE: i32 = 38;
    /// `ByteBufCodecs.VECTOR3F` — the anonymous `ByteBufCodecs$20`
    /// = `FriendlyByteBuf.readVector3f` = THREE big-endian f32s (x,
    /// y, z), 12 bytes — the same shape as ROTATIONS with the JOML
    /// vector's own field order.
    pub const VECTOR3: i32 = 39;
    /// `ByteBufCodecs.QUATERNIONF` — the anonymous `ByteBufCodecs
    /// $21` = `FriendlyByteBuf.readQuaternion` = FOUR big-endian
    /// f32s, 16 bytes, the JOML `Quaternionf(float, float, float,
    /// float)` order x, y, z, w.
    pub const QUATERNION: i32 = 40;
    /// The resolvable profile's audited wire (the audit's six hops):
    /// `ResolvableProfile.STREAM_CODEC` = `composite(either(ByteBuf
    /// Codecs.GAME_PROFILE, ResolvableProfile$Partial.STREAM_CODEC),
    /// PlayerSkin$Patch.STREAM_CODEC)` — the either is the ONE-BYTE
    /// flag (true = LEFT = the FULL GameProfile: uuid + name +
    /// properties; false = RIGHT = the Partial: optional name +
    /// optional uuid + properties); the properties map (the anonymous
    /// ByteBufCodecs$32) is `readCount(buf, 16)` — ONLY the upper
    /// bound throws (a NEGATIVE count passes and the builder loop
    /// iterates zero times: an EMPTY map, the never-refuse family) —
    /// then count × {name ≤64, value ≤32767, nullable(signature
    /// ≤1024)}; the patch is FOUR one-byte-flag optionals (body,
    /// cape, elytra: each a ClientAsset$ResourceTexture = two
    /// ≤32767-byte identifier strings — STRING_UTF8's own cap;
    /// model: a BOOL — true = SLIM, false = WIDE, the enum's own
    /// lambda pair). PLAYER_NAME is `stringUtf8(16)`.
    pub const RESOLVABLE_PROFILE: i32 = 41;
    /// `HumanoidArm.STREAM_CODEC` — the idMapper ZERO lane; two
    /// states in declaration order: LEFT = 0, RIGHT = 1 (the
    /// ordinal's own truth; no explicit id field exists on the enum
    /// — the declaration order IS the wire). NEVER refuses
    /// (out-of-range fills to LEFT in the consumer's resolution).
    pub const HUMANOID_ARM: i32 = 42;
}

/// The PARTICLE_TYPE registry's wire constants — the clinit's 125
/// `register` calls ARE the registry (angry_villager 0 ..
/// sulfur_cube_goo 124, the registration order = the id). The 22
/// payload carriers are named here (each pinned by its four-arg
/// register's codecFactory bootstrap handle); every other id in
/// [0, PARTICLE_TYPE_COUNT) is a zero-payload SimpleParticleType.
pub mod particle_types {
    /// The registry's size — byIdOrThrow refuses anything outside
    /// [0, 125).
    pub const PARTICLE_TYPE_COUNT: i32 = 125;
    pub const BLOCK: i32 = 1;
    pub const BLOCK_MARKER: i32 = 2;
    pub const GEYSER: i32 = 7;
    pub const GEYSER_BASE: i32 = 8;
    pub const GEYSER_POOF: i32 = 9;
    pub const GEYSER_PLUME: i32 = 10;
    pub const DRAGON_BREATH: i32 = 15;
    pub const DUST: i32 = 21;
    pub const DUST_COLOR_TRANSITION: i32 = 22;
    pub const EFFECT: i32 = 23;
    pub const ENTITY_EFFECT: i32 = 28;
    pub const FALLING_DUST: i32 = 36;
    pub const TINTED_LEAVES: i32 = 43;
    pub const SCULK_CHARGE: i32 = 45;
    pub const FLASH: i32 = 49;
    pub const INSTANT_EFFECT: i32 = 53;
    pub const ITEM: i32 = 54;
    pub const VIBRATION: i32 = 55;
    pub const TRAIL: i32 = 56;
    pub const SHRIEK: i32 = 112;
    pub const DUST_PILLAR: i32 = 118;
    pub const BLOCK_CRUMBLE: i32 = 122;
}

/// The DATA_COMPONENT_TYPE registry slice whose value codecs this
/// client decodes — the ids are `DataComponents.<clinit>`'s
/// REGISTRATION order (`Registry.register` call sequence; the registry
/// id IS the order, and the order is NOT semantically grouped: the
/// four numeric durability lanes sit at 1-4, the unit types at 4 and
/// 20, the component-tag lanes at 6/9, the identifier lanes scattered
/// at 10/35/71). This is the THIRD registry this project has pinned
/// by clinit order against the wiki-era instinct. The value codecs
/// (each type's own `streamCodec()`, javap of the 26.2 builder
/// lambdas): VarInt × 4, Unit (ZERO bytes) × 2, the bare component
/// tag × 2, the compound-only bare tag, the BE f32, the UTF string ×
/// 3, the capped tag list, the idMapper ordinal, and the bool — plus
/// the second wave's three composites (the enchantment map, the
/// four-list model data, the tooltip display) and the third wave's
/// adventure-mode pair (can_place_on / can_break — the SAME
/// AdventureModePredicate record). The other 90 types
/// refuse — see [`EntityDataEntry::ItemStack`].
pub mod data_component_types {
    /// `CustomData.STREAM_CODEC` = `ByteBufCodecs.COMPOUND_TAG` =
    /// `compoundTagCodec(..)` = the tagCodec bare tag + a decode-side
    /// map that THROWS unless the tag is a TAG_Compound (the
    /// DecoderException at lambda$compoundTagCodec$0).
    pub const CUSTOM_DATA: i32 = 0;
    /// `ByteBufCodecs.VAR_INT` (the persistent side is
    /// `intRange(1, 99)` — a NBT-schema concern, not the wire's).
    pub const MAX_STACK_SIZE: i32 = 1;
    /// `ByteBufCodecs.VAR_INT` (`POSITIVE_INT` on the persistent side).
    pub const MAX_DAMAGE: i32 = 2;
    /// `ByteBufCodecs.VAR_INT` (`NON_NEGATIVE_INT` persistent;
    /// `ignoreSwapAnimation` is a swap-render flag, wire-irrelevant).
    pub const DAMAGE: i32 = 3;
    /// `Unit.STREAM_CODEC` = `StreamCodec.unit(INSTANCE)` — encodes
    /// NOTHING, decodes NOTHING: the value's wire presence is the
    /// type id alone.
    pub const UNBREAKABLE: i32 = 4;
    /// `ComponentSerialization.STREAM_CODEC` — ONE bare network NBT
    /// tag (the serializer-5 machinery).
    pub const CUSTOM_NAME: i32 = 6;
    /// `ByteBufCodecs.FLOAT` — a big-endian f32.
    pub const MINIMUM_ATTACK_CHARGE: i32 = 7;
    /// `ComponentSerialization.STREAM_CODEC` — ONE bare tag.
    pub const ITEM_NAME: i32 = 9;
    /// `Identifier.STREAM_CODEC` = `ByteBufCodecs.STRING_UTF8` mapped
    /// — the VarInt-length UTF string (cap 32767).
    pub const ITEM_MODEL: i32 = 10;
    /// `ComponentSerialization.STREAM_CODEC` through
    /// `ByteBufCodecs.list(256)` — a VarInt count (capped: a larger
    /// list throws) of bare component tags.
    pub const LORE: i32 = 11;
    /// `Rarity.STREAM_CODEC` = `ByteBufCodecs.idMapper(BY_ID,
    /// Rarity::ordinal)` where `BY_ID` = `ByIdMap.continuous(..,
    /// OutOfBoundsStrategy.ZERO)` — a VarInt ordinal, NEVER refused:
    /// every key outside [0, 4) fills to values[0] = COMMON (the
    /// DIRECTION/POSE idMapper family; the wire ordinal is retained
    /// verbatim, the ZERO-fill is the consumer's resolution).
    pub const RARITY: i32 = 12;
    /// `ByteBufCodecs.VAR_INT` (`NON_NEGATIVE_INT` persistent).
    pub const REPAIR_COST: i32 = 19;
    /// `Unit.STREAM_CODEC` — the second zero-byte lane.
    pub const CREATIVE_SLOT_LOCK: i32 = 20;
    /// `ByteBufCodecs.BOOL` — one byte.
    pub const ENCHANTMENT_GLINT_OVERRIDE: i32 = 21;
    /// `Identifier.STREAM_CODEC` — the UTF string.
    pub const TOOLTIP_STYLE: i32 = 35;
    /// `Identifier.STREAM_CODEC` — the UTF string.
    pub const NOTE_BLOCK_SOUND: i32 = 71;
    // ---- the second wave: the record codecs ----
    /// `ItemEnchantments.STREAM_CODEC` = `composite(ByteBufCodecs.map(
    /// factory, Enchantment.STREAM_CODEC, VAR_INT), ..)` — ONE field:
    /// a VarInt count, then count × {the enchantment's registry id,
    /// the level}. The key id is the $29 byIdOrThrow holderRegistry
    /// VarInt (no enchantment tables client-side; a consumer resolves).
    /// The LEVEL rides the wire as a PLAIN VAR_INT — the
    /// `intRange(1, 255)` LEVEL_CODEC is the NBT-schema side, not the
    /// wire's (the wiki-era instinct to clamp at decode invents a
    /// data flow vanilla does not have).
    pub const ENCHANTMENTS: i32 = 13;
    /// `CustomModelData.STREAM_CODEC` = `composite(FLOAT.list(),
    /// BOOL.list(), STRING_UTF8.list(), INT.list(), ..)` — FOUR
    /// VarInt-count lists in composite order: floats (BE f32), flags
    /// (one-byte bools), strings (UTF), colors (`ByteBufCodecs.INT` =
    /// `readInt` = a BIG-ENDIAN i32 — NOT a VarInt; the same trap
    /// family as the LONG/VAR_LONG discovery: the codec's construction
    /// site is the shape, the field's name is not).
    pub const CUSTOM_MODEL_DATA: i32 = 17;
    /// `TooltipDisplay.STREAM_CODEC` = `composite(BOOL,
    /// DataComponentType.STREAM_CODEC.collection(factory), ..)` —
    /// one hide-tooltip byte, then a VarInt count of component type
    /// registry ids (the SAME $29 VarInt the patch entries ride; the
    /// ids are SELF-DELIMITING here — one VarInt each, no value
    /// attached — so any non-negative id is retained verbatim, the
    /// DIRECTION/RARITY line; the consumer resolves or refuses).
    pub const TOOLTIP_DISPLAY: i32 = 18;
    // ---- the third wave: the adventure-mode pair ----
    /// `AdventureModePredicate.STREAM_CODEC` =
    /// `composite(BlockPredicate.STREAM_CODEC.list(), ..)` — a VarInt
    /// count (the NO-ARG `ByteBufCodecs.list()`, whose ceiling is
    /// `Integer.MAX_VALUE` — readCount's ceiling check is the only
    /// guard, a negative count still dies at the constructor) then
    /// count × [`BlockPredicate`]. Both this and
    /// [`data_component_types::CAN_BREAK`] carry the SAME record — the
    /// two ids are one codec, registered twice (clinit order 14 and
    /// 15, BETWEEN enchantments 13 and attribute_modifiers 16).
    pub const CAN_PLACE_ON: i32 = 14;
    /// The second half of the adventure-mode pair — the same
    /// `AdventureModePredicate` record as
    /// [`data_component_types::CAN_PLACE_ON`] (one codec, two
    /// component ids: "blocks this item CAN place on" and "blocks it
    /// CAN break" share the predicate list shape).
    pub const CAN_BREAK: i32 = 15;
    // ---- the fourth wave: the stat + sustenance trio ----
    /// `ItemAttributeModifiers.STREAM_CODEC` =
    /// `composite(Entry.STREAM_CODEC.list(), ..)` — a VarInt count
    /// (the NO-ARG `ByteBufCodecs.list()`, ceiling
    /// `Integer.MAX_VALUE`) then count × FOUR-lane entries:
    /// `holderRegistry(ATTRIBUTE)` (the `$29` byIdOrThrow VarInt),
    /// `AttributeModifier.STREAM_CODEC` (composite of an Identifier
    /// UTF id, `ByteBufCodecs.DOUBLE` = a BIG-ENDIAN f64, and the
    /// Operation idMapper ordinal), `EquipmentSlotGroup.STREAM_CODEC`
    /// (the same idMapper shape — ANY=0 .. SADDLE=10) and the
    /// 26.2-NEW `Display.STREAM_CODEC` — a DISPATCH: the Type
    /// ordinal (DEFAULT=0, HIDDEN=1, OVERRIDE=2) then the variant's
    /// payload (default/hidden `StreamCodec.unit` = ZERO bytes,
    /// override one bare component tag). See
    /// [`crate::play::AttributeModifierEntry`].
    pub const ATTRIBUTE_MODIFIERS: i32 = 16;
    /// `FoodProperties.DIRECT_STREAM_CODEC` = `composite(VAR_INT,
    /// FLOAT, BOOL)` — nutrition is a VAR_INT in 26.2 (the record
    /// holds `int nutrition`; the wiki-era float instinct is not the
    /// wire's shape — the construction site decides), saturation a
    /// BE f32, can_always_eat one byte.
    pub const FOOD: i32 = 23;
    /// `Consumable.STREAM_CODEC` — a FIVE-lane composite: FLOAT
    /// consume_seconds, `ItemUseAnimation.STREAM_CODEC` (an idMapper
    /// ordinal — NONE=0 .. BRUSH=9), `SoundEvent.STREAM_CODEC` =
    /// `ByteBufCodecs.holder(..)` (a TAGGED union: wire 0 = the
    /// direct `{UTF id, f32 range}` lane, wire tag >= 1 = the
    /// `(tag - 1)` registry id, a negative tag refuses), BOOL
    /// has_consume_particles, then `ConsumeEffect.STREAM_CODEC.list()`
    /// — a count of DISPATCHED registry entries: the registry id (a
    /// `$29` byIdOrThrow VarInt — an unknown id refuses) then that
    /// type's own payload ([`ConsumeEffect`]; the five known types
    /// are apply_effects 0, remove_effects 1, clear_all_effects 2,
    /// teleport_randomly 3, play_sound 4 — the registry's clinit
    /// names carry no "status" infix; the wiki-era naming is not
    /// the wire's).
    pub const CONSUMABLE: i32 = 24;
    /// The client-side DATA_COMPONENT_TYPE registry's SIZE — vanilla's
    /// clinit census (111 registered types; TWENTY-FOUR carry value
    /// codecs, the other 87 refuse). The `byIdOrThrow` bound of every
    /// bare component-type id lane (the any-value predicate lane).
    pub const REGISTRY_SIZE: i32 = 111;
}

/// The `DataComponentPredicates` registry — the DISPATCHED
/// predicate-type table behind the component-matcher lane's PARTIAL
/// side. The ids are the clinit `register` order (`Registry.register`
/// call sequence = the registry id; the FIFTH registry this project
/// has pinned by clinit order). The NAMES are the bytecode's `ldc`
/// strings verbatim (the registry names are the wire's names — the
/// wiki-era naming is not, the consume-effect lesson again).
///
/// The payload's transport: `DataComponentPredicate$TypeBase`
/// builds every type's stream side as `ByteBufCodecs
/// .fromCodecWithRegistries(record.CODEC).map(wrap, unwrap)` — the
/// `tagCodec` bridge — so each dispatched entry's payload rides as
/// ONE full NBT tag (`NbtIo.readAnyTag`: the type byte + payload, no
/// name framing), parsed through the record's OWN NBT-side codec
/// with NbtOps. The port retains that tag verbatim (the
/// ComponentSerialization line): the per-record parse is a
/// consumer's act, and a shape the record codec would refuse arrives
/// as the retained tag, not a read-time refusal. A 0x00 root byte
/// refuses: `readNbt` maps the End tag to null and the bridge throws
/// "Expected non-null compound tag" (the port's bare-tag reader
/// errors on the same byte, the same lane, panic-free).
pub mod data_component_predicate_types {
    /// `DamagePredicate.CODEC` — the damage/durable-range record.
    pub const DAMAGE: i32 = 0;
    /// `EnchantmentsPredicate$Enchantments.CODEC`.
    pub const ENCHANTMENTS: i32 = 1;
    /// `EnchantmentsPredicate$StoredEnchantments.CODEC`.
    pub const STORED_ENCHANTMENTS: i32 = 2;
    /// `PotionsPredicate.CODEC` — the registry name is
    /// `potion_contents` (the `ldc` string verbatim).
    pub const POTION_CONTENTS: i32 = 3;
    /// `CustomDataPredicate.CODEC`.
    pub const CUSTOM_DATA: i32 = 4;
    /// `ContainerPredicate.CODEC`.
    pub const CONTAINER: i32 = 5;
    /// `BundlePredicate.CODEC` — the registry name is
    /// `bundle_contents`.
    pub const BUNDLE_CONTENTS: i32 = 6;
    /// `FireworkExplosionPredicate.CODEC`.
    pub const FIREWORK_EXPLOSION: i32 = 7;
    /// `FireworksPredicate.CODEC`.
    pub const FIREWORKS: i32 = 8;
    /// `WritableBookPredicate.CODEC` — the registry name is
    /// `writable_book_content`.
    pub const WRITABLE_BOOK_CONTENT: i32 = 9;
    /// `WrittenBookPredicate.CODEC` — the registry name is
    /// `written_book_content`.
    pub const WRITTEN_BOOK_CONTENT: i32 = 10;
    /// `AttributeModifiersPredicate.CODEC`.
    pub const ATTRIBUTE_MODIFIERS: i32 = 11;
    /// `TrimPredicate.CODEC` — the registry name is `trim`.
    pub const ARMOR_TRIM: i32 = 12;
    /// `JukeboxPlayablePredicate.CODEC`.
    pub const JUKEBOX_PLAYABLE: i32 = 13;
    /// `VillagerTypePredicate.CODEC` — the registry name carries a
    /// SLASH (`villager/variant`, the `ldc` string verbatim).
    pub const VILLAGER_VARIANT: i32 = 14;
    /// The built-in registry's SIZE — the `byIdOrThrow` bound of the
    /// dispatched type id (an unknown AND negative id throw; the
    /// consume-effect dispatch line).
    pub const REGISTRY_SIZE: i32 = 15;
}

/// One partial matcher's dispatched TYPE lane —
/// `DataComponentPredicate$Type.STREAM_CODEC` =
/// `ByteBufCodecs.either(registry(DATA_COMPONENT_PREDICATE_TYPE),
/// registry(DATA_COMPONENT_TYPE))` mapped to the Type. The either is
/// `$26` — a ONE-BYTE readBoolean, true = LEFT — then a `$29`
/// byIdOrThrow VarInt into the chosen branch's registry. BOTH
/// registries are real lookups (NOT the idMapper-continuous family):
/// a negative AND an out-of-range id throw, the port refuses at the
/// same lane.
#[derive(Debug, Clone, PartialEq)]
pub enum PredicateTypeLane {
    /// LEFT (bool 1): a `DataComponentPredicates` registry id — the
    /// built-in 15-row table (see
    /// [`data_component_predicate_types`]).
    PredicateType(i32),
    /// RIGHT (bool 0): a DATA_COMPONENT_TYPE registry id — the
    /// `AnyValueType` shape ("the component is present, any value");
    /// its payload needs no value codec, the tag is retained
    /// verbatim.
    ComponentType(i32),
}

/// ONE partial matcher entry — the `DataComponentPredicate$Single`
/// record as the wire carries it: the dispatched type lane
/// ([`PredicateTypeLane`]) plus the ONE full NBT tag its record
/// codec bridges (`fromCodecWithRegistries` = `tagCodec` + the
/// record's own parse). The tag is retained verbatim.
#[derive(Debug, Clone, PartialEq)]
pub struct PartialPredicate {
    /// The dispatched type lane (the either branch + registry id).
    pub predicate_type: PredicateTypeLane,
    /// The payload tag (the record codec's NBT-side schema is a
    /// consumer's act).
    pub payload: Nbt,
}

/// One decoded component-patch VALUE — the codec-shape variants of
/// [`data_component_types`]. The variants share by WIRE SHAPE, not by
/// meaning: the component type id rides beside the value in
/// [`ComponentPatch::added`], so `VarInt` honestly covers four
/// durability lanes and `String` three identifier lanes without
/// pretending the port knows what they mean. The meaning is a
/// consumer's act, the same decode-only line the villager ids rode.
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentValue {
    /// The four VAR_INT lanes: max_stack_size (1), max_damage (2),
    /// damage (3), repair_cost (19).
    VarInt(i32),
    /// The BE f32 lane: minimum_attack_charge (7).
    Float(f32),
    /// The zero-byte Unit lanes: unbreakable (4),
    /// creative_slot_lock (20) — the value IS the type's presence.
    Unit,
    /// The bare component tag lane: custom_name (6), item_name (9) —
    /// any tag shape, retained verbatim (the rich-text schema stays a
    /// rendering-time concern, the COMPONENT serializer's line).
    Component(Nbt),
    /// The compound-only bare tag lane: custom_data (0) — a non-compound
    /// tag refuses (vanilla's DecoderException).
    CompoundTag(Nbt),
    /// The UTF string lane: item_model (10), tooltip_style (35),
    /// note_block_sound (71).
    String(String),
    /// The capped list lane: lore (11) — a VarInt count (<= 256) of
    /// bare component tags.
    Lore(Vec<Nbt>),
    /// The idMapper ordinal lane: rarity (12), retained verbatim —
    /// vanilla's continuous ZERO map never refuses (out-of-range
    /// fills to COMMON in the consumer's resolution).
    Rarity(i32),
    /// The one-byte bool lane: enchantment_glint_override (21).
    Bool(bool),
    /// The map lane: enchantments (13) — (enchantment registry id,
    /// level) pairs in wire order (a vanilla Map.put would collapse
    /// duplicate keys last-wins; the decode retains the wire's order
    /// verbatim, the dedup is a consumer's concern).
    Enchantments(Vec<(i32, i32)>),
    /// The four-list composite lane: custom_model_data (17) — floats
    /// (BE f32), flags (one-byte bools), strings (UTF), colors (BE
    /// i32), each a VarInt count followed by its items, in composite
    /// order.
    CustomModelData {
        /// The float modifiers (BE f32 each).
        floats: Vec<f32>,
        /// The boolean flags (one byte each).
        flags: Vec<bool>,
        /// The string keys (UTF each).
        strings: Vec<String>,
        /// The color integers (BE i32 each — `ByteBufCodecs.INT`, not
        /// a VarInt).
        colors: Vec<i32>,
    },
    /// The display lane: tooltip_display (18) — the hide-tooltip byte
    /// and the hidden components' type registry ids in wire order (a
    /// vanilla Set would collapse duplicates; the decode retains the
    /// wire's order verbatim).
    TooltipDisplay {
        /// The hide-everything byte.
        hide_tooltip: bool,
        /// The hidden components' type registry ids, in wire order.
        hidden: Vec<i32>,
    },
    /// The adventure-mode lane: can_place_on (14) and can_break (15)
    /// — the SAME `AdventureModePredicate` record registered under
    /// two component ids, so one variant serves both. A predicate is
    /// the four-lane [`BlockPredicate`] composite; see it for the
    /// decode's honest refusals.
    BlockPredicates(Vec<BlockPredicate>),
    /// The stat lane: attribute_modifiers (16) — the held-item's
    /// attribute surface, the FOUR-lane entry composite (see
    /// [`AttributeModifierEntry`]). The registry and ordinal lanes
    /// are retained verbatim; the Display ordinal is FRAMING and
    /// resolves in-decode.
    AttributeModifiers(Vec<AttributeModifierEntry>),
    /// The sustenance lane: food (23) — `composite(VAR_INT nutrition,
    /// FLOAT saturation, BOOL can_always_eat)`. A negative nutrition
    /// is retained verbatim: the record constructor does not clamp
    /// (the villager round's lesson — a clamp's direction is a
    /// per-codec fact, and the javap shows none here).
    Food {
        /// The nutrition — a plain VarInt on the wire.
        nutrition: i32,
        /// The saturation modifier — a BE f32.
        saturation: f32,
        /// The always-edible flag — one byte.
        can_always_eat: bool,
    },
    /// The consumption lane: consumable (24) — the FIVE-lane
    /// composite (consume seconds, the animation ordinal, the tagged
    /// sound, the particle flag) with the effects list now WALKED:
    /// `ConsumeEffect.STREAM_CODEC` = `ByteBufCodecs.registry(
    /// CONSUME_EFFECT_TYPE)` dispatched (see [`ConsumeEffect`] — the
    /// autopush-111 pinned-EMPTY deferral retired; each of the five
    /// known types carries its own audited payload).
    Consumable {
        /// The consume time — a BE f32.
        consume_seconds: f32,
        /// The use animation's idMapper ordinal (NONE=0 .. BRUSH=9),
        /// retained verbatim (the ZERO-fill is a consumer's act).
        animation: i32,
        /// The consume sound — the `ByteBufCodecs.holder` tagged
        /// union ([`ConsumableSound`]).
        sound: ConsumableSound,
        /// The consume-particles flag — one byte.
        has_consume_particles: bool,
        /// The consume effects — the DISPATCHED registry list, in
        /// wire order (an entry's type id selects its payload walk;
        /// an unknown id refuses — byIdOrThrow).
        effects: Vec<ConsumeEffect>,
    },
}

/// The item stack's component patch — the `DataComponentPatch$3`
/// decode verbatim: an ADDED list of (component type id, value) and a
/// REMOVED list of type ids (a removal carries NO value — the absence
/// IS the payload). Both lists empty is the EMPTY patch (the early
/// return when both wire counts are zero).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ComponentPatch {
    /// The added (overridden) components, in wire order.
    pub added: Vec<(i32, ComponentValue)>,
    /// The removed components' type ids, in wire order.
    pub removed: Vec<i32>,
}

/// The vibration's dispatched position source — the
/// POSITION_SOURCE_TYPE registry's two-type union (block 0, entity
/// 1, the clinit order; `PositionSource.STREAM_CODEC` =
/// `StreamCodec.dispatch(ByteBufCodecs.registry(POSITION_SOURCE_TYPE),
/// ..)` — the $29 byIdOrThrow family, an unknown AND a negative id
/// throw).
#[derive(Debug, Clone, PartialEq)]
pub enum PositionSource {
    /// BlockPositionSource — `composite(BlockPos.STREAM_CODEC)`: the
    /// packed block-pos i64, decoded the chunk path's way.
    Block([i32; 3]),
    /// EntityPositionSource — `composite(VAR_INT, FLOAT)`: the
    /// entity id and the y-offset above its feet.
    Entity { entity_id: i32, y_offset: f32 },
}

/// One decoded particle VALUE — `ParticleTypes.STREAM_CODEC`'s
/// dispatched walk: the VarInt registry id (the $29 byIdOrThrow
/// family), then the type's own payload codec. The 103 simple types
/// ride ZERO payload bytes (`StreamCodec.unit` — the
/// clear-all-effects lesson); the 22 carriers' shapes are the option
/// classes' composites, each pinned by its javap'd `streamCodec`.
#[derive(Debug, Clone, PartialEq)]
pub enum ParticleValue {
    /// SimpleParticleType — zero payload bytes (103 of the 125 ids).
    /// The dispatched registry id is RETAINED verbatim (the write
    /// side's demand: five of the carriers cover more than one
    /// registry id, and a parse that collapsed the id could not be
    /// written back — the id was in the dispatch's hand, the store
    /// keeps it).
    Simple(i32),
    /// BlockParticleOption (`idMapper(BLOCK_STATE_REGISTRY)` — the
    /// state-palette lookup, negative refuses): block,
    /// block_marker, falling_dust, dust_pillar, block_crumble — FIVE
    /// registry ids on one shape, so the id rides the store.
    Block { particle_id: i32, state: i32 },
    /// GeyserParticleOptions — `composite(INT)`: geyser (a 1:1 lane —
    /// the writer names the constant); the record's waterBlocks.
    Geyser { water_blocks: i32 },
    /// GeyserBaseParticleOptions — `composite(INT, FLOAT)`: geyser_
    /// base, geyser_poof — TWO ids on one shape, the id retained.
    GeyserBase {
        particle_id: i32,
        water_blocks: i32,
        burst_impulse_base: f32,
    },
    /// PowerParticleOption — `composite(FLOAT)`: dragon_breath (1:1);
    /// the power field.
    Power(f32),
    /// DustParticleOptions — `composite(INT, FLOAT)`: dust (1:1); the
    /// color argb + the scale.
    Dust { color: i32, scale: f32 },
    /// DustColorTransitionOptions — `composite(INT, INT, FLOAT)`:
    /// dust_color_transition (1:1); from-color + to-color + scale.
    DustTransition { from_color: i32, to_color: i32, scale: f32 },
    /// SpellParticleOption — `composite(INT, FLOAT)`: effect,
    /// instant_effect — TWO ids on one shape, the id retained; the
    /// color argb + the power.
    Spell {
        particle_id: i32,
        color: i32,
        power: f32,
    },
    /// ColorParticleOption — `composite(INT)`: entity_effect,
    /// tinted_leaves, flash — THREE ids on one shape, the id
    /// retained; the color argb.
    Color { particle_id: i32, color: i32 },
    /// ItemParticleOption — `map(ItemStackTemplate.STREAM_CODEC)` =
    /// `composite(Item.STREAM_CODEC, VAR_INT, DataComponentPatch
    /// .STREAM_CODEC)`: the item id, the count (a RAW lane — no
    /// count-is-emptiness sentinel here, the ItemStack$1 line does
    /// not apply), and the patch (the same shape the metadata
    /// item-stack lane decodes).
    Item { item_id: i32, count: i32, patch: ComponentPatch },
    /// VibrationParticleOption — `composite(PositionSource
    /// .STREAM_CODEC, VAR_INT)`: the dispatched source + the
    /// arrival-in-ticks.
    Vibration { source: PositionSource, arrival_in_ticks: i32 },
    /// TrailParticleOption — `composite(Vec3.STREAM_CODEC, INT,
    /// VAR_INT)`: the 24-byte target (three big-endian f64), the
    /// color argb, the duration.
    Trail { target: [f64; 3], color: i32, duration: i32 },
    /// ShriekParticleOption — `composite(VAR_INT)`: shriek; the
    /// delay.
    Shriek(i32),
    /// SculkChargeParticleOptions — `composite(FLOAT)`: sculk_charge;
    /// the roll.
    SculkCharge(f32),
}

/// One profile property — the PropertyMap's element: the name/value
/// pair and the optional signature.
#[derive(Debug, Clone, PartialEq)]
pub struct ProfileProperty {
    /// The property name (≤64 bytes on the wire).
    pub name: String,
    /// The property value (≤32767 bytes on the wire).
    pub value: String,
    /// The optional signature (≤1024 bytes; the one-byte nullable
    /// flag rides first).
    pub signature: Option<String>,
}

/// A ClientAsset$ResourceTexture — the skin patch's asset: two
/// plain-string identifiers (each ≤32767 bytes — STRING_UTF8's own
/// cap; the Identifier wire is the string mapped).
#[derive(Debug, Clone, PartialEq)]
pub struct SkinAsset {
    /// The asset's identifier string.
    pub id: String,
    /// The texture path's identifier string.
    pub texture_path: String,
}

/// The skin patch — the profile composite's second lane: FOUR
/// one-byte-flag optional lanes. The model's bool maps true = SLIM,
/// false = WIDE (the enum's own lambda pair) — the wire datum is
/// the bool, retained verbatim.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerSkinPatch {
    /// The body texture asset, if present.
    pub body: Option<SkinAsset>,
    /// The cape texture asset, if present.
    pub cape: Option<SkinAsset>,
    /// The elytra texture asset, if present.
    pub elytra: Option<SkinAsset>,
    /// Some(true) = SLIM, Some(false) = WIDE (vanilla's BOOL map),
    /// None = the flag said absent.
    pub model_slim: Option<bool>,
}

/// The profile's identity side — `either(GAME_PROFILE,
/// ResolvableProfile$Partial.STREAM_CODEC)`: the FULL GameProfile
/// or the Partial. The properties map's count is readCount(buf, 16)
/// — only the UPPER bound throws; a negative count is an EMPTY map
/// (the never-refuse family).
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvableProfile {
    /// The LEFT branch: the full GameProfile.
    Full {
        /// The profile's UUID (16 bytes, msb then lsb).
        uuid: [u8; 16],
        /// The profile name (≤16 bytes — PLAYER_NAME's stringUtf8
        /// cap).
        name: String,
        /// The properties.
        properties: Vec<ProfileProperty>,
    },
    /// The RIGHT branch: the Partial — name and id are each
    /// optional (each behind its own one-byte flag).
    Partial {
        /// The optional name.
        name: Option<String>,
        /// The optional UUID.
        id: Option<[u8; 16]>,
        /// The properties.
        properties: Vec<ProfileProperty>,
    },
}

/// A retained living-entity reference: the UUID the wire carries
/// (16 bytes, msb then lsb — [`entity_data_serializers`
/// ::OPTIONAL_LIVING_ENTITY_REFERENCE]'s audited shape). Vanilla's
/// `EntityReference` wraps it in `Either<UUID, StoredEntityType>`;
/// the wire never serializes the resolved right side, so the port
/// retains the UUID and the resolution is the consumer's act (the
/// retained-tag-is-a-store line).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityReference {
    /// The referenced entity's UUID, msb long in bytes 0..8, lsb
    /// long in bytes 8..16 (FriendlyByteBuf.readUUID's two
    /// readLongs, both big-endian) — retained verbatim.
    pub uuid: [u8; 16],
}

/// A decoded global position ([`entity_data_serializers`
/// ::OPTIONAL_GLOBAL_POS]'s audited composite): the dimension key's
/// identifier STRING (retained verbatim — "minecraft:overworld";
/// the wire is `Identifier.STREAM_CODEC` mapped through
/// `ResourceKey.streamCodec`, NOT a registry id) and the packed
/// block position (the chunk decoder's own arithmetic).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalPos {
    /// The dimension's identifier string, ≤32767 bytes
    /// (STRING_UTF8's own cap).
    pub dimension: String,
    /// The world x/y/z the packed i64 decodes to.
    pub pos: [i32; 3],
}

/// A decoded painting variant ([`entity_data_serializers`
/// ::PAINTING_VARIANT]'s audited `$30 holder` shape — the holder
/// family's ONLY bias-by-one member): either the registry
/// REFERENCE (wire n >= 1, stored as the registry id n - 1,
/// retained verbatim — the consumer resolves) or the INLINE DIRECT
/// record (wire n == 0, the 5-field composite the record's own
/// accessors name).
#[derive(Debug, Clone, PartialEq)]
pub enum PaintingVariant {
    /// The registry reference: wire n stored as n - 1 (the
    /// byIdOrThrow lane's id space, retained verbatim).
    Registry(i32),
    /// The direct holder's inline record.
    Direct {
        /// The painting's width (VarInt).
        width: i32,
        /// The painting's height (VarInt).
        height: i32,
        /// The asset's identifier string, ≤32767 bytes.
        asset_id: String,
        /// The title component (flag byte + bare NBT tag), absent
        /// when the flag byte is 0.
        title: Option<Nbt>,
        /// The author component (the same optional lane).
        author: Option<Nbt>,
    },
}

/// One decoded metadata VALUE — the wire shapes of the serializers
/// this client decodes (ALL FORTY-THREE of the registry's ids: 0-42,
/// as [`entity_data_serializers`] declares — the walk completed the
/// round the variant holders, the global pos, the painting holder,
/// the enum states, and the vector/quaternion lanes landed). The i32
/// flavors are DISTINCT variants because the serializer id is part of
/// the state (vanilla's `assignValues` matches accessor and value on
/// it), not a type system accident.
#[derive(Debug, Clone, PartialEq)]
pub enum EntityDataEntry {
    /// Serializer 0: one signed byte.
    Byte(i8),
    /// Serializer 1: a VarInt.
    Int(i32),
    /// Serializer 2: a VarLong.
    Long(i64),
    /// Serializer 3: a big-endian f32.
    Float(f32),
    /// Serializer 4: the VarInt-length UTF-8 string.
    String(String),
    /// Serializer 5: one bare network NBT tag — the chat component's
    /// wire shape (a TAG_String for plain text, a TAG_Compound for the
    /// structured `{text:..}` form; the tag is retained verbatim, the
    /// rich-text schema is a rendering-time concern).
    Component(Nbt),
    /// Serializer 6: the flag, then the bare tag only if present.
    OptionalComponent(Option<Nbt>),
    /// Serializer 8: one byte, non-zero = true.
    Bool(bool),
    /// Serializer 9: three big-endian f32s (x, y, z).
    Rotations([f32; 3]),
    /// Serializer 10: the packed position long, decoded to world
    /// x/y/z (the two-shift arithmetic sign-extends the negative
    /// half-plane).
    BlockPos([i32; 3]),
    /// Serializer 11: the boolean flag, then the packed long only if
    /// present.
    OptionalBlockPos(Option<[i32; 3]>),
    /// Serializer 12: the direction's registry id (0 = down .. 5 =
    /// east), retained verbatim — vanilla's continuous WRAP map never
    /// refuses (out-of-range floorMod(k, 6) is the consumer's
    /// resolution).
    Direction(i32),
    /// Serializer 13: the flag byte, then the referenced entity's
    /// UUID (16 bytes: msb then lsb, both big-endian) — the
    /// reference retained as its UUID; resolving it to a living
    /// entity is the client's act.
    OptionalLivingEntityReference(Option<EntityReference>),
    /// Serializer 14: the block state id (the chunk decoder's id
    /// space).
    BlockState(i32),
    /// Serializer 15: the flag, then the state id only if present.
    OptionalBlockState(Option<i32>),
    /// Serializer 16: the dispatched particle — the registry id then
    /// the type's own payload (see [`ParticleValue`]).
    Particle(ParticleValue),
    /// Serializer 17: the list operation over PARTICLE — the VarInt
    /// count, then that many dispatched particle values.
    Particles(Vec<ParticleValue>),
    /// Serializer 41: the resolvable profile — the either(full/
    /// partial) identity + the skin patch (see [`ResolvableProfile`],
    /// [`PlayerSkinPatch`]).
    ResolvableProfile {
        profile: ResolvableProfile,
        patch: PlayerSkinPatch,
    },
    /// Serializer 19: the OptionalInt — wire 0 = None, wire n =
    /// Some(n - 1), the SIGNED truth retained verbatim (a negative
    /// wire n stores n - 1; the decode never refuses — the serializer
    /// field NAME says unsigned, the codec's type is the signed
    /// `OptionalInt`).
    OptionalUnsignedInt(Option<i32>),
    /// Serializer 20: the pose's registry id, retained verbatim —
    /// vanilla's continuous ZERO map never refuses (out-of-range
    /// fills to STANDING in the consumer's resolution).
    Pose(i32),
    /// Serializer 18: the villager record — two numeric registry ids
    /// (the VILLAGER_TYPE and VILLAGER_PROFESSION registries' own id
    /// spaces) and the trade level. The ids are retained verbatim
    /// (this client carries no villager registry tables; a consumer
    /// resolves them); the level is the record constructor's own
    /// `Math.max(1, level)` — the javap shows `iconst_1` feeding the
    /// `Math.max`, so the DECODE path already clamps (composite decode
    /// calls `VillagerData::new` directly), and a wire 0 level is a
    /// stored 1 without any consumer-side arithmetic.
    VillagerData {
        type_id: i32,
        profession_id: i32,
        level: i32,
    },
    /// Serializer 7: the item stack — the object-graph lane's
    /// three-layer decode (see
    /// [`entity_data_serializers::ITEM_STACK`]): the count (the
    /// emptiness SENTINEL — wire count <= 0 is the empty stack and
    /// NOTHING else is on the wire, retained verbatim), the item's
    /// numeric registry id (retained verbatim, the villager line; a
    /// negative id refuses), and the component patch
    /// ([`ComponentPatch`]). The patch's VALUES decode through the
    /// [`data_component_types`] slice and refuse elsewhere — the wire
    /// is unframed, so a value whose codec the port does not carry
    /// cannot be skipped honestly.
    ItemStack {
        count: i32,
        item_id: i32,
        patch: ComponentPatch,
    },
    /// Serializer 21: the cat variant's registry id (the
    /// holderRegistry lane — retained verbatim; a negative id
    /// refuses).
    CatVariant(i32),
    /// Serializer 22: the cat sound variant's registry id.
    CatSoundVariant(i32),
    /// Serializer 23: the cow variant's registry id.
    CowVariant(i32),
    /// Serializer 24: the cow sound variant's registry id.
    CowSoundVariant(i32),
    /// Serializer 25: the wolf variant's registry id.
    WolfVariant(i32),
    /// Serializer 26: the wolf sound variant's registry id.
    WolfSoundVariant(i32),
    /// Serializer 27: the frog variant's registry id.
    FrogVariant(i32),
    /// Serializer 28: the pig variant's registry id.
    PigVariant(i32),
    /// Serializer 29: the pig sound variant's registry id.
    PigSoundVariant(i32),
    /// Serializer 30: the chicken variant's registry id.
    ChickenVariant(i32),
    /// Serializer 31: the chicken sound variant's registry id.
    ChickenSoundVariant(i32),
    /// Serializer 32: the zombie nautilus variant's registry id.
    ZombieNautilusVariant(i32),
    /// Serializer 33: the flag byte, then the dimension string +
    /// packed position only if present (see [`GlobalPos`]).
    OptionalGlobalPos(Option<GlobalPos>),
    /// Serializer 34: the painting variant — the holder family's
    /// only bias-by-one member (see [`PaintingVariant`]).
    PaintingVariant(PaintingVariant),
    /// Serializer 35: the sniffer state's ordinal, retained
    /// verbatim (the ZERO strategy never refuses; seven states,
    /// IDLING = 0 .. RISING = 6).
    SnifferState(i32),
    /// Serializer 36: the armadillo state's ordinal (ZERO; IDLE,
    /// ROLLING, SCARED, UNROLLING).
    ArmadilloState(i32),
    /// Serializer 37: the copper golem state's ordinal (ZERO; IDLE,
    /// GETTING_ITEM, GETTING_NO_ITEM, DROPPING_ITEM,
    /// DROPPING_NO_ITEM).
    CopperGolemState(i32),
    /// Serializer 38: the weathering-copper state's ordinal
    /// (retained verbatim — vanilla's CLAMP strategy maps
    /// out-of-range keys into [0, 3] and the clamp is the
    /// consumer's act; UNAFFECTED, EXPOSED, WEATHERED, OXIDIZED).
    WeatheringCopperState(i32),
    /// Serializer 39: three big-endian f32s (x, y, z) — the
    /// readVector3f shape.
    Vector3([f32; 3]),
    /// Serializer 40: four big-endian f32s (x, y, z, w — the JOML
    /// Quaternionf's own field order) — the readQuaternion shape.
    Quaternion([f32; 4]),
    /// Serializer 42: the main hand's ordinal (LEFT = 0, RIGHT = 1,
    /// the declaration order's own truth; ZERO never refuses).
    HumanoidArm(i32),
}

/// ONE metadata entry as the wire carries it: the accessor's byte
/// index (0..=254 — 255 is the EOF marker), the serializer id (the
/// value's type tag — part of the state, not plumbing) and the
/// decoded value.
#[derive(Debug, Clone, PartialEq)]
pub struct SetEntityDataEntry {
    /// The accessor's byte index (`SynchedEntityData$DataValue.id`).
    pub index: u8,
    /// The serializer's registry id (`getSerializedId`).
    pub serializer: i32,
    /// The decoded value.
    pub value: EntityDataEntry,
}

/// The unpacked Set Entity Data payload (clientbound play, generated
/// id 99 — `minecraft:set_entity_data`). javap of the 26.2
/// `ClientboundSetEntityDataPacket`: a VarInt entity id, then
/// `unpack` — entries until the 0xFF index (the EOF marker; the
/// `readUnsignedByte` comparison is against 255, so real accessors
/// live in 0..=254). Each entry: the index byte, a VarInt serializer
/// id (`EntityDataSerializers.getSerializer` — an id the registry
/// does not know throws, failing the packet read) and the
/// serializer's value codec.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityDataSet {
    /// The entity whose metadata changes.
    pub id: i32,
    /// The entries, in wire order.
    pub entries: Vec<SetEntityDataEntry>,
}

/// The shared `ByteBufCodecs.readCount` walk: a VarInt count. readCount
/// itself only enforces the maxSize CEILING (count > maxSize is the
/// DecoderException); a NEGATIVE count passes the check and fails the
/// read at the collection/map constructor's negative capacity — the
/// read fails either way, so the port refuses at the same lane.
fn read_component_count(c: &mut &[u8], what: &str) -> Result<i32> {
    let n = read_varint(c)?;
    anyhow::ensure!(n >= 0, "negative {what} count {n}");
    Ok(n)
}

/// Read ONE component-patch VALUE through the type's own codec. The
/// wire is UNFRAMED (no length prefix — `DataComponentPatch$3.decode`
/// hands the buffer straight to the type's `streamCodec()`), so an id
/// outside [`data_component_types`] cannot be skipped honestly: a
/// NEGATIVE id refuses where vanilla's `byIdOrThrow` throws, and a
/// non-negative unknown refuses because its byte length is only
/// knowable through a codec the port does not carry — the packet read
/// fails either way, vanilla's own direction.
fn read_component_value(type_id: i32, c: &mut &[u8]) -> Result<ComponentValue> {
    use data_component_types as t;
    Ok(match type_id {
        t::CUSTOM_DATA => {
            let tag = read_bare_nbt(c)?;
            // compoundTagCodec's decode map: anything but a
            // TAG_Compound is the DecoderException
            if !matches!(tag, Nbt::Compound(_)) {
                anyhow::bail!("custom_data must be a compound tag, got another tag shape");
            }
            ComponentValue::CompoundTag(tag)
        }
        t::MAX_STACK_SIZE | t::MAX_DAMAGE | t::DAMAGE | t::REPAIR_COST => {
            ComponentValue::VarInt(read_varint(c)?)
        }
        t::UNBREAKABLE | t::CREATIVE_SLOT_LOCK => {
            // StreamCodec.unit: the value consumes ZERO bytes — the
            // type's presence is the payload
            ComponentValue::Unit
        }
        t::CUSTOM_NAME | t::ITEM_NAME => ComponentValue::Component(read_bare_nbt(c)?),
        t::MINIMUM_ATTACK_CHARGE => ComponentValue::Float(read_f32(c)?),
        t::ITEM_MODEL | t::TOOLTIP_STYLE | t::NOTE_BLOCK_SOUND => {
            ComponentValue::String(read_string(c)?)
        }
        t::LORE => {
            let n = read_varint(c)?;
            anyhow::ensure!(n >= 0, "negative lore list length {n}");
            anyhow::ensure!(
                n <= 256,
                "lore list over the codec's 256-entry cap ({n})"
            );
            let mut tags = Vec::with_capacity(n as usize);
            for _ in 0..n {
                tags.push(read_bare_nbt(c)?);
            }
            ComponentValue::Lore(tags)
        }
        t::RARITY => {
            // `Rarity.BY_ID` = `ByIdMap.continuous(Rarity::ordinal,
            // values(), OutOfBoundsStrategy.ZERO)`: the map NEVER
            // refuses — every key outside [0, 4), negative AND
            // out-of-range, fills to values[0] = COMMON. The wire
            // ordinal is retained verbatim; the ZERO-fill is the
            // consumer's resolution (the operation/slot/animation
            // line — the old negative refusal was STRICTER than
            // vanilla).
            let r = read_varint(c)?;
            ComponentValue::Rarity(r)
        }
        t::ENCHANTMENT_GLINT_OVERRIDE => ComponentValue::Bool(read_u8(c)? != 0),
        t::ENCHANTMENTS => {
            // ByteBufCodecs.map: the readCount VarInt, then count x
            // {key VarInt, value VarInt} — the level rides the wire
            // verbatim (the intRange(1,255) LEVEL_CODEC is the NBT
            // schema, not this codec)
            let n = read_component_count(c, "enchantment map")?;
            let mut entries = Vec::with_capacity(n as usize);
            for _ in 0..n {
                let id = read_varint(c)?;
                anyhow::ensure!(id >= 0, "negative enchantment registry id {id}");
                let level = read_varint(c)?;
                entries.push((id, level));
            }
            ComponentValue::Enchantments(entries)
        }
        t::CUSTOM_MODEL_DATA => {
            // the composite's FOUR lists in order: FLOAT, BOOL,
            // STRING_UTF8, INT — the colors ride BIG-ENDIAN i32s
            // (ByteBufCodecs.INT = readInt), not VarInts
            let nf = read_component_count(c, "model-data floats")?;
            let mut floats = Vec::with_capacity(nf as usize);
            for _ in 0..nf {
                floats.push(read_f32(c)?);
            }
            let nbool = read_component_count(c, "model-data flags")?;
            let mut flags = Vec::with_capacity(nbool as usize);
            for _ in 0..nbool {
                flags.push(read_u8(c)? != 0);
            }
            let ns = read_component_count(c, "model-data strings")?;
            let mut strings = Vec::with_capacity(ns as usize);
            for _ in 0..ns {
                strings.push(read_string(c)?);
            }
            let nc = read_component_count(c, "model-data colors")?;
            let mut colors = Vec::with_capacity(nc as usize);
            for _ in 0..nc {
                colors.push(read_i32(c)?);
            }
            ComponentValue::CustomModelData {
                floats,
                flags,
                strings,
                colors,
            }
        }
        t::TOOLTIP_DISPLAY => {
            // composite(BOOL, DataComponentType.STREAM_CODEC.collection)
            let hide_tooltip = read_u8(c)? != 0;
            let n = read_component_count(c, "hidden components")?;
            let mut hidden = Vec::with_capacity(n as usize);
            for _ in 0..n {
                let id = read_varint(c)?;
                anyhow::ensure!(id >= 0, "negative component type id {id}");
                hidden.push(id);
            }
            ComponentValue::TooltipDisplay { hide_tooltip, hidden }
        }
        t::CAN_PLACE_ON | t::CAN_BREAK => {
            // AdventureModePredicate.STREAM_CODEC: the same record
            // registered under BOTH component ids — one codec, two
            // registry slots
            ComponentValue::BlockPredicates(read_adventure_mode_predicate(c)?)
        }
        t::ATTRIBUTE_MODIFIERS => {
            // ItemAttributeModifiers.STREAM_CODEC: the no-arg list
            // count, then count x the FOUR-lane entry composite
            ComponentValue::AttributeModifiers(read_attribute_modifiers(c)?)
        }
        t::FOOD => {
            // FoodProperties.DIRECT_STREAM_CODEC = composite(VAR_INT,
            // FLOAT, BOOL) — nutrition is a VAR_INT in 26.2, retained
            // verbatim (the record constructor clamps nothing)
            let nutrition = read_varint(c)?;
            let saturation = read_f32(c)?;
            let can_always_eat = read_u8(c)? != 0;
            ComponentValue::Food {
                nutrition,
                saturation,
                can_always_eat,
            }
        }
        t::CONSUMABLE => {
            // Consumable.STREAM_CODEC — the FIVE-lane composite; the
            // effects list is a DISPATCHED registry walk now (each
            // entry = a byIdOrThrow type id + that type's audited
            // payload — the autopush-111 pinned-EMPTY deferral
            // retired)
            let consume_seconds = read_f32(c)?;
            let animation = read_varint(c)?;
            let sound = read_consumable_sound(c)?;
            let has_consume_particles = read_u8(c)? != 0;
            let effects = read_component_count(c, "consume-effect list")?;
            let mut consumed = Vec::with_capacity(effects as usize);
            for _ in 0..effects {
                consumed.push(read_consume_effect(c)?);
            }
            ComponentValue::Consumable {
                consume_seconds,
                animation,
                sound,
                has_consume_particles,
                effects: consumed,
            }
        }
        other => anyhow::bail!(
            "no value codec for component type {other} (the patch wire is unframed; \
             the value cannot be skipped honestly)"
        ),
    })
}

/// The block-set lane of a [`BlockPredicate`] — the
/// `ByteBufCodecs$31` (holderSet) decode verbatim: a VarInt TAG where
/// wire 0 is the NAMED set and wire tag >= 1 is a DIRECT set of
/// `tag - 1` holder ids (so wire tag 1 is the EMPTY direct set —
/// nothing else on the wire). A negative effective count cannot
/// arrive: wire 0 was the named lane, every larger tag yields a
/// non-negative direct count.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockSet {
    /// Wire tag 0: the set is a block TAG, carried as its UTF
    /// identifier (`minecraft:logs`). Vanilla resolves it against the
    /// synced block-tag registry with `orElseThrow` — an unknown tag
    /// name fails the read THERE; this client carries no block-tag
    /// tables, so the identifier is retained verbatim (the
    /// VILLAGER_DATA line: the resolution is a consumer's act) and
    /// the strictness is documented, not enacted at decode.
    Named(String),
    /// Wire tag >= 1: `(tag - 1)` NUMERIC block registry ids (the
    /// `$29` byIdOrThrow VarInt — a negative id refuses exactly where
    /// vanilla's byIdOrThrow throws; the ids themselves are retained
    /// verbatim, the decode-only line).
    Direct(Vec<i32>),
}

/// The value lane of a [`PropertyMatcher`] — the
/// `ByteBufCodecs.either(ExactMatcher, RangedMatcher)` decode: a
/// ONE-BYTE boolean discriminator (true = LEFT, the exact match;
/// false = RIGHT, the ranged match — $26 reads a bare
/// `readBoolean`).
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// The left lane: `ExactMatcher.STREAM_CODEC` = a bare UTF string
    /// ("true"/"false" for boolean properties, the literal value
    /// otherwise).
    Exact(String),
    /// The right lane: `RangedMatcher.STREAM_CODEC` =
    /// `composite(optional(STRING_UTF8), optional(STRING_UTF8))` —
    /// BOTH bounds ride as UTF STRINGS ("10"), not doubles: the
    /// numeric NBT schema is not the wire's shape (the enchantment
    /// level lesson again, in the opposite direction). Either bound
    /// may be absent (the boolean-flag-then-value prefix).
    Ranged {
        /// The inclusive minimum, or None.
        min: Option<String>,
        /// The inclusive maximum, or None.
        max: Option<String>,
    },
}

/// One block-state property constraint of a [`BlockPredicate`] —
/// `PropertyMatcher.STREAM_CODEC` = `composite(STRING_UTF8 name,
/// ValueMatcher.STREAM_CODEC)`, and the predicate itself is
/// `PropertyMatcher.list()` — a VarInt count, then count × this.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyMatcher {
    /// The block-state property's name ("facing", "half", ...).
    pub name: String,
    /// The constraint on its value.
    pub value: PropertyValue,
}

/// The component-matcher lane of a [`BlockPredicate`] —
/// `DataComponentMatchers.STREAM_CODEC` = `composite(
/// DataComponentExactPredicate.STREAM_CODEC,
/// DataComponentPredicate.STREAM_CODEC, ..)`.
///
/// The EXACT side decodes fully:
/// `DataComponentExactPredicate.STREAM_CODEC` =
/// `TypedDataComponent.STREAM_CODEC.list()` — a VarInt count, then
/// count × {the component type id, the type's OWN value}, the SAME
/// walk as the patch's added entries (an unknown id refuses — the
/// unframed wire cannot be skipped honestly).
///
/// The PARTIAL side WALKS (the wave-three deferral retired):
/// `DataComponentPredicate.STREAM_CODEC` = `SINGLE_STREAM_CODEC
/// .list(64)` mapped to a Map — a VarInt count (64-capped, a larger
/// count is vanilla's own DecoderException), then count ×
/// `DataComponentPredicate$Single` entries. Each entry =
/// `StreamCodec.dispatch(Type.STREAM_CODEC, type.singleStreamCodec)`
/// — the type lane ([`PredicateTypeLane`]: a ONE-BYTE either
/// boolean, then a byIdOrThrow VarInt into the branch's registry)
/// followed by the ONE full NBT tag the record codec bridges
/// ([`PartialPredicate`]; the payload is self-delimiting, so the
/// walk continues — the unframed-wire objection is retired by
/// construction).
#[derive(Debug, Clone, PartialEq)]
pub struct DataComponentMatchers {
    /// The exact-match entries: {component type id, the value to
    /// match}, in wire order, decoded through the same walk as the
    /// patch's added entries.
    pub exact: Vec<(i32, ComponentValue)>,
    /// The partial (sub-predicate) entries, in wire order.
    pub partial: Vec<PartialPredicate>,
}

/// ONE adventure-mode block predicate — the `BlockPredicate.STREAM_CODEC`
/// composite verbatim, FOUR lanes in wire order (the first three
/// through `ByteBufCodecs.optional` — a one-byte boolean prefix, then
/// the value only if present; `$23` reads a bare `readBoolean`):
///
/// 1. `optional(holderSet(BLOCK))` — [`BlockSet`];
/// 2. `optional(StatePropertiesPredicate.STREAM_CODEC)` — the
///    `PropertyMatcher.list()` → [`Vec<PropertyMatcher>`];
/// 3. `optional(NbtPredicate.STREAM_CODEC)` — `COMPOUND_TAG.map(..)`,
///    a bare tag whose non-compound shape refuses (the custom_data
///    DecoderException line);
/// 4. `DataComponentMatchers.STREAM_CODEC` — [`DataComponentMatchers`].
///
/// An all-absent predicate (all three optionals false, both matcher
/// lists empty) is the decodable ANY-blocks shape — nothing after
/// the four lane bytes.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockPredicate {
    /// The block-set lane: which blocks (a tag, explicit ids, or
    /// absent = any).
    pub blocks: Option<BlockSet>,
    /// The block-state property constraints, or absent.
    pub properties: Option<Vec<PropertyMatcher>>,
    /// The block-entity NBT constraint, or absent.
    pub nbt: Option<Nbt>,
    /// The component-matcher lane.
    pub matchers: DataComponentMatchers,
}

/// ONE attribute-modifier entry of the held-item's stat surface —
/// `ItemAttributeModifiers$Entry.STREAM_CODEC` = a FOUR-lane
/// composite, wire order:
///
/// 1. `attribute` — `ByteBufCodecs.holderRegistry(ATTRIBUTE)` = the
///    `$29` byIdOrThrow VarInt: a negative id refuses exactly where
///    vanilla throws, a non-negative unknown is retained verbatim
///    (this client carries no attribute registry — the ENCHANTMENTS
///    line);
/// 2. `modifier` — `AttributeModifier.STREAM_CODEC` =
///    `composite(Identifier UTF id, ByteBufCodecs.DOUBLE amount,
///    Operation.STREAM_CODEC)`: the amount rides a BIG-ENDIAN f64
///    (the DOUBLE/INT/LONG trap family — the construction site is
///    the shape, the field's name is not), the operation an idMapper
///    ordinal retained verbatim (0 = add, 1 = add_multiplied_base,
///    2 = add_multiplied_total; vanilla's
///    `ByIdMap.continuous(.., OutOfBoundsStrategy.ZERO)` NEVER
///    refuses — `lambda$continuous$0` maps every key outside
///    `[0, len)` to values[0], so a refusal would invent a data flow
///    vanilla does not have; the resolution is a consumer's act);
/// 3. `slot` — `EquipmentSlotGroup.STREAM_CODEC` = the SAME idMapper
///    shape (ANY=0, MAINHAND=1, OFFHAND=2, HAND=3, FEET=4, LEGS=5,
///    CHEST=6, HEAD=7, ARMOR=8, BODY=9, SADDLE=10), retained
///    verbatim for the same ZERO-fill reason;
/// 4. `display` — the 26.2-NEW `ItemAttributeModifiers$Display`
///    DISPATCH (`StreamCodec.dispatch(typeGetter,
///    Type.STREAM_CODEC.cast())`): the Type ordinal (DEFAULT=0,
///    HIDDEN=1, OVERRIDE=2) then the variant's payload. The ordinal
///    is FRAMING — it selects the payload walk — so an out-of-range
///    or negative ordinal fills to DEFAULT (the continuous map's
///    own behavior) and the decode walks the zero-byte unit lane;
///    the fill is pinned by a test.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeModifierEntry {
    /// The attribute's registry id (the `$29` VarInt).
    pub attribute_id: i32,
    /// The modifier record (id, amount, operation).
    pub modifier: AttributeModifier,
    /// The slot group's idMapper ordinal, retained verbatim.
    pub slot: i32,
    /// The 26.2 display dispatch, decoded to its variant.
    pub display: AttributeDisplay,
}

/// The modifier record of an [`AttributeModifierEntry`] —
/// `AttributeModifier.STREAM_CODEC`'s three lanes.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeModifier {
    /// The modifier's identifier (the UUID wire form is long gone —
    /// 26.2 rides an Identifier UTF string).
    pub id: String,
    /// The amount — a BIG-ENDIAN f64 (`ByteBufCodecs.DOUBLE`).
    pub amount: f64,
    /// The operation's idMapper ordinal, retained verbatim (the
    /// ZERO-fill is a consumer's act).
    pub operation: i32,
}

/// The 26.2-NEW display lane of an [`AttributeModifierEntry`] — the
/// `ItemAttributeModifiers$Display` dispatch decoded to its variant
/// (vanilla's own decode state IS the Display instance, so the store
/// mirrors the instance, not the raw ordinal).
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeDisplay {
    /// Wire 0 — `Display$Default.STREAM_CODEC` = `StreamCodec.unit`:
    /// ZERO payload bytes. Also the ZERO-fill lane: an out-of-range
    /// or negative Type ordinal decodes HERE (the continuous map
    /// never refuses), pinned by a test.
    Default,
    /// Wire 1 — `Display$Hidden.STREAM_CODEC` = `StreamCodec.unit`:
    /// ZERO payload bytes.
    Hidden,
    /// Wire 2 — `Display$OverrideText.STREAM_CODEC` =
    /// `composite(ComponentSerialization.STREAM_CODEC)`: ONE bare
    /// component tag.
    Override(Nbt),
}

/// The sound lane of [`ComponentValue::Consumable`] and the
/// `play_sound` consume-effect's payload — the SAME
/// `SoundEvent.STREAM_CODEC` = `ByteBufCodecs.holder(SOUND_EVENT,
/// DIRECT_STREAM_CODEC)` (one codec, two call sites — the registry
/// slot is per-component, the codec is per-RECORD): a TAGGED union
/// (the wave-three holderSet's shape family) where wire 0 is the
/// DIRECT lane and wire tag >= 1 the registry lane.
#[derive(Debug, Clone, PartialEq)]
pub enum ConsumableSound {
    /// Wire 0: `SoundEvent.DIRECT_STREAM_CODEC` =
    /// `composite(Identifier, FLOAT)` — the sound's location and its
    /// range, decoded verbatim.
    Direct {
        /// The sound's identifier ("minecraft:item.generic.eat").
        id: String,
        /// The sound's range — a BE f32.
        range: f32,
    },
    /// Wire tag >= 1: the `(tag - 1)` registry id, retained verbatim
    /// (this client carries no sound registry — the VILLAGER_DATA
    /// line). A NEGATIVE wire tag refuses: vanilla's registry lookup
    /// throws on it (the tag-1 arithmetic would go below the table).
    Registry(i32),
}

/// One entry of the consumable's effects list — the
/// `ConsumeEffect.STREAM_CODEC` DISPATCH decoded verbatim:
/// `StreamCodec.dispatch(ByteBufCodecs.registry(CONSUME_EFFECT_TYPE),
/// ..)` — a VarInt registry id (`$29` = `IdMap.byIdOrThrow`: an
/// unknown or negative id throws, the port refuses at the same lane)
/// followed by that type's own payload. The five known types ride
/// the registry's clinit registration order; their registry NAMES
/// (apply_effects, remove_effects, clear_all_effects,
/// teleport_randomly, play_sound) carry no "status" infix — the
/// wiki-era names are not the wire's (the fourth name-vs-wire
/// divergence this registry family has shown).
#[derive(Debug, Clone, PartialEq)]
pub enum ConsumeEffect {
    /// Wire id 0 — `ApplyStatusEffectsConsumeEffect.STREAM_CODEC` =
    /// `composite(MobEffectInstance.STREAM_CODEC.list(), FLOAT)` —
    /// the no-arg list (an `Integer.MAX_VALUE` ceiling; a negative
    /// count dies at the constructor, the shared readCount shape)
    /// then the application probability as a plain BE f32.
    ApplyEffects {
        /// The effects to apply, in wire order (see
        /// [`EffectInstance`]).
        effects: Vec<EffectInstance>,
        /// The application probability — a BE f32, retained verbatim
        /// (the roll is a consumer's act).
        probability: f32,
    },
    /// Wire id 1 — `RemoveStatusEffectsConsumeEffect.STREAM_CODEC` =
    /// `composite(ByteBufCodecs.holderSet(MOB_EFFECT))` — the
    /// wave-three holderSet tagged union (see [`EffectHolderSet`]).
    RemoveEffects(EffectHolderSet),
    /// Wire id 2 — `ClearAllStatusEffectsConsumeEffect.STREAM_CODEC`
    /// = `StreamCodec.unit(INSTANCE)`: ZERO payload bytes (the
    /// cheapest dispatched lane on the wire).
    ClearAllEffects,
    /// Wire id 3 — `TeleportRandomlyConsumeEffect.STREAM_CODEC` =
    /// `composite(FLOAT)`: the teleport diameter as a plain BE f32
    /// (the DEFAULT_DIAMETER constant is the NBT-side default, not
    /// the wire's — the wire always carries the float).
    TeleportRandomly {
        /// The teleport diameter — a BE f32.
        diameter: f32,
    },
    /// Wire id 4 — `PlaySoundConsumeEffect.STREAM_CODEC` =
    /// `composite(SoundEvent.STREAM_CODEC)` — the SAME tagged holder
    /// union as the consumable's sound lane ([`ConsumableSound`];
    /// one codec, two call sites).
    PlaySound(ConsumableSound),
}

/// The effect holder set — `ByteBufCodecs.holderSet(MOB_EFFECT)`,
/// the wave-three `ByteBufCodecs$31` shape exactly: a VarInt TAG
/// where wire 0 is the NAMED set (an UTF identifier, retained
/// verbatim — vanilla resolves it orElseThrow against the synced
/// effect tags, the VILLAGER_DATA line) and wire tag >= 1 a DIRECT
/// set of `(tag - 1)` byIdOrThrow effect ids (a negative id
/// refuses; a negative TAG cannot reach the arithmetic — the named
/// lane owned wire 0, the port refuses it explicitly).
#[derive(Debug, Clone, PartialEq)]
pub enum EffectHolderSet {
    /// Wire tag 0: the set is an effect TAG, carried as its UTF
    /// identifier (`minecraft:harmful`).
    Named(String),
    /// Wire tag >= 1: `(tag - 1)` numeric effect registry ids.
    Direct(Vec<i32>),
}

/// One applied effect — `MobEffectInstance.STREAM_CODEC` =
/// `composite(MobEffect.STREAM_CODEC, Details.STREAM_CODEC)` — a
/// TWO-lane composite: the effect's registry id and the recursive
/// details record.
#[derive(Debug, Clone, PartialEq)]
pub struct EffectInstance {
    /// `MobEffect.STREAM_CODEC` = `ByteBufCodecs.holderRegistry(
    /// MOB_EFFECT)` — the `$29` byIdOrThrow VarInt (a negative id
    /// refuses exactly where vanilla's lookup throws; the id is
    /// retained verbatim, the decode-only line).
    pub effect_id: i32,
    /// The recursive `Details` composite (see [`EffectDetails`]).
    pub details: EffectDetails,
}

/// The recursive `MobEffectInstance$Details.STREAM_CODEC` —
/// `StreamCodec.recursive(composite(VAR_INT amplifier, VAR_INT
/// duration, BOOL ambient, BOOL showParticles, BOOL showIcon,
/// optional(recursive self) hiddenEffect))`: six lanes in the
/// record's component order (the bootstrap strings confirm), the
/// sixth a ONE-BYTE readBoolean prefix (`ByteBufCodecs.optional`,
/// the `$23` shape) then, if present, the SAME composite again —
/// vanilla's recursion mirrored walk-for-walk (an unbounded wire
/// recursion stack-overflows vanilla's own recursive codec too).
#[derive(Debug, Clone, PartialEq)]
pub struct EffectDetails {
    /// The amplifier — a plain VarInt, retained verbatim (the
    /// MIN/MAX_AMPLIFIER constants are apply()-side, not decode-side).
    pub amplifier: i32,
    /// The duration in ticks — a plain VarInt (`-1` is the wire's
    /// own INFINITE_DURATION marker, not a refusal).
    pub duration: i32,
    /// The ambient flag — one byte.
    pub ambient: bool,
    /// The show-particles flag — one byte.
    pub show_particles: bool,
    /// The show-icon flag — one byte.
    pub show_icon: bool,
    /// The hidden (upgraded) effect — one-byte prefix then the
    /// recursive composite, boxed because the record is recursive.
    pub hidden_effect: Option<Box<EffectDetails>>,
}

/// `ByteBufCodecs.optional` (`$23`) verbatim: a one-byte boolean
/// prefix, then the value only if present. Used by the ranged
/// property matcher's two bounds (and the same shape as the
/// predicate's own optional lanes).
fn read_optional_string(c: &mut &[u8]) -> Result<Option<String>> {
    if read_u8(c)? != 0 {
        Ok(Some(read_string(c)?))
    } else {
        Ok(None)
    }
}

/// `AdventureModePredicate.STREAM_CODEC` verbatim: `composite(
/// BlockPredicate.STREAM_CODEC.list(), ..)` — a VarInt count over the
/// NO-ARG `ByteBufCodecs.list()` (ceiling `Integer.MAX_VALUE`; the
/// readCount check is the only guard, a negative count still dies at
/// the constructor), then count × [`BlockPredicate`].
fn read_adventure_mode_predicate(c: &mut &[u8]) -> Result<Vec<BlockPredicate>> {
    let n = read_component_count(c, "block predicate list")?;
    let mut predicates = Vec::with_capacity(n as usize);
    for _ in 0..n {
        predicates.push(read_block_predicate(c)?);
    }
    Ok(predicates)
}

/// ONE [`BlockPredicate`] — the four-lane composite in wire order.
/// See the type's doc for the full audit provenance.
fn read_block_predicate(c: &mut &[u8]) -> Result<BlockPredicate> {
    // lane 1: optional(holderSet(BLOCK)) — the $31 VarInt TAG:
    // wire 0 = the named set (an identifier), wire tag >= 1 = a
    // DIRECT set of (tag - 1) byIdOrThrow ids
    let blocks = if read_u8(c)? != 0 {
        let tag = read_varint(c)?;
        if tag == 0 {
            // NAMED: the tag identifier, retained verbatim —
            // vanilla resolves it orElseThrow against the synced
            // block tags; this client has no tag tables, so the
            // strictness is a consumer's act (the VILLAGER_DATA line)
            Some(BlockSet::Named(read_string(c)?))
        } else {
            let count = tag - 1; // can never be negative: wire 0 was
            // the named lane and everything below it REFUSES (vanilla's
            // ArrayList(min(i, 65536)) throws on the negative capacity;
            // the consume-effect round's probe caught this lane's
            // missing refusal — a panic was never vanilla's behavior)
            anyhow::ensure!(tag >= 1, "negative holder-set tag {tag}");
            let mut ids = Vec::with_capacity(count as usize);
            for _ in 0..count {
                let id = read_varint(c)?;
                anyhow::ensure!(id >= 0, "negative block registry id {id}");
                ids.push(id);
            }
            Some(BlockSet::Direct(ids))
        }
    } else {
        None
    };

    // lane 2: optional(StatePropertiesPredicate) — the
    // PropertyMatcher.list() walk: VarInt count, then count x
    // {UTF name, either(exact UTF, ranged {optional UTF, optional UTF})}
    let properties = if read_u8(c)? != 0 {
        let n = read_component_count(c, "state property list")?;
        let mut props = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let name = read_string(c)?;
            // ByteBufCodecs.either ($26): a bare readBoolean —
            // true = LEFT (the exact matcher), false = RIGHT (ranged)
            let value = if read_u8(c)? != 0 {
                PropertyValue::Exact(read_string(c)?)
            } else {
                PropertyValue::Ranged {
                    min: read_optional_string(c)?,
                    max: read_optional_string(c)?,
                }
            };
            props.push(PropertyMatcher { name, value });
        }
        Some(props)
    } else {
        None
    };

    // lane 3: optional(NbtPredicate) — COMPOUND_TAG.map(..): a bare
    // tag whose non-compound shape refuses (the custom_data
    // DecoderException line)
    let nbt = if read_u8(c)? != 0 {
        let tag = read_bare_nbt(c)?;
        if !matches!(tag, Nbt::Compound(_)) {
            anyhow::bail!("nbt predicate must be a compound tag, got another tag shape");
        }
        Some(tag)
    } else {
        None
    };

    // lane 4: DataComponentMatchers — the exact list decodes (the
    // patch's added-entry walk), the partial list WALKS the
    // dispatched entries (the wave-three deferral retired)
    let exact_n = read_component_count(c, "exact matcher list")?;
    let mut exact = Vec::with_capacity(exact_n as usize);
    for _ in 0..exact_n {
        let type_id = read_varint(c)?;
        anyhow::ensure!(type_id >= 0, "negative component type id {type_id}");
        let value = read_component_value(type_id, c)?;
        exact.push((type_id, value));
    }
    let partial_n = read_component_count(c, "predicate matcher list")?;
    anyhow::ensure!(
        partial_n <= 64,
        "predicate matcher list over the codec's 64-entry cap ({partial_n})"
    );
    let mut partial = Vec::with_capacity(partial_n as usize);
    for _ in 0..partial_n {
        let predicate_type = read_predicate_type_lane(c)?;
        let payload = read_bare_nbt(c)?;
        partial.push(PartialPredicate {
            predicate_type,
            payload,
        });
    }

    Ok(BlockPredicate {
        blocks,
        properties,
        nbt,
        matchers: DataComponentMatchers { exact, partial },
    })
}

/// The partial matcher's dispatched TYPE lane —
/// `DataComponentPredicate$Type.STREAM_CODEC` =
/// `ByteBufCodecs.either(registry(DATA_COMPONENT_PREDICATE_TYPE),
/// registry(DATA_COMPONENT_TYPE))` mapped to the Type: a ONE-BYTE
/// readBoolean ($26 — true = LEFT), then a `$29` byIdOrThrow VarInt
/// into the chosen branch's registry. Both registries are real
/// lookups (NOT the idMapper-continuous family): a negative AND an
/// out-of-range id throw — the port refuses at the same lane. LEFT
/// is the built-in 15-row `DataComponentPredicates` table; RIGHT is
/// the synced 111-row DATA_COMPONENT_TYPE table — the `AnyValueType`
/// ("the component is present, any value").
fn read_predicate_type_lane(c: &mut &[u8]) -> Result<PredicateTypeLane> {
    let left = read_u8(c)? != 0;
    let id = read_varint(c)?;
    if left {
        anyhow::ensure!(
            (0..data_component_predicate_types::REGISTRY_SIZE).contains(&id),
            "unknown dispatched predicate type id {id} (the byIdOrThrow lane: \
             vanilla throws for anything outside [0, {}) — the built-in \
             DataComponentPredicates table)",
            data_component_predicate_types::REGISTRY_SIZE
        );
        Ok(PredicateTypeLane::PredicateType(id))
    } else {
        anyhow::ensure!(
            (0..data_component_types::REGISTRY_SIZE).contains(&id),
            "unknown component type id {id} in the any-value predicate lane \
             (the byIdOrThrow lane: vanilla throws for anything outside \
             [0, {}) — the synced DATA_COMPONENT_TYPE table)",
            data_component_types::REGISTRY_SIZE
        );
        Ok(PredicateTypeLane::ComponentType(id))
    }
}

/// The attribute-modifiers walk — the outer NO-ARG list count, then
/// count × the FOUR-lane entry composite (see
/// [`AttributeModifierEntry`] for the full lane provenance).
fn read_attribute_modifiers(c: &mut &[u8]) -> Result<Vec<AttributeModifierEntry>> {
    let n = read_component_count(c, "attribute-modifier list")?;
    let mut entries = Vec::with_capacity(n as usize);
    for _ in 0..n {
        // lane 1: holderRegistry(ATTRIBUTE) — the $29 VarInt
        let attribute_id = read_varint(c)?;
        anyhow::ensure!(
            attribute_id >= 0,
            "negative attribute registry id {attribute_id}"
        );
        // lane 2: composite(Identifier UTF, DOUBLE = BE f64, ordinal)
        let id = read_string(c)?;
        let amount = read_f64(c)?;
        let operation = read_varint(c)?;
        // lane 3: the slot group's idMapper ordinal (retained
        // verbatim — the ZERO-fill is a consumer's act)
        let slot = read_varint(c)?;
        // lane 4: the Display dispatch — the Type ordinal selects the
        // payload walk; out-of-range fills to DEFAULT (the continuous
        // map's own behavior) and walks the zero-byte unit lane
        let display = match read_varint(c)? {
            1 => AttributeDisplay::Hidden,
            2 => AttributeDisplay::Override(read_bare_nbt(c)?),
            _ => AttributeDisplay::Default,
        };
        entries.push(AttributeModifierEntry {
            attribute_id,
            modifier: AttributeModifier {
                id,
                amount,
                operation,
            },
            slot,
            display,
        });
    }
    Ok(entries)
}

/// The consumable sound lane — `ByteBufCodecs.holder(..)`: a TAGGED
/// union where wire 0 is the DIRECT `{UTF id, f32 range}` lane and
/// wire tag >= 1 the `(tag - 1)` registry id (a negative tag refuses
/// — vanilla's registry lookup throws on it).
fn read_consumable_sound(c: &mut &[u8]) -> Result<ConsumableSound> {
    let tag = read_varint(c)?;
    if tag == 0 {
        Ok(ConsumableSound::Direct {
            id: read_string(c)?,
            range: read_f32(c)?,
        })
    } else if tag > 0 {
        Ok(ConsumableSound::Registry(tag - 1))
    } else {
        anyhow::bail!("negative sound holder tag {tag}");
    }
}

/// ONE dispatched consume-effect entry — the registry id (`$29`
/// byIdOrThrow: unknown AND negative refuse) then the type's own
/// payload walk. The five lanes mirror the five `ConsumeEffect$Type`
/// clinit registrations in registry order.
fn read_consume_effect(c: &mut &[u8]) -> Result<ConsumeEffect> {
    let type_id = read_varint(c)?;
    match type_id {
        0 => {
            // apply_effects: composite(list(MobEffectInstance), FLOAT)
            let n = read_component_count(c, "apply-effects list")?;
            let mut effects = Vec::with_capacity(n as usize);
            for _ in 0..n {
                effects.push(read_effect_instance(c)?);
            }
            Ok(ConsumeEffect::ApplyEffects {
                effects,
                probability: read_f32(c)?,
            })
        }
        1 => {
            // remove_effects: composite(holderSet(MOB_EFFECT)) — the
            // wave-three tagged union: wire 0 named, wire >= 1 direct
            let tag = read_varint(c)?;
            if tag == 0 {
                Ok(ConsumeEffect::RemoveEffects(EffectHolderSet::Named(
                    read_string(c)?,
                )))
            } else {
                // vanilla: i = tag - 1, then ArrayList(min(i, 65536)) —
                // a NEGATIVE tag yields a negative capacity and the
                // constructor throws (IllegalArgumentException), so
                // the port refuses at the same lane (the panic-free
                // form of the same refusal; the negative-tag probe
                // caught the with_capacity overflow this round)
                anyhow::ensure!(tag >= 1, "negative effect holder-set tag {tag}");
                let count = tag - 1;
                let mut ids = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    let id = read_varint(c)?;
                    anyhow::ensure!(id >= 0, "negative effect registry id {id}");
                    ids.push(id);
                }
                Ok(ConsumeEffect::RemoveEffects(EffectHolderSet::Direct(ids)))
            }
        }
        2 => Ok(ConsumeEffect::ClearAllEffects), // StreamCodec.unit: ZERO bytes
        3 => Ok(ConsumeEffect::TeleportRandomly {
            diameter: read_f32(c)?,
        }),
        4 => Ok(ConsumeEffect::PlaySound(read_consumable_sound(c)?)),
        other => anyhow::bail!("unknown consume-effect registry id {other} (byIdOrThrow)"),
    }
}

/// ONE `MobEffectInstance` — the effect's byIdOrThrow registry id
/// then the recursive details composite.
fn read_effect_instance(c: &mut &[u8]) -> Result<EffectInstance> {
    let effect_id = read_varint(c)?;
    anyhow::ensure!(effect_id >= 0, "negative effect registry id {effect_id}");
    Ok(EffectInstance {
        effect_id,
        details: read_effect_details(c)?,
    })
}

/// The recursive `Details` composite — six lanes in the record's
/// component order, the sixth `optional(recursive self)` (a one-byte
/// prefix then, if present, THIS walk again). The recursion mirrors
/// vanilla's `StreamCodec.recursive` walk-for-walk.
fn read_effect_details(c: &mut &[u8]) -> Result<EffectDetails> {
    let amplifier = read_varint(c)?;
    let duration = read_varint(c)?;
    let ambient = read_u8(c)? != 0;
    let show_particles = read_u8(c)? != 0;
    let show_icon = read_u8(c)? != 0;
    let hidden_effect = if read_u8(c)? != 0 {
        Some(Box::new(read_effect_details(c)?))
    } else {
        None
    };
    Ok(EffectDetails {
        amplifier,
        duration,
        ambient,
        show_particles,
        show_icon,
        hidden_effect,
    })
}

/// The `DataComponentPatch$3.decode` walk: VarInt added count, VarInt
/// removed count (both zero is the EMPTY patch — the early return the
/// empty-vector result honestly reproduces), then `added` × {type id,
/// the type's value} and `removed` × {type id}. A negative count
/// refuses (vanilla's loop would skip and the fastutil map
/// constructor's negative capacity throws — the read fails there too).
fn parse_component_patch(c: &mut &[u8]) -> Result<ComponentPatch> {
    let added_count = read_varint(c)?;
    let removed_count = read_varint(c)?;
    anyhow::ensure!(added_count >= 0, "negative patch added count {added_count}");
    anyhow::ensure!(
        removed_count >= 0,
        "negative patch removed count {removed_count}"
    );
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let type_id = read_varint(c)?;
        anyhow::ensure!(type_id >= 0, "negative component type id {type_id}");
        let value = read_component_value(type_id, c)?;
        added.push((type_id, value));
    }
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        let type_id = read_varint(c)?;
        anyhow::ensure!(type_id >= 0, "negative component type id {type_id}");
        removed.push(type_id);
    }
    Ok(ComponentPatch { added, removed })
}

/// The UUID's 16 bytes: msb long then lsb long, both big-endian
/// (`FriendlyByteBuf.readUUID` = `new UUID(readLong(), readLong())`
/// — the reference lane and the profile lane share the ONE codec).
fn read_uuid(r: &mut &[u8]) -> std::io::Result<[u8; 16]> {
    let mut uuid = [0u8; 16];
    r.read_exact(&mut uuid)?;
    Ok(uuid)
}

fn read_entity_reference(r: &mut &[u8]) -> std::io::Result<EntityReference> {
    Ok(EntityReference {
        uuid: read_uuid(r)?,
    })
}

/// The vibration's dispatched position source: the
/// POSITION_SOURCE_TYPE registry's VarInt id (block 0, entity 1 —
/// the clinit order; byIdOrThrow refuses anything else), then that
/// type's own composite.
fn read_position_source(r: &mut &[u8]) -> Result<PositionSource> {
    let t = read_varint(r)?;
    anyhow::ensure!(
        (0..2).contains(&t),
        "unknown position source type {t} (the registry's byIdOrThrow direction)"
    );
    Ok(match t {
        0 => {
            let (x, y, z) = decode_block_pos(read_i64(r)?);
            PositionSource::Block([x, y, z])
        }
        _ => PositionSource::Entity {
            entity_id: read_varint(r)?,
            y_offset: read_f32(r)?,
        },
    })
}

/// The dispatched particle walk: the VarInt registry id
/// (`ByteBufCodecs.registry(PARTICLE_TYPE)` = the $29 byIdOrThrow
/// family — an unknown AND a negative id throw), then the type's
/// own payload codec. The 103 simple types ride ZERO payload bytes;
/// the 22 carriers' composites are each shape-pinned by javap.
fn read_particle_value(r: &mut &[u8]) -> Result<ParticleValue> {
    let id = read_varint(r)?;
    anyhow::ensure!(
        (0..particle_types::PARTICLE_TYPE_COUNT).contains(&id),
        "unknown particle type id {id} (the registry's byIdOrThrow direction)"
    );
    Ok(match id {
        particle_types::BLOCK
        | particle_types::BLOCK_MARKER
        | particle_types::FALLING_DUST
        | particle_types::DUST_PILLAR
        | particle_types::BLOCK_CRUMBLE => {
            // BlockParticleOption = idMapper(BLOCK_STATE_REGISTRY):
            // the state-palette lookup, negative refuses
            let s = read_varint(r)?;
            anyhow::ensure!(s >= 0, "negative block state id {s}");
            ParticleValue::Block { particle_id: id, state: s }
        }
        particle_types::GEYSER => ParticleValue::Geyser {
            water_blocks: read_i32(r)?,
        },
        particle_types::GEYSER_BASE | particle_types::GEYSER_POOF => ParticleValue::GeyserBase {
            particle_id: id,
            water_blocks: read_i32(r)?,
            burst_impulse_base: read_f32(r)?,
        },
        particle_types::DRAGON_BREATH => ParticleValue::Power(read_f32(r)?),
        particle_types::DUST => ParticleValue::Dust {
            color: read_i32(r)?,
            scale: read_f32(r)?,
        },
        particle_types::DUST_COLOR_TRANSITION => ParticleValue::DustTransition {
            from_color: read_i32(r)?,
            to_color: read_i32(r)?,
            scale: read_f32(r)?,
        },
        particle_types::EFFECT | particle_types::INSTANT_EFFECT => ParticleValue::Spell {
            particle_id: id,
            color: read_i32(r)?,
            power: read_f32(r)?,
        },
        particle_types::ENTITY_EFFECT
        | particle_types::TINTED_LEAVES
        | particle_types::FLASH => {
            let color = read_i32(r)?;
            ParticleValue::Color { particle_id: id, color }
        }
        particle_types::ITEM => {
            // ItemStackTemplate: item (the $29 byIdOrThrow family —
            // negative refuses, mirroring the metadata item-stack's
            // item-id lane), the RAW count lane, the patch
            let item_id = read_varint(r)?;
            anyhow::ensure!(item_id >= 0, "negative item registry id {item_id}");
            let count = read_varint(r)?;
            let patch = parse_component_patch(r)?;
            ParticleValue::Item {
                item_id,
                count,
                patch,
            }
        }
        particle_types::VIBRATION => ParticleValue::Vibration {
            source: read_position_source(r)?,
            arrival_in_ticks: read_varint(r)?,
        },
        particle_types::TRAIL => ParticleValue::Trail {
            target: [read_f64(r)?, read_f64(r)?, read_f64(r)?],
            color: read_i32(r)?,
            duration: read_varint(r)?,
        },
        particle_types::SHRIEK => ParticleValue::Shriek(read_varint(r)?),
        particle_types::SCULK_CHARGE => ParticleValue::SculkCharge(read_f32(r)?),
        _ => ParticleValue::Simple(id),
    })
}

/// The capped string read: the wire's `Utf8String.read(buf, max)` —
/// the VarInt length IS the byte length, so the post-read check is
/// exact; an over-long string throws in vanilla.
fn read_capped_string(r: &mut &[u8], cap: usize) -> Result<String> {
    let s = read_string(r)?;
    anyhow::ensure!(
        s.len() <= cap,
        "string of {} bytes exceeds the {cap} cap (Utf8String's bound)",
        s.len()
    );
    Ok(s)
}

/// The holderRegistry lane's registry id (the twelve variant-holder
/// serializers): a VarInt where `byIdOrThrow` throws on a negative
/// AND an unknown id. The port refuses the negative half (the
/// arithmetic-only check, the villager/block-state direction) and
/// retains the non-negative unknown verbatim — no variant-registry
/// tables ride this client, so the membership check is the
/// consumer's act.
fn read_holder_id(r: &mut &[u8], what: &str) -> Result<i32> {
    let id = read_varint(r)?;
    anyhow::ensure!(id >= 0, "negative {what} registry id {id}");
    Ok(id)
}

/// The nullable string: the one-byte flag, then the capped string
/// (FriendlyByteBuf.readNullable's shape).
fn read_nullable_capped_string(r: &mut &[u8], cap: usize) -> Result<Option<String>> {
    Ok(match read_u8(r)? {
        0 => None,
        _ => Some(read_capped_string(r, cap)?),
    })
}

/// The PropertyMap (the anonymous ByteBufCodecs$32): readCount(buf,
/// 16) — ONLY the upper bound throws; a NEGATIVE count passes and
/// the builder loop iterates zero times (an EMPTY map, the
/// never-refuse family), then count × {name ≤64, value ≤32767,
/// nullable signature ≤1024}.
fn read_profile_properties(r: &mut &[u8]) -> Result<Vec<ProfileProperty>> {
    let n = read_varint(r)?;
    anyhow::ensure!(
        n <= 16,
        "profile property count {n} exceeds the 16 cap (readCount's bound)"
    );
    let mut props = Vec::new();
    for _ in 0..n {
        props.push(ProfileProperty {
            name: read_capped_string(r, 64)?,
            value: read_capped_string(r, 32767)?,
            signature: read_nullable_capped_string(r, 1024)?,
        });
    }
    Ok(props)
}

/// The optional skin asset: the one-byte flag, then the two
/// identifier strings (each riding STRING_UTF8's 32767 cap).
fn read_optional_skin_asset(r: &mut &[u8]) -> Result<Option<SkinAsset>> {
    Ok(match read_u8(r)? {
        0 => None,
        _ => Some(SkinAsset {
            id: read_capped_string(r, 32767)?,
            texture_path: read_capped_string(r, 32767)?,
        }),
    })
}

/// The resolvable profile walk: the either's flag (true = LEFT = the
/// full GameProfile), the identity side's own lanes, then the skin
/// patch's four one-byte-flag optionals (the model's bool: true =
/// SLIM, false = WIDE — the enum's own lambda map).
fn read_resolvable_profile(r: &mut &[u8]) -> Result<(ResolvableProfile, PlayerSkinPatch)> {
    let profile = if read_u8(r)? != 0 {
        ResolvableProfile::Full {
            uuid: read_uuid(r)?,
            name: read_capped_string(r, 16)?,
            properties: read_profile_properties(r)?,
        }
    } else {
        ResolvableProfile::Partial {
            name: read_nullable_capped_string(r, 16)?,
            id: match read_u8(r)? {
                0 => None,
                _ => Some(read_uuid(r)?),
            },
            properties: read_profile_properties(r)?,
        }
    };
    let patch = PlayerSkinPatch {
        body: read_optional_skin_asset(r)?,
        cape: read_optional_skin_asset(r)?,
        elytra: read_optional_skin_asset(r)?,
        model_slim: match read_u8(r)? {
            0 => None,
            _ => Some(read_u8(r)? != 0),
        },
    };
    Ok((profile, patch))
}

/// Parse Set Entity Data. The supported serializers are the wire's
/// primitive registry PLUS the chat component (the bare-NBT tag — the
/// nbt reader the chunk path already trusts) PLUS the villager record
/// (three VarInt registry ids) PLUS the item stack (the three-layer
/// count/item/patch decode through the [`data_component_types`]
/// slice) PLUS the living-entity reference (the 16-byte UUID the
/// wire carries — the resolution is the client's act) PLUS the
/// dispatched particle registry (the id + the type's own payload,
/// all 125 types — 103 simple, 22 payload carriers) PLUS the
/// resolvable profile (the either(full/partial) identity + the skin
/// patch — the count-capped properties and the flag-optional lanes)
/// PLUS the variant holders (twelve bias-zero `holderRegistry` id
/// lanes) PLUS the optional global pos (the flag byte + dimension
/// STRING + packed position) PLUS the painting variant (the holder
/// family's only bias-by-one member: n == 0 the inline record,
/// n - 1 the registry id) PLUS the enum states (three ZERO strategies
/// and the weathering-copper CLAMP, all never-refusing ordinals)
/// PLUS the vector/quaternion float lanes and the main hand — the
/// FULL registry of 43 serializers (ids 0-42). Only an id the
/// registry itself does not know refuses now: a value's byte length
/// is only knowable through its codec, so there is no honest skip,
/// and the vanilla read side fails the packet exactly the same way
/// (`getSerializer` throws on an unknown id).
pub fn parse_set_entity_data(payload: &[u8]) -> Result<EntityDataSet> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SET_ENTITY_DATA,
        "not a set entity data packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let mut entries = Vec::new();
    loop {
        let index = read_u8(&mut c)?;
        if index == 255 {
            break;
        }
        let serializer = read_varint(&mut c)?;
        let value = match serializer {
            entity_data_serializers::BYTE => EntityDataEntry::Byte(read_i8(&mut c)?),
            entity_data_serializers::INT => EntityDataEntry::Int(read_varint(&mut c)?),
            entity_data_serializers::LONG => EntityDataEntry::Long(read_varlong(&mut c)?),
            entity_data_serializers::FLOAT => EntityDataEntry::Float(read_f32(&mut c)?),
            entity_data_serializers::STRING => EntityDataEntry::String(read_string(&mut c)?),
            entity_data_serializers::COMPONENT => EntityDataEntry::Component(read_bare_nbt(&mut c)?),
            entity_data_serializers::OPTIONAL_COMPONENT => {
                let present = read_u8(&mut c)? != 0;
                let tag = if present { Some(read_bare_nbt(&mut c)?) } else { None };
                EntityDataEntry::OptionalComponent(tag)
            }
            entity_data_serializers::BOOLEAN => EntityDataEntry::Bool(read_u8(&mut c)? != 0),
            entity_data_serializers::ROTATIONS => EntityDataEntry::Rotations([
                read_f32(&mut c)?,
                read_f32(&mut c)?,
                read_f32(&mut c)?,
            ]),
            entity_data_serializers::BLOCK_POS => {
                let (x, y, z) = decode_block_pos(read_i64(&mut c)?);
                EntityDataEntry::BlockPos([x, y, z])
            }
            entity_data_serializers::OPTIONAL_BLOCK_POS => {
                let present = read_u8(&mut c)? != 0;
                let pos = if present {
                    let (x, y, z) = decode_block_pos(read_i64(&mut c)?);
                    Some([x, y, z])
                } else {
                    None
                };
                EntityDataEntry::OptionalBlockPos(pos)
            }
            entity_data_serializers::DIRECTION => {
                // ByIdMap.continuous(.., WRAP): the map NEVER refuses
                // (out-of-range keys floorMod into [0, 6)); the wire
                // ordinal is retained verbatim
                let d = read_varint(&mut c)?;
                EntityDataEntry::Direction(d)
            }
            entity_data_serializers::OPTIONAL_LIVING_ENTITY_REFERENCE => {
                // EntityReference.STREAM_CODEC = UUIDUtil.STREAM_CODEC
                // .map(wrap-left, unpack-uuid): the wire carries ONLY
                // the UUID — 16 bytes, msb then lsb — and the optional
                // operation's flag byte rides first (the same
                // invokedynamic bsm BlockPos.optional rides). The
                // resolution is the client's act; the port retains
                // the UUID verbatim.
                let present = read_u8(&mut c)? != 0;
                let r = if present {
                    Some(read_entity_reference(&mut c)?)
                } else {
                    None
                };
                EntityDataEntry::OptionalLivingEntityReference(r)
            }
            entity_data_serializers::PARTICLE => {
                // ParticleTypes.STREAM_CODEC: the dispatched registry
                // walk — the id, then the type's own payload
                EntityDataEntry::Particle(read_particle_value(&mut c)?)
            }
            entity_data_serializers::PARTICLES => {
                // the list operation over the dispatch: the VarInt
                // count, then that many dispatched values (a negative
                // count is the readCount family's constructor death —
                // the port refuses panic-free at the same lane)
                let n = read_varint(&mut c)?;
                anyhow::ensure!(n >= 0, "negative particle list count {n}");
                let mut list = Vec::new();
                for _ in 0..n {
                    list.push(read_particle_value(&mut c)?);
                }
                EntityDataEntry::Particles(list)
            }
            entity_data_serializers::RESOLVABLE_PROFILE => {
                // the either's flag, the identity side, the patch
                let (profile, patch) = read_resolvable_profile(&mut c)?;
                EntityDataEntry::ResolvableProfile { profile, patch }
            }
            entity_data_serializers::BLOCK_STATE => {
                let s = read_varint(&mut c)?;
                anyhow::ensure!(s >= 0, "negative block state id {s}");
                EntityDataEntry::BlockState(s)
            }
            entity_data_serializers::OPTIONAL_BLOCK_STATE => {
                let present = read_u8(&mut c)? != 0;
                let s = if present {
                    let s = read_varint(&mut c)?;
                    anyhow::ensure!(s >= 0, "negative block state id {s}");
                    Some(s)
                } else {
                    None
                };
                EntityDataEntry::OptionalBlockState(s)
            }
            entity_data_serializers::OPTIONAL_UNSIGNED_INT => {
                // the anonymous codec's arithmetic verbatim: 0 is the
                // EMPTY sentinel, a present value arrives BIAS-BY-ONE
                // — and the arithmetic NEVER refuses (a negative wire
                // n is OptionalInt.of(n - 1), a NEGATIVE stored
                // value); wrapping_sub mirrors the isub's two's-
                // complement wrap (wire MIN_VALUE stores MAX_VALUE)
                let n = read_varint(&mut c)?;
                EntityDataEntry::OptionalUnsignedInt(if n == 0 {
                    None
                } else {
                    Some(n.wrapping_sub(1))
                })
            }
            entity_data_serializers::POSE => {
                // ByIdMap.continuous(.., ZERO): the map NEVER refuses
                // (out-of-range keys fill to values[0]); the wire
                // ordinal is retained verbatim
                let p = read_varint(&mut c)?;
                EntityDataEntry::Pose(p)
            }
            entity_data_serializers::VILLAGER_DATA => {
                // the composite's three VarInts: type, profession,
                // level. A negative registry id is by definition
                // outside every IdMap (byIdOrThrow throws on it —
                // the same direction as the DIRECTION/POSE arms),
                // while a non-negative unknown id stays honest to
                // retain (no villager registry tables client-side;
                // the consumer resolves or refuses). The level is
                // the record constructor's clamp: max(1, wire).
                let type_id = read_varint(&mut c)?;
                anyhow::ensure!(type_id >= 0, "negative villager type id {type_id}");
                let profession_id = read_varint(&mut c)?;
                anyhow::ensure!(
                    profession_id >= 0,
                    "negative villager profession id {profession_id}"
                );
                let level = read_varint(&mut c)?;
                EntityDataEntry::VillagerData {
                    type_id,
                    profession_id,
                    level: level.max(1),
                }
            }
            entity_data_serializers::ITEM_STACK => {
                // ItemStack$2: the OPTIONAL walk (read_item_stack) plus
                // the strict "Empty ItemStack not allowed" wrapper — the
                // SAME wire shape, only the empty case's verdict
                // differs (the entity-data lane refuses what the
                // equipment lane carries). The count is retained
                // verbatim, not normalized.
                let stack = read_item_stack(&mut c)?;
                EntityDataEntry::ItemStack {
                    count: stack.count,
                    item_id: stack.item_id,
                    patch: stack.patch,
                }
            }
            entity_data_serializers::CAT_VARIANT => {
                EntityDataEntry::CatVariant(read_holder_id(&mut c, "cat variant")?)
            }
            entity_data_serializers::CAT_SOUND_VARIANT => {
                EntityDataEntry::CatSoundVariant(read_holder_id(
                    &mut c,
                    "cat sound variant",
                )?)
            }
            entity_data_serializers::COW_VARIANT => {
                EntityDataEntry::CowVariant(read_holder_id(&mut c, "cow variant")?)
            }
            entity_data_serializers::COW_SOUND_VARIANT => {
                EntityDataEntry::CowSoundVariant(read_holder_id(
                    &mut c,
                    "cow sound variant",
                )?)
            }
            entity_data_serializers::WOLF_VARIANT => {
                EntityDataEntry::WolfVariant(read_holder_id(&mut c, "wolf variant")?)
            }
            entity_data_serializers::WOLF_SOUND_VARIANT => {
                EntityDataEntry::WolfSoundVariant(read_holder_id(
                    &mut c,
                    "wolf sound variant",
                )?)
            }
            entity_data_serializers::FROG_VARIANT => {
                EntityDataEntry::FrogVariant(read_holder_id(&mut c, "frog variant")?)
            }
            entity_data_serializers::PIG_VARIANT => {
                EntityDataEntry::PigVariant(read_holder_id(&mut c, "pig variant")?)
            }
            entity_data_serializers::PIG_SOUND_VARIANT => {
                EntityDataEntry::PigSoundVariant(read_holder_id(
                    &mut c,
                    "pig sound variant",
                )?)
            }
            entity_data_serializers::CHICKEN_VARIANT => {
                EntityDataEntry::ChickenVariant(read_holder_id(&mut c, "chicken variant")?)
            }
            entity_data_serializers::CHICKEN_SOUND_VARIANT => {
                EntityDataEntry::ChickenSoundVariant(read_holder_id(
                    &mut c,
                    "chicken sound variant",
                )?)
            }
            entity_data_serializers::ZOMBIE_NAUTILUS_VARIANT => {
                EntityDataEntry::ZombieNautilusVariant(read_holder_id(
                    &mut c,
                    "zombie nautilus variant",
                )?)
            }
            entity_data_serializers::OPTIONAL_GLOBAL_POS => {
                // the #149-bsm optional operation: the flag byte,
                // then the GlobalPos composite only if present —
                // the dimension's identifier STRING (the
                // ResourceKey.streamCodec map, STRING_UTF8's 32767
                // cap) and the packed position long
                let present = read_u8(&mut c)? != 0;
                let pos = if present {
                    let dimension = read_capped_string(&mut c, 32767)?;
                    let packed = read_i64(&mut c)?;
                    let (x, y, z) = decode_block_pos(packed);
                    Some(GlobalPos { dimension, pos: [x, y, z] })
                } else {
                    None
                };
                EntityDataEntry::OptionalGlobalPos(pos)
            }
            entity_data_serializers::PAINTING_VARIANT => {
                // the $30 holder: the ONLY bias-by-one member of the
                // family — n == 0 is the INLINE direct record, n >= 1
                // is byIdOrThrow(n - 1), n < 0 throws (the port
                // refuses at the same lane)
                let n = read_varint(&mut c)?;
                anyhow::ensure!(n >= 0, "negative painting variant tag {n}");
                let v = if n == 0 {
                    let width = read_varint(&mut c)?;
                    let height = read_varint(&mut c)?;
                    let asset_id = read_capped_string(&mut c, 32767)?;
                    // title and author: TRUSTED_OPTIONAL_STREAM_CODEC
                    // = the flag byte, then the bare NBT tag only if
                    // present (the OPTIONAL_COMPONENT shape)
                    let title = match read_u8(&mut c)? {
                        0 => None,
                        _ => Some(read_bare_nbt(&mut c)?),
                    };
                    let author = match read_u8(&mut c)? {
                        0 => None,
                        _ => Some(read_bare_nbt(&mut c)?),
                    };
                    PaintingVariant::Direct {
                        width,
                        height,
                        asset_id,
                        title,
                        author,
                    }
                } else {
                    PaintingVariant::Registry(n - 1)
                };
                EntityDataEntry::PaintingVariant(v)
            }
            entity_data_serializers::SNIFFER_STATE => {
                // ByIdMap.continuous(.., ZERO): the map NEVER refuses
                // (out-of-range keys fill to IDLING in the consumer's
                // resolution); the wire ordinal is retained verbatim
                let st = read_varint(&mut c)?;
                EntityDataEntry::SnifferState(st)
            }
            entity_data_serializers::ARMADILLO_STATE => {
                // the ZERO lane again (IDLE is values[0])
                let st = read_varint(&mut c)?;
                EntityDataEntry::ArmadilloState(st)
            }
            entity_data_serializers::COPPER_GOLEM_STATE => {
                // the ZERO lane again (IDLE is values[0])
                let st = read_varint(&mut c)?;
                EntityDataEntry::CopperGolemState(st)
            }
            entity_data_serializers::WEATHERING_COPPER_STATE => {
                // the family's CLAMP member: vanilla's decode map
                // clamps out-of-range keys into [0, 3]; the port
                // retains the wire ordinal verbatim and the clamp is
                // the consumer's act (the DIRECTION/POSE line)
                let st = read_varint(&mut c)?;
                EntityDataEntry::WeatheringCopperState(st)
            }
            entity_data_serializers::VECTOR3 => {
                // readVector3f: three big-endian f32s (x, y, z)
                EntityDataEntry::Vector3([
                    read_f32(&mut c)?,
                    read_f32(&mut c)?,
                    read_f32(&mut c)?,
                ])
            }
            entity_data_serializers::QUATERNION => {
                // readQuaternion: four big-endian f32s in the JOML
                // Quaternionf(float, float, float, float) order x, y,
                // z, w
                EntityDataEntry::Quaternion([
                    read_f32(&mut c)?,
                    read_f32(&mut c)?,
                    read_f32(&mut c)?,
                    read_f32(&mut c)?,
                ])
            }
            entity_data_serializers::HUMANOID_ARM => {
                // the ZERO lane: LEFT = 0, RIGHT = 1 (the
                // declaration order's own truth); out-of-range fills
                // to LEFT in the consumer's resolution
                let arm = read_varint(&mut c)?;
                EntityDataEntry::HumanoidArm(arm)
            }
            other => anyhow::bail!("unsupported entity data serializer {other}"),
        };
        entries.push(SetEntityDataEntry {
            index,
            serializer,
            value,
        });
    }
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the set entity data packet",
        c.len()
    );
    Ok(EntityDataSet {
        id: entity_id,
        entries,
    })
}

// ---------------------------------------------------------------------------
// the write side — the encode mirrors of the audited decode walks
// ---------------------------------------------------------------------------
//
// Every writer below is the mirror of a decode walk the javap audits
// already pinned: `StreamCodec.composite` is symmetric by construction,
// so the mirror's only JUDGMENT calls are the lanes where vanilla's own
// encode is NOT the decode's inverse — the OptionalInt's wrapping +1
// (EntityDataSerializers$3.encode is `optional.orElse(-1) + 1`, the
// iadd's two's-complement wrap), the item stack's count sentinel
// (ItemStack$1.encode collapses an empty store to the VarInt 0 and
// writes NOTHING else — while the port's store RETAINS a negative
// sentinel verbatim, the wire's truth beating the object graph's
// collapse), and the painting/holder-set/sound tags' bias arithmetic.
// The writer writes the STORE; a store shape vanilla could not hold
// (a display ordinal that filled, a level that clamped) re-encodes to
// its canonical form — exactly the round-trip vanilla's own object
// graph performs.

/// The block position's pack — [`decode_block_pos`]'s exact inverse
/// (`BlockPos.asLong`): x in bits 38+, z in 12..38, y in 0..12, the
/// 26/12/26-bit masks making the two's-complement truncation the
/// getters' sign-extension inverse.
fn pack_block_pos(x: i32, y: i32, z: i32) -> i64 {
    (((x as i64) & 0x3FF_FFFF) << 38) | (((z as i64) & 0x3FF_FFFF) << 12) | ((y as i64) & 0xFFF)
}

fn write_bool(out: &mut Vec<u8>, v: bool) {
    out.push(v as u8);
}

fn write_i8(out: &mut Vec<u8>, v: i8) {
    out.push(v as u8);
}

fn write_i16(out: &mut Vec<u8>, v: i16) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn write_f32(out: &mut Vec<u8>, v: f32) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn write_f64(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn write_i32(out: &mut Vec<u8>, v: i32) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn write_i64(out: &mut Vec<u8>, v: i64) {
    out.extend_from_slice(&v.to_be_bytes());
}

/// The UUID's 16 bytes: msb long then lsb long, both big-endian — the
/// reference lane and the profile lane share the ONE codec.
fn write_uuid(out: &mut Vec<u8>, uuid: &[u8; 16]) {
    out.extend_from_slice(uuid);
}

fn write_entity_reference(out: &mut Vec<u8>, r: &EntityReference) {
    write_uuid(out, &r.uuid);
}

fn write_position_source(out: &mut Vec<u8>, s: &PositionSource) {
    match s {
        PositionSource::Block(p) => {
            write_varint(out, 0);
            write_i64(out, pack_block_pos(p[0], p[1], p[2]));
        }
        PositionSource::Entity {
            entity_id,
            y_offset,
        } => {
            write_varint(out, 1);
            write_varint(out, *entity_id);
            write_f32(out, *y_offset);
        }
    }
}

fn write_particle_value(out: &mut Vec<u8>, v: &ParticleValue) -> Result<()> {
    match v {
        ParticleValue::Simple(id) => write_varint(out, *id),
        ParticleValue::Block {
            particle_id,
            state,
        } => {
            write_varint(out, *particle_id);
            write_varint(out, *state);
        }
        ParticleValue::Geyser { water_blocks } => {
            write_varint(out, particle_types::GEYSER);
            write_i32(out, *water_blocks);
        }
        ParticleValue::GeyserBase {
            particle_id,
            water_blocks,
            burst_impulse_base,
        } => {
            write_varint(out, *particle_id);
            write_i32(out, *water_blocks);
            write_f32(out, *burst_impulse_base);
        }
        ParticleValue::Power(power) => {
            write_varint(out, particle_types::DRAGON_BREATH);
            write_f32(out, *power);
        }
        ParticleValue::Dust { color, scale } => {
            write_varint(out, particle_types::DUST);
            write_i32(out, *color);
            write_f32(out, *scale);
        }
        ParticleValue::DustTransition {
            from_color,
            to_color,
            scale,
        } => {
            write_varint(out, particle_types::DUST_COLOR_TRANSITION);
            write_i32(out, *from_color);
            write_i32(out, *to_color);
            write_f32(out, *scale);
        }
        ParticleValue::Spell {
            particle_id,
            color,
            power,
        } => {
            write_varint(out, *particle_id);
            write_i32(out, *color);
            write_f32(out, *power);
        }
        ParticleValue::Color { particle_id, color } => {
            write_varint(out, *particle_id);
            write_i32(out, *color);
        }
        ParticleValue::Item {
            item_id,
            count,
            patch,
        } => {
            write_varint(out, particle_types::ITEM);
            write_varint(out, *item_id);
            write_varint(out, *count);
            write_component_patch(out, patch)?;
        }
        ParticleValue::Vibration {
            source,
            arrival_in_ticks,
        } => {
            write_varint(out, particle_types::VIBRATION);
            write_position_source(out, source);
            write_varint(out, *arrival_in_ticks);
        }
        ParticleValue::Trail {
            target,
            color,
            duration,
        } => {
            write_varint(out, particle_types::TRAIL);
            write_f64(out, target[0]);
            write_f64(out, target[1]);
            write_f64(out, target[2]);
            write_i32(out, *color);
            write_varint(out, *duration);
        }
        ParticleValue::Shriek(delay) => {
            write_varint(out, particle_types::SHRIEK);
            write_varint(out, *delay);
        }
        ParticleValue::SculkCharge(roll) => {
            write_varint(out, particle_types::SCULK_CHARGE);
            write_f32(out, *roll);
        }
    }
    Ok(())
}

fn write_nullable_capped_string(out: &mut Vec<u8>, s: &Option<String>) {
    match s {
        None => write_bool(out, false),
        Some(v) => {
            write_bool(out, true);
            write_string(out, v);
        }
    }
}

fn write_profile_properties(out: &mut Vec<u8>, props: &[ProfileProperty]) {
    write_varint(out, props.len() as i32);
    for p in props {
        write_string(out, &p.name);
        write_string(out, &p.value);
        write_nullable_capped_string(out, &p.signature);
    }
}

fn write_optional_skin_asset(out: &mut Vec<u8>, a: &Option<SkinAsset>) {
    match a {
        None => write_bool(out, false),
        Some(s) => {
            write_bool(out, true);
            write_string(out, &s.id);
            write_string(out, &s.texture_path);
        }
    }
}

fn write_resolvable_profile(
    out: &mut Vec<u8>,
    profile: &ResolvableProfile,
    patch: &PlayerSkinPatch,
) {
    match profile {
        ResolvableProfile::Full {
            uuid,
            name,
            properties,
        } => {
            write_bool(out, true);
            write_uuid(out, uuid);
            write_string(out, name);
            write_profile_properties(out, properties);
        }
        ResolvableProfile::Partial {
            name,
            id,
            properties,
        } => {
            write_bool(out, false);
            write_nullable_capped_string(out, name);
            match id {
                None => write_bool(out, false),
                Some(u) => {
                    write_bool(out, true);
                    write_uuid(out, u);
                }
            }
            write_profile_properties(out, properties);
        }
    }
    write_optional_skin_asset(out, &patch.body);
    write_optional_skin_asset(out, &patch.cape);
    write_optional_skin_asset(out, &patch.elytra);
    match patch.model_slim {
        None => write_bool(out, false),
        Some(slim) => {
            write_bool(out, true);
            write_bool(out, slim);
        }
    }
}

/// The ItemStack$1 encode verbatim: the count rides FIRST and a
/// non-positive count is the WHOLE payload (an empty store carries no
/// item and no patch). The judgment call the javap pinned: vanilla's
/// `isEmpty()` branch writes the VarInt 0 — an object-graph collapse —
/// while this store RETAINS a negative sentinel verbatim (the wire's
/// -7 is a stored -7), so the mirror writes the retained count and
/// the round-trip keeps the wire's truth. The strict entity-data lane
/// (ItemStack$2) throws an EncoderException on an empty WRITE; this
/// port's decode retained the sentinel instead of refusing, so the
/// writer mirrors the WIRE, not the refusal vanilla's object graph
/// could never reach.
fn write_item_stack(out: &mut Vec<u8>, count: i32, item_id: i32, patch: &ComponentPatch) -> Result<()> {
    write_varint(out, count);
    if count > 0 {
        write_varint(out, item_id);
        write_component_patch(out, patch)?;
    }
    Ok(())
}

fn write_component_patch(out: &mut Vec<u8>, p: &ComponentPatch) -> Result<()> {
    write_varint(out, p.added.len() as i32);
    write_varint(out, p.removed.len() as i32);
    for (type_id, value) in &p.added {
        write_varint(out, *type_id);
        write_component_value(out, *type_id, value)?;
    }
    for type_id in &p.removed {
        write_varint(out, *type_id);
    }
    Ok(())
}

/// The component value's writer — the read dispatch's mirror, the
/// pairing (type id, value) matched as a TUPLE so a store that pairs a
/// type with the wrong value shape refuses instead of fabricating
/// bytes (the wire is unframed; the pairing is the store's invariant,
/// vanilla's TypedDataComponent holds it by construction).
fn write_component_value(out: &mut Vec<u8>, type_id: i32, v: &ComponentValue) -> Result<()> {
    use data_component_types as t;
    match (type_id, v) {
        (t::CUSTOM_DATA, ComponentValue::CompoundTag(tag)) => write_bare_nbt(tag, out)?,
        (t::MAX_STACK_SIZE | t::MAX_DAMAGE | t::DAMAGE | t::REPAIR_COST, ComponentValue::VarInt(n)) => {
            write_varint(out, *n)
        }
        // StreamCodec.unit: the value consumes ZERO bytes — the type's
        // presence is the payload
        (t::UNBREAKABLE | t::CREATIVE_SLOT_LOCK, ComponentValue::Unit) => {}
        (t::CUSTOM_NAME | t::ITEM_NAME, ComponentValue::Component(tag)) => write_bare_nbt(tag, out)?,
        (t::MINIMUM_ATTACK_CHARGE, ComponentValue::Float(f)) => write_f32(out, *f),
        (t::ITEM_MODEL | t::TOOLTIP_STYLE | t::NOTE_BLOCK_SOUND, ComponentValue::String(s)) => {
            write_string(out, s)
        }
        (t::LORE, ComponentValue::Lore(tags)) => {
            write_varint(out, tags.len() as i32);
            for tag in tags {
                write_bare_nbt(tag, out)?;
            }
        }
        (t::RARITY, ComponentValue::Rarity(r)) => write_varint(out, *r),
        (t::ENCHANTMENT_GLINT_OVERRIDE, ComponentValue::Bool(b)) => write_bool(out, *b),
        (t::ENCHANTMENTS, ComponentValue::Enchantments(entries)) => {
            write_varint(out, entries.len() as i32);
            for (id, level) in entries {
                write_varint(out, *id);
                write_varint(out, *level);
            }
        }
        (t::CUSTOM_MODEL_DATA, ComponentValue::CustomModelData { floats, flags, strings, colors }) => {
            write_varint(out, floats.len() as i32);
            for f in floats {
                write_f32(out, *f);
            }
            write_varint(out, flags.len() as i32);
            for b in flags {
                write_bool(out, *b);
            }
            write_varint(out, strings.len() as i32);
            for s in strings {
                write_string(out, s);
            }
            // the colors ride BIG-ENDIAN i32s (ByteBufCodecs.INT), not
            // VarInts
            write_varint(out, colors.len() as i32);
            for c in colors {
                write_i32(out, *c);
            }
        }
        (t::TOOLTIP_DISPLAY, ComponentValue::TooltipDisplay { hide_tooltip, hidden }) => {
            write_bool(out, *hide_tooltip);
            write_varint(out, hidden.len() as i32);
            for id in hidden {
                write_varint(out, *id);
            }
        }
        (t::CAN_PLACE_ON | t::CAN_BREAK, ComponentValue::BlockPredicates(list)) => {
            write_varint(out, list.len() as i32);
            for p in list {
                write_block_predicate(out, p)?;
            }
        }
        (t::ATTRIBUTE_MODIFIERS, ComponentValue::AttributeModifiers(entries)) => {
            write_attribute_modifiers(out, entries)?;
        }
        (t::FOOD, ComponentValue::Food { nutrition, saturation, can_always_eat }) => {
            write_varint(out, *nutrition);
            write_f32(out, *saturation);
            write_bool(out, *can_always_eat);
        }
        (t::CONSUMABLE, ComponentValue::Consumable { consume_seconds, animation, sound, has_consume_particles, effects }) => {
            write_f32(out, *consume_seconds);
            write_varint(out, *animation);
            write_consumable_sound(out, sound);
            write_bool(out, *has_consume_particles);
            write_varint(out, effects.len() as i32);
            for e in effects {
                write_consume_effect(out, e)?;
            }
        }
        _ => anyhow::bail!(
            "no value writer pairs component type {type_id} with {v:?} (the patch wire \
             is unframed — the pairing is the store's invariant)"
        ),
    }
    Ok(())
}

fn write_block_set(out: &mut Vec<u8>, s: &BlockSet) {
    match s {
        BlockSet::Named(tag) => {
            write_varint(out, 0);
            write_string(out, tag);
        }
        BlockSet::Direct(ids) => {
            write_varint(out, ids.len() as i32 + 1);
            for id in ids {
                write_varint(out, *id);
            }
        }
    }
}

fn write_optional_string(out: &mut Vec<u8>, s: &Option<String>) {
    match s {
        None => write_bool(out, false),
        Some(v) => {
            write_bool(out, true);
            write_string(out, v);
        }
    }
}

fn write_property_matcher(out: &mut Vec<u8>, m: &PropertyMatcher) {
    write_string(out, &m.name);
    match &m.value {
        PropertyValue::Exact(v) => {
            // the either's ONE-BYTE discriminator: true = LEFT
            write_bool(out, true);
            write_string(out, v);
        }
        PropertyValue::Ranged { min, max } => {
            write_bool(out, false);
            write_optional_string(out, min);
            write_optional_string(out, max);
        }
    }
}

fn write_predicate_type_lane(out: &mut Vec<u8>, lane: &PredicateTypeLane) {
    match lane {
        PredicateTypeLane::PredicateType(id) => {
            write_bool(out, true);
            write_varint(out, *id);
        }
        PredicateTypeLane::ComponentType(id) => {
            write_bool(out, false);
            write_varint(out, *id);
        }
    }
}

fn write_block_predicate(out: &mut Vec<u8>, p: &BlockPredicate) -> Result<()> {
    // lane 1: optional(holderSet(BLOCK))
    match &p.blocks {
        None => write_bool(out, false),
        Some(s) => {
            write_bool(out, true);
            write_block_set(out, s);
        }
    }
    // lane 2: optional(StatePropertiesPredicate)
    match &p.properties {
        None => write_bool(out, false),
        Some(props) => {
            write_bool(out, true);
            write_varint(out, props.len() as i32);
            for m in props {
                write_property_matcher(out, m);
            }
        }
    }
    // lane 3: optional(NbtPredicate)
    match &p.nbt {
        None => write_bool(out, false),
        Some(tag) => {
            write_bool(out, true);
            write_bare_nbt(tag, out)?;
        }
    }
    // lane 4: DataComponentMatchers — the exact list rides the patch's
    // added-entry walk, the partial list rides the dispatched type lane
    // plus the ONE self-delimiting NBT tag
    write_varint(out, p.matchers.exact.len() as i32);
    for (type_id, value) in &p.matchers.exact {
        write_varint(out, *type_id);
        write_component_value(out, *type_id, value)?;
    }
    write_varint(out, p.matchers.partial.len() as i32);
    for e in &p.matchers.partial {
        write_predicate_type_lane(out, &e.predicate_type);
        write_bare_nbt(&e.payload, out)?;
    }
    Ok(())
}

fn write_attribute_modifiers(out: &mut Vec<u8>, entries: &[AttributeModifierEntry]) -> Result<()> {
    write_varint(out, entries.len() as i32);
    for e in entries {
        write_varint(out, e.attribute_id);
        write_string(out, &e.modifier.id);
        // the amount rides a BIG-ENDIAN f64 (ByteBufCodecs.DOUBLE —
        // the construction site is the shape)
        write_f64(out, e.modifier.amount);
        write_varint(out, e.modifier.operation);
        write_varint(out, e.slot);
        match &e.display {
            // the store IS the canonical form vanilla's object graph
            // holds — a wire ordinal that filled to Default re-encodes
            // as 0, the same collapse a vanilla round-trip performs
            AttributeDisplay::Default => write_varint(out, 0),
            AttributeDisplay::Hidden => write_varint(out, 1),
            AttributeDisplay::Override(tag) => {
                write_varint(out, 2);
                write_bare_nbt(tag, out)?;
            }
        }
    }
    Ok(())
}

fn write_consumable_sound(out: &mut Vec<u8>, s: &ConsumableSound) {
    match s {
        ConsumableSound::Direct { id, range } => {
            write_varint(out, 0);
            write_string(out, id);
            write_f32(out, *range);
        }
        ConsumableSound::Registry(n) => write_varint(out, n + 1),
    }
}

fn write_effect_holder_set(out: &mut Vec<u8>, h: &EffectHolderSet) {
    match h {
        EffectHolderSet::Named(tag) => {
            write_varint(out, 0);
            write_string(out, tag);
        }
        EffectHolderSet::Direct(ids) => {
            write_varint(out, ids.len() as i32 + 1);
            for id in ids {
                write_varint(out, *id);
            }
        }
    }
}

fn write_effect_details(out: &mut Vec<u8>, d: &EffectDetails) -> Result<()> {
    write_varint(out, d.amplifier);
    write_varint(out, d.duration);
    write_bool(out, d.ambient);
    write_bool(out, d.show_particles);
    write_bool(out, d.show_icon);
    match &d.hidden_effect {
        None => write_bool(out, false),
        Some(hidden) => {
            write_bool(out, true);
            write_effect_details(out, hidden)?;
        }
    }
    Ok(())
}

fn write_effect_instance(out: &mut Vec<u8>, e: &EffectInstance) -> Result<()> {
    write_varint(out, e.effect_id);
    write_effect_details(out, &e.details)
}

fn write_consume_effect(out: &mut Vec<u8>, e: &ConsumeEffect) -> Result<()> {
    match e {
        ConsumeEffect::ApplyEffects {
            effects,
            probability,
        } => {
            write_varint(out, 0);
            write_varint(out, effects.len() as i32);
            for eff in effects {
                write_effect_instance(out, eff)?;
            }
            write_f32(out, *probability);
        }
        ConsumeEffect::RemoveEffects(hs) => {
            write_varint(out, 1);
            write_effect_holder_set(out, hs);
        }
        // StreamCodec.unit: ZERO payload bytes
        ConsumeEffect::ClearAllEffects => write_varint(out, 2),
        ConsumeEffect::TeleportRandomly { diameter } => {
            write_varint(out, 3);
            write_f32(out, *diameter);
        }
        ConsumeEffect::PlaySound(s) => {
            write_varint(out, 4);
            write_consumable_sound(out, s);
        }
    }
    Ok(())
}

/// One metadata VALUE's write mirror — the read dispatch's arm-for-arm
/// inverse (the serializer id is NOT the value's business: it rides
/// the entry, and the pairing is the store's invariant).
fn write_entity_data_value(out: &mut Vec<u8>, value: &EntityDataEntry) -> Result<()> {
    match value {
        EntityDataEntry::Byte(v) => write_i8(out, *v),
        EntityDataEntry::Int(v) => write_varint(out, *v),
        EntityDataEntry::Long(v) => write_varlong(out, *v),
        EntityDataEntry::Float(v) => write_f32(out, *v),
        EntityDataEntry::String(s) => write_string(out, s),
        EntityDataEntry::Component(tag) => write_bare_nbt(tag, out)?,
        EntityDataEntry::OptionalComponent(tag) => match tag {
            None => write_bool(out, false),
            Some(t) => {
                write_bool(out, true);
                write_bare_nbt(t, out)?;
            }
        },
        EntityDataEntry::Bool(v) => write_bool(out, *v),
        EntityDataEntry::Rotations(r) => {
            write_f32(out, r[0]);
            write_f32(out, r[1]);
            write_f32(out, r[2]);
        }
        EntityDataEntry::BlockPos(p) => write_i64(out, pack_block_pos(p[0], p[1], p[2])),
        EntityDataEntry::OptionalBlockPos(p) => match p {
            None => write_bool(out, false),
            Some(p) => {
                write_bool(out, true);
                write_i64(out, pack_block_pos(p[0], p[1], p[2]));
            }
        },
        EntityDataEntry::Direction(d) => write_varint(out, *d),
        EntityDataEntry::OptionalLivingEntityReference(r) => match r {
            None => write_bool(out, false),
            Some(r) => {
                write_bool(out, true);
                write_entity_reference(out, r);
            }
        },
        EntityDataEntry::BlockState(s) => write_varint(out, *s),
        EntityDataEntry::OptionalBlockState(s) => match s {
            None => write_bool(out, false),
            Some(s) => {
                write_bool(out, true);
                write_varint(out, *s);
            }
        },
        EntityDataEntry::Particle(v) => write_particle_value(out, v)?,
        EntityDataEntry::Particles(list) => {
            write_varint(out, list.len() as i32);
            for v in list {
                write_particle_value(out, v)?;
            }
        }
        EntityDataEntry::ResolvableProfile { profile, patch } => {
            write_resolvable_profile(out, profile, patch)
        }
        // THE AUDIT'S ARITHMETIC: EntityDataSerializers$3.encode is
        // `optional.orElse(-1) + 1` — the iadd is Java's WRAPPING int
        // add, so None rides 0 and Some(MAX_VALUE) rides MIN_VALUE;
        // the decode's wrapping_sub(1) is the exact inverse on both
        // ends.
        EntityDataEntry::OptionalUnsignedInt(v) => {
            write_varint(out, v.unwrap_or(-1).wrapping_add(1))
        }
        EntityDataEntry::Pose(p) => write_varint(out, *p),
        EntityDataEntry::VillagerData {
            type_id,
            profession_id,
            level,
        } => {
            write_varint(out, *type_id);
            write_varint(out, *profession_id);
            // the store's level is the record constructor's clamp
            // already applied — the writer writes the STORE (a wire 0
            // clamped to 1 re-encodes as 1, vanilla's own round-trip)
            write_varint(out, *level);
        }
        EntityDataEntry::ItemStack {
            count,
            item_id,
            patch,
        } => write_item_stack(out, *count, *item_id, patch)?,
        EntityDataEntry::CatVariant(v)
        | EntityDataEntry::CatSoundVariant(v)
        | EntityDataEntry::CowVariant(v)
        | EntityDataEntry::CowSoundVariant(v)
        | EntityDataEntry::WolfVariant(v)
        | EntityDataEntry::WolfSoundVariant(v)
        | EntityDataEntry::FrogVariant(v)
        | EntityDataEntry::PigVariant(v)
        | EntityDataEntry::PigSoundVariant(v)
        | EntityDataEntry::ChickenVariant(v)
        | EntityDataEntry::ChickenSoundVariant(v)
        | EntityDataEntry::ZombieNautilusVariant(v) => write_varint(out, *v),
        EntityDataEntry::OptionalGlobalPos(pos) => match pos {
            None => write_bool(out, false),
            Some(g) => {
                write_bool(out, true);
                write_string(out, &g.dimension);
                write_i64(out, pack_block_pos(g.pos[0], g.pos[1], g.pos[2]));
            }
        },
        EntityDataEntry::PaintingVariant(v) => match v {
            // the family's ONLY bias-by-one member: the store's n is
            // the registry id (wire n + 1), the direct record rides
            // wire 0
            PaintingVariant::Registry(n) => write_varint(out, n + 1),
            PaintingVariant::Direct {
                width,
                height,
                asset_id,
                title,
                author,
            } => {
                write_varint(out, 0);
                write_varint(out, *width);
                write_varint(out, *height);
                write_string(out, asset_id);
                match title {
                    None => write_bool(out, false),
                    Some(t) => {
                        write_bool(out, true);
                        write_bare_nbt(t, out)?;
                    }
                }
                match author {
                    None => write_bool(out, false),
                    Some(t) => {
                        write_bool(out, true);
                        write_bare_nbt(t, out)?;
                    }
                }
            }
        },
        EntityDataEntry::SnifferState(st) => write_varint(out, *st),
        EntityDataEntry::ArmadilloState(st) => write_varint(out, *st),
        EntityDataEntry::CopperGolemState(st) => write_varint(out, *st),
        EntityDataEntry::WeatheringCopperState(st) => write_varint(out, *st),
        EntityDataEntry::Vector3(v) => {
            write_f32(out, v[0]);
            write_f32(out, v[1]);
            write_f32(out, v[2]);
        }
        EntityDataEntry::Quaternion(q) => {
            // the JOML x, y, z, w order
            write_f32(out, q[0]);
            write_f32(out, q[1]);
            write_f32(out, q[2]);
            write_f32(out, q[3]);
        }
        EntityDataEntry::HumanoidArm(a) => write_varint(out, *a),
    }
    Ok(())
}

/// The Set Entity Data packet's write mirror — the `unpack` walk's
/// pack: the VarInt packet id, the VarInt entity id, then each
/// entry's index byte + VarInt serializer id + the value's bytes, and
/// the 0xFF EOF marker. The serializer id rides the ENTRY (the
/// value/serializer pairing is the store's invariant — vanilla's
/// DataValue holds it the same way), and an index of 255 refuses: the
/// byte IS the EOF marker, a packet wearing it would truncate the
/// walk the port itself refuses to parse past.
pub fn encode_set_entity_data(entity_id: i32, entries: &[SetEntityDataEntry]) -> Result<Vec<u8>> {
    let mut p = Vec::new();
    write_varint(&mut p, clientbound::SET_ENTITY_DATA);
    write_varint(&mut p, entity_id);
    for e in entries {
        anyhow::ensure!(
            e.index != 255,
            "entity data entry index 255 is the EOF marker, not an accessor"
        );
        p.push(e.index);
        write_varint(&mut p, e.serializer);
        write_entity_data_value(&mut p, &e.value)
            .map_err(|err| anyhow::anyhow!("entity data entry {}: {err}", e.index))?;
    }
    p.push(255);
    Ok(p)
}

/// Set Equipment's write mirror (see [`EntityEquipment`]): the paired
/// walk written back — the slot byte packs `ordinal | (more ? 0x80 : 0)`
/// (the `more` is the ITERATOR's hasNext, so the LAST pair's byte always
/// has the high bit CLEAR — the decode loop's own exit condition,
/// `b & -128 == 0`), each stack riding the OPTIONAL walk's write
/// mirror. A zero-pair packet is impossible (the decode reads the first
/// slot byte unconditionally), so an empty list refuses. The writers
/// write the STORE: a retained negative sentinel re-encodes verbatim,
/// a cleared slot rides the optional lane's count-only shape.
pub fn encode_set_equipment(entity_id: i32, slots: &[EquipmentEntry]) -> Result<Vec<u8>> {
    anyhow::ensure!(
        !slots.is_empty(),
        "a set equipment packet with zero pairs cannot exist — the decode \
         reads the first slot byte unconditionally"
    );
    let mut p = Vec::new();
    write_varint(&mut p, clientbound::SET_EQUIPMENT);
    write_varint(&mut p, entity_id);
    let last = slots.len() - 1;
    for (i, e) in slots.iter().enumerate() {
        let more: u8 = if i < last { 0x80 } else { 0 };
        p.push(equipment_slot_ordinal(e.slot) | more);
        write_item_stack(&mut p, e.stack.count, e.stack.item_id, &e.stack.patch)?;
    }
    Ok(p)
}

/// Set Container Content's write mirror (see [`ContainerContent`]): the
/// four-lane composite written back — the VarInt container id (the
/// CONTAINER_ID lane), the raw state id, the collection count riding
/// the store's OWN length (the encode side cannot fabricate the
/// negative count the decode refuses — a Vec's len is never negative),
/// count × the OPTIONAL walk, the carried item last.
pub fn encode_set_container_content(
    container_id: i32,
    state_id: i32,
    items: &[ItemStack],
    carried_item: &ItemStack,
) -> Result<Vec<u8>> {
    let mut p = Vec::new();
    write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
    write_varint(&mut p, container_id);
    write_varint(&mut p, state_id);
    write_varint(&mut p, items.len() as i32);
    for it in items {
        write_item_stack(&mut p, it.count, it.item_id, &it.patch)?;
    }
    write_item_stack(&mut p, carried_item.count, carried_item.item_id, &carried_item.patch)?;
    Ok(p)
}

/// Set Container Slot's write mirror (see [`ContainerSetSlot`]): the
/// record's four fields written back — the VarInt container id, the raw
/// state id, the SIGNED BE SHORT slot (the readShort lane's own
/// mirror; a negative slot re-encodes verbatim — the store is the wire
/// again), and the plain OPTIONAL stack (an EMPTY stack is the legal
/// cleared slot).
pub fn encode_set_container_slot(
    container_id: i32,
    state_id: i32,
    slot: i16,
    item: &ItemStack,
) -> Result<Vec<u8>> {
    let mut p = Vec::new();
    write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
    write_varint(&mut p, container_id);
    write_varint(&mut p, state_id);
    write_i16(&mut p, slot);
    write_item_stack(&mut p, item.count, item.item_id, &item.patch)?;
    Ok(p)
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

/// One clock entry of the Set Time packet (javap of the real 26.2 record:
/// `ClockNetworkState(long totalTicks, float partialTick, float rate)`,
/// each field VAR_LONG/FLOAT/FLOAT on the wire). The map KEY is a
/// `Holder<WorldClock>` — `holderRegistry(WORLD_CLOCK)` — a VarInt
/// registry id on the wire; the id -> name mapping rides the
/// configuration Registry Data (the WORLD_CLOCK registry is synced).
#[derive(Debug, Clone, PartialEq)]
pub struct ClockUpdate {
    /// The WORLD_CLOCK registry id of the clock this state belongs to.
    pub id: i32,
    /// The clock's total ticks (the overworld clock's day tick is this
    /// mod the day timeline's 24000 period).
    pub total_ticks: i64,
    /// Sub-tick fraction for smooth client-side interpolation.
    pub partial_tick: f32,
    /// The clock's tick rate (a paused clock sends rate 0).
    pub rate: f32,
}

/// Set Time (clientbound play, generated id 113): the world's game time
/// plus a map of per-clock states. The 26.2 shape is a REAL protocol
/// break with the wiki-era `(gameTime, dayTime, doDaylightCycle)` triple
/// — javap of the actual record proves `LONG gameTime` followed by a
/// `ByteBufCodecs.map` of `WorldClock` holders to `ClockNetworkState`.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeSync {
    /// The world's monotonic game time (used for daytime mob-spawn and
    /// other mechanics; NOT the day/night clock).
    pub game_time: i64,
    /// One entry per clock the server is syncing (a vanilla overworld
    /// server sends its dimension's default clock).
    pub clocks: Vec<ClockUpdate>,
}

// ---------------------------------------------------------------------------
// the mob-effect state's wire transport
// ---------------------------------------------------------------------------

/// Update Mob Effect: ONE entity's mob-effect state upsert — the effect
/// state's wire transport, the player-state surface the lightmap's
/// night-vision factor consumes. javap of the 26.2 private decode
/// constructor (a flat five-field walk, NOT the item-component's
/// Details composite): `readVarInt` entity id, `MobEffect.STREAM_CODEC`
/// effect (the `ByteBufCodecs.holderRegistry(MOB_EFFECT)` family — the
/// variant holders' `$29` byIdOrThrow: a negative id throws),
/// `readVarInt` amplifier, `readVarInt` duration ticks, `readByte`
/// flags. The constructor's flag-packing `ior` chain pins the bits:
/// bit 0 AMBIENT, bit 1 VISIBLE, bit 2 SHOW_ICON, bit 3 BLEND.
#[derive(Debug, Clone, PartialEq)]
pub struct MobEffectUpdate {
    /// The entity the state rides (the model filters its own id — the
    /// lightmap consumes the LOCAL player's effects only).
    pub entity_id: i32,
    /// The mob-effect REGISTRY id (`BuiltInRegistries.MOB_EFFECT`
    /// order — the runtime boot pins minecraft:night_vision = 15 and
    /// minecraft:conduit_power = 28; the legacy 1-based wiki ids are
    /// NOT the wire, the holderRegistry ids are the 0-based
    /// registration order).
    pub effect_id: i32,
    /// The amplifier (the vanilla 0..=255 convention; the decode
    /// validates nothing — retained verbatim).
    pub amplifier: i32,
    /// The STATED duration in ticks — the client ticks it down locally
    /// and the fade-out flicker reads the remaining window; retained
    /// verbatim (the -1 infinite sentinel included, the never-refuse
    /// line: the consumer's act is separate). The flicker is the
    /// documented non-goal (same verdict as the torch shimmer).
    pub duration: i32,
    /// Bit 0 ambient, bit 1 visible, bit 2 show icon, bit 3 blend —
    /// the packet's own packing order, retained raw.
    pub flags: u8,
}

/// Remove Mob Effect: ONE entity's effect ended (the record's two
/// fields, javap: the same VAR_INT entity id + holderRegistry effect
/// lane the update packet rides).
#[derive(Debug, Clone, PartialEq)]
pub struct MobEffectRemove {
    pub entity_id: i32,
    /// The mob-effect registry id (see [`MobEffectUpdate::effect_id`]).
    pub effect_id: i32,
}

/// Update Mob Effect's wire walk (see [`MobEffectUpdate`] — the
/// negative effect id refuses, the byIdOrThrow direction; every other
/// field is retained verbatim, the decode validates nothing).
pub fn parse_update_mob_effect(payload: &[u8]) -> Result<MobEffectUpdate> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::UPDATE_MOB_EFFECT,
        "not an update mob effect packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let effect_id = read_holder_id(&mut c, "mob-effect")?;
    let amplifier = read_varint(&mut c)?;
    let duration = read_varint(&mut c)?;
    let flags = read_u8(&mut c)?;
    Ok(MobEffectUpdate { entity_id, effect_id, amplifier, duration, flags })
}

/// Remove Mob Effect's wire walk (see [`MobEffectRemove`]).
pub fn parse_remove_mob_effect(payload: &[u8]) -> Result<MobEffectRemove> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::REMOVE_MOB_EFFECT,
        "not a remove mob effect packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let effect_id = read_holder_id(&mut c, "mob-effect")?;
    Ok(MobEffectRemove { entity_id, effect_id })
}

// ---------------------------------------------------------------------------
// the equipment state's wire transport
// ---------------------------------------------------------------------------

/// The ItemStack$1 wire shape (the codec behind BOTH
/// `ItemStack.OPTIONAL_STREAM_CODEC` and the entity-data lane's
/// strict wrapper): the count IS the emptiness sentinel — `ifgt` on
/// the raw VarInt, so count <= 0 is the EMPTY stack and NOTHING else
/// rides the wire (the count is retained verbatim, not normalized; a
/// negative count is as empty as zero). A positive stack carries the
/// item registry id (the `ByteBufCodecs.holderRegistry(ITEM)` family —
/// the variant holders' `$29` byIdOrThrow, a negative refuses) and the
/// component patch (see parse_component_patch). The particle round's
/// ItemStackTemplate is the Sibling shape WITHOUT the sentinel — the
/// count lane is raw there.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemStack {
    /// The stack size (the emptiness sentinel itself — retained
    /// verbatim, the never-refuse line).
    pub count: i32,
    /// The item REGISTRY id (`BuiltInRegistries.ITEM` order; the
    /// decode validates nothing beyond the byIdOrThrow direction).
    pub item_id: i32,
    /// The component patch riding the stack.
    pub patch: ComponentPatch,
}

/// The ItemStack$1 walk shared by the entity-data ITEM_STACK lane and
/// the Set Equipment packet's slots (the one-codec-two-call-sites
/// line — read_uuid's precedent: one codec, one reader). The
/// entity-data lane's own codec (ItemStack$2) is THIS walk plus the
/// strict empty rejection — javap-proven, not a name guess.
fn read_item_stack(c: &mut &[u8]) -> Result<ItemStack> {
    let count = read_varint(c)?;
    if count <= 0 {
        return Ok(ItemStack { count, item_id: 0, patch: ComponentPatch::default() });
    }
    let item_id = read_varint(c)?;
    anyhow::ensure!(item_id >= 0, "negative item registry id {item_id}");
    let patch = parse_component_patch(c)?;
    Ok(ItemStack { count, item_id, patch })
}

/// The eight EquipmentSlot constants in DECLARATION order — the wire's
/// index table (javap clinit: `VALUES = List.of(values())`; the packet
/// decodes with `VALUES.get(b & 127)` and encodes with `ordinal()`).
/// 26.2 grew the animal-armor BODY and SADDLE slots past the wiki-era
/// six — the declaration order IS the wire, and the enum's separate
/// `id`/BY_ID/STREAM_CODEC lane (an idMapper used elsewhere, e.g. the
/// EquipmentSlotGroup) is NOT this packet's lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentSlot {
    Mainhand,
    Offhand,
    Feet,
    Legs,
    Chest,
    Head,
    Body,
    Saddle,
}

/// The slot byte's walk: the low 7 bits index the eight-entry VALUES
/// list (`b & 127` — the mask pins which bits are the slot); an index
/// past the list's end throws in Java (the List.get bounds check —
/// the honest refusal, and it fires BEFORE the pair's stack bytes are
/// consumed, so the port refuses at the same point); `b & 127` can
/// never go negative (unlike the holder lanes' byIdOrThrow direction).
fn read_equipment_slot(byte: u8) -> Result<EquipmentSlot> {
    let idx = (byte & 0x7F) as usize;
    match idx {
        0 => Ok(EquipmentSlot::Mainhand),
        1 => Ok(EquipmentSlot::Offhand),
        2 => Ok(EquipmentSlot::Feet),
        3 => Ok(EquipmentSlot::Legs),
        4 => Ok(EquipmentSlot::Chest),
        5 => Ok(EquipmentSlot::Head),
        6 => Ok(EquipmentSlot::Body),
        7 => Ok(EquipmentSlot::Saddle),
        _ => anyhow::bail!("equipment slot index {idx} is outside the eight-entry VALUES list"),
    }
}

/// The slot byte's WRITE mirror: the enum's declaration-order ordinal
/// IS the low 7 bits (javap clinit: the packet encodes with
/// `ordinal()`), and the continue bit is the CALLER's — Java's `more`
/// is the iterator's hasNext, not the entry's property.
fn equipment_slot_ordinal(slot: EquipmentSlot) -> u8 {
    match slot {
        EquipmentSlot::Mainhand => 0,
        EquipmentSlot::Offhand => 1,
        EquipmentSlot::Feet => 2,
        EquipmentSlot::Legs => 3,
        EquipmentSlot::Chest => 4,
        EquipmentSlot::Head => 5,
        EquipmentSlot::Body => 6,
        EquipmentSlot::Saddle => 7,
    }
}

/// ONE equipped slot's entry: the slot (the byte's low 7 bits) and the
/// stack riding it. An EMPTY stack is a LEGAL entry here — the packet
/// rides ItemStack.OPTIONAL_STREAM_CODEC directly, and the optional
/// lane exists precisely to carry the cleared slot (the entity-data
/// lane's strict ItemStack$2 wrapper rejects what this lane carries).
#[derive(Debug, Clone, PartialEq)]
pub struct EquipmentEntry {
    pub slot: EquipmentSlot,
    pub stack: ItemStack,
}

/// Set Equipment (clientbound play, generated id 102): ONE entity's
/// equipment snapshot — javap of the private decode constructor: a
/// `readVarInt` entity id, then the PAIRED walk — `readByte` (the low
/// 7 bits index the eight-entry VALUES list, the HIGH bit is the
/// continue flag), each pair's stack rides
/// `ItemStack.OPTIONAL_STREAM_CODEC`. The write mirror packs
/// `ordinal | (more ? 0x80 : 0)` — the last pair's byte always has the
/// high bit CLEAR (the loop's exit is `b & -128 == 0`), and a packet
/// with zero pairs is impossible (the walk reads the first byte
/// unconditionally).
#[derive(Debug, Clone, PartialEq)]
pub struct EntityEquipment {
    /// The entity whose gear changed (the model filters by id when it
    /// renders mobs; the decode retains every entry).
    pub entity_id: i32,
    /// The slot/stack pairs, in wire order.
    pub slots: Vec<EquipmentEntry>,
}

/// Set Equipment's wire walk (see [`EntityEquipment`] — the paired
/// loop, the continue bit, the bounds refusal before the stack bytes).
pub fn parse_set_equipment(payload: &[u8]) -> Result<EntityEquipment> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SET_EQUIPMENT,
        "not a set equipment packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let mut slots = Vec::new();
    loop {
        let byte = read_u8(&mut c)?;
        let slot = read_equipment_slot(byte)?;
        let stack = read_item_stack(&mut c)?;
        slots.push(EquipmentEntry { slot, stack });
        if byte & 0x80 == 0 {
            break;
        }
    }
    Ok(EntityEquipment { entity_id, slots })
}

/// Set Container Content (clientbound play, generated id 18): ONE
/// container window's full snapshot — the record's own fields
/// (`containerId`, `stateId`, `items`, `carriedItem`), javap of the
/// clinit's four-part `StreamCodec.composite`. THE AUDIT'S CATCH: the
/// container id rides `ByteBufCodecs.CONTAINER_ID` →
/// `FriendlyByteBuf.readContainerId` → `VarInt.read` — a VARINT, not
/// the wiki-era unsigned byte (one anonymous class, one hop, the
/// name-shaped trap again). The state id rides the raw VAR_INT,
/// retained verbatim. The items ride
/// `ItemStack.OPTIONAL_LIST_STREAM_CODEC` = the OPTIONAL walk +
/// `ByteBufCodecs.collection`: `readCount(buf, MAX_VALUE)` (a VarInt
/// count; `count > maxSize` throws — unreachable at MAX_VALUE, the
/// VarInt's own 32-bit ceiling is the real bound), the collection
/// pre-allocated at `min(count, 65536)` (a DoS guard on the
/// ALLOCATION, not the entries), then count × the OPTIONAL walk. A
/// NEGATIVE count passes readCount's comparison (never > maxSize) and
/// refuses at the collection CONSTRUCTION — `new ArrayList<>(-n)`,
/// "Illegal Capacity" — BEFORE any entry bytes are consumed; the port
/// refuses at the same point. The carried item (the GUI cursor stack)
/// rides the plain OPTIONAL walk, empty-legal like the equipment lane.
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerContent {
    /// The window id (0 = the player inventory) — a VarInt, the
    /// CONTAINER_ID lane's own shape.
    pub container_id: i32,
    /// The revision counter the server bumps per change (the
    /// client's state-echo uses it); retained verbatim.
    pub state_id: i32,
    /// Every slot's stack, in window order — an EMPTY stack is a
    /// LEGAL entry (the cleared slot, the optional lane's own cargo).
    pub items: Vec<ItemStack>,
    /// The cursor stack riding the GUI mouse; the same optional lane.
    pub carried_item: ItemStack,
}

/// Set Container Content's wire walk (see [`ContainerContent`] — the
/// four-part composite, the VarInt container id, the readCount
/// validation, the Illegal-Capacity refusal ordering).
pub fn parse_set_container_content(payload: &[u8]) -> Result<ContainerContent> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::CONTAINER_SET_CONTENT,
        "not a container set content packet (0x{id:02X})"
    );
    let container_id = read_varint(&mut c)?;
    let state_id = read_varint(&mut c)?;
    let count = read_varint(&mut c)?;
    anyhow::ensure!(
        count >= 0,
        "collection count {count} is negative — the vanilla collection \
         constructor refuses (Illegal Capacity) before any entry is read"
    );
    let mut items = Vec::new();
    for _ in 0..count {
        items.push(read_item_stack(&mut c)?);
    }
    let carried_item = read_item_stack(&mut c)?;
    Ok(ContainerContent { container_id, state_id, items, carried_item })
}

/// Set Container Slot (clientbound play, generated id 20 —
/// `minecraft:container_set_slot`): ONE slot's new stack — the
/// inventory surfaces' third packet, the single-slot sibling of
/// [`ContainerContent`]. javap of the private decode constructor: the
/// record's own four fields in wire order — `containerId` rides
/// `readContainerId` (the VARINT lane the content round pinned; the
/// wiki-era byte would die past 127), `stateId` the raw VAR_INT
/// retained verbatim, `slot` rides `readShort` — A SIGNED
/// BIG-ENDIAN SHORT, not a VarInt (the round's shape trap: the
/// sibling packet has no slot lane at all, and a VarInt guess would
/// misread the width for every slot and die on the high ones; the
/// slot is retained verbatim — a negative slot stores a negative
/// slot, the consumer's bounds check is the consumer's act), and
/// `itemStack` the plain `ItemStack.OPTIONAL_STREAM_CODEC` — the
/// one-reader line's FOURTH call site, an EMPTY stack a LEGAL entry
/// (the cleared slot, the optional lane's own cargo). The
/// constructor's `ItemStack.copy()` is object-graph hygiene, not wire.
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerSetSlot {
    /// The window id (0 = the player inventory) — the VarInt
    /// CONTAINER_ID lane's own shape.
    pub container_id: i32,
    /// The revision counter the server bumps per change; retained
    /// verbatim.
    pub state_id: i32,
    /// The slot's index — a SIGNED BE SHORT (readShort), retained
    /// verbatim.
    pub slot: i16,
    /// The slot's new stack — an EMPTY stack clears the slot (the
    /// optional lane's own cargo).
    pub item: ItemStack,
}

/// Set Container Slot's wire walk (see [`ContainerSetSlot`] — the
/// four-lane walk, the readShort slot trap, the optional stack).
pub fn parse_set_container_slot(payload: &[u8]) -> Result<ContainerSetSlot> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::CONTAINER_SET_SLOT,
        "not a container set slot packet (0x{id:02X})"
    );
    let container_id = read_varint(&mut c)?;
    let state_id = read_varint(&mut c)?;
    let slot = read_i16(&mut c)?;
    let item = read_item_stack(&mut c)?;
    Ok(ContainerSetSlot { container_id, state_id, slot, item })
}

/// Set Held Slot (clientbound play, generated id 105 —
/// `minecraft:set_held_slot`): the hotbar's SELECTED slot — the
/// number-keys row's other half (the slot CONTENTS ride the container
/// packets' window-0 lane; the selection rides here). javap of the
/// record's clinit: `StreamCodec.composite(ByteBufCodecs.VAR_INT, ...)`
/// — ONE lane, the raw VAR_INT slot (the wiki-era byte table would
/// misshape here; the name-shaped trap class again). THE CONSUMER'S
/// GATE (javap of `ClientPacketListener.handleSetHeldSlot`):
/// `Inventory.isHotbarSlot(slot)` — the SELECTION-relative 0..9
/// (`Inventory.SELECTION_SIZE` = 9, the inventory's OWN hotbar
/// indices, NOT the InventoryMenu window numbering whose quirk-y
/// helper this is NOT; the two isHotbarSlot helpers coexist in the
/// jar and this packet rides the strict one) — gates
/// `Inventory.setSelectedSlot(slot)`; anything else (a window slot
/// like 36, a negative) is DROPPED, the packet never reaching the
/// inventory. The parse retains verbatim; the gate is the consumer's
/// act.
#[derive(Debug, Clone, PartialEq)]
pub struct SetHeldSlot {
    /// The selected hotbar slot, selection-relative (the gate's
    /// 0..SELECTION_SIZE window is the consumer's call; the wire's
    /// value rides verbatim).
    pub slot: i32,
}

/// Set Held Slot's wire walk (see [`SetHeldSlot`] — the one-lane
/// VarInt record, the generated-id guard).
pub fn parse_set_held_slot(payload: &[u8]) -> Result<SetHeldSlot> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SET_HELD_SLOT,
        "not a set held slot packet (0x{id:02X})"
    );
    let slot = read_varint(&mut c)?;
    Ok(SetHeldSlot { slot })
}

/// Open Screen: a container window OPENS — the container GUI's own
/// packet (the generated-id 59 pin). Three lanes (javap of the 26.2
/// `ClientboundOpenScreenPacket` clinit): the container id rides
/// `ByteBufCodecs.CONTAINER_ID` — a BARE VARINT (the $22 class's
/// decode is `FriendlyByteBuf.readContainerId`, which is
/// `VarInt.read`, no range check, no byte lane); the menu type rides
/// `ByteBufCodecs.registry(Registries.MENU)` — a VarInt holder id
/// resolved through the synced registry, here retained verbatim (the
/// mob-effect lane's own line: the wire's id is the truth, the
/// resolution is the registry's act); the title rides
/// `ComponentSerialization.TRUSTED_STREAM_CODEC` — ONE bare network
/// NBT tag (the 1.20.5+ anonymous-root shape the skip walker owns),
/// here WALKED for the wire's shape and retained VERBATIM as raw
/// bytes (the text-render round owns the decode; the wire's bytes are
/// the store's title).
#[derive(Debug, Clone, PartialEq)]
pub struct OpenScreen {
    /// The window id the server assigned — the new menu's containerId
    /// (vanilla never opens window 0; the value rides verbatim).
    pub container_id: i32,
    /// The MenuType registry id — the holder VarInt. Vanilla's
    /// built-in registration order (javap of the 26.2 `MenuType`
    /// clinit): generic_9x1 0, generic_9x2 1, generic_9x3 2 (the
    /// chest), generic_9x4 3, generic_9x5 4, generic_9x6 5 (the
    /// double chest), generic_3x3 6, crafter_3x3 7, anvil 8,
    /// beacon 9, blast_furnace 10, brewing_stand 11, crafting 12,
    /// furnace 13, grindstone 14, hopper 15, lectern 16, loom 17,
    /// merchant 18, shulker_box 19, smithing 20, smoker 21,
    /// cartography_table 22, stonecutter 23.
    pub menu_type_id: i32,
    /// The title's network-NBT bytes, retained verbatim (the
    /// anonymous-root compound the TRUSTED_STREAM_CODEC walks).
    pub title_nbt: Vec<u8>,
}

/// Open Screen's wire walk (see [`OpenScreen`] — the three-lane
/// composite, the generated-id guard, the title's verbatim capture).
pub fn parse_open_screen(payload: &[u8]) -> Result<OpenScreen> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::OPEN_SCREEN,
        "not an open screen packet (0x{id:02X})"
    );
    let container_id = read_varint(&mut c)?;
    let menu_type_id = read_varint(&mut c)?;
    // ONE bare network NBT tag — walked for the wire's shape, the
    // consumed span retained verbatim (the title's bytes ARE the
    // store's title; the decode is the text round's act).
    let before = c;
    skip_network_nbt(&mut c)?;
    let taken = before.len() - c.len();
    Ok(OpenScreen {
        container_id,
        menu_type_id,
        title_nbt: before[..taken].to_vec(),
    })
}

/// Container Close: the open container window CLOSES — the container
/// GUI's own packet (the generated-id 17 pin). ONE lane (javap of the
/// 26.2 decode body): `FriendlyByteBuf.readContainerId` — the same
/// BARE VARINT the Open Screen's id rides.
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerClose {
    /// The closing window's id, retained verbatim (the audited
    /// consumer does NOT gate on it — `Player.closeContainer` is one
    /// assignment with no id check; the model arm mirrors that).
    pub container_id: i32,
}

/// Container Close's wire walk (see [`ContainerClose`] — the
/// one-lane VarInt record, the generated-id guard).
pub fn parse_container_close(payload: &[u8]) -> Result<ContainerClose> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::CONTAINER_CLOSE,
        "not a container close packet (0x{id:02X})"
    );
    let container_id = read_varint(&mut c)?;
    Ok(ContainerClose { container_id })
}

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
    /// Respawn: a dimension change (the spawn info carries the new
    /// dimension; the model re-pins min_y from the registry).
    Respawn(SpawnInfo),
    /// Update Light: the server re-sent a column's COMPLETE light
    /// state (the standalone sibling of the chunk packet's tail — the
    /// model replaces the stored light with it).
    Light { x: i32, z: i32, light: ChunkLight },
    /// Set Time: the world's game time and the synced clocks' states
    /// (the day/night timeline driver — the renderer's sky factor is a
    /// pure function of the clock's day tick).
    Time(TimeSync),
    /// Keep-alive probe answered.
    KeepAlive(i64),
    /// Ping probe answered.
    Ping(i32),
    /// Chunk batch started; the pacing reply was sent.
    ChunkBatch,
    /// The server closed the play session (flattened reason).
    Disconnected { reason: String },
    /// Block Update: ONE block changed — the world's own traffic (a
    /// placement, a break, a fluid tick). The model mutates the stored
    /// column in place; the next materialize renders the change.
    Block { x: i32, y: i32, z: i32, state: u32 },
    /// Section Blocks Update: a batch of changes inside ONE section
    /// (the entries are section-relative + the global state id).
    SectionBlocks(SectionBlocks),
    /// Add Entity: something new exists in the world (a mob, a dropped
    /// item, a projectile) — the model's entity registry grows.
    Spawn(EntitySpawn),
    /// Remove Entities: a batch of entity ids ceased to exist.
    EntitiesRemoved(Vec<i32>),
    /// Entity Position Sync: the server's authoritative position
    /// correction for ONE entity (the model moves it in place).
    EntityMoved(EntityPositionSync),
    /// Move Entity Pos: ONE entity's per-tick walk — the raw 1/4096
    /// grid deltas the model applies through the vanilla codec's
    /// quantize-and-add (the deltas are motion, not absolute truth).
    EntityDeltaPos(EntityDeltaPos),
    /// Move Entity Pos and Rotation: the walk plus the body angles.
    EntityDeltaPosRot(EntityDeltaPosRot),
    /// Move Entity Rot: the body turns in place (no position lane).
    EntityDeltaRot(EntityDeltaRot),
    /// Rotate Head: the head yaw's own channel (a mob can look at you
    /// while it walks — the body and the head rotate separately).
    EntityHeadRot(EntityHeadRot),
    /// Set Entity Data: the accessor-indexed metadata the syncs don't
    /// carry (the on-fire/crouched/pose surfaces' wire transport) — the
    /// decoded entries the model merges into the entity's store.
    EntityData(EntityDataSet),
    /// Update Mob Effect: ONE entity's mob-effect state upsert — the
    /// effect state's wire transport; the model filters the LOCAL
    /// player's own id and the lightmap's night-vision factor consumes
    /// the store.
    MobEffectApplied(MobEffectUpdate),
    /// Remove Mob Effect: ONE entity's effect ended.
    MobEffectRemoved(MobEffectRemove),
    /// Set Equipment: ONE entity's gear snapshot — the equipment
    /// state's wire transport (the paired slot/stack walk; an empty
    /// stack is a legal entry — the cleared slot). The model consumes
    /// it when mob rendering arrives.
    EntityEquipped(EntityEquipment),
    /// Set Container Content: ONE container window's full snapshot —
    /// the inventory surfaces' second packet (the slots + the GUI
    /// cursor stack; the container id rides a VarInt, not a byte).
    /// The model consumes it when the inventory surfaces arrive.
    ContainerFilled(ContainerContent),
    /// Set Container Slot: ONE slot's new stack — the inventory
    /// surfaces' third packet (the single-slot sibling; the slot rides
    /// a SIGNED BE SHORT, not a VarInt). The model consumes it when
    /// the inventory surfaces arrive.
    ContainerSlotSet(ContainerSetSlot),
    /// Set Held Slot: the hotbar's SELECTED slot moved — the
    /// selection-relative index (0..8) the consumer's gate
    /// (`Inventory.isHotbarSlot`, NOT the window numbering) lets
    /// through.
    HeldSlotSet(SetHeldSlot),
    /// Open Screen: a container window OPENS — the window id, the
    /// MenuType registry id, and the title's raw network-NBT bytes
    /// (the container GUI's birth; the model consumes it in the
    /// open-container arm).
    ScreenOpened(OpenScreen),
    /// Container Close: the open container window CLOSES — the
    /// bare-VarInt window id (the model consumes it in the
    /// open-container arm).
    ContainerClosed(ContainerClose),
    /// Any packet without a client surface yet (skipped silently).
    Skipped,
}

/// A live play-state session on an established connection (login and
/// configuration already finished on the SAME wire). Carries the registry
/// knowledge the configuration phase retained (dimension-type layouts).
pub struct PlaySession<S: Read + Write> {
    conn: PacketConn<S>,
    pub registry: RegistryData,
    /// The skin model the login profile named (`Some("slim")` or `None`
    /// for the classic wide default) — the profile-metadata fact the
    /// first-person arm renders with. See
    /// [`crate::login::profile_skin_model`].
    pub skin_model: Option<String>,
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
            clientbound::RESPAWN => Ok(PlayEvent::Respawn(parse_respawn(&pkt)?)),
            clientbound::CHUNK_DATA => Ok(PlayEvent::Chunk(parse_chunk_data(&pkt)?)),
            clientbound::SET_TIME => Ok(PlayEvent::Time(parse_set_time(&pkt)?)),
            clientbound::UPDATE_LIGHT => {
                let (x, z, light) = parse_update_light(&pkt)?;
                Ok(PlayEvent::Light { x, z, light })
            }
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
            clientbound::BLOCK_UPDATE => {
                let (x, y, z, state) = parse_block_update(&pkt)?;
                Ok(PlayEvent::Block { x, y, z, state })
            }
            clientbound::SECTION_BLOCKS_UPDATE => {
                Ok(PlayEvent::SectionBlocks(parse_section_blocks(&pkt)?))
            }
            clientbound::ADD_ENTITY => Ok(PlayEvent::Spawn(parse_add_entity(&pkt)?)),
            clientbound::REMOVE_ENTITIES => {
                Ok(PlayEvent::EntitiesRemoved(parse_remove_entities(&pkt)?))
            }
            clientbound::ENTITY_POSITION_SYNC => {
                Ok(PlayEvent::EntityMoved(parse_entity_position_sync(&pkt)?))
            }
            clientbound::MOVE_ENTITY_POS => {
                Ok(PlayEvent::EntityDeltaPos(parse_move_entity_pos(&pkt)?))
            }
            clientbound::MOVE_ENTITY_POS_ROT => {
                Ok(PlayEvent::EntityDeltaPosRot(parse_move_entity_pos_rot(&pkt)?))
            }
            clientbound::MOVE_ENTITY_ROT => {
                Ok(PlayEvent::EntityDeltaRot(parse_move_entity_rot(&pkt)?))
            }
            clientbound::ROTATE_HEAD => Ok(PlayEvent::EntityHeadRot(parse_rotate_head(&pkt)?)),
            clientbound::SET_ENTITY_DATA => Ok(PlayEvent::EntityData(parse_set_entity_data(&pkt)?)),
            clientbound::UPDATE_MOB_EFFECT => {
                Ok(PlayEvent::MobEffectApplied(parse_update_mob_effect(&pkt)?))
            }
            clientbound::REMOVE_MOB_EFFECT => {
                Ok(PlayEvent::MobEffectRemoved(parse_remove_mob_effect(&pkt)?))
            }
            clientbound::SET_EQUIPMENT => {
                Ok(PlayEvent::EntityEquipped(parse_set_equipment(&pkt)?))
            }
            clientbound::CONTAINER_SET_CONTENT => {
                Ok(PlayEvent::ContainerFilled(parse_set_container_content(&pkt)?))
            }
            clientbound::CONTAINER_SET_SLOT => {
                Ok(PlayEvent::ContainerSlotSet(parse_set_container_slot(&pkt)?))
            }
            clientbound::SET_HELD_SLOT => {
                Ok(PlayEvent::HeldSlotSet(parse_set_held_slot(&pkt)?))
            }
            clientbound::OPEN_SCREEN => {
                Ok(PlayEvent::ScreenOpened(parse_open_screen(&pkt)?))
            }
            clientbound::CONTAINER_CLOSE => {
                Ok(PlayEvent::ContainerClosed(parse_container_close(&pkt)?))
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
        PlayOutcome::Joined { registry, skin_model, .. } => Ok(PlayStart::Session(PlaySession {
            conn,
            registry,
            skin_model,
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

/// How a live join ended. `Live` carries the milestone snapshot AND the
/// session to keep pumping: the connection stays open past the world-load
/// milestone and every later packet (re-sent chunks, teleports, probes,
/// the dimension changes) rides the SAME wire — the live view's input
/// stream. Dropping the session closes the socket (the vanilla client's
/// disconnect).
pub enum LiveJoin<S: Read + Write> {
    /// World info + the first position teleport + at least one decoded
    /// chunk (the vanilla "loading screen finished" milestone), plus the
    /// live session.
    Live {
        snapshot: WorldSnapshot,
        session: PlaySession<S>,
    },
    /// Login/config refusal or kick; carries the flattened reason.
    Disconnected { reason: String },
}

/// Connect + full vanilla join (login, configuration, play) with the
/// offline profile UUID for `username`, retaining the LIVE session after
/// the world-load milestone: the caller keeps pumping the returned
/// session for as long as the join lives (chunks keep streaming in,
/// teleports re-anchor the view, probes are answered in place — the
/// live view's input stream).
pub fn connect_and_join_live(
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<LiveJoin<TcpStream>> {
    let addr = format!("{host}:{port}");
    let timeout = std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS);
    let stream =
        std::net::TcpStream::connect(&addr).with_context(|| format!("tcp connect to {addr}"))?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    match begin_play(stream, host, port, protocol_version, username)? {
        PlayStart::Session(mut session) => match join_world(&mut session)? {
            WorldOutcome::Loaded(snapshot) => Ok(LiveJoin::Live { snapshot, session }),
            WorldOutcome::Disconnected { reason } => Ok(LiveJoin::Disconnected { reason }),
        },
        PlayStart::Disconnected { reason } => Ok(LiveJoin::Disconnected { reason }),
    }
}

/// Connect + full vanilla join, pumped to the world-load milestone. The
/// ONE-SHOT contract: the session closes when the milestone returns
/// (the snapshot is the whole world the caller gets). For a join whose
/// stream keeps living, [`connect_and_join_live`] returns the session.
pub fn connect_and_join_world(
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<WorldOutcome> {
    Ok(match connect_and_join_live(host, port, protocol_version, username)? {
        LiveJoin::Live { snapshot, .. } => WorldOutcome::Loaded(snapshot),
        LiveJoin::Disconnected { reason } => WorldOutcome::Disconnected { reason },
    })
}

/// Mock-server wire builders shared by the mc-protocol tests and the
/// mc-client model/view tests — one truth for the chunk light tail,
/// not hand-duplicated bytes in every test.
pub mod test_support {
    use super::clientbound;
    use crate::varint::write_varint;

    /// The javap-proven light tail shared by both packet shapes: four
    /// BitSets (sky/block lit + empty masks) then the sky array list,
    /// then the block list. The lit masks derive from the maps, so the
    /// vanilla writer's count == popcount invariant holds by
    /// construction (lying packets are built by hand in the error
    /// tests, never here).
    pub fn light_tail(
        sky: &[(usize, Vec<u8>)],
        empty_sky: &[usize],
        block: &[(usize, Vec<u8>)],
        empty_block: &[usize],
    ) -> Vec<u8> {
        let mut p = Vec::new();
        let write_mask = |p: &mut Vec<u8>, bits: &[usize]| {
            write_varint(p, 1); // one mask long (fixtures are far below 64 sections)
            let mut long = 0u64;
            for &b in bits {
                assert!(b < 64, "light fixture bit {b} needs more than one mask long");
                long |= 1 << b;
            }
            p.extend_from_slice(&long.to_be_bytes());
        };
        write_mask(&mut p, &sky.iter().map(|e| e.0).collect::<Vec<_>>());
        write_mask(&mut p, &block.iter().map(|e| e.0).collect::<Vec<_>>());
        write_mask(&mut p, empty_sky);
        write_mask(&mut p, empty_block);
        let write_arrays = |p: &mut Vec<u8>, arrays: &[(usize, Vec<u8>)]| {
            write_varint(p, arrays.len() as i32);
            for (_, a) in arrays {
                assert_eq!(a.len(), 2048, "light fixture arrays are 2048 bytes");
                write_varint(p, a.len() as i32);
                p.extend_from_slice(a);
            }
        };
        write_arrays(&mut p, sky);
        write_arrays(&mut p, block);
        p
    }

    /// One full Chunk Data packet: chunk coords, an empty heightmaps
    /// compound, the section buffers back to back, no block entities,
    /// then the light tail. `sky`/`block` map LIGHT-SECTION bits (0 =
    /// one section below the column, 1..=sections.len() the column's
    /// own sections, sections.len()+1 one above) to their 2048-byte
    /// nibble arrays; `empty_sky`/`empty_block` mark known-zero bits.
    pub fn chunk_packet_with_light(
        x: i32,
        z: i32,
        sections: &[Vec<u8>],
        sky: &[(usize, Vec<u8>)],
        empty_sky: &[usize],
        block: &[(usize, Vec<u8>)],
        empty_block: &[usize],
    ) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CHUNK_DATA);
        p.extend_from_slice(&x.to_be_bytes());
        p.extend_from_slice(&z.to_be_bytes());
        p.push(0x0A);
        p.push(0x00); // empty heightmaps compound
        let buffer_len: usize = sections.iter().map(|s| s.len()).sum();
        write_varint(&mut p, buffer_len as i32);
        for s in sections {
            p.extend_from_slice(s);
        }
        write_varint(&mut p, 0); // no block entities
        p.extend_from_slice(&light_tail(sky, empty_sky, block, empty_block));
        p
    }

    /// The standalone Update Light packet (generated id 48): VarInt
    /// chunk X/Z — NOT the chunk packet's Ints, javap of the real
    /// constructor — followed by the SAME light tail.
    pub fn update_light_packet(
        x: i32,
        z: i32,
        sky: &[(usize, Vec<u8>)],
        empty_sky: &[usize],
        block: &[(usize, Vec<u8>)],
        empty_block: &[usize],
    ) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::UPDATE_LIGHT);
        write_varint(&mut p, x);
        write_varint(&mut p, z);
        p.extend_from_slice(&light_tail(sky, empty_sky, block, empty_block));
        p
    }

    /// The Set Time packet (generated id 113): LONG game time, then the
    /// clock map — per entry VarInt id, VAR_LONG totalTicks, FLOAT
    /// partialTick, FLOAT rate (the javap-proven 26.2 shape).
    pub fn set_time_packet(game_time: i64, clocks: &[(i32, i64, f32, f32)]) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_TIME);
        p.extend_from_slice(&game_time.to_be_bytes());
        write_varint(&mut p, clocks.len() as i32);
        for (id, total, partial, rate) in clocks {
            write_varint(&mut p, *id);
            crate::varint::write_varlong(&mut p, *total);
            p.extend_from_slice(&partial.to_be_bytes());
            p.extend_from_slice(&rate.to_be_bytes());
        }
        p
    }

    /// The packed block position long — the inverse of the javap-proven
    /// decode (`((x & 0x3FFFFFF) << 38) | (y & 0xFFF) |
    /// ((z & 0x3FFFFFF) << 12)`, `BlockPos.asLong(III)` verbatim).
    pub fn encode_block_pos(x: i32, y: i32, z: i32) -> i64 {
        (((x as i64) & 0x3FF_FFFF) << 38) | ((y as i64) & 0xFFF) | (((z as i64) & 0x3FF_FFFF) << 12)
    }

    /// The packed section position long — the inverse of the javap-proven
    /// decode (`((x & 0x3FFFFF) << 42) | (y & 0xFFFFF) |
    /// ((z & 0x3FFFFF) << 20)`, `SectionPos.asLong(III)` verbatim).
    pub fn encode_section_pos(sx: i32, sy: i32, sz: i32) -> i64 {
        (((sx as i64) & 0x3F_FFFF) << 42) | ((sy as i64) & 0xF_FFFF) | (((sz as i64) & 0x3F_FFFF) << 20)
    }

    /// One full Block Update packet (generated id 8): the packed
    /// position long then the VarInt state id.
    pub fn block_update_packet(x: i32, y: i32, z: i32, state: u32) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::BLOCK_UPDATE);
        p.extend_from_slice(&encode_block_pos(x, y, z).to_be_bytes());
        write_varint(&mut p, state as i32);
        p
    }

    /// One full Section Blocks Update packet (generated id 84): the
    /// packed section long, the VarInt count, then the VarLong entries —
    /// each `local | (state << 12)` with local `(x << 8) | (z << 4) | y`.
    pub fn section_blocks_packet(sx: i32, sy: i32, sz: i32, blocks: &[((i32, i32, i32), u32)]) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SECTION_BLOCKS_UPDATE);
        p.extend_from_slice(&encode_section_pos(sx, sy, sz).to_be_bytes());
        write_varint(&mut p, blocks.len() as i32);
        for &((lx, ly, lz), state) in blocks {
            let local = (((lx & 0xF) << 8) | ((lz & 0xF) << 4) | (ly & 0xF)) as i64;
            crate::varint::write_varlong(&mut p, local | ((state as i64) << 12));
        }
        p
    }

    /// The wire's low-precision Vec3 ENCODER — the exact inverse of the
    /// javap-proven `LpVec3.read` (the mirror of `LpVec3.write`'s byte
    /// order: byte0, byte1, then a 4-byte int of bits 16-47, then the
    /// optional scale continuation VarInt when the scale exceeds the
    /// 2-bit field). `scale` is the magnitude the packed lanes multiply
    /// back to; callers pass it explicitly so tests can pin both the
    /// short-scale (no continuation) and the continuation shapes.
    pub fn encode_lp_vec3(x: f64, y: f64, z: f64, scale: u64) -> Vec<u8> {
        let pack = |d: f64| -> u64 {
            ((d * 0.5 + 0.5) * 32766.0).round() as u64
        };
        let lane = |v: f64| -> u64 { pack(v / scale as f64) };
        let packed = (scale & 3)
            | (if scale > 3 { 4 } else { 0 })
            | (lane(x) << 3)
            | (lane(y) << 18)
            | (lane(z) << 33);
        let mut p = Vec::new();
        p.push((packed & 0xFF) as u8);
        p.push(((packed >> 8) & 0xFF) as u8);
        p.extend_from_slice(&(((packed >> 16) & 0xFFFF_FFFF) as u32).to_be_bytes());
        if scale > 3 {
            crate::varint::write_varint(&mut p, (scale >> 2) as i32);
        }
        p
    }

    /// One full Add Entity packet (generated id 1): the VarInt id, the
    /// UUID's two big-endian longs, the VarInt type, three big-endian
    /// doubles, the low-precision movement, three angle bytes, the
    /// VarInt data.
    #[allow(clippy::too_many_arguments)]
    pub fn add_entity_packet(
        id: i32,
        uuid: u128,
        type_id: i32,
        x: f64,
        y: f64,
        z: f64,
        movement: &[u8],
        x_rot: i8,
        y_rot: i8,
        y_head_rot: i8,
        data: i32,
    ) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::ADD_ENTITY);
        write_varint(&mut p, id);
        p.extend_from_slice(&((uuid >> 64) as u64).to_be_bytes());
        p.extend_from_slice(&(uuid as u64).to_be_bytes());
        write_varint(&mut p, type_id);
        p.extend_from_slice(&x.to_be_bytes());
        p.extend_from_slice(&y.to_be_bytes());
        p.extend_from_slice(&z.to_be_bytes());
        p.extend_from_slice(movement);
        p.push(x_rot as u8);
        p.push(y_rot as u8);
        p.push(y_head_rot as u8);
        write_varint(&mut p, data);
        p
    }

    /// One full Remove Entities packet (generated id 77): the VarInt
    /// count then that many VarInt ids (`readIntIdList` verbatim).
    pub fn remove_entities_packet(ids: &[i32]) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REMOVE_ENTITIES);
        write_varint(&mut p, ids.len() as i32);
        for &id in ids {
            write_varint(&mut p, id);
        }
        p
    }

    /// One full Entity Position Sync packet (generated id 35): the
    /// VarInt id, the PositionMoveRotation record (two full-precision
    /// Vec3s then FLOAT yaw, FLOAT pitch) and the BOOL ground flag.
    #[allow(clippy::too_many_arguments)]
    pub fn entity_position_sync_packet(
        id: i32,
        x: f64,
        y: f64,
        z: f64,
        delta: [f64; 3],
        y_rot: f32,
        x_rot: f32,
        on_ground: bool,
    ) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::ENTITY_POSITION_SYNC);
        write_varint(&mut p, id);
        p.extend_from_slice(&x.to_be_bytes());
        p.extend_from_slice(&y.to_be_bytes());
        p.extend_from_slice(&z.to_be_bytes());
        p.extend_from_slice(&delta[0].to_be_bytes());
        p.extend_from_slice(&delta[1].to_be_bytes());
        p.extend_from_slice(&delta[2].to_be_bytes());
        p.extend_from_slice(&y_rot.to_be_bytes());
        p.extend_from_slice(&x_rot.to_be_bytes());
        p.push(if on_ground { 1 } else { 0 });
        p
    }

    /// One full Move Entity Pos packet (generated id 53): the VarInt
    /// id, the three RAW SHORT deltas and the BOOL ground flag.
    pub fn move_entity_pos_packet(id: i32, dx: i16, dy: i16, dz: i16, on_ground: bool) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::MOVE_ENTITY_POS);
        write_varint(&mut p, id);
        p.extend_from_slice(&dx.to_be_bytes());
        p.extend_from_slice(&dy.to_be_bytes());
        p.extend_from_slice(&dz.to_be_bytes());
        p.push(if on_ground { 1 } else { 0 });
        p
    }

    /// One full Move Entity Pos and Rotation packet (generated id 54):
    /// the Pos layout plus the yaw/pitch angle BYTES (raw — the tests
    /// pin the unpack).
    pub fn move_entity_pos_rot_packet(
        id: i32,
        dx: i16,
        dy: i16,
        dz: i16,
        y_rot: i8,
        x_rot: i8,
        on_ground: bool,
    ) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::MOVE_ENTITY_POS_ROT);
        write_varint(&mut p, id);
        p.extend_from_slice(&dx.to_be_bytes());
        p.extend_from_slice(&dy.to_be_bytes());
        p.extend_from_slice(&dz.to_be_bytes());
        p.push(y_rot as u8);
        p.push(x_rot as u8);
        p.push(if on_ground { 1 } else { 0 });
        p
    }

    /// One full Move Entity Rot packet (generated id 56 — NOT the
    /// minecart's 55): the VarInt id, the yaw/pitch angle BYTES and
    /// the BOOL; no position lane.
    pub fn move_entity_rot_packet(id: i32, y_rot: i8, x_rot: i8, on_ground: bool) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::MOVE_ENTITY_ROT);
        write_varint(&mut p, id);
        p.push(y_rot as u8);
        p.push(x_rot as u8);
        p.push(if on_ground { 1 } else { 0 });
        p
    }

    /// One full Rotate Head packet (generated id 83): the VarInt id
    /// and the ONE head-yaw angle BYTE.
    pub fn rotate_head_packet(id: i32, y_head_rot: i8) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::ROTATE_HEAD);
        write_varint(&mut p, id);
        p.push(y_head_rot as u8);
        p
    }

    /// ONE metadata entry's wire bytes: the index byte, the VarInt
    /// serializer id, then the value bytes the caller assembled (the
    /// test shapes the codec outputs — floats as BE f32, packed longs
    /// as BE i64, strings through `write_string`).
    pub fn entity_data_entry(index: u8, serializer: i32, value: &[u8]) -> Vec<u8> {
        let mut p = Vec::new();
        p.push(index);
        write_varint(&mut p, serializer);
        p.extend_from_slice(value);
        p
    }

    /// A VarInt value's wire bytes (the i32-family serializers'
    /// payloads).
    pub fn entity_data_varint(v: i32) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, v);
        p
    }

    /// One full Set Entity Data packet (generated id 99): the VarInt
    /// packet id, the VarInt entity id, the entries' bytes, then the
    /// 0xFF EOF marker the vanilla `pack` writes after the list.
    pub fn set_entity_data_packet(id: i32, entries: &[u8]) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_ENTITY_DATA);
        write_varint(&mut p, id);
        p.extend_from_slice(entries);
        p.push(255);
        p
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

fn read_u32(r: &mut &[u8]) -> std::io::Result<u32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_be_bytes(b))
}

fn read_i8(r: &mut &[u8]) -> std::io::Result<i8> {
    Ok(read_u8(r)? as i8)
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

fn read_i64(r: &mut &[u8]) -> std::io::Result<i64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(i64::from_be_bytes(b))
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
    use crate::login::{encode_login_success, encode_login_success_with_properties, encode_set_compression};
    use crate::varint::write_string;
    use std::net::TcpListener;
    use std::thread::JoinHandle;

    #[test]
    fn play_ids_are_the_generated_runtime_facts() {
        // The table is GENERATED from the official jar's own bootstrapped
        // protocol (`mc2rs-gen dump-packet-ids`); these pins come from the
        // real 26.2 runtime dump — the wiki-era constants they replaced
        // were stale for this protocol version (login was 0x27 there).
        assert_eq!(clientbound::LOGIN, 49, "login, runtime-dumped");
        assert_eq!(clientbound::CHUNK_DATA, 45, "level_chunk_with_light");
        assert_eq!(clientbound::KEEP_ALIVE, 44);
        assert_eq!(clientbound::PING, 61);
        assert_eq!(clientbound::DISCONNECT, 32);
        assert_eq!(clientbound::BATCH_START, 12, "chunk_batch_start");
        assert_eq!(clientbound::UNLOAD_CHUNK, 37, "forget_level_chunk");
        assert_eq!(clientbound::SYNCHRONIZE_POSITION, 72, "player_position");
        assert_eq!(clientbound::CENTER_CHUNK, 94, "set_chunk_cache_center");
        assert_eq!(clientbound::RESPAWN, 82, "the dimension-change packet");
        assert_eq!(serverbound::CONFIRM_TELEPORTATION, 0, "accept_teleportation");
        assert_eq!(serverbound::CHUNK_BATCH_RECEIVED, 11);
        assert_eq!(serverbound::KEEP_ALIVE, 28);
        assert_eq!(serverbound::PONG, 45);
        // both flows are dense: the last id is the entry count - 1
        let cb = &crate::generated::packet_ids::PLAY_CLIENTBOUND;
        let sb = &crate::generated::packet_ids::PLAY_SERVERBOUND;
        assert_eq!(cb.last().unwrap().0, (cb.len() - 1) as u32, "clientbound dense");
        assert_eq!(sb.last().unwrap().0, (sb.len() - 1) as u32, "serverbound dense");
        assert_eq!(cb.len(), 141);
        assert_eq!(sb.len(), 69);
    }

    /// Build a vanilla 26.2 Login payload (the javap-proven field order).
    fn vanilla_login_payload(dimension: &str, view: i32, portal: i32) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::LOGIN);
        p.extend_from_slice(&17i32.to_be_bytes()); // playerId
        p.push(0); // hardcore
        write_varint(&mut p, 2); // two levels
        write_string(&mut p, "minecraft:overworld");
        write_string(&mut p, "minecraft:the_nether");
        write_varint(&mut p, 20); // max players (dropped)
        write_varint(&mut p, view); // chunk radius
        write_varint(&mut p, 8); // simulation distance
        p.push(0);
        p.push(0);
        p.push(0); // 3 dropped bools
        write_varint(&mut p, 0); // dimension_type registry id
        write_string(&mut p, dimension);
        p.extend_from_slice(&1234i64.to_be_bytes()); // seed (dropped)
        p.push(1); // game type (dropped)
        p.push(1); // previous game type (dropped)
        p.push(0); // is debug (dropped)
        p.push(0); // is flat (dropped)
        p.push(0); // no death location
        write_varint(&mut p, portal); // portal cooldown
        write_varint(&mut p, 63); // sea level (dropped)
        p.push(1); // online mode (dropped)
        p.push(1); // enforces secure chat (dropped)
        p
    }

    #[test]
    fn login_parses_the_proven_vanilla_shape_and_pins_the_dimension() {
        let info = parse_world_info(&vanilla_login_payload("minecraft:overworld", 12, 300))
            .expect("vanilla login parses");
        assert_eq!(info.entity_id, 17);
        assert_eq!(info.view_distance, 12, "chunkRadius, not maxPlayers");
        assert_eq!(info.simulation_distance, 8);
        assert_eq!(info.portal_cooldown, 300, "from CommonPlayerSpawnInfo");
        assert_eq!(info.current_dimension, "minecraft:overworld");
        assert_eq!(
            info.dimensions,
            vec!["minecraft:overworld".to_string(), "minecraft:the_nether".to_string()]
        );
    }

    #[test]
    fn respawn_parses_the_shared_spawn_info() {
        // Respawn = CommonPlayerSpawnInfo + dataToKeep (dropped rest)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::RESPAWN);
        write_varint(&mut p, 1); // dimension_type registry id (nether entry)
        write_string(&mut p, "minecraft:the_nether");
        p.extend_from_slice(&0i64.to_be_bytes()); // seed
        p.push(1);
        p.push(1); // game types
        p.push(0);
        p.push(0); // debug/flat
        // a PRESENT death location: dimension string + packed pos — the
        // skip must consume exactly this shape
        p.push(1);
        write_string(&mut p, "minecraft:overworld");
        p.extend_from_slice(&42i64.to_be_bytes());
        write_varint(&mut p, 60); // portal cooldown
        write_varint(&mut p, 31); // sea level
        p.push(3); // dataToKeep (ignored — stays in the framed payload)
        let spawn = parse_respawn(&p).expect("respawn parses");
        assert_eq!(spawn.dimension, "minecraft:the_nether");
        assert_eq!(spawn.dimension_type_id, 1);
        assert_eq!(spawn.portal_cooldown, 60);
    }

    #[test]
    fn synchronize_position_parses_the_proven_26_2_composite() {
        // the javap-proven 26.2 shape: VAR_INT teleportId FIRST, the
        // PositionMoveRotation (position Vec3 + deltaMovement Vec3 +
        // yaw/pitch floats), then the INT relativity bitmask LAST
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SYNCHRONIZE_POSITION);
        write_varint(&mut p, 42);
        p.extend_from_slice(&8.5f64.to_be_bytes());
        p.extend_from_slice(&64.0f64.to_be_bytes());
        p.extend_from_slice(&(-3.25f64).to_be_bytes());
        p.extend_from_slice(&0.5f64.to_be_bytes());
        p.extend_from_slice(&(-1.25f64).to_be_bytes());
        p.extend_from_slice(&0.75f64.to_be_bytes()); // the delta movement
        p.extend_from_slice(&90.0f32.to_be_bytes());
        p.extend_from_slice(&(-12.5f32).to_be_bytes());
        p.extend_from_slice(&0b1_0000_0101i32.to_be_bytes()); // X + Z + DELTA_Z
        let pos = parse_synchronize_position(&p).expect("26.2 teleport parses");
        assert_eq!(pos.teleport_id, 42, "the id leads the 26.2 wire");
        assert_eq!((pos.x, pos.y, pos.z), (8.5, 64.0, -3.25));
        assert_eq!((pos.dx, pos.dy, pos.dz), (0.5, -1.25, 0.75));
        assert_eq!((pos.yaw, pos.pitch), (90.0, -12.5));
        assert_eq!(pos.flags, 0b1_0000_0101, "a full int bitmask, 9 Relative bits");
        // wrong id and truncation are errors, not guesses
        let mut wrong = Vec::new();
        write_varint(&mut wrong, 0x77);
        assert!(parse_synchronize_position(&wrong).is_err());
        assert!(parse_synchronize_position(&p[..p.len() - 6]).is_err());
        // trailing bytes (the wiki-era on-ground byte habit) are drift
        let mut trailing = p.clone();
        trailing.push(0x01);
        assert!(parse_synchronize_position(&trailing).is_err());
    }

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
    /// `biome` picks the section's single-valued biome id — the caller
    /// names it through the synced table (plains 0 for the overworld
    /// stream, crimson_forest 1 for the nether's own chunk).
    fn build_stone_dirt_section(biome: u32) -> Vec<u8> {
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
        write_varint(&mut section, biome as i32);
        section
    }

    #[test]
    fn chunk_packet_decodes_palettized_sections() {
        let buffer = build_stone_dirt_section(1);
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CHUNK_DATA);
        p.extend_from_slice(&3i32.to_be_bytes()); // chunk x
        p.extend_from_slice(&(-2i32).to_be_bytes()); // chunk z
        p.push(0x0A); // heightmaps: empty anonymous compound
        p.push(0x00);
        write_varint(&mut p, buffer.len() as i32);
        p.extend_from_slice(&buffer);
        write_varint(&mut p, 0); // no block entities
        p.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // the EMPTY light tail: four zeroed BitSets + two zero counts
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
        // the empty light tail decodes to "no light anywhere": the vecs
        // span the column's light range (1 section + the two engine
        // layers) but every slot stays the honest None
        assert_eq!(col.light.sky.len(), 3);
        assert_eq!(col.light.block.len(), 3);
        assert!(col.light.sky.iter().all(Option::is_none));
        assert!(col.light.block.iter().all(Option::is_none));
        // truncated buffer is rejected, not misparsed (cut deep into the
        // section data array — the declared chunk-buffer size no longer
        // fits the packet)
        let mut cut = p.clone();
        cut.truncate(cut.len() - 300);
        assert!(parse_chunk_data(&cut).is_err());
    }

    #[test]
    fn chunk_light_tail_decodes_masks_arrays_and_engine_layers() {
        // the four fixture arrays carry DISTINCT bytes at the indices
        // the queries later walk, pinning both the YZX index order and
        // the low/high nibble split (javap: DataLayer.getByteIndex =
        // i >> 1, getNibbleIndex = i & 1 — even index, low nibble)
        let mut own = vec![0u8; 2048];
        own[0] = 0x5A; // idx 0 (x0,y0,z0) low = 0xA; idx 1 (x1) high = 0x5
        own[1] = 0x34; // idx 2 (x2) low = 0x4
        own[16] = 0x0F; // idx 32 (z2) low = 0xF
        own[128] = 0x21; // idx 256 (y1) low = 0x1
        let below = vec![0xF0u8; 2048]; // idx 0 low = 0x0 — the under-world layer
        let above = vec![0xFFu8; 2048]; // full-bright layer above the top
        let mut blocky = vec![0u8; 2048];
        blocky[0] = 0x23; // idx 0 low = 3; idx 1 high = 2
        let p = test_support::chunk_packet_with_light(
            3,
            -2,
            &[build_stone_dirt_section(1), build_stone_dirt_section(1)],
            // the full light-section spread: bit 0 below, bits 1-2 the
            // column's own sections (1 data + 1 known-empty), bit 3 above
            &[(0, below), (1, own), (3, above)],
            &[2],
            &[(1, blocky)],
            &[],
        );
        let col = parse_chunk_data(&p).unwrap();
        assert_eq!(col.sections.len(), 2);
        assert_eq!(
            col.light.sky.len(),
            4,
            "2 sections + the two engine layers below/above"
        );
        assert_eq!(col.light.block.len(), 4);
        assert_eq!(col.light.sky[0], Some(LightData::Data(vec![0xF0u8; 2048])));
        let own_data = match &col.light.sky[1] {
            Some(LightData::Data(a)) => a,
            other => panic!("sky bit 1 should be data, got {other:?}"),
        };
        assert_eq!(own_data[0], 0x5A, "idx 0 and 1 both live in byte 0");
        assert_eq!(own_data[16], 0x0F, "idx 32 (z2) lives in byte 16");
        assert_eq!(own_data[128], 0x21, "idx 256 (y1) lives in byte 128");
        assert_eq!(col.light.sky[2], Some(LightData::Empty), "the empty mask");
        assert_eq!(col.light.sky[3], Some(LightData::Data(vec![0xFFu8; 2048])));
        let block_data = match &col.light.block[1] {
            Some(LightData::Data(a)) => a,
            other => panic!("block bit 1 should be data, got {other:?}"),
        };
        assert_eq!(block_data[0], 0x23, "the block layer carries its own array");
        // untouched bits stay the honest None
        assert_eq!(col.light.block[0], None);
        assert_eq!(col.light.block[2], None);
        assert_eq!(col.light.block[3], None);
    }

    #[test]
    fn corrupt_light_tails_are_rejected() {
        // hand-rolled tails — the mock builder cannot lie by construction
        let arr = vec![0u8; 2048];
        let one_section_packet = |tail: &[u8]| {
            let buffer = build_stone_dirt_section(1);
            let mut p = Vec::new();
            write_varint(&mut p, clientbound::CHUNK_DATA);
            p.extend_from_slice(&0i32.to_be_bytes());
            p.extend_from_slice(&0i32.to_be_bytes());
            p.push(0x0A);
            p.push(0x00);
            write_varint(&mut p, buffer.len() as i32);
            p.extend_from_slice(&buffer);
            write_varint(&mut p, 0);
            p.extend_from_slice(tail);
            p
        };
        let tail = |sky: u64,
                    block: u64,
                    empty_sky: u64,
                    empty_block: u64,
                    sky_arrays: &[&[u8]],
                    block_arrays: &[&[u8]]| {
            let mut t = Vec::new();
            for mask in [sky, block, empty_sky, empty_block] {
                write_varint(&mut t, 1);
                t.extend_from_slice(&mask.to_be_bytes());
            }
            for arrays in [sky_arrays, block_arrays] {
                write_varint(&mut t, arrays.len() as i32);
                for a in arrays {
                    write_varint(&mut t, a.len() as i32);
                    t.extend_from_slice(a);
                }
            }
            t
        };
        // a lying count: one mask bit but no array behind it
        let t = tail(0b10, 0, 0, 0, &[], &[]);
        assert!(parse_chunk_data(&one_section_packet(&t)).is_err());
        // an array too short for the nibble grid (10 bytes, not 2048)
        let t = tail(0b10, 0, 0, 0, &[&arr[..10]], &[]);
        assert!(parse_chunk_data(&one_section_packet(&t)).is_err());
        // a mask bit outside the column's light range: one section
        // spans bits 0..=2 (below/own/above), bit 4 is beyond them
        let t = tail(0b10000, 0, 0, 0, &[&arr], &[]);
        assert!(parse_chunk_data(&one_section_packet(&t)).is_err());
        // the same section marked lit AND empty — the writer never does
        let t = tail(0b10, 0, 0b10, 0, &[&arr], &[]);
        assert!(parse_chunk_data(&one_section_packet(&t)).is_err());
        // trailing bytes after the block array list are drift
        let mut t = tail(0, 0, 0, 0, &[], &[]);
        t.push(0x00);
        assert!(parse_chunk_data(&one_section_packet(&t)).is_err());
        // an absurd (negative) bitset long count — rejected BEFORE any
        // allocation
        let mut t = Vec::new();
        write_varint(&mut t, -1);
        assert!(parse_chunk_data(&one_section_packet(&t)).is_err());
    }

    #[test]
    fn update_light_parses_the_standalone_tail() {
        assert_eq!(clientbound::UPDATE_LIGHT, 48, "light_update (generated id)");
        let p = test_support::update_light_packet(
            3,
            -2,
            &[(1, vec![0x5Au8; 2048])],
            &[2],
            &[],
            &[],
        );
        let (x, z, light) = parse_update_light(&p).unwrap();
        assert_eq!((x, z), (3, -2), "VarInt coords, not the chunk packet's Ints");
        assert_eq!(
            light.sky.len(),
            3,
            "no world height on this wire: sized to the masks' own span (max bit 2 + 1)"
        );
        assert_eq!(light.sky[1], Some(LightData::Data(vec![0x5Au8; 2048])));
        assert_eq!(light.sky[2], Some(LightData::Empty));
        assert_eq!(light.block.len(), 3, "both layers span the same masks");
        assert_eq!(light.block[1], None, "the block layer carries nothing here");
        // trailing bytes after the standalone tail are drift
        let mut p2 = test_support::update_light_packet(0, 0, &[], &[], &[], &[]);
        p2.push(0x00);
        assert!(parse_update_light(&p2).is_err());
        // a lying array count fails on the standalone wire too (the mock
        // builder cannot lie by construction — hand-rolled here)
        let mut p3 = Vec::new();
        write_varint(&mut p3, clientbound::UPDATE_LIGHT);
        write_varint(&mut p3, 0);
        write_varint(&mut p3, 0);
        for mask in [0b10u64, 0, 0, 0] {
            // sky bit 1 lit...
            write_varint(&mut p3, 1);
            p3.extend_from_slice(&mask.to_be_bytes());
        }
        write_varint(&mut p3, 0); // ...but ZERO sky arrays — the lie
        write_varint(&mut p3, 0);
        assert!(parse_update_light(&p3).is_err());
    }

    #[test]
    fn set_time_parses_the_clock_map() {
        assert_eq!(clientbound::SET_TIME, 113, "set_time (generated id)");
        let p = test_support::set_time_packet(
            123_456,
            &[(0, 18_000, 0.5, 1.0), (1, -2_400, 0.0, 0.0)],
        );
        let ts = parse_set_time(&p).unwrap();
        assert_eq!(ts.game_time, 123_456, "the LONG game time verbatim");
        assert_eq!(ts.clocks.len(), 2, "VarInt count then that many entries");
        assert_eq!(
            ts.clocks[0],
            ClockUpdate { id: 0, total_ticks: 18_000, partial_tick: 0.5, rate: 1.0 }
        );
        assert_eq!(
            ts.clocks[1],
            ClockUpdate { id: 1, total_ticks: -2_400, partial_tick: 0.0, rate: 0.0 },
            "negative total ticks roundtrip through the VarLong"
        );
        // trailing bytes after the clock map are drift
        let mut p2 = test_support::set_time_packet(0, &[]);
        p2.push(0x00);
        assert!(parse_set_time(&p2).is_err());
        // a lying clock count must not reserve memory the packet cannot
        // back: each entry is >= 10 bytes on the wire, so a count far
        // above the remaining length is refused BEFORE allocation (the
        // mock builder cannot lie by construction — hand-rolled here)
        let mut p3 = Vec::new();
        write_varint(&mut p3, clientbound::SET_TIME);
        p3.extend_from_slice(&0i64.to_be_bytes());
        write_varint(&mut p3, 100_000); // claims 100k entries...
        // ...but zero entry bytes follow
        assert!(parse_set_time(&p3).is_err());
    }

    #[test]
    fn block_update_parses_the_packed_position_and_state() {
        assert_eq!(clientbound::BLOCK_UPDATE, 8, "block update (generated id)");
        // the positive-half plane and BOTH negative axes: the javap
        // packing `((x & 0x3FFFFFF) << 38) | (y & 0xFFF) |
        // ((z & 0x3FFFFFF) << 12)` sign-extends x/z from their 26-bit
        // fields and y from its 12-bit field — the arithmetic-shift walk
        let cases = [
            (48, 64, -32, 7u32),      // the mock chunk's corner column
            (-1, 70, -1, 1),          // the negative half-plane
            (0, -64, 0, 0),           // the overworld's floor, AIR
            (30_000_000 - 1, 319, 0, 9), // the 26-bit field's edge
            (0, 2_047, 0, 3),         // the 12-bit y field's top
        ];
        for (x, y, z, state) in cases {
            let p = test_support::block_update_packet(x, y, z, state);
            let (dx, dy, dz, ds) = parse_block_update(&p).unwrap();
            assert_eq!((dx, dy, dz, ds), (x, y, z, state), "round-trip at ({x},{y},{z})");
        }
        // trailing bytes are drift
        let mut p2 = test_support::block_update_packet(0, 0, 0, 0);
        p2.push(0x00);
        assert!(parse_block_update(&p2).is_err());
        // a negative VarInt state id is refused (a varint CAN carry a
        // negative through the sign bits; the registry ids cannot)
        let mut p3 = Vec::new();
        write_varint(&mut p3, clientbound::BLOCK_UPDATE);
        p3.extend_from_slice(&0i64.to_be_bytes());
        write_varint(&mut p3, -1);
        assert!(parse_block_update(&p3).is_err());
    }

    #[test]
    fn section_blocks_parse_the_packed_entries() {
        assert_eq!(clientbound::SECTION_BLOCKS_UPDATE, 84, "section blocks (generated id)");
        // a NEGATIVE section y (the overworld's floor sits at sy -4)
        // and entries across the 12-bit local packing — including the
        // (15, 15, 15) corner and a state id that forces a 2-byte VarLong
        let p = test_support::section_blocks_packet(
            3,
            -4,
            -2,
            &[
                ((0, 0, 0), 1),
                ((15, 15, 15), 0),
                ((7, 3, 12), 1234),
                ((1, 8, 2), 31),
            ],
        );
        let sb = parse_section_blocks(&p).unwrap();
        assert_eq!((sb.sx, sb.sy, sb.sz), (3, -4, -2), "the section long sign-extends");
        assert_eq!(sb.blocks.len(), 4);
        assert_eq!(sb.blocks[0], (0, 0, 0, 1));
        assert_eq!(sb.blocks[1], (15, 15, 15, 0), "the all-ones local unpacks to the corner");
        assert_eq!(sb.blocks[2], (7, 3, 12, 1234), "x@8, z@4, y@0; state = entry >>> 12");
        assert_eq!(sb.blocks[3], (1, 8, 2, 31));
        // a lying count is refused BEFORE allocation: each entry is at
        // least one wire byte, so a count above the remaining length
        // cannot be backed by the packet (the mock builder cannot lie
        // by construction — hand-rolled here)
        let mut p2 = Vec::new();
        write_varint(&mut p2, clientbound::SECTION_BLOCKS_UPDATE);
        p2.extend_from_slice(&0i64.to_be_bytes());
        write_varint(&mut p2, 5_000); // claims 5k entries...
        // ...but zero entry bytes follow
        assert!(parse_section_blocks(&p2).is_err());
        // trailing bytes are drift
        let mut p3 = test_support::section_blocks_packet(0, 0, 0, &[((0, 0, 0), 0)]);
        p3.push(0x00);
        assert!(parse_section_blocks(&p3).is_err());
    }

    #[test]
    fn add_entity_decodes_the_bytecode_layout() {
        assert_eq!(clientbound::ADD_ENTITY, 1, "add entity (generated id)");
        // the movement rides the 15-bit lane grid: at scale 1 the exact
        // points 1.0 / 0.0 / -1.0 pack to lanes 32766 / 16383 / 0 and
        // unpack BIT-EXACT (no tolerance — the arithmetic is exact at
        // the grid points)
        let movement = test_support::encode_lp_vec3(1.0, 0.0, -1.0, 1);
        // the angle bytes ride Mth.unpackDegrees: -128 * 360 / 256 =
        // exactly -180.0 (the byte field's floor), 127 * 360 / 256 =
        // 178.59375
        let p = test_support::add_entity_packet(
            99,
            0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF,
            2101, // a 2-byte VarInt type id
            -1234.5,
            64.0,
            9876.25,
            &movement,
            -128,
            127,
            0,
            7,
        );
        let e = parse_add_entity(&p).unwrap();
        assert_eq!(e.id, 99);
        assert_eq!(
            e.uuid, 0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF,
            "the UUID's 128 bits, most-long first"
        );
        assert_eq!(e.type_id, 2101);
        assert_eq!((e.x, e.y, e.z), (-1234.5, 64.0, 9876.25));
        assert_eq!(e.movement, [1.0, 0.0, -1.0], "the 15-bit lanes, bit-exact at the grid");
        assert_eq!(e.x_rot, -180.0, "-128 * 360 / 256");
        assert_eq!(e.y_rot, 178.59375, "127 * 360 / 256");
        assert_eq!(e.y_head_rot, 0.0);
        assert_eq!(e.data, 7);
        // trailing bytes are drift
        let mut p2 = p.clone();
        p2.push(0x00);
        assert!(parse_add_entity(&p2).is_err());
        // a wrong id is refused
        let mut p3 = test_support::add_entity_packet(
            1,
            0,
            0,
            0.0,
            0.0,
            0.0,
            &[0x00],
            0,
            0,
            0,
            0,
        );
        p3[0] = 0x7F; // 127 — not ADD_ENTITY
        assert!(parse_add_entity(&p3).is_err());
    }

    #[test]
    fn lp_vec3_decodes_the_lanes_scale_and_continuation() {
        // the ZERO sentinel: a zero FIRST byte ends the read — nothing
        // else is consumed (the 0xAA tail stays untouched)
        let mut c: &[u8] = &[0x00, 0xAA];
        assert_eq!(decode_lp_vec3(&mut c).unwrap(), [0.0; 3]);
        assert_eq!(c.len(), 1, "the sentinel consumed exactly one byte");
        // the continuation path: scale 100 exceeds the 2-bit field — the
        // flag bit rides byte 0 and the scale's high bits follow as one
        // VarInt (100 >> 2 = 25); the exact grid points unpack to
        // (100, 0, -100) bit-exact
        let p = test_support::encode_lp_vec3(100.0, 0.0, -100.0, 100);
        let mut c2: &[u8] = &p;
        let v = decode_lp_vec3(&mut c2).unwrap();
        assert_eq!(v, [100.0, 0.0, -100.0], "the continuation VarInt reassembles the scale");
        assert!(c2.is_empty(), "the whole encoding consumed");
        // the wire byte count: 6 fixed bytes + one VarInt continuation
        assert_eq!(p.len(), 7);
        // the general value round-trips within the 15-bit quantization
        // step (pack rounds to the nearest grid point; one step at
        // scale 3 is 3 * 2 / 32766)
        let p3 = test_support::encode_lp_vec3(1.5, -0.25, 2.25, 3);
        let v3 = decode_lp_vec3(&mut p3.as_slice()).unwrap();
        let step = 3.0 * 2.0 / 32766.0;
        assert!((v3[0] - 1.5).abs() <= step / 2.0 + 1e-12, "{} vs 1.5", v3[0]);
        assert!((v3[1] + 0.25).abs() <= step / 2.0 + 1e-12, "{} vs -0.25", v3[1]);
        assert!((v3[2] - 2.25).abs() <= step / 2.0 + 1e-12, "{} vs 2.25", v3[2]);
        // the 32767 lane value CLAMPS to 32766 (javap: Math.min) — a
        // hand-rolled packed integer proves the clamp, which the encoder
        // cannot produce (its lanes top out at 32766)
        let packed: u64 = 1
            | ((32767u64) << 3)   // x lane: the value the clamp exists for
            | ((16383u64) << 18)  // y lane: 0.0
            | ((16383u64) << 33); // z lane: 0.0
        let mut bytes = Vec::new();
        bytes.push((packed & 0xFF) as u8);
        bytes.push(((packed >> 8) & 0xFF) as u8);
        bytes.extend_from_slice(&(((packed >> 16) & 0xFFFF_FFFF) as u32).to_be_bytes());
        let v4 = decode_lp_vec3(&mut bytes.as_slice()).unwrap();
        assert_eq!(v4[0], 1.0, "32767 clamps to 32766 -> exactly 1.0");
        assert_eq!((v4[1], v4[2]), (0.0, 0.0));
    }

    #[test]
    fn remove_entities_parses_the_int_id_list() {
        assert_eq!(clientbound::REMOVE_ENTITIES, 77, "remove entities (generated id)");
        // readIntIdList: a VarInt count then that many VarInt ids —
        // including a 3-byte VarInt (1_000_000)
        let p = test_support::remove_entities_packet(&[7, 1_000_000, 42]);
        assert_eq!(parse_remove_entities(&p).unwrap(), vec![7, 1_000_000, 42]);
        // an empty batch is the honest zero
        assert_eq!(
            parse_remove_entities(&test_support::remove_entities_packet(&[])).unwrap(),
            Vec::<i32>::new()
        );
        // a lying count is refused BEFORE allocation (each id is at
        // least one wire byte — the mock builder cannot lie by
        // construction, hand-rolled here)
        let mut p2 = Vec::new();
        write_varint(&mut p2, clientbound::REMOVE_ENTITIES);
        write_varint(&mut p2, 5_000); // claims 5k ids...
        // ...but zero id bytes follow
        assert!(parse_remove_entities(&p2).is_err());
        // trailing bytes are drift
        let mut p3 = test_support::remove_entities_packet(&[1]);
        p3.push(0x00);
        assert!(parse_remove_entities(&p3).is_err());
    }

    #[test]
    fn entity_position_sync_decodes_the_position_move_rotation() {
        assert_eq!(
            clientbound::ENTITY_POSITION_SYNC, 35,
            "entity position sync (generated id)"
        );
        // the PositionMoveRotation record: TWO full-precision Vec3s
        // (position then deltaMovement) then FLOAT yaw, FLOAT pitch —
        // and the BOOL ground flag (one byte)
        let p = test_support::entity_position_sync_packet(
            99,
            -12.5,
            1000.25,
            -3.75,
            [0.1, -0.2, 0.3],
            179.5,
            -90.0,
            true,
        );
        let s = parse_entity_position_sync(&p).unwrap();
        assert_eq!(s.id, 99);
        assert_eq!((s.x, s.y, s.z), (-12.5, 1000.25, -3.75));
        assert_eq!(s.delta, [0.1, -0.2, 0.3]);
        assert_eq!((s.y_rot, s.x_rot), (179.5, -90.0));
        assert!(s.on_ground);
        // the ground flag is a real wire byte, not an omission
        let p2 = test_support::entity_position_sync_packet(
            99,
            -12.5,
            1000.25,
            -3.75,
            [0.1, -0.2, 0.3],
            179.5,
            -90.0,
            false,
        );
        assert!(!parse_entity_position_sync(&p2).unwrap().on_ground);
        // trailing bytes are drift
        let mut p3 = p.clone();
        p3.push(0x00);
        assert!(parse_entity_position_sync(&p3).is_err());
    }

    /// Move Entity Pos: the VarInt id + three RAW SHORT deltas + the
    /// BOOL, verbatim off the javap walk — negative deltas, both short
    /// boundaries, the ground flag's byte, trailing drift and the
    /// wrong id refused.
    #[test]
    fn move_entity_pos_deltas_decode_raw() {
        // the VarInt id, the three big-endian SHORTs and the BOOL —
        // exactly the javap read's five fields
        let p = test_support::move_entity_pos_packet(42, -1, 32767, -32768, true);
        let m = parse_move_entity_pos(&p).unwrap();
        assert_eq!(m.id, 42);
        assert_eq!((m.dx, m.dy, m.dz), (-1, 32767, -32768));
        assert!(m.on_ground);
        // the ground flag is a real wire byte, not an omission
        let p2 = test_support::move_entity_pos_packet(42, 0, 0, 0, false);
        assert!(!parse_move_entity_pos(&p2).unwrap().on_ground);
        assert_eq!(
            (parse_move_entity_pos(&p2).unwrap().dx, 0),
            (0, 0),
            "the all-zero payload is a legal packet"
        );
        // trailing bytes are drift
        let mut p3 = p.clone();
        p3.push(0x00);
        assert!(parse_move_entity_pos(&p3).is_err());
        // the wrong id is refused (the minecart's 55 or the rot's 56
        // must not pass the pos check)
        let mut p4 = p;
        p4[0] = clientbound::MOVE_ENTITY_ROT as u8;
        assert!(parse_move_entity_pos(&p4).is_err());
    }

    /// Move Entity Pos and Rotation: the Pos layout plus the TWO
    /// angle BYTES — the unpack is the Add Entity walk (the
    /// sign-extended byte's `b * 360 / 256`: -128 floors to -180.0,
    /// 127 lands at 178.59375), and the field order is yaw THEN pitch.
    #[test]
    fn move_entity_pos_rot_unpacks_the_angle_bytes() {
        let p = test_support::move_entity_pos_rot_packet(7, 16, -16, 0, -128, 127, false);
        let m = parse_move_entity_pos_rot(&p).unwrap();
        assert_eq!(m.id, 7);
        assert_eq!((m.dx, m.dy, m.dz), (16, -16, 0));
        assert_eq!(m.y_rot, -180.0, "the -128 byte floors to -180.0");
        assert_eq!(m.x_rot, 178.59375, "the 127 byte lands at 178.59375");
        assert!(!m.on_ground);
        // trailing bytes are drift
        let mut p2 = p.clone();
        p2.push(0x01);
        assert!(parse_move_entity_pos_rot(&p2).is_err());
    }

    /// Move Entity Rot: NO position lane — id, two angle bytes, BOOL.
    /// The generated id is 56; the wire's 55 is the minecart's.
    #[test]
    fn move_entity_rot_has_no_position_lane() {
        let p = test_support::move_entity_rot_packet(9, 64, -32, true);
        let m = parse_move_entity_rot(&p).unwrap();
        assert_eq!(m.id, 9);
        assert_eq!(m.y_rot, 90.0);
        assert_eq!(m.x_rot, -45.0);
        assert!(m.on_ground);
        // a truncated packet (missing the ground byte) is refused
        let p2 = &p[..p.len() - 1];
        assert!(parse_move_entity_rot(p2).is_err());
    }

    /// Rotate Head: the VarInt id and the ONE head-yaw byte (the
    /// private constructor IS the read), unpacked the vanilla way.
    #[test]
    fn rotate_head_is_one_byte_of_head_yaw() {
        let p = test_support::rotate_head_packet(11, -128);
        let h = parse_rotate_head(&p).unwrap();
        assert_eq!(h.id, 11);
        assert_eq!(h.y_head_rot, -180.0);
        // trailing bytes are drift
        let mut p2 = p.clone();
        p2.push(0x00);
        assert!(parse_rotate_head(&p2).is_err());
    }

    /// Set Entity Data: one packet, ALL EIGHTEEN supported
    /// serializers, distinctive values — the entry list round-trips
    /// bit-exact and the REGISTRATION-ORDER table is pinned (the
    /// wiki-era table would have decoded VILLAGER_DATA at 19 where the
    /// real wire carries the OptionalInt).
    #[test]
    fn the_entity_data_wire_round_trips_the_primitive_serializers() {
        use entity_data_serializers as s;
        let mut entries = Vec::new();
        entries.extend(test_support::entity_data_entry(0, s::BYTE, &[0xF7]));
        entries.extend(test_support::entity_data_entry(1, s::INT, &test_support::entity_data_varint(65536)));
        // the LONG serializer is ByteBufCodecs.VAR_LONG — a 7-bit-group
        // VarLong, NOT a big-endian long (42 rides one byte)
        entries.extend(test_support::entity_data_entry(2, s::LONG, &test_support::entity_data_varint(42)));
        entries.extend(test_support::entity_data_entry(3, s::FLOAT, &1.5f32.to_be_bytes()));
        let mut string_value = Vec::new();
        write_string(&mut string_value, "hi");
        entries.extend(test_support::entity_data_entry(4, s::STRING, &string_value));
        // the chat component: ONE bare NBT tag (a plain-text string)
        let mut tag = Vec::new();
        crate::nbt::write_bare_nbt(&Nbt::String("hi".into()), &mut tag).unwrap();
        entries.extend(test_support::entity_data_entry(5, s::COMPONENT, &tag));
        // the optional component, both shapes (flag false = one byte)
        entries.extend(test_support::entity_data_entry(6, s::OPTIONAL_COMPONENT, &[0]));
        let mut some_tag = vec![1u8];
        crate::nbt::write_bare_nbt(
            &Nbt::Compound(vec![("text".into(), Nbt::String("A".into()))]),
            &mut some_tag,
        )
        .unwrap();
        entries.extend(test_support::entity_data_entry(7, s::OPTIONAL_COMPONENT, &some_tag));
        entries.extend(test_support::entity_data_entry(8, s::BOOLEAN, &[1]));
        let mut rots = Vec::new();
        for v in [1.0f32, 2.0, 3.5] {
            rots.extend_from_slice(&v.to_be_bytes());
        }
        entries.extend(test_support::entity_data_entry(9, s::ROTATIONS, &rots));
        entries.extend(test_support::entity_data_entry(
            10,
            s::BLOCK_POS,
            &test_support::encode_block_pos(16, -64, -16).to_be_bytes(),
        ));
        // the optional lane, both shapes
        entries.extend(test_support::entity_data_entry(11, s::OPTIONAL_BLOCK_POS, &[0]));
        let mut some_pos = vec![1u8];
        some_pos.extend_from_slice(&test_support::encode_block_pos(-1, 320, -2).to_be_bytes());
        entries.extend(test_support::entity_data_entry(12, s::OPTIONAL_BLOCK_POS, &some_pos));
        entries.extend(test_support::entity_data_entry(13, s::DIRECTION, &test_support::entity_data_varint(5)));
        entries.extend(test_support::entity_data_entry(14, s::BLOCK_STATE, &test_support::entity_data_varint(12345)));
        entries.extend(test_support::entity_data_entry(15, s::OPTIONAL_BLOCK_STATE, &[0]));
        let mut some_state = vec![1u8];
        some_state.extend(test_support::entity_data_varint(42));
        entries.extend(test_support::entity_data_entry(16, s::OPTIONAL_BLOCK_STATE, &some_state));
        // the OptionalInt's BIAS-BY-ONE arithmetic: wire 0 = empty,
        // wire 2 = Some(1)
        entries.extend(test_support::entity_data_entry(17, s::OPTIONAL_UNSIGNED_INT, &test_support::entity_data_varint(0)));
        entries.extend(test_support::entity_data_entry(18, s::OPTIONAL_UNSIGNED_INT, &test_support::entity_data_varint(2)));
        entries.extend(test_support::entity_data_entry(19, s::POSE, &test_support::entity_data_varint(6)));
        // the villager record: THREE VarInts (type, profession, level)
        entries.extend(test_support::entity_data_entry(20, s::VILLAGER_DATA,
            &[test_support::entity_data_varint(2), test_support::entity_data_varint(14), test_support::entity_data_varint(3)].concat()));
        // the item stack: count, item registry id, ADDED count, REMOVED
        // count, then the added entries (damage VarInt + unbreakable the
        // ZERO-BYTE unit), then the removed entries (rarity) — the $3
        // layout is both counts FIRST, not count-entries-count-entries
        let mut stack_value = Vec::new();
        for v in [3i32, 745, 2, 1, 3, 35, 4, 12] {
            stack_value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        entries.extend(test_support::entity_data_entry(21, s::ITEM_STACK, &stack_value));
        // the second stack: the ADVENTURE-MODE pair rides the patch —
        // one added entry, can_place_on (14), TWO block predicates:
        // the first a NAMED set ("minecraft:logs") with one exact
        // property, the second a DIRECT set (wire tag 3 = two ids)
        // with a compound nbt lane — the four-lane walk in full
        let mut adv_value = Vec::new();
        for v in [1i32, 2, 1, 0] {
            adv_value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        adv_value.extend_from_slice(&test_support::entity_data_varint(data_component_types::CAN_PLACE_ON));
        adv_value.extend_from_slice(&test_support::entity_data_varint(2)); // predicate count
        // predicate 1: blocks present, tag 0 = named; properties
        // present, one {facing, exact "north"}; nbt absent; matchers
        // {exact [damage = 35], partial []}
        adv_value.push(1);
        adv_value.extend_from_slice(&test_support::entity_data_varint(0));
        write_string(&mut adv_value, "minecraft:logs");
        adv_value.push(1);
        adv_value.extend_from_slice(&test_support::entity_data_varint(1));
        write_string(&mut adv_value, "facing");
        adv_value.push(1); // either: LEFT = exact
        write_string(&mut adv_value, "north");
        adv_value.push(0); // nbt absent
        adv_value.extend_from_slice(&test_support::entity_data_varint(1)); // exact count
        adv_value.extend_from_slice(&test_support::entity_data_varint(3)); // damage
        adv_value.extend_from_slice(&test_support::entity_data_varint(35));
        adv_value.extend_from_slice(&test_support::entity_data_varint(0)); // partial empty
        // predicate 2: blocks present, tag 3 = direct {17, 999};
        // properties absent; nbt present (compound); matchers empty
        adv_value.push(1);
        adv_value.extend_from_slice(&test_support::entity_data_varint(3));
        adv_value.extend_from_slice(&test_support::entity_data_varint(17));
        adv_value.extend_from_slice(&test_support::entity_data_varint(999));
        adv_value.push(0); // properties absent
        adv_value.push(1); // nbt present
        crate::nbt::write_bare_nbt(
            &Nbt::Compound(vec![("tag".into(), Nbt::Byte(1))]),
            &mut adv_value,
        )
        .unwrap();
        adv_value.extend_from_slice(&test_support::entity_data_varint(0)); // exact empty
        adv_value.extend_from_slice(&test_support::entity_data_varint(0)); // partial empty
        entries.extend(test_support::entity_data_entry(22, s::ITEM_STACK, &adv_value));

        let p = test_support::set_entity_data_packet(77, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(d.id, 77);
        // THE WRITE MIRROR: every entry here re-encodes byte-for-byte —
        // the writer family's first whole-packet proof, riding the
        // fixture set the javap audits built (both item stacks ride
        // the deep patch writers: the added/removed lanes and the
        // adventure-mode predicates with their matchers)
        assert_eq!(
            encode_set_entity_data(d.id, &d.entries).unwrap(),
            p
        );
        assert_eq!(
            d.entries,
            vec![
                SetEntityDataEntry { index: 0, serializer: s::BYTE, value: EntityDataEntry::Byte(-9) },
                SetEntityDataEntry { index: 1, serializer: s::INT, value: EntityDataEntry::Int(65536) },
                SetEntityDataEntry { index: 2, serializer: s::LONG, value: EntityDataEntry::Long(42) },
                SetEntityDataEntry { index: 3, serializer: s::FLOAT, value: EntityDataEntry::Float(1.5) },
                SetEntityDataEntry { index: 4, serializer: s::STRING, value: EntityDataEntry::String("hi".into()) },
                SetEntityDataEntry { index: 5, serializer: s::COMPONENT, value: EntityDataEntry::Component(Nbt::String("hi".into())) },
                SetEntityDataEntry { index: 6, serializer: s::OPTIONAL_COMPONENT, value: EntityDataEntry::OptionalComponent(None) },
                SetEntityDataEntry { index: 7, serializer: s::OPTIONAL_COMPONENT, value: EntityDataEntry::OptionalComponent(Some(Nbt::Compound(vec![("text".into(), Nbt::String("A".into()))]))) },
                SetEntityDataEntry { index: 8, serializer: s::BOOLEAN, value: EntityDataEntry::Bool(true) },
                SetEntityDataEntry { index: 9, serializer: s::ROTATIONS, value: EntityDataEntry::Rotations([1.0, 2.0, 3.5]) },
                SetEntityDataEntry { index: 10, serializer: s::BLOCK_POS, value: EntityDataEntry::BlockPos([16, -64, -16]) },
                SetEntityDataEntry { index: 11, serializer: s::OPTIONAL_BLOCK_POS, value: EntityDataEntry::OptionalBlockPos(None) },
                SetEntityDataEntry { index: 12, serializer: s::OPTIONAL_BLOCK_POS, value: EntityDataEntry::OptionalBlockPos(Some([-1, 320, -2])) },
                SetEntityDataEntry { index: 13, serializer: s::DIRECTION, value: EntityDataEntry::Direction(5) },
                SetEntityDataEntry { index: 14, serializer: s::BLOCK_STATE, value: EntityDataEntry::BlockState(12345) },
                SetEntityDataEntry { index: 15, serializer: s::OPTIONAL_BLOCK_STATE, value: EntityDataEntry::OptionalBlockState(None) },
                SetEntityDataEntry { index: 16, serializer: s::OPTIONAL_BLOCK_STATE, value: EntityDataEntry::OptionalBlockState(Some(42)) },
                SetEntityDataEntry { index: 17, serializer: s::OPTIONAL_UNSIGNED_INT, value: EntityDataEntry::OptionalUnsignedInt(None) },
                SetEntityDataEntry { index: 18, serializer: s::OPTIONAL_UNSIGNED_INT, value: EntityDataEntry::OptionalUnsignedInt(Some(1)) },
                SetEntityDataEntry { index: 19, serializer: s::POSE, value: EntityDataEntry::Pose(6) },
                SetEntityDataEntry { index: 20, serializer: s::VILLAGER_DATA, value: EntityDataEntry::VillagerData { type_id: 2, profession_id: 14, level: 3 } },
                SetEntityDataEntry { index: 21, serializer: s::ITEM_STACK, value: EntityDataEntry::ItemStack { count: 3, item_id: 745, patch: ComponentPatch { added: vec![(3, ComponentValue::VarInt(35)), (4, ComponentValue::Unit)], removed: vec![12] } } },
                SetEntityDataEntry { index: 22, serializer: s::ITEM_STACK, value: EntityDataEntry::ItemStack { count: 1, item_id: 2, patch: ComponentPatch { added: vec![(data_component_types::CAN_PLACE_ON, ComponentValue::BlockPredicates(vec![
                    BlockPredicate { blocks: Some(BlockSet::Named("minecraft:logs".into())), properties: Some(vec![PropertyMatcher { name: "facing".into(), value: PropertyValue::Exact("north".into()) }]), nbt: None, matchers: DataComponentMatchers { exact: vec![(3, ComponentValue::VarInt(35))], partial: vec![] } },
                    BlockPredicate { blocks: Some(BlockSet::Direct(vec![17, 999])), properties: None, nbt: Some(Nbt::Compound(vec![("tag".into(), Nbt::Byte(1))])), matchers: DataComponentMatchers { exact: vec![], partial: vec![] } },
                ]))], removed: vec![] } } },
            ]
        );
    }


    /// Set Entity Data: the 0xFF index is the EOF marker — an empty
    /// list is legal, and ANY bytes after the marker are drift (the
    /// vanilla reader stops at the marker; a well-formed packet ends).
    #[test]
    fn the_entity_data_eof_marker_ends_the_list_and_refuses_drift() {
        // id + marker: the empty set
        let p = test_support::set_entity_data_packet(9, &[]);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(d.id, 9);
        assert!(d.entries.is_empty());
        // trailing bytes after the marker never reach the model
        let mut p2 = p.clone();
        p2.extend_from_slice(&[0x01, 0x02]);
        assert!(parse_set_entity_data(&p2).is_err());
        // a truncated entry (index + serializer, no value) is refused
        let mut truncated = Vec::new();
        write_varint(&mut truncated, 9);
        truncated.extend_from_slice(&test_support::entity_data_entry(3, 3, &[]));
        truncated.push(255);
        assert!(parse_set_entity_data(&truncated).is_err());
    }

    /// The write side's pinned lanes — the three mirrors the javap
    /// audits named, each proven against the vanilla encode bytecode:
    ///
    /// 1. OPTIONAL_UNSIGNED_INT (EntityDataSerializers$3.encode):
    ///    `optional.orElse(-1) + 1` — the iadd is Java's WRAPPING int
    ///    add, so None rides the VarInt 0 and Some(MAX_VALUE) rides
    ///    MIN_VALUE (the VarInt 80 80 80 80 08 on the wire); the
    ///    decode's wrapping_sub(1) closes the loop on BOTH ends.
    /// 2. ITEM_STACK (ItemStack$1.encode / the $2 strict wrapper): the
    ///    count rides FIRST and a non-positive count is the WHOLE
    ///    payload — the retained -7 sentinel re-encodes as -7 (five
    ///    VarInt bytes, no item, no patch), where vanilla's object
    ///    graph would have collapsed the stack to EMPTY and written 0;
    ///    the port writes the WIRE's truth, not the collapse.
    /// 3. OPTIONAL_LIVING_ENTITY_REFERENCE: the flag byte, then — only
    ///    then — the 16 UUID bytes; an absent reference is the flag
    ///    byte alone.
    /// And the writer's own honest refusal: an entry wearing the 0xFF
    /// EOF marker as its index is refused, not written (the packet
    /// would truncate the walk the port itself refuses to parse past).
    #[test]
    fn the_write_side_pins_the_encode_lanes_the_audits_named() {
        use entity_data_serializers as s;
        // 1. the OptionalInt's wrapping +1
        let mk = |v: EntityDataEntry, ser: i32| {
            SetEntityDataEntry {
                index: 4,
                serializer: ser,
                value: v,
            }
        };
        let none = encode_set_entity_data(
            7,
            &[mk(EntityDataEntry::OptionalUnsignedInt(None), s::OPTIONAL_UNSIGNED_INT)],
        )
        .unwrap();
        let d = parse_set_entity_data(&none).unwrap();
        assert_eq!(d.entries[0].value, EntityDataEntry::OptionalUnsignedInt(None));
        // the value byte IS the VarInt 0: the packet is id + [04 10 00 FF]
        assert_eq!(none.last(), Some(&255));
        assert_eq!(&none[none.len() - 4..none.len() - 1], &[4, s::OPTIONAL_UNSIGNED_INT as u8, 0]);
        let wrapped = encode_set_entity_data(
            7,
            &[mk(
                EntityDataEntry::OptionalUnsignedInt(Some(i32::MAX)),
                s::OPTIONAL_UNSIGNED_INT,
            )],
        )
        .unwrap();
        // Some(MAX_VALUE) rides MIN_VALUE: the VarInt 80 80 80 80 08
        // (MIN_VALUE's u32 bits, LEB128 — NOT the FF..0F of a -1)
        assert_eq!(
            &wrapped[wrapped.len() - 8..wrapped.len() - 1],
            &[
                4,
                s::OPTIONAL_UNSIGNED_INT as u8,
                0x80,
                0x80,
                0x80,
                0x80,
                0x08
            ]
        );
        let d = parse_set_entity_data(&wrapped).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::OptionalUnsignedInt(Some(i32::MAX))
        );
        // 2. the item stack's count sentinel: the retained -7 re-encodes
        //    as -7 and NOTHING ELSE rides (no item id, no patch)
        let sentinel = encode_set_entity_data(
            7,
            &[mk(
                EntityDataEntry::ItemStack {
                    count: -7,
                    item_id: 0,
                    patch: ComponentPatch::default(),
                },
                s::ITEM_STACK,
            )],
        )
        .unwrap();
        assert_eq!(
            &sentinel[sentinel.len() - 8..sentinel.len() - 1],
            &[4, s::ITEM_STACK as u8, 0xF9, 0xFF, 0xFF, 0xFF, 0x0F]
        );
        let d = parse_set_entity_data(&sentinel).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ItemStack {
                count: -7,
                item_id: 0,
                patch: ComponentPatch::default()
            }
        );
        // ...and a byte-equal round-trip: parse -> encode == the wire
        assert_eq!(encode_set_entity_data(d.id, &d.entries).unwrap(), sentinel);
        // 3. the entity reference's flag byte: absent = one byte,
        //    present = the flag THEN the 16 UUID bytes
        let absent = encode_set_entity_data(
            7,
            &[mk(EntityDataEntry::OptionalLivingEntityReference(None), s::OPTIONAL_LIVING_ENTITY_REFERENCE)],
        )
        .unwrap();
        assert_eq!(absent[absent.len() - 2], 0); // the flag byte alone
        let uuid: [u8; 16] = core::array::from_fn(|i| i as u8);
        let present = encode_set_entity_data(
            7,
            &[mk(
                EntityDataEntry::OptionalLivingEntityReference(Some(EntityReference { uuid })),
                s::OPTIONAL_LIVING_ENTITY_REFERENCE,
            )],
        )
        .unwrap();
        assert_eq!(present[present.len() - 18], 1); // the flag rides FIRST
        assert_eq!(&present[present.len() - 17..present.len() - 1], &uuid); // then the 16 bytes
        let d = parse_set_entity_data(&present).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::OptionalLivingEntityReference(Some(EntityReference { uuid }))
        );
        // the writer's own refusal: the EOF marker is not an accessor
        assert!(encode_set_entity_data(
            7,
            &[SetEntityDataEntry {
                index: 255,
                serializer: s::BYTE,
                value: EntityDataEntry::Byte(1),
            }],
        )
        .is_err());
    }

    /// Set Entity Data: an id the registry itself does not know
    /// refuses the packet — the vanilla read side fails the packet
    /// exactly the same way (an unknown serializer id throws in
    /// `getSerializer`). The refusal list's RETIREMENTS are the
    /// decode walk's own history, each with its round: the villager
    /// record left the round the composite's three VarInts landed
    /// (the registration-order proof stays in the body: 18 is NOT
    /// the optional-global-pos lane the wiki-era table claims — 33
    /// is, and the loop's id order is the wire's). COMPONENT left
    /// the round the bare-NBT reader landed. ITEM_STACK left the
    /// round the three-layer count/item/patch decode landed.
    /// PARTICLE left the round the dispatched particle-type registry
    /// walked (its 0x00 byte was the probe; now id 0 is a legal
    /// SIMPLE particle). RESOLVABLE_PROFILE left the round its
    /// either+patch walk landed. The variant holders — the LAST
    /// object-graph family — left the round the holderRegistry
    /// census walked: id 21's 0x00 probe byte is now a legal
    /// cat-variant registry id, so the survivor list is the unknown
    /// ids alone, refusing for the reason they always did: a value's
    /// byte length is only knowable through its codec, so there is
    /// no honest skip.
    #[test]
    fn the_entity_data_object_graph_and_unknown_serializers_refuse() {
        use entity_data_serializers as s;
        for serializer in [200] {
            let entries = test_support::entity_data_entry(0, serializer, &[0x00]);
            let p = test_support::set_entity_data_packet(3, &entries);
            assert!(
                parse_set_entity_data(&p).is_err(),
                "serializer {serializer} must refuse, not skip"
            );
        }
        // ...and the id on the other side of the wiki-era swap decodes
        // as the OptionalInt the registration order says it is
        let entries = test_support::entity_data_entry(0, s::OPTIONAL_UNSIGNED_INT, &[1]);
        let p = test_support::set_entity_data_packet(3, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(d.entries[0].value, EntityDataEntry::OptionalUnsignedInt(Some(0)));
    }

    /// The variant-holder census — the walk's LAST refusals retired
    /// and the registry walk COMPLETE (all 43 serializers, ids
    /// 0-42): one packet riding every new lane. The twelve holders
    /// (ids 21-32) are the bias-zero `holderRegistry` VarInts,
    /// retained verbatim (the villager line — no variant-registry
    /// tables client-side); the painting (34) is the family's ONLY
    /// bias-by-one member (wire 0 the INLINE record, wire n the
    /// registry id n - 1); the global pos (33) rides the #149-bsm
    /// flag byte with the dimension as an identifier STRING and the
    /// packed position long; the enum states (35-38, 42) are
    /// never-refusing ordinals retained verbatim (three ZERO
    /// strategies and the weathering-copper CLAMP — the clamp is the
    /// consumer's act, so a wire 999 is a stored 999); VECTOR3 and
    /// QUATERNION are the readVector3f/readQuaternion float layouts
    /// (the quaternion in the JOML x, y, z, w order).
    #[test]
    fn the_variant_holder_lanes_complete_the_registry_walk() {
        use entity_data_serializers as s;
        let holders = [
            s::CAT_VARIANT,
            s::CAT_SOUND_VARIANT,
            s::COW_VARIANT,
            s::COW_SOUND_VARIANT,
            s::WOLF_VARIANT,
            s::WOLF_SOUND_VARIANT,
            s::FROG_VARIANT,
            s::PIG_VARIANT,
            s::PIG_SOUND_VARIANT,
            s::CHICKEN_VARIANT,
            s::CHICKEN_SOUND_VARIANT,
            s::ZOMBIE_NAUTILUS_VARIANT,
        ];
        let mut entries: Vec<u8> = Vec::new();
        // the twelve holders, each lane keeping its own wire id
        // (0..=11 — the serializer id is part of the state)
        for (i, ser) in holders.iter().enumerate() {
            entries.extend(test_support::entity_data_entry(
                i as u8,
                *ser,
                &test_support::entity_data_varint(i as i32),
            ));
        }
        // the global pos, PRESENT: the flag, the dimension string,
        // the packed position (1, 64, -2) — 0x7FFFFFE040 packed the
        // BlockPos way (x in bits 38+, z in 12..38, y in 0..12)
        let mut gp = vec![1u8];
        write_string(&mut gp, "minecraft:overworld");
        gp.extend_from_slice(&0x7FFFFFE040i64.to_be_bytes());
        entries.extend(test_support::entity_data_entry(
            12,
            s::OPTIONAL_GLOBAL_POS,
            &gp,
        ));
        // ...and ABSENT: the flag byte alone
        entries.extend(test_support::entity_data_entry(
            13,
            s::OPTIONAL_GLOBAL_POS,
            &[0],
        ));
        // the painting's DIRECT record (wire 0): width, height, the
        // asset string, the title present (a bare TAG_String), the
        // author absent
        let mut pv = test_support::entity_data_varint(0);
        pv.extend(test_support::entity_data_varint(16)); // width
        pv.extend(test_support::entity_data_varint(16)); // height
        write_string(&mut pv, "minecraft:wanderer"); // assetId
        pv.push(1); // title present
        crate::nbt::write_bare_nbt(&Nbt::String("Musee".into()), &mut pv).unwrap();
        pv.push(0); // author absent
        entries.extend(test_support::entity_data_entry(
            14,
            s::PAINTING_VARIANT,
            &pv,
        ));
        // the painting's REGISTRY reference (wire 5 -> id 4): the
        // bias-by-one pin, the $30's `byIdOrThrow(n - 1)`
        entries.extend(test_support::entity_data_entry(
            15,
            s::PAINTING_VARIANT,
            &test_support::entity_data_varint(5),
        ));
        // the enum states: one named state per lane, the ordinals
        // riding verbatim
        entries.extend(test_support::entity_data_entry(
            16,
            s::SNIFFER_STATE,
            &test_support::entity_data_varint(6), // RISING
        ));
        entries.extend(test_support::entity_data_entry(
            17,
            s::ARMADILLO_STATE,
            &test_support::entity_data_varint(2), // SCARED
        ));
        entries.extend(test_support::entity_data_entry(
            18,
            s::COPPER_GOLEM_STATE,
            &test_support::entity_data_varint(4), // DROPPING_NO_ITEM
        ));
        entries.extend(test_support::entity_data_entry(
            19,
            s::WEATHERING_COPPER_STATE,
            &test_support::entity_data_varint(3), // OXIDIZED
        ));
        // the float lanes: VECTOR3 (12 bytes) and QUATERNION (16
        // bytes, x y z w)
        let mut v3 = Vec::new();
        v3.extend_from_slice(&1.0f32.to_be_bytes());
        v3.extend_from_slice(&(-2.5f32).to_be_bytes());
        v3.extend_from_slice(&3.25f32.to_be_bytes());
        entries.extend(test_support::entity_data_entry(20, s::VECTOR3, &v3));
        let mut q = Vec::new();
        for f in [0.5f32, -0.5, 0.5, 0.5] {
            q.extend_from_slice(&f.to_be_bytes());
        }
        entries.extend(test_support::entity_data_entry(21, s::QUATERNION, &q));
        // the main hand: RIGHT = 1 (the declaration order's truth)
        entries.extend(test_support::entity_data_entry(
            22,
            s::HUMANOID_ARM,
            &test_support::entity_data_varint(1),
        ));
        // the retention pins: the ZERO strategy never refuses (a
        // wire 99 sniffer state is a stored 99) and the CLAMP
        // strategy's map is the consumer's act (a wire 999
        // weathering state is a stored 999, not a clamped 3)
        entries.extend(test_support::entity_data_entry(
            23,
            s::SNIFFER_STATE,
            &test_support::entity_data_varint(99),
        ));
        entries.extend(test_support::entity_data_entry(
            24,
            s::WEATHERING_COPPER_STATE,
            &test_support::entity_data_varint(999),
        ));

        let p = test_support::set_entity_data_packet(88, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(d.id, 88);
        // the write mirror holds here too: the holders, the painting's
        // two bias lanes, the global pos, the states, the float lanes
        assert_eq!(
            encode_set_entity_data(d.id, &d.entries).unwrap(),
            p
        );
        assert_eq!(
            d.entries,
            vec![
                SetEntityDataEntry { index: 0, serializer: s::CAT_VARIANT, value: EntityDataEntry::CatVariant(0) },
                SetEntityDataEntry { index: 1, serializer: s::CAT_SOUND_VARIANT, value: EntityDataEntry::CatSoundVariant(1) },
                SetEntityDataEntry { index: 2, serializer: s::COW_VARIANT, value: EntityDataEntry::CowVariant(2) },
                SetEntityDataEntry { index: 3, serializer: s::COW_SOUND_VARIANT, value: EntityDataEntry::CowSoundVariant(3) },
                SetEntityDataEntry { index: 4, serializer: s::WOLF_VARIANT, value: EntityDataEntry::WolfVariant(4) },
                SetEntityDataEntry { index: 5, serializer: s::WOLF_SOUND_VARIANT, value: EntityDataEntry::WolfSoundVariant(5) },
                SetEntityDataEntry { index: 6, serializer: s::FROG_VARIANT, value: EntityDataEntry::FrogVariant(6) },
                SetEntityDataEntry { index: 7, serializer: s::PIG_VARIANT, value: EntityDataEntry::PigVariant(7) },
                SetEntityDataEntry { index: 8, serializer: s::PIG_SOUND_VARIANT, value: EntityDataEntry::PigSoundVariant(8) },
                SetEntityDataEntry { index: 9, serializer: s::CHICKEN_VARIANT, value: EntityDataEntry::ChickenVariant(9) },
                SetEntityDataEntry { index: 10, serializer: s::CHICKEN_SOUND_VARIANT, value: EntityDataEntry::ChickenSoundVariant(10) },
                SetEntityDataEntry { index: 11, serializer: s::ZOMBIE_NAUTILUS_VARIANT, value: EntityDataEntry::ZombieNautilusVariant(11) },
                SetEntityDataEntry { index: 12, serializer: s::OPTIONAL_GLOBAL_POS, value: EntityDataEntry::OptionalGlobalPos(Some(GlobalPos { dimension: "minecraft:overworld".into(), pos: [1, 64, -2] })) },
                SetEntityDataEntry { index: 13, serializer: s::OPTIONAL_GLOBAL_POS, value: EntityDataEntry::OptionalGlobalPos(None) },
                SetEntityDataEntry { index: 14, serializer: s::PAINTING_VARIANT, value: EntityDataEntry::PaintingVariant(PaintingVariant::Direct { width: 16, height: 16, asset_id: "minecraft:wanderer".into(), title: Some(Nbt::String("Musee".into())), author: None }) },
                SetEntityDataEntry { index: 15, serializer: s::PAINTING_VARIANT, value: EntityDataEntry::PaintingVariant(PaintingVariant::Registry(4)) },
                SetEntityDataEntry { index: 16, serializer: s::SNIFFER_STATE, value: EntityDataEntry::SnifferState(6) },
                SetEntityDataEntry { index: 17, serializer: s::ARMADILLO_STATE, value: EntityDataEntry::ArmadilloState(2) },
                SetEntityDataEntry { index: 18, serializer: s::COPPER_GOLEM_STATE, value: EntityDataEntry::CopperGolemState(4) },
                SetEntityDataEntry { index: 19, serializer: s::WEATHERING_COPPER_STATE, value: EntityDataEntry::WeatheringCopperState(3) },
                SetEntityDataEntry { index: 20, serializer: s::VECTOR3, value: EntityDataEntry::Vector3([1.0, -2.5, 3.25]) },
                SetEntityDataEntry { index: 21, serializer: s::QUATERNION, value: EntityDataEntry::Quaternion([0.5, -0.5, 0.5, 0.5]) },
                SetEntityDataEntry { index: 22, serializer: s::HUMANOID_ARM, value: EntityDataEntry::HumanoidArm(1) },
                SetEntityDataEntry { index: 23, serializer: s::SNIFFER_STATE, value: EntityDataEntry::SnifferState(99) },
                SetEntityDataEntry { index: 24, serializer: s::WEATHERING_COPPER_STATE, value: EntityDataEntry::WeatheringCopperState(999) },
            ]
        );

        // the negative half of the holder lanes refuses — the
        // byIdOrThrow direction, the port's arithmetic-only check
        for ser in [s::WOLF_VARIANT, s::ZOMBIE_NAUTILUS_VARIANT, s::PAINTING_VARIANT] {
            let e = test_support::entity_data_entry(
                0,
                ser,
                &test_support::entity_data_varint(-1),
            );
            let p = test_support::set_entity_data_packet(89, &e);
            assert!(
                parse_set_entity_data(&p).is_err(),
                "a negative id on serializer {ser} must refuse"
            );
        }
        // the truncated quaternion is the honest short-buffer failure
        let e = test_support::entity_data_entry(0, s::QUATERNION, &[0u8; 10]);
        let p = test_support::set_entity_data_packet(89, &e);
        assert!(parse_set_entity_data(&p).is_err());
        // the truncated direct painting: the title's flag byte with
        // no tag behind it refuses the same way
        let mut trunc = test_support::entity_data_varint(0);
        trunc.extend(test_support::entity_data_varint(16));
        trunc.extend(test_support::entity_data_varint(16));
        write_string(&mut trunc, "minecraft:wanderer");
        trunc.push(1); // title flag present, no tag follows
        let e = test_support::entity_data_entry(0, s::PAINTING_VARIANT, &trunc);
        let p = test_support::set_entity_data_packet(89, &e);
        assert!(parse_set_entity_data(&p).is_err());
    }

    /// The OptionalInt lane's SIGNED truth (the $3 arithmetic): the
    /// decode `OptionalInt.of(n - 1)` never refuses — a negative
    /// wire n stores a NEGATIVE value (wire -5 is Some(-6)), and the
    /// i32 wrap is the isub's two's-complement truth (wire MIN_VALUE
    /// stores MAX_VALUE; Rust's plain subtraction would panic in
    /// debug — wrapping_sub is the honest mirror). The encode mirror
    /// is `optional.orElse(-1) + 1` (empty rides 0, present
    /// MAX_VALUE rides MIN_VALUE). The delivered lane refused the
    /// negative half for twelve versions — a strictness the
    /// bytecode does not have, unpinned by any test and therefore
    /// invisible (the autopush-114 deferral's arithmetic-codec
    /// member, now retired).
    #[test]
    fn the_optional_int_lane_retains_the_signed_truth() {
        use entity_data_serializers as s;
        // wire -5 -> Some(-6): the negative half the old lane refused
        let entries = test_support::entity_data_entry(
            0,
            s::OPTIONAL_UNSIGNED_INT,
            &test_support::entity_data_varint(-5),
        );
        let p = test_support::set_entity_data_packet(4, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::OptionalUnsignedInt(Some(-6))
        );
        // the wrap edge: wire i32::MIN isub 1 is i32::MAX
        let entries = test_support::entity_data_entry(
            0,
            s::OPTIONAL_UNSIGNED_INT,
            &test_support::entity_data_varint(i32::MIN),
        );
        let p = test_support::set_entity_data_packet(4, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::OptionalUnsignedInt(Some(i32::MAX))
        );
    }

    /// The living-entity reference lane (serializer 13), retired
    /// from the object-graph refusal set: the wire is the optional
    /// operation's flag byte, then — if present — the referenced
    /// entity's UUID as 16 raw bytes (msb long then lsb long, both
    /// big-endian, `FriendlyByteBuf.readUUID`'s two readLongs). The
    /// resolution (which living entity the UUID names) is never on
    /// the wire — vanilla's `Either<UUID, StoredEntityType>` rides
    /// the LEFT side only — so the port retains the UUID verbatim
    /// and the consumer resolves. A truncated UUID (fewer than 16
    /// bytes after the flag) refuses: the byte length is knowable
    /// through the codec, and the vanilla read side fails the packet
    /// the same way on a short buffer.
    #[test]
    fn the_living_entity_reference_lane_retains_the_uuid() {
        use entity_data_serializers as s;
        // the empty lane: the flag byte alone
        let entries = test_support::entity_data_entry(0, s::OPTIONAL_LIVING_ENTITY_REFERENCE, &[0]);
        let p = test_support::set_entity_data_packet(6, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::OptionalLivingEntityReference(None)
        );
        // the present lane: 16 distinctive bytes ride verbatim
        let mut v = vec![1u8];
        v.extend_from_slice(&[
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]);
        let entries = test_support::entity_data_entry(0, s::OPTIONAL_LIVING_ENTITY_REFERENCE, &v);
        let p = test_support::set_entity_data_packet(6, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::OptionalLivingEntityReference(Some(EntityReference {
                uuid: [
                    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
                    0xdd, 0xee, 0xff
                ]
            }))
        );
        // a truncated UUID refuses — the honest short-buffer failure
        let entries =
            test_support::entity_data_entry(0, s::OPTIONAL_LIVING_ENTITY_REFERENCE, &[1, 0x00, 0x11]);
        let p = test_support::set_entity_data_packet(6, &entries);
        assert!(parse_set_entity_data(&p).is_err());
    }

    /// The PARTICLE serializer (16): `ParticleTypes.STREAM_CODEC`'s
    /// dispatched registry walk — the VarInt id (the $29 byIdOrThrow
    /// family), then the type's OWN payload codec. One packet, the
    /// payload shapes' representatives: a SIMPLE type (zero payload —
    /// the `StreamCodec.unit` lane), the block composite, both geyser
    /// shapes, the power float, dust, the dust transition, the spell,
    /// the color, the item template (item id + RAW count + patch —
    /// no count-sentinel here, the ItemStack$1 line does not apply),
    /// the vibration (its own nested dispatch: the block position
    /// source), the trail (three big-endian f64 + color + duration),
    /// shriek's delay, and the sculk charge's roll.
    #[test]
    fn the_particle_serializer_walks_the_dispatched_registry() {
        use entity_data_serializers as s;
        use particle_types as pt;
        // the dispatch id + the payload bytes, one Vec
        let pv = |id: i32, rest: &[u8]| -> Vec<u8> {
            let mut b = test_support::entity_data_varint(id);
            b.extend_from_slice(rest);
            b
        };
        let mut entries = Vec::new();
        // simple: angry_villager (0) rides ZERO payload bytes
        entries.extend(test_support::entity_data_entry(0, s::PARTICLE, &test_support::entity_data_varint(0)));
        // block (1): the block-state VarInt
        entries.extend(test_support::entity_data_entry(1, s::PARTICLE, &pv(pt::BLOCK, &test_support::entity_data_varint(12345))));
        // geyser (7): the waterBlocks INT
        entries.extend(test_support::entity_data_entry(2, s::PARTICLE, &pv(pt::GEYSER, &2i32.to_be_bytes())));
        // geyser_base (8): waterBlocks + burstImpulseBase
        entries.extend(test_support::entity_data_entry(3, s::PARTICLE, &pv(pt::GEYSER_BASE, &[2i32.to_be_bytes().as_slice(), 0.5f32.to_be_bytes().as_slice()].concat())));
        // dragon_breath (15): the power FLOAT
        entries.extend(test_support::entity_data_entry(4, s::PARTICLE, &pv(pt::DRAGON_BREATH, &1.5f32.to_be_bytes())));
        // dust (21): color argb + scale
        entries.extend(test_support::entity_data_entry(5, s::PARTICLE, &pv(pt::DUST, &[16711680i32.to_be_bytes().as_slice(), 1.0f32.to_be_bytes().as_slice()].concat())));
        // dust_color_transition (22): from + to + scale
        entries.extend(test_support::entity_data_entry(6, s::PARTICLE, &pv(pt::DUST_COLOR_TRANSITION, &[16711680i32.to_be_bytes().as_slice(), 3790560i32.to_be_bytes().as_slice(), 1.0f32.to_be_bytes().as_slice()].concat())));
        // effect (23): the spell's color + power
        entries.extend(test_support::entity_data_entry(7, s::PARTICLE, &pv(pt::EFFECT, &[16777215i32.to_be_bytes().as_slice(), 1.0f32.to_be_bytes().as_slice()].concat())));
        // entity_effect (28): the color INT
        entries.extend(test_support::entity_data_entry(8, s::PARTICLE, &pv(pt::ENTITY_EFFECT, &65280i32.to_be_bytes())));
        // sculk_charge (45): the roll FLOAT
        entries.extend(test_support::entity_data_entry(9, s::PARTICLE, &pv(pt::SCULK_CHARGE, &0.25f32.to_be_bytes())));
        // item (54): the template's item id + RAW count + patch
        // (one added VarInt(35) at component type 3, nothing removed)
        entries.extend(test_support::entity_data_entry(10, s::PARTICLE, &pv(pt::ITEM, &[
            test_support::entity_data_varint(745), test_support::entity_data_varint(3),
            test_support::entity_data_varint(1), test_support::entity_data_varint(0),
            test_support::entity_data_varint(3), test_support::entity_data_varint(35),
        ].concat())));
        // vibration (55): the nested dispatch — a BLOCK position
        // source (0) carrying the packed pos, then the arrival ticks
        entries.extend(test_support::entity_data_entry(11, s::PARTICLE, &pv(pt::VIBRATION, &[
            test_support::entity_data_varint(0), test_support::encode_block_pos(-1, 64, 2).to_be_bytes().to_vec(),
            test_support::entity_data_varint(20),
        ].concat())));
        // trail (56): three big-endian f64, the color INT, the duration
        entries.extend(test_support::entity_data_entry(12, s::PARTICLE, &pv(pt::TRAIL, &[
            1.0f64.to_be_bytes().to_vec(), 2.0f64.to_be_bytes().to_vec(),
            3.0f64.to_be_bytes().to_vec(), 7i32.to_be_bytes().to_vec(),
            test_support::entity_data_varint(9),
        ].concat())));
        // shriek (112): the delay VarInt
        entries.extend(test_support::entity_data_entry(13, s::PARTICLE, &pv(pt::SHRIEK, &test_support::entity_data_varint(5))));
        let p = test_support::set_entity_data_packet(88, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(d.id, 88);
        // the write mirror on the dispatched carriers: every payload
        // shape re-encodes byte-for-byte, the retained ids included
        assert_eq!(
            encode_set_entity_data(d.id, &d.entries).unwrap(),
            p
        );
        let v: Vec<&ParticleValue> = d
            .entries
            .iter()
            .map(|e| match &e.value {
                EntityDataEntry::Particle(v) => v,
                other => panic!("expected a particle entry, got {other:?}"),
            })
            .collect();
        assert_eq!(v[0], &ParticleValue::Simple(0));
        assert_eq!(v[1], &ParticleValue::Block { particle_id: pt::BLOCK, state: 12345 });
        assert_eq!(v[2], &ParticleValue::Geyser { water_blocks: 2 });
        assert_eq!(v[3], &ParticleValue::GeyserBase { particle_id: pt::GEYSER_BASE, water_blocks: 2, burst_impulse_base: 0.5 });
        assert_eq!(v[4], &ParticleValue::Power(1.5));
        assert_eq!(v[5], &ParticleValue::Dust { color: 16711680, scale: 1.0 });
        assert_eq!(v[6], &ParticleValue::DustTransition { from_color: 16711680, to_color: 3790560, scale: 1.0 });
        assert_eq!(v[7], &ParticleValue::Spell { particle_id: pt::EFFECT, color: 16777215, power: 1.0 });
        assert_eq!(v[8], &ParticleValue::Color { particle_id: pt::ENTITY_EFFECT, color: 65280 });
        assert_eq!(v[9], &ParticleValue::SculkCharge(0.25));
        assert_eq!(v[10], &ParticleValue::Item { item_id: 745, count: 3, patch: ComponentPatch { added: vec![(3, ComponentValue::VarInt(35))], removed: vec![] } });
        assert_eq!(v[11], &ParticleValue::Vibration { source: PositionSource::Block([-1, 64, 2]), arrival_in_ticks: 20 });
        assert_eq!(v[12], &ParticleValue::Trail { target: [1.0, 2.0, 3.0], color: 7, duration: 9 });
        assert_eq!(v[13], &ParticleValue::Shriek(5));
    }

    /// The PARTICLES serializer (17) — the list operation over the
    /// dispatch — and the refusal map: the registry id's bounds (an
    /// unknown 125 AND a negative, each the $29 byIdOrThrow direction,
    /// each probe landing ON the lane before any payload), a negative
    /// block-state payload inside the block composite, an unknown
    /// position-source type inside the vibration's nested dispatch,
    /// and the negative list count (the readCount family's
    /// constructor death, refused panic-free at the same lane).
    #[test]
    fn the_particle_list_walks_and_the_refusal_map_holds() {
        use entity_data_serializers as s;
        // the list: two dispatched values — a simple bubble (3) and a
        // shriek carrying its delay
        let mut v = test_support::entity_data_varint(2);
        v.extend(test_support::entity_data_varint(3));
        v.extend(test_support::entity_data_varint(112));
        v.extend(test_support::entity_data_varint(7));
        let entries = test_support::entity_data_entry(0, s::PARTICLES, &v);
        let p = test_support::set_entity_data_packet(9, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::Particles(vec![ParticleValue::Simple(3), ParticleValue::Shriek(7)])
        );
        // the refusal map: every probe lands ON its lane
        for (bytes, why) in [
            (test_support::entity_data_varint(125), "unknown particle id"),
            (test_support::entity_data_varint(-1), "negative particle id"),
            (
                [test_support::entity_data_varint(1), test_support::entity_data_varint(-3)].concat(),
                "negative block-state payload",
            ),
            (
                [test_support::entity_data_varint(55), test_support::entity_data_varint(2)].concat(),
                "unknown position-source type",
            ),
        ] {
            let entries = test_support::entity_data_entry(0, s::PARTICLE, &bytes);
            let p = test_support::set_entity_data_packet(9, &entries);
            assert!(parse_set_entity_data(&p).is_err(), "{why} must refuse");
        }
        // the negative LIST count refuses the same way
        let entries =
            test_support::entity_data_entry(0, s::PARTICLES, &test_support::entity_data_varint(-1));
        let p = test_support::set_entity_data_packet(9, &entries);
        assert!(parse_set_entity_data(&p).is_err());
    }

    /// The RESOLVABLE_PROFILE serializer (41): `composite(either(
    /// GAME_PROFILE, Partial.STREAM_CODEC), PlayerSkin$Patch
    /// .STREAM_CODEC)` — the either's ONE-BYTE flag (true = the FULL
    /// GameProfile, false = the Partial), the identity side's own
    /// lanes, and the patch's four flag-optional lanes. The
    /// properties map is readCount(buf, 16) — the UPPER bound
    /// refuses, and a NEGATIVE count is an EMPTY map (the
    /// never-refuse family, pinned on the Partial probe). The
    /// model bool maps true = SLIM (the enum's own lambda pair).
    #[test]
    fn the_resolvable_profile_lane_decodes_both_sides_and_the_patch() {
        use entity_data_serializers as s;
        let mut uuid = [0u8; 16];
        for (i, b) in uuid.iter_mut().enumerate() {
            *b = (i as u8) * 17;
        }
        // the LEFT side: the full GameProfile + a one-asset patch
        let mut v = vec![1u8];
        v.extend_from_slice(&uuid);
        write_string(&mut v, "mc2rs");
        v.extend_from_slice(&test_support::entity_data_varint(1)); // one property
        write_string(&mut v, "textures");
        write_string(&mut v, "ewogICJ0aW1lc3RhbXAiIDogMTAwLAp9");
        v.push(0); // the signature's nullable flag: absent
        v.push(1); // the patch's body: present
        write_string(&mut v, "minecraft:slim");
        write_string(&mut v, "textures/entity/steve.png");
        v.push(0); // cape: absent
        v.push(0); // elytra: absent
        v.push(1); // model: present
        v.push(1); // the bool: true = SLIM
        let entries = test_support::entity_data_entry(0, s::RESOLVABLE_PROFILE, &v);
        let p = test_support::set_entity_data_packet(7, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ResolvableProfile {
                profile: ResolvableProfile::Full {
                    uuid,
                    name: "mc2rs".into(),
                    properties: vec![ProfileProperty {
                        name: "textures".into(),
                        value: "ewogICJ0aW1lc3RhbXAiIDogMTAwLAp9".into(),
                        signature: None,
                    }],
                },
                patch: PlayerSkinPatch {
                    body: Some(SkinAsset {
                        id: "minecraft:slim".into(),
                        texture_path: "textures/entity/steve.png".into(),
                    }),
                    cape: None,
                    elytra: None,
                    model_slim: Some(true),
                },
            }
        );
        // the RIGHT side: the Partial — the optional name rides, the
        // optional id is absent, and the NEGATIVE property count is
        // an EMPTY map (readCount never refuses it — the never-refuse
        // family's pinned member), and the patch rides all-absent
        let mut v = vec![0u8];
        v.push(1); // the name: present
        write_string(&mut v, "xavier");
        v.push(0); // the id: absent
        v.extend_from_slice(&test_support::entity_data_varint(-1)); // the count
        v.push(0); // body absent
        v.push(0); // cape absent
        v.push(0); // elytra absent
        v.push(0); // model absent
        let entries = test_support::entity_data_entry(0, s::RESOLVABLE_PROFILE, &v);
        let p = test_support::set_entity_data_packet(7, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ResolvableProfile {
                profile: ResolvableProfile::Partial {
                    name: Some("xavier".into()),
                    id: None,
                    properties: vec![],
                },
                patch: PlayerSkinPatch {
                    body: None,
                    cape: None,
                    elytra: None,
                    model_slim: None,
                },
            }
        );
        // the UPPER bound: 17 properties refuse (readCount's throw)
        let mut v = vec![1u8];
        v.extend_from_slice(&[0u8; 16]);
        write_string(&mut v, "mc2rs");
        v.extend_from_slice(&test_support::entity_data_varint(17));
        let entries = test_support::entity_data_entry(0, s::RESOLVABLE_PROFILE, &v);
        let p = test_support::set_entity_data_packet(7, &entries);
        assert!(parse_set_entity_data(&p).is_err());
        // a truncated identity refuses (the honest short-buffer failure)
        let entries = test_support::entity_data_entry(0, s::RESOLVABLE_PROFILE, &[1, 0x00, 0x11]);
        let p = test_support::set_entity_data_packet(7, &entries);
        assert!(parse_set_entity_data(&p).is_err());
    }

    /// Set Entity Data: the villager record's wire is THREE VarInts —
    /// the javap walk: `VillagerData.STREAM_CODEC` =
    /// `composite(holderRegistry(VILLAGER_TYPE), holderRegistry(
    /// VILLAGER_PROFESSION), VAR_INT, VillagerData::new)`, and
    /// `ByteBufCodecs$29.decode` is `VarInt.read` ->
    /// `IdMap.byIdOrThrow` (a NUMERIC registry id, not a name). The
    /// ids are retained verbatim (no villager registry tables
    /// client-side; a consumer resolves them), a NEGATIVE id is
    /// outside every IdMap and refuses (byIdOrThrow's direction),
    /// and the LEVEL is the record constructor's own `Math.max(1,
    /// level)` — composite decode calls `VillagerData::new`
    /// directly, so the clamp is part of the DECODE, not a consumer
    /// concern: wire 0 is a stored 1.
    #[test]
    fn the_villager_record_is_three_registry_ids_and_the_constructor_clamps_the_level() {
        use entity_data_serializers as s;
        // the round-trip: two ids + a sane level survive verbatim
        let entries = test_support::entity_data_entry(21, s::VILLAGER_DATA,
            &[test_support::entity_data_varint(0), test_support::entity_data_varint(41), test_support::entity_data_varint(5)].concat());
        let p = test_support::set_entity_data_packet(3, &entries);
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::VillagerData { type_id: 0, profession_id: 41, level: 5 }
        );
        // the constructor clamp: wire level 0 (and negative) is a
        // stored 1 — vanilla's own VillagerData(Holder, Holder, int)
        // does `Math.max(1, level)` BEFORE any consumer runs
        for wire_level in [0, -7] {
            let entries = test_support::entity_data_entry(22, s::VILLAGER_DATA,
                &[test_support::entity_data_varint(1), test_support::entity_data_varint(2), test_support::entity_data_varint(wire_level)].concat());
            let p = test_support::set_entity_data_packet(3, &entries);
            let d = parse_set_entity_data(&p).unwrap();
            assert_eq!(
                d.entries[0].value,
                EntityDataEntry::VillagerData { type_id: 1, profession_id: 2, level: 1 },
                "wire level {wire_level} must clamp to 1 (the record constructor's Math.max)"
            );
        }
        // a negative registry id is outside every IdMap — byIdOrThrow
        // throws, the packet read fails, the client refuses the same way
        for bytes in [
            vec![test_support::entity_data_varint(-1), test_support::entity_data_varint(2), test_support::entity_data_varint(1)], // negative TYPE
            vec![test_support::entity_data_varint(1), test_support::entity_data_varint(-1), test_support::entity_data_varint(1)], // negative PROFESSION
        ] {
            let entries = test_support::entity_data_entry(23, s::VILLAGER_DATA, &bytes.concat());
            let p = test_support::set_entity_data_packet(3, &entries);
            assert!(
                parse_set_entity_data(&p).is_err(),
                "a negative villager registry id must refuse"
            );
        }
        // a truncated record (ids without the level) is drift — refused
        let entries = test_support::entity_data_entry(24, s::VILLAGER_DATA,
            &[test_support::entity_data_varint(1), test_support::entity_data_varint(2)].concat());
        let p = test_support::set_entity_data_packet(3, &entries);
        assert!(parse_set_entity_data(&p).is_err());
    }

    /// Set Entity Data, serializer 7: the item stack's THREE layers —
    /// the count (the emptiness SENTINEL: ItemStack$1's `ifgt` means
    /// count <= 0 is the EMPTY stack and NOTHING else rides the wire,
    /// the count retained verbatim — a negative count is an empty stack
    /// too, and any byte behind it is trailing drift), the item's
    /// numeric registry id (the $29 byIdOrThrow VarInt, the villager
    /// round's shape), and the component patch (DataComponentPatch$3:
    /// VarInt added, VarInt removed, added entries as {type id, the
    /// type's OWN value}, removed entries as {type id} — the absence
    /// IS the payload). The FULL decodable slice: all SIXTEEN component
    /// types of [`data_component_types`] in one added list, each value
    /// through its own codec — the four VarInt lanes, the two ZERO-BYTE
    /// unit lanes, the bare component tags, the compound-only custom
    /// data, the BE f32, the three UTF identifier strings, the capped
    /// lore list, the idMapper ordinal, and the bool.
    #[test]
    fn the_item_stack_is_count_item_and_a_component_patch() {
        use data_component_types as t;
        use entity_data_serializers as s;

        // the empty stack, both sentinel shapes
        for count in [0i32, -7] {
            let p = test_support::set_entity_data_packet(5, &test_support::entity_data_entry(
                8,
                s::ITEM_STACK,
                &test_support::entity_data_varint(count),
            ));
            let d = parse_set_entity_data(&p).unwrap();
            assert_eq!(
                d.entries[0].value,
                EntityDataEntry::ItemStack {
                    count,
                    item_id: 0,
                    patch: ComponentPatch::default()
                }
            );
        }
        // the empty stack with DRIFT behind it: the decoder returned at
        // the count, so the follower is trailing bytes — refused
        let mut drift = test_support::entity_data_varint(0);
        drift.push(0x01);
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &drift),
        );
        assert!(parse_set_entity_data(&p).is_err());

        // the plain stack: an EMPTY patch (both counts zero — the $3
        // early return) rides nothing but the four-byte header
        let mut plain = Vec::new();
        for v in [1i32, 1, 0, 0] {
            plain.extend_from_slice(&test_support::entity_data_varint(v));
        }
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &plain),
        );
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ItemStack {
                count: 1,
                item_id: 1,
                patch: ComponentPatch::default()
            }
        );

        // the removal-only patch: removed entries carry the type id
        // ALONE — the absence is the payload
        let mut removed_only = Vec::new();
        for v in [2i32, 5, 0, 2, 3, 4] {
            removed_only.extend_from_slice(&test_support::entity_data_varint(v));
        }
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &removed_only),
        );
        let d = parse_set_entity_data(&p).unwrap();
        match &d.entries[0].value {
            EntityDataEntry::ItemStack { count, item_id, patch } => {
                assert_eq!((*count, *item_id), (2, 5));
                assert!(patch.added.is_empty());
                assert_eq!(patch.removed, vec![3, 4]);
            }
            other => panic!("expected an item stack, got {other:?}"),
        }

        // the FULL decodable slice: all SIXTEEN types in one added list
        let mut value = Vec::new();
        for v in [4i32, 977, 16, 0] {
            value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        // 0 custom_data: a bare COMPOUND tag (compoundTagCodec's decode
        // map throws on any other shape)
        value.extend_from_slice(&test_support::entity_data_varint(t::CUSTOM_DATA));
        crate::nbt::write_bare_nbt(
            &Nbt::Compound(vec![("tag".into(), Nbt::Byte(1))]),
            &mut value,
        )
        .unwrap();
        // 1 max_stack_size, 2 max_damage, 3 damage: the VarInt lanes
        for (tid, v) in [(t::MAX_STACK_SIZE, 64i32), (t::MAX_DAMAGE, 2031), (t::DAMAGE, 35)] {
            value.extend_from_slice(&test_support::entity_data_varint(tid));
            value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        // 4 unbreakable: the ZERO-BYTE unit
        value.extend_from_slice(&test_support::entity_data_varint(t::UNBREAKABLE));
        // 6 custom_name: a bare component tag (a plain-text string)
        value.extend_from_slice(&test_support::entity_data_varint(t::CUSTOM_NAME));
        crate::nbt::write_bare_nbt(&Nbt::String("Excalibur".into()), &mut value).unwrap();
        // 7 minimum_attack_charge: a BE f32
        value.extend_from_slice(&test_support::entity_data_varint(t::MINIMUM_ATTACK_CHARGE));
        value.extend_from_slice(&0.5f32.to_be_bytes());
        // 9 item_name: a structured bare tag
        value.extend_from_slice(&test_support::entity_data_varint(t::ITEM_NAME));
        crate::nbt::write_bare_nbt(
            &Nbt::Compound(vec![("text".into(), Nbt::String("Sword".into()))]),
            &mut value,
        )
        .unwrap();
        // 10 item_model: a UTF identifier
        value.extend_from_slice(&test_support::entity_data_varint(t::ITEM_MODEL));
        write_string(&mut value, "minecraft:glow");
        // 11 lore: a VarInt count (<= 256) of bare tags
        value.extend_from_slice(&test_support::entity_data_varint(t::LORE));
        value.extend_from_slice(&test_support::entity_data_varint(2));
        crate::nbt::write_bare_nbt(&Nbt::String("line one".into()), &mut value).unwrap();
        crate::nbt::write_bare_nbt(&Nbt::String("line two".into()), &mut value).unwrap();
        // 12 rarity: the idMapper ordinal
        value.extend_from_slice(&test_support::entity_data_varint(t::RARITY));
        value.extend_from_slice(&test_support::entity_data_varint(3));
        // 19 repair_cost: back to the VarInt lanes
        value.extend_from_slice(&test_support::entity_data_varint(t::REPAIR_COST));
        value.extend_from_slice(&test_support::entity_data_varint(37));
        // 20 creative_slot_lock: the second zero-byte unit
        value.extend_from_slice(&test_support::entity_data_varint(t::CREATIVE_SLOT_LOCK));
        // 21 enchantment_glint_override: one bool byte
        value.extend_from_slice(&test_support::entity_data_varint(t::ENCHANTMENT_GLINT_OVERRIDE));
        value.push(1);
        // 35 tooltip_style, 71 note_block_sound: the UTF identifiers
        value.extend_from_slice(&test_support::entity_data_varint(t::TOOLTIP_STYLE));
        write_string(&mut value, "minecraft:epic");
        value.extend_from_slice(&test_support::entity_data_varint(t::NOTE_BLOCK_SOUND));
        write_string(&mut value, "minecraft:block.note_block.pling");

        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
        );
        let d = parse_set_entity_data(&p).unwrap();
        let expected_patch = ComponentPatch {
            added: vec![
                (
                    t::CUSTOM_DATA,
                    ComponentValue::CompoundTag(Nbt::Compound(vec![("tag".into(), Nbt::Byte(1))])),
                ),
                (t::MAX_STACK_SIZE, ComponentValue::VarInt(64)),
                (t::MAX_DAMAGE, ComponentValue::VarInt(2031)),
                (t::DAMAGE, ComponentValue::VarInt(35)),
                (t::UNBREAKABLE, ComponentValue::Unit),
                (t::CUSTOM_NAME, ComponentValue::Component(Nbt::String("Excalibur".into()))),
                (t::MINIMUM_ATTACK_CHARGE, ComponentValue::Float(0.5)),
                (
                    t::ITEM_NAME,
                    ComponentValue::Component(Nbt::Compound(vec![(
                        "text".into(),
                        Nbt::String("Sword".into()),
                    )])),
                ),
                (t::ITEM_MODEL, ComponentValue::String("minecraft:glow".into())),
                (
                    t::LORE,
                    ComponentValue::Lore(vec![
                        Nbt::String("line one".into()),
                        Nbt::String("line two".into()),
                    ]),
                ),
                (t::RARITY, ComponentValue::Rarity(3)),
                (t::REPAIR_COST, ComponentValue::VarInt(37)),
                (t::CREATIVE_SLOT_LOCK, ComponentValue::Unit),
                (t::ENCHANTMENT_GLINT_OVERRIDE, ComponentValue::Bool(true)),
                (t::TOOLTIP_STYLE, ComponentValue::String("minecraft:epic".into())),
                (
                    t::NOTE_BLOCK_SOUND,
                    ComponentValue::String("minecraft:block.note_block.pling".into()),
                ),
            ],
            removed: vec![],
        };
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ItemStack {
                count: 4,
                item_id: 977,
                patch: expected_patch
            }
        );
    }

    /// Set Entity Data, serializer 7: the patch REFUSES what it cannot
    /// size — the wire is UNFRAMED (no length prefix; $3 hands the
    /// buffer straight to the type's streamCodec), so the refusal map
    /// is three-directional: an UNKNOWN type id refuses where vanilla's
    /// byIdOrThrow throws, a NEGATIVE one refuses (outside every
    /// IdMap), and a KNOWN-but-unimplemented one refuses because its
    /// byte length is only knowable through a codec the port does not
    /// carry — there is no honest skip. Two shape refusals pin the
    /// frame: the negative ADDED count (vanilla's fastutil map
    /// constructor throws on the negative capacity) and the lore list
    /// over its codec's 256-entry cap; custom_data's compound-only
    /// decode map is the DecoderException direction.
    #[test]
    fn the_patch_refuses_what_it_cannot_size() {
        use entity_data_serializers as s;

        // a positive-count stack whose patch carries ONE added entry
        let stack_with = |added_bytes: &[u8]| -> Vec<u8> {
            let mut v = Vec::new();
            for prefix in [1i32, 1, 1, 0] {
                v.extend_from_slice(&test_support::entity_data_varint(prefix));
            }
            v.extend_from_slice(added_bytes);
            v
        };
        let one_stack = |added_bytes: &[u8]| -> Vec<u8> {
            test_support::set_entity_data_packet(
                5,
                &test_support::entity_data_entry(8, s::ITEM_STACK, &stack_with(added_bytes)),
            )
        };
        // the unknown registry id (999) and the negative one (-1)
        for tid in [999i32, -1] {
            assert!(
                parse_set_entity_data(&one_stack(&test_support::entity_data_varint(tid)))
                    .is_err(),
                "component type {tid} must refuse"
            );
        }
        // known to the registry but unimplemented here (5 — the
        // dense-registry slot between 4 unbreakable and 6 custom_name
        // that no wave has claimed; 13, the adventure-mode pair 14/15
        // and attribute_modifiers 16 all left the refusal list as the
        // wave-two, wave-three and wave-four rounds landed)
        assert!(parse_set_entity_data(&one_stack(&test_support::entity_data_varint(5))).is_err());
        // the negative ADDED count
        let mut neg = Vec::new();
        neg.extend_from_slice(&test_support::entity_data_varint(1));
        neg.extend_from_slice(&test_support::entity_data_varint(1));
        neg.extend_from_slice(&test_support::entity_data_varint(-1));
        neg.extend_from_slice(&test_support::entity_data_varint(0));
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &neg),
        );
        assert!(parse_set_entity_data(&p).is_err());
        // the lore cap: a 257-entry list is over the codec's limit
        let mut over = Vec::new();
        over.extend_from_slice(&test_support::entity_data_varint(1));
        over.extend_from_slice(&test_support::entity_data_varint(1));
        over.extend_from_slice(&test_support::entity_data_varint(1));
        over.extend_from_slice(&test_support::entity_data_varint(0));
        over.extend_from_slice(&test_support::entity_data_varint(11));
        over.extend_from_slice(&test_support::entity_data_varint(257));
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &over),
        );
        assert!(parse_set_entity_data(&p).is_err());
        // custom_data's compound-only map: a TAG_String is the
        // DecoderException
        let mut str_tag = Vec::new();
        str_tag.extend_from_slice(&test_support::entity_data_varint(1));
        str_tag.extend_from_slice(&test_support::entity_data_varint(1));
        str_tag.extend_from_slice(&test_support::entity_data_varint(1));
        str_tag.extend_from_slice(&test_support::entity_data_varint(0));
        str_tag.extend_from_slice(&test_support::entity_data_varint(0));
        crate::nbt::write_bare_nbt(&Nbt::String("not a compound".into()), &mut str_tag).unwrap();
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &str_tag),
        );
        assert!(parse_set_entity_data(&p).is_err());
    }

    /// The patch value codecs' FOURTH WAVE: the held-item's stat
    /// surface and its food. The audit (javap): attribute_modifiers
    /// 16 = `composite(Entry.STREAM_CODEC.list(), ..)` — a VarInt
    /// count then count × FOUR-lane entries (the $29 attribute id, an
    /// Identifier-UTF modifier id, a BIG-ENDIAN f64 amount, the
    /// Operation ordinal, the EquipmentSlotGroup ordinal, and the
    /// 26.2-NEW Display DISPATCH: the Type ordinal then the variant
    /// payload — default/hidden ZERO bytes, override one bare tag).
    /// The continuous-ZERO idMapper maps NEVER refuse (any i32
    /// decodes; out-of-range AND negative fill to values[0]) — the
    /// ordinals are retained verbatim and the DISPLAY ordinal, being
    /// FRAMING, fills to Default in-decode (pinned below). food 23 =
    /// `composite(VAR_INT, FLOAT, BOOL)` — nutrition is a VAR_INT in
    /// 26.2 (the record holds `int nutrition`; no clamp anywhere).
    #[test]
    fn the_fourth_wave_stat_and_food_decode() {
        use data_component_types as t;
        use entity_data_serializers as s;

        // one stack, TWO added components: 16 attribute_modifiers,
        // 23 food
        let mut value = Vec::new();
        for v in [1i32, 1, 2, 0] {
            value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        // 16: THREE entries — the default display, the negative
        // operation + out-of-range slot retained verbatim, and the
        // override display carrying a bare tag
        value.extend_from_slice(&test_support::entity_data_varint(t::ATTRIBUTE_MODIFIERS));
        value.extend_from_slice(&test_support::entity_data_varint(3));
        // entry 1: attribute 5, "minecraft:test", 0.5, op 2,
        // slot 1 MAINHAND, display 0 (Default — zero bytes)
        value.extend_from_slice(&test_support::entity_data_varint(5));
        write_string(&mut value, "minecraft:test");
        value.extend_from_slice(&0.5f64.to_be_bytes());
        value.extend_from_slice(&test_support::entity_data_varint(2));
        value.extend_from_slice(&test_support::entity_data_varint(1));
        value.extend_from_slice(&test_support::entity_data_varint(0));
        // entry 2: attribute 7, "minecraft:other", -1.25, op -3
        // (RETAINED — the ZERO-fill is a consumer's act), slot 42
        // (RETAINED), display 1 (Hidden — zero bytes)
        value.extend_from_slice(&test_support::entity_data_varint(7));
        write_string(&mut value, "minecraft:other");
        value.extend_from_slice(&(-1.25f64).to_be_bytes());
        value.extend_from_slice(&test_support::entity_data_varint(-3));
        value.extend_from_slice(&test_support::entity_data_varint(42));
        value.extend_from_slice(&test_support::entity_data_varint(1));
        // entry 3: attribute 0, "minecraft:third", 2.0, op 0,
        // slot 9 SADDLE, display 2 (Override — one bare tag)
        value.extend_from_slice(&test_support::entity_data_varint(0));
        write_string(&mut value, "minecraft:third");
        value.extend_from_slice(&2.0f64.to_be_bytes());
        value.extend_from_slice(&test_support::entity_data_varint(0));
        value.extend_from_slice(&test_support::entity_data_varint(9));
        value.extend_from_slice(&test_support::entity_data_varint(2));
        crate::nbt::write_bare_nbt(
            &Nbt::Compound(vec![("key".into(), Nbt::String("text".into()))]),
            &mut value,
        )
        .unwrap();
        // 23 food: nutrition -3 (RETAINED — the record constructor
        // clamps nothing), saturation 0.6, can_always_eat true
        value.extend_from_slice(&test_support::entity_data_varint(t::FOOD));
        value.extend_from_slice(&test_support::entity_data_varint(-3));
        value.extend_from_slice(&0.6f32.to_be_bytes());
        value.push(1);

        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
        );
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ItemStack {
                count: 1,
                item_id: 1,
                patch: ComponentPatch {
                    added: vec![
                        (
                            t::ATTRIBUTE_MODIFIERS,
                            ComponentValue::AttributeModifiers(vec![
                                AttributeModifierEntry {
                                    attribute_id: 5,
                                    modifier: AttributeModifier {
                                        id: "minecraft:test".into(),
                                        amount: 0.5,
                                        operation: 2
                                    },
                                    slot: 1,
                                    display: AttributeDisplay::Default
                                },
                                AttributeModifierEntry {
                                    attribute_id: 7,
                                    modifier: AttributeModifier {
                                        id: "minecraft:other".into(),
                                        amount: -1.25,
                                        operation: -3
                                    },
                                    slot: 42,
                                    display: AttributeDisplay::Hidden
                                },
                                AttributeModifierEntry {
                                    attribute_id: 0,
                                    modifier: AttributeModifier {
                                        id: "minecraft:third".into(),
                                        amount: 2.0,
                                        operation: 0
                                    },
                                    slot: 9,
                                    display: AttributeDisplay::Override(Nbt::Compound(vec![
                                        ("key".into(), Nbt::String("text".into()))
                                    ]))
                                },
                            ])
                        ),
                        (
                            t::FOOD,
                            ComponentValue::Food {
                                nutrition: -3,
                                saturation: 0.6,
                                can_always_eat: true
                            }
                        ),
                    ],
                    removed: vec![]
                }
            }
        );

        // the Display FRAMING fill: an out-of-range Type ordinal (9)
        // fills to Default — the zero-byte walk consumes nothing else
        // (lambda$continuous$0: any key outside [0, len) maps to
        // values[0]; the walk MUST mirror that or the wire desyncs)
        let mut value = Vec::new();
        for v in [1i32, 1, 1, 0] {
            value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        value.extend_from_slice(&test_support::entity_data_varint(t::ATTRIBUTE_MODIFIERS));
        value.extend_from_slice(&test_support::entity_data_varint(1));
        value.extend_from_slice(&test_support::entity_data_varint(1));
        write_string(&mut value, "minecraft:fill");
        value.extend_from_slice(&0.0f64.to_be_bytes());
        value.extend_from_slice(&test_support::entity_data_varint(1));
        value.extend_from_slice(&test_support::entity_data_varint(0));
        value.extend_from_slice(&test_support::entity_data_varint(9));
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
        );
        let d = parse_set_entity_data(&p).unwrap();
        match &d.entries[0].value {
            EntityDataEntry::ItemStack { patch, .. } => match &patch.added[0].1 {
                ComponentValue::AttributeModifiers(entries) => {
                    assert_eq!(entries[0].display, AttributeDisplay::Default);
                }
                other => panic!("expected attribute modifiers, got {other:?}"),
            },
            other => panic!("expected an item stack, got {other:?}"),
        }
    }

    /// The consumable's FIVE lanes decode (with the sound tagged
    /// union in BOTH directions) and the effects list WALKS: each
    /// entry = a dispatched registry id (`$29` byIdOrThrow) + that
    /// type's own payload. The five known types ride the registry's
    /// clinit order (apply_effects 0, remove_effects 1,
    /// clear_all_effects 2, teleport_randomly 3, play_sound 4 — the
    /// registry names carry no "status" infix; the wiki-era naming
    /// is not the wire's). Refusals: an unknown dispatch id, a
    /// NEGATIVE dispatch id, a negative effect registry id, a
    /// negative holder-set tag, and a wire that ends before the
    /// dispatch id (the unframed nature).
    #[test]
    fn the_consumable_five_lanes_decode_and_refuse() {
        use data_component_types as t;
        use entity_data_serializers as s;

        let stack_with = |consumable_bytes: &[u8]| -> Vec<u8> {
            let mut v = Vec::new();
            for prefix in [1i32, 1, 1, 0] {
                v.extend_from_slice(&test_support::entity_data_varint(prefix));
            }
            v.extend_from_slice(consumable_bytes);
            v
        };
        let one_stack = |consumable_bytes: &[u8]| -> Vec<u8> {
            test_support::set_entity_data_packet(
                5,
                &test_support::entity_data_entry(8, s::ITEM_STACK, &stack_with(consumable_bytes)),
            )
        };

        // the DIRECT sound (wire 0): 1.6s, DRINK (ordinal 2),
        // "minecraft:item.generic.eat" at range 1.0, particles on,
        // ONE consume effect — the zero-byte clear_all_effects entry
        let mut direct = Vec::new();
        direct.extend_from_slice(&test_support::entity_data_varint(t::CONSUMABLE));
        direct.extend_from_slice(&1.6f32.to_be_bytes());
        direct.extend_from_slice(&test_support::entity_data_varint(2));
        direct.extend_from_slice(&test_support::entity_data_varint(0));
        write_string(&mut direct, "minecraft:item.generic.eat");
        direct.extend_from_slice(&1.0f32.to_be_bytes());
        direct.push(1);
        direct.extend_from_slice(&test_support::entity_data_varint(1));
        direct.extend_from_slice(&test_support::entity_data_varint(2));
        let d = parse_set_entity_data(&one_stack(&direct)).unwrap();
        match &d.entries[0].value {
            EntityDataEntry::ItemStack { patch, .. } => assert_eq!(
                patch.added[0].1,
                ComponentValue::Consumable {
                    consume_seconds: 1.6,
                    animation: 2,
                    sound: ConsumableSound::Direct {
                        id: "minecraft:item.generic.eat".into(),
                        range: 1.0
                    },
                    has_consume_particles: true,
                    effects: vec![ConsumeEffect::ClearAllEffects]
                }
            ),
            other => panic!("expected an item stack, got {other:?}"),
        }

        // the REGISTRY sound (wire tag 5 = registry id 4, retained
        // verbatim — the VILLAGER_DATA line), particles off, NO
        // effects (the count walks nothing)
        let mut registry = Vec::new();
        registry.extend_from_slice(&test_support::entity_data_varint(t::CONSUMABLE));
        registry.extend_from_slice(&0.8f32.to_be_bytes());
        registry.extend_from_slice(&test_support::entity_data_varint(0));
        registry.extend_from_slice(&test_support::entity_data_varint(5));
        registry.push(0);
        registry.extend_from_slice(&test_support::entity_data_varint(0));
        let d = parse_set_entity_data(&one_stack(&registry)).unwrap();
        match &d.entries[0].value {
            EntityDataEntry::ItemStack { patch, .. } => match &patch.added[0].1 {
                ComponentValue::Consumable { sound, effects, .. } => {
                    assert_eq!(sound, &ConsumableSound::Registry(4));
                    assert!(effects.is_empty());
                }
                other => panic!("expected a consumable, got {other:?}"),
            },
            other => panic!("expected an item stack, got {other:?}"),
        }

        // the refusals, each riding the DIRECT base with its
        // trailing effects list replaced (the lanes BEFORE the list
        // are intact, so the failure is the list's own lane):
        let base = {
            let mut v = direct.clone();
            let n = v.len();
            v.truncate(n - 2); // drop the effects count + the dispatch id
            v
        };
        let probe = |tail: &[u8]| {
            let mut v = base.clone();
            v.extend_from_slice(tail);
            parse_set_entity_data(&one_stack(&v)).is_err()
        };

        // an UNKNOWN dispatch id refuses (byIdOrThrow) — wire id 5
        // is past the registry's five entries
        assert!(probe(&test_support::entity_data_varint(1)
            .iter()
            .chain(test_support::entity_data_varint(5).iter())
            .copied()
            .collect::<Vec<u8>>()));
        // a NEGATIVE dispatch id refuses (the same lookup throws)
        assert!(probe(&test_support::entity_data_varint(1)
            .iter()
            .chain(test_support::entity_data_varint(-1).iter())
            .copied()
            .collect::<Vec<u8>>()));
        // the wire ENDING before the dispatch id refuses (the list
        // is unframed — the count is the only framing there is)
        assert!(probe(&test_support::entity_data_varint(1)));

        // a negative effect registry id inside apply_effects refuses
        // (the instance's byIdOrThrow lane): count 1, dispatch id 0,
        // apply list count 1, effect id -3
        let mut neg_effect = base.clone();
        neg_effect.extend_from_slice(&test_support::entity_data_varint(1));
        neg_effect.extend_from_slice(&test_support::entity_data_varint(0));
        neg_effect.extend_from_slice(&test_support::entity_data_varint(1));
        neg_effect.extend_from_slice(&test_support::entity_data_varint(-3));
        assert!(parse_set_entity_data(&one_stack(&neg_effect)).is_err());

        // a negative holder-set tag inside remove_effects refuses:
        // count 1, dispatch id 1, tag -2
        let mut neg_tag = base.clone();
        neg_tag.extend_from_slice(&test_support::entity_data_varint(1));
        neg_tag.extend_from_slice(&test_support::entity_data_varint(1));
        neg_tag.extend_from_slice(&test_support::entity_data_varint(-2));
        assert!(parse_set_entity_data(&one_stack(&neg_tag)).is_err());
    }

    /// The consume-effect dispatch decodes ALL FIVE types, the
    /// apply_effects entry walking the RECURSIVE details composite
    /// (the hidden-effect chain two levels deep — `StreamCodec.
    /// recursive` mirrored walk-for-walk). The audit (javap, one hop
    /// per type): apply_effects = `composite(MobEffectInstance.
    /// STREAM_CODEC.list(), FLOAT)`; remove_effects =
    /// `composite(ByteBufCodecs.holderSet(MOB_EFFECT))` (the
    /// wave-three tagged union, decoded in BOTH directions);
    /// clear_all_effects = `StreamCodec.unit(INSTANCE)` (ZERO bytes);
    /// teleport_randomly = `composite(FLOAT)`; play_sound =
    /// `composite(SoundEvent.STREAM_CODEC)` (the SAME tagged holder
    /// union as the consumable's sound lane — one codec, two call
    /// sites). MobEffectInstance = `composite(holderRegistry(
    /// MOB_EFFECT), Details)` where Details =
    /// `StreamCodec.recursive(composite(VAR_INT, VAR_INT, BOOL,
    /// BOOL, BOOL, optional(self)))` — the record's component order
    /// confirmed by its bootstrap strings.
    #[test]
    fn the_consume_effect_dispatch_decodes_all_five_types() {
        use data_component_types as t;
        use entity_data_serializers as s;

        let stack_with = |consumable_bytes: &[u8]| -> Vec<u8> {
            let mut v = Vec::new();
            for prefix in [1i32, 1, 1, 0] {
                v.extend_from_slice(&test_support::entity_data_varint(prefix));
            }
            v.extend_from_slice(consumable_bytes);
            v
        };
        let one_stack = |consumable_bytes: &[u8]| -> Vec<u8> {
            test_support::set_entity_data_packet(
                5,
                &test_support::entity_data_entry(8, s::ITEM_STACK, &stack_with(consumable_bytes)),
            )
        };

        let mut w = Vec::new();
        w.extend_from_slice(&test_support::entity_data_varint(t::CONSUMABLE));
        w.extend_from_slice(&1.0f32.to_be_bytes()); // consume_seconds
        w.extend_from_slice(&test_support::entity_data_varint(1)); // EAT
        w.extend_from_slice(&test_support::entity_data_varint(0)); // DIRECT sound
        write_string(&mut w, "minecraft:entity.generic.eat");
        w.extend_from_slice(&1.0f32.to_be_bytes());
        w.push(1); // particles
        // the FIVE dispatched entries, one of each type
        w.extend_from_slice(&test_support::entity_data_varint(5));
        // [0] apply_effects: ONE instance whose details carry a
        // hidden effect (recursion depth 1), probability 0.5
        w.extend_from_slice(&test_support::entity_data_varint(0));
        w.extend_from_slice(&test_support::entity_data_varint(1)); // the list count
        w.extend_from_slice(&test_support::entity_data_varint(9)); // effect id 9
        w.extend_from_slice(&test_support::entity_data_varint(2)); // amplifier
        w.extend_from_slice(&test_support::entity_data_varint(-1)); // INFINITE_DURATION
        w.push(0); // ambient
        w.push(1); // show_particles
        w.push(1); // show_icon
        w.push(1); // hidden_effect PRESENT
        w.extend_from_slice(&test_support::entity_data_varint(3)); // nested amplifier
        w.extend_from_slice(&test_support::entity_data_varint(100)); // nested duration
        w.push(1); // nested ambient
        w.push(0); // nested show_particles
        w.push(1); // nested show_icon
        w.push(0); // nested hidden_effect ABSENT — the recursion ends
        w.extend_from_slice(&0.5f32.to_be_bytes()); // probability
        // [1] remove_effects — the NAMED set ("minecraft:harmful")
        w.extend_from_slice(&test_support::entity_data_varint(1));
        w.extend_from_slice(&test_support::entity_data_varint(0));
        write_string(&mut w, "minecraft:harmful");
        // [2] clear_all_effects — ZERO payload bytes
        w.extend_from_slice(&test_support::entity_data_varint(2));
        // [3] teleport_randomly — diameter 8.0
        w.extend_from_slice(&test_support::entity_data_varint(3));
        w.extend_from_slice(&8.0f32.to_be_bytes());
        // [4] play_sound — the REGISTRY sound (tag 5 = id 4)
        w.extend_from_slice(&test_support::entity_data_varint(4));
        w.extend_from_slice(&test_support::entity_data_varint(5));

        let d = parse_set_entity_data(&one_stack(&w)).unwrap();
        match &d.entries[0].value {
            EntityDataEntry::ItemStack { patch, .. } => match &patch.added[0].1 {
                ComponentValue::Consumable { effects, .. } => assert_eq!(
                    effects,
                    &vec![
                        ConsumeEffect::ApplyEffects {
                            effects: vec![EffectInstance {
                                effect_id: 9,
                                details: EffectDetails {
                                    amplifier: 2,
                                    duration: -1,
                                    ambient: false,
                                    show_particles: true,
                                    show_icon: true,
                                    hidden_effect: Some(Box::new(EffectDetails {
                                        amplifier: 3,
                                        duration: 100,
                                        ambient: true,
                                        show_particles: false,
                                        show_icon: true,
                                        hidden_effect: None,
                                    })),
                                },
                            }],
                            probability: 0.5,
                        },
                        ConsumeEffect::RemoveEffects(EffectHolderSet::Named(
                            "minecraft:harmful".into(),
                        )),
                        ConsumeEffect::ClearAllEffects,
                        ConsumeEffect::TeleportRandomly { diameter: 8.0 },
                        ConsumeEffect::PlaySound(ConsumableSound::Registry(4)),
                    ]
                ),
                other => panic!("expected a consumable, got {other:?}"),
            },
            other => panic!("expected an item stack, got {other:?}"),
        }
    }

    /// The patch value codecs' SECOND WAVE: the three record codecs
    /// decode their composites. The audit (javap, one hop each):
    /// enchantments 13 = `composite(ByteBufCodecs.map(factory,
    /// Enchantment.STREAM_CODEC, VAR_INT), ..)` — a VarInt count then
    /// count x {registry id, level}, the level a PLAIN VarInt (the
    /// intRange(1,255) LEVEL_CODEC is the NBT schema, not the wire);
    /// custom_model_data 17 = `composite(FLOAT.list(), BOOL.list(),
    /// STRING_UTF8.list(), INT.list(), ..)` — FOUR VarInt-count lists
    /// in composite order, the colors riding `ByteBufCodecs.INT` =
    /// `readInt` = a BIG-ENDIAN i32 (the same trap family as
    /// LONG/VAR_LONG: the construction site is the shape); tooltip_
    /// display 18 = `composite(BOOL, DataComponentType.STREAM_CODEC
    /// .collection(factory), ..)` — the hide byte, then a VarInt
    /// count of self-delimiting type ids (retained verbatim, the
    /// DIRECTION/RARITY line). The shared readCount: a negative count
    /// refuses (readCount checks only the maxSize ceiling; the
    /// negative fails at the collection constructor — the read fails
    /// either way).
    #[test]
    fn the_patch_value_records_decode_their_composites() {
        use data_component_types as t;
        use entity_data_serializers as s;

        let mut value = Vec::new();
        for v in [1i32, 1, 3, 0] {
            value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        // 13 enchantments: two {id, level} pairs
        value.extend_from_slice(&test_support::entity_data_varint(t::ENCHANTMENTS));
        value.extend_from_slice(&test_support::entity_data_varint(2));
        value.extend_from_slice(&test_support::entity_data_varint(9));
        value.extend_from_slice(&test_support::entity_data_varint(3));
        value.extend_from_slice(&test_support::entity_data_varint(70));
        value.extend_from_slice(&test_support::entity_data_varint(1));
        // 17 custom_model_data: floats [1.5, -0.25], flags [true],
        // strings ["a", "b"], colors [0xFF00FF00] — BE i32, NOT a VarInt
        value.extend_from_slice(&test_support::entity_data_varint(t::CUSTOM_MODEL_DATA));
        value.extend_from_slice(&test_support::entity_data_varint(2));
        value.extend_from_slice(&1.5f32.to_be_bytes());
        value.extend_from_slice(&(-0.25f32).to_be_bytes());
        value.extend_from_slice(&test_support::entity_data_varint(1));
        value.push(1);
        value.extend_from_slice(&test_support::entity_data_varint(2));
        write_string(&mut value, "a");
        write_string(&mut value, "b");
        value.extend_from_slice(&test_support::entity_data_varint(1));
        value.extend_from_slice(&0xFF00FF00u32.to_be_bytes());
        // 18 tooltip_display: hide = true, hidden [3, 12]
        value.extend_from_slice(&test_support::entity_data_varint(t::TOOLTIP_DISPLAY));
        value.push(1);
        value.extend_from_slice(&test_support::entity_data_varint(2));
        value.extend_from_slice(&test_support::entity_data_varint(3));
        value.extend_from_slice(&test_support::entity_data_varint(12));

        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
        );
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ItemStack {
                count: 1,
                item_id: 1,
                patch: ComponentPatch {
                    added: vec![
                        (
                            t::ENCHANTMENTS,
                            ComponentValue::Enchantments(vec![(9, 3), (70, 1)])
                        ),
                        (
                            t::CUSTOM_MODEL_DATA,
                            ComponentValue::CustomModelData {
                                floats: vec![1.5, -0.25],
                                flags: vec![true],
                                strings: vec!["a".into(), "b".into()],
                                colors: vec![0xFF00FF00u32 as i32],
                            }
                        ),
                        (
                            t::TOOLTIP_DISPLAY,
                            ComponentValue::TooltipDisplay {
                                hide_tooltip: true,
                                hidden: vec![3, 12]
                            }
                        ),
                    ],
                    removed: vec![]
                }
            }
        );

        // the EMPTY shapes: a zero-count map, all four model-data
        // lists empty, the hidden set empty with hide = false — the
        // readCount walks consume nothing but their own count bytes
        let mut value = Vec::new();
        for v in [1i32, 1, 3, 0] {
            value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        value.extend_from_slice(&test_support::entity_data_varint(t::ENCHANTMENTS));
        value.extend_from_slice(&test_support::entity_data_varint(0));
        value.extend_from_slice(&test_support::entity_data_varint(t::CUSTOM_MODEL_DATA));
        for _ in 0..4 {
            value.extend_from_slice(&test_support::entity_data_varint(0));
        }
        value.extend_from_slice(&test_support::entity_data_varint(t::TOOLTIP_DISPLAY));
        value.push(0);
        value.extend_from_slice(&test_support::entity_data_varint(0));
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
        );
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ItemStack {
                count: 1,
                item_id: 1,
                patch: ComponentPatch {
                    added: vec![
                        (t::ENCHANTMENTS, ComponentValue::Enchantments(vec![])),
                        (
                            t::CUSTOM_MODEL_DATA,
                            ComponentValue::CustomModelData {
                                floats: vec![],
                                flags: vec![],
                                strings: vec![],
                                colors: vec![]
                            }
                        ),
                        (
                            t::TOOLTIP_DISPLAY,
                            ComponentValue::TooltipDisplay {
                                hide_tooltip: false,
                                hidden: vec![]
                            }
                        ),
                    ],
                    removed: vec![]
                }
            }
        );
    }

    /// The wave-two records' SHAPE refusals: a negative count on each
    /// lane (the enchantment map, the model-data floats, the hidden
    /// set — readCount's negative passes its ceiling check and fails
    /// at the constructor; the read fails either way) and a negative
    /// registry id inside the map's key and the hidden set.
    #[test]
    fn the_wave_two_records_refuse_their_shape_drift() {
        use data_component_types as t;
        use entity_data_serializers as s;

        let stack_with = |value: &[u8]| -> Vec<u8> {
            test_support::set_entity_data_packet(
                5,
                &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
            )
        };
        // header: count 1, item 1, added 1, removed 0
        let mut base = Vec::new();
        for v in [1i32, 1, 1, 0] {
            base.extend_from_slice(&test_support::entity_data_varint(v));
        }
        let with_base = |tail: &[u8]| -> Vec<u8> {
            let mut v = base.clone();
            v.extend_from_slice(tail);
            v
        };
        // a negative map count
        let mut tail = Vec::new();
        tail.extend_from_slice(&test_support::entity_data_varint(t::ENCHANTMENTS));
        tail.extend_from_slice(&test_support::entity_data_varint(-3));
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
        // a negative map KEY
        let mut tail = Vec::new();
        tail.extend_from_slice(&test_support::entity_data_varint(t::ENCHANTMENTS));
        tail.extend_from_slice(&test_support::entity_data_varint(1));
        tail.extend_from_slice(&test_support::entity_data_varint(-9));
        tail.extend_from_slice(&test_support::entity_data_varint(1));
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
        // a negative floats count inside the model data (the first of
        // the four list lanes)
        let mut tail = Vec::new();
        tail.extend_from_slice(&test_support::entity_data_varint(t::CUSTOM_MODEL_DATA));
        tail.extend_from_slice(&test_support::entity_data_varint(-1));
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
        // a negative hidden-set count
        let mut tail = Vec::new();
        tail.extend_from_slice(&test_support::entity_data_varint(t::TOOLTIP_DISPLAY));
        tail.push(0);
        tail.extend_from_slice(&test_support::entity_data_varint(-2));
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
        // a negative id inside the hidden set
        let mut tail = Vec::new();
        tail.extend_from_slice(&test_support::entity_data_varint(t::TOOLTIP_DISPLAY));
        tail.push(0);
        tail.extend_from_slice(&test_support::entity_data_varint(1));
        tail.extend_from_slice(&test_support::entity_data_varint(-5));
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
    }

    /// The patch value codecs' THIRD WAVE: the adventure-mode pair —
    /// can_place_on (14) and can_break (15), the SAME
    /// `AdventureModePredicate` record under two registry slots. The
    /// audit (javap, four hops): the record = `composite(
    /// BlockPredicate.STREAM_CODEC.list(), ..)`; one predicate =
    /// FOUR lanes in order — `optional(holderSet(BLOCK))` (the $31
    /// VarInt TAG: 0 = a named set's UTF identifier, >= 1 = a direct
    /// set of tag-minus-one byIdOrThrow ids), `optional(
    /// StatePropertiesPredicate)` (a `PropertyMatcher.list()` whose
    /// value lane is `ByteBufCodecs.either(exact UTF, ranged
    /// {optional UTF, optional UTF})` — a bare readBoolean
    /// discriminator, both bounds riding as STRINGS), `optional(
    /// NbtPredicate)` (a bare COMPOUND_TAG, non-compound refuses),
    /// and `DataComponentMatchers` (the exact list decodes through
    /// the patch's added-entry walk; the partial list is 64-capped
    /// and WALKS the dispatched entries — each a ONE-BYTE either
    /// boolean + a byIdOrThrow VarInt + ONE full NBT payload tag).
    #[test]
    fn the_adventure_mode_predicates_decode_their_four_lanes() {
        use data_component_types as t;
        use entity_data_serializers as s;

        // the FULL four-lane predicate, both set shapes at once:
        // blocks = named "minecraft:stone"; properties = two matchers
        // (exact + ranged); nbt = one compound; matchers = one exact
        // entry (damage = 5) and an empty partial list
        let mut value = Vec::new();
        for v in [1i32, 1, 1, 0] {
            value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        value.extend_from_slice(&test_support::entity_data_varint(t::CAN_PLACE_ON));
        value.extend_from_slice(&test_support::entity_data_varint(1)); // one predicate
        value.push(1); // blocks present
        value.extend_from_slice(&test_support::entity_data_varint(0)); // tag 0 = NAMED
        write_string(&mut value, "minecraft:stone");
        value.push(1); // properties present
        value.extend_from_slice(&test_support::entity_data_varint(2)); // two matchers
        write_string(&mut value, "facing");
        value.push(1); // either LEFT = exact
        write_string(&mut value, "north");
        write_string(&mut value, "powered");
        value.push(0); // either RIGHT = ranged
        value.push(1); // min present
        write_string(&mut value, "10");
        value.push(0); // max absent
        value.push(1); // nbt present
        crate::nbt::write_bare_nbt(
            &Nbt::Compound(vec![("Items".into(), Nbt::List(vec![]))]),
            &mut value,
        )
        .unwrap();
        value.extend_from_slice(&test_support::entity_data_varint(1)); // exact count
        value.extend_from_slice(&test_support::entity_data_varint(3)); // damage
        value.extend_from_slice(&test_support::entity_data_varint(5));
        value.extend_from_slice(&test_support::entity_data_varint(0)); // partial empty
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
        );
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ItemStack {
                count: 1,
                item_id: 1,
                patch: ComponentPatch {
                    added: vec![(
                        t::CAN_PLACE_ON,
                        ComponentValue::BlockPredicates(vec![BlockPredicate {
                            blocks: Some(BlockSet::Named("minecraft:stone".into())),
                            properties: Some(vec![
                                PropertyMatcher {
                                    name: "facing".into(),
                                    value: PropertyValue::Exact("north".into())
                                },
                                PropertyMatcher {
                                    name: "powered".into(),
                                    value: PropertyValue::Ranged {
                                        min: Some("10".into()),
                                        max: None
                                    }
                                },
                            ]),
                            nbt: Some(Nbt::Compound(vec![("Items".into(), Nbt::List(vec![]))])),
                            matchers: DataComponentMatchers {
                                exact: vec![(3, ComponentValue::VarInt(5))],
                                partial: vec![],
                            },
                        }])
                    )],
                    removed: vec![]
                }
            }
        );

        // the DIRECT set lane: wire tag 3 = TWO ids (the tag-minus-one
        // arithmetic, the EMPTY direct set is wire tag 1) — and the
        // can_break id decodes through the SAME walk
        let mut value = Vec::new();
        for v in [1i32, 1, 1, 0] {
            value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        value.extend_from_slice(&test_support::entity_data_varint(t::CAN_BREAK));
        value.extend_from_slice(&test_support::entity_data_varint(1));
        value.push(1); // blocks present
        value.extend_from_slice(&test_support::entity_data_varint(3)); // tag 3 = direct, 2 ids
        value.extend_from_slice(&test_support::entity_data_varint(17));
        value.extend_from_slice(&test_support::entity_data_varint(999));
        value.push(0); // properties absent
        value.push(0); // nbt absent
        value.extend_from_slice(&test_support::entity_data_varint(0)); // exact empty
        value.extend_from_slice(&test_support::entity_data_varint(0)); // partial empty
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
        );
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ItemStack {
                count: 1,
                item_id: 1,
                patch: ComponentPatch {
                    added: vec![(
                        t::CAN_BREAK,
                        ComponentValue::BlockPredicates(vec![BlockPredicate {
                            blocks: Some(BlockSet::Direct(vec![17, 999])),
                            properties: None,
                            nbt: None,
                            matchers: DataComponentMatchers { exact: vec![], partial: vec![] },
                        }])
                    )],
                    removed: vec![]
                }
            }
        );

        // the ALL-ABSENT predicate (the ANY-blocks shape): all three
        // optional lanes false, both matcher lists empty — four lane
        // bytes total; and the EMPTY predicate list (count 0) — the
        // record with nothing to match
        let mut value = Vec::new();
        for v in [1i32, 1, 2, 0] {
            value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        value.extend_from_slice(&test_support::entity_data_varint(t::CAN_PLACE_ON));
        value.extend_from_slice(&test_support::entity_data_varint(1)); // one predicate
        value.push(0); // blocks absent
        value.push(0); // properties absent
        value.push(0); // nbt absent
        value.extend_from_slice(&test_support::entity_data_varint(0)); // exact empty
        value.extend_from_slice(&test_support::entity_data_varint(0)); // partial empty
        value.extend_from_slice(&test_support::entity_data_varint(t::CAN_BREAK));
        value.extend_from_slice(&test_support::entity_data_varint(0)); // EMPTY predicate list
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
        );
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::ItemStack {
                count: 1,
                item_id: 1,
                patch: ComponentPatch {
                    added: vec![
                        (
                            t::CAN_PLACE_ON,
                            ComponentValue::BlockPredicates(vec![BlockPredicate {
                                blocks: None,
                                properties: None,
                                nbt: None,
                                matchers: DataComponentMatchers { exact: vec![], partial: vec![] },
                            }])
                        ),
                        (t::CAN_BREAK, ComponentValue::BlockPredicates(vec![])),
                    ],
                    removed: vec![]
                }
            }
        );
    }

    /// The adventure-mode pair's SHAPE refusals: a negative predicate
    /// count, a negative block id inside a direct set (the
    /// byIdOrThrow direction), a non-compound nbt lane (the
    /// COMPOUND_TAG DecoderException), an unknown component id inside
    /// the exact matcher list (the unframed-wire sizing line), the
    /// dispatched partial lane's OWN refusal map (an unknown
    /// predicate-type id, a NEGATIVE one, an unknown and a negative
    /// any-value component id — the `$29` byIdOrThrow lookups — and
    /// the End-tag payload byte, the bridge's "Expected non-null
    /// compound tag" DecoderException), and a partial list over its
    /// 64-entry cap.
    #[test]
    fn the_adventure_mode_predicates_refuse_their_shape_drift() {
        use data_component_types as t;
        use entity_data_serializers as s;

        let stack_with = |value: &[u8]| -> Vec<u8> {
            test_support::set_entity_data_packet(
                5,
                &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
            )
        };
        let with_base = |tail: &[u8]| -> Vec<u8> {
            let mut v = Vec::new();
            for b in [1i32, 1, 1, 0] {
                v.extend_from_slice(&test_support::entity_data_varint(b));
            }
            v.extend_from_slice(tail);
            v
        };
        // a negative predicate list count
        let mut tail = Vec::new();
        tail.extend_from_slice(&test_support::entity_data_varint(t::CAN_PLACE_ON));
        tail.extend_from_slice(&test_support::entity_data_varint(-1));
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
        // a negative block id inside a direct set
        let mut tail = Vec::new();
        tail.extend_from_slice(&test_support::entity_data_varint(t::CAN_PLACE_ON));
        tail.extend_from_slice(&test_support::entity_data_varint(1));
        tail.push(1); // blocks present
        tail.extend_from_slice(&test_support::entity_data_varint(2)); // direct, 1 id
        tail.extend_from_slice(&test_support::entity_data_varint(-7));
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
        // a non-compound nbt lane
        let mut tail = Vec::new();
        tail.extend_from_slice(&test_support::entity_data_varint(t::CAN_BREAK));
        tail.extend_from_slice(&test_support::entity_data_varint(1));
        tail.push(0); // blocks absent
        tail.push(0); // properties absent
        tail.push(1); // nbt present
        crate::nbt::write_bare_nbt(&Nbt::String("not a compound".into()), &mut tail).unwrap();
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
        // an unknown component id inside the exact matcher list
        let mut tail = Vec::new();
        tail.extend_from_slice(&test_support::entity_data_varint(t::CAN_BREAK));
        tail.extend_from_slice(&test_support::entity_data_varint(1));
        tail.push(0);
        tail.push(0);
        tail.push(0);
        tail.extend_from_slice(&test_support::entity_data_varint(1)); // exact count
        tail.extend_from_slice(&test_support::entity_data_varint(999));
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
        // the dispatched partial lane's own refusal map: the $29
        // byIdOrThrow lookups refuse BEFORE the payload is read, so
        // these tails stop at the type id (a refusal probe must land
        // ON the lane, not die of byte exhaustion)
        let partial_tail = |bool_byte: u8, id: i32| -> Vec<u8> {
            let mut t = Vec::new();
            t.extend_from_slice(&test_support::entity_data_varint(t::CAN_PLACE_ON));
            t.extend_from_slice(&test_support::entity_data_varint(1));
            t.push(0);
            t.push(0);
            t.push(0);
            t.extend_from_slice(&test_support::entity_data_varint(0)); // exact empty
            t.extend_from_slice(&test_support::entity_data_varint(1)); // partial count 1
            t.push(bool_byte);
            t.extend_from_slice(&test_support::entity_data_varint(id));
            t
        };
        // an unknown PREDICATE-type id (the built-in table has 15 rows)
        assert!(parse_set_entity_data(&stack_with(&with_base(&partial_tail(1, 15)))).is_err());
        // a NEGATIVE predicate-type id
        assert!(parse_set_entity_data(&stack_with(&with_base(&partial_tail(1, -1)))).is_err());
        // an unknown ANY-VALUE component id (the synced table has 111 rows)
        assert!(parse_set_entity_data(&stack_with(&with_base(&partial_tail(0, 111)))).is_err());
        // a negative any-value component id
        assert!(parse_set_entity_data(&stack_with(&with_base(&partial_tail(0, -3)))).is_err());
        // the End-tag payload byte: a KNOWN type id (left, damage) then
        // 0x00 — readNbt maps the End tag to null and the bridge throws
        // "Expected non-null compound tag" (the port's bare-tag reader
        // refuses the same byte at the same lane)
        let mut tail = partial_tail(1, 0);
        tail.push(0x00);
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
        // a partial list over its 64-entry cap (the count itself is
        // the DecoderException, before any payload)
        let mut tail = Vec::new();
        tail.extend_from_slice(&test_support::entity_data_varint(t::CAN_BREAK));
        tail.extend_from_slice(&test_support::entity_data_varint(1));
        tail.push(0);
        tail.push(0);
        tail.push(0);
        tail.extend_from_slice(&test_support::entity_data_varint(0));
        tail.extend_from_slice(&test_support::entity_data_varint(65));
        assert!(parse_set_entity_data(&stack_with(&with_base(&tail))).is_err());
    }

    /// The dispatched predicate-type census WALKS: a partial matcher
    /// list of TWO entries — LEFT (the `damage` predicate type, id 0,
    /// its payload compound retained verbatim) and RIGHT (the
    /// AnyValue lane over the DAMAGE component type, "the component
    /// is present, any value", its empty-compound payload retained) —
    /// and the walk's framing: the payload tag is self-delimiting, so
    /// the decode consumes exactly each entry and continues (the
    /// wave-three "unframed wire" objection retired by construction).
    #[test]
    fn the_dispatched_predicate_entries_walk_their_payload_tags() {
        use data_component_predicate_types as pt;
        use data_component_types as t;
        use entity_data_serializers as s;

        let mut value = Vec::new();
        for v in [1i32, 1, 1, 0] {
            value.extend_from_slice(&test_support::entity_data_varint(v));
        }
        value.extend_from_slice(&test_support::entity_data_varint(t::CAN_PLACE_ON));
        value.extend_from_slice(&test_support::entity_data_varint(1)); // one predicate
        value.push(0); // blocks absent
        value.push(0); // properties absent
        value.push(0); // nbt absent
        value.extend_from_slice(&test_support::entity_data_varint(0)); // exact empty
        value.extend_from_slice(&test_support::entity_data_varint(2)); // partial: two entries
        // entry 1: LEFT (bool 1) = the `damage` predicate type, the
        // payload the record codec's NBT-schema shape (retained
        // verbatim — the parse is a consumer's act)
        value.push(1);
        value.extend_from_slice(&test_support::entity_data_varint(pt::DAMAGE));
        crate::nbt::write_bare_nbt(
            &Nbt::Compound(vec![
                ("durable".into(), Nbt::Int(20)),
                ("damage".into(), Nbt::Int(3)),
            ]),
            &mut value,
        )
        .unwrap();
        // entry 2: RIGHT (bool 0) = the AnyValue lane over the DAMAGE
        // component type, the payload the empty compound
        value.push(0);
        value.extend_from_slice(&test_support::entity_data_varint(t::DAMAGE));
        crate::nbt::write_bare_nbt(&Nbt::Compound(vec![]), &mut value).unwrap();

        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
        );
        let d = parse_set_entity_data(&p).unwrap();
        let EntityDataEntry::ItemStack { patch, .. } = &d.entries[0].value else {
            panic!("expected the item stack");
        };
        assert!(patch.removed.is_empty());
        let [(ty, ComponentValue::BlockPredicates(preds))] = &patch.added[..] else {
            panic!("expected the can_place_on block predicates");
        };
        assert_eq!(*ty, t::CAN_PLACE_ON);
        let [pred] = &preds[..] else {
            panic!("expected one predicate");
        };
        assert!(pred.blocks.is_none() && pred.properties.is_none() && pred.nbt.is_none());
        assert!(pred.matchers.exact.is_empty());
        // entry 1: the LEFT lane — the built-in table's `damage` row,
        // the payload compound byte-identical to the wire's
        assert_eq!(
            pred.matchers.partial[0].predicate_type,
            PredicateTypeLane::PredicateType(pt::DAMAGE)
        );
        assert_eq!(
            pred.matchers.partial[0].payload,
            Nbt::Compound(vec![
                ("durable".into(), Nbt::Int(20)),
                ("damage".into(), Nbt::Int(3)),
            ])
        );
        // entry 2: the RIGHT lane — the AnyValue shape over the DAMAGE
        // component type (the registry id, not a value codec)
        assert_eq!(
            pred.matchers.partial[1].predicate_type,
            PredicateTypeLane::ComponentType(t::DAMAGE)
        );
        assert_eq!(pred.matchers.partial[1].payload, Nbt::Compound(vec![]));
    }

    /// The idMapper ordinal lanes NEVER refuse: `Rarity.BY_ID` =
    /// `ByIdMap.continuous(.., OutOfBoundsStrategy.ZERO)` (every key
    /// outside [0, 4) fills to COMMON — the decode never throws),
    /// `Pose.BY_ID` the same ZERO shape, `Direction.BY_ID` the WRAP
    /// shape (out-of-range keys floorMod into [0, 6)). The port
    /// retains every wire ordinal VERBATIM — negative AND
    /// out-of-range — the fill/wrap being the consumer's resolution
    /// (the operation/slot/animation line; the Rarity lane's old
    /// negative refusal was STRICTER than vanilla's map, and the
    /// strictness was the invention).
    #[test]
    fn the_id_mapper_ordinal_lanes_never_refuse() {
        use data_component_types as t;
        use entity_data_serializers as s;

        // the entity-data lanes: a NEGATIVE and an out-of-range
        // ordinal decode verbatim
        for (serializer, id, expected) in [
            (s::DIRECTION, -1, EntityDataEntry::Direction(-1)),
            (s::DIRECTION, 42, EntityDataEntry::Direction(42)),
            (s::POSE, -1, EntityDataEntry::Pose(-1)),
            (s::POSE, 99, EntityDataEntry::Pose(99)),
        ] {
            let p = test_support::set_entity_data_packet(
                7,
                &test_support::entity_data_entry(
                    3,
                    serializer,
                    &test_support::entity_data_varint(id),
                ),
            );
            let d = parse_set_entity_data(&p).unwrap();
            assert_eq!(d.entries[0].value, expected);
        }

        // the component lane: rarity over the patch walk — the
        // negative and out-of-range ordinals the OLD lane refused now
        // decode verbatim (vanilla's map would ZERO-fill both to
        // COMMON in its own resolution)
        for ordinal in [-2, 9] {
            let mut value = Vec::new();
            for v in [1i32, 1, 1, 0] {
                value.extend_from_slice(&test_support::entity_data_varint(v));
            }
            value.extend_from_slice(&test_support::entity_data_varint(t::RARITY));
            value.extend_from_slice(&test_support::entity_data_varint(ordinal));
            let p = test_support::set_entity_data_packet(
                5,
                &test_support::entity_data_entry(8, s::ITEM_STACK, &value),
            );
            let d = parse_set_entity_data(&p).unwrap();
            let EntityDataEntry::ItemStack { patch, .. } = &d.entries[0].value else {
                panic!("expected the item stack");
            };
            assert_eq!(
                patch.added,
                vec![(t::RARITY, ComponentValue::Rarity(ordinal))]
            );
        }
    }

    /// Set Entity Data: the COMPONENT wire is a BARE NBT tag — the
    /// plain-text name arrives as a TAG_String, the styled name as a
    /// TAG_Compound, and both decode verbatim into the entry (the
    /// rich-text schema stays a rendering-time concern; the store is
    /// the tag). The CustomName surface's own transport.
    #[test]
    fn the_component_wire_is_a_bare_nbt_tag() {
        use entity_data_serializers as s;
        // plain text: 0x08 + u16 len + bytes
        let mut plain = Vec::new();
        crate::nbt::write_bare_nbt(&Nbt::String("Duke".into()), &mut plain).unwrap();
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(2, s::COMPONENT, &plain),
        );
        let d = parse_set_entity_data(&p).unwrap();
        assert_eq!(
            d.entries[0].value,
            EntityDataEntry::Component(Nbt::String("Duke".into()))
        );
        // the styled form: a compound with text/color/bold — read back
        // positionally through the retained tree
        let mut styled = Vec::new();
        crate::nbt::write_bare_nbt(
            &Nbt::Compound(vec![
                ("text".into(), Nbt::String("A".into())),
                ("color".into(), Nbt::String("red".into())),
                ("bold".into(), Nbt::Byte(1)),
            ]),
            &mut styled,
        )
        .unwrap();
        let mut styled_wire = vec![1u8];
        styled_wire.extend_from_slice(&styled);
        let p = test_support::set_entity_data_packet(
            5,
            &test_support::entity_data_entry(2, s::OPTIONAL_COMPONENT, &styled_wire),
        );
        let d = parse_set_entity_data(&p).unwrap();
        match &d.entries[0].value {
            EntityDataEntry::OptionalComponent(Some(Nbt::Compound(entries))) => {
                assert_eq!(entries[0].0, "text");
                assert_eq!(entries[0].1, Nbt::String("A".into()));
                assert_eq!(entries[2].1, Nbt::Byte(1), "the bold flag survives");
            }
            other => panic!("expected the styled component, got {other:?}"),
        }
    }

    /// Set Entity Data: the generated id and the re-exported constant
    /// agree (the re-export list is part of the wire surface — the
    /// pump arm is `SET_ENTITY_DATA => EntityData(parse_set_entity_data)`,
    /// a one-line match the compiler type-checks against the variant).
    #[test]
    fn the_entity_data_id_is_the_generated_runtime_fact() {
        assert_eq!(clientbound::SET_ENTITY_DATA, 99, "minecraft:set_entity_data");
        let entries = test_support::entity_data_entry(0, entity_data_serializers::BOOLEAN, &[1]);
        let p = test_support::set_entity_data_packet(4, &entries);
        assert_eq!(parse_set_entity_data(&p).unwrap().id, 4);
    }

    /// The mob-effect lanes: the generated ids and the re-exported
    /// constants agree; the update's flat five-field walk retains every
    /// field verbatim (the -1 infinite sentinel included — the decode
    /// validates nothing); the negative effect id refuses (the
    /// holderRegistry byIdOrThrow direction); the remove rides the
    /// record's two fields; a truncated update refuses honestly.
    #[test]
    fn the_mob_effect_lanes_retain_the_player_state() {
        assert_eq!(clientbound::UPDATE_MOB_EFFECT, 132, "minecraft:update_mob_effect");
        assert_eq!(clientbound::REMOVE_MOB_EFFECT, 78, "minecraft:remove_mob_effect");
        // the full update: entity 17, the night_vision registry id 15,
        // amplifier 2, duration 600 ticks, the visible|show-icon flags
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::UPDATE_MOB_EFFECT);
        write_varint(&mut p, 17);
        write_varint(&mut p, 15);
        write_varint(&mut p, 2);
        write_varint(&mut p, 600);
        p.push(0b0110);
        let u = parse_update_mob_effect(&p).unwrap();
        assert_eq!(u.entity_id, 17);
        assert_eq!(u.effect_id, 15, "the registry id, not the legacy wiki 16");
        assert_eq!(u.amplifier, 2);
        assert_eq!(u.duration, 600);
        assert_eq!(u.flags, 0b0110);
        // the infinite sentinel and the blend bit ride verbatim
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::UPDATE_MOB_EFFECT);
        write_varint(&mut p, 1);
        write_varint(&mut p, 28);
        write_varint(&mut p, 0);
        write_varint(&mut p, -1);
        p.push(0b1000);
        let u = parse_update_mob_effect(&p).unwrap();
        assert_eq!(u.duration, -1, "the wire's infinite sentinel");
        assert_eq!(u.flags, 0b1000, "the blend bit");
        // the negative effect id refuses (the byIdOrThrow direction)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::UPDATE_MOB_EFFECT);
        write_varint(&mut p, 1);
        write_varint(&mut p, -3);
        assert!(parse_update_mob_effect(&p).is_err());
        // the remove: the record's two fields
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REMOVE_MOB_EFFECT);
        write_varint(&mut p, 17);
        write_varint(&mut p, 15);
        let r = parse_remove_mob_effect(&p).unwrap();
        assert_eq!(r.entity_id, 17);
        assert_eq!(r.effect_id, 15);
        // a truncated update refuses honestly (the flags byte short)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::UPDATE_MOB_EFFECT);
        write_varint(&mut p, 17);
        write_varint(&mut p, 15);
        write_varint(&mut p, 2);
        write_varint(&mut p, 600);
        assert!(parse_update_mob_effect(&p).is_err());
    }

    /// The Set Equipment lanes retain the entity's gear: the paired
    /// walk (the slot byte's low 7 bits index the eight-entry
    /// declaration-order VALUES list, the HIGH bit is the continue
    /// flag), the stack riding ItemStack.OPTIONAL_STREAM_CODEC (the
    /// ItemStack$1 walk — an EMPTY stack is a legal entry here, the
    /// cleared slot), the bounds refusal firing BEFORE the stack bytes
    /// (Java's List.get order), and the continue-bit end refusing
    /// honestly (the loop wants the next pair).
    #[test]
    fn the_set_equipment_lanes_retain_the_entitys_gear() {
        use data_component_types as t;
        assert_eq!(clientbound::SET_EQUIPMENT, 102, "minecraft:set_equipment");
        // three pairs: mainhand (continue bit) with a full stack and a
        // one-component patch, body (continue bit) with the CLEARED
        // slot — an empty stack, legal on the optional lane — and the
        // final offhand pair whose byte has the high bit CLEAR
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_EQUIPMENT);
        write_varint(&mut p, 42);
        p.push(0x80); // slot 0 mainhand | continue
        write_varint(&mut p, 3); // count
        write_varint(&mut p, 745); // item registry id
        write_varint(&mut p, 1); // patch: one added
        write_varint(&mut p, 0); // patch: none removed
        write_varint(&mut p, t::MAX_STACK_SIZE);
        write_varint(&mut p, 35); // the VarInt value
        p.push(0x86); // slot 6 body | continue
        write_varint(&mut p, 0); // the EMPTY stack: the count IS the sentinel
        p.push(0x01); // slot 1 offhand, the FINAL pair — high bit clear
        write_varint(&mut p, 1);
        write_varint(&mut p, 2);
        write_varint(&mut p, 0); // patch: none added
        write_varint(&mut p, 0); // patch: none removed
        let e = parse_set_equipment(&p).unwrap();
        assert_eq!(e.entity_id, 42);
        assert_eq!(e.slots.len(), 3);
        assert_eq!(e.slots[0].slot, EquipmentSlot::Mainhand);
        assert_eq!(e.slots[0].stack.count, 3);
        assert_eq!(e.slots[0].stack.item_id, 745);
        assert_eq!(
            e.slots[0].stack.patch,
            ComponentPatch { added: vec![(t::MAX_STACK_SIZE, ComponentValue::VarInt(35))], removed: vec![] }
        );
        assert_eq!(e.slots[1].slot, EquipmentSlot::Body);
        assert_eq!(
            e.slots[1].stack,
            ItemStack { count: 0, item_id: 0, patch: ComponentPatch::default() },
            "the cleared slot rides the optional lane as a LEGAL empty stack"
        );
        assert_eq!(e.slots[2].slot, EquipmentSlot::Offhand);
        assert_eq!(e.slots[2].stack.count, 1);
        assert_eq!(e.slots[2].stack.item_id, 2);
        assert_eq!(e.slots[2].stack.patch, ComponentPatch::default());
        // the empty sentinel's negative shape rides verbatim too (the
        // count is retained, never normalized to zero)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_EQUIPMENT);
        write_varint(&mut p, 1);
        p.push(0x04); // slot 4 chest, final
        write_varint(&mut p, -7);
        let e = parse_set_equipment(&p).unwrap();
        assert_eq!(e.slots[0].stack.count, -7, "the raw sentinel retained");
        assert_eq!(e.slots[0].stack.item_id, 0);
        // the bounds refusal: slot index 8 is outside the eight-entry
        // VALUES list, and Java's List.get fires BEFORE the stack
        // bytes — the port refuses on the slot byte ALONE
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_EQUIPMENT);
        write_varint(&mut p, 42);
        p.push(0x88); // slot 8 | continue
        assert!(parse_set_equipment(&p).is_err());
        // the continue-bit end refuses honestly: the last byte keeps
        // the high bit SET, so the loop wants the next pair and the
        // buffer is short
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_EQUIPMENT);
        write_varint(&mut p, 42);
        p.push(0x80); // mainhand, continue
        write_varint(&mut p, 0); // the empty stack
        assert!(parse_set_equipment(&p).is_err());
        // a final pair's stack truncated mid-patch refuses (the patch
        // counts are short)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_EQUIPMENT);
        write_varint(&mut p, 42);
        p.push(0x00); // mainhand, final
        write_varint(&mut p, 1);
        write_varint(&mut p, 2);
        assert!(parse_set_equipment(&p).is_err());
    }

    /// The Set Container Content lanes retain the window: the four-part
    /// composite (the VarInt container id — the CONTAINER_ID lane's own
    /// shape, NOT the wiki-era byte —, the raw state id, the
    /// collection-count walk, the carried item), an EMPTY stack legal
    /// on the items lane, the negative count refusing at the collection
    /// CONSTRUCTION (Java's Illegal Capacity, BEFORE any entry bytes —
    /// the readCount comparison passes), and the lying/truncated counts
    /// refusing honestly.
    #[test]
    fn the_container_content_lanes_retain_the_window() {
        use data_component_types as t;
        assert_eq!(
            clientbound::CONTAINER_SET_CONTENT, 18,
            "minecraft:container_set_content"
        );
        // three slots + the carried stack: container 200 (a two-byte
        // VarInt — the byte-era table would have died here), state 7,
        // slot 0 a full stack with a one-component patch, slot 1 the
        // CLEARED slot, slot 2 a patch-less stack, the cursor empty
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 200);
        write_varint(&mut p, 7);
        write_varint(&mut p, 3); // the collection count
        write_varint(&mut p, 12); // slot 0 count
        write_varint(&mut p, 745); // item registry id
        write_varint(&mut p, 1); // patch: one added
        write_varint(&mut p, 0); // patch: none removed
        write_varint(&mut p, t::MAX_STACK_SIZE);
        write_varint(&mut p, 35); // the VarInt value
        write_varint(&mut p, 0); // slot 1: the EMPTY stack, nothing rides
        write_varint(&mut p, 1); // slot 2 count
        write_varint(&mut p, 33); // item registry id
        write_varint(&mut p, 0); // patch: none added
        write_varint(&mut p, 0); // patch: none removed
        write_varint(&mut p, 0); // the carried item: the cursor, empty
        let c = parse_set_container_content(&p).unwrap();
        assert_eq!(c.container_id, 200, "the VarInt container id, not a byte");
        assert_eq!(c.state_id, 7);
        assert_eq!(c.items.len(), 3);
        assert_eq!(c.items[0].count, 12);
        assert_eq!(c.items[0].item_id, 745);
        assert_eq!(
            c.items[0].patch,
            ComponentPatch { added: vec![(t::MAX_STACK_SIZE, ComponentValue::VarInt(35))], removed: vec![] }
        );
        assert_eq!(
            c.items[1],
            ItemStack { count: 0, item_id: 0, patch: ComponentPatch::default() },
            "the cleared slot rides the optional lane as a LEGAL empty stack"
        );
        assert_eq!(c.items[2].count, 1);
        assert_eq!(c.items[2].item_id, 33);
        assert_eq!(c.items[2].patch, ComponentPatch::default());
        assert_eq!(c.carried_item.count, 0, "the empty cursor");
        // the negative count refuses at the collection CONSTRUCTION:
        // readCount's comparison passes (a negative is never >
        // MAX_VALUE), so Java dies at `new ArrayList<>(-n)` — Illegal
        // Capacity, BEFORE any entry bytes are consumed. The stray
        // bytes after the count prove the ordering: the refusal is not
        // an EOF, it is the capacity check.
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 7);
        write_varint(&mut p, -5);
        p.push(1); // entry bytes present, never reached
        assert!(parse_set_container_content(&p).is_err());
        // a lying count refuses honestly: five slots declared, the
        // buffer carries one — the entry loop wants the next slot
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 7);
        write_varint(&mut p, 5);
        write_varint(&mut p, 0); // slot 0: empty
        assert!(parse_set_container_content(&p).is_err());
        // a stack truncated mid-patch refuses (the patch counts short)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 7);
        write_varint(&mut p, 1);
        write_varint(&mut p, 1);
        write_varint(&mut p, 2);
        assert!(parse_set_container_content(&p).is_err());
        // the carried item truncated refuses (the window ends short)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 7);
        write_varint(&mut p, 1);
        write_varint(&mut p, 0); // the one slot
        assert!(parse_set_container_content(&p).is_err());
    }

    /// The single-slot sibling (generated id 20): the record's four
    /// fields in wire order — the VarInt container id, the raw state
    /// id, the SIGNED BE SHORT slot (the round's shape trap: a VarInt
    /// walk would misread the width and die on the high slots), and
    /// the plain OPTIONAL stack (the one-reader line's FOURTH call
    /// site, an EMPTY stack a LEGAL cleared slot). The truncation map:
    /// each lane short refuses honestly.
    #[test]
    fn the_container_slot_lanes_retain_the_single_slot() {
        use data_component_types as t;
        assert_eq!(
            clientbound::CONTAINER_SET_SLOT, 20,
            "minecraft:container_set_slot"
        );
        // window 200 (the two-byte VarInt — the byte-era table dies),
        // state 9, slot 200 (0x00C8 — past a VarInt's single-byte
        // range: the short lane's own proof), the stack a full one
        // with a one-component patch
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 200);
        write_varint(&mut p, 9);
        p.extend_from_slice(&200i16.to_be_bytes()); // the readShort lane
        write_varint(&mut p, 12); // count
        write_varint(&mut p, 745); // item registry id
        write_varint(&mut p, 1); // patch: one added
        write_varint(&mut p, 0); // patch: none removed
        write_varint(&mut p, t::MAX_STACK_SIZE);
        write_varint(&mut p, 35); // the VarInt value
        let s = parse_set_container_slot(&p).unwrap();
        assert_eq!(s.container_id, 200, "the VarInt container id, not a byte");
        assert_eq!(s.state_id, 9);
        assert_eq!(s.slot, 200, "the slot rode a SIGNED BE SHORT, not a VarInt");
        assert_eq!(s.item.count, 12);
        assert_eq!(s.item.item_id, 745);
        assert_eq!(
            s.item.patch,
            ComponentPatch { added: vec![(t::MAX_STACK_SIZE, ComponentValue::VarInt(35))], removed: vec![] }
        );
        // the negative slot retained verbatim (the consumer's bounds
        // check is the consumer's act) and the CLEARED slot legal
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 1);
        p.extend_from_slice(&(-2i16).to_be_bytes());
        write_varint(&mut p, 0); // the EMPTY stack: the cleared slot
        let s = parse_set_container_slot(&p).unwrap();
        assert_eq!(s.slot, -2);
        assert_eq!(s.item, ItemStack { count: 0, item_id: 0, patch: ComponentPatch::default() });
        // the truncation map: the slot short short, the stack short,
        // the patch short — each lane refuses honestly
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 1);
        p.push(0); // one byte of the slot's two
        assert!(parse_set_container_slot(&p).is_err());
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 1);
        p.extend_from_slice(&0i16.to_be_bytes());
        // the stack: a count with no item behind it
        write_varint(&mut p, 1);
        assert!(parse_set_container_slot(&p).is_err());
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 1);
        p.extend_from_slice(&0i16.to_be_bytes());
        write_varint(&mut p, 1); // count
        write_varint(&mut p, 2); // item id, the patch counts missing
        assert!(parse_set_container_slot(&p).is_err());
        // a wrong packet id refuses (the generated-id guard)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        assert!(parse_set_container_slot(&p).is_err());
    }

    /// The inventory trilogy's write side: the three packets' writers
    /// ride the round-124 mirror line — every fixture the audits built
    /// now proves BOTH directions, byte-equal. The zero-pair refusal
    /// (the decode reads the first slot byte unconditionally), the
    /// continue-bit placement (the LAST pair's byte always has the
    /// high bit CLEAR — Java's `more` is the iterator's hasNext, not
    /// the entry's property), the sentinel's verbatim five bytes
    /// re-encoded, and the BE SHORT slot's own width mirror.
    #[test]
    fn the_inventory_trilogys_writers_round_trip_the_fixture_sets() {
        use data_component_types as t;
        // -- Set Equipment: the three-pair fixture (the mainhand with a
        //    one-component patch, the CLEARED body slot riding the
        //    optional lane, the final offhand pair) round-trips
        //    BYTE-EQUAL — the continue bits on the first two slot
        //    bytes, the final byte bare.
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_EQUIPMENT);
        write_varint(&mut p, 42);
        p.push(0x80); // slot 0 mainhand | continue
        write_varint(&mut p, 3);
        write_varint(&mut p, 745);
        write_varint(&mut p, 1);
        write_varint(&mut p, 0);
        write_varint(&mut p, t::MAX_STACK_SIZE);
        write_varint(&mut p, 35);
        p.push(0x86); // slot 6 body | continue
        write_varint(&mut p, 0); // the EMPTY stack
        p.push(0x01); // slot 1 offhand, the FINAL pair
        write_varint(&mut p, 1);
        write_varint(&mut p, 2);
        write_varint(&mut p, 0);
        write_varint(&mut p, 0);
        let e = parse_set_equipment(&p).unwrap();
        assert_eq!(
            encode_set_equipment(e.entity_id, &e.slots).unwrap(),
            p,
            "the three-pair equipment walk re-encodes byte-equal"
        );
        // the negative sentinel re-encodes VERBATIM (five bytes, no
        // item, no patch — the writer writes the STORE, not the
        // collapse vanilla's strict lane refuses)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_EQUIPMENT);
        write_varint(&mut p, 1);
        p.push(0x04); // slot 4 chest, final
        write_varint(&mut p, -7);
        let e = parse_set_equipment(&p).unwrap();
        assert_eq!(encode_set_equipment(e.entity_id, &e.slots).unwrap(), p);
        // the zero-pair refusal: the decode reads the first slot byte
        // unconditionally, so a packet with no pairs cannot exist
        assert!(encode_set_equipment(42, &[]).is_err());
        // -- Set Container Content: the window fixture (container 200,
        //    the two-byte VarInt the byte-era table would have died
        //    on, three slots, the empty cursor) round-trips byte-equal
        //    — the count lane riding the store's OWN length.
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 200);
        write_varint(&mut p, 7);
        write_varint(&mut p, 3);
        write_varint(&mut p, 12);
        write_varint(&mut p, 745);
        write_varint(&mut p, 1);
        write_varint(&mut p, 0);
        write_varint(&mut p, t::MAX_STACK_SIZE);
        write_varint(&mut p, 35);
        write_varint(&mut p, 0); // slot 1: the EMPTY stack
        write_varint(&mut p, 1);
        write_varint(&mut p, 33);
        write_varint(&mut p, 0);
        write_varint(&mut p, 0);
        write_varint(&mut p, 0); // the carried item: the cursor, empty
        let c = parse_set_container_content(&p).unwrap();
        assert_eq!(
            encode_set_container_content(
                c.container_id,
                c.state_id,
                &c.items,
                &c.carried_item
            )
            .unwrap(),
            p,
            "the window snapshot re-encodes byte-equal"
        );
        // -- Set Container Slot: the single-slot fixture (the BE SHORT
        //    slot 200 — 0x00C8, past a VarInt's single-byte range) and
        //    the negative-slot cleared slot both round-trip byte-equal
        //    — the short lane's own mirror proving the WIDTH.
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 200);
        write_varint(&mut p, 9);
        p.extend_from_slice(&200i16.to_be_bytes());
        write_varint(&mut p, 12);
        write_varint(&mut p, 745);
        write_varint(&mut p, 1);
        write_varint(&mut p, 0);
        write_varint(&mut p, t::MAX_STACK_SIZE);
        write_varint(&mut p, 35);
        let s = parse_set_container_slot(&p).unwrap();
        assert_eq!(
            encode_set_container_slot(s.container_id, s.state_id, s.slot, &s.item).unwrap(),
            p,
            "the single-slot record re-encodes byte-equal"
        );
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 1);
        p.extend_from_slice(&(-2i16).to_be_bytes());
        write_varint(&mut p, 0); // the EMPTY stack: the cleared slot
        let s = parse_set_container_slot(&p).unwrap();
        assert_eq!(
            encode_set_container_slot(s.container_id, s.state_id, s.slot, &s.item).unwrap(),
            p,
            "the negative slot re-encodes verbatim"
        );
    }

    /// The hotbar's selection packet (generated id 105): the ONE-lane
    /// raw VarInt record (the wiki-era byte table would misshape —
    /// slot 200 rides TWO bytes, the byte lane would read a negative),
    /// the generated-id pin, the verbatim retention of the values the
    /// consumer's gate will drop (the parse is not the gate), and the
    /// truncation refusing honestly.
    #[test]
    fn the_held_slot_lanes_pin_the_selection() {
        assert_eq!(clientbound::SET_HELD_SLOT, 105, "minecraft:set_held_slot");
        // the selection-relative walk: slot 3 rides ONE byte, slot 8
        // rides ONE byte — the whole legal selection range fits the
        // single-byte VarInt shape
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        write_varint(&mut p, 3);
        let s = parse_set_held_slot(&p).unwrap();
        assert_eq!(s.slot, 3);
        // the name-shaped trap's own proof: slot 200 (a WINDOW slot the
        // consumer's gate will drop) rides the two-byte VarInt C8 01 —
        // the wiki-era signed byte would read -56 — and the parse
        // RETAINS it verbatim: the gate is the consumer's act, the
        // wire's truth is separate
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        write_varint(&mut p, 200);
        assert_eq!(&p[1..], &[0xC8, 0x01], "the two-byte VarInt shape");
        let s = parse_set_held_slot(&p).unwrap();
        assert_eq!(s.slot, 200, "retained verbatim, the gate drops later");
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        write_varint(&mut p, -2);
        let s = parse_set_held_slot(&p).unwrap();
        assert_eq!(s.slot, -2, "the negative rides the VarInt verbatim");
        // the truncation map: the id alone, the slot short
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        assert!(parse_set_held_slot(&p).is_err());
        // a wrong packet id refuses (the generated-id guard)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        assert!(parse_set_held_slot(&p).is_err());
    }

    /// The Open Screen and Container Close packets' wire walks (the
    /// container GUI's own pair — javap of the 26.2
    /// `ClientboundOpenScreenPacket` clinit and the
    /// `ClientboundContainerClosePacket` decode body): the container
    /// id is a BARE VARINT both sides (`ByteBufCodecs.CONTAINER_ID` =
    /// `FriendlyByteBuf.readContainerId` = `VarInt.read`), the menu
    /// type rides the registry(MENU) VarInt, and the title is ONE bare
    /// network NBT tag — walked, retained VERBATIM.
    #[test]
    fn the_open_screen_lanes_pin_the_window() {
        assert_eq!(clientbound::OPEN_SCREEN, 59, "minecraft:open_screen");
        assert_eq!(clientbound::CONTAINER_CLOSE, 17, "minecraft:container_close");
        // the chest's walk: window 5, the menu type generic_9x3 (2),
        // the title a hand-built anonymous-root compound with one
        // string tag — the title's bytes ride back VERBATIM
        let mut title = vec![0x0A];
        title.extend_from_slice(&0u16.to_be_bytes()); // the anonymous root's empty name
        title.push(0x08); // TAG_String
        title.extend_from_slice(&4u16.to_be_bytes());
        title.extend_from_slice(b"name");
        title.extend_from_slice(&6u16.to_be_bytes());
        title.extend_from_slice(b"Chest!");
        title.push(0x00); // end of root
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::OPEN_SCREEN);
        write_varint(&mut p, 5);
        write_varint(&mut p, 2);
        p.extend_from_slice(&title);
        let o = parse_open_screen(&p).unwrap();
        assert_eq!(o.container_id, 5);
        assert_eq!(o.menu_type_id, 2, "generic_9x3, the chest");
        assert_eq!(o.title_nbt, title, "the title's bytes ride verbatim");
        // the byte-era table would misshape: window 200 rides the
        // two-byte VarInt C8 01 (the wiki-era signed byte reads -56)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::OPEN_SCREEN);
        write_varint(&mut p, 200);
        write_varint(&mut p, 0);
        p.extend_from_slice(&[0x0A, 0x00, 0x00]); // the minimal empty root
        let o = parse_open_screen(&p).unwrap();
        assert_eq!(o.container_id, 200, "the bare VarInt shape");
        assert_eq!(o.menu_type_id, 0, "generic_9x1");
        assert_eq!(o.title_nbt, vec![0x0A, 0x00, 0x00]);
        // the truncation map: the id alone, the menu type short, the
        // title's root without its end
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::OPEN_SCREEN);
        write_varint(&mut p, 5);
        assert!(parse_open_screen(&p).is_err());
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::OPEN_SCREEN);
        write_varint(&mut p, 5);
        write_varint(&mut p, 2);
        assert!(parse_open_screen(&p).is_err());
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::OPEN_SCREEN);
        write_varint(&mut p, 5);
        write_varint(&mut p, 2);
        p.extend_from_slice(&[0x0A, 0x00]); // a truncated root: no end tag
        assert!(parse_open_screen(&p).is_err());
        // a wrong packet id refuses (the generated-id guard)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_CLOSE);
        assert!(parse_open_screen(&p).is_err());
        // the close: the same bare VarInt the open screen's id rides
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_CLOSE);
        write_varint(&mut p, 200);
        assert_eq!(&p[1..], &[0xC8, 0x01], "the two-byte VarInt shape");
        let c = parse_container_close(&p).unwrap();
        assert_eq!(c.container_id, 200);
        // the close's truncation and wrong-id guards
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_CLOSE);
        assert!(parse_container_close(&p).is_err());
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::OPEN_SCREEN);
        assert!(parse_container_close(&p).is_err());
    }

    /// Mock server: login -> configuration -> play (the FULL vanilla
    /// arc), then a real world burst: login, center chunk, chunk batch,
    /// chunk data, keep-alive, ping, junk, unload, position teleport.
    /// Asserts every client reply server-side and returns nothing (the
    /// client-side snapshot is asserted by the caller). With `live_tail`
    /// the server keeps speaking AFTER the milestone — a second chunk, a
    /// second teleport, a keep-alive and a play disconnect — the traffic
    /// only a caller still pumping the retained session can serve (the
    /// live view's input stream).
    fn spawn_world_mock() -> (u16, JoinHandle<()>) {
        spawn_world_mock_with_tail(false, Vec::new())
    }

    /// The live-tail variant of the world mock (see `spawn_world_mock`).
    fn spawn_world_mock_live() -> (u16, JoinHandle<()>) {
        spawn_world_mock_with_tail(true, Vec::new())
    }

    /// The world mock whose login success carries profile properties —
    /// the profile-metadata surface (the `textures` entry a real account
    /// carries; the slim/wide selection rides it).
    fn spawn_world_mock_with_props(props: Vec<(String, String)>) -> (u16, JoinHandle<()>) {
        spawn_world_mock_with_tail(false, props)
    }

    fn spawn_world_mock_with_tail(
        live_tail: bool,
        login_props: Vec<(String, String)>,
    ) -> (u16, JoinHandle<()>) {
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
            let props_ref: Vec<(&str, &str)> =
                login_props.iter().map(|(n, v)| (n.as_str(), v.as_str())).collect();
            srv.send_packet(&encode_login_success_with_properties(&[7u8; 16], "mc2rs", &props_ref))
                .unwrap();
            for _ in 0..3 {
                srv.read_packet().unwrap(); // ack, info, brand
            }
            // probes + bulk traffic + finish (configuration) — the ids
            // come from the GENERATED configuration table (values 4/5/3
            // on the real 26.2 wire; the raw wiki-era literals these
            // replaced were one low)
            let mut ka = Vec::new();
            write_varint(&mut ka, crate::config::clientbound::KEEP_ALIVE);
            ka.extend_from_slice(&0x1122334455667788u64.to_be_bytes());
            srv.send_packet(&ka).unwrap();
            let mut pg = Vec::new();
            write_varint(&mut pg, crate::config::clientbound::PING);
            pg.extend_from_slice(&77i32.to_be_bytes());
            srv.send_packet(&pg).unwrap();
            // bulk registry traffic (configuration): the dimension_type
            // registry the client DECODES + the biome registry it RETAINS
            // (a real vanilla server streams the WHOLE worldgen/biome
            // table — here two entries, plains 0 and crimson_forest 1, so
            // the nether's own chunk can carry a different biome id)
            srv.send_packet(&crate::config::test_support::registry_packet(true)).unwrap();
            srv.send_packet(&crate::config::test_support::biome_registry_packet_multi(&[
                "minecraft:plains",
                "minecraft:crimson_forest",
            ]))
            .unwrap();
            // the world_clock id table: the Set Time map keys resolve
            // through it (a real vanilla server streams this registry too)
            srv.send_packet(&crate::config::test_support::world_clock_packet())
                .unwrap();
            // the timeline registry + the common-phase Update Tags
            // payload: the day/night sky curve resolves DATA-DRIVEN
            // through them (a real vanilla server streams both)
            srv.send_packet(&crate::config::test_support::timeline_packet())
                .unwrap();
            srv.send_packet(&crate::config::test_support::update_tags_packet())
                .unwrap();
            let mut fin = Vec::new();
            write_varint(&mut fin, crate::config::clientbound::FINISH);
            srv.send_packet(&fin).unwrap();
            for (id, tail) in [
                (crate::config::serverbound::KEEP_ALIVE, 8i64),
                (crate::config::serverbound::PONG, 77i64),
                (crate::config::serverbound::ACKNOWLEDGE_FINISH, -1),
            ] {
                let resp = srv.read_packet().expect("config reply");
                let mut c: &[u8] = &resp;
                assert_eq!(read_varint(&mut c).unwrap(), id, "config reply id");
                if tail >= 0 {
                    let n = if id == crate::config::serverbound::PONG { 4 } else { 8 };
                    assert_eq!(c.len(), n, "config reply size");
                } else {
                    assert!(c.is_empty(), "ack empty");
                }
            }
            // ---- play ----
            // login in the PROVEN vanilla 26.2 shape (playerId, hardcore,
            // levels, maxPlayers, chunkRadius, simulationDistance, 3 bools,
            // CommonPlayerSpawnInfo, onlineMode, enforcesSecureChat)
            let mut login_pkt = Vec::new();
            write_varint(&mut login_pkt, clientbound::LOGIN);
            login_pkt.extend_from_slice(&17i32.to_be_bytes()); // entity id
            login_pkt.push(0); // not hardcore
            write_varint(&mut login_pkt, 1); // one dimension
            write_string(&mut login_pkt, "minecraft:overworld");
            write_varint(&mut login_pkt, 20); // max players (dropped)
            write_varint(&mut login_pkt, 8); // chunk radius (view distance)
            write_varint(&mut login_pkt, 8); // simulation distance
            login_pkt.push(0); // reduced debug (dropped)
            login_pkt.push(1); // show death screen (dropped)
            login_pkt.push(0); // do limited crafting (dropped)
            // CommonPlayerSpawnInfo: dimension_type registry id 0 (the
            // mock's registry streams overworld first), dimension, seed,
            // game types, debug/flat, no death location, portal cooldown,
            // sea level
            write_varint(&mut login_pkt, 0); // dimension_type = overworld entry
            write_string(&mut login_pkt, "minecraft:overworld");
            login_pkt.extend_from_slice(&0i64.to_be_bytes()); // seed (dropped)
            login_pkt.push(1); // game type survival (dropped)
            login_pkt.push(1); // previous game type (dropped)
            login_pkt.push(0); // is debug (dropped)
            login_pkt.push(0); // is flat (dropped)
            login_pkt.push(0); // no death location (dropped)
            write_varint(&mut login_pkt, 300); // portal cooldown
            write_varint(&mut login_pkt, 63); // sea level (dropped)
            login_pkt.push(1); // online mode (dropped)
            login_pkt.push(1); // enforces secure chat (dropped)
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
            // chunk data (the stone+dirt section, plains underneath)
            let buffer = build_stone_dirt_section(0);
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
            // position teleport id 42 -> confirm teleportation (0x00) —
            // the javap-proven 26.2 shape: teleport id FIRST, then the
            // PositionMoveRotation (position + delta movement Vec3s +
            // yaw/pitch), then the int relativity flags
            let mut pos = Vec::new();
            write_varint(&mut pos, clientbound::SYNCHRONIZE_POSITION);
            write_varint(&mut pos, 42);
            pos.extend_from_slice(&8.5f64.to_be_bytes());
            pos.extend_from_slice(&64.0f64.to_be_bytes());
            pos.extend_from_slice(&(-3.25f64).to_be_bytes());
            pos.extend_from_slice(&0.0f64.to_be_bytes()); // delta movement
            pos.extend_from_slice(&0.0f64.to_be_bytes());
            pos.extend_from_slice(&0.0f64.to_be_bytes());
            pos.extend_from_slice(&90.0f32.to_be_bytes());
            pos.extend_from_slice(&0.0f32.to_be_bytes());
            pos.extend_from_slice(&0i32.to_be_bytes()); // all values absolute
            srv.send_packet(&pos).unwrap();
            let resp = srv.read_packet().expect("confirm teleportation");
            let mut c: &[u8] = &resp;
            assert_eq!(
                read_varint(&mut c).unwrap(),
                serverbound::CONFIRM_TELEPORTATION,
                "confirm id"
            );
            assert_eq!(read_varint(&mut c).unwrap(), 42, "teleport id verbatim");
            if !live_tail {
                return;
            }
            // ---- the LIVE leg (post-milestone traffic on the same wire) ----
            // a second chunk column (3,-1): the same stone+dirt section —
            // the stream keeps filling the view while the client plays
            let buffer2 = build_stone_dirt_section(0);
            let mut cd2 = Vec::new();
            write_varint(&mut cd2, clientbound::CHUNK_DATA);
            cd2.extend_from_slice(&3i32.to_be_bytes());
            cd2.extend_from_slice(&(-1i32).to_be_bytes());
            cd2.push(0x0A);
            cd2.push(0x00);
            write_varint(&mut cd2, buffer2.len() as i32);
            cd2.extend_from_slice(&buffer2);
            write_varint(&mut cd2, 0);
            cd2.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
            srv.send_packet(&cd2).unwrap();
            // a second teleport (id 43): the view re-anchors server-side —
            // pose (7.0, 65.0, -2.5), yaw 180, pitch 10 (looking down)
            let mut pos2 = Vec::new();
            write_varint(&mut pos2, clientbound::SYNCHRONIZE_POSITION);
            write_varint(&mut pos2, 43);
            pos2.extend_from_slice(&7.0f64.to_be_bytes());
            pos2.extend_from_slice(&65.0f64.to_be_bytes());
            pos2.extend_from_slice(&(-2.5f64).to_be_bytes());
            pos2.extend_from_slice(&0.0f64.to_be_bytes()); // delta movement
            pos2.extend_from_slice(&0.0f64.to_be_bytes());
            pos2.extend_from_slice(&0.0f64.to_be_bytes());
            pos2.extend_from_slice(&180.0f32.to_be_bytes());
            pos2.extend_from_slice(&10.0f32.to_be_bytes());
            pos2.extend_from_slice(&0i32.to_be_bytes());
            srv.send_packet(&pos2).unwrap();
            let resp = srv.read_packet().expect("confirm teleportation 43");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::CONFIRM_TELEPORTATION);
            assert_eq!(read_varint(&mut c).unwrap(), 43, "second teleport id verbatim");
            // a play keep-alive: the LIVE session must keep answering probes
            let mut kap2 = Vec::new();
            write_varint(&mut kap2, clientbound::KEEP_ALIVE);
            kap2.extend_from_slice(&99_999_999_999i64.to_be_bytes());
            srv.send_packet(&kap2).unwrap();
            let resp = srv.read_packet().expect("play keep alive echo (live)");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::KEEP_ALIVE);
            assert_eq!(
                i64::from_be_bytes(c[..8].try_into().unwrap()),
                99_999_999_999,
                "live keep-alive echoed"
            );
            // ---- the DIMENSION CHANGE leg: the vanilla crossing to the
            // nether — Respawn (the proven id 82 + CommonPlayerSpawnInfo),
            // then the new dimension's own chunk, then its spawn teleport
            let mut rp = Vec::new();
            write_varint(&mut rp, clientbound::RESPAWN);
            write_varint(&mut rp, 1); // dimension_type registry id (the nether entry)
            write_string(&mut rp, "minecraft:the_nether");
            rp.extend_from_slice(&0i64.to_be_bytes()); // seed
            rp.push(1);
            rp.push(1); // game types
            rp.push(0);
            rp.push(0); // debug/flat
            rp.push(0); // no death location
            write_varint(&mut rp, 60); // portal cooldown
            write_varint(&mut rp, 31); // sea level
            srv.send_packet(&rp).unwrap();
            let buffer3 = build_stone_dirt_section(1); // crimson_forest — the NETHER's own biome
            let mut cd3 = Vec::new();
            write_varint(&mut cd3, clientbound::CHUNK_DATA);
            cd3.extend_from_slice(&0i32.to_be_bytes());
            cd3.extend_from_slice(&0i32.to_be_bytes());
            cd3.push(0x0A);
            cd3.push(0x00); // empty heightmaps compound
            write_varint(&mut cd3, buffer3.len() as i32);
            cd3.extend_from_slice(&buffer3);
            write_varint(&mut cd3, 0); // no block entities
            cd3.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // light masks
            srv.send_packet(&cd3).unwrap();
            let mut pos3 = Vec::new();
            write_varint(&mut pos3, clientbound::SYNCHRONIZE_POSITION);
            write_varint(&mut pos3, 44);
            pos3.extend_from_slice(&8.5f64.to_be_bytes());
            pos3.extend_from_slice(&0.0f64.to_be_bytes()); // nether floor
            pos3.extend_from_slice(&(-3.25f64).to_be_bytes());
            pos3.extend_from_slice(&0.0f64.to_be_bytes()); // delta movement
            pos3.extend_from_slice(&0.0f64.to_be_bytes());
            pos3.extend_from_slice(&0.0f64.to_be_bytes());
            pos3.extend_from_slice(&90.0f32.to_be_bytes());
            pos3.extend_from_slice(&0.0f32.to_be_bytes());
            pos3.extend_from_slice(&0i32.to_be_bytes()); // all values absolute
            srv.send_packet(&pos3).unwrap();
            let resp = srv.read_packet().expect("confirm teleportation 44");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::CONFIRM_TELEPORTATION);
            assert_eq!(read_varint(&mut c).unwrap(), 44, "nether teleport id verbatim");
            // ---- the LIGHT UPDATE leg: the engine's live correction —
            // the standalone packet (VarInt coords + the SAME light
            // tail) for the nether's own column, its section going
            // full-bright
            let mut ul = Vec::new();
            write_varint(&mut ul, clientbound::UPDATE_LIGHT);
            write_varint(&mut ul, 0); // chunk x
            write_varint(&mut ul, 0); // chunk z
            ul.extend_from_slice(&test_support::light_tail(
                &[(1, vec![0xFFu8; 2048])],
                &[],
                &[],
                &[],
            ));
            srv.send_packet(&ul).unwrap();
            // ---- the SET TIME leg: the world's live clock — the
            // javap-proven 26.2 shape (LONG game time + the clock map),
            // the nether's own clock at total ticks 1_002_018_000 (day
            // tick 18_000, midnight — the timeline's night plateau)
            srv.send_packet(&test_support::set_time_packet(
                1_234_567_890,
                &[(0, 1_002_018_000, 0.0, 1.0)],
            ))
            .unwrap();
            // the server ends the session with a play disconnect: the live
            // caller surfaces the reason instead of an io error
            let mut d2 = Vec::new();
            write_varint(&mut d2, clientbound::DISCONNECT);
            write_string(&mut d2, r#"{"text":"Time"}"#);
            srv.send_packet(&d2).unwrap();
        });
        (port, handle)
    }

    #[test]
    fn the_login_skin_model_rides_the_session() {
        // the profile-metadata consumer end-to-end: a server whose login
        // success carries the `textures` property with SKIN.metadata.model
        // = "slim" lands that fact ON THE SESSION (the first-person arm's
        // slim/wide selection); the empty-property login (every offline
        // server) lands the classic None. The world milestone itself is
        // the plain mock's (its shapes are pinned by the sibling tests).
        let textures = {
            // the real property shape: base64 JSON, one SKIN entry with
            // the slim metadata (the value a real account carries)
            let json = r#"{"SKIN":{"url":"http://textures.example.net/skin.png","metadata":{"model":"slim"}}}"#;
            crate::login::test_base64_encode(json.as_bytes())
        };
        let (port, srv) = spawn_world_mock_with_props(vec![("textures".to_string(), textures)]);
        let joined =
            connect_and_join_live("127.0.0.1", port, 776, "mc2rs").expect("live join");
        match joined {
            LiveJoin::Live { snapshot, session } => {
                assert_eq!(snapshot.info.entity_id, 17, "the milestone is the mock's");
                assert_eq!(
                    session.skin_model.as_deref(),
                    Some("slim"),
                    "the profile metadata rides the session"
                );
            }
            LiveJoin::Disconnected { reason } => {
                panic!("expected a live join, got disconnected: {reason}")
            }
        }
        srv.join().unwrap();

        // the offline shape: NO properties -> the classic wide default
        let (port2, srv2) = spawn_world_mock();
        let joined2 =
            connect_and_join_live("127.0.0.1", port2, 776, "mc2rs").expect("live join 2");
        match joined2 {
            LiveJoin::Live { snapshot, session } => {
                assert_eq!(snapshot.info.entity_id, 17);
                assert_eq!(
                    session.skin_model, None,
                    "no profile metadata = the classic wide default"
                );
            }
            LiveJoin::Disconnected { reason } => {
                panic!("expected a live join, got disconnected: {reason}")
            }
        }
        srv2.join().unwrap();
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
                // the synced biome table: both dimensions' biomes in one
                // registry, the arrival order the chunk palettes reference
                assert_eq!(snap.registry.biome_name(0), Some("minecraft:plains"));
                assert_eq!(
                    snap.registry.biome_name(1),
                    Some("minecraft:crimson_forest")
                );
                // the decoded ELEMENT content rides too — the tint-relevant
                // color the client's mesher folds into the water quads
                assert_eq!(
                    snap.registry.biome(0).expect("plains").effects.water_color,
                    4_159_204,
                    "the vanilla plains water color, decoded through the wire codec"
                );
                assert_eq!(
                    snap.chunks[0].sections[0].biomes,
                    vec![0u32; 64],
                    "the overworld chunk's own biome, named through the table"
                );
            }
            other => panic!("expected Loaded, got {other:?}"),
        }
        srv.join().unwrap();
    }

    #[test]
    fn live_join_keeps_the_session_pumping_past_the_milestone() {
        // the milestone returns the snapshot AND the session; the server's
        // post-milestone traffic (second chunk, second teleport, probe,
        // disconnect) streams through the SAME wire — the live view's
        // input stream. The one-shot caller (connect_and_join_world) could
        // never serve this mock: its tail would sit unread.
        let (port, srv) = spawn_world_mock_live();
        let out = connect_and_join_live("127.0.0.1", port, 776, "mc2rs").expect("live join");
        match out {
            LiveJoin::Live { snapshot, mut session } => {
                // the milestone facts arrived intact
                assert_eq!(snapshot.info.entity_id, 17);
                assert_eq!(snapshot.player.teleport_id, 42);
                assert_eq!(snapshot.chunks.len(), 1);
                // the configuration's timeline knowledge rode the live
                // join: FOUR synced timelines + the in_overworld tag
                // (the day/night curve resolves DATA-DRIVEN through them)
                assert_eq!(session.registry.timelines.len(), 4);
                assert_eq!(
                    session.registry.timeline_tag_ids("minecraft:in_overworld"),
                    Some(&[0, 1, 2][..])
                );
                assert!(session.registry.timelines[0].sky_light_factor.is_some());
                // THE LIVE LEG: the retained session keeps speaking
                // 1) the second chunk column streams in
                match session.pump().expect("live chunk") {
                    PlayEvent::Chunk(col) => {
                        assert_eq!((col.x, col.z), (3, -1), "the post-milestone column");
                        assert_eq!(col.sections[0].blocks[291], 10, "the decoded dirt");
                    }
                    other => panic!("expected Chunk, got {other:?}"),
                }
                // 2) the second teleport arrives (the pump confirms it in
                // place — the server-side assert proves the reply)
                match session.pump().expect("live teleport") {
                    PlayEvent::Position(p) => {
                        assert_eq!(p.teleport_id, 43);
                        assert!((p.x - 7.0).abs() < 1e-9);
                        assert!((p.y - 65.0).abs() < 1e-9);
                        assert!((p.z + 2.5).abs() < 1e-9);
                        assert!((p.yaw - 180.0).abs() < 1e-6);
                        assert!((p.pitch - 10.0).abs() < 1e-6);
                    }
                    other => panic!("expected Position, got {other:?}"),
                }
                // 3) the probe is answered (server-side assert again)
                match session.pump().expect("live keep-alive") {
                    PlayEvent::KeepAlive(v) => assert_eq!(v, 99_999_999_999),
                    other => panic!("expected KeepAlive, got {other:?}"),
                }
                // 4) THE DIMENSION CHANGE: the Respawn rides the SAME live
                // wire — the spawn info carries the new dimension verbatim
                match session.pump().expect("live respawn") {
                    PlayEvent::Respawn(spawn) => {
                        assert_eq!(spawn.dimension, "minecraft:the_nether");
                        assert_eq!(spawn.dimension_type_id, 1);
                        assert_eq!(spawn.portal_cooldown, 60);
                    }
                    other => panic!("expected Respawn, got {other:?}"),
                }
                // 5) the nether's own chunk streams in (chunk (0,0))
                match session.pump().expect("nether chunk") {
                    PlayEvent::Chunk(col) => {
                        assert_eq!((col.x, col.z), (0, 0), "the new dimension's column");
                        assert_eq!(col.sections[0].blocks[291], 10, "the decoded dirt");
                        assert_eq!(
                            col.sections[0].biomes,
                            vec![1u32; 64],
                            "the crossing carries the NETHER's own biome (crimson_forest)"
                        );
                    }
                    other => panic!("expected Chunk, got {other:?}"),
                }
                // 6) the nether spawn teleport is confirmed in place
                match session.pump().expect("nether teleport") {
                    PlayEvent::Position(p) => {
                        assert_eq!(p.teleport_id, 44);
                        assert!((p.y - 0.0).abs() < 1e-9, "the nether floor pose");
                    }
                    other => panic!("expected Position, got {other:?}"),
                }
                // 7) THE LIGHT UPDATE: the standalone packet rides the
                // same live wire — VarInt coords + the shared tail
                match session.pump().expect("live light update") {
                    PlayEvent::Light { x, z, light } => {
                        assert_eq!((x, z), (0, 0));
                        assert_eq!(light.sky.len(), 2, "span from the masks (max bit 1)");
                        assert!(
                            matches!(&light.sky[1], Some(LightData::Data(a)) if a[0] == 0xFF),
                            "the nether column's section went full-bright"
                        );
                    }
                    other => panic!("expected Light, got {other:?}"),
                }
                // 8) THE SET TIME: the live clock rides the same wire —
                // game time + the clock map (the night plateau lands)
                match session.pump().expect("live set time") {
                    PlayEvent::Time(ts) => {
                        assert_eq!(ts.game_time, 1_234_567_890);
                        assert_eq!(ts.clocks.len(), 1, "one clock: the dimension's default");
                        assert_eq!(ts.clocks[0].id, 0);
                        assert_eq!(ts.clocks[0].total_ticks, 1_002_018_000);
                        assert_eq!(ts.clocks[0].rate, 1.0);
                    }
                    other => panic!("expected Time, got {other:?}"),
                }
                // 9) the server's play disconnect surfaces with its reason
                match session.pump().expect("live disconnect") {
                    PlayEvent::Disconnected { reason } => assert_eq!(reason, "Time"),
                    other => panic!("expected Disconnected, got {other:?}"),
                }
            }
            LiveJoin::Disconnected { reason } => panic!("expected Live, got kicked: {reason}"),
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
            write_varint(&mut fin, crate::config::clientbound::FINISH); // config finish
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

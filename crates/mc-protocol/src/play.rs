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
use crate::varint::{read_string, read_varint, read_varlong, write_varint};
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
        CB_LOGIN as LOGIN, CB_PING as PING, CB_PLAYER_POSITION as SYNCHRONIZE_POSITION,
        CB_REMOVE_ENTITIES as REMOVE_ENTITIES, CB_RESPAWN as RESPAWN,
        CB_SECTION_BLOCKS_UPDATE as SECTION_BLOCKS_UPDATE,
        CB_SET_CHUNK_CACHE_CENTER as CENTER_CHUNK, CB_SET_TIME as SET_TIME,
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

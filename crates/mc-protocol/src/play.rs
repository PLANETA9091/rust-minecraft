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
use crate::nbt::{read_bare_nbt, read_network_nbt, write_bare_nbt, Nbt};
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
        CB_LEVEL_EVENT as LEVEL_EVENT,
        CB_LOGIN as LOGIN, CB_MOVE_ENTITY_POS as MOVE_ENTITY_POS,
        CB_MOVE_ENTITY_POS_ROT as MOVE_ENTITY_POS_ROT, CB_MOVE_ENTITY_ROT as MOVE_ENTITY_ROT,
        CB_PING as PING, CB_PONG_RESPONSE as PONG_RESPONSE,
        CB_PLAYER_POSITION as SYNCHRONIZE_POSITION, CB_ROTATE_HEAD as ROTATE_HEAD,
        CB_REMOVE_ENTITIES as REMOVE_ENTITIES, CB_REMOVE_MOB_EFFECT as REMOVE_MOB_EFFECT,
        CB_RESPAWN as RESPAWN, CB_SECTION_BLOCKS_UPDATE as SECTION_BLOCKS_UPDATE,
        CB_SET_CHUNK_CACHE_CENTER as CENTER_CHUNK, CB_SET_ENTITY_DATA as SET_ENTITY_DATA,
        CB_SET_EQUIPMENT as SET_EQUIPMENT, CB_CONTAINER_SET_CONTENT as CONTAINER_SET_CONTENT,
        CB_CONTAINER_SET_SLOT as CONTAINER_SET_SLOT, CB_SET_HELD_SLOT as SET_HELD_SLOT,
        CB_OPEN_SCREEN as OPEN_SCREEN, CB_CONTAINER_CLOSE as CONTAINER_CLOSE,
        CB_SET_TIME as SET_TIME,
        CB_GAME_EVENT as GAME_EVENT,
        CB_PLAYER_ABILITIES as PLAYER_ABILITIES,
        CB_CHANGE_DIFFICULTY as CHANGE_DIFFICULTY,
        CB_UPDATE_MOB_EFFECT as UPDATE_MOB_EFFECT,
        CB_DEBUG_SAMPLE as DEBUG_SAMPLE,
        CB_SET_HEALTH as SET_HEALTH, CB_HURT_ANIMATION as HURT_ANIMATION,
        CB_PLAYER_COMBAT_KILL as PLAYER_COMBAT_KILL,
        CB_SOUND_ENTITY as SOUND_ENTITY, CB_SOUND as SOUND,
        CB_SET_OBJECTIVE as SET_OBJECTIVE, CB_SET_DISPLAY_OBJECTIVE as SET_DISPLAY_OBJECTIVE,
        CB_SET_PASSENGERS as SET_PASSENGERS,
        CB_SET_SCORE as SET_SCORE, CB_RESET_SCORE as RESET_SCORE,
        CB_PLAYER_INFO_REMOVE as PLAYER_INFO_REMOVE, CB_PLAYER_INFO_UPDATE as PLAYER_INFO_UPDATE,
        CB_TAB_LIST as TAB_LIST,
        CB_COMMAND_SUGGESTIONS as COMMAND_SUGGESTIONS,
        CB_COMMANDS as COMMANDS,
        CB_SYSTEM_CHAT as SYSTEM_CHAT, CB_PLAYER_CHAT as PLAYER_CHAT,
        CB_DISGUISED_CHAT as DISGUISED_CHAT,
    };
}

/// Serverbound play packet ids, GENERATED from the official jar's REAL
/// bootstrapped play protocol (see `clientbound` for the full story).
/// Runtime facts: accept teleportation = 0, chunk batch received = 11,
/// keep alive = 28, pong = 45.
pub mod serverbound {
    pub use crate::generated::packet_ids::{
        SB_ACCEPT_TELEPORTATION as CONFIRM_TELEPORTATION, SB_ATTACK as ATTACK,
        SB_CHUNK_BATCH_RECEIVED as CHUNK_BATCH_RECEIVED, SB_CONTAINER_CLICK as CONTAINER_CLICK,
        SB_CONTAINER_CLOSE as CONTAINER_CLOSE, SB_KEEP_ALIVE as KEEP_ALIVE,
        SB_PING_REQUEST as PING_REQUEST, SB_PONG as PONG,
        SB_MOVE_PLAYER_POS as MOVE_PLAYER_POS, SB_MOVE_PLAYER_POS_ROT as MOVE_PLAYER_POS_ROT,
        SB_MOVE_PLAYER_ROT as MOVE_PLAYER_ROT, SB_MOVE_PLAYER_STATUS_ONLY as MOVE_PLAYER_STATUS_ONLY,
        SB_DEBUG_SUBSCRIPTION_REQUEST as DEBUG_SUBSCRIPTION_REQUEST,
        SB_PLAYER_ACTION as PLAYER_ACTION, SB_SET_CARRIED_ITEM as SET_CARRIED_ITEM,
        SB_SWING as SWING, SB_USE_ITEM as USE_ITEM, SB_USE_ITEM_ON as USE_ITEM_ON,
        SB_CLIENT_COMMAND as CLIENT_COMMAND, SB_INTERACT as INTERACT,
        SB_PLAYER_INPUT as PLAYER_INPUT,
        SB_PADDLE_BOAT as PADDLE_BOAT,
        SB_PLAYER_ABILITIES as PLAYER_ABILITIES,
        SB_CHANGE_GAME_MODE as CHANGE_GAME_MODE,
        SB_PICK_ITEM_FROM_BLOCK as PICK_ITEM_FROM_BLOCK,
        SB_PICK_ITEM_FROM_ENTITY as PICK_ITEM_FROM_ENTITY,
        SB_BUNDLE_ITEM_SELECTED as BUNDLE_ITEM_SELECTED,
        SB_CHAT as CHAT, SB_CHAT_COMMAND as CHAT_COMMAND,
        SB_COMMAND_SUGGESTION as COMMAND_SUGGESTION,
    };
}

/// Fixed chunk-streaming pace requested in Chunk Batch Received. The
/// vanilla client adapts this from measured batch latencies; a fixed mid
/// value keeps real servers streaming without pretending to have a batch
/// timing model yet.
pub const DESIRED_CHUNKS_PER_TICK: f32 = 50.0;

/// THE GAME EVENT'S OWN WORD (javap `ClientboundGameEventPacket` — the
/// wire shape the FriendlyByteBuf read walk keeps: `readUnsignedByte`
/// event id, `readFloat` param; the $Type ids the outer clinit's
/// `new Type(i)` + `putstatic` pair names field by field). The client
/// answers the weather family through the rain/thunder levels the
/// ClientLevel state keeps; the rest the honest no-opinion skip (the
/// arms' vanilla walk names what each would do — the client owns no
/// such surface yet).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameEvent {
    /// The event's own id (the `$Type.id` the write walk sends).
    pub event: u8,
    /// The event's float param (the rain level, the game mode id, the
    /// border size — the event's own law reads it).
    pub param: f32,
}

/// The `$Type` ids — the 26.2 outer clinit's putstatic walk verbatim
/// (raw disasm `scripts/raw_disasm.py`, the NO_RESPAWN_BLOCK_AVAILABLE
/// .. LEVEL_CHUNKS_LOAD_START field order the ids 0..13 wear).
pub mod game_event {
    /// `NO_RESPAWN_BLOCK_AVAILABLE` — the clinit's `new Type(0)`.
    pub const NO_RESPAWN_BLOCK_AVAILABLE: u8 = 0;
    /// `START_RAINING` — the client answers `setRainLevel(1.0F)`.
    pub const START_RAINING: u8 = 1;
    /// `STOP_RAINING` — the client answers `setRainLevel(0.0F)`.
    pub const STOP_RAINING: u8 = 2;
    /// `CHANGE_GAME_MODEL` — `GameType.byId(floor(param + 0.5))`.
    pub const CHANGE_GAME_MODEL: u8 = 3;
    /// `WIN_GAME` — the WinScreen the vanilla client opens.
    pub const WIN_GAME: u8 = 4;
    /// `DEMO_EVENT` — the demo screens the options drive.
    pub const DEMO_EVENT: u8 = 5;
    /// `PLAY_ARROW_HIT_SOUND` — the arrow's hit word at the param pos.
    pub const PLAY_ARROW_HIT_SOUND: u8 = 6;
    /// `RAIN_LEVEL_CHANGE` — `setRainLevel(param)` the clamp walk keeps.
    pub const RAIN_LEVEL_CHANGE: u8 = 7;
    /// `THUNDER_LEVEL_CHANGE` — `setThunderLevel(param)`.
    pub const THUNDER_LEVEL_CHANGE: u8 = 8;
    /// `PUFFER_FISH_STING` — the mob-effect world the client mirrors.
    pub const PUFFER_FISH_STING: u8 = 9;
    /// `GUARDIAN_ELDER_EFFECT` — the MiningFatigue arm.
    pub const GUARDIAN_ELDER_EFFECT: u8 = 10;
    /// `IMMEDIATE_RESPAWN` — the client's respawn-screen flag.
    pub const IMMEDIATE_RESPAWN: u8 = 11;
    /// `LIMITED_CRAFTING` — the crafting gate the client flags.
    pub const LIMITED_CRAFTING: u8 = 12;
    /// `LEVEL_CHUNKS_LOAD_START` — the loading-screen word.
    pub const LEVEL_CHUNKS_LOAD_START: u8 = 13;
}

/// Parse Game Event (clientbound play, `minecraft:game_event` id 38):
/// u8 event id, f32 param — the FriendlyByteBuf walk the read ctor
/// keeps (`readUnsignedByte` for the TYPES.get lookup, `readFloat` for
/// the param).
pub fn parse_game_event(payload: &[u8]) -> Result<GameEvent> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::GAME_EVENT, "not a game event packet (0x{id:02X})");
    anyhow::ensure!(c.len() >= 5, "game event truncated");
    let event = read_u8(&mut c)?;
    let param = read_f32(&mut c)?;
    Ok(GameEvent { event, param })
}

/// THE PLAYER ABILITIES' OWN WORD (javap 26.2
/// `ClientboundPlayerAbilitiesPacket` — the read ctor's
/// `<init>(FriendlyByteBuf)` walk banks the shape verbatim): the ONE
/// byte's four flag bits (`iand 1` the invulnerable word, `iand 2` the
/// FLYING state, `iand 4` the mayfly grant, `bipush 8` the instabuild
/// grant — the putfield order invulnerable/isFlying/canFly/instabuild
/// the ctor keeps) plus the TWO speed floats the ctor reads
/// unconditionally (`readFloat` the flyingSpeed, `readFloat` the
/// walkingSpeed — no flag gates the walk, the nine-byte body the law
/// keeps). The MAYBUILD bit (16) the ctor never reads — the
/// server-side Abilities carries it, the client's packet the four
/// words alone speak (the ServerboundPlayerAbilitiesPacket's own ctor
/// the same ONE-byte shape keeps — see [`encode_player_abilities`]).
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerAbilitiesWord {
    /// The invulnerable bit (the ctor's `iand 1` arm).
    pub invulnerable: bool,
    /// THE FLYING STATE (the ctor's `iand 2` arm — the bit the
    /// outbound packet's own `ior 2` law writes).
    pub flying: bool,
    /// The mayfly grant (the ctor's `iand 4` arm — the
    /// `GameType.updatePlayerAbilities` CREATIVE/SPECTATOR word).
    pub mayfly: bool,
    /// The instabuild grant (the ctor's `iand 8` arm — the CREATIVE
    /// word alone).
    pub instabuild: bool,
    /// The flying speed (the ctor's first unconditional `readFloat`).
    pub flying_speed: f32,
    /// The walking speed (the ctor's second unconditional
    /// `readFloat` — the field the vanilla movement's own `getSpeed`
    /// fold never reads, the parity the bank keeps).
    pub walking_speed: f32,
}

/// Parse Player Abilities (clientbound play, `minecraft:player_abilities`
/// id 64): u8 flags, f32 flying speed, f32 walking speed — the
/// FriendlyByteBuf read ctor's walk (`readByte`, the four `iand` arms,
/// the two unconditionals).
pub fn parse_player_abilities(payload: &[u8]) -> Result<PlayerAbilitiesWord> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::PLAYER_ABILITIES,
        "not a player abilities packet (0x{id:02X})"
    );
    anyhow::ensure!(c.len() >= 9, "player abilities truncated");
    let flags = read_u8(&mut c)?;
    let invulnerable = flags & 1 != 0;
    let flying = flags & 2 != 0;
    let mayfly = flags & 4 != 0;
    let instabuild = flags & 8 != 0;
    let flying_speed = read_f32(&mut c)?;
    let walking_speed = read_f32(&mut c)?;
    Ok(PlayerAbilitiesWord {
        invulnerable,
        flying,
        mayfly,
        instabuild,
        flying_speed,
        walking_speed,
    })
}

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

/// Parse Pong Response (clientbound play 0x3E, `minecraft:pong_response`):
/// the ClientboundPongResponsePacket read ctor's own walk — `readLong()`
/// the FriendlyByteBuf read keeps (the sent-time long the client's
/// PingDebugMonitor subtracts from ITS Util.getMillis() — the RTT sample
/// the ping chart's columns speak).
pub fn parse_pong_response(payload: &[u8]) -> Result<i64> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::PONG_RESPONSE,
        "not a play pong response packet (0x{id:02X})"
    );
    anyhow::ensure!(c.len() >= 8, "play pong response truncated");
    Ok(i64::from_be_bytes(c[..8].try_into()?))
}

/// Ping Request (serverbound play 0x26, `minecraft:ping_request`): the
/// ServerboundPingRequestPacket's own walk — `writeLong(time)` the ONE
/// long the wire carries (the sent-time millis the server echoes
/// verbatim through the ClientboundPongResponsePacket; the difference
/// at the receive instant the RTT sample speaks).
pub fn encode_ping_request(time: i64) -> Vec<u8> {
    let mut p = Vec::with_capacity(9);
    write_varint(&mut p, serverbound::PING_REQUEST);
    p.extend_from_slice(&time.to_be_bytes());
    p
}

/// The remote sample's own type (the 26.2 RemoteDebugSampleType enum's
/// own vocabulary — ONE value the `$values` walk banks: TICK_TIME,
/// ordinal 0, the DebugSubscriptions.DEDICATED_SERVER_TICK_TIME
/// subscription word riding its ctor). The wire rides the ordinal the
/// `readEnum` decode keeps; an out-of-range ordinal the readEnum's own
/// out-of-bounds law refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteSampleType {
    TickTime,
}

/// Debug Sample (clientbound play 30, `minecraft:debug_sample`): the
/// ClientboundDebugSamplePacket record's own walk — `readLongArray()`
/// the sample (the FriendlyByteBuf read: VarInt length + length×i64
/// BE) and `readEnum(RemoteDebugSampleType.class)` the type (VarInt
/// ordinal). The dedicated server's RemoteSampleLogger.useSample
/// broadcasts ONE per tick to the subscribers the subscription request
/// named — the tick-time nanoseconds the TPS chart's columns speak
/// (the sample's four TpsDebugDimensions slots: FULL_TICK,
/// TICK_SERVER_METHOD, SCHEDULED_TASKS, IDLE).
#[derive(Debug, Clone, PartialEq)]
pub struct DebugSample {
    pub sample: Vec<i64>,
    pub sample_type: RemoteSampleType,
}

/// Parse Debug Sample (clientbound play 30, `minecraft:debug_sample`):
/// the ClientboundDebugSamplePacket read ctor's own walk — the
/// varint-framed id, `readLongArray()` the sample longs, `readEnum`
/// the type ordinal (0 the TICK_TIME word; anything else the enum's
/// own out-of-range refusal keeps).
pub fn parse_debug_sample(payload: &[u8]) -> Result<DebugSample> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::DEBUG_SAMPLE,
        "not a play debug sample packet (0x{id:02X})"
    );
    let count = read_varint(&mut c)?;
    anyhow::ensure!(count >= 0, "negative debug sample length {count}");
    anyhow::ensure!(
        c.len() >= count as usize * 8 + 1,
        "debug sample truncated ({count} longs, {} bytes)",
        c.len()
    );
    let mut sample = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let mut b = [0u8; 8];
        b.copy_from_slice(&c[..8]);
        c = &c[8..];
        sample.push(i64::from_be_bytes(b));
    }
    let ordinal = read_varint(&mut c)?;
    let sample_type = match ordinal {
        0 => RemoteSampleType::TickTime,
        _ => anyhow::bail!("unknown debug sample type ordinal {ordinal}"),
    };
    Ok(DebugSample { sample, sample_type })
}

/// Debug Subscription Request (serverbound play 23,
/// `minecraft:debug_subscription_request`): the
/// ServerboundDebugSubscriptionRequestPacket's own walk —
/// `ByteBufCodecs.collection(IntFunction, 32)` over
/// `ByteBufCodecs.registry(Registries.DEBUG_SUBSCRIPTION)` the SET
/// codec: VarInt count (the 32 cap the collection's own bound keeps)
/// + one VarInt registry id per subscription. The client's own
/// vocabulary ONE subscription speaks — DEDICATED_SERVER_TICK_TIME,
/// the FIRST registration the DebugSubscriptions clinit banks
/// (`registerSimple("dedicated_server_tick_time")` at clinit offset 2
/// — registry id 0). The F3+2 fps charts' raised lane the set arms,
/// the empty set the lowered flag's own word wears (the vanilla
/// ClientDebugSubscriber's `Set.of()` initial state the empty
/// equality keeps).
pub fn encode_debug_subscription_request(subscribe: bool) -> Vec<u8> {
    let mut p = Vec::with_capacity(4);
    write_varint(&mut p, serverbound::DEBUG_SUBSCRIPTION_REQUEST);
    if subscribe {
        // {DEDICATED_SERVER_TICK_TIME} — count 1, registry id 0
        write_varint(&mut p, 1);
        write_varint(&mut p, 0);
    } else {
        // Set.of() — the empty set the unsubscribe word speaks
        write_varint(&mut p, 0);
    }
    p
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

/// THE LEVEL EVENT'S OWN ID (the 26.2 `LevelEvent` constants class the
/// client jar banks — the growth effect the bone-meal item's own use
/// speaks): `BoneMealItem.useOn`'s walk (`javap`: `sipush 1505,
/// aload_3, bipush 15` → `Level.levelEvent`) answers ONE number — the
/// GROWTH_EFFECT word the client's own `LevelEventHandler.levelEvent`
/// case (offset 2523: `BoneMealItem.addGrowthParticles(level, pos,
/// data)` delegate) renders.
pub const GROWTH_EFFECT: i32 = 1505;

/// THE EYE-OF-ENDER SHATTER'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `PARTICLES_EYE_OF_ENDER_DEATH`, the
/// case `LevelEventHandler.levelEvent` offset 947 speaks): the server's
/// own eye-of-ender landing the broadcast — the `EyeOfEnder.tick`'s
/// landing tail `levelEvent(2003, blockPosition(), 0)` the jar's own
/// sipush scan pins (byte 7678 the one site the class carries, BOTH
/// landing branches the tail follows — the survive branch's
/// `addFreshEntity` drop and the death branch's
/// `playSound(ENDER_EYE_DEATH)` + `discard()` walk the shatter word
/// rides). The case's own body the TWO walks speak — the `bipush 8`
/// shard loop the `ItemParticleOption(ParticleTypes.ITEM,
/// Items.ENDER_EYE)` fires, the PORTAL ring the `(Math.PI * 2)`
/// half-circle walk answers — and the DATA word the case NEVER reads
/// (the zero the landing tail passes the walk's own body ignores — no
/// `iload_3` read the arm's bytecode speaks). The GLOBAL route the
/// `globalLevelEvent` switch never banks (the 1023/1028/1038 trio the
/// band alone carries — the positioned lane the case's own arm wears).
pub const EYE_OF_ENDER_SHATTER_EFFECT: i32 = 2003;

/// THE BLOCK-BREAK EFFECT'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `PARTICLES_DESTROY_BLOCK`, the case
/// `LevelEventHandler.levelEvent` offset 1561 speaks): the data word
/// the GLOBAL STATE id carries (`Block.stateById(data)` the walk's
/// own read — the break sound the `getSoundType` answer plays, the
/// `addDestroyBlockEffect` burst the state's shape speaks), the
/// server's own dig broadcasts the event on. The GLOBAL route the
/// `globalLevelEvent` switch never banks (the 1023/1028/1038 trio the
/// band alone carries — the positioned lane the case's own arm wears).
pub const DESTROY_BLOCK_EFFECT: i32 = 2001;

/// THE SHOOT-SMOKE EFFECT'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `PARTICLES_SHOOT_SMOKE`, the case
/// `LevelEventHandler.levelEvent` offset 933 speaks): the data word the
/// DIRECTION id carries (`Direction.from3DDataValue(data)` the walk's own
/// read — the 0..5 band the `DOWN/UP/NORTH/SOUTH/WEST/EAST` order keeps,
/// the `abs(irem(data, 6))` fold the out-of-range words ride), the ten
/// smoke puffs the `shootParticles` walk fires along the facing. The
/// server's own dispenser speaks it — `DefaultDispenseItemBehavior
/// .dispense`'s tail `playAnimation(source, FACING)` → `playDefault
/// Animation`'s `sipush 2000, get3DDataValue` → `ServerLevel.levelEvent`
/// walk the bytecode pins. The GLOBAL route the `globalLevelEvent` switch
/// never banks (the 1023/1028/1038 trio the band alone carries — the
/// positioned lane the case's own arm wears).
pub const SHOOT_SMOKE_EFFECT: i32 = 2000;

/// THE WHITE-SMOKE EFFECT'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `PARTICLES_SHOOT_WHITE_SMOKE`, the
/// case `LevelEventHandler.levelEvent` offset 919 speaks): the SAME
/// `shootParticles` walk the 2000 word wears (the ten-puff loop the
/// `Direction.from3DDataValue(data)` read the raw data carries), the
/// `ParticleTypes.WHITE_SMOKE` `SimpleParticleType` the ONE arg's
/// delta — the `WhiteSmokeParticle` ctor the base's own tint draw the
/// r/g/b putfield trio overrides (the constants the cp #12/#17/#21
/// decode). The server's own crafter the word broadcasts (`CrafterBlock`
/// the jar's own sipush-2010 scan pins — the craft's eject the
/// dispenser's grey puff the white twin wears). The GLOBAL route the
/// `globalLevelEvent` switch never banks (the 1023/1028/1038 trio the
/// band alone carries — the positioned lane the case's own arm wears).
pub const SHOOT_WHITE_SMOKE_EFFECT: i32 = 2010;

/// THE SPLASH-POTION EFFECT'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `PARTICLES_SPELL_POTION_SPLASH`, the
/// case `LevelEventHandler.levelEvent` offset 1199 speaks): the data
/// word the POTION COLOR carries — the RGB the handler's own decode
/// folds (`(data >> 16 & 255) / 255.0F` the red, `>> 8` the green,
/// `>> 0` the blue — the ARGB bytes the `SpellParticleOption.create`
/// walk re-packs). The case the 2007 word SHARES — the same arm both
/// ids wear, the sprite set the `Effect`/`InstantEffect` type alone
/// splits. The server's own splash-potion impacts speak it. The
/// GLOBAL route the `globalLevelEvent` switch never banks (the
/// 1023/1028/1038 trio the band alone carries — the positioned lane
/// the case's own arm wears).
pub const SPLASH_POTION_EFFECT: i32 = 2002;

/// THE INSTANT-SPLASH EFFECT'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `PARTICLES_INSTANT_POTION_SPLASH`, the
/// same case offset 1199 the 2002 word shares): the data word the
/// potion color carries exactly the 2002 walk's way, the `InstantEffect`
/// particle type the `if_icmpne` branch alone answers (the `spell_N`
/// sprite set the instant json lists — the `effect_N` set the regular
/// splash rides). The server's own instant-health/damage splash
/// impacts speak it. The GLOBAL route the `globalLevelEvent` switch
/// never banks.
pub const INSTANT_SPLASH_EFFECT: i32 = 2007;

/// THE MOB-SPAWN CLOUD'S OWN ID (the 26.2 `LevelEvent` constants class
/// the client jar banks — `PARTICLES_MOBBLOCK_SPAWN`, the case
/// `LevelEventHandler.levelEvent` offset 1689 speaks): the server's
/// own spawner the event broadcast — the `BaseSpawner` walk's
/// `levelEvent(2004, pos, 0)` the spawn tail speaks — the twenty
/// SMOKE + FLAME pairs the `bipush 20` loop fires, each pair AT THE
/// SAME full-block jitter spot the `(nextDouble() - 0.5) * 2.0` fold
/// places. The DATA word the case NEVER reads (the zero the spawner's
/// own tail passes the walk's own body ignores — no `iload_3` read
/// the arm's bytecode speaks). The GLOBAL route the `globalLevelEvent`
/// switch never banks (the 1023/1028/1038 trio the band alone
/// carries — the positioned lane the case's own arm wears).
pub const MOB_SPAWN_EFFECT: i32 = 2004;

/// THE BEE-GROWTH EFFECT'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `PARTICLES_BEE_GROWTH`, the case
/// `LevelEventHandler.levelEvent` offset 2546 speaks): the data word
/// the PARTICLE COUNT carries — the `ParticleUtils.spawnParticleInBlock
/// (level, pos, data, HAPPY_VILLAGER)` delegate the case's own body
/// speaks (the anchor cell's own state the shape read opens: the
/// `isAir` gate the 1.0 answer keeps, the block's own shape the
/// Y-max fold rides — then the `spawnParticles` walk the in-block
/// force lane arms). The server's own pollinator speaks it —
/// `Bee$BeeGrowCropGoal.tick`'s `sipush 2011, aload cropPos, bipush
/// 15` the levelEvent call the bytecode pins (the count the bipush 15
/// the pollen's own burst wears). No sound the case speaks — the
/// happy-villager burst the in-block lane's own stream alone carries.
/// The GLOBAL route the `globalLevelEvent` switch never banks (the
/// 1023/1028/1038 trio the band alone carries — the positioned lane
/// the case's own arm wears).
pub const BEE_GROWTH_EFFECT: i32 = 2011;

/// THE TURTLE-EGG PLACEMENT EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_TURTLE_EGG_PLACEMENT`, the case
/// `LevelEventHandler.levelEvent` offset 2561 speaks): the SAME
/// `spawnParticleInBlock` walk the 2011 word wears (the one case body
/// the two ids share — the `HAPPY_VILLAGER` type the both speak), the
/// data word the count carries. The server's own egg-lay the word
/// broadcasts — `TurtleEggBlock.onPlace`'s `sipush 2012, aload pos,
/// bipush 15` the levelEvent call the bytecode pins (the gate checks
/// the sand/site read the caller's own head keeps, the count the same
/// bipush 15 the lay's own burst wears). No sound the case speaks.
/// The GLOBAL route the `globalLevelEvent` switch never banks.
pub const TURTLE_EGG_PLACEMENT_EFFECT: i32 = 2012;

/// THE SMASH-ATTACK EFFECT'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `PARTICLES_SMASH_ATTACK`, the case
/// `LevelEventHandler.levelEvent` offset 2656 speaks): the data word
/// the PARTICLE COUNT carries — the `ParticleUtils.spawnSmashAttack
/// Particles(level, pos, data)` delegate the case's own body speaks
/// (the ONE `invokestatic` walk the 12-byte case keeps). The helper
/// the two loops the bytecode pins: the `vec3 = Vec3.atCenterOf(pos)
/// .add(0.0, 0.5, 0.0)` anchor fold (the block's own TOP FACE the +0.5
/// lift speaks — the y the never-drawn band keeps), the
/// `BlockParticleOption(ParticleTypes.DUST_PILLAR, level.getBlockState
/// (pos))` the anchor cell's own state carries, then the in-block
/// loop `(float)i < (float)count / 3.0F` (`nextDouble() / 2.0` the
/// x/z offsets, the `* 0.2` the dead speed trio) and the RING loop
/// `(float)i < (float)count / 1.5F` (`3.5 * Math.cos(i)` /
/// `3.5 * Math.sin(i)` the radius walk, the `* 0.05` the dead speed
/// trio). The particle the `TerrainParticle.DustPillarProvider` spawns
/// — the `createTerrainParticle` factory the AIR / MOVING_PISTON /
/// `shouldSpawnTerrainParticles() == false` states the null verdict
/// keeps, then the provider's own overrides: `setParticleSpeed
/// (nextDouble() / 30.0, ySpeed + nextDouble() / 2.0,
/// nextDouble() / 30.0)` (the caller's x/z speeds the override
/// discards, the y the `+ next/2` upward bias keeps) and
/// `setLifetime(20 + nextInt(20))` (the 20..39 band the base ctor's
/// own lifetime draw overrides). The server's own MACE the word
/// broadcasts — `MaceItem.knockback`'s head `sipush 2013,
/// target.getOnPos(), sipush 750` the levelEvent call the bytecode
/// pins (the count the `sipush 750` the smash's own burst wears, the
/// chain `MaceItem.hurtEnemy`'s `canSmashAttack` gate → `knockback`
/// offset 181 speaks), the 3.5-inflate knockback sweep the SAME
/// method's tail carries. No sound the case speaks. The GLOBAL route
/// the `globalLevelEvent` switch never banks (the 1023/1028/1038 trio
/// the band alone carries — the positioned lane the case's own arm
/// wears).
pub const SMASH_ATTACK_EFFECT: i32 = 2013;

/// THE DRAGON-FIREBALL SPLASH EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_DRAGON_FIREBALL_SPLASH`, the case
/// `LevelEventHandler.levelEvent` offset 3893 speaks): the data word
/// the SOUND GATE carries — the `if_icmpne 1` branch the tail speaks
/// (`data == 1` the `DRAGON_FIREBALL_EXPLODE` voice at the block
/// center arms, the `SoundSource.HOSTILE` the 1.0-volume walk wears,
/// the `nextFloat * 0.1F + 0.9F` pitch the draw gives; the silent
/// word `-1` the particles-only arm keeps). The burst itself the
/// `bipush 200` loop fires — each puff the `PowerParticleOption
/// .create(ParticleTypes.DRAGON_BREATH, f6)` the `f6 = nextFloat *
/// 4.0F` power arg carries, the `f7 = nextFloat * 6.2831855F` angle
/// the cos/sin folds ride (`d8 = cos(f7) * f6` the x-velocity, the
/// `d10 = 0.01 + nextDouble * 0.5` up-drift the y-velocity, the `d12
/// = sin(f7) * f6` the z-velocity — the spawn `(x + d8 * 0.1, y +
/// 0.3, z + d12 * 0.1)` the raw corner reads place), the provider's
/// `setPower(f6)` the velocity fold speaks (`xd *= p`, the `yd = (yd
/// - 0.1) * p + 0.1` the base's own 0.1 floor keeps). The server's
/// own fireball speaks it — `DragonFireball.onHit`'s tail `sipush
/// 2006, blockPosition(), isSilent() ? -1 : 1` the levelEvent call
/// the bytecode pins (the impact the dragon's breath pool the
/// AreaEffectCloud lands beside). The GLOBAL route the
/// `globalLevelEvent` switch never banks (the 1023/1028/1038 trio the
/// band alone carries — the positioned lane the case's own arm wears).
pub const DRAGON_FIREBALL_SPLASH_EFFECT: i32 = 2006;

/// THE WATER-EVAPORATING EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_WATER_EVAPORATING`, the case
/// `LevelEventHandler.levelEvent` offset 4070 speaks): the `bipush 8`
/// loop the eight CLOUD puffs fire — each puff the `addParticle
/// (CLOUD, x + nextDouble(), y + 1.2, z + nextDouble(), 0, 0, 0)`
/// walk the raw corner reads place (the x/z the nextDouble bands
/// carry, the +1.2 lift the cp #637 Double decode speaks, the zero
/// velocity trio the `dconst_0` answers). The server's own wet
/// sponge speaks it — `WetSpongeBlock.onPlace`'s ultra-warm gate the
/// nether's dry the word broadcasts (the FIRE_EXTINGUISH voice the
/// server's own playSound carries, NOT the case's — no sound the
/// case body speaks). The DATA word the case NEVER reads (no
/// `iload_3` read the arm's bytecode speaks). The GLOBAL route the
/// `globalLevelEvent` switch never banks (the 1023/1028/1038 trio
/// the band alone carries — the positioned lane the case's own arm
/// wears).
pub const WATER_EVAPORATING_EFFECT: i32 = 2009;

/// THE DRAGON BLOCK-BREAK EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_DRAGON_BLOCK_BREAKING`, the case
/// `LevelEventHandler.levelEvent` offset 3440 speaks): the ONE
/// `addParticle(EXPLOSION, x + 0.5, y + 0.5, z + 0.5, 0, 0, 0)` walk
/// the case body spends — the cp `#197` Double decode speaks the
/// 0.5 centers all three axes ride, the zero velocity trio the
/// `dconst_0` answers, the `HugeExplosionParticle` the provider
/// spawns (the ctor chain the four draws spend — the dead base
/// lifetime, the dead base quad size the ctor's own `2.0F * (1.0F -
/// (float)xd * 0.5F)` write overrides, the `6 + nextInt(4)` lifetime
/// the 6..9 band speaks, the ONE tint draw the `nextFloat * 0.6F +
/// 0.4F` grey the r/g/b trio keeps), the 16-frame ASCENDING
/// `explosion_0..15` sprite list the `explosion.json` banks — the
/// index the `MutableSpriteSet.get` walk reads IS the file index.
/// The server's own dragon speaks it — `EnderDragon.checkWalls`'s
/// tail `sipush 2008, blockPosition, iconst_0` the levelEvent call
/// the bytecode pins (the anchor the caller's own box-span scatter
/// draws — the `nextInt(max - min + 1)` walk the removed block's
/// cell re-arms, the client never re-draws it, the anchor the wire
/// carries verbatim). NO sound the case body speaks. The GLOBAL
/// route the `globalLevelEvent` switch never banks (the
/// 1023/1028/1038 trio the band alone carries — the positioned lane
/// the case's own arm wears).
pub const DRAGON_BLOCK_BREAK_EFFECT: i32 = 2008;

/// THE WAX-ON PARTICLES-AND-SOUND EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_AND_SOUND_WAX_ON`? the vanilla names the field
/// `SOUND_WAX_ON`-family constant the decode names — the case
/// `LevelEventHandler.levelEvent` offset 3366 speaks): the TWO walks
/// the case body spends — the ONE
/// `ParticleUtils.spawnParticlesOnBlockFaces(level, pos,
/// ParticleTypes.WAX_ON, UniformInt.of(3, 5))` walk FIRST (the
/// six-face loop `Direction.values()` answers — the DOWN/UP/NORTH/
/// SOUTH/WEST/EAST order the enum's own `values()` row keeps, the
/// per-face `3 + nextInt(3)` count the `randomBetweenInclusive` law
/// speaks, the per-particle position `center + axis * 0.55` /
/// `nextDouble(-0.5, 0.5)` pair the face-aligned law folds, the
/// velocity the `getRandomSpeedRanges` supplier's three
/// `nextDouble(-0.5, 0.5)` draws carry), then the ONE
/// `playLocalSound(pos, HONEYCOMB_WAX_ON, BLOCKS, 1.0F, 1.0F, false)`
/// walk the fixed fold speaks (the `item.honeycomb.wax_on` key the
/// jar's own SoundEvents class bytes carry — the 9-row weighted list
/// the sounds.json banks). The GlowParticle the
/// `GlowParticle$WaxOnProvider` spawns — the tint the ctor's own
/// `setColor(0.91F, 0.55F, 0.08F)` trio banks, the speed folds the
/// caller's velocity halves speak (`xSpeed * 0.01F / 2.0F`, the y the
/// plain `* 0.01F` keeps — NO face-axis gate the provider's own
/// multiply chain wears), the `nextInt(30) + 10` lifetime the 10..39
/// band speaks. The server's own honeycomb speaks it —
/// `HoneycombItem.lambda$useOn$0`'s `sipush 3003, aload_2, iconst_0`
/// levelEvent call the bytecode pins (the sign branch
/// `tryApplyToSign` the same word's own second caller keeps). The
/// GLOBAL route the `globalLevelEvent` switch never banks (the
/// 1023/1028/1038 trio the band alone carries — the positioned lane
/// the case's own arm wears).
pub const PARTICLES_AND_SOUND_WAX_ON_EFFECT: i32 = 3003;

/// THE WAX-OFF PARTICLES EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `PARTICLES_WAX_OFF`, the
/// case `LevelEventHandler.levelEvent` offset 3402 speaks): the ONE
/// `ParticleUtils.spawnParticlesOnBlockFaces(level, pos,
/// ParticleTypes.WAX_OFF, UniformInt.of(3, 5))` walk the case body
/// spends (the SAME six-face law the wax-on word wears — the counts,
/// the position pair, the velocity supplier the one helper keeps), NO
/// sound the case body speaks. The GlowParticle the
/// `GlowParticle$WaxOffProvider` spawns — the tint the ctor's own
/// `setColor(1.0F, 0.9F, 1.0F)` trio banks, the speed folds the
/// wax-on provider's own halves law speaks. The server's own axe
/// speaks it — `AxeItem.evaluateNewBlockState`'s `AXE_WAX_OFF` branch
/// `sipush 3004, ... spawnSoundAndParticle` the bytecode pins (the
/// scrape word the SAME method's own sibling branch banks). The
/// GLOBAL route the `globalLevelEvent` switch never banks — the
/// positioned lane the case's own arm wears.
pub const PARTICLES_WAX_OFF_EFFECT: i32 = 3004;

/// THE SCRAPE PARTICLES EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `PARTICLES_SCRAPE`, the
/// case `LevelEventHandler.levelEvent` offset 3421 speaks): the ONE
/// `ParticleUtils.spawnParticlesOnBlockFaces(level, pos,
/// ParticleTypes.SCRAPE, UniformInt.of(3, 5))` walk the case body
/// spends (the SAME six-face law the wax pair wears), NO sound the
/// case body speaks. The GlowParticle the
/// `GlowParticle$ScrapeProvider` spawns — the tint the ctor's own
/// `nextBoolean()` gate picks (the TRUE arm the
/// `setColor(0.29F, 0.58F, 0.51F)` teal speaks, the FALSE arm the
/// `setColor(0.43F, 0.77F, 0.62F)` lighter twin keeps — the bit the
/// `nextLong() & 1` law reads), the speed folds the PLAIN `* 0.01F`
/// trio the wax pair's `/ 2.0F` halves the one constant's own
/// difference carries. The server's own axe speaks it —
/// `AxeItem.evaluateNewBlockState`'s `AXE_SCRAPE` branch `sipush
/// 3005, ... spawnSoundAndParticle` the bytecode pins. The GLOBAL
/// route the `globalLevelEvent` switch never banks — the positioned
/// lane the case's own arm wears.
pub const PARTICLES_SCRAPE_EFFECT: i32 = 3005;

/// THE END-GATEWAY-SPAWN EXPLOSION-AND-VOICE EFFECT'S OWN ID (the 26.2
/// `LevelEvent` constants class the client jar banks —
/// `END_GATEWAY_SPAWN`, the case `LevelEventHandler.levelEvent` offset
/// 5083 speaks): the TWO walks the case body spends — the ONE
/// `addAlwaysVisibleParticle(ParticleTypes.EXPLOSION_EMITTER, true,
/// pos.x + 0.5, pos.y + 0.5, pos.z + 0.5, 0.0, 0.0, 0.0)` walk FIRST
/// (the `dconst_0` velocity trio the seed's own spawn keeps, the
/// `+0.5` block-center pair the cp `#197` Double fold speaks, the
/// force flag true the always-visible lane wears — the emitter the
/// distance cull never hides), then the ONE
/// `playLocalSound(pos, END_GATEWAY_SPAWN, BLOCKS, 10.0F, pitch,
/// false)` walk the randoms carry (the `ldc_w 10.0F` volume the cp
/// `#737` Float speaks, the pitch the `0.7F * (1.0F + (nextFloat() -
/// nextFloat()) * 0.2F)` chain the cps `#682`/`#360` bank — the
/// [0.56, 0.84] band the two-draw difference rides, the BLOCK CENTER
/// the `playLocalSound(BlockPos, ...)` resolve places, the
/// distance-delay flag false the immediate voice keeps, the data
/// word the case's own body never reads — no `iload_3` the arm's
/// bytecode speaks). The SEED the emitter option spawns —
/// `HugeExplosionSeedParticle` the `NoRenderParticle` subclass keeps
/// (the 8-tick countdown the ctor's `lifetime = 8` write speaks, the
/// per-tick SIX `ParticleTypes.EXPLOSION` children the tick's
/// `i < 6` loop banks — each child the `(r - r) * 4.0` triple-offset
/// draw pair carries, the child's own size input the seed's
/// `(float)age / (float)lifetime` walk feeds — the
/// `HugeExplosionParticle`'s `2.0F * (1.0F - xa * 0.5F)` quadSize law
/// the shrink rides), the 16-frame explosion bank the
/// `explosion.json`'s ascending list holds. The server's own dragon
/// fight speaks it — `EnderDragonFight.spawnNewGateway`'s
/// `ServerLevel.levelEvent(3000, pos, 0)` call the bytecode pins
/// (the dragon's death the new gateway the walk celebrates). The
/// GLOBAL route the `globalLevelEvent` switch never banks — the
/// positioned lane the case's own arm wears.
pub const END_GATEWAY_SPAWN_EFFECT: i32 = 3000;

/// THE ENDER-DRAGON GROWL VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `ENDER_DRAGON_GROWL`, the
/// case `LevelEventHandler.levelEvent` offset 5170 speaks): the ONE
/// `playLocalSound(pos, ENDER_DRAGON_GROWL, HOSTILE, 64.0F, pitch,
/// false)` walk the case body spends — the `ldc_w 64.0F` volume the
/// cp `#797` Float speaks (the 1024-block reach the armed max keeps
/// — the vanilla attenuation law `volume * 16` folds), the pitch the
/// `0.8F + nextFloat() * 0.3F` chain the cps `#138`/`#380` bank (the
/// ONE draw the `invokeinterface RandomSource.nextFloat` walk spends,
/// the [0.8, 1.1] band the fold gives), the HOSTILE source the
/// `getstatic #86` row keeps, the BLOCK CENTER the
/// `playLocalSound(BlockPos, ...)` resolve places, the distance-delay
/// flag false the immediate voice keeps, the data word the case's own
/// body never reads (no `iload_3` the arm's bytecode speaks — the
/// transport honesty the dispenser/gateway arms keep). NO particles
/// the case speaks — the voice alone the broadcast renders. NO
/// vanilla caller the 26.2 jar banks — `EnderDragonFight` carries no
/// `sipush 3001` walk (the dragon's own growl the CLIENT-SIDE
/// `aiStep` arm answers instead: `playLocalSound(DDD, ENDER_DRAGON_GROWL,
/// getSoundSource(), 2.5F, 0.8F + nextFloat() * 0.3F, false)` at the
/// dragon's own position every `nextInt(200)` ticks — the wire word
/// the plugins/older servers' broadcast the lane keeps). The GLOBAL
/// route the `globalLevelEvent` switch never banks — the positioned
/// lane the case's own arm wears.
pub const ENDER_DRAGON_GROWL_EFFECT: i32 = 3001;

/// THE ELECTRIC-SPARK PARTICLES EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `ELECTRIC_SPARK`, the case
/// `LevelEventHandler.levelEvent` offset 2596 speaks): the DATA word
/// the case's own body reads (`iload_3` the arm's bytecode opens with
/// — the axis table's own index) and the TWO branches the decode
/// splits: the `0..3` arm the `Direction.Axis.VALUES[data]` pick
/// banks — the ONE `ParticleUtils.spawnParticlesAlongAxis(axis,
/// level, pos, 0.125, ParticleTypes.ELECTRIC_SPARK, UniformInt.of(10,
/// 19))` walk the `ldc2_w 0.125` edge distance and the count provider
/// speak — while the `data < 0 || data >= 3` arm the ONE
/// `ParticleUtils.spawnParticlesOnBlockFaces(level, pos,
/// ParticleTypes.ELECTRIC_SPARK, UniformInt.of(3, 5))` walk the same
/// six-face law the wax trio wears. The sparks the
/// `GlowParticle$ElectricSparkProvider` spawns — the tint the ctor's
/// own `setColor(1.0F, 0.9F, 1.0F)` trio banks (the wax-off twin's
/// own white-pink the pair shares), the speed the provider's own
/// `setParticleSpeed(xa * 0.25, ya * 0.25, za * 0.25)` multiply chain
/// carries, the `nextInt(2) + 2` lifetime the 2..3 band speaks, the
/// friction 0.96 / quadSize x0.75 / hasPhysics false the
/// GlowParticle's own ctor keeps, the `minecraft:glow` sprite the
/// electric_spark.json's one-texture set banks (the wax trio's own
/// 8x8 glow.png the atlas pair shares). NO sound the case body
/// speaks — the sparks alone the broadcast renders. The server's own
/// lightning speaks it — `LightningBolt.randomStepCleaningCopper`'s
/// `levelEvent(3002, pos, -1)` call the bytecode pins (the copper
/// cleaning walk the data −1 word rides — the faces branch the
/// negative word arms; the axis branch the 0..2 words arm the case's
/// own bytecodes keep). The GLOBAL route the `globalLevelEvent`
/// switch never banks — the positioned lane the case's own arm
/// wears.
pub const ELECTRIC_SPARK_EFFECT: i32 = 3002;

/// THE SCULK-CHARGE PARTICLE EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `PARTICLES_SCULK_CHARGE`,
/// the case `LevelEventHandler.levelEvent` offset 2668 speaks): the
/// DATA word the case's own body SPLITS (`iload_3` the decode opens
/// with): the `data >> 6` count the sound gate and the face walk
/// ride — the count > 0 arm the probability-gated
/// `playLocalSound(pos, SCULK_BLOCK_CHARGE, BLOCKS, 0.15F + 0.02F *
/// count * count * nextFloat(), 0.4F + 0.3F * count * nextFloat(),
/// false)` walk spends (`block.sculk.charge` the SoundEvents clinit
/// #7045 bytes carry, the gate `nextFloat() < 0.3F + count * 0.1F`
/// the fcmpg/ifge pair keeps) plus the per-face sculk-charge burst
/// the `ParticleUtils.spawnParticlesOnBlockFace` delegates bank (the
/// `packed = data & 63` byte the zero arm's SIX
/// `Direction.values()` faces keeps — the DOWN face's π roll the
/// `SculkChargeParticleOptions` rides, the 0.65/0.57 axis spans —
/// the unpack arm the `MultifaceBlock.unpack` ordinal-order set
/// walks, the UP face's π roll, the 0.35 span) — while the
/// count <= 0 arm the ONE fixed
/// `playLocalSound(pos, SCULK_BLOCK_CHARGE, BLOCKS, 1.0F, 1.0F,
/// false)` voice + the sculk-charge-pop cube burst the
/// `isCollisionShapeFullBlock` split keeps (40 pops at ±0.45 over a
/// full block, 20 at ±0.25 otherwise, the `2.0F * nextFloat() -
/// 1.0F` trio the ±0.07 speeds ride). THE CALLER the spreader's own
/// decay pins — `SculkSpreader.updateCursors`' `charge <= 0` gate
/// the `sipush 3006, cursor.getPos(), iconst_0` levelEvent call the
/// bytecode speaks (the data 0 word the pop arm rides — the ONLY
/// vanilla caller the 26.2 jar banks; the count>0 face walk the
/// plugins/older servers' word the wire band keeps). The GLOBAL
/// route the `globalLevelEvent` switch never banks — the positioned
/// lane the case's own arm wears.
pub const PARTICLES_SCULK_CHARGE_EFFECT: i32 = 3006;

/// THE SCULK-SHRIEK PARTICLE EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `PARTICLES_SCULK_SHRIEK`,
/// the case `LevelEventHandler.levelEvent` offset 3193 speaks): the
/// DATA word the case's own body NEVER READS (no `iload_3` the arm's
/// bytecode speaks — the transport honesty the dispenser/gateway arms
/// keep). The body the TWO walks spends: the TEN
/// `addParticle(new ShriekParticleOption(i * 5), pos.x + 0.5, pos.y +
/// TOP_Y, pos.z + 0.5, 0, 0, 0)` walks the `i < 10` loop banks (the
/// delay 0, 5, .., 45 ticks the option carries, the shriek height the
/// `SculkShriekerBlock.TOP_Y` = 0.5 constant rides — the bottom-half
/// collision shape's own max the clinit answers) — then the
/// WATERLOGGED-GATED voice the `hasProperty(WATERLOGGED) &&
/// getValue(WATERLOGGED)` pair arms (the silent twin the drowned
/// shrieker keeps): the ONE `playLocalSound(pos.x + 0.5, pos.y +
/// TOP_Y, pos.z + 0.5, SCULK_SHRIEKER_SHRIEK, BLOCKS, 2.0F, 0.6F +
/// nextFloat() * 0.4F, false)` walk the DDD coords overload speaks
/// (`block.sculk_shrieker.shriek` key the sounds.json row banks — the
/// shriek1..3 files the table holds, the 2.0F volume the armed max
/// keeps). THE CALLER the shrieker's own reply pins —
/// `SculkShriekerBlockEntity.shriek`'s `sipush 3007, pos, iconst_0`
/// levelEvent call the bytecode speaks (the shrieking flag + the
/// 90-tick block tick + the SHRIEK game event the server's own walk
/// spends beside it). The GLOBAL route the `globalLevelEvent` switch
/// never banks — the positioned lane the case's own arm wears.
pub const PARTICLES_SCULK_SHRIEK_EFFECT: i32 = 3007;

/// THE BRUSH-COMPLETE PARTICLE-AND-SOUND EFFECT'S OWN ID (the 26.2
/// `LevelEvent` constants class the client jar banks —
/// `PARTICLES_AND_SOUND_BRUSH_BLOCK_COMPLETE`, the case
/// `LevelEventHandler.levelEvent` offset 1629 speaks): the DATA word
/// the case READS — the `iload_3` the `Block.stateById(data)` state
/// decode opens with (the brushable block's own state id the caller's
/// `Block.getId(state)` walk carries). The body the TWO walks spend:
/// the `instanceof BrushableBlock` verdict the ONE
/// `playLocalSound(pos, brushable.getBrushCompletedSound(), PLAYERS,
/// 1.0F, 1.0F, false)` voice arms (the `fconst_1` pair the volume/
/// pitch both carry — no randoms the fixed fold speaks, the PLAYERS
/// source the getstatic #336 row keeps, the distance-delay flag false
/// the immediate voice keeps; the completed sound the 26.2
/// unification banks — the Blocks lambdas' `new BrushableBlock`
/// getstatic pair the SAME ref reads twice, suspicious_sand →
/// `BRUSH_SAND`, suspicious_gravel → `BRUSH_GRAVEL`, the
/// `item.brush.brushing.sand`/`item.brush.brushing.gravel` keys the
/// sounds.json rows bank, the brushing1..4 files the table holds, NO
/// new assets the lane demands) — then the
/// `level.addDestroyBlockEffect(pos, blockstate)` burst the BOTH
/// branches reach (the non-brushable `ifeq` lands beside the
/// `aload_0` the burst's own head keeps — the 2001 sibling's own
/// walk; the AIR state the `addDestroyBlockEffect` method's own
/// `isAir` gate the no-op keeps, the `shouldSpawnTerrainParticles`
/// verdict the default-true law speaks — the BARRIER and
/// STRUCTURE_VOID exceptions the `noTerrainParticles` scan pins, the
/// two the terrain banks never render). The caller the brush's own
/// completion pins — `BrushableBlockEntity.brushingCompleted`'s
/// `sipush 3008, getBlockPos, Block.getId(state)` levelEvent call the
/// bytecode speaks (the drop content + the `turnsInto` setBlock the
/// server's own walk spends beside it). The GLOBAL route the
/// `globalLevelEvent` switch never banks — the positioned lane the
/// case's own arm wears.
pub const PARTICLES_AND_SOUND_BRUSH_BLOCK_COMPLETE_EFFECT: i32 = 3008;

/// THE EGG-CRACK PARTICLE EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `PARTICLES_EGG_CRACK`, the
/// case `LevelEventHandler.levelEvent` offset 2576 speaks): the DATA
/// word the case's own body NEVER reads (no `iload_3` the arm's
/// bytecode speaks, the same transport honesty the shrieker/growl
/// arms keep — the caller's `iconst_0` the wire carries the case
/// discards). The body the ONE walk spends: the
/// `ParticleUtils.spawnParticlesOnBlockFaces(level, pos,
/// ParticleTypes.EGG_CRACK, UniformInt.of(3, 6))` burst the six
/// `Direction.values()` faces answer (the per-face count the
/// `3 + nextInt(4)` band the IntProvider's sample speaks, the
/// supplier the `getRandomSpeedRanges` walk banks — the THREE
/// `Mth.nextDouble(-0.5, 0.5)` velocity draws the Vec3 ctor args
/// carry, the face-axis position the `center + step * 0.55`
/// deterministic arm rides, the perpendicular pair the
/// `Mth.nextDouble(-0.5, 0.5)` draws keep, the velocity the IN-PLANE
/// pair alone keeps the aligned axis's own `dconst_0` zeroing). NO
/// sound the case speaks — the crack burst alone the broadcast
/// renders. The particle the `SuspendedTownParticle$EggCrackProvider`
/// chain arms: the glint sprite pick the egg_crack.json's
/// one-texture set consumes (`minecraft:glint`, the SAME 8x8
/// particle/glint.png the happy-villager bank shares — NO new assets
/// the lane demands), the THREE dead draws the ctor spends (the tint
/// the `setColor(1.0F, 1.0F, 1.0F)` putfield trio overrides, the
/// base lifetime the ctor's own write keeps — the provider NO
/// setLifetime speaks), the white tint the override banks, the 0.99
/// friction the tick's own walk keeps, the no gravity the base law
/// never arms. THE CALLER the sniffer's own nest pins —
/// `SnifferEggBlock.onPlace`'s `sipush 3009, aload_3, iconst_0`
/// levelEvent call the bytecode speaks (the hatch-boost verdict the
/// `hatchBoost` tag walk arms — the moss bed the egg's own hatch the
/// boost rides; the thrown egg's own crack the ENTITY event the
/// `ThrownEgg.onHit`'s `broadcastEntityEvent(this, (byte) 3)` walk
/// carries, the level-event word the sniffer's own placement
/// broadcasts). The GLOBAL route the `globalLevelEvent` switch never
/// banks — the positioned lane the case's own arm wears.
pub const PARTICLES_EGG_CRACK_EFFECT: i32 = 3009;

/// THE TRIAL-SPAWNER SPAWN PARTICLE EFFECT'S OWN ID (the 26.2
/// `LevelEvent` constants class the client jar banks —
/// `PARTICLES_TRIAL_SPAWNER_SPAWN`, the case
/// `LevelEventHandler.levelEvent` offset 1827 speaks): the DATA word
/// the case's own body READS (the `iload_3` the
/// `TrialSpawner$FlameParticle.decode(I)` decode opens with — the
/// flame-flavor id the wire carries, NOT a block-state id the 2001
/// sibling resolves). The decode the jar's own bytecodes pin (pc
/// 0-21, len=22): `values()` first, then `id > values.length` →
/// NORMAL, `id >= 0` → `values[id]`, the fall-through the negative
/// words keep — so `0` decodes NORMAL (the `ParticleTypes.FLAME`
/// the clinit's first constant banks) and `1` decodes OMINOUS (the
/// `ParticleTypes.SOUL_FIRE_FLAME` the second banks — the 26.2 enum
/// the OMINOUS name speaks, NOT the 1.20 SHA twin the older banks
/// keep). The `id == values.length` edge the `>` guard NEVER banks —
/// the vanilla walk the `values[2]` ArrayIndexOutOfBounds crash
/// wears; the honest Rust arm the NORMAL answer the jar's own intent
/// keeps (the server never speaks a 2 — the `isOminous ? OMINOUS :
/// NORMAL` caller the spawnMob walk pins, the encode the ordinals
/// 0/1 alone give). The body the ONE walk spends: the
/// `TrialSpawner.addSpawnParticles(level, pos, random,
/// FlameParticle.decode(data).particleType)` static the case body's
/// own invokestatic speaks — the `bipush 20` loop verbatim (pc 0-121,
/// len=124): the THREE `RandomSource.nextDouble()` draws per
/// iteration the position triple carries (`pos.getX() + 0.5 +
/// (nextDouble() - 0.5) * 2.0` per axis — the ±1.0 spread the dsub
/// `0.5` and dmul `2.0` constants fold), the ZERO velocity the
/// `dconst_0` trio speaks, then TWO `Level.addParticle` calls share
/// the SAME position (the SMOKE companion the `getstatic
/// ParticleTypes.SMOKE` always fires + the decoded flame type the
/// `aload 4` arg passes). NO sound the case speaks — the particle
/// pair alone the broadcast renders (the SOUND twins the sibling
/// words bank: 3012 the TRIAL_SPAWNER_SPAWN_MOB voice
/// `playLocalSound(pos, ..., BLOCKS, 1.0F, (nf - nf) * ldc + 1.0F)`
/// rides, 3021 the SPAWN_ITEM twin — the pair the SAME decode +
/// addSpawnParticles walk beside the voice spends). The client
/// chains the providers' own walks bank: the SMOKE companion the
/// `SmokeParticle` ctor chain wears (the NINE draws the torch-smoke
/// bank pins — the dead base lifetime, the five-draw
/// spread+normalize pair, the size, the tint, the own lifetime, the
/// *0.1 velocity scale the ctor's xdScale args speak), the flame the
/// `FlameParticle$Provider` chain wears (the sprite pick
/// `SpriteSet.get(random)` the one-texture `flame.json` list
/// answers — the draw the pick always spends — then the
/// RisingParticle FIFTEEN-draw chain the torch-flame bank pins: the
/// dead base lifetime, the spread+normalize pair, the size, the SIX
/// jitter draws, the own lifetime, the *0.01 velocity fold the ctor
/// speaks), the OMINOUS twin the SAME `FlameParticle$Provider` chain
/// wears — the CORRECTIVE BYTE AUDIT the fresh `registerProviders`
/// dump banks (the round-454 handoff's SmallFlameProvider claim the
/// raw invokedynamic operands correct): BOTH `FLAME` (pc 581) and
/// `SOUL_FIRE_FLAME` (pc 653) ride the IDENTICAL invokedynamic call
/// site cp#125 (the dedup the identical lambda shapes speak — the
/// bootstrap arg cp#1085 the `FlameParticle$Provider.<init>`
/// methodref answers), the FULL quad the plain provider keeps (NO
/// scale the provider's own walk spends — the blue pixels the
/// soul_fire_flame sprite banks the ONLY delta the twin speaks; the
/// `scale(0.5F)` half the `FlameParticle$SmallFlameProvider` tail
/// carries belongs to the `SMALL_FLAME` type — indy cp#447, the
/// bootstrap arg cp#1349 the SmallFlameProvider methodref answers —
/// the candle/vault flame the OTHER registration rides, the eject
/// walk's own flame the 3014/3017 words bank). The caller
/// the spawner's own mob spawn pins — `TrialSpawner.spawnMob`'s
/// `sipush 3011, aload_2, FlameParticle.encode()` levelEvent call
/// the bytecode speaks (pc 473-483; the 3012 twin at pc 486-497 the
/// spawnPos anchor carries — the `isOminous ? OMINOUS : NORMAL`
/// flame the getstatic pair picks, the encode the ordinal's own
/// int gives). The GLOBAL route the `globalLevelEvent` switch never
/// banks — the positioned lane the case's own arm wears.
pub const PARTICLES_TRIAL_SPAWNER_SPAWN_EFFECT: i32 = 3011;

/// THE TRIAL-SPAWNER SPAWN-MOB VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_AND_SOUND_TRIAL_SPAWNER_SPAWN_MOB`, the case
/// `LevelEventHandler.levelEvent` offset 1847 speaks): the ONE
/// `playLocalSound(pos, TRIAL_SPAWNER_SPAWN_MOB, BLOCKS, 1.0F,
/// (nextFloat() - nextFloat()) * 0.2F + 1.0F, true)` walk the case
/// body spends FIRST (the `fconst_1` volume the fixed fold keeps, the
/// TWO pitch draws the fsub/fmul/fadd chain answers — the scale the
/// cp #360 Float 0.2F banks, the [0.8, 1.2] band the pair gives, the
/// BLOCK CENTER the `playLocalSound(BlockPos, ...)` resolve places,
/// the distance-delay flag TRUE the jar's own `iconst_1` keeps —
/// pc 1880, the `false` the round-453 handoff spoke the fresh byte
/// audit corrects), THEN the
/// SAME `TrialSpawner$FlameParticle.decode(data)` +
/// `TrialSpawner.addSpawnParticles` walk the 3011 sibling's own tail
/// spends (pc 1884-1898 — the identical iload_3 decode, the identical
/// `bipush 20` loop the SMOKE + flame pairs answer, the same draws
/// the burst alone consumes beside the voice's own two). The
/// `block.trial_spawner.spawn_mob` key the sounds.json row banks (the
/// spawn1..4 files the table holds, NO new assets the lane demands).
/// The caller the spawner's own mob spawn pins —
/// `TrialSpawner.spawnMob`'s `sipush 3012, aload 9, FlameParticle
/// .encode()` levelEvent call the bytecode speaks (pc 486-497 — the
/// spawnPos anchor the twin carries, the SAME `isOminous ? OMINOUS :
/// NORMAL` flame the getstatic pair picks; the 3011 particle word
/// rides the pos anchor beside it). The GLOBAL route the
/// `globalLevelEvent` switch never banks — the positioned lane the
/// case's own arm wears.
pub const PARTICLES_AND_SOUND_TRIAL_SPAWNER_SPAWN_MOB_EFFECT: i32 = 3012;

/// THE TRIAL-SPAWNER SPAWN-ITEM VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_AND_SOUND_TRIAL_SPAWNER_SPAWN_ITEM`, the case
/// `LevelEventHandler.levelEvent` offset 1904 speaks): the ONE
/// `playLocalSound(pos, TRIAL_SPAWNER_SPAWN_ITEM, BLOCKS, 1.0F,
/// (nextFloat() - nextFloat()) * 0.2F + 1.0F, true)` walk the case
/// body spends FIRST (pc 1904-1938 — the `fconst_1` volume the fixed
/// fold keeps, the TWO pitch draws the fsub/fmul/fadd chain answers,
/// the SAME [0.8, 1.2] band the 0.2F scale gives, the BLOCK CENTER
/// the `playLocalSound(BlockPos, ...)` resolve places, the
/// distance-delay flag TRUE the jar's own `iconst_1` keeps — pc 1937,
/// the same flag the 3012 twin's fresh byte audit pins), THEN the
/// SAME `TrialSpawner$FlameParticle.decode(data)` +
/// `TrialSpawner.addSpawnParticles` walk the 3011/3012 siblings' own
/// tail spends (pc 1941-1958 — the identical iload_3 decode the
/// `particleType` getfield answers, the identical `bipush 20` loop
/// the SMOKE + flame pairs speak — the same 540 draws the burst
/// alone consumes beside the voice's own two). The
/// `block.trial_spawner.spawn_item` key the sounds.json row banks
/// (the spawn_item1..3 files the runtime table holds, NO new assets
/// the lane demands). The caller the OMINOUS item spawner's own
/// eject pins — `OminousItemSpawner.spawnItem`'s
/// `sipush 3021, blockPosition(), iconst_1` levelEvent call the
/// bytecode speaks (pc 95-104 — the spawner ENTITY's own block the
/// anchor resolves, the data word 1 the `iconst_1` operand carries,
/// the OMINOUS flame flavor the decode arms). The GLOBAL route the
/// `globalLevelEvent` switch never banks — the positioned lane the
/// case's own arm wears.
pub const PARTICLES_AND_SOUND_TRIAL_SPAWNER_SPAWN_ITEM_EFFECT: i32 = 3021;

/// THE TRIAL-SPAWNER DETECT-PLAYER VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_TRIAL_SPAWNER_DETECT_PLAYER`, the case
/// `LevelEventHandler.levelEvent` offset 1961 speaks): the ONE
/// `playLocalSound(pos, TRIAL_SPAWNER_DETECTED_PLAYER, BLOCKS, 1.0F,
/// (nextFloat() - nextFloat()) * 0.2F + 1.0F, true)` walk the case
/// body spends FIRST (pc 1961-1995 — the fconst_1 volume the fixed
/// fold keeps, the TWO pitch draws the fsub/fmul/fadd chain answers,
/// the SAME [0.8, 1.2] band the 0.2F scale gives, the BLOCK CENTER
/// the `playLocalSound(BlockPos, ...)` resolve places, the
/// distance-delay flag TRUE the jar's own `iconst_1` keeps — pc 1994,
/// the same flag the 3012/3021 twins' fresh byte audits pin), THEN
/// the ONE `TrialSpawner.addDetectPlayerParticles(level, pos, random,
/// data, ParticleTypes.TRIAL_SPAWNER_DETECTED_PLAYER)` walk the case
/// tail spends (pc 1998-2012 — the `iload_3` the amount arg carries:
/// the DATA word the particle COUNT answers). THE WALK the jar's own
/// bytecodes pin (`addDetectPlayerParticles`, len=126): the loop
/// bound `30 + Math.min(amount, 10) * 5` the bipush 30/bipush 10/imul
/// trio speaks, the per-iteration THREE `nextFloat()` draws the
/// position pair + the height carry (`dx = (nf * 2.0F - 1.0F) *
/// 0.65D`, `dz` the same shape, `py = pos.getY() + 0.1 + nf * 0.8D`
/// — the ±0.65 spread the dmul 0.65 folds, the 0.1..0.9 band the
/// height fold keeps, the ZERO velocity the dconst_0 trio speaks —
/// the `addParticle(type, px, py, pz, 0, 0, 0)` call the walk's own
/// tail answers). THE SNOWFLAKE the `SnowflakeParticle` chain wears:
/// the provider `setColor(0.923F, 0.964F, 0.999F)` the near-white
/// bluish tint speaks (NO draw the three ldc1 constants spend), the
/// ctor's SIX draws the bytecodes pin — the THREE velocity spreads
/// `xd += (nf * 2.0F - 1.0F) * 0.05F` (pc 34-55 per axis, x→y→z the
/// order keeps), the size `0.1F * (nf * nf * 1.0F + 1.0F)` the TWO
/// draws' product speaks (pc 106-133 — the [0.1, 0.2] band the pair
/// gives, the `base_quad_size` law's own twin the different fold
/// arms: the PRODUCT the two draws speak, not the `r * 0.5 + 0.5`
/// mirror), the lifetime `(int)(16.0 / (nextFloat() * 0.8D + 0.2D))
/// + 2` the f2d-widened draw answers (pc 136-161 — the 18..81 band
/// the denom band keeps, the +2 the iconst tail adds), the sprite
/// the `setSpriteFromAge` walk picks (the snowflake.json list the
/// EIGHT generic_0..7 frames banks — the SAME descending list the
/// poof/smoke jsons speak, the SAME `get(age, lifetime)` frame law
/// the render lane carries, NO new assets the bank demands — the
/// generic rows the runtime cache already holds). The tick's own
/// damping the tail speaks: `xd *= 0.95, yd *= 0.9, zd *= 0.95` —
/// the gravity 0.225 the ctor's field keeps, the friction 1.0F the
/// base law's own no-op arms. THE CALLER the detector's own walk
/// pins — `TrialSpawnerStateData.tryDetectPlayers`'s
/// `levelEvent(isOminous() ? 3019 : 3013, pos, detectedPlayers.size())`
/// levelEvent call the bytecode speaks (pc 230-261 — the OMINOUS
/// twin the 3019 word answers, the amount the DETECTED PLAYER COUNT
/// the set's size carries, the 40-tick `nextMobSpawnsAt` gate the
/// sibling walk rides beside). The GLOBAL route the
/// `globalLevelEvent` switch never banks — the positioned lane the
/// case's own arm wears.
pub const PARTICLES_TRIAL_SPAWNER_DETECT_PLAYER_EFFECT: i32 = 3013;

/// THE TRIAL-SPAWNER DETECT-PLAYER VOICE'S OMINOUS TWIN'S OWN ID (the
/// 26.2 `LevelEvent` constants class the client jar banks —
/// `PARTICLES_TRIAL_SPAWNER_DETECT_PLAYER_OMINOUS`, the case
/// `LevelEventHandler.levelEvent` offset 2015 speaks): the ONE
/// `playLocalSound(pos, TRIAL_SPAWNER_DETECTED_PLAYER, BLOCKS, 1.0F,
/// (nextFloat() - nextFloat()) * 0.2F + 1.0F, true)` walk the case
/// body spends FIRST (pc 2015-2049 — the SAME voice event the 3013
/// sibling's getstatic speaks, the TWO pitch draws the same chain
/// answers, the distance-delay flag TRUE the jar's own `iconst_1`
/// keeps — pc 2048), THEN the SAME
/// `TrialSpawner.addDetectPlayerParticles(level, pos, random, data,
/// ParticleTypes.TRIAL_SPAWNER_DETECTED_PLAYER_OMINOUS)` walk the
/// 3013 sibling's tail spends (pc 2052-2066 — the identical iload_3
/// amount arg, the identical `30 + Math.min(amount, 10) * 5` loop
/// the bipush trio speaks, the same NINE draws the per-iteration
/// chain spends). THE SNOWFLAKE the SAME provider chain wears — the
/// `ParticleResources.registerProviders` walk the jar's own
/// bytecodes pin: BOTH `TRIAL_SPAWNER_DETECTED_PLAYER` and
/// `TRIAL_SPAWNER_DETECTED_PLAYER_OMINOUS` ride the IDENTICAL
/// invokedynamic call site (the raw operand bytes `02 13 00 00` the
/// pc 1417/1429 pair speaks — the one CallSite spec the lambda
/// metafactory answers), the `SnowflakeParticle$Provider.<init>`
/// methodref cp#1262 the bootstrap arg carries — the OMINOUS word
/// the SAME bluish-white snowflake bank answers (the 0.923/0.964/
/// 0.999 tint the provider's own constants keep, the generic_0..7
/// frames the same snowflake.json list holds, NO render delta the
/// twin's wire speaks). THE CALLER the detector's own branch pins —
/// `TrialSpawnerStateData.tryDetectPlayers`'s
/// `levelEvent(isOminous() ? 3019 : 3013, pos,
/// detectedPlayers.size())` levelEvent call the bytecode speaks
/// (pc 230-261 — the `isOminous()` verdict the sipush pair picks,
/// the SAME amount word the set's size carries). The GLOBAL route
/// the `globalLevelEvent` switch never banks — the positioned lane
/// the case's own arm wears.
pub const PARTICLES_TRIAL_SPAWNER_DETECT_PLAYER_OMINOUS_EFFECT: i32 = 3019;

/// THE TRIAL-SPAWNER OMINOUS-ACTIVATE VOICE'S OWN ID (the 26.2
/// `LevelEvent` constants class the client jar banks —
/// `PARTICLES_AND_SOUND_TRIAL_SPAWNER_OMINOUS_ACTIVATE`, the case
/// `LevelEventHandler.levelEvent` offset 2069 speaks): the ONE
/// `playLocalSound(pos, TRIAL_SPAWNER_OMINOUS_ACTIVATE, BLOCKS,
/// data != 0 ? 1.0F : 0.3F, (nextFloat() - nextFloat()) * 0.2F +
/// 1.0F, true)` walk the case body spends FIRST (pc 2069-2113 — the
/// `iload_3` + `ifne` pair the VOLUME GATE answers, the non-zero
/// data word the fconst_1 arms, the zero word the ldc2w 0.3F branch
/// keeps — the ONLY 3000-band case whose volume the wire arms; the
/// TWO pitch draws the same [0.8, 1.2] band the family's 0.2F scale
/// gives, the distance-delay flag TRUE the jar's own `iconst_1` pc
/// 2112 keeps), THEN the ONE `TrialSpawner.addDetectPlayerParticles(
/// level, pos, random, 0, TRIAL_SPAWNER_DETECTED_PLAYER_OMINOUS)`
/// walk the case's middle spends (pc 2116-2127 — the `iconst_0`
/// amount arg the case's OWN word carries, NOT the wire's data the
/// volume gate spends — the 30-iteration snowfall the min-free bound
/// answers, the SAME OMINOUS snowflake the 3019 case's provider call
/// site banks), THEN the ONE `TrialSpawner.addBecomeOminousParticles(
/// level, pos, random)` walk the case tail spends (pc 2130-2137 —
/// the amount-free signature the three-arg walk keeps). THE BECOME
/// WALK the jar's own bytecodes pin (`addBecomeOminousParticles`,
/// len=167): the `bipush 20` loop the iconst_0/if_icmpge pair
/// answers, the per-iteration SIX draws — the THREE `nextDouble()`
/// position spreads (`pos.getX() + 0.5 + (nextDouble() - 0.5) * 2.0`
/// per axis, the [−0.5, +1.5] band the 2.0F scale folds), the THREE
/// `nextGaussian() * 0.02` velocity draws the dmul chain carries —
/// and the TWO `Level.addParticle` calls the SAME position +
/// velocity args share (the dload quartet both calls re-read): the
/// TRIAL_OMEN puff the first getstatic pc 123 banks, the
/// SOUL_FIRE_FLAME the second pc 142 speaks (the omen-first order
/// the case's own call pair keeps). THE PUFF the plain
/// `SpellParticle$Provider` chain answers (the bootstrap arg
/// `SpellParticle$Provider.<init>` the indy cp#253 resolves — NO
/// setColor the pass-through createParticle spends, the WHITE
/// [1, 1, 1] tint the SingleQuadParticle defaults keep): the
/// super-arg pair the STATIC `SpellParticle.RANDOM` stream spends
/// (`0.5 - nextDouble()` twice), the yd `* 0.2` fold the ctor's own
/// write keeps, the quad ×0.75 fold, the `(int)(8.0 / (nextFloat() *
/// 0.8F + 0.2F))` lifetime, the zero-args arm the dcmp pair guards.
/// THE CALLERS the jar's own walks pin: `TrialSpawner.applyOminous`'s
/// `sipush 3020, aload_2, iconst_1` levelEvent call the OMINOUS
/// flip's own answer speaks (pc 25-31 — the data word 1 the VOLUME
/// 1.0 arms, the spawner's own pos the aload_2 carries);
/// `TrialSpawnerStateData.lambda$tryDetectPlayers$0`'s `sipush 3020,
/// getEyePosition.containing, iconst_0` the bad-omen transform's own
/// event speaks (pc 24-37 — the data word 0 the VOLUME 0.3 arms, the
/// PLAYER'S EYE pos the anchor carries — both events the sequence
/// fires, the second the applyOminous call beside it triggers). The
/// GLOBAL route the `globalLevelEvent` switch never banks — the
/// positioned lane the case's own arm wears.
pub const PARTICLES_AND_SOUND_TRIAL_SPAWNER_OMINOUS_ACTIVATE_EFFECT: i32 = 3020;

/// THE COBWEB-SPAWN WORD'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `ANIMATION_SPAWN_COBWEB`, the case
/// `LevelEventHandler.levelEvent` offset 2367 speaks): the ONE
/// `bipush 10` loop the case body spends FIRST (the `iconst_0` +
/// `istore 5` + `if_icmpge 10` trio the ten-iteration bound answers —
/// NOT the family's twenty the bipush 20 loop carries), the
/// per-iteration SIX level draws the walk's own order keeps — the
/// THREE `nextGaussian()` velocity draws FIRST (pc 2377-2414, the
/// `* 0.02` dmul trio the ldc2_w 0.02 answers, the `dstore 6/8/10`
/// trio the velocity args carry), THEN the THREE `nextDouble()`
/// position draws the `addParticle` arg walk interleaves (pc
/// 2423-2461 — `pos.getX() + nextDouble()` per axis the dadd trio
/// speaks, the `[0, 1)` corner band the bare nextDouble keeps — NO
/// +0.5 center the family's other walks carry, NO spread scale the
/// dmul never arms), the ONE `ClientLevel.addParticle(POOF, x, y, z,
/// vx, vy, vz)` call the SAME six args share (pc 2468 — the
/// `ParticleTypes.POOF` getstatic the provider chain answers). THE
/// PROVIDER the POOF bank's own chain wears (the `ExplosionParticle`
/// ctor the death-cloud round pins — the EIGHT draws per member: the
/// base lifetime float the super's law overrides, the THREE jitter
/// floats the `(nf * 2 - 1) * 0.05` fold adds to the walk's raw
/// gaussians, the grey float the `nf * 0.3 + 0.7` band speaks, the
/// size's TWO floats the product fold carries, the lifetime float
/// the `16.0 / (nf * 0.8 + 0.2) + 2` walk answers). THEN the ONE
/// `playLocalSound(pos, COBWEB_PLACE, BLOCKS, 1.0F, (nextFloat() -
/// nextFloat()) * 0.2F + 1.0F, true)` walk the case tail spends (pc
/// 2477-2514 — the fconst_1 volume the fixed fold keeps, the TWO
/// pitch draws the fsub/fmul/fadd chain answers, the SAME [0.8, 1.2]
/// band the 0.2F scale gives, the BLOCK CENTER the
/// `playLocalSound(BlockPos, ...)` resolve places, the distance-delay
/// flag TRUE the jar's own `iconst_1` pc 2510 keeps). The DATA word
/// the case body NEVER reads (no `iload_3` the whole arm speaks —
/// the `iconst_0` data the caller's own tail passes the case
/// discards). THE CALLER the weaving effect's own death walk pins —
/// `WeavingMobEffect.spawnCobwebsRandomlyAround`'s `sipush 3018,
/// aload 7, iconst_0` levelEvent call the bytecode speaks (pc
/// 182-189 — the `setBlock(pos, COBWEB.defaultBlockState(), 3)` walk
/// beside it fires, ONE event per PLACED cobweb the loop answers:
/// the `maxCobwebs` amplifier bound theToIntFunction arms, the
/// `BlockPos.randomInCube(random, 15, pos, 1)` candidates the
/// `canBeReplaced` + `isFaceSturdy(UP)` gate filters, the
/// `LivingEntity.getRandom()` stream the draws ride, the
/// `onMobRemoved` KILLED-reason + `MOB_GRIEFING` gamerule the entry
/// walk arms). The GLOBAL route the `globalLevelEvent` switch never
/// banks — the positioned lane the case's own arm wears.
pub const PARTICLES_AND_SOUND_SPAWN_COBWEB_EFFECT: i32 = 3018;

/// THE TRIAL-SPAWNER EJECT-ITEM VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_AND_SOUND_TRIAL_SPAWNER_EJECT_ITEM`, the case
/// `LevelEventHandler.levelEvent` offset 2143 speaks): the ONE
/// `playLocalSound(pos, TRIAL_SPAWNER_EJECT_ITEM, BLOCKS, 1.0F,
/// (nextFloat() - nextFloat()) * 0.2F + 1.0F, true)` walk the case
/// body spends FIRST (pc 2143-2177 — the fconst_1 volume the fixed
/// fold keeps, the TWO pitch draws the fsub/fmul/fadd chain answers,
/// the SAME [0.8, 1.2] band the 0.2F scale gives, the BLOCK CENTER
/// the `playLocalSound(BlockPos, ...)` resolve places, the
/// distance-delay flag TRUE the jar's own `iconst_1` keeps — pc
/// 2176), THEN the ONE `TrialSpawner.addEjectItemParticles(level,
/// pos, random)` walk the case tail spends (pc 2180-2187 — NO data
/// arg the walk's own signature carries, the `iload_3` word the
/// case body never READS). THE WALK the jar's own bytecodes pin
/// (`addEjectItemParticles`, len=159): the `bipush 20` loop the
/// iconst_0/if_icmpge pair answers, the per-iteration SIX draws —
/// the THREE `nextDouble()` position spreads (`pos.getX() + 0.4 +
/// nextDouble() * 0.2` per axis, the +0.4 anchor the ldc2w 0.4
/// carries, the [0.4, 0.6] band the dmul 0.2 folds), the THREE
/// `nextGaussian()` velocity draws (`* 0.02` per axis the dmul 0.02
/// trio speaks). The TWO `Level.addParticle` calls the SAME
/// position + velocity args share: the SMALL_FLAME the getstatic
/// pc 111 banks (the zd arg the `0.25 dmul` fold scales — the
/// flame's own quarter-pull the fold speaks), the SMOKE the pc 134
/// banks (the raw args the pair rides). The PROVIDER chain the
/// fresh audit banks: `SMALL_FLAME` the `FlameParticle
/// $SmallFlameProvider` chain wears (indy cp#447 — the SAME class
/// the plain provider arms with the ONE `scale(0.5F)` tail the ldc1
/// 0.5 + invokevirtual scale pair pc 34-36 speaks, the flame sprite
/// the one-texture `small_flame.json` list banks — `minecraft:flame`
/// the SAME pixels the plain flame holds, NO new assets the lane
/// demands; the ctor's FIFTEEN draws the torch-flame bank pins —
/// the arg velocities the `xd = xd * 0.01D + xdArg` fold ADDS, the
/// RisingParticle ctor pc 26-66 speaks), the SMOKE the
/// `SmokeParticle` ctor chain wears (the NINE draws the torch-smoke
/// bank pins, the arg velocities the BaseAshSmokeParticle
/// `arg + spread * 0.1` fold adds). THE CALLER the spawner's own
/// item eject pins — `TrialSpawner.ejectReward`'s
/// `sipush 3014, aload_2 (pos), iconst_0` levelEvent call the
/// bytecode speaks (pc 101-107 — the loot-table eject the
/// `DefaultDispenseItemBehavior.spawnItem` walk beside it fires,
/// the data word 0 the iconst_0 carries — the case body never
/// reads it; the SILENT sibling banks: 3017 the eject-particles
/// word the vault's own eject answers). The GLOBAL route the
/// `globalLevelEvent` switch never banks — the positioned lane the
/// case's own arm wears.
pub const PARTICLES_AND_SOUND_TRIAL_SPAWNER_EJECT_ITEM_EFFECT: i32 = 3014;

/// THE TRIAL-SPAWNER EJECT-ITEM PARTICLES' OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_TRIAL_SPAWNER_EJECT_ITEM`, the case
/// `LevelEventHandler.levelEvent` offset 2193 speaks): the TWELVE
/// BYTES the case body spends — the ONE `TrialSpawner
/// .addEjectItemParticles(level, pos, random)` invokestatic walk
/// alone (pc 2193-2200, the goto the tail answers), NO voice the
/// case speaks, NO data word the `iload_3` read the body ever makes
/// (the silent twin the 3014 voice word banks — the SAME walk the
/// eject burst answers, the 600 draws the twenty-iteration chain
/// consumes). THE CALLER the vault's own reward eject pins —
/// `VaultState.ejectResultItem`'s `sipush 3017, aload_1 (pos),
/// iconst_0` levelEvent call the bytecode speaks (pc 23-28 — the
/// `DefaultDispenseItemBehavior.spawnItem` walk beside it fires,
/// the data word 0 the iconst_0 carries; the vault's own eject
/// VOICE the `playSound(null, pos, VAULT_EJECT_ITEM, BLOCKS, 0.8F,
/// 0.4F + f3)` DIRECT call the same body spends — pc 34-48 — the
/// sound-packet lane the level-event lane never carries, NO voice
/// the 3017 word itself speaks). The 3013/3019 detect pair's own
/// silent-vs-voice law the eject family mirrors (the one-invoke
/// arm the 3017 case's own size keeps). The GLOBAL route the
/// `globalLevelEvent` switch never banks — the positioned lane the
/// case's own arm wears.
pub const PARTICLES_TRIAL_SPAWNER_EJECT_ITEM_EFFECT: i32 = 3017;

/// THE VAULT ACTIVATE VOICE'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `PARTICLES_AND_SOUND_VAULT_ACTIVATE`,
/// the case `LevelEventHandler.levelEvent` offset 2206 speaks): the
/// BLOCK-ENTITY GATE the case body spends FIRST — the ONE
/// `level.getBlockEntity(pos) instanceof VaultBlockEntity` walk the
/// `ifeq -> 2303` tail answers (the non-vault anchor the whole arm
/// skips — NO burst the failed gate ever fires, NO voice it speaks),
/// THEN the ONE `VaultBlockEntity$Client.emitActivationParticles(
/// level, vault.getBlockPos(), vault.getBlockState(),
/// vault.getSharedData(), data != 0 ? SOUL_FIRE_FLAME : SMALL_FLAME)`
/// walk the gate's own body spends (pc 2231-2263 — the `iload_3` +
/// `ifne` pair the flame gate answers, the SOUL word the ominous
/// data carries, the SMALL word the normal data keeps; the BLOCK
/// ENTITY'S OWN pos the `getBlockPos` resolve places — NOT the event
/// pos the `aload_2` banks), THEN the ONE `playLocalSound(pos,
/// VAULT_ACTIVATE, BLOCKS, 1.0F, (nextFloat() - nextFloat()) * 0.2F
/// + 1.0F, true)` walk the case tail spends (pc 2266-2300 — the TWO
/// pitch draws the fsub/fmul/fadd chain answers, the SAME [0.8, 1.2]
/// band the 0.2F scale gives, the BLOCK CENTER the
/// `playLocalSound(BlockPos, ...)` resolve places, the distance-delay
/// flag TRUE the `iconst_1` pc 2299 keeps). THE WALK the jar's own
/// bytecodes pin (`emitActivationParticles`, len=87): the connection
/// tail FIRST — `emitConnectionParticlesForNearbyPlayers(level, pos,
/// state, sharedData)` the pc 4 invokestatic spends (the
/// `getConnectedPlayers()` set the `isEmpty()` early-return answers —
/// the client's own BE sync the set carries, the EMPTY set the
/// no-op's own return speaks), THEN the `bipush 20` loop the
/// iconst_0/if_icmpge pair spends — the THREE `Mth.nextDouble(random,
/// ...)` position draws the `randomPosInsideCage(pos, random)` walk
/// carries per iteration (the [0.1, 0.9] x/z bands + the [0.25,
/// 0.75] y band the ldc2w quartet speaks — the `min + nextDouble() *
/// (max - min)` fold the Mth's own walk answers), the TWO
/// `Level.addParticle` calls the SAME position + the ZERO velocity
/// args share (the `dconst_0` trio both calls speak) — the SMOKE the
/// first getstatic pc 32 banks, the flame ARG the second `aload [4]`
/// carries (the gate's own pick). The chains the providers' own
/// walks bank: the SMOKE the `SmokeParticle` ctor chain (the NINE
/// draws, the `arg + spread * 0.1` fold the zero args keep), the
/// SMALL_FLAME the `FlameParticle$SmallFlameProvider` chain (the
/// FIFTEEN draws + the `scale(0.5F)` half-quad, the `xd * 0.01D +
/// xdArg` fold the zero args keep), the SOUL_FIRE_FLAME the plain
/// `FlameParticle$Provider` chain (the FULL quad the identical indy
/// call site cp#125 banks — the corrective audit the eject round
/// pinned). THE CALLER the vault's own activation pins —
/// `VaultState$2.onEnter` (the ACTIVE state's own onEnter)'s
/// `sipush 3015, pos, z ? 1 : 0` levelEvent call the bytecode pins
/// (pc 17-32 — the display-item cycle the
/// `cycleDisplayItemFromLootTable` walk beside it fires, the data
/// word the ominous gate's own ternary passes — the boolean arg the
/// onEnter signature carries the `iload [5]` + `ifeq` pair answers).
/// The GLOBAL route the `globalLevelEvent` switch never banks — the
/// positioned lane the case's own arm wears.
pub const PARTICLES_AND_SOUND_VAULT_ACTIVATE_EFFECT: i32 = 3015;

/// THE VAULT DEACTIVATE VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_AND_SOUND_VAULT_DEACTIVATE`, the case
/// `LevelEventHandler.levelEvent` offset 2306 speaks): NO block-entity
/// gate the case body spends — the ONE
/// `VaultBlockEntity$Client.emitDeactivationParticles(level, pos,
/// data != 0 ? SOUL_FIRE_FLAME : SMALL_FLAME)` walk the body spends
/// FIRST (pc 2306-2324 — the `iload_3` + `ifne` pair the SAME flame
/// gate the 3015 sibling keeps, the EVENT pos the `aload_2` carries —
/// no `getBlockEntity` walk the twelve-instruction head answers),
/// THEN the ONE `playLocalSound(pos, VAULT_DEACTIVATE, BLOCKS, 1.0F,
/// (nextFloat() - nextFloat()) * 0.2F + 1.0F, true)` walk the case
/// tail spends (pc 2327-2361 — the SAME pitch chain the 3015 voice
/// keeps, the SAME [0.8, 1.2] band the 0.2F scale gives, the
/// distance-delay flag TRUE the `iconst_1` pc 2360 keeps). THE WALK
/// the jar's own bytecodes pin (`emitDeactivationParticles`, len=103):
/// the `bipush 20` loop the iconst_0/if_icmpge pair spends — the
/// THREE `Mth.nextDouble(random, 0.4, 0.6)` position draws the
/// `randomPosCenterOfCage(pos, random)` walk carries per iteration
/// (the [0.4, 0.6] band the ldc2w trio speaks — the cage CENTER the
/// tight band keeps), the THREE `nextGaussian() * 0.02` velocity
/// draws the `new Vec3(...)` ctor carries (the `dmul 0.02` trio the
/// eject walk's own shape answers), the ONE `Level.addParticle`
/// call the flame ARG spends (NO smoke companion the activation's
/// own pair keeps). The chain the providers' own walks bank: the
/// SMALL_FLAME the `FlameParticle$SmallFlameProvider` chain (the
/// FIFTEEN draws + the `scale(0.5F)` half-quad, the `xd * 0.01D +
/// xdArg` fold the gaussian args ride at FULL scale), the
/// SOUL_FIRE_FLAME the plain `FlameParticle$Provider` chain (the
/// FULL quad the identical indy call site banks). THE CALLER the
/// vault's own deactivation pins — `VaultState$1.onEnter` (the
/// INACTIVE state's own onEnter)'s `sipush 3016, pos, z ? 1 : 0`
/// levelEvent call the bytecode pins (pc 8-23 — the EMPTY display
/// item the `setDisplayItem` walk beside it resets, the data word
/// the SAME ominous ternary the 3015 caller passes). The GLOBAL
/// route the `globalLevelEvent` switch never banks — the positioned
/// lane the case's own arm wears.
pub const PARTICLES_AND_SOUND_VAULT_DEACTIVATE_EFFECT: i32 = 3016;

/// THE DISPENSER-DISPENSE VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `SOUND_DISPENSER_DISPENSE`,
/// the case `LevelEventHandler.levelEvent` offset 776 speaks): the
/// ONE `playLocalSound(pos, DISPENSER_DISPENSE, BLOCKS, 1.0F, 1.0F,
/// false)` walk the case body spends — the `fconst_1` pair the
/// volume/pitch both carry (no randoms the fixed fold speaks), the
/// BLOCK CENTER the `playLocalSound(BlockPos, ...)` resolve places,
/// the distance-delay flag false the immediate voice keeps, the data
/// word the case's own body never reads (no `iload_3` the arm's
/// bytecode speaks). NO particles the case speaks — the voice alone
/// the broadcast renders. The server's own dispenser speaks it —
/// `DefaultDispenseItemBehavior.playDefaultSound`'s `sipush 1000,
/// BlockSource.pos, iconst_0` the levelEvent call the bytecode pins
/// (the default behavior's own eject the sound the walk's own tail
/// broadcasts; the empty-slot `sipush 1001` FAIL word the sibling
/// case answers). The GLOBAL route the `globalLevelEvent` switch
/// never banks (the 1023/1028/1038 trio the band alone carries —
/// the positioned lane the case's own arm wears).
/// THE COMPOSTER-FILL VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `COMPOSTER_FILL`, the case
/// `LevelEventHandler.levelEvent` offset 3483 speaks): the ONE arm
/// the `ComposterBlock.handleFill(level, pos, data != 0)` walk the
/// case body spends — the FIRST 1500-band word whose DATA the case
/// READS (the `iload_3` + `ifle` the success fold keeps — the bit
/// the wire carries, the `data != 0` answer the boolean rides). The
/// server's own composter speaks it — `ComposterBlock.useItemOn`'s
/// tail the `sipush 1500, pos, blockstate1 != blockstate ? 1 : 0`
/// levelEvent call the bytecode pins (the state-changed ternary the
/// success bit carries — the insert's accept the 1 speaks, the
/// chance-fail the 0). The handleFill walk the jar's own dump pins:
/// the ONE `playLocalSound(pos, success ? COMPOSTER_FILL_SUCCESS :
/// COMPOSTER_FILL, BLOCKS, 1.0F, 1.0F, false)` walk FIRST (the
/// `fconst_1` pair the volume/pitch both carry, no randoms the fixed
/// fold speaks), then the `bipush 10` loop the ten COMPOSTER puffs
/// fire — the THREE `nextGaussian() * 0.02` velocity draws the
/// addParticle args carry first, the position floats next (the `x +
/// 0.1875 + 0.625 * nextFloat()` the cp `#469` Double 0.1875 × the
/// `#471` Double 0.625 folds keep, the `y + d0 + nextFloat() * (1.0
/// - d0)` the shape fold answers — `d0 = shape.max(Axis.Y, 0.5,
/// 0.5) + 0.03125` the cp `#465` Double speaks: the 26.2 shape
/// lambda `cube − column(12, clamp(1 + level*2, 2, 16), 16)` the
/// center-xz audit folds — the rim top 1.0 every live level keeps,
/// the +0.03125 the constant rides, the level-8 empty slice the
/// insertion law never arms). The COMPOSTER particle the
/// `SuspendedTownParticle$ComposterFillProvider` spawns (the ctor
/// chain the seven draws spend — the one-frame glint pick, the three
/// dead tint draws the setColor(1,1,1) override keeps, the size, the
/// dead base lifetime, the `3 + nextInt(5)` lifetime the 3..7 band
/// gives). The GLOBAL route the `globalLevelEvent` switch never
/// banks (the 1023/1028/1038 trio the band alone carries — the
/// positioned lane the case's own arm wears).
pub const COMPOSTER_FILL_EFFECT: i32 = 1500;

/// THE LAVA-FIZZ VOICE'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `LAVA_FIZZ`, the case
/// `LevelEventHandler.levelEvent` offset 3522 speaks): the pitched
/// fizz voice the case body spends FIRST — the ONE
/// `playLocalSound(pos, LAVA_EXTINGUISH, BLOCKS, 0.5F, 2.6F +
/// (random.nextFloat() - random.nextFloat()) * 0.8F, false)` walk
/// the cp decodes pin (the volume `#177` Float 0.5 the fixed fold
/// keeps, the base `#633` Float 2.6 the pitch constant rides, the
/// scale `#138` Float 0.8 the TWO nextFloat draws carry — the FIRST
/// minus the SECOND the fsub order speaks, the BLOCK CENTER the
/// `playLocalSound(BlockPos, ...)` resolve places, the distance-delay
/// flag false the immediate voice keeps) — then the `bipush 8` loop
/// the eight LARGE_SMOKE puffs fire (`addParticle(LARGE_SMOKE,
/// x + nextDouble(), y + 1.2, z + nextDouble(), 0, 0, 0)` — the x
/// draw FIRST, the z draw SECOND, the +1.2 lift the cp `#637` Double
/// decode speaks, the zero velocity trio the `dconst_0` answers).
/// The server's own lava speaks it — `LavaFluid.fizz`'s tail the
/// `sipush 1501, pos, iconst_0` levelEvent call the bytecode pins
/// (the water-meets-lava walk's own broadcast; the data word the
/// case's own body never reads — no `iload_3` the arm's bytecode
/// speaks). The GLOBAL route the `globalLevelEvent` switch never
/// banks (the 1023/1028/1038 trio the band alone carries — the
/// positioned lane the case's own arm wears).
pub const LAVA_FIZZ_EFFECT: i32 = 1501;

/// THE REDSTONE-TORCH BURNOUT VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `REDSTONE_TORCH_BURNOUT`, the case
/// `LevelEventHandler.levelEvent` offset 3628 speaks): the pitched
/// burnout voice the case body spends FIRST — the ONE
/// `playLocalSound(pos, REDSTONE_TORCH_BURNOUT, BLOCKS, 0.5F, 2.6F +
/// (random.nextFloat() - random.nextFloat()) * 0.8F, false)` walk
/// the SAME cp trio the fizz voice keeps (the volume `#177` Float
/// 0.5, the base `#633` Float 2.6, the scale `#138` Float 0.8 the
/// TWO nextFloat draws carry — the first minus the second the fsub
/// order speaks) — then the `iconst_5` loop the five SMOKE puffs
/// fire (`addParticle(SMOKE, x + nextDouble() * 0.6 + 0.2, y +
/// nextDouble() * 0.6 + 0.2, z + nextDouble() * 0.6 + 0.2, 0, 0,
/// 0)` — the x/y/z draw order the three dstore rows speak, the
/// `#642` Double 0.6 × the `#229` Double 0.2 shifts the
/// 0.2..0.8 corner bands keep, the zero velocity trio the
/// `dconst_0` answers). The server's own torch speaks it —
/// `RedstoneTorchBlock.tick`'s tail the `sipush 1502, pos,
/// iconst_0` levelEvent call the bytecode pins (the `LIT` false
/// setBlock walk the burnout broadcast follows; the data word the
/// case's own body never reads — no `iload_3` the arm's bytecode
/// speaks). The GLOBAL route the `globalLevelEvent` switch never
/// banks (the 1023/1028/1038 trio the band alone carries — the
/// positioned lane the case's own arm wears).
pub const REDSTONE_TORCH_BURNOUT_EFFECT: i32 = 1502;

/// THE END-PORTAL-FRAME FILL EFFECT'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `PARTICLES_END_PORTAL_FRAME_FILL`, the case
/// `LevelEventHandler.levelEvent` offset 3773 speaks): the FIXED
/// voice the case body spends FIRST — the ONE
/// `playLocalSound(pos, END_PORTAL_FRAME_FILL, BLOCKS, 1.0F, 1.0F,
/// false)` walk the `fconst_1` pair the volume/pitch both carry (no
/// randoms the fixed fold speaks, the BLOCK CENTER the
/// `playLocalSound(BlockPos, ...)` resolve places, the distance-delay
/// flag false the immediate voice keeps) — then the `bipush 16` loop
/// the sixteen SMOKE puffs fire (`addParticle(SMOKE, x + (5.0 +
/// nextDouble() * 6.0) / 16.0, y + 0.8125, z + (5.0 + nextDouble() *
/// 6.0) / 16.0, 0, 0, 0)` — the x draw FIRST, the z draw SECOND, the
/// `#246` Double 5.0 × the `#647` Double 6.0 × the `#649` Double
/// 16.0 folds the 0.3125..0.6875 band keeps, the `#651` Double
/// 0.8125 lift the frame's own top the constant rides, the zero
/// velocity trio the `dconst_0` answers). The server's own
/// ender-eye placement speaks it — `EnderEyeItem.useOn`'s tail the
/// `sipush 1503, pos, iconst_0` levelEvent call the bytecode pins
/// (the eye the slot consumes — `shrink(1)` the walk keeps — the
/// portal-completion scan `getOrCreatePortalShape().find(...)` the
/// same tail answers; the data word the case's own body never reads
/// — no `iload_3` the arm's bytecode speaks). The GLOBAL route the
/// `globalLevelEvent` switch never banks (the 1023/1028/1038 trio
/// the band alone carries — the positioned lane the case's own arm
/// wears).
pub const END_PORTAL_FRAME_FILL_EFFECT: i32 = 1503;

/// THE DRIPSTONE-DRIP EFFECT'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `DRIPSTONE_DRIP`, the case
/// `LevelEventHandler.levelEvent` offset 3503 speaks): the ONE
/// `PointedDripstoneBlock.spawnDripParticle(level, pos,
/// level.getBlockState(pos))` walk the case body spends — the data
/// word the case never reads, NO sound the case speaks. The spawn
/// walk's own law the 26.2 jar pins: the `getFluidAboveStalactite`
/// gate FIRST (the `isStalactite` state read — the
/// `vertical_direction=down` tip the stalactite keeps; the up-walk
/// `findBlockVertical` the chain's root finds — the first
/// non-dripstone cell the ≤10-step walk answers, the state's own
/// `TIP_DIRECTION == down` continue-predicate), the fluid ABOVE the
/// root the drip pick reads (`minecraft:mud` → WATER, `minecraft:lava`
/// → LAVA, `minecraft:water` / `waterlogged=true` → WATER, else EMPTY
/// → the `DEFAULT_DRIPSTONE_PARTICLE` attribute the
/// `DRIPPING_DRIPSTONE_WATER` default answers); the position folds the
/// 4-arg spawn walk keeps (`state.getOffset(pos)` — the XZ offsetter
/// `Mth.getSeed(x, 0, z)`'s 4-bit draws the ±maxHorizontalOffset
/// clamp carries — the speleothem's own 0.125 the `SHAPE_BASE.min(X)`
/// speaks — then `x + 0.5 + offset.x`, the `y + STALACTITE_DRIP_START_PIXEL −
/// 0.0625` lift the `SHAPE_TIP_DOWN.min(Y)` fold keeps — 0.3125 −
/// 0.0625 = the +0.25 the tip's own point answers, `z + 0.5 +
/// offset.z`, the zero speed trio the `dconst_0` rows keep). The
/// server's own fluid-transfer walk speaks it —
/// `PointedDripstoneBlock.maybeTransferFluid`'s TWO `sipush 1504,
/// tipPos, iconst_0` levelEvent calls the bytecode pins (the
/// mud-to-clay branch and the cauldron branch BOTH broadcast at the
/// `findTip` cell the walk keeps — the fluid precondition the server
/// walk guarantees). The GLOBAL route the `globalLevelEvent` switch
/// never banks (the 1023/1028/1038 trio the band alone carries — the
/// positioned lane the case's own arm wears).
pub const DRIPSTONE_DRIP_EFFECT: i32 = 1504;

/// THE DISPENSER-DISPENSE VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `SOUND_DISPENSER_DISPENSE`,
/// the case `LevelEventHandler.levelEvent` offset 776 speaks): the
/// ONE `playLocalSound(pos, DISPENSER_DISPENSE, BLOCKS, 1.0F, 1.0F,
/// false)` walk the case body spends — the `fconst_1` pair the
/// volume/pitch both carry (no randoms the fixed fold speaks), the
/// BLOCK CENTER the `playLocalSound(BlockPos, ...)` resolve places,
/// the distance-delay flag false the immediate voice keeps, the data
/// word the case's own body never reads (no `iload_3` the arm's
/// bytecode speaks). NO particles the case speaks — the voice alone
/// the broadcast renders. The server's own dispenser speaks it —
/// `DefaultDispenseItemBehavior.playDefaultSound`'s `sipush 1000,
/// BlockSource.pos, iconst_0` the levelEvent call the bytecode pins
/// (the default behavior's own eject the sound the walk's own tail
/// broadcasts; the empty-slot `sipush 1001` FAIL word the sibling
/// case answers). The GLOBAL route the `globalLevelEvent` switch
/// never banks (the 1023/1028/1038 trio the band alone carries —
/// the positioned lane the case's own arm wears).
pub const SOUND_DISPENSER_DISPENSE_EFFECT: i32 = 1000;

/// THE DISPENSER-FAIL VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `SOUND_DISPENSER_FAIL`,
/// the case `LevelEventHandler.levelEvent` offset 755 speaks): the
/// ONE `playLocalSound(pos, DISPENSER_FAIL, BLOCKS, 1.0F, 1.2F,
/// false)` walk the case body spends — the `fconst_1` volume the
/// fixed fold keeps, the `ldc1 1.2` pitch the CP Float
/// 1.2000000476837158 speaks (the FAILED eject's own raised voice —
/// the pitch the pair's launch word shares), the BLOCK CENTER the
/// `playLocalSound(BlockPos, ...)` resolve places, the distance-delay
/// flag false the immediate voice keeps, the data word the case's
/// own body never reads (no `iload_3` the arm's bytecode speaks).
/// NO particles the case speaks — the old fail-smoke burst the
/// modern case never carries (the voice alone the broadcast
/// renders). The server's own dispensers speak it —
/// `DispenserBlock.dispenseFrom`'s fail branch and
/// `DropperBlock.dispenseFrom`'s fail branch (the `sipush 1001,
/// pos, iconst_0` levelEvent calls the bytecode pins), the optional
/// behavior's own ternary (`OptionalDispenseItemBehavior.playSound`'
/// `isSuccess` bit the `sipush 1000`/`sipush 1001` pair picks — the
/// success word and the fail word the ONE method speaks). The GLOBAL
/// route the `globalLevelEvent` switch never banks (the
/// 1023/1028/1038 trio the band alone carries — the positioned lane
/// the case's own arm wears).
pub const SOUND_DISPENSER_FAIL_EFFECT: i32 = 1001;

/// THE DISPENSER-LAUNCH VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks —
/// `SOUND_DISPENSER_PROJECTILE_LAUNCH`, the case
/// `LevelEventHandler.levelEvent` offset 857 speaks): the ONE
/// `playLocalSound(pos, DISPENSER_LAUNCH, BLOCKS, 1.0F, 1.2F,
/// false)` walk the case body spends — the `fconst_1` volume the
/// fixed fold keeps, the `ldc1 1.2` pitch the CP Float
/// 1.2000000476837158 speaks (the fail word's own raised pitch the
/// pair shares — the launch's bow voice the same band wears), the
/// BLOCK CENTER the `playLocalSound(BlockPos, ...)` resolve places,
/// the distance-delay flag false the immediate voice keeps, the data
/// word the case's own body never reads (no `iload_3` the arm's
/// bytecode speaks). NO particles the case speaks — the voice alone
/// the broadcast renders (the projectile's own entity the server
/// spawns by the packet's own means, the voice the event alone
/// carries). The server's own projectile behaviors speak it —
/// `ProjectileDispenseBehavior.playSound`'s tail (the `sipush 1002,
/// source.pos(), iconst_0` levelEvent call the bytecode pins — the
/// `source.level()` resolve the invokevirtual chain keeps). The
/// GLOBAL route the `globalLevelEvent` switch never banks (the
/// 1023/1028/1038 trio the band alone carries — the positioned lane
/// the case's own arm wears).
pub const SOUND_DISPENSER_PROJECTILE_LAUNCH_EFFECT: i32 = 1002;

/// THE FIREWORK-ROCKET SHOOT VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `SOUND_FIREWORK_SHOOT`
/// the ConstantValue attribute the decode pins, the case
/// `LevelEventHandler.levelEvent` offset 836 speaks): the ONE
/// `playLocalSound(pos, FIREWORK_ROCKET_SHOOT, NEUTRAL, 1.0F, 1.2F,
/// false)` walk the case body spends — the `fconst_1` volume the
/// fixed fold keeps, the `ldc1 1.2` pitch the CP Float
/// 1.2000000476837158 speaks (the dispenser band's own raised pitch
/// the sibling words share), the BLOCK CENTER the
/// `playLocalSound(BlockPos, ...)` resolve places, the distance-delay
/// flag false the immediate voice keeps, the data word the case's
/// own body never reads (no `iload_3` the arm's bytecode speaks).
/// NO particles the case speaks — the voice alone the broadcast
/// renders. The source the NEUTRAL shelf the `getstatic` row picks
/// (the dispenser pair's BLOCKS words the sibling rows keep — the
/// one `getstatic` the case's own bytecode names). The server's own
/// firework rockets speak it — `FireworkRocketItem.createDispenseConfig`'s
/// builder chain the `.overrideDispenseEvent(1004)` word pins
/// (`positionFunction(...).uncertainty(1.0F).power(0.5F)` the chain
/// the bytecode keeps), `ProjectileDispenseBehavior.playSound`'s
/// tail the word broadcasts (`levelEvent(dispenseConfig
/// .overrideDispenseEvent().orElse(1000), source.pos(), 0)` the
/// invokevirtual chain speaks — the firework's own dispense the
/// overridden word rides). The GLOBAL route the `globalLevelEvent`
/// switch never banks (the 1023/1028/1038 trio the band alone
/// carries — the positioned lane the case's own arm wears).
pub const FIREWORK_ROCKET_SHOOT_EFFECT: i32 = 1004;

/// THE FIRE-EXTINGUISH VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `SOUND_EXTINGUISH_FIRE`,
/// the case `LevelEventHandler.levelEvent` offset 4137 speaks): the
/// DATA word the case READS — the TWO-branch fold the `iload_3`
/// pairs keep (`data == 0` the ONE `playLocalSound(pos,
/// FIRE_EXTINGUISH, BLOCKS, 0.5F, 2.6F + (nextFloat() - nextFloat())
/// * 0.8F, false)` walk the head branch speaks, `data == 1` the ONE
/// `playLocalSound(pos, GENERIC_EXTINGUISH_FIRE, BLOCKS, 0.7F, 1.6F
/// + (nextFloat() - nextFloat()) * 0.4F, false)` walk the tail
/// branch speaks — the pitch draws the first-minus-second the fsub
/// order spends, the volume the `ldc` pair each branch carries, the
/// BLOCK CENTER the `playLocalSound(BlockPos, ...)` resolve places,
/// the distance-delay flag false the immediate voices keep); the
/// words the neither branch answers the case's own silence keeps —
/// no voice the pair of `ifne`/`if_icmpne` guards speaks). NO
/// particles the case speaks — the voice alone the broadcast
/// renders. The sound keys the official sounds.json table banks
/// (`block.fire.extinguish` → random/fizz, `entity.generic.
/// extinguish_fire` → random/fizz — the SAME file the pitched-smoke
/// band's fizz word speaks, NO new assets the lane demands). The
/// server's own fire the word broadcasts — `BaseFireBlock
/// .playerWillDestroy`'s `isClientSide` guard the `sipush 1009,
/// aload pos, iconst_0` levelEvent call the bytecode pins (the
/// broken-fire word the extinguish voice wears). The GLOBAL route
/// the `globalLevelEvent` switch never banks (the 1023/1028/1038
/// trio the band alone carries — the positioned lane the case's own
/// arm wears).
pub const SOUND_EXTINGUISH_FIRE_EFFECT: i32 = 1009;

/// THE JUKEBOX-PLAY VOICE'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `SOUND_PLAY_JUKEBOX_SONG`, the case
/// `LevelEventHandler.levelEvent` offset 4405 speaks): the DATA word
/// the case READS (`iload_3` the arm's bytecode speaks — the jukebox
/// song's synced-registry id the `registryAccess()` +
/// `Registries.JUKEBOX_SONG` resolve carries, the `Optional.ifPresent`
/// gate the case body spends) — the present arm the ONE
/// `playJukeboxSong(song, pos)` walk speaks: the PREVIOUS record at
/// the anchor the `stopJukeboxSong(pos)` fold ends first (the
/// `playingJukeboxSongs` map's own remove — the `SoundManager.stop`
/// the non-null gate keeps), then the ONE
/// `SimpleSoundInstance.forJukeboxSong(song.value().soundEvent(),
/// Vec3.atCenterOf(pos))` walk banks — the `SoundSource.RECORDS`
/// source the static row picks, the `ldc 4.0` volume the CP Float
/// speaks, the `fconst_1` pitch the fixed fold keeps, the unseeded
/// random the createUnseededRandom walk spends, the looping false /
/// delay 0 pair the iconst_0 pair keeps, the LINEAR attenuation the
/// static row answers — then `playingJukeboxSongs.put(pos, instance)`
/// + `SoundManager.play` the map-and-voice pair speaks, the
/// `Hud.setNowPlaying(song.description())` the HUD walk banks (the
/// `record.nowPlaying` format the description arg fills, the
/// `setOverlayMessage(component, true)` animated-color arm the
/// `iconst_1` keeps — the 60-tick overlay life the bipush 60
/// speaks), and `notifyNearbyEntities(level, pos, true)` the tail
/// walk spends (the `AABB(pos).inflate(3.0)` sweep the entities the
/// `getEntitiesOfClass(LivingEntity.class, ...)` query answers — the
/// per-entity `setRecordPlayingNearby(pos, true)` call the jar's own
/// hook keeps EMPTY, `0: return` the 26.2 bytecode speaks). The
/// absent arm (the data word outside the synced table) the
/// `Optional.ifPresent` gate answers with NOTHING — no stop, no
/// play, no line the empty resolve keeps.
pub const SOUND_PLAY_JUKEBOX_SONG_EFFECT: i32 = 1010;

/// THE JUKEBOX-STOP VOICE'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `SOUND_STOP_JUKEBOX_SONG`, the case
/// `LevelEventHandler.levelEvent` offset 4439 speaks): the ONE
/// `stopJukeboxSongAndNotifyNearby(pos)` walk the case body spends —
/// the `stopJukeboxSong(pos)` fold the tracked record ends (the
/// `playingJukeboxSongs` map's own remove — the `SoundManager.stop`
/// the non-null gate keeps, the absent record the silent arm keeps)
/// + `notifyNearbyEntities(level, pos, false)` the tail walk spends
/// (the same sweep the play arm's tail speaks — the per-entity hook
/// the jar keeps EMPTY, `0: return` the 26.2 bytecode speaks). The
/// data word the case's own body never reads (no `iload_3` the arm's
/// bytecode speaks). NO particles the case speaks — the stopped
/// record the silence itself renders.
pub const SOUND_STOP_JUKEBOX_SONG_EFFECT: i32 = 1011;

/// THE CHORUS-FLOWER GROW VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `SOUND_CHORUS_FLOWER_GROW`,
/// the case `LevelEventHandler.levelEvent` offset 680 speaks): the
/// ONE `playLocalSound(pos, CHORUS_FLOWER_GROW, BLOCKS, 1.0F, 1.0F,
/// false)` walk the case body spends — the `fconst_1` pair the
/// volume/pitch both carry (no randoms the fixed fold speaks), the
/// BLOCK CENTER the `playLocalSound(BlockPos, ...)` resolve places,
/// the distance-delay flag false the immediate voice keeps, the data
/// word the case's own body never reads (no `iload_3` the arm's
/// bytecode speaks). NO particles the case speaks — the voice alone
/// the broadcast renders. The server's own chorus flower speaks it —
/// `ChorusFlowerBlock.placeGrownFlower`'s tail the `sipush 1033,
/// aload_2 pos, iconst_0` levelEvent call the bytecode pins (the
/// survived-growth word the flower's own spread broadcasts). The
/// GLOBAL route the `globalLevelEvent` switch never banks (the
/// 1023/1028/1038 trio the band alone carries — the positioned lane
/// the case's own arm wears).
pub const SOUND_CHORUS_GROW_EFFECT: i32 = 1033;

/// THE CHORUS-FLOWER DEATH VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `SOUND_CHORUS_FLOWER_DEATH`,
/// the case `LevelEventHandler.levelEvent` offset 700 speaks): the
/// ONE `playLocalSound(pos, CHORUS_FLOWER_DEATH, BLOCKS, 1.0F, 1.0F,
/// false)` walk the case body spends — the `fconst_1` pair the
/// volume/pitch both carry, the BLOCK CENTER the
/// `playLocalSound(BlockPos, ...)` resolve places, the distance-delay
/// flag false the immediate voice keeps, the data word the case's own
/// body never reads. NO particles the case speaks — the voice alone
/// the broadcast renders. The server's own chorus flower speaks it —
/// `ChorusFlowerBlock.placeDeadFlower`'s tail the `sipush 1034,
/// aload_2 pos, iconst_0` levelEvent call the bytecode pins (the
/// withered word the flower's own death broadcasts). The GLOBAL route
/// the `globalLevelEvent` switch never banks — the positioned lane
/// the case's own arm wears.
pub const SOUND_CHORUS_DEATH_EFFECT: i32 = 1034;

/// THE BREWING-STAND BREW VOICE'S OWN ID (the 26.2 `LevelEvent`
/// constants class the client jar banks — `SOUND_BREWING_STAND_BREW`,
/// the case `LevelEventHandler.levelEvent` offset 660 speaks): the
/// ONE `playLocalSound(pos, BREWING_STAND_BREW, BLOCKS, 1.0F, 1.0F,
/// false)` walk the case body spends — the `fconst_1` pair the
/// volume/pitch both carry, the BLOCK CENTER the
/// `playLocalSound(BlockPos, ...)` resolve places, the distance-delay
/// flag false the immediate voice keeps, the data word the case's own
/// body never reads. NO particles the case speaks — the voice alone
/// the broadcast renders. The server's own brewing stand speaks it —
/// `BrewingStandBlockEntity.doBrew`'s tail the `sipush 1035,
/// aload pos, iconst_0` levelEvent call the bytecode pins (the
/// potion-complete word the stand's own brew broadcasts). The GLOBAL
/// route the `globalLevelEvent` switch never banks — the positioned
/// lane the case's own arm wears.
pub const SOUND_BREWING_STAND_BREW_EFFECT: i32 = 1035;

/// THE CRAFTER CRAFT VOICE'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `SOUND_CRAFTER_CRAFT`, the case
/// `LevelEventHandler.levelEvent` offset 796 speaks): the ONE
/// `playLocalSound(pos, CRAFTER_CRAFT, BLOCKS, 1.0F, 1.0F, false)`
/// walk the case body spends — the `fconst_1` pair the volume/pitch
/// both carry, the BLOCK CENTER the `playLocalSound(BlockPos, ...)`
/// resolve places, the distance-delay flag false the immediate voice
/// keeps, the data word the case's own body never reads. NO particles
/// the case speaks — the voice alone the broadcast renders. The
/// server's own crafter speaks it — `CrafterBlock.dispenseItem`'s
/// tail the `sipush 1049, aload pos, iconst_0` levelEvent call the
/// bytecode pins (the crafted word the eject's own success
/// broadcasts; the sibling `sipush 2010` the white-smoke lane's own
/// word the same walk carries — the white lane's arms already bank).
/// The GLOBAL route the `globalLevelEvent` switch never banks — the
/// positioned lane the case's own arm wears.
pub const SOUND_CRAFTER_CRAFT_EFFECT: i32 = 1049;

/// THE CRAFTER FAIL VOICE'S OWN ID (the 26.2 `LevelEvent` constants
/// class the client jar banks — `SOUND_CRAFTER_FAIL`, the case
/// `LevelEventHandler.levelEvent` offset 816 speaks): the ONE
/// `playLocalSound(pos, CRAFTER_FAIL, BLOCKS, 1.0F, 1.0F, false)`
/// walk the case body spends — the `fconst_1` pair the volume/pitch
/// both carry, the BLOCK CENTER the `playLocalSound(BlockPos, ...)`
/// resolve places, the distance-delay flag false the immediate voice
/// keeps, the data word the case's own body never reads. NO particles
/// the case speaks — the voice alone the broadcast renders. The
/// server's own crafter speaks it — `CrafterBlock.dispenseFrom`'s
/// empty-slot arms the `sipush 1050, aload pos, iconst_0` levelEvent
/// call the bytecode pins (the refused word the empty inventory's
/// own craft attempt broadcasts, BOTH early-return tails the same
/// walk carries). The GLOBAL route the `globalLevelEvent` switch
/// never banks — the positioned lane the case's own arm wears.
pub const SOUND_CRAFTER_FAIL_EFFECT: i32 = 1050;

/// One Level Event (clientbound play, generated id 46 —
/// `minecraft:level_event`). javap of the 26.2 write walk verbatim:
/// `writeInt(type)`, `writeBlockPos(pos)`, `writeInt(data)`,
/// `writeBoolean(globalEvent)` — the CLASSIC int-long-int-bool shape
/// (no varints), the codec's own composite order.
#[derive(Debug, Clone, PartialEq)]
pub struct LevelEventPacket {
    /// The event id (the `LevelEvent` constants' own word —
    /// [`GROWTH_EFFECT`] the bone-meal walk answers).
    pub effect: i32,
    /// The event's anchor block (the `writeBlockPos` LONG the packed
    /// layout carries — [`decode_block_pos`] the same walk resolves).
    pub x: i32,
    pub y: i32,
    pub z: i32,
    /// The event's own data word (the growth effect's particle count
    /// the server's `bipush 15` speaks; the handler passes it verbatim
    /// — the old client's `0 → 15` fold the 26.2 walk no longer keeps).
    pub data: i32,
    /// The GLOBAL flag (the broadcast the coords ignore — the
    /// wither's spawn, the end-dragon's defeat; the growth effect the
    /// positioned lane speaks, the flag false).
    pub global: bool,
}

/// Parse Level Event. javap of the 26.2 read ctor verbatim:
/// `readInt()` → type, `readBlockPos()` → pos, `readInt()` → data,
/// `readBoolean()` → globalEvent.
pub fn parse_level_event(payload: &[u8]) -> Result<LevelEventPacket> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::LEVEL_EVENT,
        "not a level event packet (0x{id:02X})"
    );
    let effect = read_i32(&mut c)?;
    let packed = read_i64(&mut c)?;
    let data = read_i32(&mut c)?;
    let global = read_bool(&mut c)?;
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the level event",
        c.len()
    );
    let (x, y, z) = decode_block_pos(packed);
    Ok(LevelEventPacket {
        effect,
        x,
        y,
        z,
        data,
        global,
    })
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

/// Set Passengers: who rides what NOW (the vanilla `ClientboundSet-
/// PassengersPacket` — generated id 107, `minecraft:set_passengers`).
/// THE WIRE (javap of the real record's ctor): the vehicle's entity id
/// as ONE VarInt, then the passengers as `readVarIntArray()` — a VarInt
/// count then that many VarInt entity ids. An EMPTY array is the
/// dismount broadcast (the vehicle now carries nobody); the packet is
/// the FULL list, not a delta.
#[derive(Debug, Clone, PartialEq)]
pub struct SetPassengers {
    /// The vehicle's entity id.
    pub vehicle: i32,
    /// The FULL passenger list (empty = the vehicle carries nobody).
    pub passengers: Vec<i32>,
}

/// The Set Passengers walk (the ctor's own two reads, verbatim).
pub fn parse_set_passengers(payload: &[u8]) -> Result<SetPassengers> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SET_PASSENGERS,
        "not a set passengers packet (0x{id:02X})"
    );
    let vehicle = read_varint(&mut c)?;
    let count = read_varint(&mut c)?;
    if count < 0 || count as usize > c.len() {
        return Err(anyhow::anyhow!(
            "absurd passenger count {count} for {} remaining bytes",
            c.len()
        ));
    }
    let mut passengers = Vec::with_capacity(count as usize);
    for _ in 0..count {
        passengers.push(read_varint(&mut c)?);
    }
    anyhow::ensure!(
        c.is_empty(),
        "{} trailing bytes after the set passengers packet",
        c.len()
    );
    Ok(SetPassengers { vehicle, passengers })
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
    /// `BundleContents.STREAM_CODEC` (the clinit walk's registration
    /// order: custom_data 0 .. bundle_contents 50 — the same census
    /// the table's 0/1/3 pins share) — `ItemStackTemplate.STREAM_CODEC
    /// .apply(ByteBufCodecs.list())` mapped: a VarInt count (the
    /// NO-ARG `ByteBufCodecs.list()`, ceiling `Integer.MAX_VALUE` —
    /// readCount's ceiling check is the only guard, a negative count
    /// still dies at the constructor) then count × ItemStackTemplate:
    /// the item's `$29` byIdOrThrow VarInt (AIR — id 0 — refuses at
    /// the template ctor's `Item must be non-empty`), the RAW count
    /// lane (the template carries NO emptiness sentinel — the
    /// particle lane's own note), and the entry's OWN
    /// `DataComponentPatch.STREAM_CODEC` (the recursion: a bundle
    /// riding a bundle — the nesting the port caps at the NBT
    /// reader's own 128-depth discipline). The wire NEVER carries the
    /// selected index — the `map`'s wrap builds the contents with the
    /// (list) ctor's -1 default, any server sync RESETS the client's
    /// selection (the bundle-selection round's own law).
    pub const BUNDLE_CONTENTS: i32 = 50;
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

/// One bundle-contents ENTRY — the `ItemStackTemplate.STREAM_CODEC`
/// composite the `BundleContents.STREAM_CODEC` lists: the item's
/// registry id (the `$29` byIdOrThrow VarInt — AIR, id 0, refuses at
/// the template ctor's `Item must be non-empty`), the RAW count lane
/// (the template carries NO emptiness sentinel — the count is the
/// record's own int, retained verbatim), and the entry's OWN
/// component patch (the recursion lane — a bundle riding a bundle).
#[derive(Debug, Clone, PartialEq)]
pub struct BundleEntry {
    /// The entry's item REGISTRY id (`BuiltInRegistries.ITEM` order;
    /// never 0 — the template ctor's own refusal).
    pub item_id: i32,
    /// The entry's stack size — the RAW VarInt the template lane
    /// carries (retained verbatim; the consumer filters).
    pub count: i32,
    /// The entry's component patch (the nested lane — the bundle
    /// riding a bundle's own contents rides here).
    pub patch: ComponentPatch,
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
    /// The `bundle_contents` component's decoded walk
    /// ([`data_component_types::BUNDLE_CONTENTS`]): the template list
    /// the `BundleContents.STREAM_CODEC` carries, in wire order. The
    /// selected index the wire never speaks (the wrap's -1 default —
    /// the client's belief, any sync's reset).
    BundleContents(Vec<BundleEntry>),
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
fn read_component_value(type_id: i32, c: &mut &[u8], depth: u32) -> Result<ComponentValue> {
    use data_component_types as t;
    Ok(match type_id {
        t::BUNDLE_CONTENTS => {
            // THE TEMPLATE LIST (the `BundleContents.STREAM_CODEC`
            // walk): the no-arg list's ceiling the house law keeps
            // (the negative count refuses, the byte shortage is the
            // unframed lane's own guard), then the templates — the
            // AIR refusal the ctor's IllegalStateException speaks,
            // the RAW count lane, the entry's own patch (the
            // recursion the depth cap disciplines)
            let n = read_component_count(c, "bundle-contents")?;
            let mut items = Vec::new();
            for _ in 0..n {
                let item_id = read_varint(c)?;
                anyhow::ensure!(
                    item_id >= 0,
                    "negative bundle entry item id {item_id}"
                );
                anyhow::ensure!(
                    item_id != 0,
                    "bundle entry item id 0 (air) — the ItemStackTemplate \
                     ctor's 'Item must be non-empty' IllegalStateException"
                );
                let count = read_varint(c)?;
                let patch = parse_component_patch_depth(c, depth + 1)?;
                items.push(BundleEntry { item_id, count, patch });
            }
            ComponentValue::BundleContents(items)
        }
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
        let value = read_component_value(type_id, c, 0)?;
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
    parse_component_patch_depth(c, 0)
}

/// The depth-threaded patch walk — the recursion the bundle
/// contents' entries ride (a bundle riding a bundle's own patch
/// nests the dispatch again). The cap mirrors the NBT reader's own
/// `MAX_DEPTH` discipline: the wire is unframed, an unbounded nest
/// would exhaust the reader's stack — the port refuses at the same
/// lane the NBT family does.
fn parse_component_patch_depth(c: &mut &[u8], depth: u32) -> Result<ComponentPatch> {
    const MAX_DEPTH: u32 = 128;
    anyhow::ensure!(
        depth <= MAX_DEPTH,
        "component-patch nesting past the {}-depth lane (the NBT reader's own cap family)",
        MAX_DEPTH
    );
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
        let value = read_component_value(type_id, c, depth)?;
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
/// (`BlockPos.asLong`): x in bits 38+, z in 12..38, y in 0..12 — the
/// 26-bit HORIZONTAL spans (both x and z, the
/// PACKED_HORIZONTAL_LENGTH's own 26) with the 12-bit y between the
/// sign-extension truncations the getters invert. The dig lane's hex
/// test re-proved the 26-bit z: the negative coordinates' sign bits
/// ride z's own field, never bleeding into x (the spans tile the i64
/// exactly: 26+26+12 = 64).
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
// the container's serverbound voice
// ---------------------------------------------------------------------------

/// The click's input mode — the 26.2 `ContainerInput` enum's registration
/// order, DUMPED from the official jar's own enum (the checked-in
/// `ReflectContainerClick.java` lists the constants in
/// `getEnumConstants()` order): PICKUP, QUICK_MOVE, SWAP, CLONE, THROW,
/// QUICK_CRAFT, PICKUP_ALL. The wire rides the ordinal as a VarInt (the
/// enum's codec is the ByIdMap id-mapper — ordinal in, ordinal out).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerInput {
    Pickup,
    QuickMove,
    Swap,
    Clone,
    Throw,
    QuickCraft,
    PickupAll,
}

impl ContainerInput {
    /// The wire's own VarInt value (the registration-order ordinal).
    pub fn wire_ordinal(self) -> i32 {
        self as i32
    }
}

/// The hashed component patch — the 26.2 `HashedPatchMap` record
/// (`Map addedComponents, Set removedComponents`, the dump's own record
/// components). The wire shape is DUMPED (`ReflectHashedStack.java`):
/// each ADDED entry rides the component type's data_component_type
/// REGISTRY id as a VarInt followed by the value's hash as a FIXED
/// BIG-ENDIAN int32 (the negative and the large probes refused a VarInt
/// reading: `06 ff ff ff 9d` is -99, `03 07 5b cd 15` is 123456789); the
/// REMOVED entries ride the type ids as plain VarInts. The keys are REAL
/// component types — a synthetic int key was REFUSED by the codec ("Can't
/// find id for '19' in map Registry[minecraft:data_component_type]") —
/// this boot pins DAMAGE = 3, CUSTOM_NAME = 6.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashedPatchMap {
    /// (the component type's registry id, the value's hash) pairs.
    pub added: Vec<(i32, i32)>,
    /// The removed component types' registry ids.
    pub removed: Vec<i32>,
}

impl HashedPatchMap {
    /// The empty patch — both lanes empty (the dump's `00 00`, the shape
    /// every default-component stack rides).
    pub const EMPTY: HashedPatchMap = HashedPatchMap {
        added: Vec::new(),
        removed: Vec::new(),
    };

    /// An added-only patch (the dump's synthetic shape: the component
    /// type's registry id + the value's hash).
    pub fn added(ty: i32, hash: i32) -> HashedPatchMap {
        HashedPatchMap {
            added: vec![(ty, hash)],
            removed: Vec::new(),
        }
    }
}

/// The client's believed stack state — the 26.2 `HashedStack` prediction
/// lane. The click packet's changed-slots and carried lanes carry HASHED
/// stacks (the components ride as hashes, not bytes): the item's WIRE id
/// + count + the component PATCH map (`HashedPatchMap` — the dump's
/// record `Map addedComponents, Set removedComponents`). The prediction-
/// free client's honest record is the count pair with the EMPTY patch
/// (`00 00`); the full patch shape is DUMPED and speakable
/// (`ReflectHashedStack.java`), and the store's future component lanes
/// fill it where the count pair alone is not the whole belief.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashedStack {
    /// The `00` byte — the optional lane's emptiness.
    Empty,
    /// The `01` byte + the item's wire registry id + the count + the
    /// hashed patch map (the EMPTY map for every default-component
    /// stack; the dump pins the non-empty shapes byte for byte).
    ActualItem {
        item_id: i32,
        count: i32,
        patch: HashedPatchMap,
    },
}

impl HashedStack {
    /// The count pair with the EMPTY patch — the shape a (item_id,
    /// count) store projects honestly (every default-component stack's
    /// own lane).
    pub fn item(item_id: i32, count: i32) -> HashedStack {
        HashedStack::ActualItem {
            item_id,
            count,
            patch: HashedPatchMap::EMPTY,
        }
    }
}

/// The container click the vanilla client speaks (the 26.2 record
/// `ServerboundContainerClickPacket`: containerId, stateId, slotNum,
/// buttonNum, containerInput, changedSlots, carriedItem — the wire
/// layout DUMPED from the official STREAM_CODEC by the checked-in
/// `ReflectContainerClick.java`; see the encoder for the bytes).
/// `state_id` is the revision the client last SAW (the Set
/// Content/Slot packets' counter — the store's own `state_id`), the
/// slot rides the MENU's own numbering, and the changed-slots/carried
/// lanes are the client's BELIEF (a prediction-free client sends an
/// empty delta list with an empty cursor — the server's authoritative
/// corrections ride back through the container lane).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerClick {
    /// The window id the server assigned (the Open Screen packet's).
    pub container_id: i32,
    /// The revision the client last saw (the store's `state_id`).
    pub state_id: i32,
    /// The clicked slot, the menu's own numbering (a chest: 0..27 the
    /// grid, 27..63 the player mirror).
    pub slot: i16,
    /// The button (0 = the left press).
    pub button: i8,
    /// The click's input mode (the wire's own enum).
    pub input: ContainerInput,
    /// The client's believed per-slot deltas (the prediction rounds
    /// fill this; the prediction-free lane rides empty).
    pub changed_slots: Vec<(i16, HashedStack)>,
    /// The client's believed cursor stack (empty without prediction).
    pub carried: HashedStack,
}

/// Container Click (serverbound play 0x12): the record's seven lanes
/// written back — the VarInt container id, the VarInt state id, the
/// SIGNED BE SHORT slot, the BYTE button, the VarInt input ordinal, the
/// count-prefixed changed-slot pairs (SHORT + HashedStack each) and the
/// carried HashedStack last. The bytes are the official STREAM_CODEC's
/// own output for the pinned fixtures (the dumps' evidence, byte for
/// byte):
/// `container=2 state=7 slot=13 button=0 PICKUP empty/empty` =
/// `02 07 000d 00 00 0000` (body — the packet id prefix is the
/// dispatch's own lane, the house convention); the HashedStack's own
/// shapes ride `ReflectHashedStack.java`'s fixtures (the empty patch
/// `01 9e 07 40 00 00`, the non-empty patches — see `write_hashed_stack`).
pub fn encode_container_click(click: &ContainerClick) -> Result<Vec<u8>> {
    let mut p = Vec::with_capacity(9);
    write_varint(&mut p, serverbound::CONTAINER_CLICK);
    write_varint(&mut p, click.container_id);
    write_varint(&mut p, click.state_id);
    write_i16(&mut p, click.slot);
    p.push(click.button as u8);
    write_varint(&mut p, click.input.wire_ordinal());
    write_varint(&mut p, click.changed_slots.len() as i32);
    for (slot, stack) in &click.changed_slots {
        write_i16(&mut p, *slot);
        write_hashed_stack(&mut p, stack)?;
    }
    write_hashed_stack(&mut p, &click.carried)?;
    Ok(p)
}

/// One HashedStack lane's write mirror: the presence byte, then the
/// ActualItem's item id + count as VarInts and the hashed patch map —
/// the VarInt added-count, per entry the component type's registry id
/// as a VarInt + the value's hash as a FIXED BIG-ENDIAN int32, then the
/// VarInt removed-count with the type ids as VarInts. The bytes are the
/// official STREAM_CODEC's own output for the dump's fixtures
/// (`ReflectHashedStack.java`):
/// `diamond x64, empty patch` = `01 9e 07 40 00 00`,
/// `added{DAMAGE:12345}` = `01 9e 07 40 01 03 00 00 30 39 00`,
/// `removed{CUSTOM_NAME}` = `01 9e 07 40 00 01 06`,
/// `added{DAMAGE:12345,CUSTOM_NAME:-99} removed{CUSTOM_NAME}` =
/// `01 9e 07 40 02 03 00 00 30 39 06 ff ff ff 9d 01 06`,
/// `added{DAMAGE:123456789}` = `01 9e 07 40 01 03 07 5b cd 15 00`
/// (DAMAGE = component-type id 3, CUSTOM_NAME = 6 — this boot's own
/// registry order).
fn write_hashed_stack(out: &mut Vec<u8>, stack: &HashedStack) -> Result<()> {
    match stack {
        HashedStack::Empty => out.push(0),
        HashedStack::ActualItem {
            item_id,
            count,
            patch,
        } => {
            out.push(1);
            write_varint(out, *item_id);
            write_varint(out, *count);
            write_varint(out, patch.added.len() as i32);
            for (ty, hash) in &patch.added {
                write_varint(out, *ty);
                out.extend_from_slice(&hash.to_be_bytes());
            }
            write_varint(out, patch.removed.len() as i32);
            for ty in &patch.removed {
                write_varint(out, *ty);
            }
        }
    }
    Ok(())
}

/// Container Close (serverbound play 0x13): the window id — the
/// player's OWN close (Escape/E on a container screen), the echo the
/// clientbound Container Close lane mirrors.
pub fn encode_container_close(container_id: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(3);
    write_varint(&mut p, serverbound::CONTAINER_CLOSE);
    write_varint(&mut p, container_id);
    p
}

/// The player's own hotbar selection — the serverbound Set Carried
/// Item packet (the generated id table's SB_SET_CARRIED_ITEM = 53,
/// `minecraft:set_carried_item`). THE WIRE (the checked-in
/// `ReflectSetCarriedItem.java`'s dump): the ctor takes the
/// SELECTION-relative slot (the number keys' own 0..8,
/// `Inventory.selected`) and the official STREAM_CODEC writes it as a
/// FIXED BIG-ENDIAN SHORT — the two-byte shapes `0000` / `0003` /
/// `0008` refuse a VarInt reading (slot 3's VarInt would be the
/// single `03`); the wiki-era VarInt memory is wrong, the dump is the
/// truth. The caller speaks the 0..8 domain (the vanilla selection's
/// own range — the packet itself would ride a wider short, the model's
/// gate is where the domain lives).
pub fn encode_set_carried_item(slot: u8) -> Vec<u8> {
    let mut p = Vec::with_capacity(3);
    write_varint(&mut p, serverbound::SET_CARRIED_ITEM);
    p.extend_from_slice(&(slot as i16).to_be_bytes());
    p
}

/// The middle-click PICK's wire half (the vanilla
/// `MultiPlayerGameMode.handlePickItemFromBlock`'s own echo —
/// ServerboundPickItemFromBlockPacket, generated id 36,
/// `minecraft:pick_item_from_block`). THE WIRE (javap of the real
/// 26.2 class: the STREAM_CODEC composite the static init speaks —
/// `BlockPos.STREAM_CODEC` first, `ByteBufCodecs.BOOL` second): the
/// hit block's position as a BE i64 (the same `asLong` 38/12/0 pack
/// the use-item-on walk rides) then the includeData boolean — the
/// Ctrl flag the client's hasControlDown() carries (the survival
/// conjunct the server's own `hasInfiniteMaterials() && includeData()`
/// gate keeps FALSE). THE CLIENT PREDICTS NOTHING: the inventory law
/// lives server-side (the integrated server's tryPickItem walk — the
/// findSlotMatchingItem scan, the pickSlot swap, the Set Held Slot
/// echo + broadcastChanges the ALWAYS tail speaks), the packet the
/// gameMode's networked half sends ALONE.
pub fn encode_pick_item_from_block(x: i32, y: i32, z: i32, include_data: bool) -> Vec<u8> {
    let mut p = Vec::with_capacity(10);
    write_varint(&mut p, serverbound::PICK_ITEM_FROM_BLOCK);
    p.extend_from_slice(&pack_block_pos(x, y, z).to_be_bytes());
    write_bool(&mut p, include_data);
    p
}

/// The pick's ENTITY arm's wire (the vanilla
/// `MultiPlayerGameMode.handlePickItemFromEntity`'s own echo —
/// ServerboundPickItemFromEntityPacket, generated id 37,
/// `minecraft:pick_item_from_entity`). THE WIRE (javap of the real
/// 26.2 class: the STREAM_CODEC composite — `ByteBufCodecs.VAR_INT`
/// first, `ByteBufCodecs.BOOL` second): the target's entity id as a
/// VarInt, then the includeData boolean. The SURVIVAL client never
/// sends this one (the pick result the mob lanes never name —
/// `getPickResult` null, the spawn eggs the creative gate holds); the
/// encoder rides the protocol's own completeness (the generated pair
/// the id table keeps together).
pub fn encode_pick_item_from_entity(entity_id: i32, include_data: bool) -> Vec<u8> {
    let mut p = Vec::with_capacity(6);
    write_varint(&mut p, serverbound::PICK_ITEM_FROM_ENTITY);
    write_varint(&mut p, entity_id);
    write_bool(&mut p, include_data);
    p
}

/// The bundle selection's wire half (the vanilla
/// `BundleMouseActions.toggleSelectedBundleItem`'s own echo —
/// ServerboundSelectBundleItemPacket, generated id 3,
/// `minecraft:bundle_item_selected`). THE WIRE (javap of the real 26.2
/// class: the STREAM_CODEC composite the clinit's
/// `Packet.codec(StreamMemberEncoder, StreamDecoder)` keeps — the
/// member `write(FriendlyByteBuf)` writes the record's two fields):
/// the hovered slot's MENU id as a VarInt, then the selected index as
/// a VarInt. THE VALIDATION lives on the READ side (the server's
/// `read(FriendlyByteBuf)` ctor: `selectedItemIndex < 0 &&
/// selectedItemIndex != -1` throws — the deselected -1 the law keeps,
/// every other negative refuses); the WRITE path (the client's own
/// walk) carries no check — the `selected >= n` bound the
/// `toggleSelectedBundleItem` guard speaks is the walk's own, the
/// wrap the `[0, n)` domain the caller keeps.
pub fn encode_select_bundle_item(slot_id: i32, selected_item_index: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(4);
    write_varint(&mut p, serverbound::BUNDLE_ITEM_SELECTED);
    write_varint(&mut p, slot_id);
    write_varint(&mut p, selected_item_index);
    p
}

/// The right-mouse USE (the vanilla `MultiPlayerGameMode.useItem`'s
/// serverbound echo — ServerboundUseItemPacket, generated id 67,
/// `minecraft:use_item`). THE WIRE (javap of the real 26.2 class, its
/// private `write(FriendlyByteBuf)`): the hand as a VarInt (the
/// InteractionHand enum's ordinal — MAIN_HAND = 0), the prediction
/// sequence as a VarInt, then the player's yRot and xRot as BE f32s.
/// The caller speaks the camera's own angles converted to the vanilla
/// degrees (`world_view::wire_angles`) and a monotonically increasing
/// sequence (the startPrediction counter's own shape).
pub fn encode_use_item(hand: u8, sequence: i32, yaw: f32, pitch: f32) -> Vec<u8> {
    let mut p = Vec::with_capacity(12);
    write_varint(&mut p, serverbound::USE_ITEM);
    write_varint(&mut p, hand as i32);
    write_varint(&mut p, sequence);
    write_f32(&mut p, yaw);
    write_f32(&mut p, pitch);
    p
}

/// The use ON A BLOCK (the vanilla `MultiPlayerGameMode.useItemOn`'s own
/// echo — ServerboundUseItemOnPacket, generated id 66,
/// `minecraft:use_item_on`). THE WIRE (javap of the real 26.2 class's
/// `write(FriendlyByteBuf)`): the hand as a VarInt (the writeEnum's
/// ordinal), the BlockHitResult through FriendlyByteBuf's
/// `writeBlockHitResult` (javap verbatim: the BlockPos as a BE i64 —
/// `asLong`'s 38/12/0 pack, the direction as a VarInt (the writeEnum's
/// ordinal — DOWN=0, UP=1, NORTH=2, SOUTH=3, WEST=4, EAST=5), the CURSOR
/// as the hit LOCATION minus the block position's origin, each computed
/// in double and cast to float, three BE f32s, the `isInside` boolean,
/// the `isWorldBorderHit` boolean), then the prediction sequence as a
/// VarInt. The caller speaks the raycast's own hit (the client clip's
/// face ordinal and exact location) and a monotonically increasing
/// sequence (the same startPrediction counter the use/release echo).
pub fn encode_use_item_on(
    hand: u8,
    x: i32,
    y: i32,
    z: i32,
    face: u8,
    cursor: (f32, f32, f32),
    inside: bool,
    world_border: bool,
    sequence: i32,
) -> Vec<u8> {
    let mut p = Vec::with_capacity(22);
    write_varint(&mut p, serverbound::USE_ITEM_ON);
    write_varint(&mut p, hand as i32);
    p.extend_from_slice(&pack_block_pos(x, y, z).to_be_bytes());
    write_varint(&mut p, face as i32);
    write_f32(&mut p, cursor.0);
    write_f32(&mut p, cursor.1);
    write_f32(&mut p, cursor.2);
    p.push(if inside { 1 } else { 0 });
    p.push(if world_border { 1 } else { 0 });
    write_varint(&mut p, sequence);
    p
}

/// The use RELEASE (the vanilla `MultiPlayerGameMode.releaseUsingItem`'s
/// own echo — ServerboundPlayerActionPacket, generated id 41,
/// `minecraft:player_action`, the RELEASE_USE_ITEM action). THE WIRE
/// (javap of the real 26.2 class): the action as a VarInt (the enum's
/// ordinal — RELEASE_USE_ITEM = 5, the static init's iconst_5), the
/// BlockPos as a BE i64 (the release's BlockPos.ZERO = 0), the
/// direction as a single byte (the release's Direction.DOWN, the
/// 3D data value 0), the sequence as a VarInt (the 3-arg ctor's
/// iconst_0; the caller passes its own counter — the same increasing
/// number the use spoke).
pub fn encode_player_action_release(sequence: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(9);
    write_varint(&mut p, serverbound::PLAYER_ACTION);
    write_varint(&mut p, 5); // Action.RELEASE_USE_ITEM's ordinal
    p.extend_from_slice(&0i64.to_be_bytes()); // BlockPos.ZERO
    p.push(0); // Direction.DOWN's 3D data value
    write_varint(&mut p, sequence);
    p
}

/// The DIG wire — the destroy arc's own actions through the SAME
/// ServerboundPlayerActionPacket (generated id 41, `minecraft:player_action`).
/// THE ACTIONS (javap of the real 26.2 enum's static init, the ordinal
/// order): START_DESTROY_BLOCK = 0, ABORT_DESTROY_BLOCK = 1,
/// STOP_DESTROY_BLOCK = 2 (RELEASE_USE_ITEM = 5 — the release lane's
/// own constant above). THE WIRE (the read side's exact order, the
/// STREAM_CODEC's FriendlyByteBuf reads): the action as a VarInt (the
/// writeEnum's ordinal), the BlockPos as a BE i64 (`asLong`'s 38/12/0
/// pack — the pack reuses the existing block-pos pack), the direction
/// as a SINGLE BYTE (`readUnsignedByte` -> `from3DDataValue` — the 3D
/// data value, NOT a VarInt: DOWN=0, UP=1, NORTH=2, SOUTH=3, WEST=4,
/// EAST=5), the sequence as a VarInt. Every dig send rides
/// `MultiPlayerGameMode.startPrediction` — the same monotonic
/// prediction counter the use/use-on/release speaks (the caller passes
/// its own increasing number).
pub fn encode_player_action(
    action: u8,
    x: i32,
    y: i32,
    z: i32,
    face: u8,
    sequence: i32,
) -> Vec<u8> {
    let mut p = Vec::with_capacity(9);
    write_varint(&mut p, serverbound::PLAYER_ACTION);
    write_varint(&mut p, action as i32);
    p.extend_from_slice(&pack_block_pos(x, y, z).to_be_bytes());
    p.push(face); // the direction's 3D data value, one raw byte
    write_varint(&mut p, sequence);
    p
}

/// The arm SWING (the vanilla `LocalPlayer.swing`'s own echo —
/// ServerboundSwingPacket, generated id 63, `minecraft:swing`). THE
/// WIRE (javap of the real 26.2 class's `write(FriendlyByteBuf)`):
/// exactly one field — the hand as a VarInt (the writeEnum's ordinal,
/// MAIN_HAND = 0, OFF_HAND = 1). The dig arc swings on every attack
/// press and on every mining tick's `continueAttack` beat (the
/// vanilla's own cadence).
pub fn encode_swing(hand: u8) -> Vec<u8> {
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, serverbound::SWING);
    write_varint(&mut p, hand as i32);
    p
}

/// The CHAT wire (javap of the real 26.2 `ServerboundChatPacket.write` —
/// the FriendlyByteBuf walk verbatim): the message `writeUtf(msg, 256)`
/// (the VarInt BYTE length + the utf-8 bytes, the 256 the encoded-length
/// cap the write refuses past), the `writeInstant` = the
/// `Instant.toEpochMilli()` ONE long (the read side folds
/// `Instant.ofEpochMilli`), the salt long (the unsigned session's zero —
/// the house speaks no signed chat), the signature `writeNullable` (the
/// boolean first — `false` the absent lane), then the
/// `LastSeenMessages$Update.write` UNCONDITIONALLY (the read side's
/// direct `new Update(buf)` — the record `(int offset, BitSet
/// acknowledged, byte checksum)` the 26.2 class speaks: `writeVarInt(0)`
/// the no-offset walk, `writeFixedBitSet(empty, 20)` = THREE zero bytes
/// (the `(bits + 8) >> 3` fold: 20 bits -> 3), the checksum byte zero).
/// The vanilla server accepts the unsigned chat (the insecure badge the
/// signed-era machinery wears) — the wire the OFFLINE session's chat
/// lane honestly rides.
pub fn encode_chat(message: &str, epoch_millis: i64) -> Result<Vec<u8>> {
    anyhow::ensure!(
        message.len() <= 256,
        "chat message {} utf-8 bytes past the 256 writeUtf cap",
        message.len()
    );
    let mut p = Vec::with_capacity(4 + message.len() + 8 + 8 + 1 + 6);
    write_varint(&mut p, serverbound::CHAT);
    write_string(&mut p, message);
    p.extend_from_slice(&epoch_millis.to_be_bytes());
    p.extend_from_slice(&0i64.to_be_bytes()); // the salt: the unsigned session's zero
    p.push(0); // the signature: nullable absent
    write_varint(&mut p, 0); // the last-seen offset: none
    p.extend_from_slice(&[0u8; 3]); // the acknowledged bitset: 20 bits, empty
    p.push(0); // the checksum byte
    Ok(p)
}

/// The COMMAND wire (javap of the real 26.2 `ServerboundChatCommandPacket`
/// — the clinit's composite loads `ByteBufCodecs.UTF8` ALONE): the record
/// `(String command)` rides ONE bare utf — the signed-era
/// timestamp/salt/argument-signature fields the 1.21.x packet carried the
/// 26.2 protocol DROPPED (the signed commands the server-side history
/// walks no longer wear). The slash prefix the ChatScreen's own
/// `string.substring(1)` walk strips — the wire carries the command BODY
/// ("time set day"), never the '/'.
pub fn encode_chat_command(command: &str) -> Vec<u8> {
    let mut p = Vec::with_capacity(4 + command.len());
    write_varint(&mut p, serverbound::CHAT_COMMAND);
    write_string(&mut p, command);
    p
}

/// The SUGGESTION REQUEST wire (javap of the real 26.2
/// `ServerboundCommandSuggestionPacket` — the clinit's composite the
/// `Packet.codec` walk speaks): the record `(int id, String command)` —
/// `writeVarInt(id)` FIRST, then `writeUtf(command, 32500)`. The 26.2
/// packet the 1.20.x has-position fields DROPPED (the looked-at block
/// pos the request no longer carries) — the dump's write walk answers:
/// TWO fields, the transaction id the pendingSuggestionsId counter
/// speaks (the client's own increment the stale-reply gate answers),
/// the command the FULL input INCLUDING the leading '/' (the server's
/// `handleCustomCommandSuggestions` `if (peek == '/') skip()` walk the
/// strip keeps). The customSuggestion lane sends the parsed context's
/// input — the slash the StringReader skip never removed from the
/// string, the cursor it only advanced.
pub fn encode_command_suggestion(id: i32, command: &str) -> Result<Vec<u8>> {
    anyhow::ensure!(
        command.len() <= 32500,
        "suggestion command {} utf-8 bytes past the 32500 writeUtf cap",
        command.len()
    );
    let mut p = Vec::with_capacity(4 + command.len());
    write_varint(&mut p, serverbound::COMMAND_SUGGESTION);
    write_varint(&mut p, id);
    write_string(&mut p, command);
    Ok(p)
}

/// One suggestion row the reply's list carries (the 26.2
/// `ClientboundCommandSuggestionsPacket$Entry` record): the
/// `STRING_UTF8` text and the `TRUSTED_OPTIONAL_STREAM_CODEC` tooltip —
/// the optional the flag byte speaks (0 = absent; else the bare NBT
/// component, the text the [`flatten_component_text`] walk keeps).
#[derive(Debug, Clone, PartialEq)]
pub struct CommandSuggestionsMatch {
    /// The suggestion's own text — the replacement fragment the
    /// `Suggestion.getText` walk carries (a command word, a player
    /// name, an argument's option).
    pub text: String,
    /// The optional tooltip's flattened plain text (the hover the
    /// vanilla render's `setTooltipForNextFrame` lane keeps).
    pub tooltip: Option<String>,
}

/// The SUGGESTION REPLY (javap of the real 26.2
/// `ClientboundCommandSuggestionsPacket` — the clinit's
/// `composite(VAR_INT, VAR_INT, VAR_INT, list(Entry.STREAM_CODEC))` the
/// bytecode speaks): the transaction id (the request's own echo — the
/// stale-response gate the pendingSuggestionsFuture's id check keeps),
/// the replace range (start + length, offsets into the FULL input
/// string the client sent — the slash the server's parse keeps), and
/// the rows (the server caps `getList().subList(0, 1000)` the same
/// walk answers).
#[derive(Debug, Clone, PartialEq)]
pub struct CommandSuggestionsReply {
    /// The transaction id — the request packet's own `id` echo.
    pub id: i32,
    /// The replace range's start offset (chars into the sent input).
    pub start: i32,
    /// The replace range's length (the chars the text replaces).
    pub length: i32,
    /// The suggestion rows (the server's 1000-entry cap the walk keeps).
    pub matches: Vec<CommandSuggestionsMatch>,
}

/// The SUGGESTION REPLY's own read walk (see
/// [`CommandSuggestionsReply`] — the composite order the clinit
/// speaks): id, start, length, then the count + the rows (each the
/// capped utf + the flag-byte optional NBT tooltip — the same
/// TRUSTED_OPTIONAL walk the painting variant's title/author keeps).
pub fn parse_command_suggestions(payload: &[u8]) -> Result<CommandSuggestionsReply> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::COMMAND_SUGGESTIONS,
        "not a command suggestions packet (0x{id:02X})"
    );
    let tx_id = read_varint(&mut c)?;
    let start = read_varint(&mut c)?;
    let length = read_varint(&mut c)?;
    let count = read_varint(&mut c)?;
    anyhow::ensure!(
        count >= 0 && count <= 1000,
        "suggestion count {count} outside the server's subList(0, 1000) walk"
    );
    let mut matches = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let text = read_capped_string(&mut c, 32767)?;
        let tooltip = match read_u8(&mut c)? {
            0 => None,
            _ => Some(flatten_component_text(&read_bare_nbt(&mut c)?)),
        };
        matches.push(CommandSuggestionsMatch { text, tooltip });
    }
    Ok(CommandSuggestionsReply {
        id: tx_id,
        start,
        length,
        matches,
    })
}

// ---------------------------------------------------------------------------
// the commands tree (the server's own Brigadier graph — the local
// suggestions the chat screen's '/' lane answers)
// ---------------------------------------------------------------------------

/// The node's own kind — the flags byte's low two bits (javap
/// `ClientboundCommandsPacket.createEntry`'s tableswitch walk: the
/// RootCommandNode branch ORs `iconst_0`, the LiteralCommandNode branch
/// ORs `iconst_1`, the ArgumentCommandNode branch ORs `iconst_2` — the
/// unknown class the `UnsupportedOperationException` throw refuses).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandNodeKind {
    /// The tree's own entry point (type 0) — rides NO stub at all (the
    /// `aconst_null` the root branch stores).
    Root,
    /// A literal word (type 1) — `/time`'s own `time`.
    Literal,
    /// An argument slot (type 2) — `<targets>`'s own parser ride.
    Argument,
}

/// One node of the [`CommandTree`] — the 26.2 `Entry` record's decoded
/// shape (javap: the record `(NodeStub stub, int flags, int redirect,
/// int[] children)`; the stub's THREE shapes the inner classes speak).
#[derive(Debug, Clone, PartialEq)]
pub struct CommandNode {
    /// The kind the flags' low bits carry.
    pub kind: CommandNodeKind,
    /// `isExecutable` the flags' 0x04 bit (the node a command's own
    /// execution may land on).
    pub executable: bool,
    /// `isRestricted` the flags' 0x20 bit (the 26.2's own flag — the
    /// requires(src.hasPermission(2)) walk the owner-only nodes wear).
    pub restricted: bool,
    /// The children's entry indices (the `writeVarIntArray` walk).
    pub children: Vec<u32>,
    /// The redirect target's entry index (the flags' 0x08 bit — the
    /// `/tp`-aliases-`/teleport` graph the Brigadier walk keeps).
    pub redirect: Option<u32>,
    /// The literal's word or the argument's name — the stub's own utf
    /// (empty for the root, the only stub-less node).
    pub name: String,
    /// The argument's parser: the registry id + the synced name it
    /// resolved through (the `serializeCap` varint the
    /// command_argument_type registry answers; `None` for root/literal).
    pub parser: Option<(u32, String)>,
    /// The argument's suggestion provider id (the flags' 0x10 bit — the
    /// `writeIdentifier` walk; `minecraft:ask_server` the server's own
    /// dynamic lane rides, `None` for root/literal too).
    pub suggestion_id: Option<String>,
}

/// The server's whole command graph — the `ClientboundCommandsPacket`
/// record (readList(entries), readVarInt(rootIndex)) decoded. The chat
/// screen's local suggestions walk consumes the literals; the argument
/// slots' dynamic suggestions the ask-server lane still answers.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CommandTree {
    /// The root entry's index (the `getRoot` resolver's own walk).
    pub root_index: u32,
    /// The nodes in wire order — the index IS the id the children /
    /// redirect varints reference.
    pub nodes: Vec<CommandNode>,
}

/// The BUILT-IN command_argument_type registration order — the 26.2
/// jar's own `ArgumentTypeInfos` bootstrap verbatim (the `register`
/// calls' string order IS the registry id order the skip-sync path
/// answers; a server that streamed the registry REPLACES this table —
/// the arrival-order law every synced table wears). The namespaced
/// forms the registry sync carries (the `minecraft:` prefix the
/// `ResourceKey.create` walk speaks).
pub const BUILTIN_COMMAND_ARGUMENT_TYPES: &[&str] = &[
    "minecraft:brigadier:bool",
    "minecraft:brigadier:float",
    "minecraft:brigadier:double",
    "minecraft:brigadier:integer",
    "minecraft:brigadier:long",
    "minecraft:brigadier:string",
    "minecraft:entity",
    "minecraft:game_profile",
    "minecraft:block_pos",
    "minecraft:column_pos",
    "minecraft:block_state",
    "minecraft:block_predicate",
    "minecraft:item_stack",
    "minecraft:item_predicate",
    "minecraft:team_color",
    "minecraft:hex_color",
    "minecraft:component",
    "minecraft:style",
    "minecraft:message",
    "minecraft:nbt_compound_tag",
    "minecraft:nbt_tag",
    "minecraft:nbt_path",
    "minecraft:objective",
    "minecraft:objective_criteria",
    "minecraft:operation",
    "minecraft:particle",
    "minecraft:angle",
    "minecraft:rotation",
    "minecraft:scoreboard_slot",
    "minecraft:score_holder",
    "minecraft:swizzle",
    "minecraft:team",
    "minecraft:item_slot",
    "minecraft:item_slots",
    "minecraft:resource_location",
    "minecraft:function",
    "minecraft:entity_anchor",
    "minecraft:int_range",
    "minecraft:float_range",
    "minecraft:dimension",
    "minecraft:gamemode",
    "minecraft:time",
    "minecraft:resource_or_tag",
    "minecraft:resource_or_tag_key",
    "minecraft:resource",
    "minecraft:resource_key",
    "minecraft:resource_selector",
    "minecraft:template_mirror",
    "minecraft:template_rotation",
    "minecraft:heightmap",
    "minecraft:loot_table",
    "minecraft:loot_predicate",
    "minecraft:loot_modifier",
    "minecraft:dialog",
    "minecraft:uuid",
];

/// One parser id's registry name — the synced table the lookup answers,
/// the built-in order the skip-sync fallback keeps (`None` for an id
/// the server never named — the honest unknown).
pub fn command_argument_type_name(
    id: u32,
    synced: &[String],
) -> Option<&'static str> {
    if let Some(name) = synced.get(id as usize) {
        // the synced table is the server's truth — resolve THROUGH it,
        // folding back into the built-in spelling space when the entry
        // matches one of its names (the extra-payload walk keys on the
        // built-in consts)
        let stripped = name.strip_prefix("minecraft:").unwrap_or(name);
        BUILTIN_COMMAND_ARGUMENT_TYPES
            .iter()
            .copied()
            .find(|b| b.strip_prefix("minecraft:") == Some(stripped))
    } else {
        // the skip-sync path: the built-in order IS the id space
        BUILTIN_COMMAND_ARGUMENT_TYPES.get(id as usize).copied()
    }
}

/// Consume one argument node's parser EXTRA payload — the
/// `serializeToNetwork` template walk each ArgumentTypeInfo speaks
/// (javap-audited, the 13 shapes the jar's own custom infos carry; the
/// SingletonArgumentInfo templates the `return` alone keeps). The
/// UNKNOWN name is the refusal the desync the walk cannot survive
/// answers (the vanilla client's own unknown-id throw — the buffer the
/// following entries share can never realign).
fn skip_argument_extra(name: &str, c: &mut &[u8]) -> Result<()> {
    let stripped = name.strip_prefix("minecraft:").unwrap_or(name);
    match stripped {
        // brigadier:float/double/integer/long — the
        // ArgumentUtils.createNumberFlags byte (bit 1 = min rides,
        // bit 2 = max), the values the flag answers
        "brigadier:float" => {
            let flags = read_u8(c)?;
            if flags & 0x01 != 0 {
                read_f32(c)?;
            }
            if flags & 0x02 != 0 {
                read_f32(c)?;
            }
        }
        "brigadier:double" => {
            let flags = read_u8(c)?;
            if flags & 0x01 != 0 {
                read_f64(c)?;
            }
            if flags & 0x02 != 0 {
                read_f64(c)?;
            }
        }
        "brigadier:integer" => {
            let flags = read_u8(c)?;
            if flags & 0x01 != 0 {
                read_i32(c)?;
            }
            if flags & 0x02 != 0 {
                read_i32(c)?;
            }
        }
        "brigadier:long" => {
            let flags = read_u8(c)?;
            if flags & 0x01 != 0 {
                read_i64(c)?;
            }
            if flags & 0x02 != 0 {
                read_i64(c)?;
            }
        }
        // brigadier:string — the writeEnum(StringType) varint (the
        // SINGLE_WORD/QUOTABLE_PHRASE/GREEDY_STRING ordinals)
        "brigadier:string" => {
            read_varint(c)?;
        }
        // entity / score_holder — the one flag byte (the single /
        // multiple booleans the templates carry)
        "entity" | "score_holder" => {
            read_u8(c)?;
        }
        // time — the writeInt(min) the template's own floor wears
        "time" => {
            read_i32(c)?;
        }
        // the resource family — the writeResourceKey identifier (the
        // registry key the template resolves through)
        "resource_or_tag" | "resource_or_tag_key" | "resource" | "resource_key"
        | "resource_selector" => {
            read_capped_string(c, 32767)?;
        }
        // every other synced name the SingletonArgumentInfo template
        // answers — `serializeToNetwork` is the bare `return` walk
        _ if BUILTIN_COMMAND_ARGUMENT_TYPES.iter().any(|b| {
            b.strip_prefix("minecraft:") == Some(stripped)
        }) => {}
        _ => {
            anyhow::bail!(
                "unknown argument type '{name}' — the extra payload the node walk cannot consume"
            );
        }
    }
    Ok(())
}

/// Parse the Commands packet (clientbound play, `minecraft:commands`
/// id 16) — the server's whole Brigadier graph, the login-time packet
/// `ClientPacketListener.handleCommands` builds the local dispatcher
/// from. THE WIRE (javap of the 26.2 class + the `Entry` record's own
/// write walk): VarInt entry count, then per entry — the flags byte,
/// the children as a VarInt array, the redirect VarInt (the 0x08 bit),
/// the stub (root: nothing; literal: the bare utf; argument: the utf
/// name, the parser registry varint, the suggestion id identifier when
/// the 0x10 bit rides, the parser's own extra payload the
/// [`skip_argument_extra`] walk consumes) — then the root index
/// VarInt. The parser ids resolve through the synced
/// command_argument_type table the caller passes ([`RegistryData`]'s
/// own arrival order; empty = the built-in order answers).
pub fn parse_commands(payload: &[u8], parser_names: &[String]) -> Result<CommandTree> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::COMMANDS,
        "not a commands packet (0x{id:02X})"
    );
    let count = read_varint(&mut c)?;
    anyhow::ensure!(
        count >= 0 && count <= 8192,
        "commands tree carries {count} nodes (past any sane graph)"
    );
    let mut nodes = Vec::with_capacity(count as usize);
    for i in 0..count {
        let flags = read_u8(&mut c)?;
        let kind = match flags & 0x03 {
            0 => CommandNodeKind::Root,
            1 => CommandNodeKind::Literal,
            _ => CommandNodeKind::Argument,
        };
        let child_count = read_varint(&mut c)?;
        anyhow::ensure!(
            child_count >= 0,
            "node {i} carries a negative child count"
        );
        let mut children = Vec::with_capacity(child_count as usize);
        for _ in 0..child_count {
            let child = read_varint(&mut c)?;
            anyhow::ensure!(child >= 0, "node {i} carries a negative child id");
            children.push(child as u32);
        }
        let redirect = if flags & 0x08 != 0 {
            let r = read_varint(&mut c)?;
            anyhow::ensure!(r >= 0, "node {i} redirects to a negative id");
            Some(r as u32)
        } else {
            None
        };
        let (name, parser, suggestion_id) = match kind {
            CommandNodeKind::Root => (String::new(), None, None),
            CommandNodeKind::Literal => (read_capped_string(&mut c, 32767)?, None, None),
            CommandNodeKind::Argument => {
                let name = read_capped_string(&mut c, 32767)?;
                let parser_id = read_varint(&mut c)?;
                anyhow::ensure!(parser_id >= 0, "node {i} names a negative parser id");
                let parser_id = parser_id as u32;
                let suggestion_id = if flags & 0x10 != 0 {
                    Some(read_capped_string(&mut c, 32767)?)
                } else {
                    None
                };
                // the parser's name first (the extra walk keys on it),
                // the resolution the synced table / built-in order
                // answers — an unknown id the honest refusal keeps
                let parser_name = command_argument_type_name(parser_id, parser_names)
                    .with_context(|| {
                        format!(
                            "node {i} ('{name}') names parser id {parser_id} — \
                             no synced entry and past the built-in order"
                        )
                    })?;
                skip_argument_extra(parser_name, &mut c)
                    .with_context(|| format!("node {i} ('{name}') extra payload"))?;
                (name, Some((parser_id, parser_name.to_string())), suggestion_id)
            }
        };
        nodes.push(CommandNode {
            kind,
            executable: flags & 0x04 != 0,
            restricted: flags & 0x20 != 0,
            children,
            redirect,
            name,
            parser,
            suggestion_id,
        });
    }
    let root_index = read_varint(&mut c)?;
    anyhow::ensure!(root_index >= 0, "negative root index");
    anyhow::ensure!(
        c.is_empty(),
        "trailing bytes after the commands tree"
    );
    Ok(CommandTree {
        root_index: root_index as u32,
        nodes,
    })
}

/// The ATTACK wire (the vanilla `MultiPlayerGameMode.attack`'s own
/// echo — the 26.2's dedicated ServerboundAttackPacket, generated id
/// 1, `minecraft:attack`, the generated table's SB_ATTACK). THE WIRE
/// (javap of the real 26.2 class): the record `(int entityId)` — the
/// clinit's composite loads ByteBufCodecs.VAR_INT alone (the
/// single-field StreamCodec the two-Function call speaks), the id the
/// Entity.getId carries. The packet the attack walk sends BEFORE the
/// local prediction (the dump's own order: ensure, SEND, attack,
/// RESET) — the server owns the wire body's health, the client's own
/// prediction never speaks it.
pub fn encode_attack(entity_id: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(6);
    write_varint(&mut p, serverbound::ATTACK);
    write_varint(&mut p, entity_id);
    p
}

/// The INTERACT wire (the vanilla `MultiPlayerGameMode.interact`'s own
/// echo — the 26.2's flat ServerboundInteractPacket record, generated
/// id 26, `minecraft:interact`, the generated table's SB_INTERACT).
/// THE WIRE (javap of the real 26.2 record's clinit composite, the
/// field order the record declares): the entity id as a VarInt, the
/// hand as a VarInt (the writeEnum's ordinal, MAIN_HAND = 0,
/// OFF_HAND = 1), the CURSOR as the `Vec3.LP_STREAM_CODEC` the
/// `net/minecraft/network/LpVec3` class speaks (the hit location minus
/// the entity's own origin — the walk's
/// `hit.getLocation().subtract(entity.getX/Y/Z)` the dump keeps), the
/// `usingSecondaryAction` as ONE BOOL byte (the isShiftKeyDown the
/// sneak key speaks). The walk the dump keeps beside: ensure, the
/// cursor, the SEND, the spectator PASS, `player.interactOn` the local
/// prediction's own return.
pub fn encode_interact(
    entity_id: i32,
    hand: u8,
    cursor: (f64, f64, f64),
    shift: bool,
) -> Vec<u8> {
    let mut p = Vec::with_capacity(10);
    write_varint(&mut p, serverbound::INTERACT);
    write_varint(&mut p, entity_id);
    write_varint(&mut p, hand as i32);
    p.extend_from_slice(&encode_lp_vec3(cursor.0, cursor.1, cursor.2));
    write_bool(&mut p, shift);
    p
}

/// The ride's INPUT RECORD (the 26.2's own `net/minecraft/world/entity/
/// player/Input` — the record ServerboundPlayerInputPacket carries, the
/// `Input.STREAM_CODEC` the packet's clinit composite passes). SEVEN
/// BOOLS: forward, backward, left, right, jump, shift, sprint — the
/// ctor's own parameter order the javap walk keeps. THE WIRE: one BYTE,
/// bit 0 = forward, bit 1 = backward, bit 2 = left, bit 3 = right,
/// bit 4 = jump, bit 5 = shift, bit 6 = sprint (the encode/decode
/// walks' own 1/2/4/8/16/32/64 constants — the ior fold the encode
/// speaks, the iand probes the decode answers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InputFlags {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub shift: bool,
    pub sprint: bool,
}

impl InputFlags {
    /// The record's ONE-BYTE wire form (the Input$1 codec's own ior
    /// fold: each boolean ORs its ordinal's power of two, the byte the
    /// writeByte/iand pair the decompile keeps).
    pub fn to_byte(self) -> u8 {
        let mut b = 0u8;
        if self.forward {
            b |= 1;
        }
        if self.backward {
            b |= 2;
        }
        if self.left {
            b |= 4;
        }
        if self.right {
            b |= 8;
        }
        if self.jump {
            b |= 16;
        }
        if self.shift {
            b |= 32;
        }
        if self.sprint {
            b |= 64;
        }
        b
    }

    /// The byte's own inverse (the decode's iand probes: bit i set =
    /// the boolean true).
    pub fn from_byte(b: u8) -> InputFlags {
        InputFlags {
            forward: b & 1 != 0,
            backward: b & 2 != 0,
            left: b & 4 != 0,
            right: b & 8 != 0,
            jump: b & 16 != 0,
            shift: b & 32 != 0,
            sprint: b & 64 != 0,
        }
    }
}

/// The PLAYER INPUT wire (the vanilla `LocalPlayer.tick`'s own send —
/// `lastSentInput != input.keyPresses()` sends one ServerboundPlayer-
/// InputPacket, the record rides the Input.STREAM_CODEC, the generated
/// id 43, `minecraft:player_input`): the packet id then the record's
/// ONE BYTE (the seven booleans' bit law [`InputFlags::to_byte`]).
pub fn encode_player_input(flags: InputFlags) -> Vec<u8> {
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, serverbound::PLAYER_INPUT);
    p.push(flags.to_byte());
    p
}

/// The PADDLE BOAT wire (the vanilla `AbstractBoat.tick`'s own send —
/// the client's authoritative boat speaks ONE ServerboundPaddleBoat-
/// Packet EVERY tick while it carries the local player, paddling or
/// not: `level.sendPacketToServer(new ServerboundPaddleBoatPacket(
/// getPaddleState(0), getPaddleState(1)))` — the raw_disasm walk's
/// own 163-188 lane, NO change gate). The generated id 35,
/// `minecraft:paddle_boat`; the body the class's own write walk
/// keeps: `writeBoolean(left)` then `writeBoolean(right)` — TWO
/// bytes, the left paddle first (FriendlyByteBuf.readBoolean's own
/// one-byte truth).
pub fn encode_paddle_boat(left: bool, right: bool) -> Vec<u8> {
    let mut p = Vec::with_capacity(3);
    write_varint(&mut p, serverbound::PADDLE_BOAT);
    p.push(left as u8);
    p.push(right as u8);
    p
}

/// THE LOW-PRECISION VEC3 (the 26.2's own `net/minecraft/network/LpVec3`
/// — the codec the Interact record's `location` field rides, the
/// `Vec3.LP_STREAM_CODEC` the clinit's composite passes). THE WIRE
/// (javap of the real class's write/read/sanitize/pack/unpack/
/// hasContinuationBit walks + the constant pool's own values):
///
/// * sanitize: NaN → 0.0, else clamp to ±17179869183.0 (the 2^34−1 the
///   clinit's #96/#98 doubles speak);
/// * max = absMax(absMax(x, y), z) = max(|x|, |y|, |z|) (Mth.absMax's
///   own abs-then-max walk — always positive, the threshold compare
///   safe);
/// * THE ZERO FORM: max < 3.051944088384301e-05 (the 2^-15 the #66
///   double speaks) → the single 0x00 byte (the read's `if (b0 == 0)
///   return Vec3.ZERO`);
/// * else the SCALE `l = ceilLong(max)` = (long)Math.ceil(max), the
///   three axes' 15-bit fields the pack walk produces (pack =
///   round((d·0.5 + 0.5)·32766.0) — the [-1,1] normalize the x/l
///   division speaks, 0.0 the exact midpoint 16383 keeps), packed
///   `word = head | fx << 3 | fy << 18 | fz << 33` where the HEAD is
///   l's low TWO bits (bit 2 = the continuation flag the
///   hasContinuationBit(i) = (i & 4) == 4 speaks, set iff `(l & 3) != l`
///   — the #76/#78 longs' own 3 and 4);
/// * the 48-bit word rides SIX bytes (writeByte word, writeByte
///   word >> 8, writeInt word >> 16 — the BE int the read's
///   readUnsignedInt << 16 mirrors); the scale's bits 2..33 follow as
///   ONE VarInt((int)(l >> 2)) when the flag lives;
/// * unpack reverses: min(field & 0x7FFF, 32766.0)·2.0/32766.0 − 1.0
///   (the 32767 sentinel the Math.min clamps — the #113/#118/#125
///   constants), × the scale the read reconstructs: `(byte0 & 3) |
///   ((VarInt & 0xFFFFFFFF) << 2)` — the 0xFFFFFFFF mask (#37) the
///   signed VarInt's own roundtrip keeps (the 2^34−1 bound's l >> 2
///   overflows the i32 the mask un-folds).
pub fn encode_lp_vec3(x: f64, y: f64, z: f64) -> Vec<u8> {
    let sanitize = |d: f64| {
        if d.is_nan() {
            0.0
        } else {
            d.clamp(-17179869183.0, 17179869183.0)
        }
    };
    let x = sanitize(x);
    let y = sanitize(y);
    let z = sanitize(z);
    let max = x.abs().max(y.abs()).max(z.abs());
    if max < 3.051944088384301e-05 {
        return vec![0];
    }
    let pack = |d: f64| ((d * 0.5 + 0.5) * 32766.0).round() as i64;
    let l = max.ceil() as i64;
    let cont = (l & 3) != l;
    let head = if cont { (l & 3) | 4 } else { l };
    let word = head | pack(x / l as f64) << 3 | pack(y / l as f64) << 18 | pack(z / l as f64) << 33;
    let mut p = Vec::with_capacity(7);
    p.push((word & 0xFF) as u8);
    p.push(((word >> 8) & 0xFF) as u8);
    p.extend_from_slice(&((word >> 16) as i32).to_be_bytes());
    if cont {
        write_varint(&mut p, (l >> 2) as i32);
    }
    p
}

/// The low-precision Vec3's read side — the exact inverse the LpVec3
/// read walk speaks (the zero form's single byte, the 6-byte word, the
/// continuation scale's VarInt through the 0xFFFFFFFF mask). Returns
/// the vector and the bytes consumed (the tail's own length the
/// variable form speaks).
pub fn parse_lp_vec3(payload: &[u8]) -> Result<((f64, f64, f64), usize)> {
    let mut c: &[u8] = payload;
    let b0 = read_u8(&mut c)?;
    if b0 == 0 {
        return Ok(((0.0, 0.0, 0.0), 1));
    }
    let b1 = read_u8(&mut c)?;
    let hi = read_u32(&mut c)? as i64;
    let word = (hi << 16) | ((b1 as i64) << 8) | (b0 as i64);
    let mut scale = (b0 & 3) as i64;
    if b0 & 4 != 0 {
        let tail = read_varint(&mut c)?;
        scale |= (tail as i64 & 0xFFFF_FFFF) << 2;
    }
    let unpack =
        |j: i64| (j & 0x7FFF).min(32766) as f64 * 2.0 / 32766.0 - 1.0;
    Ok((
        (
            unpack(word >> 3) * scale as f64,
            unpack(word >> 18) * scale as f64,
            unpack(word >> 33) * scale as f64,
        ),
        payload.len() - c.len(),
    ))
}

/// Client Command's wire shape (generated id 12 —
/// `minecraft:client_command`). javap of the real 26.2
/// `ServerboundClientCommandPacket`'s STREAM_CODEC write: the action's
/// VarInt enum ordinal — `PERFORM_RESPAWN` 0, `REQUEST_STATS` 1,
/// `REQUEST_GAMERULE_VALUES` 2 (the `$Action` clinit's own
/// registration order).
pub fn encode_client_command(action: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, serverbound::CLIENT_COMMAND);
    write_varint(&mut p, action);
    p
}

/// The abilities wire (generated id 40 —
/// `minecraft:player_abilities`). javap of the real 26.2
/// `ServerboundPlayerAbilitiesPacket`: the STREAM_CODEC is
/// `codec(write, <init>(FriendlyByteBuf))` (the clinit's two
/// invokedynamic pair), the write walk `write(FriendlyByteBuf)` pcs
/// 0-20 verbatim — local b0 = 0, `if (isFlying) b0 |= 2`
/// (FLAG_FLYING, the cp's `#67 = Int 2` the constant banks), ONE
/// `writeByte(b0)`. The OTHER flag bits — invulnerable 1, canFly 4,
/// mayBuild 8, instabuild 16 — the SERVER-side Abilities carries;
/// the CLIENT ctor `<init>(Abilities)` banks ONLY `isFlying`
/// (pcs 4-9: the single getfield putfield pair), so the clientbound
/// packet the flying verdict alone speaks. The send sites —
/// `LocalPlayer.onUpdateAbilities()` (the ONLY send walk the class
/// keeps: `connection.send(new ServerboundPlayerAbilitiesPacket(
/// getAbilities()))` unconditional) — ride the flying CHANGED ticks
/// the aiStep arms earn (the spectator sync pc 529, the jump toggle
/// pc 635, the landing retire pc 1133) and the inbound
/// handlePlayerAbilities echo.
pub fn encode_player_abilities(flying: bool) -> Vec<u8> {
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, serverbound::PLAYER_ABILITIES);
    let mut b0 = 0u8;
    if flying {
        b0 |= 2; // FLAG_FLYING — the write walk's `ior 2` the byte keeps
    }
    p.push(b0);
    p
}

/// The game-mode wire (generated id 5 —
/// `minecraft:change_game_mode`). javap of the real 26.2
/// `ServerboundChangeGameModePacket`: the STREAM_CODEC is
/// `composite(GameType.STREAM_CODEC, ::mode, ::new)` (the clinit's
/// getstatic composite walk — the getter the `mode()` accessor, the
/// from the `<init>(GameType)` newInvokeSpecial the bootstrap pair
/// resolves), and `GameType.STREAM_CODEC` is
/// `ByteBufCodecs.idMapper(BY_ID, id)` — the VarInt id codec the
/// `ByIdMap.continuous(values(), ZERO)` fold keeps (an unknown id the
/// values()[0] fallback speaks). The ids — the clinit's
/// `new GameType(String, ordinal, id, String)` putstatic pairs
/// verbatim: SURVIVAL 0, CREATIVE 1, ADVENTURE 2, SPECTATOR 3.
/// The send site — `GameModeSwitcherScreen.switchToHoveredGameMode(
/// Minecraft, GameModeIcon)` pcs 49-60: the F4 apply's
/// `connection.send(new ServerboundChangeGameModePacket(icon.mode))`
/// — the mode the SERVER's belief owns (the client's local mode the
/// serverbound echo never predicts).
pub fn encode_change_game_mode(mode_id: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, serverbound::CHANGE_GAME_MODE);
    write_varint(&mut p, mode_id);
    p
}

/// The movement wire — the four ServerboundMovePlayer packets (generated
/// ids 30..=33: `minecraft:move_player_pos` 30, `move_player_pos_rot` 31,
/// `move_player_rot` 32, `move_player_status_only` 33). THE WIRE (javap
/// of the real 26.2 class, the STREAM_CODEC reads): the three position
/// doubles BE when the packet carries position (`hasPos`), the two
/// rotation floats BE when it carries rotation (`hasRot`), then the
/// STATUS BYTE — `packFlags(onGround, horizontalCollision)` verbatim:
/// bit 0 = on ground, bit 1 = horizontal collision (the unpack
/// `iand 1` / `iand 2` laws). The dispatch law rides
/// `LocalPlayer.sendPosition` (the body's bookkeeping — the caller
/// speaks WHICH packet this tick named through that law).
pub fn encode_move_player_pos_rot(
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
    horizontal_collision: bool,
) -> Vec<u8> {
    let mut p = Vec::with_capacity(34);
    write_varint(&mut p, serverbound::MOVE_PLAYER_POS_ROT);
    p.extend_from_slice(&x.to_be_bytes());
    p.extend_from_slice(&y.to_be_bytes());
    p.extend_from_slice(&z.to_be_bytes());
    p.extend_from_slice(&yaw.to_be_bytes());
    p.extend_from_slice(&pitch.to_be_bytes());
    p.push(move_flags(on_ground, horizontal_collision));
    p
}

pub fn encode_move_player_pos(x: f64, y: f64, z: f64, on_ground: bool, horizontal_collision: bool) -> Vec<u8> {
    let mut p = Vec::with_capacity(26);
    write_varint(&mut p, serverbound::MOVE_PLAYER_POS);
    p.extend_from_slice(&x.to_be_bytes());
    p.extend_from_slice(&y.to_be_bytes());
    p.extend_from_slice(&z.to_be_bytes());
    p.push(move_flags(on_ground, horizontal_collision));
    p
}

pub fn encode_move_player_rot(yaw: f32, pitch: f32, on_ground: bool, horizontal_collision: bool) -> Vec<u8> {
    let mut p = Vec::with_capacity(10);
    write_varint(&mut p, serverbound::MOVE_PLAYER_ROT);
    p.extend_from_slice(&yaw.to_be_bytes());
    p.extend_from_slice(&pitch.to_be_bytes());
    p.push(move_flags(on_ground, horizontal_collision));
    p
}

pub fn encode_move_player_status_only(on_ground: bool, horizontal_collision: bool) -> Vec<u8> {
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, serverbound::MOVE_PLAYER_STATUS_ONLY);
    p.push(move_flags(on_ground, horizontal_collision));
    p
}

/// The status byte — `ServerboundMovePlayerPacket.packFlags` verbatim:
/// `(onGround ? 1 : 0) | (horizontalCollision ? 2 : 0)`.
fn move_flags(on_ground: bool, horizontal_collision: bool) -> u8 {
    (if on_ground { 1 } else { 0 }) | (if horizontal_collision { 2 } else { 0 })
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

/// Change Difficulty (clientbound play, generated id 10): the server's
/// own level-data word — the danger dial the host set (or a remote
/// admin's `/difficulty` turn) plus the LOCKED bit the difficulty lock
/// speaks. javap of the 26.2 `ClientboundChangeDifficultyPacket` the
/// record keeps: `Difficulty difficulty` + `boolean locked` (the
/// record's own component pair, the `difficulty;locked` header the
/// classfile banks), the STREAM_CODEC the clinit's composite walk
/// banks — `Difficulty.STREAM_CODEC` (the `ByteBufCodecs.idMapper`
/// family: the variant's id a VARINT, 0 Peaceful..3 Hard) +
/// `ByteBufCodecs.BOOL` (the byte the `readBoolean` walk reads:
/// nonzero true). The client's own handler the
/// `ClientPacketListener.handleChangeDifficulty` walk keeps — the
/// `levelData.setDifficulty(...)` + `levelData.setDifficultyLocked(...)`
/// pair the store speaks, the open screen's `HasDifficultyReaction
/// .onDifficultyChanged()` re-sync the tail wears.
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeDifficultyWord {
    /// The dial's own id (the `Difficulty.STREAM_CODEC` idMapper's
    /// VARINT — 0 Peaceful, 1 Easy, 2 Normal, 3 Hard). The strict
    /// 0..=3 range the parse refuses past (the byIdOrThrow family's
    /// own throw law — an unknown id the packet ERROR keeps).
    pub difficulty: u8,
    /// The LOCKED bit (the `ByteBufCodecs.BOOL` byte — `!= 0` true, the
    /// readBoolean law verbatim): the locked world the difficulty
    /// change the vanilla client's own buttons never wear.
    pub locked: bool,
}

/// The Change Difficulty packet's own parse (javap the 26.2
/// `ClientboundChangeDifficultyPacket` STREAM_CODEC composite walk —
/// the id VARINT the idMapper reads, the BOOL byte the locked bit
/// keeps): the packet id the header varint banks, the two-field body
/// the record's component order wears.
pub fn parse_change_difficulty(payload: &[u8]) -> Result<ChangeDifficultyWord> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::CHANGE_DIFFICULTY,
        "not a change difficulty packet (0x{id:02X})"
    );
    let difficulty = read_varint(&mut c)?;
    anyhow::ensure!(difficulty <= 3, "unknown difficulty id {difficulty}");
    anyhow::ensure!(!c.is_empty(), "change difficulty truncated (no locked byte)");
    let locked = read_u8(&mut c)?;
    Ok(ChangeDifficultyWord { difficulty: difficulty as u8, locked: locked != 0 })
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

/// Set Health (clientbound play, generated id 104 —
/// `minecraft:set_health`). javap of the real 26.2
/// `ClientboundSetHealthPacket`'s STREAM_CODEC read: a BE float, a
/// VarInt, a BE float — `health`, `food`, `saturation` in that order.
/// The player's own vitals lane (the vanilla handleSetHealth's
/// consumers: the entity-data health the HUD's hearts speak, the food
/// the hunger row speaks, the saturation the food system folds).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SetHealth {
    /// The health the server speaks — the entity data's float.
    pub health: f32,
    /// The food level (the hunger's 0..=20 int).
    pub food: i32,
    /// The hidden saturation float.
    pub saturation: f32,
}

/// Set Health's wire walk (see [`SetHealth`] — the float/VarInt/float
/// triple, the generated-id guard).
pub fn parse_set_health(payload: &[u8]) -> Result<SetHealth> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::SET_HEALTH, "not a set health packet (0x{id:02X})");
    let health = read_f32(&mut c)?;
    let food = read_varint(&mut c)?;
    let saturation = read_f32(&mut c)?;
    Ok(SetHealth { health, food, saturation })
}

/// Hurt Animation (clientbound play, generated id 42 —
/// `minecraft:hurt_animation`). javap of the real 26.2
/// `ClientboundHurtAnimationPacket`'s STREAM_CODEC read: a VarInt
/// entity id, a BE float — the tilt yaw this 26.x WIDENED from the
/// old byte (the `readVarInt, readFloat` pair feeding the
/// `(int, float)` constructor). The hurt tilt's own channel (the
/// vanilla handleHurtAnimation calls `animateHurt(yaw)` on the named
/// entity — the flash-and-lean the damage speaks).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HurtAnimation {
    /// The hurt entity's id (the level's entity registry key).
    pub entity_id: i32,
    /// The tilt yaw in vanilla wire degrees.
    pub yaw: f32,
}

/// Hurt Animation's wire walk (see [`HurtAnimation`] — the VarInt id +
/// the BE float yaw, the generated-id guard).
pub fn parse_hurt_animation(payload: &[u8]) -> Result<HurtAnimation> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::HURT_ANIMATION,
        "not a hurt animation packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let yaw = read_f32(&mut c)?;
    Ok(HurtAnimation { entity_id, yaw })
}

/// Entity Sound (clientbound play, generated id 116 —
/// `minecraft:sound_entity`). javap of the real 26.2
/// `ClientboundEntitySoundPacket`'s STREAM_CODEC read: a VarInt entity
/// id, the SoundEvent holder (the SAME tagged union the consumable's
/// sound lane reads — [`ConsumableSound`]; one codec, N call sites),
/// the BE float volume, the BE float pitch, the i64 seed. The mob
/// voices' own channel — the SERVER plays the ambient/hurt/death
/// sounds (`Mob.baseTick`'s playAmbientSound, `LivingEntity.hurt`'s
/// playHurtSound, `die`'s tail) and the named entity's client copy
/// speaks them (`ClientLevel.handleEntitySound` → the entity lookup
/// the voice rides).
#[derive(Debug, Clone, PartialEq)]
pub struct EntitySoundPacket {
    /// The sounding entity's id (the level's entity registry key).
    pub entity_id: i32,
    /// The sound event: the registry id (the synced sound_event
    /// table's arrival order) or the direct lane (location + range).
    pub sound: ConsumableSound,
    /// The volume, verbatim (the client's playSound carries it).
    pub volume: f32,
    /// The pitch, verbatim.
    pub pitch: f32,
    /// The seeded random's seed (the file-variant pick's own seed —
    /// the engine's weighted roll keeps its own draw).
    pub seed: i64,
}

/// Entity Sound's wire walk (see [`EntitySoundPacket`] — the VarInt id
/// + the tagged sound holder + the two floats + the seed, the
/// generated-id guard).
pub fn parse_entity_sound(payload: &[u8]) -> Result<EntitySoundPacket> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SOUND_ENTITY,
        "not an entity sound packet (0x{id:02X})"
    );
    let entity_id = read_varint(&mut c)?;
    let sound = read_consumable_sound(&mut c)?;
    let volume = read_f32(&mut c)?;
    let pitch = read_f32(&mut c)?;
    let seed = read_i64(&mut c)?;
    Ok(EntitySoundPacket { entity_id, sound, volume, pitch, seed })
}

/// Sound (clientbound play, generated id 117 — `minecraft:sound`).
/// javap of the real 26.2 `ClientboundSoundPacket`'s STREAM_CODEC read
/// (the RegistryFriendlyByteBuf ctor): the SoundEvent holder (the SAME
/// tagged union the entity voice reads — [`ConsumableSound`]), the
/// SoundSource enum (the VarInt ordinal — the declaration order the
/// jar's constant pool speaks: MASTER, MUSIC, RECORD, WEATHER, BLOCK,
/// HOSTILE, NEUTRAL, PLAYER, AMBIENT, VOICE), the THREE BE ints (the
/// position at 1/8 block resolution — the ctor's `* 8.0` fold the
/// getter's /8.0 law mirrors), the BE float volume, the BE float
/// pitch, the i64 seed. The GLOBAL sound channel — the sounds the
/// SERVER plays at a world spot (the door the block-use opened on the
/// other player's screen, the chests, the dispensers, the weather)
/// the entity voices' position-less sibling answers.
#[derive(Debug, Clone, PartialEq)]
pub struct WorldSoundPacket {
    /// The sound source's enum ordinal (the SoundSource declaration
    /// order the jar keeps: 0 master .. 9 voice).
    pub source: u8,
    /// The sound event: the registry id (the synced sound_event
    /// table's arrival order) or the direct lane (location + range).
    pub sound: ConsumableSound,
    /// The world spot (the wire's 1/8-block ints the /8 law reads).
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// The volume, verbatim (the client's playSound carries it — the
    /// 16-and-over the silence law keeps downstream).
    pub volume: f32,
    /// The pitch, verbatim.
    pub pitch: f32,
    /// The seeded random's seed (the file-variant pick's own seed).
    pub seed: i64,
}

/// Sound's wire walk (see [`WorldSoundPacket`] — the holder + the
/// source enum + the 1/8-grid ints + the two floats + the seed, the
/// generated-id guard).
pub fn parse_sound(payload: &[u8]) -> Result<WorldSoundPacket> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SOUND,
        "not a sound packet (0x{id:02X})"
    );
    let sound = read_consumable_sound(&mut c)?;
    let source = read_varint(&mut c)?;
    anyhow::ensure!(
        (0..=9).contains(&source),
        "sound source ordinal {source} outside the enum"
    );
    let x = read_i32(&mut c)? as f64 / 8.0;
    let y = read_i32(&mut c)? as f64 / 8.0;
    let z = read_i32(&mut c)? as f64 / 8.0;
    let volume = read_f32(&mut c)?;
    let pitch = read_f32(&mut c)?;
    let seed = read_i64(&mut c)?;
    Ok(WorldSoundPacket { source: source as u8, sound, x, y, z, volume, pitch, seed })
}

// ===== THE SCOREBOARD WIRE TRIO =====
// Set Objective (106) / Set Display Objective (98) / Set Score (110) +
// the sibling Reset Score (79) — the sidebar lane the live server
// speaks. Every walk below was raw-disassembled from the 26.2 jar
// (the four packets' STREAM_CODEC composites + the four
// ClientPacketListener handlers).

/// Set Objective's method byte (the ctor's own dispatch): 0 = add (the
/// payload branch), 1 = remove (no payload), 2 = update display (the
/// SAME payload branch the add reads). Any other byte walks the
/// payload-less defaults branch on the wire and is a no-op in the
/// handler.
pub const OBJECTIVE_METHOD_ADD: u8 = 0;
pub const OBJECTIVE_METHOD_REMOVE: u8 = 1;
pub const OBJECTIVE_METHOD_UPDATE: u8 = 2;

/// `ObjectiveCriteria$RenderType`'s own declaration order (the
/// readEnum VarInt ordinal): INTEGER = 0, HEARTS = 1. The readEnum
/// guard refuses everything else (the values-array bounds throw).
pub const RENDER_TYPE_INTEGER: u8 = 0;
pub const RENDER_TYPE_HEARTS: u8 = 1;

/// The DisplaySlot id the SIDEBAR renders from (the
/// `extractScoreboardSidebar` walk's own slot; 0 = list, 2 =
/// below-name, 3..18 the team sidebars).
pub const DISPLAY_SLOT_SIDEBAR: u8 = 1;

/// The DisplaySlot id the BELOW-NAME lane renders from (the
/// `Entity.belowNameDisplay` walk's own slot — the score the
/// nametag's own display draws under the name within ten blocks).
pub const DISPLAY_SLOT_BELOW_NAME: u8 = 2;

/// One Set Objective packet (clientbound play, generated id 106).
/// The wire walk (raw disasm of `ClientboundSetObjectivePacket.<init>(
/// RegistryFriendlyByteBuf)`): `readUtf()` objective name, `readByte()`
/// method, then — ONLY for method 0 or 2 (the ctor's own `ifeq` +
/// `iconst_2` dispatch) — the display Component (ONE bare network NBT
/// tag, the TRUSTED_STREAM_CODEC machinery), the render type
/// (`readEnum` VarInt ordinal), the optional number format. Method 1
/// (and every other byte) reads NOTHING further — the ctor's own
/// defaults branch (EMPTY display, INTEGER, empty format).
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectiveUpdate {
    /// The objective's registry name (the server's own key).
    pub objective: String,
    /// The method byte, verbatim (see the METHOD_* constants).
    pub method: u8,
    /// The display component's flattened TEXT (the sidebar title
    /// renders) — `None` for the payload-less methods.
    pub display: Option<String>,
    /// The render type's VarInt ordinal, guarded to 0..=1.
    pub render_type: u8,
}

/// One Set Display Objective packet (clientbound play, generated id
/// 98). The wire walk (raw disasm): `readById(DisplaySlot.BY_ID)` — a
/// VarInt slot id (the BY_ID map's own OutOfBoundsStrategy.ZERO folds
/// an out-of-range id onto slot 0; the id rides here VERBATIM, the
/// house's ordinal law) — then `readUtf()` the objective name, the
/// EMPTY string the handler's getObjective(null-objective) walk
/// reads as "clear the slot".
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayObjectiveUpdate {
    /// The display slot id, verbatim (0 list, 1 sidebar, 2
    /// below-name, 3..18 the team sidebars).
    pub slot: u8,
    /// The objective's name; EMPTY = clear the slot.
    pub objective: String,
}

/// One Set Score packet (clientbound play, generated id 110). The
/// wire walk (raw disasm of the record's STREAM_CODEC composite, in
/// order): STRING_UTF8 owner, STRING_UTF8 objective name, VAR_INT
/// score, TRUSTED_OPTIONAL_STREAM_CODEC display component (a boolean
/// flag, then ONE bare network NBT tag), NumberFormatTypes's
/// OPTIONAL_STREAM_CODEC number format (a boolean flag, then the
/// dispatched payload).
#[derive(Debug, Clone, PartialEq)]
pub struct ScoreUpdate {
    /// The score holder (the player's scoreboard name — the "#hidden"
    /// convention the sidebar's own filter keeps lives here).
    pub owner: String,
    /// The objective the score rides (the handler gates on it: an
    /// unknown objective is a logged WARN + drop, no state change).
    pub objective: String,
    /// The value the sidebar renders.
    pub score: i32,
    /// The display component's flattened text — the sidebar row's
    /// name the owner's own string overrides when the server named
    /// one (`None` = render the owner).
    pub display: Option<String>,
}

/// One Reset Score packet (clientbound play, generated id 79). The
/// wire walk (raw disasm of the `<init>(FriendlyByteBuf)` decode):
/// `readUtf()` owner, then `readNullable(STRING_UTF8)` objective — a
/// boolean flag, then the name. A `None` objective resets EVERY
/// score the holder owns (the resetAllPlayerScore walk).
#[derive(Debug, Clone, PartialEq)]
pub struct ResetScore {
    /// The score holder whose rows die.
    pub owner: String,
    /// The objective's name — `None` = every objective's row.
    pub objective: Option<String>,
}

/// The component tag's own TEXT (the flatten the sidebar renders):
/// a bare TAG_String IS the text; a structured compound speaks its
/// "text" field followed by every "extra" child (the vanilla walk's
/// own order); anything else renders empty — the honest
/// never-a-guess. (The structured form's colors/styles the model
/// never sees — the sidebar is the monochrome text's lane.)
pub fn flatten_component_text(tag: &Nbt) -> String {
    match tag {
        Nbt::String(s) => s.clone(),
        Nbt::Compound(_) => {
            let mut out = String::new();
            if let Some(Nbt::String(s)) = tag.get("text") {
                out.push_str(s);
            }
            if let Some(Nbt::List(children)) = tag.get("extra") {
                for child in children {
                    out.push_str(&flatten_component_text(child));
                }
            }
            out
        }
        _ => String::new(),
    }
}

/// THE CHAT COMPONENT'S OWN COLOR PALETTE (the vanilla `ChatFormatting`
/// enum's own sixteen — the `§0`..`§f` legacy codes the font renderer's
/// `StringDecomposer` walk speaks, the exact hex the enum's constants
/// carry). The walk's emission alphabet: a color the wire names the
/// matching code char wears.
const CHAT_FORMAT_CODES: [(&str, char); 16] = [
    ("black", '0'),
    ("dark_blue", '1'),
    ("dark_green", '2'),
    ("dark_aqua", '3'),
    ("dark_red", '4'),
    ("dark_purple", '5'),
    ("gold", '6'),
    ("gray", '7'),
    ("dark_gray", '8'),
    ("blue", '9'),
    ("green", 'a'),
    ("aqua", 'b'),
    ("red", 'c'),
    ("light_purple", 'd'),
    ("yellow", 'e'),
    ("white", 'f'),
];

/// The component's own color the walk resolves: the named code the
/// sixteen `ChatFormatting` constants speak, the true-hex RGB the
/// legacy `§x` alphabet carries, the "reset" the base the legacy law
/// keeps.
#[derive(Clone, Copy, PartialEq)]
enum ChatColor {
    /// The named palette code (`§0`..`§f` — the draw lane's own
    /// `COLOR_CODES` lookup the char carries).
    Code(char),
    /// THE TRUE HEX (the `#RRGGBB` the 16-code palette cannot name —
    /// the legacy `§x§r§r§g§g§b§b` form the real servers' plugin
    /// strings speak, the `CraftChatMessage` convention the ecosystem
    /// de-facto walks; vanilla's own `StringDecomposer` the raw §x
    /// never parses — the house draw lane the exact-RGB consumption
    /// the SAME alphabet answers).
    Rgb(u32),
}

/// The `#rrggbb` form the parse accepts: exactly six hex digits after
/// the hash, the case the ascii fold answers; any other string the
/// `None` the named/reset lanes keep walking.
fn hex_color_value(color: &str) -> Option<u32> {
    let hex = color.strip_prefix('#')?;
    if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    u32::from_str_radix(hex, 16).ok()
}

/// The component's own color the wire names: `Some(explicit)` the
/// field carries (a named code, the true-hex rgb, or the "reset" the
/// legacy law keeps — the reset the `None` the base returns to),
/// `None` the field names nothing the walk recognizes (the inherited
/// color rides — the vanilla Style resolution's own omission law).
fn chat_format_color(color: &str) -> Option<Option<ChatColor>> {
    if let Some(rgb) = hex_color_value(color) {
        return Some(Some(ChatColor::Rgb(rgb)));
    }
    let lower = color.to_ascii_lowercase();
    if lower == "reset" {
        return Some(None);
    }
    CHAT_FORMAT_CODES
        .iter()
        .find(|(name, _)| *name == lower)
        .map(|(_, code)| Some(ChatColor::Code(*code)))
}

/// THE LEGACY HEX EMISSION (the `CraftChatMessage` walk's own form —
/// the `§x` prefix the SIX (§, hexdigit) pairs follow, one pair per
/// nibble high-to-low R R G G B B, the lowercase digits
/// `Integer.toHexString` speaks).
fn push_legacy_hex(rgb: u32, out: &mut String) {
    const HEX: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
    out.push('\u{a7}');
    out.push('x');
    for shift in [20u32, 16, 12, 8, 4, 0] {
        out.push('\u{a7}');
        out.push(HEX[((rgb >> shift) & 0xF) as usize]);
    }
}

/// THE RESOLVED STYLE THE WALK CARRIES (the color the §-code speaks —
/// the named code OR the true-hex rgb the discriminated pair keeps,
/// at most one Some the invariant banks), the flags the effect codes
/// answer — the inheritance the component chain walks, the obfuscated
/// the chain keeps AND the emission now carries: the draw lane's
/// random-glyph walk the §k answer keeps — the pick the 26.2
/// getRandomGlyph bytecode speaks).
#[derive(Clone, Copy, PartialEq, Default)]
struct ChatStyleState {
    color: Option<char>,
    /// THE TRUE HEX (the `§x` walk's own RGB — the `Some` the named
    /// code's palette overrides, the paired `color` the None keeps).
    rgb: Option<u32>,
    bold: bool,
    italic: bool,
    underline: bool,
    strike: bool,
    obfuscated: bool,
}

/// The flat emission's own state machine (the draw walk's OWN
/// consumption law the emission models: the §-code sequence the draw
/// resolves to the target state speaks): the flag-loss the §r answers
/// (the legacy alphabet's only eraser — no per-flag off code exists),
/// the color switch the fresh-style law wears (the flags it kills the
/// re-emission restores), the effect codes the accumulate law speaks.
fn emit_chat_state(target: ChatStyleState, cur: &mut ChatStyleState, out: &mut String) {
    // THE FLAG-LOSS RESET (the flags cur wears the target lacks —
    // the §r the only eraser the alphabet carries)
    if (cur.bold && !target.bold)
        || (cur.italic && !target.italic)
        || (cur.underline && !target.underline)
        || (cur.strike && !target.strike)
        || (cur.obfuscated && !target.obfuscated)
    {
        out.push('§');
        out.push('r');
        *cur = ChatStyleState::default();
    }
    // THE COLOR'S OWN SWITCH (the discriminated pair the equality
    // walks — the named code and the true-hex rgb never co-carry)
    if target.color != cur.color || target.rgb != cur.rgb {
        match target.rgb {
            Some(rgb) => push_legacy_hex(rgb, out),
            None => {
                out.push('§');
                match target.color {
                    Some(code) => out.push(code),
                    None => out.push('r'),
                }
            }
        }
        // THE COLOR'S OWN CLEAR (javap Style.applyLegacyFormat — the
        // fresh style the color arm builds, the five Boolean.valueOf(0)
        // the flags wear)
        cur.color = target.color;
        cur.rgb = target.rgb;
        cur.bold = false;
        cur.italic = false;
        cur.underline = false;
        cur.strike = false;
        cur.obfuscated = false;
    }
    if target.obfuscated && !cur.obfuscated {
        out.push_str("\u{a7}k");
        cur.obfuscated = true;
    }
    if target.bold && !cur.bold {
        out.push_str("\u{a7}l");
        cur.bold = true;
    }
    if target.strike && !cur.strike {
        out.push_str("\u{a7}m");
        cur.strike = true;
    }
    if target.underline && !cur.underline {
        out.push_str("\u{a7}n");
        cur.underline = true;
    }
    if target.italic && !cur.italic {
        out.push_str("\u{a7}o");
        cur.italic = true;
    }
    *cur = target;
}

/// The colored flatten's own recursion: `inherit` the style the
/// component's PARENT chain carries (the vanilla Style resolution's
/// own direction — a child the field omits wears the parent's color,
/// the SIBLINGS the COMMON parent wears, never each other), `cur` the
/// state the flat emission currently speaks (the draw walk's linear
/// state — a code the text only emits when the resolved state
/// differs).
fn flatten_component_colored_walk(
    tag: &Nbt,
    inherit: ChatStyleState,
    out: &mut String,
    cur: &mut ChatStyleState,
) {
    // The component's OWN color the field carries: `Some(explicit)` a
    // named code / the true-hex rgb / the "reset" speaks, `None` the
    // field the component omits (or the unknown string) — the
    // inherited color rides.
    let own_color = match tag.get("color").and_then(Nbt::as_str) {
        Some(c) => chat_format_color(c),
        None => None,
    };
    // THE DISCRIMINATED PAIR (the explicit field the own walk names,
    // the omission the parent's pair inherits — the two fields at
    // most one Some the invariant keeps)
    let resolved_pair = match own_color {
        Some(Some(ChatColor::Code(c))) => (Some(c), None),
        Some(Some(ChatColor::Rgb(v))) => (None, Some(v)),
        Some(None) => (None, None),
        None => (inherit.color, inherit.rgb),
    };
    // The component's OWN FLAG the field carries (the explicit true/
    // false the Byte/String booleans speak, the omission the parent's
    // flag inherits).
    let flag = |key: &str, inherited: bool| -> bool {
        match tag.get(key) {
            Some(Nbt::Byte(b)) => *b != 0,
            Some(Nbt::String(s)) => s == "true",
            _ => inherited,
        }
    };
    // THE SUBTREE'S OWN RESOLVED STYLE (the vanilla chain fold: the
    // explicit fields override, the omissions inherit)
    let resolved = ChatStyleState {
        color: resolved_pair.0,
        rgb: resolved_pair.1,
        bold: flag("bold", inherit.bold),
        italic: flag("italic", inherit.italic),
        underline: flag("underlined", inherit.underline),
        strike: flag("strikethrough", inherit.strike),
        obfuscated: flag("obfuscated", inherit.obfuscated),
    };
    if let Nbt::String(s) = tag {
        // THE BARE LEAF (the string IS the text — the inherited style
        // the parent chain carries)
        emit_chat_state(resolved, cur, out);
        out.push_str(s);
        return;
    }
    match tag {
        Nbt::String(_) => unreachable!("the bare leaf the arm above answers"),
        Nbt::Compound(_) => {
            emit_chat_state(resolved, cur, out);
            if let Some(Nbt::String(s)) = tag.get("text") {
                out.push_str(s);
            }
            // THE EXTRA WALK (the vanilla component's own sibling law:
            // each child's parent style IS this component's — the
            // resolved style the subtree carries)
            if let Some(Nbt::List(children)) = tag.get("extra") {
                for child in children {
                    flatten_component_colored_walk(child, resolved, out, cur);
                }
            }
        }
        // THE LIST FORM (the component-array root the wire speaks —
        // the elements the SIBLING walk renders, the empty root style
        // the inherited color keeps)
        Nbt::List(children) => {
            for child in children {
                flatten_component_colored_walk(child, inherit, out, cur);
            }
        }
        _ => {}
    }
}

/// The component tag's own COLORED text (the chat display's flatten —
/// the walk [`flatten_component_text`] speaks PLUS the styles: the
/// structured compound's "color" field the sixteen `ChatFormatting`
/// codes emit (`§0`..`§f`, the palette the font renderer's own
/// `StringDecomposer` walk wears), the "reset" the `§r` the draw's
/// entry-base law answers, the child the color field omits the
/// PARENT's color wears (the vanilla Style resolution), the siblings
/// the common parent wears (never each other — the extra walk's own
/// law), the TRUE HEX the `#RRGGBB` field the legacy `§x§r§r§g§g§b§b`
/// emission speaks (the `CraftChatMessage` convention the real
/// servers' plugin strings carry, the rgb switch the SAME
/// state-machine law walks the named codes answer, the rgb child the
/// field omits the PARENT's rgb inherits). THE STYLE FLAGS the same walk
/// emits: the bold/underlined/strikethrough/italic fields the `§l`/
/// `§n`/`§m`/`§o` codes speak (the effect accumulation the
/// `applyLegacyFormat` law banks), the flag the child omits the
/// PARENT's flag wears, the flag-loss the `§r` reset answers (the
/// legacy alphabet's only eraser), the color switch the flag-clear
/// law wears (the fresh style the applyLegacyFormat's own
/// `Boolean.valueOf(0)` walk builds — the flags the re-emission
/// restores), the obfuscated the `§k` code speaks (the draw lane's
/// random-glyph walk the pick answers — the 26.2 `getRandomGlyph`
/// class law the font table's width-class pool wears). The
/// bare TAG_String IS the text (the codes inside it the draw's own
/// inline walk answers); the LIST root the component-array form the
/// sibling walk renders. The sidebar/tab lanes keep the monochrome
/// [`flatten_component_text`] — the colored walk the CHAT display's
/// law wears.
pub fn flatten_component_colored(tag: &Nbt) -> String {
    let mut out = String::new();
    let mut cur = ChatStyleState::default(); // the draw call's own base (the chat's white)
    flatten_component_colored_walk(tag, ChatStyleState::default(), &mut out, &mut cur);
    out
}

/// THE SYSTEM CHAT'S OWN FACTS (javap of the real 26.2
/// `ClientboundSystemChatPacket` — the clinit's composite: the
/// `ComponentSerialization.TRUSTED_STREAM_CODEC` component + the
/// `ByteBufCodecs.BOOL` overlay). The component rides one bare NBT tag
/// (the house's own component read the entity-data lane already walks);
/// the overlay boolean says the HUD-center overlay message the death
/// screen's word wears (`extractOverlayMessage`'s lane) instead of the
/// chat history.
#[derive(Debug, Clone, PartialEq)]
pub struct SystemChatMessage {
    /// The component flattened to text with the colors the inline §
    /// codes carry (the `text` + `extra` + `color` walk
    /// [`flatten_component_colored`] speaks — the colored chat the
    /// real servers' own words wear).
    pub content: String,
    /// The overlay flag (false = the chat history, true = the HUD
    /// overlay message).
    pub overlay: bool,
}

/// Parse the System Chat packet (generated id 121): ONE bare-NBT
/// component + the overlay boolean. Truncation refuses.
pub fn parse_system_chat(payload: &[u8]) -> Result<SystemChatMessage> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SYSTEM_CHAT,
        "not a system chat packet (0x{id:02X})"
    );
    let content =
        flatten_component_colored(&read_bare_nbt(&mut c).context("system chat component")?);
    anyhow::ensure!(!c.is_empty(), "system chat truncated (overlay flag)");
    let overlay = read_bool(&mut c)?;
    Ok(SystemChatMessage { content, overlay })
}

/// THE PLAYER CHAT'S OWN FACTS (javap of the real 26.2
/// `ClientboundPlayerChatPacket` — the RegistryFriendlyByteBuf read
/// verbatim): globalIndex VarInt, sender UUID, index VarInt, the
/// signature `readNullable` (bool + 256 raw bytes when present — the
/// `MessageSignature.read`'s fixed array), the `SignedMessageBody$Packed`
/// (content Utf(256) + Instant long + salt long + the last-seen
/// `readCollection` limit 20 = VarInt count x 256-byte entries), the
/// unsigned content `readNullable` component, the `FilterMask.read`
/// (the readEnum Type ordinal — PASS_THROUGH 0 / FULLY_FILTERED 1 /
/// PARTIALLY_FILTERED 2 the clinit's `new Type(.., 0..2)` walk banks;
/// the partial lane's `readBitSet` = VarInt length + bytes), then the
/// `ChatType$Bound.STREAM_CODEC`: the holder varint (the house's own
/// tagged-holder law — 0 the direct NBT lane, n >= 1 the `(n - 1)`
/// registry id), the sender name component, the optional target name
/// component.
///
/// The display the CLIENT's own decoration walk answers: the
/// unsigned content when present else the signed body content (the
/// vanilla `PlayerChatMessage.decode` walk), the FULLY_FILTERED mask
/// the empty string wears (the filtered-out message the client never
/// speaks), the chat type's `translation_key` + `parameters` the
/// format fills (the registry table the configuration phase synced).
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerChatMessage {
    /// The sender's profile UUID (the tab-list population the same
    /// ids speak).
    pub sender: [u8; 16],
    /// The sender name component flattened (the bound's own display
    /// name).
    pub sender_name: String,
    /// The optional target name component flattened (the team-msg /
    /// msg-command decorations' target slot).
    pub target_name: Option<String>,
    /// The displayed text: the unsigned content when present else the
    /// signed body content; the FULLY_FILTERED mask the empty string.
    pub content: String,
    /// The chat type's decoration translation key (the registry
    /// entry's `chat.translation_key` — "chat.type.text" the vanilla
    /// chat entry speaks).
    pub chat_key: String,
    /// The chat type's decoration parameter names in order ("sender",
    /// "content", "target").
    pub parameters: Vec<String>,
}

/// Parse the Player Chat packet (generated id 65). The chat-type id
/// resolves through the synced registry table (`chat_types` — the
/// configuration phase's arrival order); an id the table never synced
/// is the drift it is — refused, the byIdOrThrow direction. The
/// direct-holder lane (wire tag 0) reads the element NBT itself.
pub fn parse_player_chat(
    payload: &[u8],
    chat_types: &[crate::config::ChatTypeEntry],
) -> Result<PlayerChatMessage> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::PLAYER_CHAT,
        "not a player chat packet (0x{id:02X})"
    );
    let _global_index = read_varint(&mut c)?;
    let sender = read_uuid(&mut c)?;
    let _index = read_varint(&mut c)?;
    // the signature: nullable, 256 raw bytes when present
    if read_bool(&mut c)? {
        anyhow::ensure!(c.len() >= 256, "player chat signature truncated");
        c = &c[256..];
    }
    // the SignedMessageBody$Packed: content + instant + salt + last-seen
    let body_content = read_string(&mut c)?;
    let _time_millis = read_i64(&mut c)?;
    let _salt = read_i64(&mut c)?;
    let seen = read_varint(&mut c)?;
    anyhow::ensure!(seen <= 20, "player chat last-seen {seen} past the 20 cap");
    anyhow::ensure!(
        c.len() >= seen as usize * 256,
        "player chat last-seen entries truncated"
    );
    c = &c[seen as usize * 256..];
    // the unsigned content: nullable component
    let unsigned = if read_bool(&mut c)? {
        Some(flatten_component_colored(
            &read_bare_nbt(&mut c).context("player chat unsigned content")?,
        ))
    } else {
        None
    };
    // the filter mask: the Type ordinal + the partial lane's bitset
    let mask = read_varint(&mut c)?;
    let fully_filtered = match mask {
        0 => false,        // PASS_THROUGH
        1 => true,         // FULLY_FILTERED
        2 => {
            // PARTIALLY_FILTERED: readBitSet = the VarInt length + bytes
            let len = read_varint(&mut c)?;
            anyhow::ensure!(c.len() >= len as usize, "player chat filter mask truncated");
            c = &c[len as usize..];
            false
        }
        other => anyhow::bail!("unknown filter mask type {other} (the enum's 0..=2 walk)"),
    };
    // the chat type bound: the tagged holder + the names (the walk the
    // [`read_chat_type_bound`] helper speaks — the same record the
    // disguised chat's bound carries)
    let (chat_key, parameters, sender_name, target_name) =
        read_chat_type_bound(&mut c, chat_types)?;
    let content = if fully_filtered {
        String::new()
    } else {
        unsigned.unwrap_or(body_content)
    };
    Ok(PlayerChatMessage {
        sender,
        sender_name,
        target_name,
        content,
        chat_key,
        parameters,
    })
}

/// THE BOUND'S OWN WALK (javap of the real 26.2
/// `ChatType$Bound.STREAM_CODEC` — the clinit's composite: the
/// `ChatType.STREAM_CODEC` holder + the sender name component + the
/// OPTIONAL target name component, the two TRUSTED_STREAM_CODEC reads
/// the composite speaks). The tagged-holder law the house's own
/// sound-event lane banks: wire tag 0 the DIRECT element NBT (the
/// `{chat:{translation_key, parameters}}` record the same
/// [`crate::config::parse_chat_type_element`] walk answers), n >= 1 the
/// `(n - 1)` registry id the synced chat_types table resolves — the
/// unknown id the byIdOrThrow direction refuses. The player chat and
/// the disguised chat packets carry the SAME bound record.
fn read_chat_type_bound<'a>(
    c: &mut &'a [u8],
    chat_types: &[crate::config::ChatTypeEntry],
) -> Result<(String, Vec<String>, String, Option<String>)> {
    let holder = read_varint(c)?;
    let (chat_key, parameters) = if holder == 0 {
        let element = read_network_nbt(c).context("chat-type direct element")?;
        let entry = crate::config::parse_chat_type_element(&element, "<direct>")
            .context("chat-type direct decoration")?;
        (entry.chat_key, entry.parameters)
    } else {
        let idx = (holder - 1) as usize;
        let entry = chat_types.get(idx).with_context(|| {
            format!(
                "chat-type id {} past the synced {}-entry table (the byIdOrThrow direction)",
                holder - 1,
                chat_types.len()
            )
        })?;
        (entry.chat_key.clone(), entry.parameters.clone())
    };
    let sender_name =
        flatten_component_colored(&read_bare_nbt(c).context("chat bound sender name")?);
    let target_name = if read_bool(c)? {
        Some(flatten_component_colored(
            &read_bare_nbt(c).context("chat bound target name")?,
        ))
    } else {
        None
    };
    Ok((chat_key, parameters, sender_name, target_name))
}

/// THE DISGUISED CHAT'S OWN FACTS (javap of the real 26.2
/// `ClientboundDisguisedChatPacket` — the clinit's composite: the
/// `ComponentSerialization.TRUSTED_STREAM_CODEC` content + the
/// `ChatType$Bound.STREAM_CODEC` bound; NO sender UUID, NO index, NO
/// signed body — the server-authored words the /say, /me, /tellraw and
/// the plugin broadcasts speak). The display the same decoration walk
/// the player chat answers — the bound's key + parameters the format
/// fills, the component text the content slot carries.
#[derive(Debug, Clone, PartialEq)]
pub struct DisguisedChatMessage {
    /// The component flattened to plain text (the content slot the
    /// decoration's "content" parameter fills).
    pub content: String,
    /// The bound's sender name component flattened (the "sender"
    /// parameter's source — the /say command source's display name).
    pub sender_name: String,
    /// The bound's optional target name component flattened.
    pub target_name: Option<String>,
    /// The chat type's decoration translation key (the registry
    /// entry's `chat.translation_key` — "chat.type.announcement" the
    /// say-command entry speaks).
    pub chat_key: String,
    /// The chat type's decoration parameter names in order.
    pub parameters: Vec<String>,
}

/// Parse the Disguised Chat packet (generated id 33): the flattened
/// component + the chat type bound. The id the chat wire round's
/// unknown-skip lane used to swallow — the plugin words the real
/// servers' /say and /tellraw broadcast.
pub fn parse_disguised_chat(
    payload: &[u8],
    chat_types: &[crate::config::ChatTypeEntry],
) -> Result<DisguisedChatMessage> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::DISGUISED_CHAT,
        "not a disguised chat packet (0x{id:02X})"
    );
    let content =
        flatten_component_colored(&read_bare_nbt(&mut c).context("disguised chat content")?);
    let (chat_key, parameters, sender_name, target_name) = read_chat_type_bound(&mut c, chat_types)?;
    Ok(DisguisedChatMessage {
        content,
        sender_name,
        target_name,
        chat_key,
        parameters,
    })
}

/// One number format's payload walk (the dispatch the registry ids
/// speak — 26.2's own bootstrap order: blank, styled, fixed; the
/// FIXED-WIDTH format the 1.21 era spoke is GONE from 26.2). The
/// model renders plain integers (the INTEGER render type's own
/// default law), so the format's only wire duty is to be CONSUMED:
/// blank reads nothing, styled/fixed are ONE bare network NBT tag
/// each (the Style / Component TRUSTED_STREAM_CODEC machinery).
fn skip_number_format(r: &mut &[u8]) -> Result<()> {
    let ty = read_varint(r)?;
    match ty {
        0 => Ok(()),
        1 | 2 => {
            read_bare_nbt(r)?;
            Ok(())
        }
        _ => Err(anyhow::anyhow!(
            "number format type {ty} outside 26.2's registry (blank 0 / styled 1 / fixed 2)"
        )),
    }
}

/// The OPTIONAL op over the number format (the same boolean-flag walk
/// every optional codec the house speaks keeps).
fn skip_optional_number_format(r: &mut &[u8]) -> Result<()> {
    if read_bool(r)? {
        skip_number_format(r)
    } else {
        Ok(())
    }
}

/// Set Objective's wire walk (see [`ObjectiveUpdate`] — the
/// generated-id guard).
pub fn parse_set_objective(payload: &[u8]) -> Result<ObjectiveUpdate> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SET_OBJECTIVE,
        "not a set objective packet (0x{id:02X})"
    );
    let objective = read_string(&mut c)?;
    let method = read_u8(&mut c)?;
    let (display, render_type) = if method == OBJECTIVE_METHOD_ADD
        || method == OBJECTIVE_METHOD_UPDATE
    {
        let display = flatten_component_text(&read_bare_nbt(&mut c)?);
        let rt = read_varint(&mut c)?;
        anyhow::ensure!(
            (0..=1).contains(&rt),
            "objective render type ordinal {rt} outside the enum"
        );
        skip_optional_number_format(&mut c)?;
        (Some(display), rt as u8)
    } else {
        // the ctor's own defaults branch: no further wire reads
        (None, RENDER_TYPE_INTEGER)
    };
    Ok(ObjectiveUpdate { objective, method, display, render_type })
}

/// Set Display Objective's wire walk (see [`DisplayObjectiveUpdate`] —
/// the generated-id guard).
pub fn parse_set_display_objective(payload: &[u8]) -> Result<DisplayObjectiveUpdate> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SET_DISPLAY_OBJECTIVE,
        "not a set display objective packet (0x{id:02X})"
    );
    let slot = read_varint(&mut c)?;
    anyhow::ensure!((0..=255).contains(&slot), "display slot {slot} outside the byte range");
    let objective = read_string(&mut c)?;
    Ok(DisplayObjectiveUpdate { slot: slot as u8, objective })
}

/// Set Score's wire walk (see [`ScoreUpdate`] — the generated-id
/// guard).
pub fn parse_set_score(payload: &[u8]) -> Result<ScoreUpdate> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::SET_SCORE, "not a set score packet (0x{id:02X})");
    let owner = read_string(&mut c)?;
    let objective = read_string(&mut c)?;
    let score = read_varint(&mut c)?;
    let display = if read_bool(&mut c)? {
        Some(flatten_component_text(&read_bare_nbt(&mut c)?))
    } else {
        None
    };
    skip_optional_number_format(&mut c)?;
    Ok(ScoreUpdate { owner, objective, score, display })
}

/// Reset Score's wire walk (see [`ResetScore`] — the generated-id
/// guard).
pub fn parse_reset_score(payload: &[u8]) -> Result<ResetScore> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::RESET_SCORE,
        "not a reset score packet (0x{id:02X})"
    );
    let owner = read_string(&mut c)?;
    let objective = if read_bool(&mut c)? {
        Some(read_string(&mut c)?)
    } else {
        None
    };
    Ok(ResetScore { owner, objective })
}

/// One player-info entry's PATCH (the ClientboundPlayerInfoUpdate
/// packet's per-entry walk): the UUID every entry carries FIRST, then
/// ONE field per action bit the packet's mask spoke — `None` means
/// the action was not in the mask, the field the wire never spoke.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerEntryPatch {
    /// The profile's UUID (two big-endian longs, most-significant
    /// first — the `new UUID(readLong(), readLong())` law).
    pub uuid: u128,
    /// ADD_PLAYER's profile name (the tab row's own text).
    pub name: Option<String>,
    /// UPDATE_GAME_MODE's GameType.byId VarInt (0 survival, 1
    /// creative, 2 adventure, 3 spectator).
    pub game_mode: Option<u8>,
    /// UPDATE_LISTED's Boolean — the tab's own visibility bit.
    pub listed: Option<bool>,
    /// UPDATE_LATENCY's VarInt (the milliseconds the bars read).
    pub latency: Option<i32>,
    /// UPDATE_DISPLAY_NAME's optional component (the flat text; the
    /// double option: `Some(None)` = the flag spoke an ABSENT name).
    pub display_name: Option<Option<String>>,
    /// UPDATE_LIST_ORDER's VarInt (the sort's first key).
    pub list_order: Option<i32>,
}

/// The Player Info Update packet (clientbound play, generated id 70 —
/// `minecraft:player_info_update`). The wire walk (raw disasm of the
/// 26.2 ctor + the Action enum's clinit/readers): `readEnumSet` = ONE
/// `readFixedBitSet(8)` byte (bit i = Action ordinal i), then
/// `readList` — a VarInt count, and per entry `readUUID()` followed by
/// EACH masked action's reader IN ORDINAL ORDER:
/// ADD_PLAYER(0) = STRING name + the property map, INITIALIZE_CHAT(1)
/// = readNullable chat session, UPDATE_GAME_MODE(2) = VarInt,
/// UPDATE_LISTED(3) = Boolean, UPDATE_LATENCY(4) = VarInt,
/// UPDATE_DISPLAY_NAME(5) = readNullable component, UPDATE_LIST_ORDER
/// (6) = VarInt, UPDATE_HAT(7) = Boolean. 26.2's enum has NO
/// REMOVE_PLAYER — the removal is its own packet (id 69).
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfoUpdate {
    /// One patch per entry, in the wire's own order.
    pub entries: Vec<PlayerEntryPatch>,
}

/// The Player Info Remove packet (clientbound play, generated id 69 —
/// `minecraft:player_info_remove`). The wire walk (raw disasm of the
/// record's `<init>(FriendlyByteBuf)`): `readList(UUIDUtil.STREAM_CODEC)`
/// — a VarInt count then that many UUIDs.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfoRemove {
    /// The profile UUIDs whose entries die.
    pub uuids: Vec<u128>,
}

/// The Tab List packet (clientbound play, generated id 122 —
/// `minecraft:tab_list`). The wire walk (raw disasm of the composite
/// clinit): header + footer, BOTH `ComponentSerialization.
/// TRUSTED_STREAM_CODEC` — ONE bare network NBT tag each, the same
/// walk the objective display reads (never absent: an EMPTY title
/// still ships the bare tag).
#[derive(Debug, Clone, PartialEq)]
pub struct TabListUpdate {
    /// The header's flattened TEXT.
    pub header: String,
    /// The footer's flattened TEXT.
    pub footer: String,
}

/// The ADD_PLAYER property map's own walk (the authlib GameProfile
/// codec's second read — the stable 1.19.3+ law the modern client
/// keeps): a VarInt count, then per property the name STRING, the
/// value STRING, and an optional signature (a Boolean flag, then the
/// STRING). The house carries no skin lane — the map's only wire duty
/// is to be CONSUMED (the textures property's base64 never parses).
fn skip_property_map(r: &mut &[u8]) -> Result<()> {
    let count = read_varint(r)?;
    for _ in 0..count {
        let _name = read_string(r)?;
        let _value = read_string(r)?;
        if read_bool(r)? {
            let _signature = read_string(r)?;
        }
    }
    Ok(())
}

/// The INITIALIZE_CHAT action's own consumption (the
/// `RemoteChatSession.Data.read` walk the raw disasm speaks): a
/// Boolean flag, then UUID + `ProfilePublicKey.Data` — the expiry
/// Instant (a long, the epoch millis), the public key (a
/// varint-length byte array), the key signature (a varint-length byte
/// array). The house speaks no signed chat — the session's only wire
/// duty is to be consumed exactly.
fn skip_chat_session(r: &mut &[u8]) -> Result<()> {
    if read_bool(r)? {
        let _session_id = u128::from_be_bytes(read_uuid(r)?);
        let _expires_at = read_i64(r)?;
        let key = read_varint(r)? as usize;
        skip_len_bytes(r, key)?;
        let sig = read_varint(r)? as usize;
        skip_len_bytes(r, sig)?;
    }
    Ok(())
}

/// One varint-length byte array's own consumption (the
/// `readByteArray(n)` law the key/signature reads speak): the length
/// was already read — the payload's duty is to be exactly there.
fn skip_len_bytes(r: &mut &[u8], len: usize) -> Result<()> {
    if r.len() < len {
        anyhow::bail!("byte array needs {len} bytes, only {} left", r.len());
    }
    *r = &r[len..];
    Ok(())
}

/// The TWO-LONG UUID's own fold (the `new UUID(readLong(), readLong())`
/// law the Add Entity walk shares): the most-significant long FIRST,
/// the pair a big-endian u128.
fn read_u128(r: &mut &[u8]) -> Result<u128> {
    let msb = read_i64(r)? as u64;
    let lsb = read_i64(r)? as u64;
    Ok(((msb as u128) << 64) | (lsb as u128))
}

/// Player Info Update's wire walk (see [`PlayerInfoUpdate`] — the
/// generated-id guard, the bitmask's ordinal dispatch).
pub fn parse_player_info_update(payload: &[u8]) -> Result<PlayerInfoUpdate> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::PLAYER_INFO_UPDATE,
        "not a player info update packet (0x{id:02X})"
    );
    let mask = read_u8(&mut c)?;
    let count = read_varint(&mut c)?;
    let mut entries = Vec::with_capacity(count.min(1024) as usize);
    for _ in 0..count {
        let uuid = read_u128(&mut c)?;
        let mut patch = PlayerEntryPatch {
            uuid,
            name: None,
            game_mode: None,
            listed: None,
            latency: None,
            display_name: None,
            list_order: None,
        };
        if mask & 0x01 != 0 {
            patch.name = Some(read_string(&mut c)?);
            skip_property_map(&mut c)?;
        }
        if mask & 0x02 != 0 {
            skip_chat_session(&mut c)?;
        }
        if mask & 0x04 != 0 {
            let gm = read_varint(&mut c)?;
            anyhow::ensure!(
                (0..=3).contains(&gm),
                "game mode {gm} outside the GameType enum"
            );
            patch.game_mode = Some(gm as u8);
        }
        if mask & 0x08 != 0 {
            patch.listed = Some(read_bool(&mut c)?);
        }
        if mask & 0x10 != 0 {
            patch.latency = Some(read_varint(&mut c)?);
        }
        if mask & 0x20 != 0 {
            patch.display_name = Some(if read_bool(&mut c)? {
                Some(flatten_component_text(&read_bare_nbt(&mut c)?))
            } else {
                None
            });
        }
        if mask & 0x40 != 0 {
            patch.list_order = Some(read_varint(&mut c)?);
        }
        if mask & 0x80 != 0 {
            let _show_hat = read_bool(&mut c)?;
        }
        entries.push(patch);
    }
    Ok(PlayerInfoUpdate { entries })
}

/// Player Info Remove's wire walk (see [`PlayerInfoRemove`] — the
/// generated-id guard, the UUID list).
pub fn parse_player_info_remove(payload: &[u8]) -> Result<PlayerInfoRemove> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::PLAYER_INFO_REMOVE,
        "not a player info remove packet (0x{id:02X})"
    );
    let count = read_varint(&mut c)?;
    let mut uuids = Vec::with_capacity(count.min(1024) as usize);
    for _ in 0..count {
        uuids.push(read_u128(&mut c)?);
    }
    Ok(PlayerInfoRemove { uuids })
}

/// Tab List's wire walk (see [`TabListUpdate`] — the generated-id
/// guard, the two bare-NBT components).
pub fn parse_tab_list(payload: &[u8]) -> Result<TabListUpdate> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::TAB_LIST, "not a tab list packet (0x{id:02X})");
    let header = flatten_component_text(&read_bare_nbt(&mut c)?);
    let footer = flatten_component_text(&read_bare_nbt(&mut c)?);
    Ok(TabListUpdate { header, footer })
}

/// Player Combat Kill (clientbound play, generated id 68 —
/// `minecraft:player_combat_kill`). javap of the real 26.2 record's
/// STREAM_CODEC read: a VarInt player id, then the death message
/// Component — the TEXT VARIANT (an `IdMapper`-resolved holder id or
/// the bare network NBT; the disconnect lane's own string-or-NBT
/// ambiguity — here read as the component TEXT the JSON flatten
/// speaks, the disconnect's own law). The death screen's message.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerCombatKill {
    /// The dead player's entity id.
    pub player_id: i32,
    /// The flattened death message (the component's own text).
    pub message: String,
}

/// Player Combat Kill's wire walk (see [`PlayerCombatKill`] — the
/// VarInt id, the component read the disconnect flattens).
pub fn parse_player_combat_kill(payload: &[u8]) -> Result<PlayerCombatKill> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::PLAYER_COMBAT_KILL,
        "not a player combat kill packet (0x{id:02X})"
    );
    let player_id = read_varint(&mut c)?;
    let raw = read_string(&mut c)?;
    let flat = serde_json::from_str::<serde_json::Value>(&raw)
        .map(|v| crate::status::extract_text(&v))
        .unwrap_or(raw);
    Ok(PlayerCombatKill { player_id, message: flat })
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
    /// Game Event: the server's one-byte world-opinion word (the
    /// weather the client cannot otherwise learn — the rain the sky
    /// renders, the thunder the clouds speak) plus a float param the
    /// event's own law reads. The event byte's own vocabulary the
    /// [`game_event`] module's consts name — the
    /// ClientboundGameEventPacket$Type ids the 26.2 clinit's
    /// `new Type(0)..new Type(13)` walk banks verbatim.
    GameEvent(GameEvent),
    /// Player Abilities: the server's own abilities word — the ONE
    /// byte the four flag bits keep (bit 1 the invulnerable word,
    /// bit 2 the FLYING state, bit 4 the mayfly grant, bit 8 the
    /// instabuild grant) plus the two speed floats the read ctor
    /// takes unconditionally. The echo the abilities wire family
    /// completes: the outbound [`encode_player_abilities`] ONE byte
    /// the changed fly tick speaks, the serverbound lane's own
    /// answer this packet rides back — the handler's six-putfield
    /// walk (flying, instabuild, invulnerable, mayfly, flyingSpeed,
    /// walkingSpeed) the frame thread's installer wears. See
    /// [`PlayerAbilitiesWord`].
    PlayerAbilities(PlayerAbilitiesWord),
    /// Change Difficulty: the server's own level-data word — the
    /// danger dial's id (0 Peaceful..3 Hard, the VARINT the
    /// `Difficulty.STREAM_CODEC` idMapper keeps) plus the LOCKED bit
    /// the difficulty lock speaks. The store the client's own
    /// `handleChangeDifficulty` walk banks — the levelData pair the
    /// networked world's dial + lock wear. See
    /// [`ChangeDifficultyWord`].
    ChangeDifficulty(ChangeDifficultyWord),
    /// Keep-alive probe answered.
    KeepAlive(i64),
    /// Ping probe answered.
    Ping(i32),
    /// Pong Response: the server's echo of the client's OWN ping
    /// request (the PingDebugMonitor's RTT lane — the sent-time long
    /// the roundtrip echoes verbatim; the receive-time difference the
    /// monitor's sample speaks). The answer the debug chart's own
    /// wire pair carries — see [`encode_ping_request`].
    PongResponse(i64),
    /// Debug Sample: the server's tick-time broadcast (the TPS chart's
    /// own wire lane — the ClientboundDebugSamplePacket the dedicated
    /// server's RemoteSampleLogger broadcasts to the subscribers the
    /// subscription request named; the four TpsDebugDimensions
    /// nanosecond slots the sample carries, the chart's stacked
    /// columns the dims speak).
    DebugSample(DebugSample),
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
    /// Level Event: the server's own world-effect word (the growth
    /// effect the bone-meal item's use broadcasts — the client's
    /// particle vocabulary the render lane answers). See
    /// [`LevelEventPacket`].
    LevelEvent(LevelEventPacket),
    /// Add Entity: something new exists in the world (a mob, a dropped
    /// item, a projectile) — the model's entity registry grows.
    Spawn(EntitySpawn),
    /// Remove Entities: a batch of entity ids ceased to exist.
    EntitiesRemoved(Vec<i32>),
    /// Set Passengers: who rides what NOW (the full list, the empty
    /// array the dismount broadcast — the model's vehicle belief
    /// re-baselines at it).
    SetPassengers(SetPassengers),
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
    /// Set Health: the LOCAL player's vitals snapshot — the health
    /// float, the food int, the saturation float (the HUD's hearts and
    /// the hunger row's own wire transport).
    Health(SetHealth),
    /// Hurt Animation: ONE entity's hurt flash-and-lean — the VarInt id
    /// + the tilt yaw float (the damage feedback's own channel; the
    /// model filters the LOCAL player's own id).
    Hurt(HurtAnimation),
    /// Entity Sound: ONE entity's played voice (the ambient/hurt/death
    /// sounds the SERVER plays — `Mob.baseTick`'s playAmbientSound and
    /// friends) — the VarInt id + the tagged sound holder + the
    /// volume/pitch floats + the seed. The mob lane's own audio
    /// channel (the voice the named entity's client copy speaks).
    EntitySound(EntitySoundPacket),
    /// Sound: the GLOBAL sound channel — the sound the SERVER played
    /// at a WORLD SPOT (the block-use voices the other players' lanes
    /// trigger, the server-side machines, the weather) — the tagged
    /// holder + the source-category enum + the 1/8-grid world
    /// position + the volume/pitch/seed (see [`WorldSoundPacket`]).
    /// The model resolves the name through the synced sound_event
    /// table and arms the positioned voice the ear's own distance
    /// law folds.
    WorldSound(WorldSoundPacket),
    /// Player Combat Kill: the LOCAL player died — the VarInt player id
    /// + the death message component (the death screen's own wire).
    CombatKill(PlayerCombatKill),
    /// Set Objective: the scoreboard's objective bookkeeping — the add
    /// (display + render type + optional number format), the remove,
    /// the display update (see [`ObjectiveUpdate`]). The sidebar lane's
    /// first wire word.
    Objective(ObjectiveUpdate),
    /// Set Display Objective: the slot the sidebar renders from — the
    /// slot id + the objective name (the EMPTY name clears the slot;
    /// see [`DisplayObjectiveUpdate`]).
    DisplayObjective(DisplayObjectiveUpdate),
    /// Set Score: one score row's upsert — the holder, the objective,
    /// the value, the optional display override (see [`ScoreUpdate`]).
    Score(ScoreUpdate),
    /// Reset Score: one holder's row dies — the named objective's or
    /// every objective's (see [`ResetScore`]).
    ResetScore(ResetScore),
    /// Player Info Update: the tab's own population walk — the add
    /// (name + properties), the game-mode/listed/latency/display-name/
    /// list-order/hat updates, the chat session (see
    /// [`PlayerInfoUpdate`]).
    PlayerInfoUpdate(PlayerInfoUpdate),
    /// Player Info Remove: the profiles whose tab entries die (see
    /// [`PlayerInfoRemove`]).
    PlayerInfoRemove(PlayerInfoRemove),
    /// Tab List: the header + footer components the TAB overlay draws
    /// (see [`TabListUpdate`]).
    TabList(TabListUpdate),
    /// System Chat: the server's OWN line (the join/leave/death of the
    /// unsigned worlds, the plugin words, the advancement toasts'
    /// text lane) — the component flattened + the overlay flag (see
    /// [`SystemChatMessage`]). The chat history the display queue
    /// answers; the overlay flag the HUD-center lane's word.
    SystemChat(SystemChatMessage),
    /// Player Chat: one player's said word — the bound decoration's
    /// key + parameters the client's own format walk fills (the
    /// en_us table the `<%s> %s` chat.type.text speaks), the sender
    /// + content the display line carries (see
    /// [`PlayerChatMessage`]). The signed-era machinery the house
    /// speaks none of — the unsigned display the wire honestly
    /// carries.
    PlayerChat(PlayerChatMessage),
    /// Disguised Chat: the SERVER-authored word (the /say, /me,
    /// /tellraw and the plugin broadcasts the real servers speak —
    /// no sender UUID, no signed body, just the component + the
    /// bound) — the same decoration walk the player chat answers
    /// (see [`DisguisedChatMessage`]).
    DisguisedChat(DisguisedChatMessage),
    /// Command Suggestions: the server's answer to the chat screen's
    /// ask-server walk (the
    /// [`encode_command_suggestion`] request the typing lane sends on
    /// every '/' edit, the reply's replace range + rows the
    /// suggestion list renders — see [`CommandSuggestionsReply`]).
    /// The stale transaction ids the consumer's pending gate drops.
    CommandSuggestions(CommandSuggestionsReply),
    /// Commands: the server's whole Brigadier tree — the login-time
    /// packet `ClientPacketListener.handleCommands` builds the local
    /// dispatcher from (the chat screen's LOCAL suggestion lane the
    /// literals answer; the argument slots' dynamic suggestions the
    /// ask-server walk still owns). See [`parse_commands`].
    Commands(CommandTree),
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
            clientbound::CHANGE_DIFFICULTY => {
                Ok(PlayEvent::ChangeDifficulty(parse_change_difficulty(&pkt)?))
            }
            clientbound::UPDATE_LIGHT => {
                let (x, z, light) = parse_update_light(&pkt)?;
                Ok(PlayEvent::Light { x, z, light })
            }
            clientbound::KEEP_ALIVE => {
                let probe = parse_play_keep_alive(&pkt)?;
                self.conn.send_packet(&encode_play_keep_alive_response(probe))?;
                Ok(PlayEvent::KeepAlive(probe))
            }
            clientbound::GAME_EVENT => Ok(PlayEvent::GameEvent(parse_game_event(&pkt)?)),
            clientbound::PLAYER_ABILITIES => {
                Ok(PlayEvent::PlayerAbilities(parse_player_abilities(&pkt)?))
            }
            clientbound::PING => {
                let probe = parse_play_ping(&pkt)?;
                self.conn.send_packet(&encode_play_pong(probe))?;
                Ok(PlayEvent::Ping(probe))
            }
            clientbound::PONG_RESPONSE => {
                Ok(PlayEvent::PongResponse(parse_pong_response(&pkt)?))
            }
            clientbound::DEBUG_SAMPLE => {
                Ok(PlayEvent::DebugSample(parse_debug_sample(&pkt)?))
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
            clientbound::LEVEL_EVENT => Ok(PlayEvent::LevelEvent(parse_level_event(&pkt)?)),
            clientbound::ADD_ENTITY => Ok(PlayEvent::Spawn(parse_add_entity(&pkt)?)),
            clientbound::REMOVE_ENTITIES => {
                Ok(PlayEvent::EntitiesRemoved(parse_remove_entities(&pkt)?))
            }
            clientbound::SET_PASSENGERS => {
                Ok(PlayEvent::SetPassengers(parse_set_passengers(&pkt)?))
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
            clientbound::SET_HEALTH => Ok(PlayEvent::Health(parse_set_health(&pkt)?)),
            clientbound::HURT_ANIMATION => Ok(PlayEvent::Hurt(parse_hurt_animation(&pkt)?)),
            clientbound::SOUND_ENTITY => {
                Ok(PlayEvent::EntitySound(parse_entity_sound(&pkt)?))
            }
            clientbound::SOUND => Ok(PlayEvent::WorldSound(parse_sound(&pkt)?)),
            clientbound::PLAYER_COMBAT_KILL => {
                Ok(PlayEvent::CombatKill(parse_player_combat_kill(&pkt)?))
            }
            clientbound::SET_OBJECTIVE => {
                Ok(PlayEvent::Objective(parse_set_objective(&pkt)?))
            }
            clientbound::SET_DISPLAY_OBJECTIVE => {
                Ok(PlayEvent::DisplayObjective(parse_set_display_objective(&pkt)?))
            }
            clientbound::SET_SCORE => Ok(PlayEvent::Score(parse_set_score(&pkt)?)),
            clientbound::RESET_SCORE => {
                Ok(PlayEvent::ResetScore(parse_reset_score(&pkt)?))
            }
            clientbound::PLAYER_INFO_UPDATE => {
                Ok(PlayEvent::PlayerInfoUpdate(parse_player_info_update(&pkt)?))
            }
            clientbound::PLAYER_INFO_REMOVE => {
                Ok(PlayEvent::PlayerInfoRemove(parse_player_info_remove(&pkt)?))
            }
            clientbound::TAB_LIST => Ok(PlayEvent::TabList(parse_tab_list(&pkt)?)),
            clientbound::SYSTEM_CHAT => Ok(PlayEvent::SystemChat(parse_system_chat(&pkt)?)),
            clientbound::COMMAND_SUGGESTIONS => Ok(PlayEvent::CommandSuggestions(
                parse_command_suggestions(&pkt)?,
            )),
            clientbound::COMMANDS => Ok(PlayEvent::Commands(parse_commands(
                &pkt,
                &self.registry.command_argument_types,
            )?)),
            clientbound::PLAYER_CHAT => Ok(PlayEvent::PlayerChat(parse_player_chat(
                &pkt,
                &self.registry.chat_types,
            )?)),
            clientbound::DISGUISED_CHAT => Ok(PlayEvent::DisguisedChat(parse_disguised_chat(
                &pkt,
                &self.registry.chat_types,
            )?)),
            _ => Ok(PlayEvent::Skipped),
        }
    }

    /// The player's container click (the vanilla handleContainerInput's
    /// own serverbound echo): the record encoded and sent in place.
    pub fn click_container(&mut self, click: &ContainerClick) -> Result<()> {
        let pkt = encode_container_click(click)?;
        self.conn
            .send_packet(&pkt)
            .context("send container click")
    }

    /// The player's OWN container close (the vanilla closeContainer's
    /// echo — Escape/E on a container screen).
    pub fn close_container(&mut self, container_id: i32) -> Result<()> {
        self.conn
            .send_packet(&encode_container_close(container_id))
            .context("send container close")
    }

    /// The player's OWN hotbar selection (the vanilla keyPress's
    /// serverbound echo — the number keys' lane, the Set Carried Item
    /// packet).
    pub fn set_carried_item(&mut self, slot: u8) -> Result<()> {
        self.conn
            .send_packet(&encode_set_carried_item(slot))
            .context("send set carried item")
    }

    /// The middle-click PICK's wire half (the vanilla
    /// gameMode.handlePickItemFromBlock's own echo — the pick the
    /// keyPickItem loop answers). The hit block's position and the
    /// Ctrl flag the packet's includeData slot rides; the client
    /// predicts NOTHING (the inventory law server-side — the Set Held
    /// Slot echo the verdict's own ride back).
    pub fn pick_item_from_block(&mut self, pos: (i32, i32, i32), include_data: bool) -> Result<()> {
        self.conn
            .send_packet(&encode_pick_item_from_block(pos.0, pos.1, pos.2, include_data))
            .context("send pick item from block")
    }

    /// The pick's ENTITY arm's wire (the vanilla
    /// gameMode.handlePickItemFromEntity's own echo). The SURVIVAL
    /// client never sends it (the pick result the mob lanes never
    /// name); the method rides the protocol's own completeness.
    pub fn pick_item_from_entity(&mut self, entity_id: i32, include_data: bool) -> Result<()> {
        self.conn
            .send_packet(&encode_pick_item_from_entity(entity_id, include_data))
            .context("send pick item from entity")
    }

    /// The bundle selection's wire half (the vanilla
    /// `BundleMouseActions.toggleSelectedBundleItem`'s own echo — the
    /// ServerboundSelectBundleItemPacket the scroll's tail sends after
    /// the local set): the hovered slot's MENU id and the selection the
    /// walk moved (the two VarInts the record's write speaks).
    pub fn select_bundle_item(&mut self, slot_id: i32, selected_item_index: i32) -> Result<()> {
        self.conn
            .send_packet(&encode_select_bundle_item(slot_id, selected_item_index))
            .context("send select bundle item")
    }

    /// The player's right-mouse USE (the vanilla gameMode.useItem's own
    /// echo — the Use Item packet; `hand` is the InteractionHand ordinal,
    /// MAIN_HAND = 0). The sequence is the caller's monotonic prediction
    /// counter (the startPrediction shape — each use/release speaks the
    /// next number).
    pub fn use_item(&mut self, hand: u8, sequence: i32, yaw: f32, pitch: f32) -> Result<()> {
        self.conn
            .send_packet(&encode_use_item(hand, sequence, yaw, pitch))
            .context("send use item")
    }

    /// The player's use ON A BLOCK (the vanilla gameMode.useItemOn's own
    /// echo — the Use Item On packet: the hand, the hit block's wire
    /// position, the hit face, the cursor, the two hit flags and the
    /// prediction sequence; the same counter the use/release spoke).
    #[allow(clippy::too_many_arguments)]
    pub fn use_item_on(
        &mut self,
        hand: u8,
        pos: (i32, i32, i32),
        face: u8,
        cursor: (f32, f32, f32),
        inside: bool,
        world_border: bool,
        sequence: i32,
    ) -> Result<()> {
        self.conn
            .send_packet(&encode_use_item_on(
                hand, pos.0, pos.1, pos.2, face, cursor, inside, world_border, sequence,
            ))
            .context("send use item on")
    }

    /// The player's use RELEASE (the vanilla gameMode.releaseUsingItem's
    /// own echo — the Player Action packet's RELEASE_USE_ITEM action at
    /// BlockPos.ZERO / Direction.DOWN; the same counter the use spoke).
    pub fn release_use_item(&mut self, sequence: i32) -> Result<()> {
        self.conn
            .send_packet(&encode_player_action_release(sequence))
            .context("send release use item")
    }

    /// The DIG wire (the vanilla gameMode's destroy half — the Player
    /// Action packet's START/ABORT/STOP actions through the same
    /// startPrediction counter the use arc speaks; the direction byte is
    /// the clip's face ordinal verbatim — the wire's 3D data value).
    pub fn player_action(
        &mut self,
        action: u8,
        pos: (i32, i32, i32),
        face: u8,
        sequence: i32,
    ) -> Result<()> {
        self.conn
            .send_packet(&encode_player_action(action, pos.0, pos.1, pos.2, face, sequence))
            .context("send player action")
    }

    /// The arm SWING (the vanilla LocalPlayer.swing's echo — the attack
    /// press's own beat and the mining hold's per-tick beat ride this).
    pub fn swing(&mut self, hand: u8) -> Result<()> {
        self.conn
            .send_packet(&encode_swing(hand))
            .context("send swing")
    }

    /// The CHAT send (the vanilla `ServerGamePacketListener`-bound
    /// `sendChat`'s own wire — [`encode_chat`] the byte law speaks):
    /// the message + the wall-clock millis, the unsigned session's
    /// zero-salt / absent-signature / empty last-seen walk. The server's
    /// own echo (the clientbound Player Chat) rides back through the
    /// pump — the vanilla client displays the SERVER's copy, never its
    /// own.
    pub fn send_chat(&mut self, message: &str, epoch_millis: i64) -> Result<()> {
        let pkt = encode_chat(message, epoch_millis).context("encode chat")?;
        self.conn.send_packet(&pkt).context("send chat")
    }

    /// The COMMAND send (the vanilla `sendCommand`'s own wire —
    /// [`encode_chat_command`] the bare-utf law speaks; the 26.2 packet
    /// the signed-era fields dropped). The command body rides WITHOUT
    /// the slash — the strip the ChatScreen's substring walk speaks.
    pub fn send_chat_command(&mut self, command: &str) -> Result<()> {
        self.conn
            .send_packet(&encode_chat_command(command))
            .context("send chat command")
    }

    /// The SUGGESTION REQUEST send (the vanilla
    /// ClientSuggestionProvider.customSuggestion's own wire — the
    /// ServerboundCommandSuggestionPacket the ask-server node's parse
    /// lane sends: the incrementing transaction id the pending gate
    /// remembers, the FULL input the `/`-prefixed command the server's
    /// own skip walk answers — see [`encode_command_suggestion`]). The
    /// reply rides the CommandSuggestions event back through the pump.
    pub fn send_command_suggestion(&mut self, id: i32, command: &str) -> Result<()> {
        let pkt = encode_command_suggestion(id, command).context("encode command suggestion")?;
        self.conn
            .send_packet(&pkt)
            .context("send command suggestion")
    }

    /// The ATTACK wire (the vanilla gameMode.attack's own echo — the
    /// 26.2's dedicated ServerboundAttackPacket the attack walk sends
    /// BEFORE the local prediction; the record's ONE VarInt entityId
    /// the clinit's composite speaks).
    pub fn attack(&mut self, entity_id: i32) -> Result<()> {
        self.conn
            .send_packet(&encode_attack(entity_id))
            .context("send attack")
    }

    /// The INTERACT wire (the vanilla gameMode.interact's own echo —
    /// the 26.2's flat ServerboundInteractPacket the record's composite
    /// order carries: the VarInt id, the VarInt hand, the LpVec3 cursor
    /// the hit location minus the entity origin speaks, the Bool the
    /// shift key rides).
    pub fn interact(
        &mut self,
        entity_id: i32,
        hand: u8,
        cursor: (f64, f64, f64),
        shift: bool,
    ) -> Result<()> {
        self.conn
            .send_packet(&encode_interact(entity_id, hand, cursor, shift))
            .context("send interact")
    }

    /// The INPUT WIRE (the vanilla `LocalPlayer.tick`'s own send —
    /// `lastSentInput != input.keyPresses()` sends one
    /// ServerboundPlayerInputPacket): the seven booleans' record rides
    /// the Input.STREAM_CODEC, the ONE byte the generated-id-43 packet
    /// keeps. The riding lanes read it (the boat's steering the
    /// server-side input drives, the shift the dismount asks).
    pub fn player_input(&mut self, flags: InputFlags) -> Result<()> {
        self.conn
            .send_packet(&encode_player_input(flags))
            .context("send player input")
    }

    /// The PADDLE BOAT wire (the vanilla `AbstractBoat.tick`'s own
    /// send — every client tick while the boat carries the local
    /// player, the two paddle booleans the controlBoat law derived:
    /// the server stores them into the boat's entity data, the
    /// echo the Set Entity Data lane speaks back).
    pub fn paddle_boat(&mut self, left: bool, right: bool) -> Result<()> {
        self.conn
            .send_packet(&encode_paddle_boat(left, right))
            .context("send paddle boat")
    }

    /// The ping request (the vanilla `PingDebugMonitor.tick`'s own
    /// send): the sent-time long rides the generated-id-38 packet —
    /// the server echoes it verbatim through the Clientbound
    /// PongResponsePacket, the receive-instant difference the RTT
    /// sample the ping chart's columns speak. The F3+3 network
    /// charts' raised lane the monitor's cadence arms.
    pub fn send_ping_request(&mut self, time: i64) -> Result<()> {
        self.conn
            .send_packet(&encode_ping_request(time))
            .context("send ping request")
    }

    /// The debug subscription request (the vanilla
    /// ClientDebugSubscriber.onSubscriptionsChanged's own send): the
    /// Set<DebugSubscription> the F3+2 fps charts' state answers —
    /// {DEDICATED_SERVER_TICK_TIME} the raised lane speaks, the empty
    /// set the lowered one (the equality gate the caller's own state
    /// walk keeps — the request rides ONLY the set's change).
    pub fn send_debug_subscription(&mut self, subscribe: bool) -> Result<()> {
        self.conn
            .send_packet(&encode_debug_subscription_request(subscribe))
            .context("send debug subscription request")
    }

    /// The bandwidth monitor's own drain (the vanilla
    /// `BandwidthDebugMonitor.tick`'s `bytesReceived.getAndSet(0)`
    /// walk): the wire bytes the frames accumulated since the last
    /// drain return ONCE and the counter zeroes — the bytes-per-tick
    /// sample the bandwidth chart's columns speak. The connection
    /// tick's own unconditional lane (no chart gate the ping
    /// request's raised lane keeps).
    pub fn take_received_bytes(&mut self) -> u64 {
        std::mem::take(&mut self.conn.bytes_received)
    }

    /// The client command (the vanilla `LocalPlayer`'s own
    /// serverbound echo): the action's VarInt ordinal rides the
    /// generated-id-12 packet — action 0 = PERFORM_RESPAWN (the
    /// death screen's Respawn button's own wire: the server respawns
    /// the player and the position/health streams re-sync).
    pub fn client_command(&mut self, action: i32) -> Result<()> {
        self.conn
            .send_packet(&encode_client_command(action))
            .context("send client command")
    }

    /// The abilities wire (the vanilla `LocalPlayer.onUpdateAbilities`'s
    /// own send — the ONLY ServerboundPlayerAbilitiesPacket walk the
    /// class keeps): the flying state the body's changed tick banks,
    /// the ONE byte the write's `ior 2` law keeps. The aiStep arms the
    /// flying CHANGED verdict earns — the spectator sync, the jump
    /// toggle, the landing retire — the caller's own lanes speak.
    pub fn player_abilities(&mut self, flying: bool) -> Result<()> {
        self.conn
            .send_packet(&encode_player_abilities(flying))
            .context("send player abilities")
    }

    /// The game-mode wire (the vanilla
    /// `GameModeSwitcherScreen.switchToHoveredGameMode`'s own send —
    /// the F4 apply's serverbound half): the hovered mode's GameType
    /// id the VarInt codec carries, the mode the SERVER's belief owns
    /// (the local prediction the switcher's walk never predicts — the
    /// serverbound echo the server's own answer answers).
    pub fn change_game_mode(&mut self, mode_id: i32) -> Result<()> {
        self.conn
            .send_packet(&encode_change_game_mode(mode_id))
            .context("send change game mode")
    }

    /// The movement wire — the body's sendPosition dispatch speaks one
    /// of the four Move Player packets per tick (the law lives in the
    /// body's bookkeeping; these ride the encode fns' javap shapes).
    pub fn move_player_pos_rot(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
        horizontal_collision: bool,
    ) -> Result<()> {
        self.conn
            .send_packet(&encode_move_player_pos_rot(
                x, y, z, yaw, pitch, on_ground, horizontal_collision,
            ))
            .context("send move player pos rot")
    }

    pub fn move_player_pos(&mut self, x: f64, y: f64, z: f64, on_ground: bool, horizontal_collision: bool) -> Result<()> {
        self.conn
            .send_packet(&encode_move_player_pos(x, y, z, on_ground, horizontal_collision))
            .context("send move player pos")
    }

    pub fn move_player_rot(&mut self, yaw: f32, pitch: f32, on_ground: bool, horizontal_collision: bool) -> Result<()> {
        self.conn
            .send_packet(&encode_move_player_rot(yaw, pitch, on_ground, horizontal_collision))
            .context("send move player rot")
    }

    pub fn move_player_status_only(&mut self, on_ground: bool, horizontal_collision: bool) -> Result<()> {
        self.conn
            .send_packet(&encode_move_player_status_only(on_ground, horizontal_collision))
            .context("send move player status only")
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
    /// ((z & 0x3FFFFFF) << 12)`, `BlockPos.asLong(III)` verbatim — the
    /// 26-bit horizontal spans for BOTH x and z, the 12-bit y at the
    /// low end).
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
    use crate::nbt::write_network_nbt;
    use crate::varint::write_string;
    use crate::varint::write_varint;
    use std::net::TcpListener;
    use std::thread::JoinHandle;

    /// The isolated HashedStack lane's encoder (the write mirror's test
    /// surface — the packet is the real unit, the lane's bytes are the
    /// dump's own fixtures).
    fn encode_hashed_stack(stack: &HashedStack) -> Result<Vec<u8>> {
        let mut p = Vec::new();
        write_hashed_stack(&mut p, stack)?;
        Ok(p)
    }

    #[test]
    fn game_event_wire_word_the_clinit_order_pins() {
        // THE PACKET ID: the generated table's runtime fact (the 26.2
        // bootstrapped protocol dump) — `minecraft:game_event` id 38.
        assert_eq!(clientbound::GAME_EVENT, 38);
        // THE $Type VOCABULARY: the outer clinit's `new Type(i)` +
        // putstatic walk banks the field order verbatim — ids 0..13 in
        // the NO_RESPAWN_BLOCK_AVAILABLE .. LEVEL_CHUNKS_LOAD_START
        // order the raw disasm keeps.
        assert_eq!(game_event::NO_RESPAWN_BLOCK_AVAILABLE, 0);
        assert_eq!(game_event::START_RAINING, 1);
        assert_eq!(game_event::STOP_RAINING, 2);
        assert_eq!(game_event::CHANGE_GAME_MODEL, 3);
        assert_eq!(game_event::WIN_GAME, 4);
        assert_eq!(game_event::DEMO_EVENT, 5);
        assert_eq!(game_event::PLAY_ARROW_HIT_SOUND, 6);
        assert_eq!(game_event::RAIN_LEVEL_CHANGE, 7);
        assert_eq!(game_event::THUNDER_LEVEL_CHANGE, 8);
        assert_eq!(game_event::PUFFER_FISH_STING, 9);
        assert_eq!(game_event::GUARDIAN_ELDER_EFFECT, 10);
        assert_eq!(game_event::IMMEDIATE_RESPAWN, 11);
        assert_eq!(game_event::LIMITED_CRAFTING, 12);
        assert_eq!(game_event::LEVEL_CHUNKS_LOAD_START, 13);
        // THE WIRE SHAPE: varint id + u8 event + f32 param — the
        // FriendlyByteBuf read walk (`readUnsignedByte` for the TYPES
        // lookup, `readFloat` for the param). The rain-level change the
        // ClientLevel state answers: event 7, param 0.6875.
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::GAME_EVENT as i32);
        p.push(game_event::RAIN_LEVEL_CHANGE);
        p.extend_from_slice(&0.6875f32.to_be_bytes());
        let ev = parse_game_event(&p).expect("the weather word decodes");
        assert_eq!(ev.event, game_event::RAIN_LEVEL_CHANGE);
        assert!((ev.param - 0.6875).abs() < 1e-9);
        // the pump answers the same word (the dispatch arm the match
        // keeps) — a stop-rain event the model's setRainLevel(0.0F)
        // walk consumes
        let mut s = Vec::new();
        write_varint(&mut s, clientbound::GAME_EVENT as i32);
        s.push(game_event::STOP_RAINING);
        s.extend_from_slice(&0.0f32.to_be_bytes());
        match parse_game_event(&s).expect("stop-rain decodes") {
            GameEvent { event, param } => {
                assert_eq!(event, game_event::STOP_RAINING);
                assert_eq!(param, 0.0);
            }
        }
        // THE TRUNCATED BODY the ensure keeps: the id alone is not the
        // packet — the five-byte tail the read ctor demands
        let mut t = Vec::new();
        write_varint(&mut t, clientbound::GAME_EVENT as i32);
        t.push(game_event::START_RAINING);
        assert!(parse_game_event(&t).is_err(), "the float tail missing");
        // THE WRONG ID the honest refusal keeps
        let mut w = Vec::new();
        write_varint(&mut w, clientbound::KEEP_ALIVE as i32);
        assert!(parse_game_event(&w).is_err(), "not a game event");
    }

    #[test]
    fn change_difficulty_wire_word_the_composite_codec_pins() {
        // THE PACKET ID: the generated table's runtime fact (the 26.2
        // bootstrapped protocol dump) — `minecraft:change_difficulty`
        // clientbound id 10.
        assert_eq!(clientbound::CHANGE_DIFFICULTY, 10);
        // THE WIRE SHAPE (javap the 26.2 STREAM_CODEC composite walk —
        // the Difficulty.STREAM_CODEC idMapper's VARINT + the
        // ByteBufCodecs.BOOL byte the record's component order keeps):
        // the Hard dial (id 3) + the locked bit the lock lane speaks.
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CHANGE_DIFFICULTY as i32);
        write_varint(&mut p, 3);
        p.push(1);
        let ev = parse_change_difficulty(&p).expect("the level-data word decodes");
        assert_eq!(ev.difficulty, 3, "the Hard id the idMapper banks");
        assert!(ev.locked, "the locked bit the BOOL byte keeps");
        // THE UNLOCKED PAIR (the Normal dial, the byte the readBoolean
        // `!= 0` law answers false on)
        let mut u = Vec::new();
        write_varint(&mut u, clientbound::CHANGE_DIFFICULTY as i32);
        write_varint(&mut u, 2);
        u.push(0);
        let ev = parse_change_difficulty(&u).expect("the unlocked word decodes");
        assert_eq!(ev.difficulty, 2, "the Normal id the same codec keeps");
        assert!(!ev.locked, "the unlocked bit the zero byte wears");
        // THE NONZERO BOOL the readBoolean law answers true (any
        // nonzero byte the vanilla read keeps — the byte IS the flag)
        let mut n = Vec::new();
        write_varint(&mut n, clientbound::CHANGE_DIFFICULTY as i32);
        write_varint(&mut n, 0);
        n.push(0x7F);
        let ev = parse_change_difficulty(&n).expect("the peaceful word decodes");
        assert_eq!(ev.difficulty, 0, "the Peaceful id the table's head keeps");
        assert!(ev.locked, "the nonzero byte the readBoolean law speaks true");
        // THE UNKNOWN DIFFICULTY the strict gate refuses (the
        // byIdOrThrow family's own throw law — id 4 is no variant)
        let mut x = Vec::new();
        write_varint(&mut x, clientbound::CHANGE_DIFFICULTY as i32);
        write_varint(&mut x, 4);
        x.push(0);
        assert!(parse_change_difficulty(&x).is_err(), "id 4 no variant");
        // THE TRUNCATED BODY the ensure keeps: the id + difficulty
        // alone is not the packet — the BOOL byte the ctor demands
        let mut t = Vec::new();
        write_varint(&mut t, clientbound::CHANGE_DIFFICULTY as i32);
        write_varint(&mut t, 1);
        assert!(parse_change_difficulty(&t).is_err(), "the locked byte missing");
        // THE WRONG ID the honest refusal keeps
        let mut w = Vec::new();
        write_varint(&mut w, clientbound::KEEP_ALIVE as i32);
        assert!(parse_change_difficulty(&w).is_err(), "not a change difficulty");
    }

    #[test]
    fn player_abilities_wire_word_the_read_ctor_pins() {
        // THE PACKET ID: the generated table's runtime fact (the 26.2
        // bootstrapped protocol dump) — `minecraft:player_abilities`
        // clientbound id 64 (the serverbound sibling the abilities
        // wire's own ONE-byte lane speaks is id 40).
        assert_eq!(clientbound::PLAYER_ABILITIES, 64);
        // THE WIRE SHAPE: varint id + u8 flags + f32 flyingSpeed +
        // f32 walkingSpeed — the read ctor's walk (`readByte`, the
        // four `iand` arms, the two unconditionals). The vanilla
        // Abilities <init> defaults the floats keep: 0.05 / 0.1.
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::PLAYER_ABILITIES as i32);
        p.push(0x00); // the flag byte: all four words silent
        p.extend_from_slice(&0.05f32.to_be_bytes());
        p.extend_from_slice(&0.1f32.to_be_bytes());
        let w = parse_player_abilities(&p).expect("the silent word decodes");
        assert!(!w.invulnerable && !w.flying && !w.mayfly && !w.instabuild);
        assert!((w.flying_speed - 0.05).abs() < 1e-9);
        assert!((w.walking_speed - 0.1).abs() < 1e-9);
        // THE FOUR BITS the ctor's `iand` arms fold: bit 1 the
        // invulnerable word, bit 2 the FLYING state, bit 4 the
        // mayfly grant, bit 8 the instabuild grant. The full byte
        // 0x0F all four arms wear.
        let mut f = Vec::new();
        write_varint(&mut f, clientbound::PLAYER_ABILITIES as i32);
        f.push(0x0F);
        f.extend_from_slice(&0.07f32.to_be_bytes());
        f.extend_from_slice(&0.12f32.to_be_bytes());
        let w = parse_player_abilities(&f).expect("the full word decodes");
        assert!(w.invulnerable && w.flying && w.mayfly && w.instabuild);
        // THE SINGLE BITS the arms isolate: each bit alone its own
        // word wears (the peer arms stay silent)
        for (bit, name) in [(1u8, "invulnerable"), (2, "flying"), (4, "mayfly"), (8, "instabuild")] {
            let mut b = Vec::new();
            write_varint(&mut b, clientbound::PLAYER_ABILITIES as i32);
            b.push(bit);
            b.extend_from_slice(&0.0f32.to_be_bytes());
            b.extend_from_slice(&0.0f32.to_be_bytes());
            let w = parse_player_abilities(&b).expect("the single bit decodes");
            let (i, fl, m, s) = (
                w.invulnerable,
                w.flying,
                w.mayfly,
                w.instabuild,
            );
            match name {
                "invulnerable" => assert!(i && !fl && !m && !s),
                "flying" => assert!(!i && fl && !m && !s),
                "mayfly" => assert!(!i && !fl && m && !s),
                _ => assert!(!i && !fl && !m && s),
            }
        }
        // THE MAYBUILD BIT (16) the read ctor never reads: the byte
        // 0x10 all four words silent — the server-side Abilities
        // carries it, the client's packet never folds it
        let mut mb = Vec::new();
        write_varint(&mut mb, clientbound::PLAYER_ABILITIES as i32);
        mb.push(0x10);
        mb.extend_from_slice(&0.0f32.to_be_bytes());
        mb.extend_from_slice(&0.0f32.to_be_bytes());
        let w = parse_player_abilities(&mb).expect("the maybuild byte decodes");
        assert!(!w.invulnerable && !w.flying && !w.mayfly && !w.instabuild);
        // THE TRUNCATED BODY the ensure keeps: the id + flag alone
        // is not the packet — the eight-byte float tail the read
        // ctor demands
        let mut t = Vec::new();
        write_varint(&mut t, clientbound::PLAYER_ABILITIES as i32);
        t.push(0x02);
        assert!(parse_player_abilities(&t).is_err(), "the float tail missing");
        // THE WRONG ID the honest refusal keeps
        let mut w2 = Vec::new();
        write_varint(&mut w2, clientbound::KEEP_ALIVE as i32);
        w2.push(0x00);
        w2.extend_from_slice(&0.0f32.to_be_bytes());
        w2.extend_from_slice(&0.0f32.to_be_bytes());
        assert!(parse_player_abilities(&w2).is_err(), "not an abilities packet");
    }

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
    fn level_event_parses_the_int_long_int_bool_layout() {
        assert_eq!(clientbound::LEVEL_EVENT, 46, "level event (generated id)");
        assert_eq!(GROWTH_EFFECT, 1505, "the growth effect the useOn walk speaks");
        assert_eq!(
            EYE_OF_ENDER_SHATTER_EFFECT,
            2003,
            "the eye-of-ender shatter the landing tail broadcasts"
        );
        assert_eq!(
            DESTROY_BLOCK_EFFECT,
            2001,
            "the block-break effect the server's own dig broadcast speaks"
        );
        assert_eq!(
            SHOOT_SMOKE_EFFECT,
            2000,
            "the shoot-smoke effect the dispenser's own playAnimation speaks"
        );
        assert_eq!(
            SHOOT_WHITE_SMOKE_EFFECT,
            2010,
            "the white-smoke effect the crafter's own eject speaks"
        );
        assert_eq!(
            PARTICLES_AND_SOUND_WAX_ON_EFFECT,
            3003,
            "the wax-on effect the honeycomb's own useOn speaks"
        );
        assert_eq!(
            PARTICLES_WAX_OFF_EFFECT,
            3004,
            "the wax-off effect the axe's own scrape walk speaks"
        );
        assert_eq!(
            PARTICLES_SCRAPE_EFFECT,
            3005,
            "the scrape effect the axe's own oxide walk speaks"
        );
        assert_eq!(
            END_GATEWAY_SPAWN_EFFECT,
            3000,
            "the gateway-spawn effect the dragon fight's own gateway walk speaks"
        );
        assert_eq!(
            ELECTRIC_SPARK_EFFECT,
            3002,
            "the electric-spark effect the lightning's own copper clean speaks"
        );
        assert_eq!(
            PARTICLES_SCULK_CHARGE_EFFECT,
            3006,
            "the sculk-charge effect the spreader's own decay walk speaks"
        );
        assert_eq!(
            PARTICLES_SCULK_SHRIEK_EFFECT,
            3007,
            "the sculk-shriek effect the shrieker's own reply walk speaks"
        );
        assert_eq!(
            PARTICLES_AND_SOUND_BRUSH_BLOCK_COMPLETE_EFFECT,
            3008,
            "the brush-complete effect the brushable block entity's own completion speaks"
        );
        assert_eq!(
            PARTICLES_EGG_CRACK_EFFECT,
            3009,
            "the egg-crack effect the sniffer egg's own placement speaks"
        );
        assert_eq!(
            PARTICLES_TRIAL_SPAWNER_SPAWN_EFFECT,
            3011,
            "the trial-spawner spawn effect the spawner's own mob spawn speaks"
        );
        assert_eq!(
            ENDER_DRAGON_GROWL_EFFECT,
            3001,
            "the ender-dragon growl effect the plugins' broadcast keeps"
        );
        assert_eq!(
            SPLASH_POTION_EFFECT,
            2002,
            "the splash-potion effect the server's own potion impact speaks"
        );
        assert_eq!(
            INSTANT_SPLASH_EFFECT,
            2007,
            "the instant-splash effect the same arm's branch speaks"
        );
        assert_eq!(
            MOB_SPAWN_EFFECT,
            2004,
            "the mob-spawn cloud the spawner's own broadcast speaks"
        );
        assert_eq!(
            BEE_GROWTH_EFFECT,
            2011,
            "the bee-growth effect the pollinator's own tick speaks"
        );
        assert_eq!(
            TURTLE_EGG_PLACEMENT_EFFECT,
            2012,
            "the turtle-egg placement the egg-lay's own onPlace speaks"
        );
        assert_eq!(
            SMASH_ATTACK_EFFECT,
            2013,
            "the smash-attack burst the mace knockback's own head speaks"
        );
        assert_eq!(
            DRAGON_FIREBALL_SPLASH_EFFECT,
            2006,
            "the dragon-fireball splash the fireball's own onHit speaks"
        );
        assert_eq!(
            WATER_EVAPORATING_EFFECT,
            2009,
            "the water evaporating the wet sponge's own onPlace speaks"
        );
        assert_eq!(
            DRAGON_BLOCK_BREAK_EFFECT,
            2008,
            "the dragon block break the dragon's own checkWalls speaks"
        );
        assert_eq!(
            SOUND_DISPENSER_DISPENSE_EFFECT,
            1000,
            "the dispenser dispense voice the default behavior's own tail speaks"
        );
        assert_eq!(
            SOUND_DISPENSER_FAIL_EFFECT,
            1001,
            "the dispenser fail voice the dispenseFrom fail branches speak"
        );
        assert_eq!(
            SOUND_DISPENSER_PROJECTILE_LAUNCH_EFFECT,
            1002,
            "the dispenser launch voice the projectile behavior's own tail speaks"
        );
        assert_eq!(
            FIREWORK_ROCKET_SHOOT_EFFECT,
            1004,
            "the firework shoot voice the dispense config's own override speaks"
        );
        assert_eq!(
            SOUND_EXTINGUISH_FIRE_EFFECT,
            1009,
            "the fire-extinguish voice the fire block's own break speaks"
        );
        assert_eq!(
            SOUND_PLAY_JUKEBOX_SONG_EFFECT,
            1010,
            "the jukebox-play voice the registry-resolve case speaks"
        );
        assert_eq!(
            SOUND_STOP_JUKEBOX_SONG_EFFECT,
            1011,
            "the jukebox-stop voice the tracked-record case speaks"
        );
        assert_eq!(
            SOUND_CHORUS_GROW_EFFECT,
            1033,
            "the chorus grow voice the flower's own placeGrownFlower speaks"
        );
        assert_eq!(
            SOUND_CHORUS_DEATH_EFFECT,
            1034,
            "the chorus death voice the flower's own placeDeadFlower speaks"
        );
        assert_eq!(
            SOUND_BREWING_STAND_BREW_EFFECT,
            1035,
            "the brewing voice the stand's own doBrew speaks"
        );
        assert_eq!(
            SOUND_CRAFTER_CRAFT_EFFECT,
            1049,
            "the crafter craft voice the dispenseItem's own success speaks"
        );
        assert_eq!(
            SOUND_CRAFTER_FAIL_EFFECT,
            1050,
            "the crafter fail voice the dispenseFrom's own empty slots speak"
        );
        assert_eq!(
            COMPOSTER_FILL_EFFECT,
            1500,
            "the composter fill voice the useItemOn's own tail speaks"
        );
        assert_eq!(
            LAVA_FIZZ_EFFECT,
            1501,
            "the lava fizz voice the fluid's own fizz tail speaks"
        );
        assert_eq!(
            REDSTONE_TORCH_BURNOUT_EFFECT,
            1502,
            "the torch burnout voice the tick's own tail speaks"
        );
        assert_eq!(
            END_PORTAL_FRAME_FILL_EFFECT,
            1503,
            "the portal-frame fill the eye's own useOn tail speaks"
        );
        assert_eq!(
            DRIPSTONE_DRIP_EFFECT,
            1504,
            "the dripstone drip the fluid-transfer's own tails speak"
        );
        // hand-rolled wire: varint id, i32 BE effect, packed pos long,
        // i32 BE data, bool global — the 26.2 write walk's own order
        let build = |effect: i32, x: i32, y: i32, z: i32, data: i32, global: bool| {
            let mut p = Vec::new();
            write_varint(&mut p, clientbound::LEVEL_EVENT);
            p.extend_from_slice(&effect.to_be_bytes());
            p.extend_from_slice(&test_support::encode_block_pos(x, y, z).to_be_bytes());
            p.extend_from_slice(&data.to_be_bytes());
            p.push(global as u8);
            p
        };
        // the bone-meal walk's own word: growth effect at the crop cell,
        // data 15, positioned (the global flag false)
        let ev = parse_level_event(&build(GROWTH_EFFECT, 12, 64, -7, 15, false)).unwrap();
        assert_eq!(ev.effect, GROWTH_EFFECT);
        assert_eq!((ev.x, ev.y, ev.z), (12, 64, -7), "the packed long sign-extends");
        assert_eq!(ev.data, 15);
        assert!(!ev.global);
        // the negative-coord pair the sign-extension law keeps
        let ev2 = parse_level_event(&build(GROWTH_EFFECT, -3, -60, 9, 15, false)).unwrap();
        assert_eq!((ev2.x, ev2.y, ev2.z), (-3, -60, 9));
        // a GLOBAL event rides the same shape (the flag the byte carries)
        let ev3 = parse_level_event(&build(1032, 0, 0, 0, 0, true)).unwrap();
        assert!(ev3.global);
        // an unknown event id still parses — the honest word the
        // consumer's vocabulary refuses (the render lane's own gate)
        let ev4 = parse_level_event(&build(1030, 1, 2, 3, 4, false)).unwrap();
        assert_eq!(ev4.effect, 1030);
        // trailing bytes are drift
        let mut p5 = build(GROWTH_EFFECT, 0, 0, 0, 15, false);
        p5.push(0x00);
        assert!(parse_level_event(&p5).is_err());
        // a wrong packet id is refused
        let mut p6 = Vec::new();
        write_varint(&mut p6, clientbound::BLOCK_UPDATE);
        p6.extend_from_slice(&1505i32.to_be_bytes());
        assert!(parse_level_event(&p6).is_err());
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

    /// The container click's serverbound lane: the bytes are the
    /// official 26.2 STREAM_CODEC's own output for three real packet
    /// instances (the checked-in `ReflectContainerClick.java` built the
    /// packets and serialized them through the jar's own codec — the
    /// hex below is that dump, byte for byte). The id-prefix byte rides
    /// the house convention (the generated table's 0x12).
    #[test]
    fn the_container_click_lane_speaks_the_dumped_wire() {
        assert_eq!(serverbound::CONTAINER_CLICK, 18, "minecraft:container_click");
        assert_eq!(serverbound::CONTAINER_CLOSE, 19, "minecraft:container_close");
        // the input modes' registration order — the dump's own
        // getEnumConstants() order, the wire's VarInt values
        assert_eq!(ContainerInput::Pickup.wire_ordinal(), 0);
        assert_eq!(ContainerInput::QuickMove.wire_ordinal(), 1);
        assert_eq!(ContainerInput::Swap.wire_ordinal(), 2);
        assert_eq!(ContainerInput::Clone.wire_ordinal(), 3);
        assert_eq!(ContainerInput::Throw.wire_ordinal(), 4);
        assert_eq!(ContainerInput::QuickCraft.wire_ordinal(), 5);
        assert_eq!(ContainerInput::PickupAll.wire_ordinal(), 6);
        // the prediction-free click: window 2, revision 7, the menu's
        // slot 13, the left button, PICKUP, an empty delta list and an
        // empty cursor — the exact bytes the official codec wrote for
        // the same record (`02 07 000d 00 00 0000` + the id prefix)
        let p = encode_container_click(&ContainerClick {
            container_id: 2,
            state_id: 7,
            slot: 13,
            button: 0,
            input: ContainerInput::Pickup,
            changed_slots: Vec::new(),
            carried: HashedStack::Empty,
        })
        .unwrap();
        assert_eq!(
            p,
            vec![0x12, 0x02, 0x07, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x00],
            "the minimal click, byte-equal to the dump"
        );
        // the prediction-laden click: a changed slot 7 (diamond x3 — the
        // wire's registry id 926 the dump resolved) and a carried apple
        // x1 (921): the deltas' SHORT + HashedStack pair shape, the
        // ActualItem's presence byte + VarInt id + VarInt count + the
        // EMPTY hashed patch map (`00 00`)
        let p = encode_container_click(&ContainerClick {
            container_id: 2,
            state_id: 7,
            slot: 13,
            button: 1,
            input: ContainerInput::Pickup,
            changed_slots: vec![(7, HashedStack::item(926, 3))],
            carried: HashedStack::item(921, 1),
        })
        .unwrap();
        assert_eq!(
            p,
            vec![
                0x12, 0x02, 0x07, 0x00, 0x0d, 0x01, 0x00, // the header + PICKUP
                0x01, 0x00, 0x07, 0x01, 0x9e, 0x07, 0x03, 0x00, 0x00, // {7: diamond x3}
                0x01, 0x99, 0x07, 0x01, 0x00, 0x00, // the carried apple x1
            ],
            "the prediction-laden click, byte-equal to the dump"
        );
        // the QUICK_MOVE ordinal rides its registration-order value
        let p = encode_container_click(&ContainerClick {
            container_id: 2,
            state_id: 7,
            slot: 5,
            button: 0,
            input: ContainerInput::QuickMove,
            changed_slots: Vec::new(),
            carried: HashedStack::Empty,
        })
        .unwrap();
        assert_eq!(
            p,
            vec![0x12, 0x02, 0x07, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00],
            "the QUICK_MOVE ordinal, byte-equal to the dump"
        );
        // the close: the window id alone
        assert_eq!(encode_container_close(2), vec![0x13, 0x02]);
    }

    /// The bundle selection's serverbound lane — the
    /// `BundleMouseActions.toggleSelectedBundleItem` tail's own echo.
    /// THE WIRE (javap of the real 26.2 class: the member
    /// `write(FriendlyByteBuf)` writes the record's two fields, the
    /// read ctor's `selectedItemIndex < 0 && selectedItemIndex != -1`
    /// throw the validation the WRITE path never carries): the hovered
    /// slot's MENU id as a VarInt, the selected index as a VarInt —
    /// the id prefix rides the house convention (the generated
    /// SB_BUNDLE_ITEM_SELECTED = 3, the single-byte VarInt).
    #[test]
    fn the_select_bundle_item_lane_speaks_the_dumped_wire() {
        assert_eq!(
            serverbound::BUNDLE_ITEM_SELECTED,
            3,
            "minecraft:bundle_item_selected"
        );
        // the selection walk's own shape: the inventory menu's store
        // slot 12, the selection 0 the first scroll lands
        assert_eq!(
            encode_select_bundle_item(12, 0),
            vec![0x03, 0x0c, 0x00],
            "slot 12, selection 0 — the two VarInts"
        );
        // a deeper slot and a wrapped selection
        assert_eq!(
            encode_select_bundle_item(13, 4),
            vec![0x03, 0x0d, 0x04],
            "slot 13, selection 4"
        );
        // the deselected index the read ctor's law keeps (the -1 the
        // write path never speaks — the walk's wrap lands in [0, n))
        assert_eq!(
            encode_select_bundle_item(2, -1),
            vec![0x03, 0x02, 0xff, 0xff, 0xff, 0xff, 0x0f],
            "selection -1 rides its five-byte VarInt"
        );
    }

    /// The bundle_contents component's VALUE lane — the type-50 walk
    /// the patch dispatch speaks (the 26.2 BundleContents.STREAM_CODEC:
    /// the template list the ItemStackTemplate composite carries — the
    /// item's VarInt, the RAW count lane, the entry's OWN patch; the
    /// AIR entry the ctor's non-empty law refuses, the nesting the
    /// 128-depth lane caps). The bytes are hand-packed over the
    /// codec's own shape, riding a Set Container Slot's stack.
    #[test]
    fn the_bundle_contents_component_speaks_the_wire() {
        assert_eq!(
            data_component_types::BUNDLE_CONTENTS,
            50,
            "the clinit walk's registration order"
        );
        // the flat shape: the bundle stack (item 926) carries two
        // entries — diamond x3 + apple x1, each the EMPTY patch
        let p: Vec<u8> = vec![
            0x14, 0x02, 0x07, 0x00, 0x0d, // the set-slot header: id, window 2, state 7, slot 13
            0x01, 0x9e, 0x07, // the stack: count 1, item 926
            0x01, 0x00, // the patch: added 1, removed 0
            0x32, // the type: bundle_contents (50)
            0x02, // the list: two entries
            0x9e, 0x07, 0x03, 0x00, 0x00, // diamond x3, the empty patch
            0x99, 0x07, 0x01, 0x00, 0x00, // apple x1, the empty patch
        ];
        let s = parse_set_container_slot(&p).unwrap();
        assert_eq!(s.item.count, 1, "the stack the sentinel carries");
        assert_eq!(s.item.item_id, 926);
        let (ty, value) = s.item.patch.added.first().expect("one added component");
        assert_eq!(*ty, data_component_types::BUNDLE_CONTENTS);
        match value {
            ComponentValue::BundleContents(entries) => {
                assert_eq!(entries.len(), 2, "the list the wire speaks");
                assert_eq!(entries[0].item_id, 926);
                assert_eq!(entries[0].count, 3, "the RAW count lane");
                assert!(entries[0].patch.added.is_empty(), "the empty patch");
                assert_eq!(entries[1].item_id, 921);
                assert_eq!(entries[1].count, 1);
            }
            other => panic!("the bundle walk, got {other:?}"),
        }
        // the NESTED shape: the first entry rides a bundle of its own
        // (its patch carries the inner contents — apple x2)
        let p: Vec<u8> = vec![
            0x14, 0x02, 0x07, 0x00, 0x0d,
            0x01, 0x9e, 0x07,
            0x01, 0x00,
            0x32,
            0x02,
            0x9e, 0x07, 0x03, // entry 1: item 926, count 3, THE NESTED PATCH:
            0x01, 0x00, // the added 1 / removed 0
            0x32, // the inner type
            0x01, // the inner list: one entry
            0x99, 0x07, 0x02, 0x00, 0x00, // apple x2, the empty patch
            0x99, 0x07, 0x01, 0x00, 0x00, // entry 2: apple x1, the empty patch
        ];
        let s = parse_set_container_slot(&p).unwrap();
        let (_, value) = s.item.patch.added.first().unwrap();
        match value {
            ComponentValue::BundleContents(entries) => {
                assert_eq!(entries.len(), 2);
                let inner = &entries[0].patch.added;
                assert_eq!(inner.len(), 1, "the entry's own patch spoke");
                assert_eq!(inner[0].0, data_component_types::BUNDLE_CONTENTS);
                match &inner[0].1 {
                    ComponentValue::BundleContents(items) => {
                        assert_eq!(items.len(), 1, "the inner bundle's own list");
                        assert_eq!(items[0].item_id, 921);
                        assert_eq!(items[0].count, 2);
                    }
                    other => panic!("the inner walk, got {other:?}"),
                }
            }
            other => panic!("the bundle walk, got {other:?}"),
        }
        // THE AIR REFUSAL (the template ctor's own law): an entry with
        // item id 0 refuses the read
        let p: Vec<u8> = vec![
            0x14, 0x02, 0x07, 0x00, 0x0d,
            0x01, 0x9e, 0x07,
            0x01, 0x00,
            0x32,
            0x01, // one entry
            0x00, 0x01, 0x00, 0x00, // item 0 (air) — the refusal speaks
        ];
        assert!(
            parse_set_container_slot(&p).is_err(),
            "the air entry the ctor refuses"
        );
        // THE DEPTH CAP (the NBT reader's own discipline): 200 nested
        // bundles refuse the read — the stack the unframed wire's
        // reader never exhausts
        let mut value: Vec<u8> = vec![0]; // the innermost: an empty list
        for _ in 0..200 {
            let mut entry = vec![0x9e, 0x07, 0x01]; // item 926, count 1
            entry.extend_from_slice(&[0x01, 0x00]); // the entry's patch: added 1, removed 0
            entry.push(0x32); // the inner type
            entry.extend_from_slice(&value); // the inner list
            value = vec![0x01]; // the next list: one entry
            value.extend_from_slice(&entry);
        }
        let mut p: Vec<u8> = vec![0x14, 0x02, 0x07, 0x00, 0x0d, 0x01, 0x9e, 0x07, 0x01, 0x00, 0x32];
        p.extend_from_slice(&value);
        assert!(
            parse_set_container_slot(&p).is_err(),
            "the nesting past the depth lane refuses"
        );
        // the same nest INSIDE the cap still reads (the boundary the
        // cap's own law keeps — 100 deep, well under the 128)
        let mut value: Vec<u8> = vec![0];
        for _ in 0..100 {
            let mut entry = vec![0x9e, 0x07, 0x01];
            entry.extend_from_slice(&[0x01, 0x00]);
            entry.push(0x32);
            entry.extend_from_slice(&value);
            value = vec![0x01];
            value.extend_from_slice(&entry);
        }
        let mut p: Vec<u8> = vec![0x14, 0x02, 0x07, 0x00, 0x0d, 0x01, 0x9e, 0x07, 0x01, 0x00, 0x32];
        p.extend_from_slice(&value);
        let s = parse_set_container_slot(&p).unwrap();
        let (_, value) = s.item.patch.added.first().unwrap();
        assert!(
            matches!(value, ComponentValue::BundleContents(_)),
            "the in-cap nest reads whole"
        );
    }

    /// The serverbound Set Carried Item lane — the number keys'
    /// serverbound echo. The checked-in `ReflectSetCarriedItem.java`
    /// built the REAL packet records over the selection domain's ends
    /// and middle (`ctor: (int)`) and serialized them through the jar's
    /// own STREAM_CODEC — the hex below is that dump, byte for byte.
    /// THE AUDIT'S OWN CATCH: the payload is a FIXED BIG-ENDIAN SHORT —
    /// the two-byte shapes `0000` / `0003` / `0008` refuse a VarInt
    /// reading (slot 3's VarInt would be the single `03`); the
    /// wiki-era VarInt memory is wrong.
    #[test]
    fn the_set_carried_item_lane_speaks_the_dumped_wire() {
        // the id prefix rides the house convention (the generated
        // SB_SET_CARRIED_ITEM = 53 = 0x35, the single-byte VarInt), the
        // payload the dump's own bytes
        assert_eq!(
            encode_set_carried_item(0),
            vec![0x35, 0x00, 0x00],
            "slot 0: 00 00 — the BE short"
        );
        assert_eq!(
            encode_set_carried_item(3),
            vec![0x35, 0x00, 0x03],
            "slot 3: 00 03 — a VarInt would be the single 03"
        );
        assert_eq!(
            encode_set_carried_item(8),
            vec![0x35, 0x00, 0x08],
            "slot 8: the selection domain's own end"
        );
    }

    /// The middle-click pick's serverbound pair — the BLOCK half the
    /// survival client speaks, the ENTITY arm the protocol keeps
    /// whole. The bytes are the javap STREAM_CODEC composites' own
    /// composition (the real 26.2 classes): Pick Item From Block =
    /// the generated id 36 = 0x24 (the single-byte VarInt), the
    /// BlockPos BE i64 (the asLong 38/12/0 pack — (1, 64, 2) folds to
    /// (1<<38) | (2<<12) | 64 = 0x0000_0040_0000_2040), the includeData
    /// boolean; Pick Item From Entity = the id 37 = 0x25, the entity
    /// id VarInt (100 rides the single 0x64 — under the 128 the
    /// continuation bit never lights), the boolean.
    #[test]
    fn the_pick_item_lanes_speak_the_codec_composites() {
        // the BLOCK half: id 36, the packed pos, the Ctrl flag FALSE
        assert_eq!(
            encode_pick_item_from_block(1, 64, 2, false),
            vec![0x24, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x20, 0x40, 0x00],
            "pos (1, 64, 2): the asLong pack then the false byte"
        );
        // the Ctrl form — the includeData byte lights
        assert_eq!(
            encode_pick_item_from_block(1, 64, 2, true),
            vec![0x24, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x20, 0x40, 0x01],
            "the same pos, the includeData byte the Ctrl flag rides"
        );
        // the ZERO pos — the origin's own i64 (the release lane's
        // BlockPos.ZERO the player-action walk keeps company with)
        assert_eq!(
            encode_pick_item_from_block(0, 0, 0, false),
            vec![0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            "the origin packs to the zero i64"
        );
        // the ENTITY arm: id 37, the VarInt id, the boolean
        assert_eq!(
            encode_pick_item_from_entity(100, true),
            vec![0x25, 0x64, 0x01],
            "entity 100: the single-byte VarInt then the true byte"
        );
        // a wide id — 300 needs the continuation bit (0xAC 0x02)
        assert_eq!(
            encode_pick_item_from_entity(300, false),
            vec![0x25, 0xAC, 0x02, 0x00],
            "entity 300: the two-byte VarInt then the false byte"
        );
    }

    /// The use-item arc's serverbound echo — the two packets the
    /// right-mouse hold speaks. The hex is the javap `write` methods'
    /// own composition (the real 26.2 ServerboundUseItemPacket and
    /// ServerboundPlayerActionPacket classes): Use Item = the hand
    /// VarInt (the InteractionHand ordinal), the sequence VarInt, the
    /// yRot/xRot BE f32s; Player Action = the action VarInt (the
    /// static init's RELEASE_USE_ITEM = iconst_5), the BlockPos BE
    /// i64 (the release's ZERO), the direction byte (the release's
    /// DOWN = 0), the sequence VarInt.
    #[test]
    fn the_use_item_lane_speaks_the_javap_wire() {
        // the ids: SB_USE_ITEM = 67 = 0x43, SB_PLAYER_ACTION = 41 = 0x29
        // (the single-byte VarInts); MAIN_HAND = 0, sequence 7 = 0x07,
        // yaw 90.0 = 42 B4 00 00, pitch 0.0 = 00 00 00 00
        assert_eq!(
            encode_use_item(0, 7, 90.0, 0.0),
            vec![0x43, 0x00, 0x07, 0x42, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            "hand 0, seq 7, yaw 90, pitch 0 — the write order the bytecode speaks"
        );
        // the off-hand ordinal rides 1; the negative pitch is the BE
        // f32's sign bit (−45.0 = C2 34 00 00)
        assert_eq!(
            encode_use_item(1, 128, 0.0, -45.0),
            vec![
                0x43, 0x01, // off hand
                0x80, 0x01, // sequence 128 = the two-byte VarInt
                0x00, 0x00, 0x00, 0x00, // yaw 0.0
                0xC2, 0x34, 0x00, 0x00, // pitch −45.0
            ],
            "the off-hand lane, the VarInt growth, the negative f32"
        );
        // the release: action 5, pos 0 (the BE i64), direction 0, seq 8
        assert_eq!(
            encode_player_action_release(8),
            vec![
                0x29, 0x05, // RELEASE_USE_ITEM's ordinal
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // BlockPos.ZERO
                0x00, // Direction.DOWN
                0x08, // the sequence
            ],
            "the release's own constants — the javap ctor's arguments"
        );
    }

    /// THE DIG LANE's wire — the destroy arc's own Player Action
    /// actions (START/ABORT/STOP) and the swing. The hex is the javap
    /// read order of the real 26.2 ServerboundPlayerActionPacket: the
    /// action VarInt, the BlockPos BE i64 (the 38/12/0 pack), the
    /// direction as a SINGLE BYTE (readUnsignedByte -> from3DDataValue,
    /// NOT a VarInt), the sequence VarInt; the swing rides
    /// ServerboundSwingPacket's single writeEnum hand VarInt
    /// (SB_SWING = 63 = 0x3F).
    #[test]
    fn the_dig_lane_speaks_the_javap_wire() {
        // START at (1, 64, -2) on the UP face (the 3D value 1), seq 9:
        // the pos pack's own spans — x<<38 | z<<12 | y (the vanilla
        // BlockPos.asLong's X_OFFSET 38 / Z_OFFSET 12 / Y_OFFSET 0, the
        // HORIZONTAL spans 26 bits wide — Y rides LOW, z's 26 bits
        // carry the negative coordinate's own sign)
        let pos = (1i64 << 38) | (((-2i64) & 0x3FF_FFFF) << 12) | (64i64 & 0xFFF);
        let mut expect = vec![0x29, 0x00]; // START_DESTROY_BLOCK
        expect.extend_from_slice(&pos.to_be_bytes());
        expect.push(0x01); // Direction.UP's 3D data value, one raw byte
        expect.push(0x09); // the sequence
        assert_eq!(
            encode_player_action(0, 1, 64, -2, 1, 9),
            expect,
            "the START's own composition — the direction rides ONE byte"
        );
        // ABORT the old cell on the retarget (the ordinal 1), the
        // NEW face rides (the vanilla quirk the state machine speaks)
        let pos = (5i64 << 38) | ((3i64 & 0x3FF_FFFF) << 12) | (70i64 & 0xFFF);
        let mut expect = vec![0x29, 0x01]; // ABORT_DESTROY_BLOCK
        expect.extend_from_slice(&pos.to_be_bytes());
        expect.push(0x04); // WEST's 3D data value
        expect.push(0x0A); // the sequence
        assert_eq!(encode_player_action(1, 5, 70, 3, 4, 10), expect);
        // STOP on the completion (the ordinal 2), DOWN's 0, the
        // two-byte sequence VarInt (129 = 0x81 0x01)
        let pos = (-3i64 & 0x3FF_FFFF) << 38; // x = -3, y = z = 0
        let mut expect = vec![0x29, 0x02]; // STOP_DESTROY_BLOCK
        expect.extend_from_slice(&pos.to_be_bytes());
        expect.push(0x00); // Direction.DOWN
        expect.extend_from_slice(&[0x81, 0x01]); // 129
        assert_eq!(
            encode_player_action(2, -3, 0, 0, 0, 129),
            expect,
            "the completion's STOP, the grown VarInt"
        );
        // THE SWING: the packet id + the hand, nothing else (the javap
        // write's single field)
        assert_eq!(
            encode_swing(0),
            vec![0x3F, 0x00],
            "MAIN_HAND's own beat"
        );
        assert_eq!(encode_swing(1), vec![0x3F, 0x01], "the off-hand ordinal");
    }

    /// The ATTACK lane's serverbound echo — the 26.2's dedicated
    /// ServerboundAttackPacket the gameMode.attack walk sends BEFORE
    /// the local prediction. The hex is the real class's own
    /// composition (javap: the record `(int entityId)`, the clinit's
    /// composite loading ByteBufCodecs.VAR_INT alone — the
    /// single-field StreamCodec the two-Function call speaks).
    #[test]
    fn the_attack_lane_speaks_the_record_varint() {
        // the ids: SB_ATTACK = 1 = 0x01 (the single-byte VarInt)
        assert_eq!(
            encode_attack(99),
            vec![0x01, 0x63],
            "entity id 99: the record's ONE VarInt payload"
        );
        // the two-byte VarInt form (128 = 0x80 0x01 — the continuation
        // bit the high byte's own set speaks)
        assert_eq!(
            encode_attack(128),
            vec![0x01, 0x80, 0x01],
            "entity id 128: the VarInt's own two-byte form"
        );
        // the id 1 shape — the packet id's own byte and the payload's
        // own byte the same value, the two the adjacency separates
        assert_eq!(
            encode_attack(1),
            vec![0x01, 0x01],
            "the id byte and the payload byte the same value — the two speak"
        );
    }

    /// The interact record's own wire — the composite order the clinit
    /// speaks (VarInt id, VarInt hand, the LpVec3 cursor, the Bool
    /// shift) and the cursor segment the LpVec3 codec's own walk
    /// produces byte for byte.
    #[test]
    fn the_interact_speaks_the_flat_record() {
        // the ids: SB_INTERACT = 26 = 0x1A (the single-byte VarInt);
        // THE ZERO CURSOR: the LpVec3's zero form the 2^-15 threshold
        // keeps — the whole packet the FIVE bytes the record's own
        // order speaks
        assert_eq!(
            encode_interact(1, 0, (0.0, 0.0, 0.0), false),
            vec![0x1A, 0x01, 0x00, 0x00, 0x00],
            "the zero cursor's single 0x00 the LpVec3 zero form speaks"
        );
        // the cursor segment rides the SAME bytes the codec's own walk
        // produces — the exact roundtrip the unit axis' midpoints keep
        // (pack(±1.0) = 32766/0, pack(0.0) = the 16383 midpoint), the
        // consumed count the variable tail the Bool byte follows
        let packet = encode_interact(99, 1, (1.0, 0.0, -1.0), true);
        assert_eq!(
            &packet[..3],
            &[0x1A, 0x63, 0x01],
            "id 26, entity 99, the OFF_HAND ordinal"
        );
        let (decoded, used) = parse_lp_vec3(&packet[3..]).expect("the cursor segment");
        assert_eq!(
            decoded,
            (1.0, 0.0, -1.0),
            "the exact unpack the axis' own fields keep"
        );
        assert_eq!(
            used,
            packet.len() - 4,
            "the LpVec3 tail the record's ONE Bool byte follows"
        );
        assert_eq!(*packet.last().expect("the tail"), 0x01, "the shift byte");
    }

    /// The low-precision Vec3's own wire — the hex the LpVec3 dump's
    /// walk speaks (the zero form, the 6-byte word, the continuation
    /// scale's VarInt tail, the 2^34−1 bound's signed-VarInt roundtrip).
    #[test]
    fn the_lp_vec3_speaks_the_packed_wire() {
        // THE ZERO FORM: the max below the 2^-15 threshold the #66
        // double speaks — the single byte the read's `if (b0 == 0)`
        // answers with Vec3.ZERO
        assert_eq!(encode_lp_vec3(0.0, 0.0, 0.0), vec![0x00]);
        assert_eq!(
            encode_lp_vec3(1.0e-6, 0.0, 0.0),
            vec![0x00],
            "the sub-threshold max the zero form keeps"
        );
        // THE UNIT AXIS: l = 1 (no continuation — the head's own bit 0
        // rides), fx = 32766 the pack(1.0) speaks, fy/fz the midpoint
        // 16383 the pack(0.0) keeps
        assert_eq!(
            encode_lp_vec3(1.0, 0.0, 0.0),
            vec![0xF1, 0xFF, 0x7F, 0xFE, 0xFF, 0xFF]
        );
        // THE NEGATIVE ARM: fx = pack(-1.0) = 0 — the head's bit 0 the
        // only low word
        assert_eq!(
            encode_lp_vec3(-1.0, 0.0, 0.0),
            vec![0x01, 0x00, 0x7F, 0xFE, 0xFF, 0xFC]
        );
        // THE CONTINUATION: 1000.5 → l = 1001, the flag bit 2 the head
        // 0xB5 = 1|4|(1001&3) speaks, the VarInt(250) tail the l>>2
        // carries
        assert_eq!(
            encode_lp_vec3(1000.5, 0.0, 0.0),
            vec![0xB5, 0xFF, 0x7F, 0xFE, 0xFF, 0xFF, 0xFA, 0x01]
        );
        // THE 2^34−1 BOUND: l = 17179869183, the l>>2 = 0xFFFFFFFF the
        // i32 overflow rides — the 5-byte VarInt(-1) the 0xFFFFFFFF
        // mask un-folds on the read
        assert_eq!(
            encode_lp_vec3(17179869183.0, 0.0, 0.0),
            vec![
                0xF7, 0xFF, 0x7F, 0xFE, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F
            ]
        );
        // THE ROUNDTRIP: every vector the encode speaks, the read
        // answers within the quantization's own half-step bound (the
        // scale l's 2/32766 step the pack/unpack pair rides); the
        // consumed count the form's own length keeps
        for (x, y, z) in [
            (1.0, 0.0, 0.0),
            (-1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
            (1000.5, 0.0, 0.0),
            (-12.7, 64.0, 3.25),
            (17179869183.0, 0.0, 0.0),
            (-17179869183.0, 0.0, 0.0),
            (0.5, -0.5, 0.25),
            (3.051944088384301e-05, 0.0, 0.0),
        ] {
            let wire = encode_lp_vec3(x, y, z);
            let ((rx, ry, rz), used) = parse_lp_vec3(&wire).unwrap();
            assert_eq!(used, wire.len(), "the consumed bytes the form's own length keeps");
            let max = x.abs().max(y.abs()).max(z.abs());
            if max < 3.051944088384301e-05 {
                assert_eq!(
                    (rx, ry, rz),
                    (0.0, 0.0, 0.0),
                    "the zero form the read answers exactly"
                );
                continue;
            }
            let half_step = max.ceil() * 2.0 / 32766.0 / 2.0;
            assert!(
                (rx - x).abs() <= half_step,
                "x {x} -> {rx} beyond the half-step {half_step}"
            );
            assert!(
                (ry - y).abs() <= half_step,
                "y {y} -> {ry} beyond the half-step {half_step}"
            );
            assert!(
                (rz - z).abs() <= half_step,
                "z {z} -> {rz} beyond the half-step {half_step}"
            );
        }
        // the sanitize's own arms the walk speaks: the NaN the zero
        // keeps, the overflow the clamp answers
        assert_eq!(encode_lp_vec3(f64::NAN, 0.0, 0.0), vec![0x00]);
        assert_eq!(
            encode_lp_vec3(1.0e30, 0.0, 0.0),
            encode_lp_vec3(17179869183.0, 0.0, 0.0),
            "the overflow the 2^34-1 clamp answers"
        );
    }

    /// The use-item-ON lane's serverbound echo — the block half of the
    /// use arc. The hex is the javap `write` composition of the real
    /// 26.2 ServerboundUseItemOnPacket + FriendlyByteBuf.
    /// writeBlockHitResult: the BlockPos BE i64 (asLong's 38/12/0 pack),
    /// the direction VarInt (the writeEnum ordinal), the cursor as the
    /// hit location minus the block origin (double math, float cast),
    /// the inside + world-border booleans; then the sequence VarInt.
    #[test]
    fn the_use_item_on_lane_speaks_the_javap_wire() {
        // THE PACK: asLong's own spans — the 26-bit HORIZONTAL fields
        // (x and z, the PACKED_HORIZONTAL_LENGTH's 26) with the 12-bit
        // y at the low end; a negative coordinate's two's-complement
        // low bits ride the mask (the vanilla law for in-range coords)
        assert_eq!(pack_block_pos(0, 0, 0), 0, "the ZERO block");
        assert_eq!(pack_block_pos(1, 2, 3), (1i64 << 38) | (3i64 << 12) | 2);
        assert_eq!(pack_block_pos(-1, 0, -1), (0x3FF_FFFFi64 << 38) | (0x3FF_FFFFi64 << 12));
        // the wire: SB_USE_ITEM_ON = 66 = 0x42 (the single-byte VarInt),
        // hand 0, the block (1, 2, 3) packed as (1<<38)|(3<<12)|2, the
        // face WEST=4, the cursor (0.5, 1.0, 0.25), the flags, seq 9
        let packed = (1i64 << 38) | (3i64 << 12) | 2;
        let be = packed.to_be_bytes();
        assert_eq!(
            encode_use_item_on(0, 1, 2, 3, 4, (0.5, 1.0, 0.25), false, false, 9),
            vec![
                0x42, 0x00, // the packet id, MAIN_HAND
                be[0], be[1], be[2], be[3], be[4], be[5], be[6], be[7], // the packed pos
                0x04, // the face ordinal (WEST)
                0x3F, 0x00, 0x00, 0x00, // cursor x 0.5
                0x3F, 0x80, 0x00, 0x00, // cursor y 1.0
                0x3E, 0x80, 0x00, 0x00, // cursor z 0.25
                0x00, // inside = false
                0x00, // world border = false
                0x09, // the sequence
            ],
            "the writeBlockHitResult order — the javap body verbatim"
        );
        // the inside flag rides true (the eye-inside law) and the
        // sequence 128 speaks the two-byte VarInt (0x80 0x01)
        let hit = encode_use_item_on(1, 0, 64, 0, 1, (0.0, 0.0, 0.0), true, false, 128);
        assert_eq!(hit[0], 0x42);
        assert_eq!(hit[1], 0x01, "the off hand");
        // the packed pos for (0, 64, 0): y=64 rides the low 12 bits
        assert_eq!(&hit[2..10], &64i64.to_be_bytes(), "y 64 = the zero x/z pack");
        assert_eq!(hit[10], 0x01, "the UP ordinal");
        assert_eq!(&hit[23], &0x01, "inside = true (border rides false)");
        assert_eq!(&hit[hit.len() - 2..], &[0x80, 0x01], "the two-byte sequence VarInt");
    }

    /// The movement wire's hex — the four ServerboundMovePlayer packets
    /// the body's sendPosition dispatch speaks. The shapes are the javap
    /// STREAM_CODEC reads (the three BE doubles when hasPos, the two BE
    /// floats when hasRot, the status byte's packFlags: bit 0 on ground,
    /// bit 1 horizontal collision); the ids are the generated table's
    /// 30..=33.
    #[test]
    fn the_move_player_lanes_speak_the_javap_wire() {
        // PosRot: id 31 = 0x1F, pos (1, 2, 3) BE f64s, yaw 90 = 42 B4 00
        // 00, pitch 0, flags 0x01 (on ground, no horizontal collision)
        let mut want = vec![0x1F];
        want.extend_from_slice(&1.0f64.to_be_bytes());
        want.extend_from_slice(&2.0f64.to_be_bytes());
        want.extend_from_slice(&3.0f64.to_be_bytes());
        want.extend_from_slice(&90.0f32.to_be_bytes());
        want.extend_from_slice(&0.0f32.to_be_bytes());
        want.push(0x01);
        assert_eq!(
            encode_move_player_pos_rot(1.0, 2.0, 3.0, 90.0, 0.0, true, false),
            want,
            "the PosRot shape: 3 f64 + 2 f32 + the status byte"
        );
        // Pos: id 30 = 0x1E, the same pack minus the floats
        let mut want = vec![0x1E];
        want.extend_from_slice(&(-1.5f64).to_be_bytes());
        want.extend_from_slice(&64.0f64.to_be_bytes());
        want.extend_from_slice(&0.25f64.to_be_bytes());
        want.push(0x03); // on ground + horizontal collision
        assert_eq!(
            encode_move_player_pos(-1.5, 64.0, 0.25, true, true),
            want,
            "the Pos shape; the flags pack both bits"
        );
        // Rot: id 32 = 0x20, the two floats, the ground bit alone
        let mut want = vec![0x20];
        want.extend_from_slice(&(-180.0f32).to_be_bytes());
        want.extend_from_slice(&45.5f32.to_be_bytes());
        want.push(0x02); // horizontal collision only (in the air)
        assert_eq!(
            encode_move_player_rot(-180.0, 45.5, false, true),
            want,
            "the Rot shape; the flag bits are the packFlags law"
        );
        // StatusOnly: id 33 = 0x21, the single status byte
        assert_eq!(
            encode_move_player_status_only(false, false),
            vec![0x21, 0x00],
            "the airborne no-collision status"
        );
        assert_eq!(
            encode_move_player_status_only(true, false),
            vec![0x21, 0x01],
            "the landed status"
        );
    }

    /// The HashedStack's FULL patch lane — the non-empty shapes the
    /// container-click round's dump left unrepresentable. The checked-in
    /// `ReflectHashedStack.java` built `HashedStack.ActualItem(Holder,
    /// count, HashedPatchMap)` records over REAL DataComponentTypes (the
    /// codec RESOLVES the keys through the data_component_type registry —
    /// the synthetic int key was refused) and serialized them through the
    /// jar's own STREAM_CODEC — the hex below is that dump, byte for
    /// byte. The deliberate hash values double as WIDTH probes: a
    /// negative hash and a large one refuse a VarInt reading — the hash
    /// lane is a FIXED BIG-ENDIAN int32.
    #[test]
    fn the_hashed_patch_lane_speaks_the_dumped_wire() {
        // the record shapes, straight from the running classes
        // (`HashedStack.ActualItem record components: Holder item, int
        // count, HashedPatchMap components` / `HashedPatchMap record
        // components: Map addedComponents, Set removedComponents` — the
        // dump's own lines; this boot pins DAMAGE = 3, CUSTOM_NAME = 6)
        // the baseline: diamond x64 with the EMPTY patch — the same
        // shape every default-component stack rides (`01 9e 07 40 00 00`)
        let p = encode_hashed_stack(&HashedStack::item(926, 64)).unwrap();
        assert_eq!(
            p,
            vec![0x01, 0x9e, 0x07, 0x40, 0x00, 0x00],
            "the empty patch, byte-equal to the dump"
        );
        // added{DAMAGE:12345}: the type id as a VarInt + the hash as a
        // FIXED BE int32 (`01 9e 07 40 01 03 00 00 30 39 00`)
        let p = encode_hashed_stack(&HashedStack::ActualItem {
            item_id: 926,
            count: 64,
            patch: HashedPatchMap::added(3, 12345),
        })
        .unwrap();
        assert_eq!(
            p,
            vec![
                0x01, 0x9e, 0x07, 0x40, 0x01, 0x03, 0x00, 0x00, 0x30, 0x39, 0x00
            ],
            "the added-only patch, byte-equal to the dump"
        );
        // removed{CUSTOM_NAME}: the type ids as plain VarInts
        // (`01 9e 07 40 00 01 06`)
        let p = encode_hashed_stack(&HashedStack::ActualItem {
            item_id: 926,
            count: 64,
            patch: HashedPatchMap {
                added: Vec::new(),
                removed: vec![6],
            },
        })
        .unwrap();
        assert_eq!(
            p,
            vec![0x01, 0x9e, 0x07, 0x40, 0x00, 0x01, 0x06],
            "the removed-only patch, byte-equal to the dump"
        );
        // added{DAMAGE:12345,CUSTOM_NAME:-99} removed{CUSTOM_NAME}: the
        // negative probe — the hash rides `ff ff ff 9d`, the two's-
        // complement BE int32, NOT a VarInt's sign extension
        // (`01 9e 07 40 02 03 00 00 30 39 06 ff ff ff 9d 01 06`)
        let p = encode_hashed_stack(&HashedStack::ActualItem {
            item_id: 926,
            count: 64,
            patch: HashedPatchMap {
                added: vec![(3, 12345), (6, -99)],
                removed: vec![6],
            },
        })
        .unwrap();
        assert_eq!(
            p,
            vec![
                0x01, 0x9e, 0x07, 0x40, 0x02, 0x03, 0x00, 0x00, 0x30, 0x39, 0x06, 0xff, 0xff,
                0xff, 0x9d, 0x01, 0x06
            ],
            "the two-lane patch with the negative probe, byte-equal to the dump"
        );
        // added{DAMAGE:123456789}: the large probe — the hash rides
        // `07 5b cd 15`, refusing the 1-2 byte VarInt reading
        // (`01 9e 07 40 01 03 07 5b cd 15 00`)
        let p = encode_hashed_stack(&HashedStack::ActualItem {
            item_id: 926,
            count: 64,
            patch: HashedPatchMap::added(3, 123456789),
        })
        .unwrap();
        assert_eq!(
            p,
            vec![
                0x01, 0x9e, 0x07, 0x40, 0x01, 0x03, 0x07, 0x5b, 0xcd, 0x15, 0x00
            ],
            "the large-value probe, byte-equal to the dump"
        );
        // and the patch rides INSIDE the packet unchanged: one changed
        // slot carrying the added-only stack, the carried lane empty
        let p = encode_container_click(&ContainerClick {
            container_id: 2,
            state_id: 7,
            slot: 13,
            button: 0,
            input: ContainerInput::Pickup,
            changed_slots: vec![(9, HashedStack::ActualItem {
                item_id: 926,
                count: 64,
                patch: HashedPatchMap::added(3, 12345),
            })],
            carried: HashedStack::Empty,
        })
        .unwrap();
        assert_eq!(
            p,
            vec![
                0x12, 0x02, 0x07, 0x00, 0x0d, // header + the BE short slot
                0x00, 0x00, // the button + the PICKUP ordinal
                0x01, // one delta
                0x00, 0x09, // the delta's SHORT slot 9
                0x01, 0x9e, 0x07, 0x40, 0x01, 0x03, 0x00, 0x00, 0x30, 0x39, 0x00, // the stack
                0x00, // the carried lane's emptiness
            ],
            "the non-empty patch inside the full packet"
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

    /// The vitals lane's three packets (the hurt/health arc's own wire —
    /// javap of the real 26.2 `ClientboundSetHealthPacket`,
    /// `ClientboundHurtAnimationPacket` and the
    /// `ClientboundPlayerCombatKillPacket` record, plus the
    /// `ServerboundClientCommandPacket` write): the Set Health
    /// float/VarInt/float triple, the Hurt Animation's VarInt id + BE
    /// float yaw (the 26.x WIDENING from the old byte), the Combat
    /// Kill's component flatten, and the Client Command's VarInt
    /// action (PERFORM_RESPAWN = 0).
    #[test]
    fn health_hurt_combat_kill_speak_the_javap_layouts() {
        // SET HEALTH: 20.0 health, 20 food, 5.0 saturation
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HEALTH);
        p.extend_from_slice(&20.0f32.to_be_bytes());
        write_varint(&mut p, 20);
        p.extend_from_slice(&5.0f32.to_be_bytes());
        let h = parse_set_health(&p).unwrap();
        assert_eq!(h.health, 20.0);
        assert_eq!(h.food, 20);
        assert_eq!(h.saturation, 5.0);
        // a half heart's own number rides exact: 0.5
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HEALTH);
        p.extend_from_slice(&0.5f32.to_be_bytes());
        write_varint(&mut p, 0);
        p.extend_from_slice(&0.0f32.to_be_bytes());
        let h = parse_set_health(&p).unwrap();
        assert_eq!(h.health, 0.5, "the half heart's own float");
        // the truncation map: the body short refuses
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HEALTH);
        p.extend_from_slice(&20.0f32.to_be_bytes());
        assert!(parse_set_health(&p).is_err(), "food and saturation missing");

        // HURT ANIMATION: the VarInt id + the BE float yaw (the widened
        // 26.x lane — the old byte's -1 yaw rides 4294967295.0's own
        // float bytes, not a signed byte)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::HURT_ANIMATION);
        write_varint(&mut p, 137);
        p.extend_from_slice(&(-45.5f32).to_be_bytes());
        let a = parse_hurt_animation(&p).unwrap();
        assert_eq!(a.entity_id, 137);
        assert_eq!(a.yaw, -45.5);
        // the truncation map: the id alone refuses
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::HURT_ANIMATION);
        write_varint(&mut p, 1);
        assert!(parse_hurt_animation(&p).is_err());

        // ENTITY SOUND: the VarInt id + the tagged sound holder + the
        // volume/pitch floats + the seed (the registry arm rides tag 1
        // = registry id 0 — the synced sound_event table's own order)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SOUND_ENTITY);
        write_varint(&mut p, 137);
        write_varint(&mut p, 1); // holder tag 1 -> registry id 0
        p.extend_from_slice(&0.85f32.to_be_bytes());
        p.extend_from_slice(&1.2f32.to_be_bytes());
        p.extend_from_slice(&1234567890123i64.to_be_bytes());
        let s = parse_entity_sound(&p).unwrap();
        assert_eq!(s.entity_id, 137);
        assert_eq!(s.sound, ConsumableSound::Registry(0), "the registry arm");
        assert_eq!(s.volume, 0.85);
        assert_eq!(s.pitch, 1.2);
        assert_eq!(s.seed, 1234567890123);
        // the DIRECT lane rides tag 0: the location string + the range
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SOUND_ENTITY);
        write_varint(&mut p, 5);
        write_varint(&mut p, 0); // holder tag 0 -> direct
        write_string(&mut p, "minecraft:entity.zombie.ambient");
        p.extend_from_slice(&16.0f32.to_be_bytes());
        p.extend_from_slice(&1.0f32.to_be_bytes());
        p.extend_from_slice(&2.0f32.to_be_bytes());
        p.extend_from_slice(&0i64.to_be_bytes());
        let s = parse_entity_sound(&p).unwrap();
        assert_eq!(
            s.sound,
            ConsumableSound::Direct {
                id: "minecraft:entity.zombie.ambient".to_string(),
                range: 16.0,
            },
            "the direct lane verbatim"
        );
        // the truncation map: the seed missing refuses
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SOUND_ENTITY);
        write_varint(&mut p, 5);
        write_varint(&mut p, 1);
        p.extend_from_slice(&1.0f32.to_be_bytes());
        p.extend_from_slice(&1.0f32.to_be_bytes());
        assert!(parse_entity_sound(&p).is_err(), "seed missing");
        // the NEGATIVE holder tag refuses (the consumable's own law)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SOUND_ENTITY);
        write_varint(&mut p, 5);
        write_varint(&mut p, -7);
        assert!(parse_entity_sound(&p).is_err(), "negative holder tag");

        // SOUND (the GLOBAL lane): the tagged holder + the source
        // enum + the THREE 1/8-grid ints + the volume/pitch floats +
        // the seed (the 26.2 ctor's *8 fold the /8 law reads — the
        // BLOCK source the enum's own order names 4)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SOUND);
        write_varint(&mut p, 1); // holder tag 1 -> registry id 0
        write_varint(&mut p, 4); // source BLOCK
        p.extend_from_slice(&768i32.to_be_bytes()); // 768 / 8 = 96.0
        p.extend_from_slice(&520i32.to_be_bytes()); // 520 / 8 = 65.0
        p.extend_from_slice(&(-1024i32).to_be_bytes()); // -1024 / 8 = -128.0
        p.extend_from_slice(&1.0f32.to_be_bytes());
        p.extend_from_slice(&0.9f32.to_be_bytes());
        p.extend_from_slice(&999i64.to_be_bytes());
        let s = parse_sound(&p).unwrap();
        assert_eq!(s.sound, ConsumableSound::Registry(0), "the registry arm");
        assert_eq!(s.source, 4, "the BLOCK source the enum order keeps");
        assert_eq!((s.x, s.y, s.z), (96.0, 65.0, -128.0), "the /8 law");
        assert_eq!(s.volume, 1.0);
        assert_eq!(s.pitch, 0.9);
        assert_eq!(s.seed, 999);
        // the DIRECT lane rides tag 0 verbatim (the location + range
        // the holder's own union speaks)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SOUND);
        write_varint(&mut p, 0); // holder tag 0 -> direct
        write_string(&mut p, "minecraft:block.wooden_door.open");
        write_varint(&mut p, 3); // source HOSTILE rides after the holder
        assert!(parse_sound(&p).is_err(), "the truncated tail refuses");
        // the source ordinal outside the enum refuses (the 0..=9 guard
        // the declaration's own 10 constants keep)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SOUND);
        write_varint(&mut p, 1);
        write_varint(&mut p, 10);
        assert!(parse_sound(&p).is_err(), "source 10 outside the enum");
        // the truncation map: the seed missing refuses
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SOUND);
        write_varint(&mut p, 1);
        write_varint(&mut p, 4);
        p.extend_from_slice(&0i32.to_be_bytes());
        p.extend_from_slice(&0i32.to_be_bytes());
        p.extend_from_slice(&0i32.to_be_bytes());
        p.extend_from_slice(&1.0f32.to_be_bytes());
        p.extend_from_slice(&1.0f32.to_be_bytes());
        assert!(parse_sound(&p).is_err(), "seed missing");

        // PLAYER COMBAT KILL: the VarInt id + the JSON component the
        // disconnect's own flatten law reads
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::PLAYER_COMBAT_KILL);
        write_varint(&mut p, 42);
        write_string(&mut p, r#"{"text":"Player fell from a high place"}"#);
        let k = parse_player_combat_kill(&p).unwrap();
        assert_eq!(k.player_id, 42);
        assert_eq!(k.message, "Player fell from a high place");
        // a plain string rides verbatim (the JSON parse's unwrap-or law)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::PLAYER_COMBAT_KILL);
        write_varint(&mut p, 42);
        write_string(&mut p, "You died!");
        assert_eq!(parse_player_combat_kill(&p).unwrap().message, "You died!");

        // ===== THE SCOREBOARD WIRE TRIO =====

        // SET OBJECTIVE add: the name + the method 0 + the display
        // component (a plain string tag — the bare NBT the TRUSTED
        // codec speaks) + the render type + the format's absence flag
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_OBJECTIVE);
        write_string(&mut p, "money");
        p.push(0); // method ADD
        write_bare_nbt(&Nbt::String("Coins".into()), &mut p).unwrap();
        write_varint(&mut p, 0); // render type INTEGER
        p.push(0); // optional number format: absent
        let o = parse_set_objective(&p).unwrap();
        assert_eq!(o.objective, "money");
        assert_eq!(o.method, OBJECTIVE_METHOD_ADD);
        assert_eq!(o.display.as_deref(), Some("Coins"));
        assert_eq!(o.render_type, RENDER_TYPE_INTEGER);

        // the structured compound flattens through text + extra (the
        // vanilla component walk's own order)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_OBJECTIVE);
        write_string(&mut p, "money");
        p.push(0);
        write_bare_nbt(
            &Nbt::Compound(vec![
                ("text".into(), Nbt::String("A".into())),
                ("extra".into(), Nbt::List(vec![Nbt::String("B".into())])),
            ]),
            &mut p,
        )
        .unwrap();
        write_varint(&mut p, 1); // render type HEARTS
        p.push(1); // optional number format: PRESENT
        write_varint(&mut p, 1); // ... styled — ONE bare style tag
        write_bare_nbt(&Nbt::Compound(vec![]), &mut p).unwrap();
        let o = parse_set_objective(&p).unwrap();
        assert_eq!(o.display.as_deref(), Some("AB"), "the text+extra flatten");
        assert_eq!(o.render_type, RENDER_TYPE_HEARTS, "the readEnum ordinal");

        // the UPDATE method rides the SAME payload branch (the ctor's
        // own dispatch) — and the render type outside the enum refuses
        // (the readEnum bounds the readEnum guard keeps)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_OBJECTIVE);
        write_string(&mut p, "money");
        p.push(2); // method UPDATE
        write_bare_nbt(&Nbt::String("Coins!".into()), &mut p).unwrap();
        write_varint(&mut p, 2); // OUTSIDE the enum
        assert!(parse_set_objective(&p).is_err(), "render type 2 refuses");

        // the REMOVE method reads NOTHING further (the ctor's defaults
        // branch — the empty tail the walk accepts)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_OBJECTIVE);
        write_string(&mut p, "money");
        p.push(1); // method REMOVE
        let o = parse_set_objective(&p).unwrap();
        assert_eq!(o.display, None, "the defaults branch");
        assert_eq!(o.render_type, RENDER_TYPE_INTEGER);

        // SET DISPLAY OBJECTIVE: the slot VarInt + the name (the EMPTY
        // name the clear walk keeps); the slot rides verbatim
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_DISPLAY_OBJECTIVE);
        write_varint(&mut p, 1); // SIDEBAR
        write_string(&mut p, "money");
        let d = parse_set_display_objective(&p).unwrap();
        assert_eq!((d.slot, d.objective.as_str()), (1u8, "money"));
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_DISPLAY_OBJECTIVE);
        write_varint(&mut p, 1);
        write_string(&mut p, "");
        assert_eq!(parse_set_display_objective(&p).unwrap().objective, "");

        // SET SCORE: the owner + the objective + the value + the
        // optional display + the optional number format (a FIXED —
        // registry id 2 — the bare component tag the walk consumes)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_SCORE);
        write_string(&mut p, "Zed");
        write_string(&mut p, "money");
        write_varint(&mut p, 42);
        p.push(1); // display present
        write_bare_nbt(&Nbt::String("Zed Inc.".into()), &mut p).unwrap();
        p.push(1); // number format present
        write_varint(&mut p, 2); // ... fixed
        write_bare_nbt(&Nbt::String("42 coins".into()), &mut p).unwrap();
        let s = parse_set_score(&p).unwrap();
        assert_eq!(s.owner, "Zed");
        assert_eq!(s.objective, "money");
        assert_eq!(s.score, 42);
        assert_eq!(s.display.as_deref(), Some("Zed Inc."));
        // no display + a BLANK format (registry id 0 — zero payload)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_SCORE);
        write_string(&mut p, "Zed");
        write_string(&mut p, "money");
        write_varint(&mut p, -7);
        p.push(0); // display absent
        p.push(1); // format present
        write_varint(&mut p, 0); // ... blank
        let s = parse_set_score(&p).unwrap();
        assert_eq!((s.score, s.display), (-7, None));
        // an unknown format type refuses (the registry's own bounds)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_SCORE);
        write_string(&mut p, "Zed");
        write_string(&mut p, "money");
        write_varint(&mut p, 0);
        p.push(0);
        p.push(1);
        write_varint(&mut p, 3); // the fixed-width the 26.2 registry LOST
        assert!(parse_set_score(&p).is_err(), "format type 3 refuses");

        // RESET SCORE: the owner + the optional objective name
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::RESET_SCORE);
        write_string(&mut p, "Zed");
        p.push(0); // no objective — the holder's WHOLE ledger dies
        let r = parse_reset_score(&p).unwrap();
        assert_eq!((r.owner, r.objective), ("Zed".to_string(), None));
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::RESET_SCORE);
        write_string(&mut p, "Zed");
        p.push(1);
        write_string(&mut p, "money");
        let r = parse_reset_score(&p).unwrap();
        assert_eq!(r.objective.as_deref(), Some("money"));

        // the flatten's own walk: the non-component renders empty
        assert_eq!(flatten_component_text(&Nbt::Int(5)), "");
        assert_eq!(flatten_component_text(&Nbt::Compound(vec![])), "");

        // CLIENT COMMAND encode: the generated id 12 + the action
        // VarInt — PERFORM_RESPAWN 0's exact two bytes
        let p = encode_client_command(0);
        assert_eq!(p, vec![0x0C, 0x00], "the respawn wire's own bytes");
        assert_eq!(encode_client_command(1), vec![0x0C, 0x01], "REQUEST_STATS");

        // PLAYER ABILITIES encode: the generated id 40 + the ONE flag
        // byte — the write walk's `b0 |= 2` law (FLAG_FLYING) the
        // flying verdict alone wears; the grounded byte the 0x00 keeps
        let p = encode_player_abilities(true);
        assert_eq!(p, vec![0x28, 0x02], "the flying flag's own bytes");
        assert_eq!(encode_player_abilities(false), vec![0x28, 0x00], "the grounded byte");

        // CHANGE GAME MODE encode: the generated id 5 + the GameType
        // id VarInt — the audited clinit's id pairs (SURVIVAL 0,
        // CREATIVE 1, ADVENTURE 2, SPECTATOR 3) the four modes speak
        let p = encode_change_game_mode(3);
        assert_eq!(p, vec![0x05, 0x03], "the spectator wire's own bytes");
        assert_eq!(encode_change_game_mode(0), vec![0x05, 0x00], "SURVIVAL");
        assert_eq!(encode_change_game_mode(1), vec![0x05, 0x01], "CREATIVE");
        assert_eq!(encode_change_game_mode(2), vec![0x05, 0x02], "ADVENTURE");
    }

    /// The player-info + tab-list lanes' wire walks (the TAB
    /// overlay's own wire — raw disasm of the 26.2
    /// `ClientboundPlayerInfoUpdatePacket` ctor + Action clinit,
    /// `ClientboundPlayerInfoRemovePacket`, `ClientboundTabListPacket`):
    /// the mask byte (bit i = Action ordinal i, the readFixedBitSet(8)
    /// law), the per-entry walks in ordinal order (ADD = name +
    /// properties, CHAT = the nullable session, GAME_MODE = VarInt,
    /// LISTED = Boolean, LATENCY = VarInt, DISPLAY_NAME = optional
    /// component, LIST_ORDER = VarInt, HAT = Boolean), the UUID list
    /// the removal rides, the two bare-NBT components the header and
    /// footer speak.
    #[test]
    fn the_player_info_lanes_walk_the_masked_entries() {
        // THE FULL WALK: ADD(0) + GAME_MODE(2) + LISTED(3) + LATENCY(4)
        // = mask 0x1D — one entry, the fields in the enum's own order
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::PLAYER_INFO_UPDATE);
        p.push(0x1D); // ADD | GAME_MODE | LISTED | LATENCY
        write_varint(&mut p, 1); // one entry
        p.extend_from_slice(&[0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // msb
        p.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x55, 0xAA]); // lsb
        write_string(&mut p, "Steve");
        write_varint(&mut p, 0); // no properties
        write_varint(&mut p, 1); // game mode CREATIVE
        p.push(1); // listed
        write_varint(&mut p, 42); // latency ms
        let u = parse_player_info_update(&p).unwrap();
        assert_eq!(u.entries.len(), 1);
        let e = &u.entries[0];
        assert_eq!(
            e.uuid,
            0x5500_0000_0000_0000_0000_0000_0000_55AAu128,
            "the two-long UUID fold, msb first"
        );
        assert_eq!(e.name.as_deref(), Some("Steve"));
        assert_eq!(e.game_mode, Some(1));
        assert_eq!(e.listed, Some(true));
        assert_eq!(e.latency, Some(42));
        assert_eq!(e.display_name, None, "the mask never spoke it");
        assert_eq!(e.list_order, None);

        // THE CHAT SESSION'S OWN CONSUMPTION (ADD + INITIALIZE_CHAT =
        // mask 0x03): the property WITH its signature, the session's
        // uuid + expiry + key + signature arrays the walk swallows
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::PLAYER_INFO_UPDATE);
        p.push(0x03);
        write_varint(&mut p, 1);
        p.extend_from_slice(&[0u8; 16]);
        write_string(&mut p, "Alex");
        write_varint(&mut p, 1); // one property
        write_string(&mut p, "textures");
        write_string(&mut p, "base64-skin");
        p.push(1); // signature present
        write_string(&mut p, "sig");
        p.push(1); // chat session present
        p.extend_from_slice(&[2u8; 16]); // session id
        p.extend_from_slice(&1234567i64.to_be_bytes()); // expiry
        write_varint(&mut p, 3); // key bytes
        p.extend_from_slice(&[9, 9, 9]);
        write_varint(&mut p, 2); // signature bytes
        p.extend_from_slice(&[8, 8]);
        let u = parse_player_info_update(&p).unwrap();
        assert_eq!(u.entries[0].name.as_deref(), Some("Alex"));
        assert!(p.len() > 0 && p.len() < usize::MAX, "consumed");

        // the truncated tail refuses (the byte array's own bounds)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::PLAYER_INFO_UPDATE);
        p.push(0x02); // CHAT only
        write_varint(&mut p, 1);
        p.extend_from_slice(&[1u8; 10]); // the session's 16-byte uuid starves
        assert!(parse_player_info_update(&p).is_err(), "truncated chat session refuses");

        // the game-mode guard (the GameType enum's own bounds)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::PLAYER_INFO_UPDATE);
        p.push(0x04); // GAME_MODE
        write_varint(&mut p, 1);
        p.extend_from_slice(&[0u8; 16]);
        write_varint(&mut p, 7); // outside 0..=3
        assert!(parse_player_info_update(&p).is_err(), "game mode 7 refuses");

        // THE REMOVAL: the VarInt count + the UUIDs (the
        // readList(UUIDUtil.STREAM_CODEC) law)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::PLAYER_INFO_REMOVE);
        write_varint(&mut p, 2);
        p.extend_from_slice(&[1u8; 16]);
        p.extend_from_slice(&[2u8; 16]);
        let r = parse_player_info_remove(&p).unwrap();
        assert_eq!(r.uuids.len(), 2, "two entries");
        assert_eq!(r.uuids[0], u128::from_be_bytes([1u8; 16]));
        assert_eq!(r.uuids[1], u128::from_be_bytes([2u8; 16]));

        // THE TAB LIST: header + footer, ONE bare NBT tag each — the
        // string form and the compound form the flatten walks
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::TAB_LIST);
        write_bare_nbt(&Nbt::String("Welcome".into()), &mut p).unwrap();
        write_bare_nbt(
            &Nbt::Compound(vec![(
                "text".into(),
                Nbt::String("server ip".into()),
            )]),
            &mut p,
        )
        .unwrap();
        let t = parse_tab_list(&p).unwrap();
        assert_eq!(t.header, "Welcome");
        assert_eq!(t.footer, "server ip");

        // not-the-right-packet refusals (the generated-id guards)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::TAB_LIST);
        assert!(parse_player_info_update(&p).is_err());
        write_varint(&mut p, clientbound::PLAYER_INFO_UPDATE);
        assert!(parse_tab_list(&p).is_err());
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
    fn the_set_passengers_wire_rides_the_full_list() {
        // THE MOUNT (the ctor's own two reads): vehicle 7 carries the
        // two riders 1 (the local player) and 9 — the VarInt vehicle,
        // the VarInt count, the VarInt riders in the wire's own order
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_PASSENGERS);
        write_varint(&mut p, 7);
        write_varint(&mut p, 2);
        write_varint(&mut p, 1);
        write_varint(&mut p, 9);
        let u = parse_set_passengers(&p).unwrap();
        assert_eq!(u.vehicle, 7);
        assert_eq!(u.passengers, vec![1, 9], "the FULL list, the wire's own order");

        // THE DISMOUNT BROADCAST: the empty array the vehicle now
        // carries nobody — the packet's count 0 speaks it
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_PASSENGERS);
        write_varint(&mut p, 7);
        write_varint(&mut p, 0);
        let u = parse_set_passengers(&p).unwrap();
        assert_eq!(u.vehicle, 7);
        assert!(u.passengers.is_empty(), "the empty list IS the dismount");

        // the wrong id refuses (the parse's own ensure)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REMOVE_ENTITIES);
        assert!(parse_set_passengers(&p).is_err());
        // the absurd count refuses (the array's own bounds)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_PASSENGERS);
        write_varint(&mut p, 7);
        write_varint(&mut p, 99);
        p.push(0);
        assert!(parse_set_passengers(&p).is_err());
    }

    #[test]
    fn the_input_record_byte_keeps_the_seven_keys() {
        // THE BIT LAW (the Input$1 codec's own ior fold): bit 0
        // forward, bit 1 backward, bit 2 left, bit 3 right, bit 4
        // jump, bit 5 shift, bit 6 sprint — each boolean its own
        // power of two
        let f = InputFlags {
            forward: true,
            backward: true,
            left: true,
            right: true,
            jump: true,
            shift: true,
            sprint: true,
        };
        assert_eq!(f.to_byte(), 0b0111_1111);
        assert_eq!(InputFlags::from_byte(0b0111_1111), f, "the decode's own iand probes");
        // the single keys' own pins (the powers of two verbatim)
        assert_eq!(InputFlags { forward: true, ..Default::default() }.to_byte(), 1);
        assert_eq!(InputFlags { backward: true, ..Default::default() }.to_byte(), 2);
        assert_eq!(InputFlags { left: true, ..Default::default() }.to_byte(), 4);
        assert_eq!(InputFlags { right: true, ..Default::default() }.to_byte(), 8);
        assert_eq!(InputFlags { jump: true, ..Default::default() }.to_byte(), 16);
        assert_eq!(InputFlags { shift: true, ..Default::default() }.to_byte(), 32);
        assert_eq!(InputFlags { sprint: true, ..Default::default() }.to_byte(), 64);
        // the silence is the zero byte (the ride's quiet keys)
        assert_eq!(InputFlags::default().to_byte(), 0);
        // THE PACKET (LocalPlayer.tick's own send): the generated id
        // 43 then the ONE byte
        let p = encode_player_input(InputFlags { forward: true, jump: true, ..Default::default() });
        assert_eq!(p, vec![43, 1 | 16], "the id then the record's byte");
    }

    #[test]
    fn the_paddle_boat_rides_the_two_booleans() {
        // THE ID (the generated table's own word): 35,
        // minecraft:paddle_boat — the ServerboundPaddleBoatPacket's
        // type() the GamePacketTypes constant speaks
        assert_eq!(serverbound::PADDLE_BOAT, 35, "minecraft:paddle_boat");
        // THE BODY (the class's own write walk): writeBoolean(left)
        // then writeBoolean(right) — TWO one-byte booleans, the left
        // paddle first
        assert_eq!(encode_paddle_boat(true, false), vec![35, 1, 0], "left only");
        assert_eq!(encode_paddle_boat(false, true), vec![35, 0, 1], "right only");
        assert_eq!(encode_paddle_boat(true, true), vec![35, 1, 1], "both row");
        assert_eq!(encode_paddle_boat(false, false), vec![35, 0, 0], "the quiet ride");
    }

    #[test]
    fn the_open_screen_lanes_pin_the_window() {
        assert_eq!(clientbound::OPEN_SCREEN, 59, "minecraft:open_screen");
        assert_eq!(clientbound::CONTAINER_CLOSE, 17, "minecraft:container_close");
        // the chest's walk: window 5, the menu type generic_9x3 (2),
        // the title a hand-built anonymous-root compound with one
        // string tag — the title's bytes ride back VERBATIM. NOTE the
        // anonymous root's shape: the 1.20.5+ network format has NO
        // root name — the root byte is followed DIRECTLY by the first
        // tag (the walker's own contract)
        let mut title = vec![0x0A];
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
        p.extend_from_slice(&[0x0A, 0x00]); // the minimal empty root: no end-tag name — the root is anonymous
        let o = parse_open_screen(&p).unwrap();
        assert_eq!(o.container_id, 200, "the bare VarInt shape");
        assert_eq!(o.menu_type_id, 0, "generic_9x1");
        assert_eq!(o.title_nbt, vec![0x0A, 0x00]);
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
        p.extend_from_slice(&[0x0A]); // a truncated root: no end tag
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

    #[test]
    fn pong_response_wire_the_ping_request_pair_speaks() {
        // THE ID PIN (the generated table's runtime fact — the
        // ping_request the serverbound play table names 38, the
        // pong_response the clientbound one names 62)
        assert_eq!(serverbound::PING_REQUEST, 38);
        assert_eq!(clientbound::PONG_RESPONSE, 62);

        // THE ENCODE WALK (ServerboundPingRequestPacket.write —
        // `writeLong(time)` the ONE long after the id varint): the
        // sent-time millis the wire carries verbatim
        let p = encode_ping_request(777);
        let mut c: &[u8] = &p;
        assert_eq!(read_varint(&mut c).unwrap(), serverbound::PING_REQUEST);
        assert_eq!(i64::from_be_bytes(c[..8].try_into().unwrap()), 777);
        assert_eq!(p.len(), 9, "one varint byte (777 = 0x0309 fits one) + the one long");

        // THE DECODE WALK (ClientboundPongResponsePacket read ctor —
        // `readLong()` the FriendlyByteBuf read keeps): the echoed
        // sent-time the monitor subtracts from ITS receive instant
        let mut pkt = Vec::new();
        write_varint(&mut pkt, clientbound::PONG_RESPONSE);
        pkt.extend_from_slice(&(-5_000_000_000i64).to_be_bytes());
        assert_eq!(parse_pong_response(&pkt).unwrap(), -5_000_000_000);

        // the strict refusals (the truncation the ensure keeps, the
        // wrong id the walk refuses)
        assert!(parse_pong_response(&pkt[..pkt.len() - 4]).is_err());
        let mut wrong = Vec::new();
        write_varint(&mut wrong, clientbound::PING);
        wrong.extend_from_slice(&1i64.to_be_bytes());
        assert!(parse_pong_response(&wrong).is_err());
    }

    #[test]
    fn debug_sample_wire_the_subscription_pair_speaks() {
        // THE ID PINS (the generated table's runtime facts — the
        // debug_sample the clientbound play table names 30, the
        // debug_subscription_request the serverbound one 23)
        assert_eq!(clientbound::DEBUG_SAMPLE, 30);
        assert_eq!(serverbound::DEBUG_SUBSCRIPTION_REQUEST, 23);

        // THE DECODE WALK (ClientboundDebugSamplePacket read ctor —
        // `readLongArray()` the VarInt-length longs, `readEnum` the
        // type ordinal): the 26.2 tick sample's own shape — FOUR
        // TpsDebugDimensions nanosecond slots, TICK_TIME the type
        let mut pkt = Vec::new();
        write_varint(&mut pkt, clientbound::DEBUG_SAMPLE);
        write_varint(&mut pkt, 4);
        for v in [50_000_000i64, 41_000_000, 2_000_000, 1_000_000] {
            pkt.extend_from_slice(&v.to_be_bytes());
        }
        write_varint(&mut pkt, 0); // RemoteDebugSampleType.TICK_TIME
        let ds = parse_debug_sample(&pkt).unwrap();
        assert_eq!(ds.sample, vec![50_000_000, 41_000_000, 2_000_000, 1_000_000]);
        assert_eq!(ds.sample_type, RemoteSampleType::TickTime);

        // the UNKNOWN ORDINAL the readEnum's own out-of-range law
        // refuses (the enum's one-value vocabulary the wire keeps)
        let mut bad = pkt.clone();
        *bad.last_mut().unwrap() = 1;
        assert!(parse_debug_sample(&bad).is_err());

        // the strict refusals (the truncated longs the ensure keeps,
        // the wrong id the walk refuses)
        assert!(parse_debug_sample(&pkt[..pkt.len() - 2]).is_err());
        let mut wrong = Vec::new();
        write_varint(&mut wrong, clientbound::PING);
        write_varint(&mut wrong, 0);
        assert!(parse_debug_sample(&wrong).is_err());

        // THE ENCODE WALK (ServerboundDebugSubscriptionRequestPacket
        // — the `ByteBufCodecs.collection(..., 32)` SET codec: VarInt
        // count + one registry id per subscription; the client's ONE
        // word DEDICATED_SERVER_TICK_TIME the FIRST registration the
        // DebugSubscriptions clinit banks — registry id 0)
        let p = encode_debug_subscription_request(true);
        assert_eq!(p, vec![23, 1, 0], "varint id + count 1 + registry id 0");
        let mut c: &[u8] = &p;
        assert_eq!(read_varint(&mut c).unwrap(), 23);
        assert_eq!(read_varint(&mut c).unwrap(), 1);
        assert_eq!(read_varint(&mut c).unwrap(), 0);
        // THE EMPTY SET (the lowered flag's unsubscribe word —
        // count 0, no ids)
        assert_eq!(encode_debug_subscription_request(false), vec![23, 0]);
    }

    // ---- THE CHAT WIRE (javap of the real 26.2 packet classes — the
    // byte laws the dumps spoke) ----

    fn chat_types_table() -> Vec<crate::config::ChatTypeEntry> {
        vec![crate::config::ChatTypeEntry {
            name: "minecraft:chat".to_string(),
            chat_key: "chat.type.text".to_string(),
            parameters: vec!["sender".to_string(), "content".to_string()],
        }]
    }

    fn text_component(text: &str) -> Nbt {
        Nbt::Compound(vec![("text".to_string(), Nbt::String(text.to_string()))])
    }

    /// The Player Chat fixture: the full audited field walk, the
    /// caller pins the interesting lanes (unsigned content, filter
    /// mask, chat-type holder).
    fn player_chat_fixture(holder: i32, unsigned: Option<&str>, mask: i32) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::PLAYER_CHAT);
        write_varint(&mut p, 0); // globalIndex
        p.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]); // sender uuid
        write_varint(&mut p, 0); // index
        p.push(0); // signature absent
        write_string(&mut p, "gg"); // the signed body content
        p.extend_from_slice(&1234567890123i64.to_be_bytes()); // the body instant
        p.extend_from_slice(&42i64.to_be_bytes()); // the body salt
        write_varint(&mut p, 0); // the last-seen entries: none
        match unsigned {
            Some(text) => {
                p.push(1);
                write_bare_nbt(&text_component(text), &mut p).unwrap();
            }
            None => p.push(0),
        }
        write_varint(&mut p, mask);
        if mask == 2 {
            write_varint(&mut p, 0); // the partial lane's empty bitset
        }
        write_varint(&mut p, holder);
        if holder == 0 {
            // the direct lane's element NBT: the {chat:{translation_key,
            // parameters}} record the ChatType codec speaks
            let element = Nbt::Compound(vec![(
                "chat".to_string(),
                Nbt::Compound(vec![
                    ("translation_key".to_string(), Nbt::String("chat.type.text".to_string())),
                    (
                        "parameters".to_string(),
                        Nbt::List(vec![
                            Nbt::String("sender".to_string()),
                            Nbt::String("content".to_string()),
                        ]),
                    ),
                ]),
            )]);
            write_network_nbt(&element, &mut p).unwrap();
        }
        write_bare_nbt(&text_component("Steve"), &mut p).unwrap(); // sender name
        p.push(0); // target name absent
        p
    }

    #[test]
    fn the_chat_packet_the_unsigned_walk_the_bytes_keep() {
        // THE BYTE LAW (the audited write verbatim): varint id 9, the
        // utf "hi" (varint byte-length 2 + the bytes), the instant
        // 1234567890123 one long, the zero salt, the absent signature,
        // the empty Update (offset 0 + three zero bitset bytes + the
        // zero checksum) — 26 bytes total
        let p = encode_chat("hi", 1234567890123).unwrap();
        let mut expected = vec![9u8, 2, b'h', b'i'];
        expected.extend_from_slice(&1234567890123i64.to_be_bytes());
        expected.extend_from_slice(&[0u8; 8]); // salt
        expected.push(0); // signature absent
        expected.push(0); // last-seen offset
        expected.extend_from_slice(&[0u8; 3]); // the 20-bit empty bitset
        expected.push(0); // checksum
        assert_eq!(p, expected, "the unsigned chat the byte law keeps");
        // THE CAP (the writeUtf(256) law — 257 utf-8 bytes refuse)
        assert!(encode_chat(&"x".repeat(256), 0).is_ok(), "256 rides");
        assert!(encode_chat(&"x".repeat(257), 0).is_err(), "257 refuses");
    }

    #[test]
    fn the_chat_command_the_bare_utf_walks() {
        // THE 26.2 LAW (the clinit's composite: UTF8 ALONE — the
        // signed-era fields dropped): varint id 7 + one bare utf
        let p = encode_chat_command("time set day");
        let mut expected = vec![7u8];
        expected.push("time set day".len() as u8);
        expected.extend_from_slice(b"time set day");
        assert_eq!(p, expected, "the command the bare utf carries");
    }

    #[test]
    fn the_system_chat_the_component_flattens() {
        // THE WIRE (id 121 + one bare-NBT component + the overlay bool):
        // the extra walk the flatten answers
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SYSTEM_CHAT);
        let comp = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("Hello ".to_string())),
            (
                "extra".to_string(),
                Nbt::List(vec![text_component("world")]),
            ),
        ]);
        write_bare_nbt(&comp, &mut p).unwrap();
        p.push(0);
        let m = parse_system_chat(&p).unwrap();
        assert_eq!(m.content, "Hello world");
        assert!(!m.overlay);
        // the overlay lane
        let mut q = Vec::new();
        write_varint(&mut q, clientbound::SYSTEM_CHAT);
        write_bare_nbt(&text_component("Word"), &mut q).unwrap();
        q.push(1);
        let m = parse_system_chat(&q).unwrap();
        assert!(m.overlay, "the overlay flag rides");
        // THE TRUNCATION (the missing overlay byte refuses)
        let mut r = Vec::new();
        write_varint(&mut r, clientbound::SYSTEM_CHAT);
        write_bare_nbt(&text_component("Word"), &mut r).unwrap();
        assert!(parse_system_chat(&r).is_err());
    }

    #[test]
    fn the_player_chat_the_bound_the_display_walks() {
        // THE FULL WALK (holder 1 -> the table's chat entry; the signed
        // content the unsigned-absent lane displays)
        let p = player_chat_fixture(1, None, 0);
        let m = parse_player_chat(&p, &chat_types_table()).unwrap();
        assert_eq!(m.sender, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        assert_eq!(m.sender_name, "Steve");
        assert_eq!(m.target_name, None);
        assert_eq!(m.content, "gg");
        assert_eq!(m.chat_key, "chat.type.text");
        assert_eq!(m.parameters, vec!["sender", "content"]);
    }

    #[test]
    fn the_player_chat_the_unsigned_content_the_display_prefers() {
        // THE DISPLAY WALK (the unsigned content when present — the
        // vanilla PlayerChatMessage.decode's own preference)
        let p = player_chat_fixture(1, Some("the unsigned word"), 0);
        let m = parse_player_chat(&p, &chat_types_table()).unwrap();
        assert_eq!(m.content, "the unsigned word");
    }

    #[test]
    fn the_player_chat_the_full_filter_the_empty_speaks() {
        // THE FILTERED WALK (the FULLY_FILTERED mask the empty content
        // wears — the filtered-out message the client never speaks)
        let p = player_chat_fixture(1, None, 1);
        let m = parse_player_chat(&p, &chat_types_table()).unwrap();
        assert_eq!(m.content, "");
    }

    #[test]
    fn the_player_chat_the_unknown_chat_type_the_refusal_keeps() {
        // THE BYIDTHROW DIRECTION (holder 5 past the 1-entry table —
        // the drift refused, never guessed)
        let p = player_chat_fixture(5, None, 0);
        assert!(parse_player_chat(&p, &chat_types_table()).is_err());
    }

    #[test]
    fn the_player_chat_the_direct_holder_the_element_walks() {
        // THE DIRECT LANE (wire tag 0 — the element NBT itself the
        // decoration's source)
        let p = player_chat_fixture(0, None, 0);
        let m = parse_player_chat(&p, &chat_types_table()).unwrap();
        assert_eq!(m.chat_key, "chat.type.text");
        assert_eq!(m.parameters, vec!["sender", "content"]);
    }

    #[test]
    fn the_disguised_chat_the_say_lane_the_bound_answers() {
        // THE WIRE (id 33 + the bare-NBT content + the bound — NO uuid,
        // NO signed body, the server-authored record the /say and the
        // plugin broadcasts speak): the holder 2 resolves the say-type's
        // announcement key, the parameters the format's sources name
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::DISGUISED_CHAT);
        write_bare_nbt(&text_component("the server spoke"), &mut p).unwrap();
        // the bound: holder 2 -> the second table entry, sender name,
        // target absent
        write_varint(&mut p, 2);
        write_bare_nbt(&text_component("Server"), &mut p).unwrap();
        p.push(0);
        let types = vec![
            crate::config::ChatTypeEntry {
                name: "minecraft:chat".to_string(),
                chat_key: "chat.type.text".to_string(),
                parameters: vec!["sender".to_string(), "content".to_string()],
            },
            crate::config::ChatTypeEntry {
                name: "minecraft:say_command".to_string(),
                chat_key: "chat.type.announcement".to_string(),
                parameters: vec!["sender".to_string(), "content".to_string()],
            },
        ];
        let m = parse_disguised_chat(&p, &types).unwrap();
        assert_eq!(m.content, "the server spoke");
        assert_eq!(m.sender_name, "Server");
        assert_eq!(m.target_name, None);
        assert_eq!(m.chat_key, "chat.type.announcement");
        assert_eq!(m.parameters, vec!["sender", "content"]);
        // THE TRUNCATION (a bound-less payload refuses)
        let mut q = Vec::new();
        write_varint(&mut q, clientbound::DISGUISED_CHAT);
        write_bare_nbt(&text_component("word"), &mut q).unwrap();
        assert!(parse_disguised_chat(&q, &types).is_err());
    }

    #[test]
    fn the_player_chat_the_shared_bound_the_same_verdicts_keeps() {
        // THE REFACTOR'S OWN GUARD (the bound walk the helper speaks —
        // the player chat's verdicts ride the SAME read the disguised
        // lane answers: the unknown id the refusal keeps, the target
        // the optional walk banks)
        let mut p = player_chat_fixture(2, None, 0);
        // rewrite the fixture's holder: the fixture builder's walk
        // already wrote holder 2's table entry — the SECOND entry the
        // say-command's announcement key resolves
        let types = vec![
            crate::config::ChatTypeEntry {
                name: "minecraft:chat".to_string(),
                chat_key: "chat.type.text".to_string(),
                parameters: vec!["sender".to_string(), "content".to_string()],
            },
            crate::config::ChatTypeEntry {
                name: "minecraft:say_command".to_string(),
                chat_key: "chat.type.announcement".to_string(),
                parameters: vec!["sender".to_string(), "content".to_string()],
            },
        ];
        let m = parse_player_chat(&p, &types).unwrap();
        assert_eq!(m.chat_key, "chat.type.announcement");
        assert_eq!(m.sender_name, "Steve");
        p = player_chat_fixture(9, None, 0);
        assert!(parse_player_chat(&p, &types).is_err(), "the unknown id refuses");
    }

    /// THE CHAT COLORS' OWN WALK (the `ChatFormatting` sixteen the
    /// `§0`..`§f` codes speak — the component's `color` field the
    /// flat emission wears, the child the field omits the PARENT's
    /// color inherits, the SIBLINGS the common parent wears (never
    /// each other — the extra walk's own law), the "reset" the base
    /// the §r answers, the hex the §x emission speaks,
    /// the bold flag the draw lane's no-variant ignore keeps, the
    /// LIST root the component-array form the sibling walk renders,
    /// the uncolored tree the plain flatten's own bytes wears).
    #[test]
    fn the_chat_colors_the_component_walk_speaks() {
        // THE NAMED COLOR (the code the palette names)
        let red = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("A".to_string())),
            ("color".to_string(), Nbt::String("red".to_string())),
        ]);
        assert_eq!(flatten_component_colored(&red), "\u{a7}cA");
        // THE INHERITANCE (the child the field omits the PARENT's
        // color wears — one code the whole subtree speaks)
        let inherit = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("color".to_string(), Nbt::String("red".to_string())),
            (
                "extra".to_string(),
                Nbt::List(vec![text_component("b")]),
            ),
        ]);
        assert_eq!(flatten_component_colored(&inherit), "\u{a7}cab");
        // THE SIBLING LAW (the second child the PARENT's red wears,
        // never the previous sibling's blue — the extra walk's own
        // resolution; the code the state re-emits when it differs)
        let siblings = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("color".to_string(), Nbt::String("red".to_string())),
            (
                "extra".to_string(),
                Nbt::List(vec![
                    Nbt::Compound(vec![
                        ("text".to_string(), Nbt::String("b".to_string())),
                        ("color".to_string(), Nbt::String("blue".to_string())),
                    ]),
                    text_component("c"),
                ]),
            ),
        ]);
        assert_eq!(flatten_component_colored(&siblings), "\u{a7}ca\u{a7}9b\u{a7}cc");
        // THE RESET (the child's "reset" the BASE returns to — the
        // §r the draw's entry-base law answers; the NEXT sibling the
        // PARENT's red still wears — the chain the reset never strips)
        let reset = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("color".to_string(), Nbt::String("red".to_string())),
            (
                "extra".to_string(),
                Nbt::List(vec![
                    Nbt::Compound(vec![
                        ("text".to_string(), Nbt::String("b".to_string())),
                        ("color".to_string(), Nbt::String("reset".to_string())),
                    ]),
                    text_component("c"),
                ]),
            ),
        ]);
        assert_eq!(flatten_component_colored(&reset), "\u{a7}ca\u{a7}rb\u{a7}cc");
        // THE TRUE HEX (the #RRGGBB the legacy §x form speaks — the
        // six nibble pairs the CraftChatMessage convention walks)
        let hex = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("x".to_string())),
            ("color".to_string(), Nbt::String("#55FFFF".to_string())),
        ]);
        assert_eq!(
            flatten_component_colored(&hex),
            "\u{a7}x\u{a7}5\u{a7}5\u{a7}f\u{a7}f\u{a7}f\u{a7}fx"
        );
        // THE BOLD+COLOR ORDER (the draw's own consumption — the color
        // switch the flags clear, the §l the re-emission restores)
        let bold = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("A".to_string())),
            ("color".to_string(), Nbt::String("gold".to_string())),
            ("bold".to_string(), Nbt::Byte(1)),
        ]);
        assert_eq!(flatten_component_colored(&bold), "\u{a7}6\u{a7}lA");
        // THE LIST ROOT (the component-array form — the bare-String
        // element the default the chain-less root wears, the §r the
        // state's own switch answers)
        let list = Nbt::List(vec![
            red.clone(),
            Nbt::String("b".to_string()),
        ]);
        assert_eq!(flatten_component_colored(&list), "\u{a7}cA\u{a7}rb");
        // THE PASSTHROUGH (the uncolored tree the plain flatten's own
        // bytes wears — the two walks the color-less verdict twins)
        let plain = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("Hello ".to_string())),
            (
                "extra".to_string(),
                Nbt::List(vec![text_component("world")]),
            ),
        ]);
        assert_eq!(
            flatten_component_colored(&plain),
            flatten_component_text(&plain)
        );
        assert_eq!(flatten_component_colored(&plain), "Hello world");
        // THE WIRE (the parse lane the colored component's content
        // carries — the yellow the system chat speaks, the codes the
        // player chat's unsigned content rides)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SYSTEM_CHAT);
        write_bare_nbt(
            &Nbt::Compound(vec![
                ("text".to_string(), Nbt::String("Hi ".to_string())),
                ("color".to_string(), Nbt::String("yellow".to_string())),
                (
                    "extra".to_string(),
                    Nbt::List(vec![text_component("there")]),
                ),
            ]),
            &mut p,
        )
        .unwrap();
        p.push(0);
        let m = parse_system_chat(&p).unwrap();
        assert_eq!(m.content, "\u{a7}eHi there", "the color the content carries");
    }

    /// THE TRUE HEX'S OWN WALK (the `#RRGGBB` field the legacy
    /// `§x§r§r§g§g§b§b` emission speaks — the six nibble pairs the
    /// `CraftChatMessage` convention walks, the lowercase digits
    /// `Integer.toHexString` banks): the rgb the SAME state machine
    /// the named codes answer (the switch the flags clear, the §r the
    /// flag-loss re-opens), the rgb→rgb the re-emission only when the
    /// VALUE differs, the rgb→named / named→rgb the cross walks, the
    /// rgb child the field omits the PARENT's rgb inherits, the
    /// malformed hex the inherit law keeps, the case-insensitive
    /// parse the fold speaks.
    #[test]
    fn the_true_hex_the_legacy_emission_speaks() {
        // THE CANONICAL FORM (the six pairs, one per nibble, the
        // lowercase digits — 0x55FFFF the §5 §5 §f §f §f §f wears)
        let hex = |color: &str| {
            Nbt::Compound(vec![
                ("text".to_string(), Nbt::String("x".to_string())),
                ("color".to_string(), Nbt::String(color.to_string())),
            ])
        };
        assert_eq!(
            flatten_component_colored(&hex("#55FFFF")),
            "\u{a7}x\u{a7}5\u{a7}5\u{a7}f\u{a7}f\u{a7}f\u{a7}fx"
        );
        // THE UPPERCASE FIELD (the parse the ascii fold answers — the
        // emission the SAME lowercase digits speaks)
        assert_eq!(
            flatten_component_colored(&hex("#55ffff")),
            flatten_component_colored(&hex("#55FFFF"))
        );
        // THE MALFORMED (the five digits the palette's names never
        // speak — the field the inherit law keeps)
        assert_eq!(flatten_component_colored(&hex("#55FFF")), "x");
        assert_eq!(flatten_component_colored(&hex("55FFFF")), "x");
        assert_eq!(flatten_component_colored(&hex("#GGGGGG")), "x");
        // THE RGB→RGB SWITCH (the re-emission only when the VALUE
        // differs — the equal rgb the sibling law skips)
        let two_rgb = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("color".to_string(), Nbt::String("#FF0080".to_string())),
            (
                "extra".to_string(),
                Nbt::List(vec![
                    Nbt::Compound(vec![
                        ("text".to_string(), Nbt::String("b".to_string())),
                        ("color".to_string(), Nbt::String("#00FF00".to_string())),
                    ]),
                    Nbt::Compound(vec![
                        ("text".to_string(), Nbt::String("c".to_string())),
                        ("color".to_string(), Nbt::String("#FF0080".to_string())),
                    ]),
                ]),
            ),
        ]);
        assert_eq!(
            flatten_component_colored(&two_rgb),
            // #FF0080 → §f§f §0§0 §8§0, #00FF00 → §0§0 §f§f §0§0
            "\u{a7}x\u{a7}f\u{a7}f\u{a7}0\u{a7}0\u{a7}8\u{a7}0a\
             \u{a7}x\u{a7}0\u{a7}0\u{a7}f\u{a7}f\u{a7}0\u{a7}0b\
             \u{a7}x\u{a7}f\u{a7}f\u{a7}0\u{a7}0\u{a7}8\u{a7}0c"
        );
        // THE CROSS WALKS (the rgb→named the code emits, the named→rgb
        // the hex emits — the switch the fresh-style clear always wears)
        let cross = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("color".to_string(), Nbt::String("#FF0080".to_string())),
            (
                "extra".to_string(),
                Nbt::List(vec![Nbt::Compound(vec![
                    ("text".to_string(), Nbt::String("b".to_string())),
                    ("color".to_string(), Nbt::String("red".to_string())),
                ])]),
            ),
        ]);
        assert_eq!(
            flatten_component_colored(&cross),
            "\u{a7}x\u{a7}f\u{a7}f\u{a7}0\u{a7}0\u{a7}8\u{a7}0a\u{a7}cb"
        );
        let cross_back = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("color".to_string(), Nbt::String("red".to_string())),
            (
                "extra".to_string(),
                Nbt::List(vec![Nbt::Compound(vec![
                    ("text".to_string(), Nbt::String("b".to_string())),
                    ("color".to_string(), Nbt::String("#FF0080".to_string())),
                ])]),
            ),
        ]);
        assert_eq!(
            flatten_component_colored(&cross_back),
            "\u{a7}ca\u{a7}x\u{a7}f\u{a7}f\u{a7}0\u{a7}0\u{a7}8\u{a7}0b"
        );
        // THE INHERITANCE (the rgb child the field omits the PARENT's
        // rgb wears — one hex walk the whole subtree speaks)
        let inherit = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("color".to_string(), Nbt::String("#FF0080".to_string())),
            (
                "extra".to_string(),
                Nbt::List(vec![text_component("b")]),
            ),
        ]);
        assert_eq!(
            flatten_component_colored(&inherit),
            "\u{a7}x\u{a7}f\u{a7}f\u{a7}0\u{a7}0\u{a7}8\u{a7}0ab"
        );
        // THE FLAG-LOSS RE-OPEN (the bold then the EXPLICITLY
        // flag-less sibling — the inherited bold the omission keeps,
        // the child's bold:false the §r eraser answers, the rgb the
        // fresh style re-emits)
        let flag_loss = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("color".to_string(), Nbt::String("#FF0080".to_string())),
            ("bold".to_string(), Nbt::Byte(1)),
            (
                "extra".to_string(),
                Nbt::List(vec![Nbt::Compound(vec![
                    ("text".to_string(), Nbt::String("b".to_string())),
                    ("bold".to_string(), Nbt::Byte(0)),
                ])]),
            ),
        ]);
        assert_eq!(
            flatten_component_colored(&flag_loss),
            "\u{a7}x\u{a7}f\u{a7}f\u{a7}0\u{a7}0\u{a7}8\u{a7}0\u{a7}la\
             \u{a7}r\u{a7}x\u{a7}f\u{a7}f\u{a7}0\u{a7}0\u{a7}8\u{a7}0b"
        );
        // THE RESET FROM RGB (the child's "reset" the base returns to,
        // the NEXT sibling the PARENT's rgb still wears)
        let reset = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("color".to_string(), Nbt::String("#FF0080".to_string())),
            (
                "extra".to_string(),
                Nbt::List(vec![
                    Nbt::Compound(vec![
                        ("text".to_string(), Nbt::String("b".to_string())),
                        ("color".to_string(), Nbt::String("reset".to_string())),
                    ]),
                    text_component("c"),
                ]),
            ),
        ]);
        assert_eq!(
            flatten_component_colored(&reset),
            "\u{a7}x\u{a7}f\u{a7}f\u{a7}0\u{a7}0\u{a7}8\u{a7}0a\u{a7}rb\
             \u{a7}x\u{a7}f\u{a7}f\u{a7}0\u{a7}0\u{a7}8\u{a7}0c"
        );
    }

    /// THE CHAT STYLES' OWN WALK (the effect codes the `applyLegacyFormat`
    /// law speaks: the bold/underlined/strikethrough/italic fields the
    /// `§l`/`§n`/`§m`/`§o` codes emit, the flag the child omits the
    /// PARENT's flag wears, the flag-loss the §r the only eraser answers,
    /// the color switch the flag-clear law wears — the fresh style the
    /// `Boolean.valueOf(0)` walk builds, the flags the re-emission
    /// restores, the obfuscated the §k code the draw's pick answers).
    #[test]
    fn the_chat_styles_the_flags_the_legacy_alphabet_speaks() {
        // THE BOLD (the flag the §l code speaks)
        let bold = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("A".to_string())),
            ("bold".to_string(), Nbt::Byte(1)),
        ]);
        assert_eq!(flatten_component_colored(&bold), "\u{a7}lA");
        // THE COLOR-THEN-FLAG ORDER (the draw's own consumption — the
        // color switch the flags clear, the re-emission restores)
        let red_bold = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("A".to_string())),
            ("color".to_string(), Nbt::String("red".to_string())),
            ("bold".to_string(), Nbt::Byte(1)),
        ]);
        assert_eq!(flatten_component_colored(&red_bold), "\u{a7}c\u{a7}lA");
        // THE INHERITANCE (the child the flag field omits the PARENT's
        // bold wears — one §l the whole subtree speaks)
        let inherit = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("bold".to_string(), Nbt::Byte(1)),
            (
                "extra".to_string(),
                Nbt::List(vec![text_component("b")]),
            ),
        ]);
        assert_eq!(flatten_component_colored(&inherit), "\u{a7}lab");
        // THE FLAG-LOSS RESET (the child's explicit false — the §r the
        // only eraser the legacy alphabet carries)
        let loss = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("bold".to_string(), Nbt::Byte(1)),
            (
                "extra".to_string(),
                Nbt::List(vec![Nbt::Compound(vec![
                    ("text".to_string(), Nbt::String("b".to_string())),
                    ("bold".to_string(), Nbt::Byte(0)),
                ])]),
            ),
        ]);
        assert_eq!(flatten_component_colored(&loss), "\u{a7}la\u{a7}rb");
        // THE COLOR-CLEARS-FLAGS INTERACTION (the color switch the
        // fresh style wears — the bold the re-emission restores)
        let both = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("color".to_string(), Nbt::String("red".to_string())),
            ("bold".to_string(), Nbt::Byte(1)),
            (
                "extra".to_string(),
                Nbt::List(vec![Nbt::Compound(vec![
                    ("text".to_string(), Nbt::String("b".to_string())),
                    ("color".to_string(), Nbt::String("blue".to_string())),
                    ("bold".to_string(), Nbt::Byte(1)),
                ])]),
            ),
        ]);
        assert_eq!(
            flatten_component_colored(&both),
            "\u{a7}c\u{a7}la\u{a7}9\u{a7}lb"
        );
        // THE EFFECT TRIO (the strike/underline/italic codes the
        // accumulation speaks — the machine's own emission order)
        let trio = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("x".to_string())),
            ("strikethrough".to_string(), Nbt::Byte(1)),
            ("underlined".to_string(), Nbt::Byte(1)),
            ("italic".to_string(), Nbt::Byte(1)),
        ]);
        assert_eq!(flatten_component_colored(&trio), "\u{a7}m\u{a7}n\u{a7}ox");
        // THE OBFUSCATED CARRY (the §k code the field's flag speaks —
        // the draw lane's random-glyph pick the 26.2 getRandomGlyph law
        // answers; the enum order the k FIRST the k/l/m/n/o walk keeps)
        let obf = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("A".to_string())),
            ("obfuscated".to_string(), Nbt::Byte(1)),
        ]);
        assert_eq!(flatten_component_colored(&obf), "\u{a7}kA");
        // THE OBF+TRIO ORDER (the four effect fields the enum-order
        // codes speak — k before l/m/n/o)
        let obf_trio = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("x".to_string())),
            ("obfuscated".to_string(), Nbt::Byte(1)),
            ("strikethrough".to_string(), Nbt::Byte(1)),
            ("underlined".to_string(), Nbt::Byte(1)),
            ("italic".to_string(), Nbt::Byte(1)),
        ]);
        assert_eq!(
            flatten_component_colored(&obf_trio),
            "\u{a7}k\u{a7}m\u{a7}n\u{a7}ox"
        );
        // THE INHERIT (the child the obfuscated field omits the PARENT's
        // flag wears — one §k the whole subtree speaks)
        let obf_inherit = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("obfuscated".to_string(), Nbt::Byte(1)),
            (
                "extra".to_string(),
                Nbt::List(vec![text_component("b")]),
            ),
        ]);
        assert_eq!(flatten_component_colored(&obf_inherit), "\u{a7}kab");
        // THE OBF FLAG-LOSS (the child's explicit false — the §r the
        // only eraser answers, the reset the child's own state carries)
        let obf_loss = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("obfuscated".to_string(), Nbt::Byte(1)),
            (
                "extra".to_string(),
                Nbt::List(vec![Nbt::Compound(vec![
                    ("text".to_string(), Nbt::String("b".to_string())),
                    ("obfuscated".to_string(), Nbt::Byte(0)),
                ])]),
            ),
        ]);
        assert_eq!(flatten_component_colored(&obf_loss), "\u{a7}ka\u{a7}rb");
        // THE COLOR-CLEARS-OBF INTERACTION (the color switch the fresh
        // style wears — the obfuscated flag the five valueOf(0) kill;
        // the child the field omits the PARENT's flag INHERITS, the
        // re-emission the §k the fresh clear killed restores)
        let obf_color = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("obfuscated".to_string(), Nbt::Byte(1)),
            (
                "extra".to_string(),
                Nbt::List(vec![Nbt::Compound(vec![
                    ("text".to_string(), Nbt::String("b".to_string())),
                    ("color".to_string(), Nbt::String("blue".to_string())),
                ])]),
            ),
        ]);
        assert_eq!(
            flatten_component_colored(&obf_color),
            "\u{a7}ka\u{a7}9\u{a7}kb"
        );
        // THE OBF FLAG-LOSS + COLOR (the child's EXPLICIT false the §r
        // eraser answers, the color the re-walk emits)
        let obf_loss_color = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("a".to_string())),
            ("obfuscated".to_string(), Nbt::Byte(1)),
            (
                "extra".to_string(),
                Nbt::List(vec![Nbt::Compound(vec![
                    ("text".to_string(), Nbt::String("b".to_string())),
                    ("color".to_string(), Nbt::String("blue".to_string())),
                    ("obfuscated".to_string(), Nbt::Byte(0)),
                ])]),
            ),
        ]);
        assert_eq!(
            flatten_component_colored(&obf_loss_color),
            "\u{a7}ka\u{a7}r\u{a7}9b"
        );
        // THE STRING BOOLEAN (the JSON-ish form the field also wears)
        let strflag = Nbt::Compound(vec![
            ("text".to_string(), Nbt::String("A".to_string())),
            ("bold".to_string(), Nbt::String("true".to_string())),
        ]);
        assert_eq!(flatten_component_colored(&strflag), "\u{a7}lA");
    }

    /// THE SUGGESTION REQUEST'S OWN WIRE (javap of the real 26.2
    /// ServerboundCommandSuggestionPacket — the generated id 15 the
    /// packet_ids table banks, the write walk `writeVarInt(id)` FIRST
    /// then `writeUtf(command, 32500)`): the bytes the encode answers,
    /// the transaction id the prefix speaks, the FULL input the
    /// '/'-prefixed command the server's own skip walk keeps, the cap
    /// the 32500 writeUtf law answers.
    #[test]
    fn the_suggestion_request_ride_the_write_walk_speaks() {
        // THE GENERATED IDS (the bootstrapped protocol's runtime facts)
        assert_eq!(serverbound::COMMAND_SUGGESTION, 15);
        assert_eq!(clientbound::COMMAND_SUGGESTIONS, 15);
        // THE BYTES (the id prefix, the transaction, the slash-ful utf)
        let p = encode_command_suggestion(7, "/tp ").unwrap();
        let mut want = Vec::new();
        write_varint(&mut want, serverbound::COMMAND_SUGGESTION);
        write_varint(&mut want, 7);
        write_string(&mut want, "/tp ");
        assert_eq!(p, want);
        // the id the FIRST field rides (the write walk's own order)
        assert_eq!(&p[0..2], &[15, 7], "the id prefix + tx id");
        assert_eq!(&p[2..3], b"\x04", "the utf length first");
        assert_eq!(&p[3..], b"/tp ", "the FULL input the '/' rides");
        // THE CAP (the 32500 writeUtf law — the past-cap command the
        // honest refusal keeps)
        assert!(encode_command_suggestion(0, "/").is_ok());
        assert!(
            encode_command_suggestion(0, &"x".repeat(32501)).is_err(),
            "the 32501-byte utf the cap refuses"
        );
    }

    /// THE SUGGESTION REPLY'S OWN READ WALK (javap of the real 26.2
    /// ClientboundCommandSuggestionsPacket — the composite the clinit
    /// speaks: id/start/length VarInts + the Entry list, each Entry
    /// the capped utf + the TRUSTED_OPTIONAL tooltip the flag byte
    /// wears): the parse the fixture's own bytes answer, the tooltip
    /// the flatten keeps, the count the 1000 walk guards, the wrong
    /// id the honest refusal keeps.
    #[test]
    fn the_suggestion_reply_walk_the_composite_speaks() {
        // THE FIXTURE (id 15, tx 3, start 1, length 2, two rows: the
        // bare "tp" and the tooltip-ful "teleport")
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::COMMAND_SUGGESTIONS);
        write_varint(&mut p, 3);
        write_varint(&mut p, 1);
        write_varint(&mut p, 2);
        write_varint(&mut p, 2);
        write_string(&mut p, "tp");
        p.push(0); // the tooltip: absent (the flag byte)
        write_string(&mut p, "teleport");
        p.push(1); // the flag byte: present
        let tooltip = Nbt::Compound(vec![(
            "text".to_string(),
            Nbt::String("Teleports things".to_string()),
        )]);
        crate::nbt::write_bare_nbt(&tooltip, &mut p).unwrap();
        let m = parse_command_suggestions(&p).unwrap();
        assert_eq!(m.id, 3, "the transaction id the echo keeps");
        assert_eq!(m.start, 1);
        assert_eq!(m.length, 2);
        assert_eq!(m.matches.len(), 2);
        assert_eq!(m.matches[0].text, "tp");
        assert_eq!(m.matches[0].tooltip, None, "the absent flag the None keeps");
        assert_eq!(m.matches[1].text, "teleport");
        assert_eq!(
            m.matches[1].tooltip.as_deref(),
            Some("Teleports things"),
            "the NBT component the flatten keeps"
        );
        // THE COUNT GUARD (the server's subList(0, 1000) walk — the
        // past-cap count the honest refusal keeps)
        let mut big = Vec::new();
        write_varint(&mut big, clientbound::COMMAND_SUGGESTIONS);
        write_varint(&mut big, 1);
        write_varint(&mut big, 0);
        write_varint(&mut big, 0);
        write_varint(&mut big, 1001);
        assert!(
            parse_command_suggestions(&big).is_err(),
            "the 1001-row reply the cap refuses"
        );
        // THE WRONG ID (the not-a-suggestions bytes the guard drops)
        let mut wrong = Vec::new();
        write_varint(&mut wrong, clientbound::SYSTEM_CHAT);
        assert!(parse_command_suggestions(&wrong).is_err());
        // THE STALE-ID CONSUMER WALK rides the screens' tests — the
        // wire's own law the parse's tx echo answers.
    }

    /// A minimal hand-built commands-tree fixture: root(0) with the
    /// literals `time`(1, executable) and `tp`(2, redirect→3), the
    /// `teleport` node(3, literal) carrying ONE argument child(4, the
    /// `minecraft:entity` parser, the ask_server suggestion id).
    fn commands_fixture() -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::COMMANDS);
        write_varint(&mut p, 6); // six entries: root, time, tp, teleport, targets, add
        // entry 0: the root (flags 0, no children yet — filled below)
        p.push(0x00);
        write_varint(&mut p, 2); // children: time, tp
        write_varint(&mut p, 1);
        write_varint(&mut p, 2);
        // entry 1: literal "time" — executable, child = add(5)
        p.push(0x01 | 0x04);
        write_varint(&mut p, 1);
        write_varint(&mut p, 5);
        write_string(&mut p, "time");
        // entry 2: literal "tp" — redirect to 3
        p.push(0x01 | 0x08);
        write_varint(&mut p, 0);
        write_varint(&mut p, 3);
        write_string(&mut p, "tp");
        // entry 3: literal "teleport" — restricted (0x20), child = arg 4
        p.push(0x01 | 0x20);
        write_varint(&mut p, 1);
        write_varint(&mut p, 4);
        write_string(&mut p, "teleport");
        // entry 4: argument "targets" — parser 6 (minecraft:entity,
        // the built-in order's 7th id), suggestion id ask_server,
        // the entity extra's one flag byte
        p.push(0x02 | 0x10);
        write_varint(&mut p, 0);
        write_string(&mut p, "targets");
        write_varint(&mut p, 6); // minecraft:entity (built-in id 6)
        write_string(&mut p, "minecraft:ask_server");
        p.push(0x01); // the entity template's single-entity flag byte
        // entry 5: literal "add" (the time sub-command) — executable
        p.push(0x01 | 0x04);
        write_varint(&mut p, 0);
        write_string(&mut p, "add");
        // the root index
        write_varint(&mut p, 0);
        p
    }

    /// THE COMMANDS TREE'S OWN READ WALK (javap of the real 26.2
    /// ClientboundCommandsPacket + the Entry record's write — the
    /// flags bits the createEntry composition banks: 0x01 literal /
    /// 0x02 argument / 0x04 executable / 0x08 redirect / 0x10 has-
    /// suggestions / 0x20 restricted): the fixture's graph the parse
    /// answers node by node, the stub shapes the kinds speak, the
    /// parser id the built-in order resolves, the redirect the
    /// varint walk keeps, the root index the tail names.
    #[test]
    fn the_commands_tree_the_brigadier_walk_speaks() {
        assert_eq!(clientbound::COMMANDS, 16, "the generated id");
        let t = parse_commands(&commands_fixture(), &[]).unwrap();
        assert_eq!(t.root_index, 0, "the root index the tail carries");
        assert_eq!(t.nodes.len(), 6);
        // the root: no stub, no name
        assert_eq!(t.nodes[0].kind, CommandNodeKind::Root);
        assert_eq!(t.nodes[0].name, "");
        assert_eq!(t.nodes[0].children, vec![1, 2]);
        // the literal: executable, the name the utf keeps
        assert_eq!(t.nodes[1].kind, CommandNodeKind::Literal);
        assert!(t.nodes[1].executable, "the 0x04 bit the walk keeps");
        assert_eq!(t.nodes[1].name, "time");
        assert_eq!(t.nodes[1].children, vec![5]);
        // the redirect: the 0x08 bit the varint follows
        assert_eq!(t.nodes[2].name, "tp");
        assert_eq!(t.nodes[2].redirect, Some(3), "the /tp→/teleport walk");
        // the restricted literal: the 0x20 bit
        assert_eq!(t.nodes[3].name, "teleport");
        assert!(t.nodes[3].restricted, "the owner-only node the flag banks");
        // the argument: the parser the built-in order resolves, the
        // suggestion id the 0x10 flag keeps, the extra byte the
        // entity template consumes
        assert_eq!(t.nodes[4].kind, CommandNodeKind::Argument);
        assert_eq!(
            t.nodes[4].parser,
            Some((6, "minecraft:entity".to_string())),
            "the built-in id 6 the entity speaks"
        );
        assert_eq!(
            t.nodes[4].suggestion_id.as_deref(),
            Some("minecraft:ask_server"),
            "the provider id the identifier walk keeps"
        );
        assert_eq!(t.nodes[4].name, "targets");
        // THE TRAILING BYTES (the aligned walk the guard keeps)
        let mut trailing = commands_fixture();
        trailing.push(0xFF);
        assert!(parse_commands(&trailing, &[]).is_err());
        // THE WRONG ID (the not-a-commands bytes the guard drops)
        let mut wrong = Vec::new();
        write_varint(&mut wrong, clientbound::SYSTEM_CHAT);
        assert!(parse_commands(&wrong, &[]).is_err());
    }

    /// THE ARGUMENT EXTRAS' OWN SHAPES (javap of the 13 custom
    /// serializeToNetwork walks — the number-flag bytes the
    /// ArgumentUtils.createNumberFlags law keeps (bit 1 = min, bit 2
    /// = max, the sentinel comparisons the MIN_VALUE/0 folds speak),
    /// the string's writeEnum varint, the resource family's
    /// identifier): the shapes the parse consumes byte-aligned — the
    /// nodes AFTER an extra-ful argument the alignment verdicts.
    #[test]
    fn the_commands_extra_payloads_the_template_walks_speak() {
        // the builder: one argument entry with the named parser, then
        // a sentinel literal; the sentinel's PARSE answers alignment
        fn build(parser_name: &str, extra: &[u8]) -> Vec<u8> {
            let idx = BUILTIN_COMMAND_ARGUMENT_TYPES
                .iter()
                .position(|b| b.strip_prefix("minecraft:") == Some(parser_name))
                .expect("the built-in table names the parser");
            let mut p = Vec::new();
            write_varint(&mut p, clientbound::COMMANDS);
            write_varint(&mut p, 3);
            // 0: root with child 1
            p.push(0x00);
            write_varint(&mut p, 1);
            write_varint(&mut p, 1);
            // 1: argument "x", parser idx, NO suggestion id, the extra
            p.push(0x02);
            write_varint(&mut p, 0);
            write_string(&mut p, "x");
            write_varint(&mut p, idx as i32);
            p.extend_from_slice(extra);
            // 2: the sentinel literal
            p.push(0x01);
            write_varint(&mut p, 0);
            write_string(&mut p, "sentinel");
            write_varint(&mut p, 0);
            p
        }
        let assert_aligned = |p: &[u8]| {
            let t = parse_commands(p, &[]).unwrap_or_else(|e| {
                panic!("the walk misaligned on {e}");
            });
            assert_eq!(t.nodes[2].name, "sentinel", "the sentinel the alignment speaks");
        };
        // brigadier:integer — min-only (flag 1), max-only (flag 2),
        // both (flag 3), neither (flag 0 — the sentinel defaults)
        let int_extra = |flags: u8| -> Vec<u8> {
            let mut e = vec![flags];
            if flags & 1 != 0 {
                e.extend_from_slice(&42i32.to_be_bytes());
            }
            if flags & 2 != 0 {
                e.extend_from_slice(&7i32.to_be_bytes());
            }
            e
        };
        for flags in [0u8, 1, 2, 3] {
            assert_aligned(&build("brigadier:integer", &int_extra(flags)));
        }
        // brigadier:float — the f32 pair the flags answer
        let mut f = vec![3u8];
        f.extend_from_slice(&1.5f32.to_be_bytes());
        f.extend_from_slice(&2.5f32.to_be_bytes());
        assert_aligned(&build("brigadier:float", &f));
        // brigadier:double — the f64 pair
        let mut d = vec![1u8];
        d.extend_from_slice(&1.5f64.to_be_bytes());
        assert_aligned(&build("brigadier:double", &d));
        // brigadier:long — the i64 pair
        let mut l = vec![2u8];
        l.extend_from_slice(&77i64.to_be_bytes());
        assert_aligned(&build("brigadier:long", &l));
        // brigadier:string — the writeEnum varint (GREEDY_STRING = 2)
        assert_aligned(&build("brigadier:string", &[2]));
        // entity / score_holder — the one flag byte
        assert_aligned(&build("entity", &[1]));
        assert_aligned(&build("score_holder", &[3]));
        // time — the writeInt(min)
        let mut ti = Vec::new();
        ti.extend_from_slice(&1i32.to_be_bytes());
        assert_aligned(&build("time", &ti));
        // the resource family — the registry-key identifier
        assert_aligned(&build(
            "resource",
            {
                let mut e = Vec::new();
                write_string(&mut e, "minecraft:item");
                e
            }
            .as_slice(),
        ));
        assert_aligned(&build(
            "resource_selector",
            {
                let mut e = Vec::new();
                write_string(&mut e, "minecraft:entity");
                e
            }
            .as_slice(),
        ));
        // the SINGLETON names — the bare return walk (no extra bytes)
        assert_aligned(&build("game_profile", &[]));
        assert_aligned(&build("block_pos", &[]));
        assert_aligned(&build("uuid", &[]));
        // THE SYNCED TABLE (a server's own registry order the id
        // resolves through — the same name the walk keys on)
        let synced = vec![
            "minecraft:custom_one".to_string(),
            "minecraft:entity".to_string(),
        ];
        let mut p2 = Vec::new();
        write_varint(&mut p2, clientbound::COMMANDS);
        write_varint(&mut p2, 3);
        p2.push(0x00);
        write_varint(&mut p2, 1);
        write_varint(&mut p2, 1);
        p2.push(0x02);
        write_varint(&mut p2, 0);
        write_string(&mut p2, "x");
        write_varint(&mut p2, 1); // id 1 in the SYNCED order = entity
        p2.push(9); // the entity flag byte
        p2.push(0x01);
        write_varint(&mut p2, 0);
        write_string(&mut p2, "sentinel");
        write_varint(&mut p2, 0);
        let t2 = parse_commands(&p2, &synced).unwrap();
        assert_eq!(
            t2.nodes[1].parser,
            Some((1, "minecraft:entity".to_string())),
            "the synced table the resolution wears"
        );
        // THE UNKNOWN NAME (a synced entry no built-in const names —
        // the extra the walk cannot consume, the refusal the desync
        // answers)
        let unknown = vec!["minecraft:plugin_magic".to_string()];
        let mut p3 = Vec::new();
        write_varint(&mut p3, clientbound::COMMANDS);
        write_varint(&mut p3, 2);
        p3.push(0x00);
        write_varint(&mut p3, 1);
        write_varint(&mut p3, 1);
        p3.push(0x02);
        write_varint(&mut p3, 0);
        write_string(&mut p3, "x");
        write_varint(&mut p3, 0); // id 0 = plugin_magic in the synced order
        p3.push(0x01);
        write_varint(&mut p3, 0);
        assert!(
            parse_commands(&p3, &unknown).is_err(),
            "the unknown parser the honest refusal keeps"
        );
        // THE UNKNOWN ID (past both tables)
        let mut p4 = Vec::new();
        write_varint(&mut p4, clientbound::COMMANDS);
        write_varint(&mut p4, 2);
        p4.push(0x00);
        write_varint(&mut p4, 1);
        write_varint(&mut p4, 1);
        p4.push(0x02);
        write_varint(&mut p4, 0);
        write_string(&mut p4, "x");
        write_varint(&mut p4, 9999);
        write_varint(&mut p4, 0);
        assert!(parse_commands(&p4, &[]).is_err());
    }

    /// THE BUILT-IN ARGUMENT TABLE'S OWN ORDER (the 26.2
    /// ArgumentTypeInfos bootstrap verbatim — the registration order
    /// IS the registry id space the skip-sync path answers): the
    /// uniqueness, the bookends, the resolution helper's two laws
    /// (the synced entry the name-fold answers, the out-of-table id
    /// the built-in slice keeps).
    #[test]
    fn the_argument_type_table_the_builtin_order_speaks() {
        assert_eq!(BUILTIN_COMMAND_ARGUMENT_TYPES.len(), 55);
        assert_eq!(
            BUILTIN_COMMAND_ARGUMENT_TYPES[0], "minecraft:brigadier:bool",
            "the bool the registration's first walk banks"
        );
        assert_eq!(
            BUILTIN_COMMAND_ARGUMENT_TYPES[54], "minecraft:uuid",
            "the uuid the bootstrap's tail keeps"
        );
        assert_eq!(BUILTIN_COMMAND_ARGUMENT_TYPES[6], "minecraft:entity");
        assert_eq!(
            BUILTIN_COMMAND_ARGUMENT_TYPES[41], "minecraft:time",
            "the time the resource family's predecessor rides"
        );
        // the names unique (a duplicate would fold two ids into one)
        let mut sorted: Vec<&str> = BUILTIN_COMMAND_ARGUMENT_TYPES.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), BUILTIN_COMMAND_ARGUMENT_TYPES.len());
        // THE RESOLUTION HELPER: the synced entry (a reordered table —
        // id 0 = entity) the name-fold answers
        let synced = vec!["minecraft:entity".to_string()];
        assert_eq!(
            command_argument_type_name(0, &synced),
            Some("minecraft:entity"),
            "the synced table the lookup wears"
        );
        // the out-of-synced-table id the built-in slice keeps
        assert_eq!(
            command_argument_type_name(6, &synced),
            Some("minecraft:entity"),
            "the built-in order the skip-sync law answers"
        );
        // the past-both id the None keeps
        assert_eq!(command_argument_type_name(999, &[]), None);
    }
}

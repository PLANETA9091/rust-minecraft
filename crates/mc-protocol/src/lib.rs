//! mc-protocol — hand-rolled Minecraft protocol layer (clean-room, wiki.vg-based).
//!
//! Includes: VarInt/VarLong, packet framing with zlib compression support,
//! handshake + status (server list ping), offline-mode login sequence, the
//! configuration phase that reaches the play state, and the play state
//! itself — the first real world from the network (probes answered, chunk
//! data decoded into resolved block/biome ids).

pub mod config;
pub mod framing;
pub mod login;
pub mod play;
pub mod status;
pub mod varint;

pub use config::{connect_and_join_play, join_play, join_play_conn, PlayOutcome};
pub use play::{
    begin_play, connect_and_join_world, join_world, ChunkColumn, ChunkSection, PlayEvent,
    PlaySession, PlayStart, PlayerPosition, WorldInfo, WorldOutcome, WorldSnapshot,
    DESIRED_CHUNKS_PER_TICK, pack_entries,
};
pub use login::{connect_and_login, login, LoginOutcome};
pub use status::{ping, PingResponse};

/// Default network timeout used by blocking IO helpers (seconds).
pub const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// Handshake intent: next state = status.
pub const NEXT_STATE_STATUS: i32 = 1;
/// Handshake intent: next state = login.
pub const NEXT_STATE_LOGIN: i32 = 2;

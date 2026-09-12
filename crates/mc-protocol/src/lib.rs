//! mc-protocol — hand-rolled Minecraft protocol layer (clean-room, wiki.vg-based).
//!
//! Includes: VarInt/VarLong, packet framing with zlib compression support,
//! handshake + status (server list ping), login-start encoding.

pub mod framing;
pub mod status;
pub mod varint;

pub use status::{ping, PingResponse};

/// Default network timeout used by blocking IO helpers (seconds).
pub const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// Handshake intent: next state = status.
pub const NEXT_STATE_STATUS: i32 = 1;
/// Handshake intent: next state = login.
pub const NEXT_STATE_LOGIN: i32 = 2;

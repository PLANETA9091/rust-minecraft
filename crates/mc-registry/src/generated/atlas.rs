// @generated fallback — run `cargo run -p mc2rs-gen -- atlas --version 26.2`
// against the official jar to replace this with real texture data.
pub const ATLAS_VERSION: &str = "26.2";
pub const TILE_SIZE: usize = 16;
pub const TILE_NONE: u16 = u16::MAX;
pub const N_KINDS: usize = 8;
pub const KIND_NAMES: [&str; N_KINDS] = ["air", "stone", "dirt", "grass_block", "oak_log", "oak_leaves", "water", "sand"];
pub const FACE_TILES: [[u16; 6]; N_KINDS] = [[TILE_NONE; 6]; N_KINDS];
pub const TINTS: [[u32; 6]; N_KINDS] = [[0; 6]; N_KINDS];
pub static TILES: [&[u8]; 0] = [];
pub static TILE_NAMES: [&str; 0] = [];
pub fn has_tiles() -> bool { !TILES.is_empty() }
pub fn tile_rgba(idx: u16) -> &'static [u8] { TILES[idx as usize] }
pub fn tile_name(idx: u16) -> &'static str { TILE_NAMES[idx as usize] }

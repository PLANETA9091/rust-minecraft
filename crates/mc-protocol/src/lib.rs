//! mc-protocol — hand-rolled Minecraft protocol layer (clean-room).
//!
//! Includes: VarInt/VarLong, packet framing with zlib compression support,
//! handshake + status (server list ping), offline-mode login sequence, the
//! configuration phase that reaches the play state (its Registry Data is
//! decoded — the dimension-type vertical layout), network NBT read/write,
//! and the play state itself — the first real world from the network
//! (probes answered, chunk data decoded into resolved block/biome ids).
//! The wire ids of EVERY phase are GENERATED from the official jar's own
//! bootstrapped protocol (see `generated::packet_ids`): play, login,
//! configuration, status and handshake all dump their ids from the
//! vanilla dispatch codecs' live id maps — the hand-kept wiki-era tables
//! had drifted in BOTH waves (the play ids drifted several packets; the
//! configuration clientbound ids were all one low: registry_data is 0x07
//! on the real 26.2 wire, not 0x06).

pub mod config;
pub mod framing;
pub mod generated;
pub mod login;
pub mod nbt;
pub mod play;
pub mod status;
pub mod varint;

pub use config::{
    connect_and_join_play, join_play, join_play_conn, BiomeEffects, BiomeEntry, DimensionTypeEntry,
    GrassColorModifier, PlayOutcome, RegistryData, Skybox, TimelineEntry,
};
pub use nbt::Nbt;
pub use play::{
    begin_play, connect_and_join_live, connect_and_join_world, join_world, AttributeDisplay,
    AttributeModifier, AttributeModifierEntry, BlockPredicate, BlockSet, ChunkColumn, ChunkLight,
    ChunkSection, ClockUpdate, ComponentPatch, ComponentValue, ConsumeEffect, ConsumableSound,
    ContainerClick, ContainerClose, ContainerContent, ContainerInput, ContainerSetSlot,
    DataComponentMatchers, EffectDetails,
    EffectHolderSet, EffectInstance,
    EntityDataEntry,
    EntityDataSet, EntityDeltaPos, EntityDeltaPosRot, EntityDeltaRot, EntityEquipment,
    EntityHeadRot,
    EntityPositionSync, EntityReference, EntitySpawn, EquipmentEntry, EquipmentSlot, GlobalPos,
    HashedPatchMap, HashedStack, ItemStack, LightData, LiveJoin,
    MobEffectRemove, MobEffectUpdate, OpenScreen, PaintingVariant, PartialPredicate,
    ParticleValue, PlayerSkinPatch, PlayEvent, PositionSource,
    ProfileProperty, ResolvableProfile, SkinAsset,
    PlaySession, PlayStart, PlayerPosition, PredicateTypeLane, PropertyMatcher, PropertyValue,
    SetEntityDataEntry, SetHeldSlot, SpawnInfo, TimeSync, WorldInfo, WorldOutcome, WorldSnapshot,
    DESIRED_CHUNKS_PER_TICK, encode_container_click, encode_container_close, pack_entries,
};
pub use login::{connect_and_login, login, LoginOutcome};
pub use status::{ping, PingResponse};

/// Default network timeout used by blocking IO helpers (seconds).
pub const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// Handshake intent: next state = status.
pub const NEXT_STATE_STATUS: i32 = 1;
/// Handshake intent: next state = login.
pub const NEXT_STATE_LOGIN: i32 = 2;

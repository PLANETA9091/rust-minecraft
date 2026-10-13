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
    AttributeModifier, AttributeModifierEntry, AttributeSnapshot, BlockPredicate, BlockSet,
    ChunkColumn, ChunkLight,
    ChunkSection, ClockUpdate, ComponentPatch, ComponentValue, ConsumeEffect, ConsumableSound,
    ContainerClick, ContainerClose, ContainerContent, ContainerInput, ContainerSetSlot,
    DataComponentMatchers, EffectDetails,
    EffectHolderSet, EffectInstance,
    EntityDataEntry,
    EntityDataSet, EntityDeltaPos, EntityDeltaPosRot, EntityDeltaRot, EntityEquipment,
    EntityHeadRot,
    EntityPositionSync, EntityReference, EntitySpawn, EquipmentEntry, EquipmentSlot, FireworkExplosion, GlobalPos,
    HashedPatchMap, HashedStack, ItemStack, LightData, LiveJoin,
    MobEffectRemove, MobEffectUpdate, OpenScreen, PaintingVariant, PartialPredicate,
    ParticleValue, PlayerSkinPatch, PlayEvent, PositionSource,
    ProfileProperty, ResolvableProfile, SkinAsset,
    PlaySession, PlayStart, PlayerPosition, PredicateTypeLane, PropertyMatcher, PropertyValue,
    SetEntityDataEntry, SetHeldSlot, SetHealth, SpawnInfo, TimeSync, UpdateAttributes, WorldInfo, WorldOutcome, WorldSnapshot,
    ObjectiveUpdate, DisplayObjectiveUpdate, ScoreUpdate, ResetScore,
    SetPassengers, InputFlags,
    flatten_component_text, parse_set_objective, parse_set_display_objective, parse_set_score,
    parse_reset_score, OBJECTIVE_METHOD_ADD, OBJECTIVE_METHOD_REMOVE, OBJECTIVE_METHOD_UPDATE,
    RENDER_TYPE_INTEGER, RENDER_TYPE_HEARTS, DISPLAY_SLOT_SIDEBAR, DISPLAY_SLOT_BELOW_NAME,
    PlayerEntryPatch, PlayerInfoUpdate, PlayerInfoRemove, TabListUpdate,
    parse_player_info_update, parse_player_info_remove, parse_tab_list,
    DESIRED_CHUNKS_PER_TICK, encode_client_command, encode_container_click, encode_container_close, pack_entries,
    CommandSuggestionsMatch, CommandSuggestionsReply, encode_command_suggestion,
    parse_command_suggestions,
    encode_player_input, encode_paddle_boat, parse_set_passengers,
    encode_player_abilities, encode_change_game_mode,
    HurtAnimation, PlayerCombatKill, EntitySoundPacket, WorldSoundPacket,
    LevelEventPacket, GROWTH_EFFECT, EYE_OF_ENDER_SHATTER_EFFECT, DESTROY_BLOCK_EFFECT, SHOOT_SMOKE_EFFECT,
    SHOOT_WHITE_SMOKE_EFFECT, SPLASH_POTION_EFFECT, INSTANT_SPLASH_EFFECT, MOB_SPAWN_EFFECT,
    BEE_GROWTH_EFFECT, TURTLE_EGG_PLACEMENT_EFFECT, DRAGON_FIREBALL_SPLASH_EFFECT,
    SMASH_ATTACK_EFFECT,
    WATER_EVAPORATING_EFFECT, DRAGON_BLOCK_BREAK_EFFECT, SOUND_DISPENSER_DISPENSE_EFFECT,
    PARTICLES_AND_SOUND_WAX_ON_EFFECT, PARTICLES_WAX_OFF_EFFECT, PARTICLES_SCRAPE_EFFECT,
    END_GATEWAY_SPAWN_EFFECT, ENDER_DRAGON_GROWL_EFFECT, ELECTRIC_SPARK_EFFECT,
    PARTICLES_SCULK_CHARGE_EFFECT, PARTICLES_SCULK_SHRIEK_EFFECT,
    PARTICLES_AND_SOUND_BRUSH_BLOCK_COMPLETE_EFFECT, PARTICLES_EGG_CRACK_EFFECT,
    PARTICLES_TRIAL_SPAWNER_SPAWN_EFFECT, PARTICLES_AND_SOUND_TRIAL_SPAWNER_SPAWN_MOB_EFFECT,
    PARTICLES_AND_SOUND_TRIAL_SPAWNER_SPAWN_ITEM_EFFECT,
    PARTICLES_TRIAL_SPAWNER_DETECT_PLAYER_EFFECT,
    PARTICLES_TRIAL_SPAWNER_DETECT_PLAYER_OMINOUS_EFFECT,
    PARTICLES_AND_SOUND_TRIAL_SPAWNER_EJECT_ITEM_EFFECT, PARTICLES_TRIAL_SPAWNER_EJECT_ITEM_EFFECT,
    PARTICLES_AND_SOUND_TRIAL_SPAWNER_OMINOUS_ACTIVATE_EFFECT,
    PARTICLES_AND_SOUND_SPAWN_COBWEB_EFFECT,
    PARTICLES_AND_SOUND_VAULT_ACTIVATE_EFFECT, PARTICLES_AND_SOUND_VAULT_DEACTIVATE_EFFECT,
    SOUND_DISPENSER_FAIL_EFFECT, SOUND_DISPENSER_PROJECTILE_LAUNCH_EFFECT,
    FIREWORK_ROCKET_SHOOT_EFFECT, SOUND_EXTINGUISH_FIRE_EFFECT,
    SOUND_PLAY_JUKEBOX_SONG_EFFECT, SOUND_STOP_JUKEBOX_SONG_EFFECT,
    SOUND_CHORUS_GROW_EFFECT, SOUND_CHORUS_DEATH_EFFECT, SOUND_BREWING_STAND_BREW_EFFECT,
    SOUND_CRAFTER_CRAFT_EFFECT, SOUND_CRAFTER_FAIL_EFFECT,
    SOUND_GHAST_WARNING_EFFECT, SOUND_GHAST_FIREBALL_EFFECT,
    SOUND_DRAGON_FIREBALL_EFFECT, SOUND_BLAZE_FIREBALL_EFFECT,
    SOUND_ZOMBIE_WOODEN_DOOR_EFFECT, SOUND_ZOMBIE_IRON_DOOR_EFFECT,
    SOUND_ZOMBIE_DOOR_CRASH_EFFECT, SOUND_WITHER_BLOCK_BREAK_EFFECT,
    SOUND_WITHER_BOSS_SHOOT_EFFECT, SOUND_ZOMBIE_INFECTED_EFFECT,
    SOUND_ZOMBIE_CONVERTED_EFFECT, SOUND_ZOMBIE_TO_DROWNED_EFFECT,
    SOUND_HUSK_TO_ZOMBIE_EFFECT,
    SOUND_ANVIL_BROKEN_EFFECT, SOUND_ANVIL_USED_EFFECT, SOUND_ANVIL_LAND_EFFECT,
    SOUND_PHANTOM_BITE_EFFECT, SOUND_GRINDSTONE_USED_EFFECT, SOUND_PAGE_TURN_EFFECT,
    SOUND_SMITHING_TABLE_USED_EFFECT, SOUND_POINTED_DRIPSTONE_LAND_EFFECT,
    SOUND_DRIP_LAVA_INTO_CAULDRON_EFFECT, SOUND_DRIP_WATER_INTO_CAULDRON_EFFECT,
    SOUND_SULFUR_SPIKE_LAND_EFFECT,
    SOUND_SKELETON_TO_STRAY_EFFECT, SOUND_BAT_LIFTOFF_EFFECT,
    SOUND_WIND_CHARGE_SHOOT_EFFECT, SOUND_PORTAL_TRAVEL_EFFECT,
    COMPOSTER_FILL_EFFECT, LAVA_FIZZ_EFFECT, REDSTONE_TORCH_BURNOUT_EFFECT,
    END_PORTAL_FRAME_FILL_EFFECT, DRIPSTONE_DRIP_EFFECT,
};
pub use login::{connect_and_login, login, LoginOutcome};
pub use status::{ping, PingResponse};

/// Default network timeout used by blocking IO helpers (seconds).
pub const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// Handshake intent: next state = status.
pub const NEXT_STATE_STATUS: i32 = 1;
/// Handshake intent: next state = login.
pub const NEXT_STATE_LOGIN: i32 = 2;

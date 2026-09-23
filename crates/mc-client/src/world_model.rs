//! Client-side world model — compact storage for the chunks the play
//! state streams in. This is the first place the project HOLDS a network
//! world as queryable state: `PlayEvent`s from the live session (or a
//! `WorldSnapshot` from the join milestone) land here, keyed by chunk
//! column, and block queries resolve world coordinates through the
//! vanilla index conventions the decoder pins.
//!
//! Design notes (documented tradeoffs, all deliberate):
//! - Columns store RESOLVED global state ids (what the decoder emits) —
//!   no palette re-compression until memory pressure shows up. The map is
//!   keyed `(chunk_x, chunk_z)`; a re-sent column REPLACES the stored one
//!   (vanilla semantics: the newest Chunk Data wins), Unload Chunk drops
//!   the entry.
//! - The CURRENT dimension is pinned at login (vanilla 1.20.5+ carries it
//!   in CommonPlayerSpawnInfo — javap-proven) and updated by Respawn;
//!   `min_y` is looked up from the decoded dimension-type registry the
//!   moment the dimension is known. Until then `block()` takes
//!   COLUMN-LOCAL y (0 = bottom of the lowest section); once known,
//!   world-Y queries subtract it before the section math. A Respawn
//!   CLEARS the loaded columns — the vanilla level recreation: another
//!   dimension's blocks are meaningless (and, re-read under the new
//!   min_y, pure fiction), so the map starts empty in the new world.
//! - Light travels WITH the columns: each decoded column carries the
//!   server's sky/block light arrays (`ChunkLight`), and `sky_light()`/
//!   `block_light()` answer through the engine-layer grid (one extra
//!   section below/above the column). No client-side propagation model
//!   exists — the network's arrays ARE the truth (the server runs the
//!   light engine); a local BFS for future local edits is future work.
//!   The standalone Update Light packet REPLACES a column's light on
//!   the live wire (the vanilla masks are the engine's full current
//!   storage for the column) — refused honestly for unloaded columns or
//!   out-of-range bits, never half-applied.
//! - The join path is WIRED into the frame loop: the world-load
//!   milestone's snapshot adopts here and the player-centered window
//!   materializes into the renderer grid (see `world_view`). A live,
//!   continuously-updated model (streaming chunks into the SAME model the
//!   screen renders) is the future work.

use std::collections::{BTreeMap, HashMap};

use mc_protocol::{
    ChunkColumn, ChunkLight, ClockUpdate, ComponentPatch, EntityDataEntry, ItemStack, LightData,
    PlayEvent, PlayerPosition, RegistryData, SetHealth, WorldInfo, WorldSnapshot,
};
#[cfg(test)]
use mc_protocol::{EntityDataSet, MobEffectRemove, MobEffectUpdate};

/// The model's view of ONE entity — the facts the entity packets have
/// actually delivered (the spawn's registry-typed existence, the latest
/// authoritative position). Rotation rides the spawn's byte-unpacked
/// degrees until a sync or move packet overrides it (the sync's floats
/// are exact; the spawn's and move bytes are the vanilla
/// `b * 360 / 256` quantization).
#[derive(Debug, Clone, PartialEq)]
pub struct EntitySnapshot {
    /// The entity type's registry id (resolved by the synced registry).
    pub type_id: i32,
    /// The entity's wire UUID (the 128 bits packed most-long first) —
    /// the SCOREBOARD HOLDER the below-name lane resolves (the vanilla
    /// getScoreboardName walk returns the stringUUID: the hyphenated
    /// lowercase form [`string_uuid`] formats).
    pub uuid: u128,
    /// World position (the latest full-precision doubles).
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// Yaw in degrees (the latest sync's float, else the spawn byte
    /// unpacked the vanilla way; the move packets' bytes override).
    pub y_rot: f32,
    /// Pitch in degrees (the latest sync's float, else the spawn byte
    /// unpacked the vanilla way; the move packets' bytes override).
    pub x_rot: f32,
    /// Head yaw in degrees (the spawn byte unpacked the vanilla way,
    /// kept current by Rotate Head — the head's own channel, distinct
    /// from the body's yaw).
    pub y_head_rot: f32,
    /// The entity's metadata store — the accessor-indexed entries the
    /// Set Entity Data packets have delivered (a BTreeMap: deterministic
    /// order for tests and for any future rendering diff). The spawn
    /// starts it empty; each packet's entries REPLACE the listed
    /// accessors and leave the unmentioned ones (the assignValues walk).
    pub data: BTreeMap<u8, EntityDataEntry>,
    /// THE SPAWN'S OWN MOTION WORD — the low-precision Vec3 the Add
    /// Entity packet carries (the projectile's initial velocity the
    /// vanilla `ClientboundAddEntityPacket.getVelocity` speaks, the
    /// decode's 1/32766 fold the LP lane answers). The arrow's own
    /// flight law consumes it (the client-side sim the vanilla client
    /// runs from the server's spawn word); the mobs' lanes never read
    /// it (the lerpTo walk the position syncs pace instead).
    pub motion: [f64; 3],
}

/// The client's view of ONE server world: loaded chunk columns, the
/// server-declared center chunk, the world's static description and the
/// latest player sync — everything the play state has actually sent.
#[allow(dead_code)] // wired into the frame loop when the mesher input lands
pub struct WorldModel {
    /// Set Center Chunk: where the server says the view is anchored.
    pub center: (i32, i32),
    /// The play-state Login packet's world description, once sent.
    pub info: Option<WorldInfo>,
    /// The latest Synchronize Player Position (stored raw — relative-flag
    /// math is future work; vanilla absolute teleports are exact).
    pub player: Option<PlayerPosition>,
    /// World-Y offset of the bottom section (dimension type), when known.
    /// `None` = `block()` takes column-local y.
    pub min_y: Option<i32>,
    /// The dimension the player is currently in, as the server named it
    /// (pinned at login, updated on Respawn).
    pub current_dimension: Option<String>,
    /// The dimension-type vertical layout table the configuration phase
    /// decoded (registry key -> min_y/height). `min_y` above resolves
    /// through it once `current_dimension` is pinned.
    pub registry: RegistryData,
    /// The latest Set Time: the world's monotonic game time (0 until
    /// the server's first sync).
    pub game_time: i64,
    /// THE WEATHER'S OWN LEVELS (javap `Level` — the rain/thunder state
    /// the Game Event packets speak: the `setRainLevel`/`setThunderLevel`
    /// walks keep the `Mth.clamp(param, 0.0, 1.0)` pair and store the
    /// SAME value into the old/current slots — no easing on the client,
    /// the server's incremental RAIN_LEVEL_CHANGE stream IS the ease).
    /// The renderer's `rainBrightness = 1.0 - rainLevel` reads the rain
    /// lane; the `getThunderLevel` walk multiplies the thunder value by
    /// the rain level (the effective thunder the splash walk speaks).
    rain_level: f32,
    thunder_level: f32,
    /// THE GAME-MODE ECHO'S OWN WORD (javap 26.2
    /// `ClientPacketListener.handleGameEvent` pcs 104-126 — the
    /// CHANGE_GAME_MODEL arm the local mode re-syncs: the
    /// `floor(param + 0.5)` fold (pcs 37-46, computed BEFORE the arms)
    /// then `gameMode.setLocalMode(GameType.byId(id))`): the folded id
    /// the server's own answer banks — `None` until the echo speaks
    /// (the join's login word the snapshot carries, the echo the
    /// server's LATER changes ride). The frame thread's App bank reads
    /// it through the shipped refresh (the setLocalMode walk — the
    /// previous bank, the mode bank, the abilities re-grant — the
    /// installer's own law).
    local_game_mode: Option<u8>,
    /// THE ABILITIES ECHO'S OWN BANK (javap 26.2
    /// `ClientPacketListener.handlePlayerAbilities` — the server's own
    /// abilities word the ClientboundPlayerAbilitiesPacket keeps: the
    /// ONE byte the four flag bits fold, the two speed floats the read
    /// ctor takes unconditionally): the full word the bank keeps —
    /// `None` until the echo lands, `Some(word)` the latest echo
    /// replaces (the refresh rides it verbatim, the frame thread's
    /// installer the six-putfield walk wears — the Body-owned fields
    /// the trio banks, the mayfly/instabuild/walking-speed words the
    /// model's own bank preserves).
    abilities_word: Option<mc_protocol::play::PlayerAbilitiesWord>,
    /// THE DIFFICULTY'S OWN WIRE BANK (javap 26.2
    /// `ClientPacketListener.handleChangeDifficulty` — the
    /// ClientboundChangeDifficultyPacket the levelData pair stores:
    /// `setDifficulty(packet.difficulty())` +
    /// `setDifficultyLocked(packet.locked())`): the wire's own
    /// `(difficulty id, locked)` pair `None` until the packet speaks
    /// (the join's world the absent lane the default Normal word
    /// wears — the vanilla ClientLevelData's own default), `Some`
    /// the latest packet replaces. The RESPAWN the bank KEEPS: the
    /// handleRespawn walk the new ClientLevelData the OLD one's
    /// `getDifficulty()` ctor arg re-seats — the difficulty (and its
    /// lock) the level swap carries across, the fresh packet the
    /// server's later turn speaks.
    wire_difficulty: Option<(u8, bool)>,
    /// The latest Set Time's clock states, keyed by order of arrival
    /// (a vanilla overworld server syncs its dimension's default
    /// clock). Multi-clock worlds need the WORLD_CLOCK registry names
    /// to disambiguate — the first entry serves until that parse
    /// lands; with NONE the world is honestly at day (no opinion).
    clocks: Vec<ClockUpdate>,
    columns: HashMap<(i32, i32), ChunkColumn>,
    /// The world's living population, keyed by entity id (the Add
    /// Entity packet's numeric key). Cleared on Respawn — the entities
    /// of the crossed dimension's level do not follow the player.
    entities: HashMap<i32, EntitySnapshot>,
    /// THE RIDE BELIEF: the vehicle's entity id -> its FULL passenger
    /// list (the Set Passengers packet's own shape — the vanilla
    /// ClientPacketListener.handleSetEntityPassengersPacket walk: the
    /// unknown vehicle the warn-and-return keeps, the eject + the
    /// startRiding walk the list rebuild speaks). Cleared on Respawn
    /// (the vehicles ARE the entities) and pruned by the entity
    /// removals (the dead mount takes its riders' belief away).
    vehicles: HashMap<i32, Vec<i32>>,
    /// The LOCAL player's active mob effects, keyed by the mob-effect
    /// registry id. Only the own-entity traffic lands here (vanilla's
    /// LocalPlayer carries its own effects; the lightmap consumes the
    /// LOCAL player's state — other entities' effect packets have no
    /// client surface and are dropped). NOT cleared on Respawn: the
    /// effect rides the PLAYER (the server-side MobEffectInstance
    /// lives on the entity that crosses the dimension and vanilla's
    /// client keeps LocalPlayer effects across a level recreation),
    /// unlike the columns and the entity registry.
    effects: HashMap<i32, MobEffectState>,
    /// The LOCAL player's inventory window (window 0) — the
    /// `inventoryMenu` lane the audited consumer routes container
    /// packets through. NOT cleared on Respawn (rides the player).
    inventory: InventoryState,
    /// The OPEN container window — the `containerMenu` lane the
    /// audited consumer routes foreign-window container packets
    /// through (`None` while no screen is open; see
    /// [`OpenContainerState`]). NOT cleared on Respawn (the wire's
    /// own close/open moves it).
    open_container: Option<OpenContainerState>,
    /// The LOCAL player's vitals (the Set Health packet's own store:
    /// the health float, the food, the saturation) — `None` until the
    /// server speaks one (the vanilla's own default rides the login's
    /// entity data, our lane's honest absence until the packet lands).
    pub health: Option<SetHealth>,
    /// The LOCAL player's hurt flash (the Hurt Animation packet's own
    /// store: the 10-tick window the animateHurt law arms + the tilt
    /// yaw float the packet carries) — `None` while unhurt.
    pub hurt: Option<(i32, f32)>,
    /// The LOCAL player's death message (the Player Combat Kill
    /// packet's own store — the death screen's flattened component);
    /// cleared on the next Respawn's world adoption.
    pub death_message: Option<String>,
    /// The wire mobs' PENDING hurt arms (the Hurt Animation packets
    /// the own-entity filter passes over — ClientLevel's
    /// handleHurtAnimation names each entity, LivingEntity's
    /// animateHurt arms the 10-tick window): the id → the window the
    /// packet armed. The store is a MAILBOX, not a standing clock —
    /// the refresh ships each arm exactly once (the projection reads
    /// it, the mob lane's own tick owns the countdown after the
    /// landing) and [`WorldModel::drain_entity_hurts`] empties it; a
    /// fresh packet re-arms. Dies with the level (the dimension
    /// swap's population does not cross).
    pub entity_hurts: std::collections::BTreeMap<i32, i32>,
    /// The wire mobs' EQUIPMENT STATE (the Set Equipment packets —
    /// the slot snapshots the server broadcasts:
    /// `ClientPacketListener.handleSetEquipment`'s own walk merges
    /// the packet's slots into the entity's gear, the ABSENT slots
    /// the snapshot never spoke keep their word): the id → the slot
    /// ordinal (the wire's own declaration index — mainhand 0, the
    /// enum's `as u8` the packet's own order speaks) → the (item id,
    /// count) the stack carries. The store is STATE, not a mailbox —
    /// the merge replaces only the packet's slots (the OPTIONAL
    /// codec's empty stack — count <= 0, item 0 — the slot CLEARS),
    /// the never-spoken entity the honest empty. Dies with the level
    /// (the dimension swap's population does not cross).
    pub entity_equipment:
        std::collections::BTreeMap<i32, std::collections::BTreeMap<u8, (i32, i32)>>,
    /// The mobs' PENDING VOICES (the Entity Sound packets — the
    /// ambient/hurt/death sounds the SERVER plays and broadcasts:
    /// `Mob.baseTick`'s playAmbientSound, `LivingEntity.hurt`'s
    /// playHurtSound, `die`'s tail): the arms in wire order, each the
    /// entity id + the sound's NAME (the registry arm resolved through
    /// the synced sound_event table, the direct lane's own location —
    /// `None` when the id lies outside the table, the honest
    /// no-voice) + the volume/pitch. A MAILBOX like the hurts' — the
    /// refresh ships each voice exactly once, the drain empties it.
    pub entity_sounds: Vec<EntityVoiceArm>,
    /// The arrows' PENDING SYNC MAILBOX — the Entity Position Sync
    /// packets' words for `minecraft:arrow` entities (see
    /// [`ArrowSyncArm`]): the server's authoritative corrections the
    /// client-side sim lane re-poses at, shipped once with the refresh
    /// that carried them (the worker's own drain empties the mailbox
    /// after — each word rides exactly one ship). The OTHER entity
    /// types never enter it: the mobs and the hulls ride the store's
    /// own projection law (the lerp the refresh serves), the arrows
    /// are the lane whose client-side sim never reads the store back.
    pub arrow_syncs: Vec<ArrowSyncArm>,
    /// The world's PENDING VOICES (the Sound packets — the GLOBAL
    /// sounds the SERVER plays at a world spot: the doors and the
    /// machines the block-use lanes trigger, the weather, the
    /// server-side effects): the arms in wire order, each the
    /// sound's NAME (the registry arm resolved through the synced
    /// sound_event table, the direct lane's own location — `None`
    /// when the id lies outside the table, the honest no-voice) +
    /// the WORLD SPOT the packet carried + the volume/pitch. A
    /// MAILBOX like the voices' — the refresh ships each exactly
    /// once, the drain empties it, the dimension swap clears it.
    pub world_sounds: Vec<WorldVoiceArm>,
    /// THE GROWTH EFFECT'S OWN MAILBOX (the Level Event packets the
    /// bone-meal item's use broadcasts — [`mc_protocol::GROWTH_EFFECT`]
    /// the vocabulary speaks): the arms in wire order, each the
    /// event's anchor block + the data word the particle count's law
    /// reads. A MAILBOX like the voices' — the refresh ships each
    /// exactly once, the drain empties it, the dimension swap clears
    /// it. An event id OUTSIDE the vocabulary (the mob-spawner flames,
    /// the wither's broadcast, the rest of the constants class) the
    /// apply arm never enqueues — the honest silence the vanilla
    /// client's own unwalked cases keep.
    pub growth_events: Vec<GrowthEffectArm>,
    /// THE BLOCK-BREAK EFFECT'S OWN MAILBOX (the Level Event packet's
    /// second vocabulary word — [`mc_protocol::play::DESTROY_BLOCK_EFFECT`]
    /// the case the `LevelEventHandler.levelEvent` offset 1561 speaks:
    /// the server's own digs the event broadcast, the break sound + the
    /// `addDestroyBlockEffect` burst the client's render lane folds).
    /// The arms the state walk resolved (the name the sound table
    /// speaks, the palette kind the burst tile reads — the resolution
    /// the jar's `Block.stateById` + `isAir` + `getSoundType` walk
    /// answers at the handler's own head). The GLOBAL route the arm
    /// never enqueues (the `globalLevelEvent` switch the 2001 id never
    /// carries — the positioned lane the case's own arm wears).
    pub break_events: Vec<BreakEffectArm>,
    /// THE SHOOT-SMOKE EFFECT'S OWN MAILBOX (the Level Event packet's
    /// third vocabulary word — [`mc_protocol::play::SHOOT_SMOKE_EFFECT`]
    /// the case the `LevelEventHandler.levelEvent` offset 933 speaks:
    /// the server's own dispenser the event broadcast, the ten smoke
    /// puffs the `shootParticles` walk the facing fires). The arms the
    /// anchor + the raw direction id carry — the facing fold the render
    /// lane's own law speaks. The GLOBAL route the arm never enqueues
    /// (the `globalLevelEvent` switch the 2000 id never carries — the
    /// positioned lane the case's own arm wears).
    pub shoot_smoke_events: Vec<ShootSmokeArm>,
    /// THE WHITE-SMOKE EFFECT'S OWN MAILBOX (the Level Event packet's
    /// sibling vocabulary word — [`mc_protocol::play::SHOOT_WHITE_SMOKE_EFFECT`]
    /// the case the `LevelEventHandler.levelEvent` offset 919 speaks,
    /// the 2000 word's own `shootParticles` walk the WHITE_SMOKE type
    /// the one arg's delta wears): the server's own crafter the event
    /// carries — the anchor + the raw direction id the arms carry (the
    /// facing fold the render lane's own law speaks, the smoke arm's
    /// own layering). The GLOBAL route the arm never enqueues (the
    /// `globalLevelEvent` switch the 2010 id never carries — the
    /// positioned lane the case's own arm wears).
    pub white_smoke_events: Vec<WhiteSmokeArm>,
    /// THE SPLASH-POTION EFFECT'S OWN MAILBOX (the Level Event packet's
    /// next vocabulary word — [`mc_protocol::play::SPLASH_POTION_EFFECT`]
    /// and [`mc_protocol::play::INSTANT_SPLASH_EFFECT`] the case the
    /// `LevelEventHandler.levelEvent` offset 1199 speaks, BOTH ids the
    /// one arm wears): the server's own potion impacts the event
    /// broadcast, the eight shard burst + the hundred-puff spell ring +
    /// the glass-break voice the client's render lane folds). The arms
    /// the anchor + the raw color word carry — the RGB decode the render
    /// lane's own law speaks, the instant flag the sprite set's own
    /// split answers. The GLOBAL route the arm never enqueues (the
    /// `globalLevelEvent` switch the 2002/2007 ids never carry — the
    /// positioned lane the case's own arm wears).
    pub splash_events: Vec<SplashArm>,
    /// THE MOB-SPAWN CLOUD'S OWN MAILBOX (the Level Event packet's next
    /// vocabulary word — [`mc_protocol::play::MOB_SPAWN_EFFECT`] the
    /// case the `LevelEventHandler.levelEvent` offset 1689 speaks:
    /// the server's own spawner the event broadcast, the twenty
    /// SMOKE + FLAME pairs the `bipush 20` loop fires at the full-block
    /// jitter spots the `(nextDouble() - 0.5) * 2.0` fold places). The
    /// arms the anchor carry — the data word the case NEVER reads (no
    /// `iload_3` read the arm's own bytecode speaks, the arm's body the
    /// anchor alone needs). The GLOBAL route the arm never enqueues
    /// (the `globalLevelEvent` switch the 2004 id never carries — the
    /// positioned lane the case's own arm wears).
    pub mob_spawn_events: Vec<MobSpawnArm>,
    /// THE SCOREBOARD'S OWN BELIEF (the wire trio's store — Set
    /// Objective / Set Display Objective / Set Score + the sibling
    /// Reset Score): the objectives by name, the display slots by id
    /// (0 list, 1 sidebar, 2 below-name, 3..18 the team sidebars),
    /// the scores by (holder, objective). NOT cleared on Respawn: the
    /// vanilla scoreboard is the CONNECTION's own object
    /// (ClientPacketListener holds one for the session; the level's
    /// getScoreboard borrows it — the sidebar survives the dimension
    /// swap exactly like the real client's does).
    pub objectives: std::collections::HashMap<String, ObjectiveState>,
    pub display_slots: std::collections::HashMap<u8, String>,
    pub scores: std::collections::HashMap<(String, String), ScoreState>,
    /// THE TAB'S OWN POPULATION (the Player Info Update / Remove
    /// packets' store — the vanilla `ClientPacketListener.players` map
    /// the PlayerTabOverlay's getPlayerInfos walks): the entries by
    /// profile UUID (the 128 wire bits the entry's own key keeps),
    /// each carrying the name, the game mode, the LISTED bit, the
    /// latency and the list order. NOT cleared on Respawn: the tab is
    /// the CONNECTION's own object (the vanilla map lives on the
    /// listener, one level above the level the Respawn recreates —
    /// the player list survives the dimension swap exactly like the
    /// scoreboard does).
    pub players: std::collections::HashMap<u128, PlayerEntryState>,
    /// THE TAB'S OWN HEADER (the Tab List packet's flattened header
    /// component — the overlay's top band's text). The vanilla
    /// PlayerTabOverlay's header starts NULL (no band until the
    /// server speaks); the house's EMPTY string renders no band the
    /// same honest way (the band draws only when the text is
    /// non-empty — the render's own gate).
    pub tab_header: String,
    /// THE TAB'S OWN FOOTER (the Tab List packet's flattened footer —
    /// the overlay's bottom band's text, the same empty-means-silent
    /// law the header keeps).
    pub tab_footer: String,
}

/// One objective's standing belief (the Set Objective packet's own
/// store): the display component's flattened TEXT the sidebar's
/// title renders, the render type's ordinal verbatim (0 integer, 1
/// hearts — the hearts the model never draws, honest data).
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectiveState {
    pub display: String,
    pub render_type: u8,
}

/// One score row (the Set Score packet's own store): the value the
/// sidebar renders + the display override the row's name keeps
/// (`None` = render the holder's own string, the ownerName walk).
#[derive(Debug, Clone, PartialEq)]
pub struct ScoreState {
    pub value: i32,
    pub display: Option<String>,
}

/// One tab entry's standing belief (the Player Info Update packet's
/// own store — the vanilla `PlayerInfo` the ClientPacketListener
/// keeps): the profile name the ADD_PLAYER carried, the game mode's
/// ordinal verbatim (the UPDATE_GAME_MODE walk's GameType.byId law —
/// 3 spectator), the LISTED bit (the tab's own visibility), the
/// latency milliseconds (the bars' law reads), the list order (the
/// sort's first key). A name the ADD alone never carried is the
/// honest no-entry — an update-only packet for an unknown uuid is a
/// no-op, the wire's own shape.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerEntryState {
    pub name: String,
    pub game_mode: u8,
    pub listed: bool,
    pub latency: i32,
    pub list_order: i32,
}

/// The scoreboard HOLDER's own string (the vanilla
/// `Entity.getScoreboardName` walk — `return this.stringUUID`): the
/// 128 wire bits render as Java's lowercase hyphenated form, the
/// most-significant long first (`8-4-4-4-12` hex words).
pub fn string_uuid(u: u128) -> String {
    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        (u >> 96) & 0xFFFF_FFFF,
        (u >> 80) & 0xFFFF,
        (u >> 64) & 0xFFFF,
        (u >> 48) & 0xFFFF,
        u & 0xFFFF_FFFF_FFFF,
    )
}

/// One pending mob voice — the Entity Sound packet's resolution the
/// model keeps until the refresh ships it (see
/// [`WorldModel::entity_sounds`]).
#[derive(Debug, Clone, PartialEq)]
pub struct EntityVoiceArm {
    /// The sounding entity's id (the mob lane's own match key).
    pub entity_id: i32,
    /// The sound event's name — the registry arm resolved through the
    /// synced sound_event table, the direct lane's own location;
    /// `None` when the registry id lies outside the table (the honest
    /// no-voice the projection drops).
    pub name: Option<String>,
    /// The volume, verbatim from the packet.
    pub volume: f32,
    /// The pitch, verbatim from the packet.
    pub pitch: f32,
}

/// One pending WORLD voice — the Sound packet's resolution the model
/// keeps until the refresh ships it (see
/// [`WorldModel::world_sounds`]).
#[derive(Debug, Clone, PartialEq)]
pub struct WorldVoiceArm {
    /// The sound event's name — the registry arm resolved through the
    /// synced sound_event table, the direct lane's own location;
    /// `None` when the registry id lies outside the table (the honest
    /// no-voice the projection drops).
    pub name: Option<String>,
    /// The world spot the packet carried (the 1/8-grid ints the
    /// parse's /8 law read) — the ear's own distance law folds the
    /// gain from it.
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// The volume, verbatim from the packet.
    pub volume: f32,
    /// The pitch, verbatim from the packet.
    pub pitch: f32,
}

/// One pending GROWTH EFFECT arm — the Level Event packet's word the
/// model keeps until the refresh ships it (see
/// [`WorldModel::growth_events`]). The anchor and the data word ride
/// verbatim (the block the effect speaks AT, the count law's input);
/// the GLOBAL flag is deliberately absent — the growth effect is the
/// POSITIONED lane's own word, the broadcast events the vocabulary
/// never enqueues.
#[derive(Debug, Clone, PartialEq)]
pub struct GrowthEffectArm {
    /// The event's anchor block, verbatim (the packet's packed LONG
    /// decoded — the grass/crop cell the bone-meal walk spoke at).
    pub x: i32,
    pub y: i32,
    pub z: i32,
    /// The event's data word, verbatim (the particle count the render
    /// lane's own law folds — the server's `bipush 15`).
    pub data: i32,
}

/// One pending BLOCK-BREAK EFFECT arm — the Level Event packet's second
/// vocabulary word the model resolved (see
/// [`WorldModel::break_events`]). The state walk's answers ride, not
/// the raw state id: `sound_name` the break voice's own word (the
/// stripped registry name the sound table speaks — `None` the AIR
/// state the `isAir` gate keeps, the sound the case never plays),
/// `burst_kind` the debris burst's own tile source (the palette kind
/// the state's block names — `None` the kinds the palette never
/// banks: the tile the burst never finds, the sound the honest ride
/// keeps). The anchor verbatim.
#[derive(Debug, Clone, PartialEq)]
pub struct BreakEffectArm {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    /// The break voice's own word (the stripped registry name — the
    /// sound table's row the audio lane resolves; `None` the AIR
    /// state's gate keeps).
    pub sound_name: Option<&'static str>,
    /// The burst's own palette kind (the particle tile the atlas
    /// carries; `None` the unbanked kinds the burst never finds).
    pub burst_kind: Option<u16>,
}

/// One pending SHOOT-SMOKE arm — the Level Event packet's third
/// vocabulary word the model keeps verbatim (see
/// [`WorldModel::shoot_smoke_events`]). The DATA word rides raw (the
/// direction id the `Direction.from3DDataValue` fold resolves in the
/// render lane — the same layering the growth arm's own data keeps:
/// the model's mailbox is a transport, the facing fold the walk's own
/// read speaks). The anchor verbatim.
#[derive(Debug, Clone, PartialEq)]
pub struct ShootSmokeArm {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    /// The direction id (the `get3DDataValue` word the dispenser's
    /// facing speaks — 0..5 the `DOWN/UP/NORTH/SOUTH/WEST/EAST` band,
    /// the out-of-range words the `abs(irem(data, 6))` fold rides).
    pub data: i32,
}

/// One pending WHITE-SMOKE arm — the Level Event packet's sibling
/// vocabulary word the model keeps verbatim (see
/// [`WorldModel::white_smoke_events`]). The SAME shape the smoke arm
/// wears (the 26.2 case the one `shootParticles` walk both ids call —
/// the `ParticleTypes.WHITE_SMOKE` type the one arg's delta): the DATA
/// word rides raw (the direction id the render lane's
/// `Direction.from3DDataValue` fold resolves — the smoke arm's own
/// layering), the anchor verbatim. The white puff's own tint the
/// render lane's constants carry — the transport's honesty the raw
/// word keeps.
#[derive(Debug, Clone, PartialEq)]
pub struct WhiteSmokeArm {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    /// The direction id (the `get3DDataValue` word the crafter's
    /// facing speaks — the smoke arm's own band the fold rides).
    pub data: i32,
}

/// One pending SPLASH-POTION arm — the Level Event packet's next
/// vocabulary word the model keeps verbatim (see
/// [`WorldModel::splash_events`]). The DATA word rides raw (the potion
/// color the handler's own `(data >> 16 & 255) / 255.0F` decode
/// resolves in the render lane — the same transport layering the
/// smoke arm's direction id keeps), the instant flag the sprite set's
/// own split answers (`2007` the `InstantEffect` word the arm's own
/// `if_icmpne` branch matched — the model's one match the flag banks,
/// the type fold the walk's own read speaks). The anchor verbatim.
#[derive(Debug, Clone, PartialEq)]
pub struct SplashArm {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    /// The potion color word (the ARGB bytes the server's own option
    /// carries — the red the `>> 16` band, the green the `>> 8`, the
    /// blue the `>> 0`, the alpha the vanilla's own 1.0F create
    /// never reads).
    pub data: i32,
    /// The instant flag (the `InstantEffect` type the `if_icmpne 2007`
    /// branch answers — the `spell_N` sprite set the flag's true arm
    /// picks, the `effect_N` set the regular splash rides).
    pub instant: bool,
}

/// One pending MOB-SPAWN arm — the Level Event packet's next
/// vocabulary word the model keeps verbatim (see
/// [`WorldModel::mob_spawn_events`]). The DATA word the arm
/// deliberately absent — the 26.2 case's raw walk never reads it
/// (no `iload_3` the arm's own bytecode speaks, the zero the
/// spawner's own tail passes the walk ignores), the twenty
/// SMOKE + FLAME pairs the anchor alone places. The anchor
/// verbatim.
#[derive(Debug, Clone, PartialEq)]
pub struct MobSpawnArm {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// One pending ARROW position sync — the Entity Position Sync packet's
/// word for an `minecraft:arrow` entity the model keeps until the
/// refresh ships it (see [`WorldModel::arrow_syncs`]). The position and
/// the exact float angles ride verbatim; the DELTA (velocity) word is
/// deliberately absent — the 26.2 handler's raw walk never reads it
/// (no `setDeltaMovement` call the whole method speaks), the client
/// sim's own flight keeps its physics.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrowSyncArm {
    /// The synced arrow's id (the net-arrow lane's own match key).
    pub id: i32,
    /// The synced position (full-precision doubles, verbatim).
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// The synced yaw, verbatim.
    pub y_rot: f32,
    /// The synced pitch, verbatim.
    pub x_rot: f32,
}

/// The model's view of ONE active mob effect on the LOCAL player — the
/// facts the Update Mob Effect packet has actually delivered, retained
/// verbatim (the amplifier/duration/flags the future HUD and the fade
/// consumer act on; the duration's local tick-down and the fade-out
/// flicker are the documented non-goals).
#[derive(Debug, Clone, PartialEq)]
pub struct MobEffectState {
    /// The amplifier, retained verbatim.
    pub amplifier: i32,
    /// The STATED duration in ticks — never ticked down locally (the
    /// wire's next Update/Remove is the truth; the -1 infinite
    /// sentinel rides verbatim).
    pub duration: i32,
    /// Bit 0 ambient, bit 1 visible, bit 2 show icon, bit 3 blend —
    /// the packet's packing order, retained raw.
    pub flags: u8,
}

/// The view's wire-shaped effect quad (the model store's own snapshot —
/// the fields the HUD's effect lane reads, copied out of the model's
/// map; the `Clone`-owned shape the HealthView's transport carries).
#[derive(Debug, Clone, PartialEq)]
pub struct MobEffectWireState {
    pub effect_id: i32,
    pub amplifier: i32,
    pub duration: i32,
    pub flags: u8,
}

/// `minecraft:night_vision`'s mob-effect registry id — the 0-based
/// built-in registration (= wire) order, now GENERATED:
/// `mc_registry::generated::mob_effects::MOB_EFFECT_ORDER` (the checked-in
/// DumpMobEffects.java boots the official jar's real registries; the
/// night_vision test below pins this const to the table's slot). The
/// legacy 1-based wiki ids are NOT the wire: the holderRegistry lane
/// carries the 0-based registry id.
pub const MOB_EFFECT_NIGHT_VISION: i32 = 15;

/// The LOCAL player's inventory window (window 0) — the facts the
/// container packets have actually delivered. The consumer split is
/// AUDITED (javap of the 26.2 `ClientPacketListener`):
/// `handleContainerContent` and `handleContainerSetSlot` both route
/// `containerId == 0` to **`player.inventoryMenu`** — the player's OWN
/// inventory, live whether or not a container screen is open — while
/// any other window id goes to `player.containerMenu` (the currently
/// OPEN container) or is dropped on mismatch. The model's store is the
/// inventoryMenu lane; the open-container lane waits for the container
/// GUI rounds. NOT cleared on Respawn (the inventory rides the PLAYER,
/// the effects precedent — vanilla's Inventory survives the level
/// recreation).
#[derive(Debug, Clone, PartialEq)]
pub struct InventoryState {
    /// The window's slots in the wire's own numbering (slot 0 = the
    /// result slot, 1..5 craft, 5..9 armor, 9..36 the main grid, 36..45
    /// the hotbar — the USE_ROW — and 45 the offhand/shield slot; the
    /// `InventoryMenu` constants' audited values, verbatim from the
    /// jar). The full snapshot REPLACES the list; the single-slot delta
    /// upserts in place (a slot past the current end GROWS the list
    /// with empty stacks — the retention line; the wire never
    /// legitimately addresses past the menu).
    pub slots: Vec<ItemStack>,
    /// The revision counter the server bumps per change — the state
    /// echo's future input, retained verbatim from whichever packet
    /// spoke last.
    pub state_id: i32,
    /// The GUI cursor stack (the Set Content packet's carriedItem —
    /// the stack riding the mouse; the slot packet has no carried
    /// lane and leaves it untouched).
    pub carried: ItemStack,
    /// The SELECTED hotbar slot — SELECTION-relative 0..8 (the
    /// number keys' own numbering, `Inventory.SELECTION_SIZE` = 9;
    /// NOT the window's slot numbering whose hotbar row lives at
    /// 36..45). Rides the Set Held Slot packet's audited gate; the
    /// container snapshots do NOT touch it (vanilla's
    /// initializeContents sets slots/carried/stateId only).
    pub selected: usize,
}

/// The store's own empty: an EMPTY stack is the wire's count-sentinel
/// shape (`ItemStack` carries no `Default` — the sentinel is the
/// protocol's explicit construction, and the store mirrors it).
impl Default for InventoryState {
    fn default() -> Self {
        InventoryState {
            slots: Vec::new(),
            state_id: 0,
            carried: ItemStack {
                count: 0,
                item_id: 0,
                patch: ComponentPatch::default(),
            },
            selected: 0,
        }
    }
}

/// The `InventoryMenu` window id the audited consumer routes to the
/// player's own inventory (`handleContainerContent`'s `ifne` on the
/// container id; the constant's jar value is 0).
pub const INVENTORY_WINDOW_ID: i32 = 0;

/// `InventoryMenu.USE_ROW_SLOT_START` — the hotbar's first window slot
/// (the jar's own constant: 36).
pub const USE_ROW_SLOT_START: usize = 36;
/// `InventoryMenu.USE_ROW_SLOT_END` — one past the hotbar's last
/// window slot (the jar's own constant: 45 — nine slots, the number
/// keys' row).
pub const USE_ROW_SLOT_END: usize = 45;
/// `InventoryMenu.SHIELD_SLOT` — the offhand's window slot (the jar's
/// own constant: 45).
pub const SHIELD_SLOT: usize = 45;

/// `MenuType`'s generic_9x3 — the chest's registry id (javap of the
/// 26.2 `MenuType` clinit's registration order: generic_9x1 0 ..,
/// the 9x3 grid lands at 2; the wire resolves through the synced
/// registry — vanilla-to-vanilla pins this number).
pub const MENU_TYPE_CHEST: i32 = 2;

/// `MenuType`'s crafting — the CraftingMenu's registry id (the SAME
/// clinit order speaks: generic_9x6 5, generic_3x3 6, crafter_3x3 7,
/// anvil 8, beacon 9, blast_furnace 10, brewing_stand 11, CRAFTING
/// lands at 12 — the wire's own word the live table lane routes on).
pub const MENU_TYPE_CRAFTING: i32 = 12;

/// An OPEN container window — the `containerMenu` lane the audited
/// consumer routes foreign-window container packets through (javap of
/// the 26.2 `ClientPacketListener`): the Open Screen packet BIRTHS the
/// menu (`MenuScreens.create(type, minecraft, containerId, title)` —
/// the screen constructs the menu FOR the window id via
/// `menuType.create(containerId, player.getInventory())`), the Set
/// Content / Set Slot packets feed it ONLY when their window id
/// MATCHES `player.containerMenu.containerId` (the fresh javap: the
/// `if_icmpne` falls through to return — the mismatched window is
/// dropped), and the Container Close packet resets `containerMenu` to
/// `inventoryMenu` — ONE assignment (`Player.closeContainer`), NO id
/// check, the inventory store untouched. The store holds the menu's
/// own slot numbering (a chest window's menu carries the container's
/// slots AND the player-inventory mirror rows — whatever the wire
/// delivered, verbatim). NOT cleared on Respawn (the wire's own
/// close/open moves it; vanilla's death-flow GUI close is a
/// screen-level nicety the model does not simulate).
#[derive(Debug, Clone, PartialEq)]
pub struct OpenContainerState {
    /// The window id the server assigned — the match key the foreign
    /// container traffic rides.
    pub id: i32,
    /// The MenuType registry id — the holder VarInt, retained
    /// verbatim (the resolution is the synced registry's act; the
    /// parse's doc table names vanilla's built-in order).
    pub menu_type_id: i32,
    /// The title's network-NBT bytes, retained verbatim (the
    /// text-render round owns the decode; the wire's bytes are the
    /// store's title).
    pub title_nbt: Vec<u8>,
    /// The menu's slots in the menu's own numbering — the snapshot
    /// REPLACES the list; the single-slot delta upserts in place (a
    /// slot past the current end GROWS with empty stacks — the
    /// retention line; the wire never legitimately addresses past
    /// the menu).
    pub slots: Vec<ItemStack>,
    /// The revision counter — whichever packet spoke last is the
    /// echo's truth.
    pub state_id: i32,
    /// The GUI cursor stack (the Set Content packet's carriedItem;
    /// the slot packet has no carried lane and leaves it untouched).
    pub carried: ItemStack,
}

/// `InventoryMenu.isHotbarSlot`'s audited shape — NOTE THE QUIRK: the
/// bytecode returns true for the half-open range 36..45 AND for slot
/// 45 itself (the `slot == 45` fall-through at `if_icmpne 22`), so the
/// helper treats the OFFHAND pickup like a hotbar pickup at its
/// call site (the Set Slot packet's pop-time nicety). The hotbar
/// SLICE is the layout's `USE_ROW_SLOT_START..USE_ROW_SLOT_END`; the
/// HELPER is the helper — both facts kept separate, the wiki-era
/// `36..45` claim describing only one of them.
#[cfg_attr(not(test), expect(dead_code))]
pub fn is_hotbar_slot(slot: i32) -> bool {
    (36..45).contains(&slot) || slot == 45
}

/// The container stores' slot-write policy, shared by the
/// inventory-menu and open-container lanes: a NEGATIVE slot cannot
/// index anything and is ignored (the parse retained it verbatim, the
/// consumer's bounds check is the consumer's act); a slot past the
/// current end GROWS the list with empty stacks (the retention line —
/// vanilla's `AbstractContainerMenu.setItem` would throw on the same
/// shape, the growth is the port's own convention banked with the
/// inventory round, and the wire never legitimately addresses past
/// the menu); a live slot is REPLACED.
fn store_slot_write(slots: &mut Vec<ItemStack>, slot: i16, item: &ItemStack) {
    if slot >= 0 {
        let idx = slot as usize;
        if idx >= slots.len() {
            slots.resize(
                idx + 1,
                ItemStack {
                    count: 0,
                    item_id: 0,
                    patch: ComponentPatch::default(),
                },
            );
        }
        slots[idx] = item.clone();
    }
}

impl Default for WorldModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Java's `Math.round(double)`: the closest long with TIES TOWARD
/// POSITIVE INFINITY. Rust's `f64::round` rounds halves AWAY FROM
/// ZERO — on a negative half-grid base the two differ by one 1/4096
/// grid step (`Math.round(-0.5)` is 0, `(-0.5f64).round()` is -1),
/// exactly the negative-coordinate class the wire keeps burning, so
/// the walk pins the vanilla direction.
fn vanilla_round(d: f64) -> f64 {
    let r = d.floor();
    if d - r >= 0.5 {
        r + 1.0
    } else {
        r
    }
}

/// ONE axis of the `VecDeltaCodec` walk (javap of the 26.2
/// `VecDeltaCodec.decode(JJJ)`, the per-tick move's own arithmetic): a
/// ZERO delta keeps the axis EXACT — the lane returns the base
/// untouched, no quantization; a non-zero delta moves the QUANTIZED
/// base, `(Math.round(base * 4096) + delta) / 4096` — every real step
/// snaps the accumulated position onto the 1/4096 grid.
fn delta_axis(base: f64, delta: i16) -> f64 {
    if delta == 0 {
        base
    } else {
        (vanilla_round(base * 4096.0) + delta as f64) / 4096.0
    }
}

/// THE RELATIVE BITS (the 1.21.2+ Relative enum's own ordinals — the
/// packet's flags int the bits speak; the javap-proven order X, Y, Z,
/// Y_ROT, X_ROT, DELTA_X, DELTA_Y, DELTA_Z, ROTATE_DELTA).
const RELATIVE_X: i32 = 1 << 0;
const RELATIVE_Y: i32 = 1 << 1;
const RELATIVE_Z: i32 = 1 << 2;
const RELATIVE_Y_ROT: i32 = 1 << 3;
const RELATIVE_X_ROT: i32 = 1 << 4;
const RELATIVE_DELTA_X: i32 = 1 << 5;
const RELATIVE_DELTA_Y: i32 = 1 << 6;
const RELATIVE_DELTA_Z: i32 = 1 << 7;
const RELATIVE_ROTATE_DELTA: i32 = 1 << 8;

/// THE SIN TABLE'S OWN CELL (Mth.SIN's static fill —
/// `lambda$static$0`'s own walk: `SIN[i] = (float)Math.sin((double)i /
/// 10430.378350470453)`, the DIVISION the bytecode keeps).
fn table_sin_cell(idx: i64) -> f32 {
    (((idx as f64) / 10430.378350470453).sin()) as f32
}

/// ONE Mth.sin(D)F of the bytecode's own walk: the angle's 65536-step
/// table quantization — `SIN[(int)((long)(angle * 10430.378350470453)
/// & 65535)]` — the cell the table's own construction computes. The
/// quantization IS the law: nearby angles share a cell, the walk never
/// interpolates. (The boat hurt tilt's submit fold reads it too —
/// `Mth.sin((double)hurtTime)` the extract's f2d walk keeps.)
pub(crate) fn narrow_sin(angle: f32) -> f64 {
    let idx = (f64::from(angle) * 10430.378350470453) as i64 & 65535;
    f64::from(table_sin_cell(idx))
}

/// ONE Mth.cos(D)F — the quarter-turn shift the +16384.0 speaks
/// (65536 steps / 4): `SIN[(int)((long)(angle * 10430.378350470453 +
/// 16384.0) & 65535)]` (see [`narrow_sin`]).
pub(crate) fn narrow_cos(angle: f32) -> f64 {
    let idx = (f64::from(angle) * 10430.378350470453 + 16384.0) as i64 & 65535;
    f64::from(table_sin_cell(idx))
}

/// THE POSITION SYNC'S OWN FOLD (PositionMoveRotation.calculateAbsolute's
/// own walk — ClientPacketListener.setValuesFromPositionPacket's first
/// step, the javap-proven 26.2 law): the stored pose the CURRENT the
/// fold reads, the packet's relative bits the ADD arms speak, the
/// absent bits the REPLACE arms keep. Position x/y/z, yaw, and the
/// pitch the Mth.clamp's ±90 band keeps; the velocity word the
/// ROTATE_DELTA bit may spin FIRST (Vec3.xRot then Vec3.yRot — the
/// bytecode's own order; the angles the current-minus-new deltas
/// speak), then each axis calculateDelta's (the relative bit adds,
/// the absent bit replaces). The teleport id and the flags ride
/// verbatim — the fold is the pose's act, never the record's rewrite.
fn fold_position_sync(current: &PlayerPosition, p: &PlayerPosition) -> PlayerPosition {
    let has = |bit: i32| p.flags & bit != 0;
    let x = if has(RELATIVE_X) {
        current.x + p.x
    } else {
        p.x
    };
    let y = if has(RELATIVE_Y) {
        current.y + p.y
    } else {
        p.y
    };
    let z = if has(RELATIVE_Z) {
        current.z + p.z
    } else {
        p.z
    };
    let yaw = if has(RELATIVE_Y_ROT) {
        current.yaw + p.yaw
    } else {
        p.yaw
    };
    let pitch = if has(RELATIVE_X_ROT) {
        (current.pitch + p.pitch).clamp(-90.0, 90.0)
    } else {
        p.pitch
    };
    // THE DELTA'S OWN WALK (the current delta the rotation arm may
    // spin; the angles the f32-cast radians the d2f keeps)
    let (mut dx, mut dy, mut dz) = (current.dx, current.dy, current.dz);
    if has(RELATIVE_ROTATE_DELTA) {
        // Vec3.xRot first: y' = y·cos + z·sin, z' = z·cos − y·sin
        // (the angles the f32 fsub THEN the f64 toRadians the d2f
        // cast keeps — the bytecode's own precision ladder)
        let angle = ((current.pitch - pitch) as f64).to_radians() as f32;
        let (c, s) = (narrow_cos(angle), narrow_sin(angle));
        let (ny, nz) = (dy * c + dz * s, dz * c - dy * s);
        dy = ny;
        dz = nz;
        // Vec3.yRot second: x' = x·cos + z·sin, z' = z·cos − x·sin
        let angle = ((current.yaw - yaw) as f64).to_radians() as f32;
        let (c, s) = (narrow_cos(angle), narrow_sin(angle));
        let (nx, nz) = (dx * c + dz * s, dz * c - dx * s);
        dx = nx;
        dz = nz;
    }
    let dx = if has(RELATIVE_DELTA_X) {
        dx + p.dx
    } else {
        p.dx
    };
    let dy = if has(RELATIVE_DELTA_Y) {
        dy + p.dy
    } else {
        p.dy
    };
    let dz = if has(RELATIVE_DELTA_Z) {
        dz + p.dz
    } else {
        p.dz
    };
    PlayerPosition {
        teleport_id: p.teleport_id,
        x,
        y,
        z,
        dx,
        dy,
        dz,
        yaw,
        pitch,
        flags: p.flags,
    }
}

impl WorldModel {
    #![allow(dead_code)] // wired into the frame loop when the mesher input lands

    pub fn new() -> Self {
        WorldModel {
            center: (0, 0),
            info: None,
            player: None,
            min_y: None,
            current_dimension: None,
            registry: RegistryData::default(),
            game_time: 0,
            rain_level: 0.0,
            thunder_level: 0.0,
            local_game_mode: None,
            abilities_word: None,
            wire_difficulty: None,
            clocks: Vec::new(),
            columns: HashMap::new(),
            entities: HashMap::new(),
            vehicles: HashMap::new(),
            effects: HashMap::new(),
            inventory: InventoryState::default(),
            open_container: None,
            health: None,
            hurt: None,
            death_message: None,
            entity_hurts: std::collections::BTreeMap::new(),
            entity_equipment: std::collections::BTreeMap::new(),
            entity_sounds: Vec::new(),
            arrow_syncs: Vec::new(),
            world_sounds: Vec::new(),
            growth_events: Vec::new(),
            break_events: Vec::new(),
            shoot_smoke_events: Vec::new(),
            white_smoke_events: Vec::new(),
            splash_events: Vec::new(),
            mob_spawn_events: Vec::new(),
            objectives: std::collections::HashMap::new(),
            display_slots: std::collections::HashMap::new(),
            scores: std::collections::HashMap::new(),
            players: std::collections::HashMap::new(),
            tab_header: String::new(),
            tab_footer: String::new(),
        }
    }

    /// One entity's current snapshot, by id.
    pub fn entity(&self, id: i32) -> Option<&EntitySnapshot> {
        self.entities.get(&id)
    }

    /// The world's whole population, id + snapshot pairs (the Add
    /// Entity store's own walk — the id KEYS the map, the render
    /// lane's projection rides both).
    pub fn entities(&self) -> impl Iterator<Item = (i32, &EntitySnapshot)> {
        self.entities.iter().map(|(id, e)| (*id, e))
    }

    /// THE DIFFICULTY'S OWN WIRE READ (the bank's own accessor — the
    /// `ClientLevelData.getDifficulty()` + `isDifficultyLocked()` pair
    /// the materialize walk and the WorldOptions dial's networked lane
    /// read): `None` the absent packet the default Normal word wears,
    /// `Some((id, locked))` the latest Change Difficulty packet keeps.
    pub fn wire_difficulty(&self) -> Option<(u8, bool)> {
        self.wire_difficulty
    }

    /// WHO CARRIES this entity right now (the inverse walk of the
    /// belief — the passenger's id scanned across the vehicles' lists;
    /// the map's own size the vanilla client's passenger sets keep
    /// small). The silent walk is the honest None (nothing carries).
    pub fn vehicle_of(&self, passenger: i32) -> Option<i32> {
        self.vehicles
            .iter()
            .find(|(_, riders)| riders.contains(&passenger))
            .map(|(vehicle, _)| *vehicle)
    }

    /// THE PASSENGER'S SEAT INDEX inside its vehicle's list (the
    /// AbstractBoat.getPassengerAttachmentPoint's own indexOf — the
    /// multi-rider seat offsets the index speaks). `None` when nothing
    /// carries the entity.
    pub fn passenger_index(&self, passenger: i32) -> Option<usize> {
        self.vehicles
            .iter()
            .find_map(|(_, riders)| riders.iter().position(|&r| r == passenger))
    }

    /// The VEHICLE'S OWN rider list (the belief's verbatim read — the
    /// seat count the multi-rider offsets gate on). `None` when the id
    /// carries nobody.
    pub fn vehicle_passengers(&self, vehicle: i32) -> Option<&Vec<i32>> {
        self.vehicles.get(&vehicle)
    }

    /// The mobs' hurt mailbox's own drain (`std::mem::take` — the
    /// vanilla `Entity.hurtTime` the client entity counts down is the
    /// MOB's clock, not the store's): the refresh ships each arm
    /// exactly once — the projection reads the map, the mob lane's
    /// own tick owns the countdown after the landing — and the drain
    /// empties it, so a stale arm never re-arms a flash the mob's
    /// own window already spent. A fresh packet re-arms.
    pub fn drain_entity_hurts(&mut self) -> std::collections::BTreeMap<i32, i32> {
        std::mem::take(&mut self.entity_hurts)
    }

    /// THE MOB-HELD WORD (the MAINHAND slot's own read — the record's
    /// `held_weapon` the render lane draws): the equipment state's
    /// slot-0 stack, the count>0/item!=air gate the non-empty law
    /// keeps, the item id the registry's wire order resolves to the
    /// BARE name (the same word the spawn roll's equipment speaks —
    /// the render's own model walk reads it). The never-spoken entity
    /// (or the cleared slot) the honest None keeps.
    pub fn entity_mainhand_word(&self, id: i32) -> Option<&'static str> {
        let (item_id, count) = *self.entity_equipment.get(&id)?.get(&0)?;
        if count <= 0 || item_id == 0 {
            return None;
        }
        mc_registry::item_name(item_id).filter(|name| *name != "air")
    }

    /// THE OFFHAND WORD (the store's slot-1 read — javap
    /// ItemInHandLayer.submit's SECOND submitArmWithItem the LEFT arm
    /// the vanilla client renders): the same non-empty gate the
    /// mainhand's own walk keeps, the bare name the registry's wire
    /// order resolves. The never-spoken (or cleared) slot the honest
    /// None keeps — the natural spawn the slot never fills.
    pub fn entity_offhand_word(&self, id: i32) -> Option<&'static str> {
        let (item_id, count) = *self.entity_equipment.get(&id)?.get(&1)?;
        if count <= 0 || item_id == 0 {
            return None;
        }
        mc_registry::item_name(item_id).filter(|name| *name != "air")
    }

    /// THE ARMOR WORDS (the store's FEET/LEGS/CHEST/HEAD ordinals 2..5
    /// — `EquipmentSlot.values()`'s own declaration order the wire's
    /// slot bytes index, the two hand slots the mainhand/offhand reads
    /// own): the four rows the record's armor lane carries IN THE
    /// STORE'S OWN WALK ORDER, each the same non-empty gate the hands'
    /// reads keep, the bare name the registry's wire order resolves.
    /// The never-spoken entity (or the cleared slot) the honest None
    /// keeps — the vanilla server the bare mobs' armor rows never
    /// speaks.
    pub fn entity_armor_words(&self, id: i32) -> [Option<&'static str>; 4] {
        let Some(rows) = self.entity_equipment.get(&id) else {
            return [None; 4];
        };
        let word = |ordinal: u8| -> Option<&'static str> {
            let (item_id, count) = *rows.get(&ordinal)?;
            if count <= 0 || item_id == 0 {
                return None;
            }
            mc_registry::item_name(item_id).filter(|name| *name != "air")
        };
        [word(2), word(3), word(4), word(5)]
    }

    /// The mobs' voice mailbox's own drain (`std::mem::take` — the
    /// vanilla `handleEntitySound` plays the voice the moment the
    /// packet lands; the client's lane ships each voice exactly once,
    /// the mob lane's audio edge owns the playback after).
    pub fn drain_entity_sounds(&mut self) -> Vec<EntityVoiceArm> {
        std::mem::take(&mut self.entity_sounds)
    }

    /// The world sounds' mailbox's own drain (the same walk the
    /// voices' speak — each arm ships exactly once): the positioned
    /// voices the Sound packet armed, the consumer's ear law folds
    /// the gain from the arm's own spot.
    pub fn drain_world_sounds(&mut self) -> Vec<WorldVoiceArm> {
        std::mem::take(&mut self.world_sounds)
    }

    /// The growth effect's mailbox's own drain (the same walk the
    /// voices' speak — each arm ships exactly once): the bone-meal
    /// words the Level Event packet armed, the render lane's own law
    /// folds the particle burst from the anchor + the data word.
    pub fn drain_growth_events(&mut self) -> Vec<GrowthEffectArm> {
        std::mem::take(&mut self.growth_events)
    }

    /// The block-break effect's mailbox's own drain (the same walk the
    /// voices' speak — each arm ships exactly once): the 2001 words the
    /// Level Event packet armed, the render lane's own law folds the
    /// break voice + the debris burst from the resolved state.
    pub fn drain_break_events(&mut self) -> Vec<BreakEffectArm> {
        std::mem::take(&mut self.break_events)
    }

    /// The shoot-smoke effect's mailbox's own drain (the same walk the
    /// voices' speak — each arm ships exactly once): the 2000 words the
    /// Level Event packet armed, the render lane's own law folds the
    /// ten-puff smoke walk from the anchor + the direction id.
    pub fn drain_shoot_smoke_events(&mut self) -> Vec<ShootSmokeArm> {
        std::mem::take(&mut self.shoot_smoke_events)
    }

    /// The white-smoke effect's mailbox's own drain (the same walk the
    /// smoke arm's drain speaks — the arms the shipped refresh's own
    /// projection empties).
    pub fn drain_white_smoke_events(&mut self) -> Vec<WhiteSmokeArm> {
        std::mem::take(&mut self.white_smoke_events)
    }

    /// The splash-potion effect's mailbox's own drain (the same walk the
    /// voices' speak — each arm ships exactly once): the 2002/2007 words
    /// the Level Event packet armed, the render lane's own law folds the
    /// shard burst + the spell ring + the glass voice from the anchor +
    /// the raw color word + the instant flag.
    pub fn drain_splash_events(&mut self) -> Vec<SplashArm> {
        std::mem::take(&mut self.splash_events)
    }

    /// The mob-spawn cloud's mailbox's own drain (the same walk the
    /// voices' speak — each arm ships exactly once): the 2004 words
    /// the Level Event packet armed, the render lane's own law folds
    /// the twenty SMOKE + FLAME pairs from the anchor alone (the data
    /// word the case's own body never reads).
    pub fn drain_mob_spawn_events(&mut self) -> Vec<MobSpawnArm> {
        std::mem::take(&mut self.mob_spawn_events)
    }

    /// The arrows' sync mailbox's own drain (the same walk the voices'
    /// speak — each word ships exactly once): the Entity Position Sync
    /// corrections the net-arrow lane re-poses at, the delta word the
    /// 26.2 handler never read stays absent from the arms.
    pub fn drain_arrow_syncs(&mut self) -> Vec<ArrowSyncArm> {
        std::mem::take(&mut self.arrow_syncs)
    }

    /// One entity type's registry name, by the synced-registry id the
    /// Add Entity packet carries (`minecraft:zombie` the mob lane's
    /// render model speaks). `None` when the registry never synced or
    /// the id lies outside the table — the honest unknown, never a
    /// guessed type.
    pub fn entity_type_name(&self, type_id: i32) -> Option<&str> {
        self.registry
            .entity_types
            .get(type_id as usize)
            .map(|s| s.as_str())
    }

    /// The session's own entity id (the play Login packet's playerId) —
    /// the filter the effect-state traffic rides through (the model's
    /// effect store is the LOCAL player's).
    pub fn own_entity_id(&self) -> Option<i32> {
        self.info.as_ref().map(|i| i.entity_id)
    }

    /// The LOCAL player's inventory window (window 0) — the store the
    /// container packets' `inventoryMenu` lane feeds (the audited
    /// consumer split; see [`InventoryState`]).
    pub fn inventory(&self) -> &InventoryState {
        &self.inventory
    }

    /// One slot of the LOCAL player's inventory window, by the window's
    /// own index (the wire's slot numbering, verbatim — see
    /// [`InventoryState::slots`] for the audited layout). The hotbar
    /// row is `USE_ROW_SLOT_START..USE_ROW_SLOT_END`.
    pub fn inventory_slot(&self, index: usize) -> Option<&ItemStack> {
        self.inventory.slots.get(index)
    }

    /// The OPEN container window — the store the container packets'
    /// `containerMenu` lane feeds (the audited consumer split: the
    /// Open Screen births it, the matching-window traffic feeds it,
    /// the Container Close resets it; see [`OpenContainerState`]).
    /// `None` while no screen is open.
    pub fn open_container(&self) -> Option<&OpenContainerState> {
        self.open_container.as_ref()
    }

    /// The SELECTED hotbar slot — the selection-relative index
    /// (`Inventory.SELECTION_SIZE` numbering, 0..8) the Set Held Slot
    /// packet's audited gate moves; the window slot it names is
    /// `USE_ROW_SLOT_START + selected`.
    pub fn selected_slot(&self) -> usize {
        self.inventory.selected
    }

    /// The player's OWN hotbar selection (the vanilla keyPress's local
    /// move — the number keys' lane): the SELECTION-relative 0..8 gate
    /// mirrors the Set Held Slot packet's, the repeat naming the held
    /// slot is the no-op, and the move is the CLIENT's own belief — the
    /// vanilla client updates `Inventory.selected` immediately and the
    /// serverbound Set Carried Item echo tells the server (the
    /// authoritative clientbound Set Held Slot still lands here through
    /// its gate). The hand's projection reads this slot.
    pub fn select_hotbar(&mut self, selection: usize) -> bool {
        if selection < 9 && selection != self.inventory.selected {
            self.inventory.selected = selection;
            true
        } else {
            false
        }
    }

    /// The LOCAL player's active mob-effect state, by the effect's
    /// registry id.
    pub fn mob_effect(&self, effect_id: i32) -> Option<&MobEffectState> {
        self.effects.get(&effect_id)
    }

    /// The effect state's DEBUG surface: the LOCAL player's active
    /// effects as `(id, resolved name, state)` triples, the name lane
    /// resolved through the generated mob-effect table
    /// ([`mc_registry::mob_effect_name`] — the 0-based wire order,
    /// the table's header documents the NOT-synchronized caveat). An
    /// id the table cannot resolve — a server registry newer than
    /// this client jar's boot — names `None` while the STATE still
    /// rides: the honest unknown, never a guessed name (vanilla's
    /// `IdMapper.byId` throws there; the listing keeps walking). The
    /// listing's order is the store's (the map's); the future
    /// HUD/debug-screen rounds own any sort.
    pub fn active_effects_debug(&self) -> Vec<(i32, Option<&'static str>, &MobEffectState)> {
        self.effects
            .iter()
            .map(|(id, st)| (*id, mc_registry::mob_effect_name(*id), st))
            .collect()
    }

    /// The lightmap's night-vision factor — the consumer the mesher's
    /// documented debt awaited (the `NightVisionFactor` extract's
    /// bytecode: `hasEffect(NIGHT_VISION)` ->
    /// `GameRenderer.nightVisionScale` — the 1.0 plateau; the fade-out
    /// flicker `0.7 + sin(..) · 0.3` inside the last-200-tick window
    /// is the documented non-goal, same verdict as the torch
    /// shimmer; the conduit-power branch needs the underwater state
    /// this model does not track, so it stays the 0.0 it always
    /// was). With the effect absent the factor is EXACTLY 0.0 — the
    /// golden scenes' bit-for-bit identity.
    pub fn night_vision_factor(&self) -> f32 {
        if self.effects.contains_key(&MOB_EFFECT_NIGHT_VISION) {
            1.0
        } else {
            0.0
        }
    }

    /// Pin the current dimension and resolve its vertical layout from the
    /// registry when the entry exists (login pins it; Respawn re-pins it
    /// on a dimension change). An unknown key keeps the previous min_y —
    /// the honest state is "no knowledge", never a guess.
    fn pin_dimension(&mut self, dimension: &str) {
        self.current_dimension = Some(dimension.to_string());
        if let Some(min_y) = self.dimension_min_y(dimension) {
            self.min_y = Some(min_y);
        }
    }

    /// Feed one play event into the model. Returns `true` when the event
    /// touched world state (info, position, chunk, unload or center) —
    /// probe echoes and skipped packets never do. The log-free, append-only
    /// discipline of the UI logs does not apply here: the model is the
    /// CURRENT world, so re-sent columns replace and unloads delete.
    pub fn apply(&mut self, event: PlayEvent) -> bool {
        match event {
            PlayEvent::WorldInfo(w) => {
                self.pin_dimension(&w.current_dimension);
                self.info = Some(w);
                true
            }
            PlayEvent::Respawn(spawn) => {
                // a dimension change is a NEW WORLD: the vanilla client
                // recreates its level on Respawn, so every loaded column
                // of the old dimension is meaningless in the new one and
                // would be worse than missing — its blocks re-read under
                // the NEW min_y are pure fiction. The honest state is an
                // empty map the new dimension's chunks fill; the
                // mid-transition window truthfully shows air (the vanilla
                // loading screen's world state).
                self.columns.clear();
                // the entity registry follows the level: a dimension
                // swap's population does not cross (the vanilla client
                // swaps the whole ClientLevel — entities included)
                self.entities.clear();
                // the ride belief follows the level too (the vehicles
                // ARE the entities — the crossed dimension's mounts
                // never carry the new one's rider)
                self.vehicles.clear();
                // the death screen's message dies with the level (the
                // respawn's own world recreation — the vanilla client
                // closes the DeathScreen when the new level lands)
                self.death_message = None;
                self.hurt = None;
                // the mobs' hurt mailbox dies with the level too (the
                // arms name the OLD population's ids — the new level's
                // entities never read them)
                self.entity_hurts.clear();
                // the equipment state dies with the level too (the
                // same ids — the new level's entities never wear the
                // old one's gear)
                self.entity_equipment.clear();
                // the voices' mailbox dies with the level too (the same
                // ids — the new level's population speaks its own)
                self.entity_sounds.clear();
                // the arrows' sync mailbox dies with the level too (the
                // same ids — the new level's shots speak their own)
                self.arrow_syncs.clear();
                // the world sounds' mailbox dies with the level (the
                // same walk — the old level's spots never speak again)
                self.world_sounds.clear();
                // the growth effect's mailbox dies with the level (the
                // same walk — the old level's bone-meal words never
                // burst again)
                self.growth_events.clear();
                // the block-break effect's mailbox dies with the level
                // (the same walk — the old level's 2001 words never
                // burst again)
                self.break_events.clear();
                // the shoot-smoke effect's mailbox dies with the level
                // (the same walk — the old level's 2000 words never
                // puff again)
                self.shoot_smoke_events.clear();
                self.white_smoke_events.clear();
                // the splash-potion effect's mailbox dies with the level
                // (the same walk — the old level's 2002/2007 words never
                // burst again)
                self.splash_events.clear();
                // the mob-spawn cloud's mailbox dies with the level
                // (the same walk — the old level's 2004 words never
                // puff again)
                self.mob_spawn_events.clear();
                self.pin_dimension(&spawn.dimension);
                true
            }
            PlayEvent::Position(p) => {
                // THE RIDING GATE (ClientPacketListener.handleMovePlayer's
                // own pc20 law — `if (!player.isPassenger())` the
                // setValuesFromPositionPacket call never passes while a
                // vehicle holds the player: the boat's own syncs carry the
                // rider, the stored pose stays the mount teleport's word,
                // and the confirm still spoke inside the pump. The
                // dismount's empty-list broadcast drops the belief and the
                // gate opens — the dismount teleport's own word lands then.)
                let riding = self
                    .info
                    .as_ref()
                    .and_then(|i| self.vehicle_of(i.entity_id))
                    .is_some();
                if riding {
                    return true;
                }
                // THE FOLD (PositionMoveRotation.calculateAbsolute's own
                // walk — the stored pose the CURRENT the relative bits
                // read, the absolute bits the replace arms keep; the
                // first sync the zero-baseline the None current keeps,
                // the relative arms' 0.0 + change the verbatim lands)
                self.player = Some(match self.player.as_ref() {
                    Some(cur) => fold_position_sync(cur, &p),
                    None => p,
                });
                true
            }
            PlayEvent::Chunk(col) => {
                self.columns.insert((col.x, col.z), col);
                true
            }
            PlayEvent::Time(ts) => {
                self.game_time = ts.game_time;
                self.clocks = ts.clocks;
                true
            }
            PlayEvent::ChangeDifficulty(w) => {
                // THE LEVELDATA'S OWN STORE (javap 26.2
                // `ClientPacketListener.handleChangeDifficulty` — the
                // `setDifficulty` + `setDifficultyLocked` pair the
                // walk speaks; the open screen's HasDifficultyReaction
                // re-sync the render's own re-read answers — the
                // shipped refresh the WorldOptions dial's next frame
                // wears). The RESPAWN the bank keeps (the handleRespawn
                // walk the new ClientLevelData the OLD one's
                // getDifficulty() ctor arg re-seats).
                self.wire_difficulty = Some((w.difficulty, w.locked));
                true
            }
            PlayEvent::Light { x, z, light } => {
                let Some(col) = self.columns.get_mut(&(x, z)) else {
                    // no column — nothing to relight (vanilla: the chunk
                    // is not loaded; an update for a crossed dimension's
                    // dead column finds nothing after the Respawn drop)
                    return false;
                };
                let total = col.sections.len() + 2;
                // strict: a documented slot outside the column's light
                // range is drift — the WHOLE update is refused and the
                // old light stays (never a half-applied lie)
                let fits = |v: &Vec<Option<LightData>>| {
                    v.iter().enumerate().all(|(i, s)| s.is_none() || i < total)
                };
                if !fits(&light.sky) || !fits(&light.block) {
                    return false;
                }
                // full replace: the update carries the column's COMPLETE
                // light state (the vanilla masks are the engine's full
                // current storage); slots it never mentions drop to the
                // honest None
                let adopt = |v: &Vec<Option<LightData>>| {
                    (0..total)
                        .map(|i| v.get(i).cloned().unwrap_or(None))
                        .collect::<Vec<_>>()
                };
                col.light = ChunkLight {
                    sky: adopt(&light.sky),
                    block: adopt(&light.block),
                };
                true
            }
            PlayEvent::UnloadChunk { x, z } => self.columns.remove(&(x, z)).is_some(),
            PlayEvent::Block { x, y, z, state } => {
                let Some(col) = self.columns.get_mut(&(x >> 4, z >> 4)) else {
                    // no column — vanilla ignores a block change for a
                    // chunk it has not loaded (the update crossed the
                    // Respawn drop, or raced the chunk's arrival)
                    return false;
                };
                // the SAME arithmetic block() reads through (world-y when
                // min_y is known, column-local otherwise) — and the
                // Light event's strict rule: an out-of-range slot
                // refuses the WHOLE update, never a half-applied lie
                let y = match self.min_y {
                    Some(min_y) => y - min_y,
                    None => y,
                };
                let (sy, ly) = (y >> 4, y & 15);
                if sy < 0 {
                    return false;
                }
                let Some(sec) = col.sections.get_mut(sy as usize) else {
                    return false;
                };
                let idx = (ly as usize) * 256 + ((z & 15) as usize) * 16 + (x & 15) as usize;
                let Some(slot) = sec.blocks.get_mut(idx) else {
                    return false;
                };
                // the section's block_count is the wire's PARSE-TIME fact
                // (nothing in this client consumes it) — the blocks
                // array is the model's truth, and it is what changes
                *slot = state;
                true
            }
            PlayEvent::SectionBlocks(sb) => {
                let Some(col) = self.columns.get_mut(&(sb.sx, sb.sz)) else {
                    return false;
                };
                // the wire's sy is the WORLD section y; the column's
                // sections are bottom-first from min_y — the same floor
                // arithmetic block() walks (`(world_base_y - min_y) >> 4`,
                // exact for any min_y), and column-local when min_y is
                // unknown (block()'s own convention)
                let sidx = match self.min_y {
                    Some(min_y) => (sb.sy * 16 - min_y) >> 4,
                    None => sb.sy,
                };
                if sidx < 0 {
                    return false;
                }
                let Some(sec) = col.sections.get_mut(sidx as usize) else {
                    return false;
                };
                // validate ALL entries before ANY mutation (the Light
                // event's all-or-nothing rule — never a half-applied lie)
                let indices: Option<Vec<usize>> = sb
                    .blocks
                    .iter()
                    .map(|&(lx, ly, lz, _)| {
                        let idx = ly as usize * 256 + lz as usize * 16 + lx as usize;
                        if idx < sec.blocks.len() {
                            Some(idx)
                        } else {
                            None
                        }
                    })
                    .collect();
                let Some(indices) = indices else {
                    return false;
                };
                for (idx, &(_, _, _, state)) in indices.iter().zip(sb.blocks.iter()) {
                    sec.blocks[*idx] = state;
                }
                true
            }
            PlayEvent::Spawn(e) => {
                // the registry's insert IS the vanilla semantics: adding
                // an id that already exists REPLACES the old entity (the
                // client's entity lookup is keyed by id)
                self.entities.insert(
                    e.id,
                    EntitySnapshot {
                        type_id: e.type_id,
                        uuid: e.uuid,
                        x: e.x,
                        y: e.y,
                        z: e.z,
                        y_rot: e.y_rot,
                        x_rot: e.x_rot,
                        y_head_rot: e.y_head_rot,
                        data: BTreeMap::new(),
                        // the spawn's own motion word rides (the LP Vec3
                        // the packet carries — the projectile's initial
                        // velocity the flight lane consumes)
                        motion: e.movement,
                    },
                );
                true
            }
            PlayEvent::EntitiesRemoved(ids) => {
                // vanilla ignores unknown ids in the batch (the entity is
                // already gone client-side); the model removes whatever
                // exists and reports whether ANYTHING changed — the same
                // no-op honesty UnloadChunk's `.is_some()` carries
                let mut any = false;
                for id in ids {
                    any |= self.entities.remove(&id).is_some();
                    // the ride belief prunes with the population: a dead
                    // vehicle takes its whole rider list away, a dead
                    // rider's seat empties (the removal IS the auto-
                    // dismount the vanilla server speaks)
                    any |= self.vehicles.remove(&id).is_some();
                    let mut pruned = false;
                    for riders in self.vehicles.values_mut() {
                        let before = riders.len();
                        riders.retain(|&r| r != id);
                        pruned |= riders.len() != before;
                    }
                    any |= pruned;
                }
                any
            }
            PlayEvent::SetPassengers(p) => {
                // THE VANILLA HANDLER'S OWN LAW (ClientPacketListener.
                // handleSetEntityPassengersPacket): the vehicle MUST be
                // a known entity — `level.getEntity(vehicle) == null`
                // warns and returns, the packet's word lands NOTHING
                // (an unknown mount can never carry a belief)
                if !self.entities.contains_key(&p.vehicle) {
                    return false;
                }
                // the eject + startRiding walk: each listed passenger
                // un-rides EVERY other vehicle first (the startRiding's
                // own stopRiding — a passenger holds one seat), then the
                // vehicle's list re-baselines at the packet's FULL
                // list; the empty list (the dismount broadcast) drops
                // the entry entirely (no riders, no belief)
                let changed = self.vehicles.get(&p.vehicle).map(|r| r.as_slice())
                    != Some(p.passengers.as_slice())
                    || p.passengers.iter().any(|r| self.vehicle_of(*r) != Some(p.vehicle));
                for rider in self.vehicles.values_mut() {
                    rider.retain(|r| !p.passengers.contains(r));
                }
                // the emptied seats drop (no riders, no entry — the
                // remount's own stopRiding the eject walk keeps)
                self.vehicles.retain(|_, riders| !riders.is_empty());
                if p.passengers.is_empty() {
                    self.vehicles.remove(&p.vehicle);
                } else {
                    self.vehicles.insert(p.vehicle, p.passengers.clone());
                }
                changed
            }
            PlayEvent::EntityMoved(sync) => {
                // a position correction names an entity the client is
                // tracking; for an unknown id there is nothing to move —
                // refusing (not spawning) keeps the spawn packet the
                // ONLY creator of entities
                let Some(e) = self.entities.get_mut(&sync.id) else {
                    return false;
                };
                e.x = sync.x;
                e.y = sync.y;
                e.z = sync.z;
                e.y_rot = sync.y_rot;
                e.x_rot = sync.x_rot;
                let type_id = e.type_id;
                // (the borrow ends here — the no-op drop(&mut) the
                // NLL loan already speaks)
                // THE ARROW SYNC'S OWN MAILBOX (the client-side sim
                // lane's wire word): the projectile entities the net-
                // arrow lane simulates never read the store back, so
                // the server's correction reaches them through this
                // mailbox — the refresh ships it, the drain empties it,
                // each word rides exactly one ship. The OTHER types stay
                // outside: the mobs and the hulls ride the store's own
                // projection law the refresh serves.
                if self.entity_type_name(type_id) == Some("minecraft:arrow") {
                    self.arrow_syncs.push(ArrowSyncArm {
                        id: sync.id,
                        x: sync.x,
                        y: sync.y,
                        z: sync.z,
                        y_rot: sync.y_rot,
                        x_rot: sync.x_rot,
                    });
                }
                true
            }
            PlayEvent::EntityDeltaPos(m) => {
                // the per-tick walk names an entity the client tracks;
                // an unknown id is refused (the spawn packet stays the
                // ONLY creator). The walk is the vanilla codec's, not a
                // naive += : the ALL-ZERO payload is the codec's own
                // early return (the base comes back UNCHANGED — nothing
                // moved), each non-zero lane moves the QUANTIZED base
                // and each zero lane keeps its axis exact.
                let Some(e) = self.entities.get_mut(&m.id) else {
                    return false;
                };
                if m.dx == 0 && m.dy == 0 && m.dz == 0 {
                    return false;
                }
                e.x = delta_axis(e.x, m.dx);
                e.y = delta_axis(e.y, m.dy);
                e.z = delta_axis(e.z, m.dz);
                true
            }
            PlayEvent::EntityDeltaPosRot(m) => {
                // the same walk plus the wire's angle bytes — the
                // sign-extended byte's `b * 360 / 256` degrees, the Add
                // Entity angles' own quantization; the rotation is
                // asserted even on an all-zero walk
                let Some(e) = self.entities.get_mut(&m.id) else {
                    return false;
                };
                if m.dx != 0 || m.dy != 0 || m.dz != 0 {
                    e.x = delta_axis(e.x, m.dx);
                    e.y = delta_axis(e.y, m.dy);
                    e.z = delta_axis(e.z, m.dz);
                }
                e.y_rot = m.y_rot;
                e.x_rot = m.x_rot;
                true
            }
            PlayEvent::EntityDeltaRot(r) => {
                // rotation-only: the position is untouched, the body
                // turns in place (the wire carries no position lane)
                let Some(e) = self.entities.get_mut(&r.id) else {
                    return false;
                };
                e.y_rot = r.y_rot;
                e.x_rot = r.x_rot;
                true
            }
            PlayEvent::EntityHeadRot(h) => {
                // the head yaw is its OWN channel (Rotate Head rides
                // beside the move packets, never inside them); an
                // unknown id is refused like every other mover
                let Some(e) = self.entities.get_mut(&h.id) else {
                    return false;
                };
                e.y_head_rot = h.y_head_rot;
                true
            }
            PlayEvent::EntityData(d) => {
                // the metadata packets name entities the client tracks;
                // vanilla's handleSetEntityData looks the entity up and
                // a NULL lookup DROPS the packet (javap: `ifnull ->
                // return`) — metadata keeps current entities current
                // and NEVER creates one (the spawn packet stays the
                // only creator). The entries are the assignValues walk:
                // each listed accessor's value REPLACES the stored one,
                // the accessors the packet does not mention are
                // untouched.
                let Some(e) = self.entities.get_mut(&d.id) else {
                    return false;
                };
                for entry in d.entries {
                    e.data.insert(entry.index, entry.value);
                }
                true
            }
            PlayEvent::MobEffectApplied(u) => {
                // vanilla's handleUpdateMobEffect acts on the named
                // entity; the model's effect store is the LOCAL
                // player's (the lightmap's consumer) — other entities'
                // effect traffic is dropped, the same no-surface skip
                // the unknown-id lane takes. The upsert REPLACES the
                // effect id's stored state (the vanilla upsert).
                if self.own_entity_id() == Some(u.entity_id) {
                    self.effects.insert(
                        u.effect_id,
                        MobEffectState { amplifier: u.amplifier, duration: u.duration, flags: u.flags },
                    );
                    true
                } else {
                    false
                }
            }
            PlayEvent::MobEffectRemoved(r) => {
                if self.own_entity_id() == Some(r.entity_id) {
                    self.effects.remove(&r.effect_id).is_some()
                } else {
                    false
                }
            }
            PlayEvent::EntityEquipped(eq) => {
                // THE EQUIPMENT STATE (ClientPacketListener.
                // handleSetEquipment's own walk — the snapshot's slots
                // the store MERGES: each pair replaces its own slot,
                // the OPTIONAL codec's empty stack — count <= 0, item
                // 0 — the slot CLEARS, the absent slots the packet
                // never spoke keep their word, the partial snapshot
                // the vanilla client wears). The entity's own map the
                // entry arms (the ids the Add Entity never spoke the
                // store keeps — the spawn the later word wears).
                let slots = self.entity_equipment.entry(eq.entity_id).or_default();
                for entry in &eq.slots {
                    let ordinal = entry.slot as u8;
                    if entry.stack.count <= 0 || entry.stack.item_id == 0 {
                        slots.remove(&ordinal);
                    } else {
                        slots.insert(ordinal, (entry.stack.item_id, entry.stack.count));
                    }
                }
                true
            }
            PlayEvent::ContainerFilled(c) => {
                // The audited consumer split (javap of the 26.2
                // ClientPacketListener.handleContainerContent): window 0
                // IS the player's own inventory — `player.inventoryMenu
                // .initializeContents(stateId, items, carriedItem)` —
                // live whether or not a container screen is open; any
                // other window goes to the OPEN container's menu
                // (containerMenu, matched by id) or is dropped. The
                // inventory-menu lane REPLACES the player store (the
                // snapshot semantics); the open-container lane REPLACES
                // the matching window's store (the same snapshot
                // semantics), the mismatch DROPS (the fresh javap: the
                // `if_icmpne` falls through to return).
                if c.container_id == INVENTORY_WINDOW_ID {
                    // the snapshot REPLACES slots/carried/stateId and
                    // leaves the SELECTION alone (vanilla's
                    // initializeContents sets those three only — the
                    // javap-pinned field list; the selected hotbar slot
                    // survives the snapshot)
                    let selected = self.inventory.selected;
                    self.inventory = InventoryState {
                        state_id: c.state_id,
                        slots: c.items,
                        carried: c.carried_item,
                        selected,
                    };
                    true
                } else if self
                    .open_container
                    .as_ref()
                    .map_or(false, |o| o.id == c.container_id)
                {
                    // the audited foreign branch: containerId ==
                    // player.containerMenu.containerId → the OPEN menu's
                    // initializeContents — the same snapshot semantics
                    // the window-0 lane rides (the slots, the carried
                    // cursor, the counter; the store has no selection
                    // and nothing else to preserve)
                    let o = self.open_container.as_mut().unwrap();
                    o.state_id = c.state_id;
                    o.slots = c.items;
                    o.carried = c.carried_item;
                    true
                } else {
                    // the mismatched window is DROPPED (the audited
                    // if_icmpne falls through to return) — the packet
                    // rode the wire, the store never hears of it
                    false
                }
            }
            PlayEvent::ContainerSlotSet(s) => {
                // The sibling's audited consumer (handleContainerSetSlot,
                // fresh javap of the tail): window 0 acts on
                // `player.inventoryMenu.setItem(slot, stateId, item)` —
                // the single-slot delta onto the player's own window;
                // the foreign lane compares the packet's container id
                // against `player.containerMenu.containerId` and calls
                // the OPEN menu's setItem on a match (the
                // creative-screen remote-slot rider is the GUI's own
                // act, a documented non-goal), else DROPS. The store
                // policy rides the shared [`store_slot_write`]: the
                // revision counter advances FIRST (whichever packet
                // spoke last is the echo's truth), the slot write
                // applies where an index exists — a slot past the
                // current end GROWS the store with empty stacks (the
                // retention line), a NEGATIVE slot cannot index
                // anything and is ignored. Vanilla's pop-time pickup
                // flash is the GUI's nicety, not store semantics — the
                // documented non-goal.
                if s.container_id == INVENTORY_WINDOW_ID {
                    self.inventory.state_id = s.state_id;
                    store_slot_write(&mut self.inventory.slots, s.slot, &s.item);
                    true
                } else if self
                    .open_container
                    .as_ref()
                    .map_or(false, |o| o.id == s.container_id)
                {
                    // the audited foreign branch: the OPEN menu's
                    // setItem, the same store policy the window-0 lane
                    // rides
                    let o = self.open_container.as_mut().unwrap();
                    o.state_id = s.state_id;
                    store_slot_write(&mut o.slots, s.slot, &s.item);
                    true
                } else {
                    // the mismatched window is DROPPED (the audited
                    // if_icmpne falls through to return) — the packet
                    // rode the wire, the store never hears of it
                    false
                }
            }
            PlayEvent::HeldSlotSet(s) => {
                // The audited gate (javap of
                // ClientPacketListener.handleSetHeldSlot):
                // `Inventory.isHotbarSlot(slot)` — the SELECTION-relative
                // 0..SELECTION_SIZE (9), the inventory's OWN numbering,
                // NOT the InventoryMenu window helper — gates
                // `Inventory.setSelectedSlot(slot)`; anything else (a
                // window slot like 36, a negative) is DROPPED before it
                // reaches the inventory. The mirror: the selection moves
                // only for a hotbar slot, and a repeat naming the slot
                // already held is the no-op (the CenterChunk
                // convention).
                if (0..9).contains(&s.slot)
                    && self.inventory.selected != s.slot as usize
                {
                    self.inventory.selected = s.slot as usize;
                    true
                } else {
                    false
                }
            }
            PlayEvent::ScreenOpened(o) => {
                // The audited consumer (javap of handleOpenScreen →
                // MenuScreens.create): the concrete screen constructs
                // the menu FOR the window id
                // (`menuType.create(containerId, player.getInventory())`)
                // and the screen's show lands it in
                // `player.containerMenu`. The model mirrors the menu's
                // birth: the store OPENS — or RE-OPENS (a fresh menu
                // has FRESH slots; the store resets wholesale — with
                // the id/type/title riding verbatim) — with an empty
                // slot list; the contents ride the container packets'
                // foreign lane.
                self.open_container = Some(OpenContainerState {
                    id: o.container_id,
                    menu_type_id: o.menu_type_id,
                    title_nbt: o.title_nbt,
                    slots: Vec::new(),
                    state_id: 0,
                    carried: ItemStack {
                        count: 0,
                        item_id: 0,
                        patch: ComponentPatch::default(),
                    },
                });
                true
            }
            PlayEvent::ContainerClosed(_) => {
                // The audited consumer (javap of handleContainerClose →
                // LocalPlayer.clientSideCloseContainer →
                // Player.closeContainer): `containerMenu =
                // inventoryMenu` — ONE assignment, NO id check; the
                // packet's id rides the wire as the record of WHICH
                // window closed, not a gate (the parse retains it
                // verbatim). The open container DROPS wholesale; the
                // inventory store (window 0) is untouched. A close with
                // nothing open is the no-op repeat (the CenterChunk
                // convention — the assignment would write the same
                // value).
                self.open_container.take().is_some()
            }
            PlayEvent::CenterChunk { x, z } => {
                if self.center != (x, z) {
                    self.center = (x, z);
                    true
                } else {
                    false
                }
            }
            PlayEvent::Health(h) => {
                // the vitals snapshot REPLACES the store (the vanilla
                // handleSetHealth's own assignment); the first packet
                // arms the HUD, the repeat that writes the same triple
                // stays the no-op repeat (the CenterChunk convention)
                if self.health != Some(h) {
                    self.health = Some(h);
                    true
                } else {
                    false
                }
            }
            PlayEvent::Hurt(a) => {
                // the flash-and-lean acts on the named entity; the
                // model's hurt store is the LOCAL player's (the HUD's
                // flash consumer) — the OTHER entities' hurt traffic
                // arms the mobs' own mailbox (the animateHurt 10-tick
                // window the mob lane's render speaks; the tilt yaw
                // the packet carries is the lean the avatar model
                // doesn't draw yet)
                if self.own_entity_id() == Some(a.entity_id) {
                    // the animateHurt law: the 10-tick window arms,
                    // the tilt yaw rides verbatim
                    self.hurt = Some((10, a.yaw));
                    true
                } else {
                    // THE MOBS' OWN FLASH (handleHurtAnimation's other
                    // arm): the mailbox arms — the refresh ships it
                    // once, the mob's own tick counts the window down
                    // after the landing (the client entity's clock,
                    // exactly the vanilla client's own hurtTime)
                    self.entity_hurts.insert(a.entity_id, 10);
                    true
                }
            }
            PlayEvent::EntitySound(s) => {
                // THE MOBS' OWN VOICES (the SERVER plays the ambient/
                // hurt/death sounds and broadcasts them): the voice
                // resolves NOW, at the arm's birth — the registry id
                // through the synced sound_event table, the direct
                // lane's own location; the id outside the table ships
                // UNNAMED (the honest no-voice the projection drops)
                let name = match &s.sound {
                    mc_protocol::ConsumableSound::Registry(id) => self
                        .registry
                        .sound_events
                        .get(*id as usize)
                        .cloned(),
                    mc_protocol::ConsumableSound::Direct { id, .. } => Some(id.clone()),
                };
                self.entity_sounds.push(EntityVoiceArm {
                    entity_id: s.entity_id,
                    name,
                    volume: s.volume,
                    pitch: s.pitch,
                });
                true
            }
            PlayEvent::WorldSound(s) => {
                // THE WORLD'S OWN VOICES (the SERVER plays a sound at a
                // world spot — the global channel the entity voices'
                // sibling answers): the name resolves NOW, at the arm's
                // birth (the same walk the entity lane keeps — the
                // registry id through the synced sound_event table, the
                // direct lane's own location; the id outside the table
                // ships UNNAMED, the honest no-voice the projection
                // drops), the spot rides verbatim (the parse's /8 law
                // read it), the consumer's ear law folds the gain
                let name = match &s.sound {
                    mc_protocol::ConsumableSound::Registry(id) => self
                        .registry
                        .sound_events
                        .get(*id as usize)
                        .cloned(),
                    mc_protocol::ConsumableSound::Direct { id, .. } => Some(id.clone()),
                };
                self.world_sounds.push(WorldVoiceArm {
                    name,
                    x: s.x,
                    y: s.y,
                    z: s.z,
                    volume: s.volume,
                    pitch: s.pitch,
                });
                true
            }
            PlayEvent::LevelEvent(ev) => {
                // THE LEVEL EVENT'S OWN GATE (javap ClientPacketListener
                // .handleLevelEvent → ClientLevel.levelEvent → the 26.2
                // LevelEventHandler delegate): the vocabulary speaks the
                // GROWTH EFFECT alone (`BoneMealItem.addGrowthParticles`
                // the case at offset 2523 answers) — the anchor + the
                // data word ride into the mailbox, the render lane's
                // own law folds the burst. EVERY OTHER id the unwalked
                // case keeps: the honest `false` (the event touched
                // nothing this model speaks), never a guessed render.
                if ev.effect == mc_protocol::GROWTH_EFFECT {
                    self.growth_events.push(GrowthEffectArm {
                        x: ev.x,
                        y: ev.y,
                        z: ev.z,
                        data: ev.data,
                    });
                    true
                } else if ev.effect == mc_protocol::DESTROY_BLOCK_EFFECT && !ev.global {
                    // THE BLOCK-BREAK EFFECT'S OWN ARM (javap the case
                    // offset 1561 — the positioned route the GLOBAL
                    // switch never banks): the data word the GLOBAL
                    // STATE id carries (`Block.stateById(data)` the
                    // walk's own read — the null the `Blocks.AIR.
                    // defaultBlockState()` fallback answers), the
                    // resolution the registry's own ranges table speaks
                    // (the state id → the block name → the palette kind).
                    // The SOUND the `isAir` gate refuses (the AIR state
                    // — the fallback included — the voice never plays),
                    // the BURST the tile the palette kind banks (the
                    // unbanked kinds the honest absence keeps — the
                    // sound the ride keeps). The arm lands even for the
                    // silent walks — the render lane's own gates the
                    // no-ops wear, the vanilla's own shape the faithful
                    // port keeps.
                    let resolved =
                        mc_registry::generated::block_ids::block_name_of(ev.data.max(0) as u32)
                            .map(|full| full.strip_prefix("minecraft:").unwrap_or(full));
                    let kind = resolved.and_then(|name| {
                        crate::world::NAME_OF
                            .iter()
                            .position(|n| *n == name)
                            .map(|i| i as u16)
                    });
                    let sound_name = resolved.filter(|n| *n != "air");
                    self.break_events.push(BreakEffectArm {
                        x: ev.x,
                        y: ev.y,
                        z: ev.z,
                        sound_name,
                        burst_kind: kind,
                    });
                    true
                } else if ev.effect == mc_protocol::SHOOT_SMOKE_EFFECT && !ev.global {
                    // THE SHOOT-SMOKE EFFECT'S OWN ARM (javap the case
                    // offset 933 — the positioned route the GLOBAL
                    // switch never banks): the anchor + the raw
                    // direction id the mailbox carries (the facing
                    // fold the render lane's own law speaks — the
                    // `shootParticles` walk the `Direction
                    // .from3DDataValue(data)` read the case's own head
                    // opens). No sound the case speaks — the ten-puff
                    // burst the smoke lane's own stream alone carries.
                    self.shoot_smoke_events.push(ShootSmokeArm {
                        x: ev.x,
                        y: ev.y,
                        z: ev.z,
                        data: ev.data,
                    });
                    true
                } else if ev.effect == mc_protocol::SHOOT_WHITE_SMOKE_EFFECT && !ev.global {
                    // THE WHITE-SMOKE EFFECT'S OWN ARM (javap the case
                    // offset 919 — the positioned route the GLOBAL
                    // switch never banks): the SAME `shootParticles`
                    // walk the 2000 word wears (the ten-puff loop the
                    // WHITE_SMOKE type the one arg's delta speaks), the
                    // anchor + the raw direction id the mailbox carries
                    // (the smoke arm's own layering). No sound the case
                    // speaks — the ten-puff burst the white lane's own
                    // stream alone carries.
                    self.white_smoke_events.push(WhiteSmokeArm {
                        x: ev.x,
                        y: ev.y,
                        z: ev.z,
                        data: ev.data,
                    });
                    true
                } else if (ev.effect == mc_protocol::SPLASH_POTION_EFFECT
                    || ev.effect == mc_protocol::INSTANT_SPLASH_EFFECT)
                    && !ev.global
                {
                    // THE SPLASH-POTION EFFECT'S OWN ARM (javap the case
                    // offset 1199 — the positioned route the GLOBAL
                    // switch never banks, BOTH ids the one arm wears):
                    // the anchor + the raw color word the mailbox
                    // carries (the RGB decode the render lane's own law
                    // speaks), the instant flag the type's own split
                    // banks (`2007` the `InstantEffect` word the arm's
                    // own `if_icmpne` branch answers).
                    self.splash_events.push(SplashArm {
                        x: ev.x,
                        y: ev.y,
                        z: ev.z,
                        data: ev.data,
                        instant: ev.effect == mc_protocol::INSTANT_SPLASH_EFFECT,
                    });
                    true
                } else if ev.effect == mc_protocol::MOB_SPAWN_EFFECT && !ev.global {
                    // THE MOB-SPAWN CLOUD'S OWN ARM (javap the case
                    // offset 1689 — the positioned route the GLOBAL
                    // switch never banks): the anchor the mailbox
                    // carries, the data word the case's own body NEVER
                    // reads (no `iload_3` the arm's bytecode speaks —
                    // the zero the spawner's own tail passes the walk
                    // ignores). No sound the case speaks — the twenty
                    // SMOKE + FLAME pairs the render lane's own stream
                    // alone carries.
                    self.mob_spawn_events.push(MobSpawnArm {
                        x: ev.x,
                        y: ev.y,
                        z: ev.z,
                    });
                    true
                } else {
                    false
                }
            }
            PlayEvent::CombatKill(k) => {
                if self.own_entity_id() == Some(k.player_id) {
                    self.death_message = Some(k.message);
                    true
                } else {
                    false
                }
            }
            PlayEvent::Objective(u) => {
                // THE HANDLER'S OWN DISPATCH (javap
                // ClientPacketListener.handleAddObjective): 0 = the add
                // (the display + render type ride the packet), 1 = the
                // remove GATED on the objective existing (the null walk
                // is a no-op), 2 = the display/render-type update GATED
                // the same way, anything else = the ctor's no-op
                // fallthrough. The remove's own bookkeeping walks the
                // slots pointing at the objective and the scores keyed
                // by it (the removeObjective cleanup — the sidebar
                // empties the moment the objective dies).
                match u.method {
                    mc_protocol::OBJECTIVE_METHOD_ADD => {
                        self.objectives.insert(
                            u.objective,
                            ObjectiveState {
                                display: u.display.unwrap_or_default(),
                                render_type: u.render_type,
                            },
                        );
                        true
                    }
                    mc_protocol::OBJECTIVE_METHOD_REMOVE => {
                        if self.objectives.remove(&u.objective).is_none() {
                            return false;
                        }
                        self.display_slots
                            .retain(|_, name| name != &u.objective);
                        self.scores.retain(|(_, obj), _| obj != &u.objective);
                        true
                    }
                    mc_protocol::OBJECTIVE_METHOD_UPDATE => {
                        match self.objectives.get_mut(&u.objective) {
                            Some(o) => {
                                o.display = u.display.unwrap_or_default();
                                o.render_type = u.render_type;
                                true
                            }
                            None => false,
                        }
                    }
                    _ => false,
                }
            }
            PlayEvent::DisplayObjective(u) => {
                // THE HANDLER'S OWN WALK (javap
                // handleSetDisplayObjective): the objective is looked
                // up BY NAME and the slot set UNCONDITIONALLY with the
                // result — the EMPTY name (and any unknown name) is the
                // NULL objective the set clears the slot with.
                let known = self.objectives.contains_key(&u.objective);
                if known {
                    let changed = self.display_slots.get(&u.slot) != Some(&u.objective);
                    self.display_slots.insert(u.slot, u.objective);
                    changed
                } else {
                    self.display_slots.remove(&u.slot).is_some()
                }
            }
            PlayEvent::Score(u) => {
                // THE HANDLER'S OWN GATE (javap handleSetScore): the
                // objective lookup FIRST — an unknown objective is the
                // logged warn + drop, no state change. The known one
                // upserts the row (the getOrCreatePlayerScore + set +
                // display walk).
                if !self.objectives.contains_key(&u.objective) {
                    return false;
                }
                let state = ScoreState { value: u.score, display: u.display };
                let changed =
                    self.scores.get(&(u.owner.clone(), u.objective.clone())) != Some(&state);
                self.scores.insert((u.owner, u.objective), state);
                changed
            }
            PlayEvent::ResetScore(u) => {
                // THE HANDLER'S OWN WALK (javap handleResetScore): a
                // NULL objective resets EVERY score the holder owns
                // (resetAllPlayerScores); a named one is GATED on the
                // objective existing (the same warn + drop the Set
                // Score walk keeps).
                match u.objective {
                    None => {
                        let before = self.scores.len();
                        self.scores.retain(|(owner, _), _| owner != &u.owner);
                        before != self.scores.len()
                    }
                    Some(name) => {
                        if !self.objectives.contains_key(&name) {
                            return false;
                        }
                        self.scores.remove(&(u.owner, name)).is_some()
                    }
                }
            }
            // probes and skipped packets carry no world state
            PlayEvent::KeepAlive(_)
            | PlayEvent::Ping(_)
            | PlayEvent::PongResponse(_)
            | PlayEvent::ChunkBatch => false,
            PlayEvent::DebugSample(_) => false,
            PlayEvent::GameEvent(g) => {
                // THE GAME EVENT'S OWN WALK (javap
                // ClientPacketListener.handleGameEvent + Level, the 26.2
                // bytecode): the weather family answers through the
                // Level setters — START_RAINING the `setRainLevel(1.0F)`
                // arm speaks, STOP_RAINING the `setRainLevel(0.0F)` one,
                // the two LEVEL_CHANGE packets the param arms carry, each
                // `Mth.clamp(param, 0.0, 1.0)` the setter keeps. The
                // GAME-MODE arm the echo speaks — the `floor(param +
                // 0.5)` id fold the handleGameEvent head computes (pcs
                // 37-46), the folded id the model banks (the
                // setLocalMode walk the frame thread's installer wears —
                // the previous bank, the mode bank, the abilities
                // re-grant). The rest of the vocabulary the honest
                // no-opinion skip (the win-screen / border arms name
                // surfaces this client does not own yet — the decode
                // never lies about them, the arms simply keep no state).
                match g.event {
                    mc_protocol::play::game_event::START_RAINING => {
                        self.rain_level = 1.0;
                        true
                    }
                    mc_protocol::play::game_event::STOP_RAINING => {
                        self.rain_level = 0.0;
                        true
                    }
                    mc_protocol::play::game_event::RAIN_LEVEL_CHANGE => {
                        self.rain_level = g.param.clamp(0.0, 1.0);
                        true
                    }
                    mc_protocol::play::game_event::THUNDER_LEVEL_CHANGE => {
                        self.thunder_level = g.param.clamp(0.0, 1.0);
                        true
                    }
                    mc_protocol::play::game_event::CHANGE_GAME_MODEL => {
                        // THE ECHO'S OWN FOLD (pcs 37-46 — the
                        // `fadd 0.5` + `floor` pair the id computes; the
                        // byte the GameType ids 0..3 the byId walk
                        // resolves, the App's installer the byId law
                        // wears)
                        self.local_game_mode = Some((g.param + 0.5).floor() as u8);
                        true
                    }
                    _ => false,
                }
            }
            PlayEvent::PlayerAbilities(w) => {
                // THE ABILITIES ECHO'S OWN BANK (the server's own word
                // the abilities wire rides — the full word the model
                // keeps, the Body-owned fields the frame thread's
                // installer banks)
                self.abilities_word = Some(w);
                true
            }
            PlayEvent::PlayerInfoUpdate(u) => {
                // THE HANDLER'S OWN WALK (javap
                // ClientPacketListener.handlePlayerInfoUpdate): the
                // ADD_PLAYER action creates the entry (the putIfAbsent
                // walk — the vanilla PlayerInfo ctor's own defaults:
                // latency -1, unlisted, the default game mode); every
                // OTHER action updates the entry in place, GATED on it
                // existing (an update for an unknown uuid is the
                // silent no-op the map walk speaks). The per-entry
                // patch's `Some` fields overlay the belief; `None`
                // fields the mask never spoke.
                let mut changed = false;
                for patch in &u.entries {
                    match self.players.get_mut(&patch.uuid) {
                        Some(entry) => {
                            if let Some(name) = &patch.name {
                                if entry.name != *name {
                                    entry.name = name.clone();
                                    changed = true;
                                }
                            }
                            if let Some(gm) = patch.game_mode {
                                if entry.game_mode != gm {
                                    entry.game_mode = gm;
                                    changed = true;
                                }
                            }
                            if let Some(l) = patch.listed {
                                if entry.listed != l {
                                    entry.listed = l;
                                    changed = true;
                                }
                            }
                            if let Some(l) = patch.latency {
                                if entry.latency != l {
                                    entry.latency = l;
                                    changed = true;
                                }
                            }
                            if let Some(o) = patch.list_order {
                                if entry.list_order != o {
                                    entry.list_order = o;
                                    changed = true;
                                }
                            }
                        }
                        // the birth walk: only a patch the ADD action
                        // named creates (an update-only packet for an
                        // unknown uuid — the honest no-op)
                        None => {
                            if let Some(name) = &patch.name {
                                self.players.insert(
                                    patch.uuid,
                                    PlayerEntryState {
                                        name: name.clone(),
                                        game_mode: patch.game_mode.unwrap_or(0),
                                        listed: patch.listed.unwrap_or(false),
                                        latency: patch.latency.unwrap_or(-1),
                                        list_order: patch.list_order.unwrap_or(0),
                                    },
                                );
                                changed = true;
                            }
                        }
                    }
                }
                changed
            }
            PlayEvent::PlayerInfoRemove(u) => {
                // THE HANDLER'S OWN WALK (javap handlePlayerInfoRemove):
                // each uuid's entry dies; an unknown uuid is no-op.
                let before = self.players.len();
                for uuid in &u.uuids {
                    self.players.remove(uuid);
                }
                before != self.players.len()
            }
            PlayEvent::TabList(u) => {
                // THE HANDLER'S OWN WALK (javap
                // handleTabListCustomisation): the header/footer set
                // verbatim (the PlayerTabOverlay.setHeader/setFooter
                // the nulls gate — the house's empty string IS the
                // no-band silence).
                let changed =
                    self.tab_header != u.header || self.tab_footer != u.footer;
                self.tab_header = u.header;
                self.tab_footer = u.footer;
                changed
            }
            // the chat lines' surface: the WORKER intercepts them at the
            // pump (the display queue's dedicated channel ships the lines
            // straight to the frame thread) — the model's world belief
            // never carries them, the honest no-op answers here
            PlayEvent::SystemChat(_)
            | PlayEvent::PlayerChat(_)
            | PlayEvent::DisguisedChat(_) => false,
            // THE SUGGESTION REPLY + THE COMMANDS TREE (the chat
            // screen's own lanes — the world model the neither
            // touches, the UI the gate's own false keeps; the tree
            // rides its dedicated channel like the replies do)
            PlayEvent::CommandSuggestions(_) | PlayEvent::Commands(_) => false,
            PlayEvent::Disconnected { .. } | PlayEvent::Skipped => false,
        }
    }

    /// Adopt a join milestone: the snapshot's columns move into the map
    /// (replace-on-collision, like any re-sent chunk) and the info/player
    /// facts fill in where missing or newer. The snapshot's registry
    /// table REPLACES whatever is stored — the join's registry is the
    /// session's truth (a re-join re-decodes it wholesale).
    pub fn adopt_snapshot(&mut self, snap: WorldSnapshot) {
        let WorldSnapshot { info, player, chunks, registry } = snap;
        self.registry = registry;
        self.pin_dimension(&info.current_dimension);
        self.info = Some(info);
        self.player = Some(player);
        for col in chunks {
            self.columns.insert((col.x, col.z), col);
        }
    }

    /// The decoded vertical layout of one dimension (by registry key).
    /// The SYNCED registry is the server's truth. When the sync carried
    /// NO dimension table at all (a bare server, or one that skipped
    /// Registry Data because the client claimed its known packs), the
    /// BUILT-IN table the generator emitted from the same jar serves —
    /// the dimension-layout slice of the skip-sync fallback. A sync
    /// that DID arrive but omits the key keeps `None`: the server's
    /// own truth is "not a vanilla built-in", never a guess.
    pub fn dimension_min_y(&self, name: &str) -> Option<i32> {
        self.registry
            .dimension(name)
            .map(|d| d.min_y)
            .or_else(|| {
                if self.registry.dimension_types.is_empty() {
                    mc_registry::generated::worldgen::dimension_type(name).map(|d| d.min_y)
                } else {
                    None
                }
            })
    }

    /// How many chunk columns are currently loaded.
    pub fn loaded_chunks(&self) -> usize {
        self.columns.len()
    }

    /// The stored column at a chunk coordinate, if loaded.
    pub fn column(&self, cx: i32, cz: i32) -> Option<&ChunkColumn> {
        self.columns.get(&(cx, cz))
    }

    /// The current dimension's lightmap AmbientColor base — the
    /// `visual/ambient_light_color` attribute as RAW 0..=255 channel
    /// bytes (the vanilla overworld #0a0a0a, nether #302821, end
    /// #3f473f), the shader's `max(AmbientColor, nightVisionColor)`
    /// starting color. THE SYNCED REGISTRY IS THE SERVER'S TRUTH: a
    /// synced dimension entry answers with its own decoded attribute
    /// (v0.3.89's wire decode — an element that states none reads the
    /// bytecode-default black, the vanilla EnvironmentAttribute
    /// fallback). When the sync carried NO dimension table at all (a
    /// bare server, or one that skipped Registry Data because the
    /// client claimed its known packs), the BUILT-IN table the
    /// generator emitted from the same jar serves — the skip-sync
    /// fallback, the same shape [`Self::dimension_min_y`] rides. A sync
    /// that DID arrive but omits the key keeps `None`: the server's own
    /// truth is "not a vanilla built-in", never a guess.
    pub fn ambient_light_color(&self) -> Option<[u8; 3]> {
        let dim = self.current_dimension.as_deref()?;
        if let Some(d) = self.registry.dimension(dim) {
            return d.ambient_light_color;
        }
        if self.registry.dimension_types.is_empty() {
            return mc_registry::generated::worldgen::dimension_type(dim)
                .map(|d| d.ambient_light_color);
        }
        None
    }

    /// The current dimension's skybox — the `DimensionType$Skybox` the
    /// sky pipeline gates the celestials on (the vanilla extract's
    /// dimensionType.skybox() discipline: NONE returns before ANY sky
    /// state is extracted, END extracts only the end-flash — the
    /// disc/sun/moon/stars path is OVERWORLD's alone). THE SYNCED
    /// REGISTRY IS THE SERVER'S TRUTH: a synced dimension entry answers
    /// with its own decoded skybox (the wire decode — an element that
    /// states nothing reads the codec's `optionalFieldOf` default,
    /// Overworld, the vanilla binding; a STATED `none` is its own
    /// verdict). When the sync carried NO dimension table at all (a
    /// bare server, or one that skipped Registry Data because the
    /// client claimed its known packs), the BUILT-IN table the
    /// generator emitted from the same jar serves — the skip-sync
    /// fallback, the same shape [`Self::ambient_light_color`] and
    /// [`Self::dimension_min_y`] ride. No dimension pinned yet, an
    /// entry the sync never named, or an unknown custom key: the
    /// Overworld sky path — the vanilla default, the same fallback the
    /// codec itself binds.
    pub fn skybox(&self) -> mc_registry::generated::worldgen::Skybox {
        use mc_registry::generated::worldgen::Skybox;
        let Some(dim) = self.current_dimension.as_deref() else {
            return Skybox::Overworld;
        };
        if let Some(d) = self.registry.dimension(dim) {
            return d
                .skybox
                .map(|w| match w {
                    mc_protocol::Skybox::None => Skybox::None,
                    mc_protocol::Skybox::Overworld => Skybox::Overworld,
                    mc_protocol::Skybox::End => Skybox::End,
                })
                .unwrap_or(Skybox::Overworld);
        }
        if self.registry.dimension_types.is_empty() {
            if let Some(d) = mc_registry::generated::worldgen::dimension_type(dim) {
                return d.skybox;
            }
        }
        Skybox::Overworld
    }

    /// The current dimension's STATED fog distances (blocks) — the
    /// `(start, end)` pair the element's attributes carry (the nether's
    /// real 10/96, the fogged walls). THE SYNCED REGISTRY IS THE
    /// SERVER'S TRUTH: a synced entry answers with its own stated pair
    /// (the wire decode — the pair-or-nothing rule: a one-sided
    /// statement is not a real vanilla shape and reads as the honest
    /// absence). When the sync carried NO dimension table at all (the
    /// skip-sync case), the BUILT-IN table the generator emitted from
    /// the same jar serves. `None` = no dimension pinned yet, an entry
    /// the sync never named, or no pair stated ANYWHERE — which is the
    /// bytecode defaults' shape (start 0.0 / end 1024.0, javap
    /// EnvironmentAttributes): far beyond this client's 5x5-chunk
    /// window, where the documented window treatment renders instead of
    /// a nearly-invisible vanilla fog.
    pub fn fog_distances(&self) -> Option<(f32, f32)> {
        let dim = self.current_dimension.as_deref()?;
        if let Some(d) = self.registry.dimension(dim) {
            return match (d.fog_start_distance, d.fog_end_distance) {
                (Some(s), Some(e)) => Some((s, e)),
                _ => None,
            };
        }
        if self.registry.dimension_types.is_empty() {
            if let Some(d) = mc_registry::generated::worldgen::dimension_type(dim) {
                return match (d.fog_start_distance, d.fog_end_distance) {
                    (Some(s), Some(e)) => Some((s, e)),
                    _ => None,
                };
            }
        }
        None
    }

    /// The dimension's STATED cloud-fog end (blocks) — `None` when no
    /// dimension states one: the consumer's bytecode-default verdict
    /// (`visual/cloud_fog_end_distance` 2048.0, javap) covers the
    /// absence, and the built-in table is None for all four vanilla
    /// dimensions — only a custom server's statement reads as data.
    pub fn cloud_fog_end_distance(&self) -> Option<f32> {
        let dim = self.current_dimension.as_deref()?;
        if let Some(d) = self.registry.dimension(dim) {
            return d.cloud_fog_end_distance;
        }
        if self.registry.dimension_types.is_empty() {
            if let Some(d) = mc_registry::generated::worldgen::dimension_type(dim) {
                return d.cloud_fog_end_distance;
            }
        }
        None
    }

    /// The dimension's STATED block-light tint — `None` when no
    /// dimension states one: the consumer's bytecode-default verdict
    /// (`visual/block_light_tint` -10100 = #FFD88C, the warm torch
    /// cast, javap) covers the absence, and the built-in table is None
    /// for all four vanilla dimensions — only a custom server's
    /// statement reads as data. The three-branch precedence: the
    /// synced entry wins, the built-in table serves skip-sync, None
    /// is the default's verdict.
    pub fn block_light_tint(&self) -> Option<[u8; 3]> {
        let dim = self.current_dimension.as_deref()?;
        if let Some(d) = self.registry.dimension(dim) {
            return d.block_light_tint;
        }
        if self.registry.dimension_types.is_empty() {
            if let Some(d) = mc_registry::generated::worldgen::dimension_type(dim) {
                return d.block_light_tint;
            }
        }
        None
    }

    /// The dimension's STATED sky-light factor — `None` when no
    /// dimension states one: the consumer's bytecode-default verdict
    /// (`visual/sky_light_factor` 1.0, javap) covers the absence. THE
    /// VANILLA SHAPE (the raw jar JSON, this round's discovery): the
    /// END and the NETHER state **0.0** — their lightmap's sky term is
    /// pinned to ZERO (no sky light reaches those lightmaps); the
    /// overworlds state none. The three-branch precedence: the synced
    /// entry wins, the built-in table serves skip-sync, None is the
    /// default's verdict. The COMPOSED render value is [`sky_factor()`]
    /// — this accessor is the stated attribute, the multiply-BASE the
    /// attribute-system fold applies BEFORE the timeline's track.
    pub fn sky_light_factor(&self) -> Option<f32> {
        let dim = self.current_dimension.as_deref()?;
        if let Some(d) = self.registry.dimension(dim) {
            return d.sky_light_factor;
        }
        if self.registry.dimension_types.is_empty() {
            if let Some(d) = mc_registry::generated::worldgen::dimension_type(dim) {
                return d.sky_light_factor;
            }
        }
        None
    }

    /// The dimension's STATED sky-light color — `None` when no
    /// dimension states one: the consumer's bytecode-default verdict
    /// (`visual/sky_light_color` -1 = 0xFFFFFFFF = #FFFFFF white, the
    /// ARGB multiply identity, javap) covers the absence. THE REAL
    /// JAR'S SHAPE (the generated table's honest rows): the END states
    /// **#ac60cd** and the NETHER **#7a7aff** (the moonlight pairs);
    /// the overworlds state none. The three-branch precedence: the
    /// synced entry wins, the built-in table serves skip-sync, None is
    /// the default's verdict. The COMPOSED render value folds this
    /// stated base UNDER the timeline's multiply track per channel,
    /// INTEGER arithmetic (javap ARGB.multiply: imul / idiv 255, the
    /// white short-circuit — the fold the mesher's sky term applies).
    pub fn sky_light_color(&self) -> Option<[u8; 3]> {
        let dim = self.current_dimension.as_deref()?;
        if let Some(d) = self.registry.dimension(dim) {
            return d.sky_light_color;
        }
        if self.registry.dimension_types.is_empty() {
            if let Some(d) = mc_registry::generated::worldgen::dimension_type(dim) {
                return d.sky_light_color;
            }
        }
        None
    }

    /// The dimension's STATED night-vision color — `None` when no
    /// dimension states one: the consumer's bytecode-default verdict
    /// (`visual/night_vision_color` -6710887 = 0xFF999999 = #999999
    /// gray, javap EnvironmentAttributes) covers the absence. NO
    /// vanilla dimension states it (the end/nether state their sky
    /// pairs, not this) — the built-in table's four rows are all None,
    /// only a custom server's statement reads as data. The three-branch
    /// precedence: the synced entry wins, the built-in table serves
    /// skip-sync, None is the default's verdict. The shader consumes
    /// the value under the night-vision FACTOR — the player's EFFECT
    /// state, not a timeline (javap LightmapRenderStateExtractor:
    /// the NIGHT_VISION effect's nightVisionScale, conduit power +
    /// water vision, else 0.0).
    pub fn night_vision_color(&self) -> Option<[u8; 3]> {
        let dim = self.current_dimension.as_deref()?;
        if let Some(d) = self.registry.dimension(dim) {
            return d.night_vision_color;
        }
        if self.registry.dimension_types.is_empty() {
            if let Some(d) = mc_registry::generated::worldgen::dimension_type(dim) {
                return d.night_vision_color;
            }
        }
        None
    }

    /// Global block state id at (x, y, z) — world coordinates for x/z
    /// (arithmetic shift, so negative chunks index correctly), and world-y
    /// when `min_y` is known else column-local y. `None` outside loaded
    /// columns or the column's vertical extent. The section index walk is
    /// the decoder's own convention: sections bottom-first, blocks indexed
    /// y*256 + z*16 + x inside a section.
    pub fn block(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        let col = self.columns.get(&(x >> 4, z >> 4))?;
        let y = match self.min_y {
            Some(min_y) => y - min_y,
            None => y,
        };
        let (sy, ly) = (y >> 4, y & 15);
        if sy < 0 {
            return None;
        }
        let sec = col.sections.get(sy as usize)?;
        let idx = (ly as usize) * 256 + ((z & 15) as usize) * 16 + (x & 15) as usize;
        sec.blocks.get(idx).copied()
    }

    /// Resolved biome id at (x, y, z) — the same coordinate conventions
    /// as [`WorldModel::block`], on the biome grid: each section carries
    /// a 4×4×4 cell array (64 entries, index = y*16 + z*4 + x on the
    /// QUARTER cells — the decoder's own convention), so a query rounds
    /// the block position down to its 4-block cell. `None` outside the
    /// loaded columns or the column's vertical extent.
    pub fn biome(&self, x: i32, y: i32, z: i32) -> Option<u32> {
        let col = self.columns.get(&(x >> 4, z >> 4))?;
        let y = match self.min_y {
            Some(min_y) => y - min_y,
            None => y,
        };
        let (sy, ly) = (y >> 4, y & 15);
        if sy < 0 {
            return None;
        }
        let sec = col.sections.get(sy as usize)?;
        let idx = (ly as usize >> 2) * 16 + (((z & 15) as usize) >> 2) * 4 + (((x & 15) as usize) >> 2);
        sec.biomes.get(idx).copied()
    }

    /// The biome's registry NAME at (x, y, z) — the chunk palette's
    /// resolved id walked through the configuration-phase biome table
    /// (the arrival-order ids the server synced). When the sync carried
    /// NO biome table at all (the skip-sync case — a server that
    /// accepted the client's known-packs claim), the wire id walks the
    /// BUILT-IN wire-order table the runtime dumper emitted instead —
    /// the vanilla registration order IS the id contract. A sync that
    /// DID arrive but omits the id keeps `None`, and an id beyond the
    /// built-in order too — the honest "unknown biome", never a guess.
    pub fn biome_name(&self, x: i32, y: i32, z: i32) -> Option<&str> {
        let id = self.biome(x, y, z)?;
        self.registry.biome_name(id).or_else(|| {
            if self.registry.biomes.is_empty() {
                mc_registry::generated::registry_order::BIOME_ORDER.get(id as usize).copied()
            } else {
                None
            }
        })
    }

    /// The tint quads the biome layer indexes — one per registry id, the
    /// same order the chunk palettes reference. The SYNCED registry is
    /// the server's truth (each synced element through the shared
    /// resolution chain). When the sync carried NO biome table at all
    /// (the skip-sync case), the BUILT-IN jar tables serve in WIRE
    /// order: `BIOME_ORDER` names every id, [`crate::tint::built_in_tints`]
    /// resolves each name through the same chain the synced path runs —
    /// the biome-tint slice of the skip-sync fallback. An id the built-in
    /// tables do not carry resolves to an all-`None` quad (the no-op
    /// rule renders it as the baked look), never a guess.
    pub fn biome_tints(&self) -> Vec<[Option<u32>; 4]> {
        use mc_registry::generated::registry_order::BIOME_ORDER;
        if self.registry.biomes.is_empty() {
            BIOME_ORDER
                .iter()
                .map(|name| crate::tint::built_in_tints(name).unwrap_or([None; 4]))
                .collect()
        } else {
            self.registry.biomes.iter().map(crate::tint::entry_tints).collect()
        }
    }

    /// The climate pairs the biome layer indexes — one per registry id,
    /// the SAME arrival-order table [`Self::biome_tints`] serves. The
    /// SYNCED registry's own elements speak (the `has_precipitation`
    /// boolean + the `temperature` float the wire's biome NBT carried —
    /// `Biome.getPrecipitationAt`'s own inputs, the weather renderer's
    /// precipitation pick). The skip-sync fallback walks the BUILT-IN
    /// jar tables through the same name chain the tint slice runs
    /// (`BIOME_ORDER` -> `worldgen::biome`) — the `temperature_modifier`
    /// word the generated rows now carry rides the same mapping the
    /// synced arm keeps (the frozen oceans' patchwork the fallback too
    /// speaks); a name the built-in tables
    /// do not carry resolves to [`BiomeClimate::NONE`] — the honest
    /// no-opinion word, never a guess (no rain from an unknown climate).
    pub fn biome_climates(&self) -> Vec<crate::world::BiomeClimate> {
        use mc_registry::generated::registry_order::BIOME_ORDER;
        if self.registry.biomes.is_empty() {
            BIOME_ORDER
                .iter()
                .map(|name| {
                    mc_registry::generated::worldgen::biome(name)
                        .map(|d| crate::world::BiomeClimate {
                            has_precipitation: d.has_precipitation,
                            temperature: d.temperature,
                            // the generated table's own word (the jar's
                            // optional codec field, "frozen" the one
                            // non-default string) drives the enum — the
                            // same mapping the synced arm below keeps;
                            // the table predates nothing any more, the
                            // frozen oceans' patchwork the fallback too
                            // speaks
                            temperature_modifier: match d.temperature_modifier {
                                Some("frozen") => crate::world::TemperatureModifier::Frozen,
                                _ => crate::world::TemperatureModifier::None,
                            },
                        })
                        .unwrap_or(crate::world::BiomeClimate::NONE)
                })
                .collect()
        } else {
            self.registry
                .biomes
                .iter()
                .map(|e| crate::world::BiomeClimate {
                    has_precipitation: e.has_precipitation,
                    temperature: e.temperature,
                    // the synced string drives the enum — `frozen` the
                    // only non-identity word; anything else the jar's
                    // own registry codec would have refused at load
                    // (the StringRepresentable fromEnum law), so the
                    // honest word maps it to None, never a guess
                    temperature_modifier: match e.temperature_modifier.as_deref() {
                        Some("frozen") => crate::world::TemperatureModifier::Frozen,
                        _ => crate::world::TemperatureModifier::None,
                    },
                })
                .collect()
        }
    }

    /// The current dimension's day tick — the synced clock's total
    /// ticks mod the day timeline's period — or `None` before the
    /// server's first Set Time (the honest state: no opinion, the
    /// renderer stays at day). The clock is resolved through the
    /// configuration knowledge: the current dimension's
    /// `default_clock` NAME walks the synced `minecraft:world_clock`
    /// table to an id, and the clock state with THAT id wins (the
    /// end's clock never masquerades as the overworld's, whatever
    /// order the server sent them in). A resolved name whose clock
    /// state never arrived is `None` — the honest no-opinion, never
    /// another clock's ticks. Without any registry knowledge (a bare
    /// mock server) the first clock in the packet serves — documented
    /// fallback, the arrival-order guess the table replaces.
    ///
    /// THE SKIP-SYNC FALLBACK (the world-clock slice, v0.3.56): a
    /// server that accepted the known-packs claim streams NO registry
    /// data at all — that state (no dimension sync) is the gate for
    /// BOTH rescues: the clock NAME through the built-in dimension
    /// table, and the clock ID through the wire-order table whose
    /// member set the built-in WORLD_CLOCKS twin pins. A sync that DID
    /// arrive keeps every existing contract — its element names the
    /// clock (or doesn't, the documented first-clock shape), its
    /// world_clock table maps the id, and a key it omits gets no
    /// built-in rescue.
    pub fn day_tick(&self) -> Option<i64> {
        let skip_sync = self.registry.dimension_types.is_empty();
        let named = self.current_dimension.as_ref().and_then(|dim| {
            let clock_name = match self.registry.dimension(dim) {
                Some(d) => d.default_clock.clone(),
                None if skip_sync => {
                    mc_registry::generated::worldgen::dimension_type(dim)
                        .and_then(|d| d.default_clock.map(String::from))
                }
                None => None,
            };
            let clock_name = clock_name.as_deref()?;
            let id = self.registry.world_clock_id(clock_name).or_else(|| {
                if skip_sync {
                    mc_registry::generated::registry_order::WORLD_CLOCK_ORDER
                        .iter()
                        .position(|n| *n == clock_name)
                        .map(|p| p as i32)
                } else {
                    None
                }
            })?;
            Some(self.clocks.iter().find(|c| c.id == id).map(|c| c.total_ticks))
        });
        match named {
            // the named clock's state arrived — its ticks are the truth
            Some(Some(ticks)) => Some(ticks.rem_euclid(crate::timeline::DAY_PERIOD_TICKS)),
            // a default_clock + table resolved the id but no state for
            // it ever arrived: the honest no-opinion (never another
            // clock's ticks)
            Some(None) => None,
            // no registry knowledge at all (a bare mock server): the
            // first clock in the packet serves — documented fallback,
            // the arrival-order guess the table replaces
            None => self
                .clocks
                .first()
                .map(|c| c.total_ticks.rem_euclid(crate::timeline::DAY_PERIOD_TICKS)),
        }
    }

    /// THE RAIN LEVEL'S OWN ANSWER (javap `Level.getRainLevel` — the
    /// `Mth.lerp(partial, oRainLevel, rainLevel)` walk with BOTH slots
    /// holding the same clamped value the setter stores; the Game
    /// Event stream the state keeps). `0.0` before any weather word —
    /// the dry world the golden scenes render.
    pub fn rain_level(&self) -> f32 {
        self.rain_level
    }

    /// THE ECHO'S OWN WORD (the model's banked GameType id — the
    /// CHANGE_GAME_MODEL fold the game-event arm keeps): `None` until
    /// the server's echo speaks, `Some(id)` the refresh ships to the
    /// frame thread's installer (the setLocalMode walk the App bank
    /// wears — the idempotent re-bank the shipped persistence keeps
    /// silent).
    pub fn local_game_mode(&self) -> Option<u8> {
        self.local_game_mode
    }

    /// THE ABILITIES ECHO'S OWN WORD (the model's banked
    /// ClientboundPlayerAbilitiesPacket — the server's own word the
    /// abilities wire rides): `None` until the echo lands, `Some(word)`
    /// the refresh ships to the frame thread's installer (the
    /// handlePlayerAbilities walk the App bank wears — the idempotent
    /// re-bank the unchanged word's same-value putfields keep).
    pub fn abilities_word(&self) -> Option<&mc_protocol::play::PlayerAbilitiesWord> {
        self.abilities_word.as_ref()
    }

    /// THE EFFECTIVE THUNDER'S OWN ANSWER (javap `Level.getThunderLevel`
    /// — the lerp walk MULTIPLIED BY the rain level the same method's
    /// tail keeps: `(lerp(oThunder, thunder) * getRainLevel(partial))`).
    /// The effective value the rain gates — the dry world's thunder
    /// word is 0.0 whatever the state holds.
    pub fn thunder_level(&self) -> f32 {
        self.thunder_level * self.rain_level
    }

    /// The renderer's sky factor for the current clock state — the day
    /// timeline's `visual/sky_light_factor` at [`WorldModel::day_tick`]
    /// (jar-extracted keyframes: 1.0 day / 0.24 night). `1.0` before
    /// any Set Time: the world with no clock opinion is day, and the
    /// factor multiplies the sky brightness term exactly as the
    /// vanilla lightmap shader's `SkyFactor` does.
    ///
    /// The curve is DATA-DRIVEN when the configuration knowledge can
    /// name one: the synced TIMELINE registry's entry whose track the
    /// dimension carries, evaluated through the same math as the
    /// compile-time snapshot ([`crate::timeline::evaluate`]). A server
    /// with no timeline knowledge falls back to the snapshot — the
    /// jar's day.json keyframes, bit-identical values.
    pub fn sky_factor(&self) -> f32 {
        // the attribute-system fold (javap EnvironmentAttributeSystem:
        // the sampler folds the layers in add order — the dimension's
        // constant layer OVERRIDES the registered default, the
        // timeline's TimeBased layer applies LAST with the track's
        // modifier; the day track's modifier is multiply): the stated
        // factor is the multiply-BASE, the timeline track scales it.
        // No clock -> no TimeBased layer -> the stated (override of
        // the default 1.0) alone.
        let stated = self.sky_light_factor().unwrap_or(1.0);
        match self.day_tick() {
            None => stated,
            Some(tick) => match self.timeline_curve() {
                Some((keyframes, period)) => {
                    stated * crate::timeline::evaluate(keyframes, Some(period), tick)
                }
                None => stated * crate::timeline::sky_light_factor(tick),
            },
        }
    }

    /// The dimension's `visual/sky_light_factor` curve — `Some((keyframes,
    /// period))` when the configuration knowledge names one. TWO sources,
    /// in strict precedence order: the SYNCED registry first (a sync that
    /// arrived is the unchallenged truth — `synced_timeline_curve`), then,
    /// ONLY in the skip-sync state (no registry data arrived at all — the
    /// claim-accepted server, the same gate the v0.3.52/55/56 fallbacks
    /// run under), the BUILT-IN jar tables (`built_in_timeline_curve`).
    /// When neither names a curve the compile-time snapshot serves —
    /// for the real 26.2 jar every path CONVERGES on the day curve (the
    /// only `visual/sky_light_factor` carrier in the jar is
    /// `minecraft:day`, byte-pinned to the snapshot since the timeline
    /// arc), and a future jar that drifts from the snapshot fails the
    /// pin instead of the pixels.
    fn timeline_curve(&self) -> Option<(&[(i64, f32)], i64)> {
        if let Some(curve) = self.synced_timeline_curve() {
            return Some(curve);
        }
        if self.registry.dimension_types.is_empty() {
            return self.built_in_timeline_curve();
        }
        None
    }

    /// The dimension's curve from the SYNCED TIMELINE registry. The chain
    /// (each narrowing, the first hit wins): the current dimension's
    /// `timelines` reference names the candidates — the TAG form
    /// `#minecraft:in_overworld` resolves through the Update Tags
    /// payload's member ids, a bare id names one timeline directly —
    /// and with NO reference the fallback candidates are every
    /// timeline whose `clock` matches the dimension's `default_clock`.
    /// An UNKNOWN tag (its Update Tags payload never arrived) resolves
    /// NOTHING — the snapshot serves instead, never a blind clock
    /// filter that could resurrect an off-tag carrier (the exact thing
    /// the tag exists to prevent). Among the candidates the first
    /// timeline CARRYING the track serves: vanilla merges multiple
    /// timelines per attribute track, but in real data exactly one
    /// overworld timeline (`minecraft:day`) carries
    /// `visual/sky_light_factor` — the first-carrier rule is the honest
    /// minimal contract, the multi-timeline merge (the track's
    /// `modifier`) is future work.
    fn synced_timeline_curve(&self) -> Option<(&[(i64, f32)], i64)> {
        let dim = self.current_dimension.as_ref()?;
        let entry = self.registry.dimension(dim)?;
        let clock_name = entry.default_clock.as_deref()?;
        let mut candidates: Vec<&mc_protocol::TimelineEntry> = Vec::new();
        match entry.timelines.as_deref() {
            Some(t) if t.starts_with('#') => {
                let ids = self.registry.timeline_tag_ids(&t[1..])?;
                candidates.extend(ids.iter().filter_map(|&id| self.registry.timelines.get(id as usize)));
            }
            Some(id) => candidates.extend(self.registry.timelines.iter().filter(|e| e.name == id)),
            None => candidates.extend(
                self.registry.timelines.iter().filter(|e| e.clock == clock_name),
            ),
        }
        let tl = candidates.into_iter().find(|e| e.sky_light_factor.is_some())?;
        Some((tl.sky_light_factor.as_deref()?, tl.period_ticks? as i64))
    }

    /// The SKIP-SYNC timeline fallback: the SAME resolution chain the
    /// synced path runs, walked against the BUILT-IN jar tables (the
    /// generated dimension_type / timeline twins + the REGISTRY's tag
    /// resolver) instead of the streamed registry. The tag branch reads
    /// [`mc_registry::flatten_timeline_tag`] — the vanilla built-tag
    /// verdict the registry has owned since the tag arc (file-order
    /// walk, depth-first `#` expansion in place, first-occurrence dedup,
    /// fail-closed on unknown tag / unknown member / cycle) — retiring
    /// the hand-rolled walk that duplicated it client-side: ONE
    /// expansion truth, the same consolidation the tint chain banks.
    /// On the real 26.2 jar the two agree member-for-member (the
    /// registry sweep proves every timeline-tag member resolves; the
    /// client test pins the order), so the fail-closed verdict changes
    /// nothing observable — and a drifted jar now refuses WHOLE, the
    /// vanilla TagLoader shape, instead of partially resolving. A sync
    /// that DID arrive never reaches here (the dispatcher's gate — the
    /// partial-sync model gets no built-in rescue, mirroring the
    /// v0.3.56 day-tick scoping).
    fn built_in_timeline_curve(&self) -> Option<(&'static [(i64, f32)], i64)> {
        let dim = self.current_dimension.as_ref()?;
        let entry = mc_registry::generated::worldgen::dimension_type(dim)?;
        let mut candidates: Vec<&'static mc_registry::generated::worldgen::TimelineDef> =
            Vec::new();
        match entry.timelines {
            Some(t) if t.starts_with('#') => {
                if let Some(members) = mc_registry::flatten_timeline_tag(&t[1..]) {
                    // the flatten already validated every member against
                    // the timeline table (the registry-bound load rule);
                    // filter_map keeps the client total on a hypothetical
                    // registry drift — the sweep test pins the invariant
                    candidates.extend(
                        members
                            .iter()
                            .filter_map(|m| mc_registry::generated::worldgen::timeline(m)),
                    );
                }
            }
            Some(id) => {
                if let Some(tl) = mc_registry::generated::worldgen::timeline(id) {
                    candidates.push(tl);
                }
            }
            None => {
                if let Some(clock) = entry.default_clock {
                    candidates.extend(
                        mc_registry::generated::worldgen::TIMELINES
                            .iter()
                            .filter(|e| e.clock == clock),
                    );
                }
            }
        }
        let tl = candidates
            .into_iter()
            .find(|e| !e.sky_light_factor.is_empty())?;
        Some((tl.sky_light_factor, tl.period_ticks?))
    }

    /// Sky light (0..=15) at (x, y, z) — the same coordinate conventions
    /// as [`WorldModel::block`], on the vanilla LIGHT section grid: the
    /// engine keeps one extra layer below and above the block sections
    /// (javap: `getLightSectionCount() = getSectionsCount() + 2`), so
    /// the column's section sy answers through light bit sy + 1 — and
    /// the layers just OUTSIDE the column (world y one section below
    /// its bottom / above its top) answer through bits 0 and len+1.
    /// `None` outside the loaded columns, beyond the engine's light
    /// range, or where the server sent nothing — the honest unknown,
    /// never a guess.
    pub fn sky_light(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        self.light_at(x, y, z, |l| &l.sky)
    }

    /// Block light (0..=15) at (x, y, z) — [`WorldModel::sky_light`]
    /// reading the block-light layer instead of the sky layer.
    pub fn block_light(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        self.light_at(x, y, z, |l| &l.block)
    }

    /// The shared light walk: column lookup, the min_y shift, the
    /// engine-layer bit (sy + 1 — unlike `block()` a section index of
    /// -1 is LEGITIMATE here, it is the under-world layer), then the
    /// section's server truth: known-empty answers 0, a data array
    /// answers its YZX nibble (even index in the low nibble, the
    /// decoder's javap-pinned DataLayer convention).
    fn light_at(
        &self,
        x: i32,
        y: i32,
        z: i32,
        layer: impl Fn(&ChunkLight) -> &Vec<Option<LightData>>,
    ) -> Option<u8> {
        let col = self.columns.get(&(x >> 4, z >> 4))?;
        let y = match self.min_y {
            Some(min_y) => y - min_y,
            None => y,
        };
        let (sy, ly) = (y >> 4, y & 15);
        let bit = sy + 1; // light bit 0 is one section BELOW the column
        if bit < 0 {
            return None;
        }
        match layer(&col.light).get(bit as usize)? {
            Some(LightData::Empty) => Some(0),
            Some(LightData::Data(arr)) => {
                let idx = (ly as usize) * 256 + ((z & 15) as usize) * 16 + (x & 15) as usize;
                let b = *arr.get(idx >> 1)?;
                Some(if idx & 1 == 0 { b & 0xF } else { b >> 4 })
            }
            None => None, // the server said nothing about this layer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mc_protocol::DimensionTypeEntry;
    use mc_protocol::SpawnInfo;
    use mc_protocol::varint::write_varint;
    use mc_protocol::{
        BiomeEffects, BiomeEntry, EntityDeltaPos, EntityDeltaPosRot, EntityDeltaRot,
        EntityHeadRot, EntityPositionSync, EntitySpawn, GrassColorModifier, SetEntityDataEntry,
        SetPassengers,
    };

    /// One synced biome element under the mock tables — the real
    /// NETWORK_CODEC shape with plains' climate (the name is what the
    /// assertions resolve; the tint fields are the plains defaults).
    fn biome_entry(name: &str) -> BiomeEntry {
        BiomeEntry {
            name: name.into(),
            has_precipitation: true,
            temperature: 0.8,
            temperature_modifier: None,
            downfall: 0.4,
            effects: BiomeEffects {
                water_color: 4_159_204,
                grass_color: None,
                foliage_color: None,
                dry_foliage_color: None,
                grass_color_modifier: GrassColorModifier::None,
            },
        }
    }

    /// One palettized section buffer (the mc-protocol mock shape): stone
    /// at (0,0,0) and dirt at (3,1,2) of the section, via a 5-bit indirect
    /// palette whose data array carries PALETTE INDICES. `count` controls
    /// the declared non-air block count so multi-section columns stay legal.
    fn stone_dirt_section(count: i16) -> Vec<u8> {
        let palette = [0u32, 1, 10]; // air, stone, dirt
        let idx = |x: usize, y: usize, z: usize| (y << 8) | (z << 4) | x;
        let mut entries = vec![0u32; 4096];
        entries[idx(0, 0, 0)] = 1;
        entries[idx(3, 1, 2)] = 2;
        let mut s = Vec::new();
        s.extend_from_slice(&count.to_be_bytes());
        s.push(5); // bits per entry -> indirect
        write_varint(&mut s, palette.len() as i32);
        for id in palette {
            write_varint(&mut s, id as i32);
        }
        s.extend_from_slice(&mc_protocol::pack_entries(&entries, 5));
        s.push(0); // biomes: single valued
        write_varint(&mut s, 1);
        s
    }

    /// One all-air section (block_count 0, single-valued states+biomes).
    fn air_section() -> Vec<u8> {
        let mut s = Vec::new();
        s.extend_from_slice(&0i16.to_be_bytes());
        s.push(0);
        write_varint(&mut s, 0);
        s.push(0);
        write_varint(&mut s, 187);
        s
    }

    /// Decode a section buffer through the REAL parser (packaged into a
    /// one-section chunk packet) so tests run against the actual wire shape.
    fn parse_section_buffer(buffer: &[u8]) -> mc_protocol::ChunkSection {
        let mut p = Vec::new();
        write_varint(&mut p, mc_protocol::play::clientbound::CHUNK_DATA);
        p.extend_from_slice(&0i32.to_be_bytes());
        p.extend_from_slice(&0i32.to_be_bytes());
        p.push(0x0A);
        p.push(0x00); // empty heightmaps compound
        write_varint(&mut p, buffer.len() as i32);
        p.extend_from_slice(buffer);
        write_varint(&mut p, 0); // no block entities
        p.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // light masks
        let col = mc_protocol::play::parse_chunk_data(&p).expect("chunk parses");
        col.sections.into_iter().next().expect("one section")
    }

    #[test]
    fn columns_replace_unload_and_center_track() {
        let mut m = WorldModel::new();
        assert_eq!(m.loaded_chunks(), 0);
        // a chunk arrives
        let col_a = ChunkColumn {
            x: 3,
            z: -2,
            sections: vec![parse_section_buffer(&stone_dirt_section(2))],
            light: ChunkLight::default(),
        };
        assert!(m.apply(PlayEvent::Chunk(col_a)));
        assert_eq!(m.loaded_chunks(), 1);
        // a re-sent column for the same key REPLACES (newest wins) —
        // the old content is gone, not merged, and the map stays size 1
        let col_b = ChunkColumn {
            x: 3,
            z: -2,
            sections: vec![parse_section_buffer(&air_section())],
            light: ChunkLight::default(),
        };
        assert!(m.apply(PlayEvent::Chunk(col_b)));
        assert_eq!(m.loaded_chunks(), 1);
        assert_eq!(m.block(48, 0, -32), Some(0), "replaced by air");
        // probes never touch world state
        assert!(!m.apply(PlayEvent::KeepAlive(-5)));
        assert!(!m.apply(PlayEvent::Ping(7)));
        assert!(!m.apply(PlayEvent::ChunkBatch));
        assert!(!m.apply(PlayEvent::Skipped));
        assert_eq!(m.loaded_chunks(), 1);
        // the center chunk moves
        assert!(m.apply(PlayEvent::CenterChunk { x: 3, z: -2 }));
        assert_eq!(m.center, (3, -2));
        assert!(!m.apply(PlayEvent::CenterChunk { x: 3, z: -2 }), "no-op repeat");
        // an unload drops the column; unloading something absent is a no-op
        assert!(m.apply(PlayEvent::UnloadChunk { x: 3, z: -2 }));
        assert_eq!(m.loaded_chunks(), 0);
        assert!(!m.apply(PlayEvent::UnloadChunk { x: 3, z: -2 }));
        // a disconnect is world-state neutral
        assert!(!m.apply(PlayEvent::Disconnected { reason: "bye".into() }));
    }

    /// THE DIFFICULTY'S OWN WIRE BANK (javap 26.2
    /// `handleChangeDifficulty` — the levelData pair the packet's
    /// arrival stores): the latest packet replaces, the absent lane
    /// the honest None, the RESPAWN the bank KEEPS (the handleRespawn
    /// walk the new ClientLevelData the OLD one's getDifficulty()
    /// ctor arg re-seats — the difficulty the level swap carries).
    #[test]
    fn the_wire_difficulty_bank_wears_the_level_data_store() {
        let mut m = WorldModel::new();
        assert!(
            m.wire_difficulty().is_none(),
            "the absent packet the honest None keeps"
        );
        // the first packet lands (the join's own sync — the vanilla
        // server the packet at play-state entry sends)
        assert!(m.apply(PlayEvent::ChangeDifficulty(mc_protocol::play::ChangeDifficultyWord {
            difficulty: 0,
            locked: false,
        })));
        assert_eq!(
            m.wire_difficulty(),
            Some((0, false)),
            "the Peaceful pair the setDifficulty + setDifficultyLocked walk stores"
        );
        // the latest packet replaces (the server's /difficulty turn —
        // the same packet the change wears, the newest word the bank
        // keeps)
        assert!(m.apply(PlayEvent::ChangeDifficulty(
            mc_protocol::play::ChangeDifficultyWord { difficulty: 3, locked: true }
        )));
        assert_eq!(
            m.wire_difficulty(),
            Some((3, true)),
            "the Hard + locked pair the latest packet banks"
        );
        // THE RESPAWN THE BANK KEEPS (the handleRespawn walk the new
        // ClientLevelData the OLD one's getDifficulty() ctor arg
        // re-seats — the level swap the difficulty carries across)
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 1,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 60,
        })));
        assert_eq!(
            m.wire_difficulty(),
            Some((3, true)),
            "the respawn the bank keeps — the new levelData the old dial wears"
        );
    }

    #[test]
    fn block_queries_resolve_the_vanilla_index_conventions() {
        let mut m = WorldModel::new();
        let col = ChunkColumn {
            x: 3,
            z: -2,
            sections: vec![
                parse_section_buffer(&stone_dirt_section(2)),
                parse_section_buffer(&air_section()),
            ],
            light: ChunkLight::default(),
        };
        m.apply(PlayEvent::Chunk(col));
        // chunk (3, -2) spans world x 48..=63, z -32..=17 — the column-local
        // stone at (0,0,0) lands at world (48, 0, -32) and the dirt at
        // local (3,1,2) -> world (51, 1, -30)
        assert_eq!(m.block(48, 0, -32), Some(1), "stone, column-local y");
        assert_eq!(m.block(51, 1, -30), Some(10), "dirt, y*256+z*16+x walk");
        assert_eq!(m.block(49, 0, -32), Some(0), "air next to it");
        // section stride: section 1 is all air
        assert_eq!(m.block(48, 16, -32), Some(0), "second section bottom");
        assert_eq!(m.block(48, 31, -32), Some(0), "second section top");
        // out of every bound the model knows
        assert_eq!(m.block(0, 0, 0), None, "unloaded column");
        assert_eq!(m.block(48, 32, -32), None, "above the column");
        assert_eq!(m.block(48, -1, -32), None, "below without min_y");
        // with a known dimension min_y, world-y queries shift by it
        m.min_y = Some(-64);
        assert_eq!(m.block(48, -64, -32), Some(1), "world y -64 -> section 0 bottom");
        assert_eq!(m.block(48, -49, -32), Some(0), "world y -49 -> section 0 top slice (air)");
        assert_eq!(m.block(48, -48, -32), Some(0), "world y -48 -> section 1");
        assert_eq!(m.block(48, -65, -32), None, "below the column");
        // negative chunk coordinates: chunk (-1, -1) local (15, y, 15) is
        // world (-1, y, -1) — arithmetic shift + mask, no wraparound
        let neg = ChunkColumn {
            x: -1,
            z: -1,
            sections: vec![parse_section_buffer(&stone_dirt_section(2))],
            light: ChunkLight::default(),
        };
        m.apply(PlayEvent::Chunk(neg));
        assert_eq!(m.block(-1, -64, -1), Some(0), "world (-1,-64,-1) is local (15,0,15): air");
        assert_eq!(m.block(-16, -64, -16), Some(1), "world (-16,-64,-16) is local (0,0,0): stone");
        assert_eq!(m.block(-13, -63, -14), Some(10), "local (3,1,2) of chunk (-1,-1)");
    }

    #[test]
    fn the_block_update_mutates_the_stored_column() {
        // Block Update is the world's own traffic: one block changed,
        // the stored column mutated IN PLACE through the SAME arithmetic
        // block() reads through — the next materialize renders the
        // change, nothing else exists to invalidate. Strict all-or-nothing
        // (the Light event's rule): absent column or out-of-range slot
        // refuses the update, never a half-applied lie.
        let mut m = WorldModel::new();
        let col = ChunkColumn {
            x: 3,
            z: -2,
            sections: vec![
                parse_section_buffer(&stone_dirt_section(2)),
                parse_section_buffer(&air_section()),
            ],
            light: ChunkLight::default(),
        };
        m.apply(PlayEvent::Chunk(col));
        // the column-local convention (min_y unknown): world (48, 0, -32)
        assert_eq!(m.block(48, 0, -32), Some(1), "stone before the update");
        assert!(m.apply(PlayEvent::Block { x: 48, y: 0, z: -32, state: 5 }));
        assert_eq!(m.block(48, 0, -32), Some(5), "the update is the model's truth");
        // a second section lands through the same arithmetic
        assert!(m.apply(PlayEvent::Block { x: 51, y: 16, z: -30, state: 7 }));
        assert_eq!(m.block(51, 16, -30), Some(7), "section 1 top, in-place");
        // an update ABOVE the column is refused whole
        assert!(!m.apply(PlayEvent::Block { x: 48, y: 32, z: -32, state: 5 }));
        assert_eq!(m.block(48, 0, -32), Some(5), "the refused update changed nothing");
        // an update for an UNLOADED column is refused (vanilla's own
        // ignore: the update crossed the Respawn drop or raced the chunk)
        assert!(!m.apply(PlayEvent::Block { x: 0, y: 0, z: 0, state: 5 }));
    }

    #[test]
    fn the_section_blocks_mutate_the_one_section_strictly() {
        // Section Blocks Update: a BATCH inside one section. The wire's
        // sy is the WORLD section y — the column index is the floor walk
        // `(sy * 16 - min_y) >> 4`, and the entries are section-relative.
        // The all-or-nothing rule covers the BATCH: one bad entry means
        // NOTHING lands, even entries that arrived before it.
        let mut m = WorldModel::new();
        m.min_y = Some(-64); // the overworld's floor: section -4 is the column's bottom
        let col = ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![
                parse_section_buffer(&stone_dirt_section(2)),
                parse_section_buffer(&air_section()),
            ],
            light: ChunkLight::default(),
        };
        m.apply(PlayEvent::Chunk(col));
        // world section -4 -> index 0; entries land at their relative
        // slots (block() reads world-y -64 + ly)
        let sb = mc_protocol::play::SectionBlocks {
            sx: 0,
            sy: -4,
            sz: 0,
            blocks: vec![(0, 0, 0, 7), (3, 1, 2, 9), (15, 15, 15, 11)],
        };
        assert!(m.apply(PlayEvent::SectionBlocks(sb)));
        assert_eq!(m.block(0, -64, 0), Some(7));
        assert_eq!(m.block(3, -63, 2), Some(9), "relative (3,1,2) is world-y -63");
        assert_eq!(m.block(15, -49, 15), Some(11), "the section's top corner");
        // the OTHER sections are untouched
        assert_eq!(m.block(0, -48, 0), Some(0), "section -3 (index 1) keeps its air");
        // a batch with ONE out-of-range entry lands NOTHING — the strict
        // batch rule, the Light event's shape
        let bad = mc_protocol::play::SectionBlocks {
            sx: 0,
            sy: -4,
            sz: 0,
            blocks: vec![(0, 0, 0, 99), (0, 16, 0, 98)],
        };
        assert!(!m.apply(PlayEvent::SectionBlocks(bad)));
        assert_eq!(
            m.block(0, -64, 0),
            Some(7),
            "the refused batch changed nothing, not even the valid prefix"
        );
        // a batch for an unloaded column is refused
        let away = mc_protocol::play::SectionBlocks {
            sx: 9,
            sy: -4,
            sz: 9,
            blocks: vec![(0, 0, 0, 7)],
        };
        assert!(!m.apply(PlayEvent::SectionBlocks(away)));
    }

    /// One all-air section whose biomes ride a 2-bit indirect palette
    /// (the vanilla 4×4×4 cell array — 64 entries of PALETTE INDICES
    /// into the given global-bio-me-id palette).
    fn biome_cells_section(palette: &[u32], cells: &[u32; 64]) -> Vec<u8> {
        let mut s = Vec::new();
        s.extend_from_slice(&0i16.to_be_bytes()); // block count 0
        s.push(0); // blocks: single valued
        write_varint(&mut s, 0); // air
        s.push(2); // biomes: bits 2 -> indirect
        write_varint(&mut s, palette.len() as i32);
        for id in palette {
            write_varint(&mut s, *id as i32);
        }
        s.extend_from_slice(&mc_protocol::pack_entries(cells, 2));
        s
    }

    #[test]
    fn biome_queries_walk_the_4x4x4_cell_grid() {
        let mut m = WorldModel::new();
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![DimensionTypeEntry {
                name: "minecraft:overworld".into(),
                min_y: -64,
                height: 384,
                default_clock: Some("minecraft:overworld".into()),
                timelines: None,
            ambient_light_color: None,
            skybox: None,
            fog_start_distance: None,
            fog_end_distance: None,
            cloud_fog_end_distance: None,
            block_light_tint: None,
            sky_light_color: None,
            sky_light_factor: None,
            night_vision_color: None,
            }],
            // the synced biome table: ids 0/1/2, arrival order
            biomes: vec![
                biome_entry("minecraft:plains"),
                biome_entry("minecraft:swamp"),
                biome_entry("minecraft:desert"),
            ],
            biome_tags: Vec::new(),
            world_clocks: Vec::new(),
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        // the login pins the dimension: min_y goes live (world y -64 is
        // section 0's bottom, like every real overworld join)
        assert!(m.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 17,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        })));
        // cell index = (y>>2)*16 + (z>>2)*4 + (x>>2); four cells carry
        // distinct biomes, everything else stays plains (0)
        let mut cells = [0u32; 64];
        cells[1] = 2; // x-cell stride: local x 4..7
        cells[4] = 1; // z-cell stride: local z 4..7
        cells[16] = 1; // y-cell stride: local y 4..7 (the SECOND 4-block row)
        cells[63] = 2; // the far corner
        let col = ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![parse_section_buffer(&biome_cells_section(&[0, 1, 2], &cells))],
            light: ChunkLight::default(),
        };
        m.apply(PlayEvent::Chunk(col));
        // cell 0 covers block positions 0..4 in each axis — every query
        // inside it rounds to the SAME entry (min_y -64: world -64..-49
        // is section 0)
        assert_eq!(m.biome(0, -64, 0), Some(0));
        assert_eq!(m.biome_name(0, -64, 0), Some("minecraft:plains"));
        assert_eq!(m.biome(3, -61, 3), Some(0), "the whole 4-block cell");
        // the x stride: local x 4 lands in the NEXT cell
        assert_eq!(m.biome(4, -64, 0), Some(2));
        assert_eq!(m.biome_name(4, -64, 0), Some("minecraft:desert"));
        // the z stride and the y stride
        assert_eq!(m.biome(0, -64, 4), Some(1));
        assert_eq!(m.biome(0, -60, 0), Some(1));
        assert_eq!(m.biome_name(0, -60, 0), Some("minecraft:swamp"));
        // the far corner of the section
        assert_eq!(m.biome(15, -49, 15), Some(2));
        // negative chunks: chunk (-1,-1) local (15,y,15) is world (-1,y,-1)
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: -1,
            z: -1,
            sections: vec![parse_section_buffer(&biome_cells_section(&[0, 1, 2], &cells))],
            light: ChunkLight::default(),
        }));
        assert_eq!(m.biome(-1, -64, -1), Some(0), "local (15,0,15) -> cell (3,0,3), still plains");
        assert_eq!(m.biome_name(-16, -64, -16), Some("minecraft:plains"), "local (0,0,0)");
        // out of bounds mirrors block(): absent columns and sections
        assert_eq!(m.biome(64, -64, 0), None, "unloaded column");
        assert_eq!(m.biome(0, -48, 0), None, "above the one-section column");
        assert_eq!(m.biome(0, -65, 0), None, "below the column");
        // an id the server never named resolves to NO name (honest
        // unknown): the mock air sections carry single-valued 187
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 7,
            z: 7,
            sections: vec![parse_section_buffer(&air_section())],
            light: ChunkLight::default(),
        }));
        assert_eq!(m.biome(112, -64, 112), Some(187), "the id is real");
        assert_eq!(m.biome_name(112, -64, 112), None, "the name is unknown");
    }

    #[test]
    fn the_built_in_biome_table_is_the_skip_sync_fallback() {
        // NO registry data at all (the claim-accepted skip-sync case):
        // the wire-order table names the ids, the built-in jar tables
        // resolve the tint quads — the same verdicts the synced path
        // would produce for the same vanilla biome
        let wire_dark = mc_registry::generated::registry_order::BIOME_ORDER
            .iter()
            .position(|n| *n == "minecraft:dark_forest")
            .expect("the vanilla order names dark_forest") as u32;
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![parse_section_buffer(&biome_cells_section(
                &[wire_dark],
                &[0; 64],
            ))],
            light: ChunkLight::default(),
        }));
        // the name walks the built-in wire order (a bare model has no
        // min_y — the query is column-local, section 0 is y 0..16)
        assert_eq!(m.biome_name(0, 0, 0), Some("minecraft:dark_forest"));
        // an id beyond the built-in order stays unknown — honest
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 1,
            z: 0,
            sections: vec![parse_section_buffer(&air_section())],
            light: ChunkLight::default(),
        }));
        assert_eq!(m.biome_name(16, 0, 0), None, "187 names nothing in the vanilla order");
        // the quads: 66 built-in resolutions in wire order, and the
        // dark_forest quad EQUALS the synced path's verdict for the same
        // jar surface (climate 0.7/0.8, no color overrides, the
        // dark_forest modifier blend, the dry override #7B5334)
        let tints = m.biome_tints();
        assert_eq!(tints.len(), 66, "one quad per vanilla wire id");
        let mut synced_dark = biome_entry("minecraft:dark_forest");
        synced_dark.temperature = 0.7;
        synced_dark.downfall = 0.8;
        synced_dark.effects.grass_color_modifier = GrassColorModifier::DarkForest;
        synced_dark.effects.dry_foliage_color = Some(0x7B5334);
        assert_eq!(tints[wire_dark as usize], crate::tint::entry_tints(&synced_dark));
        assert_eq!(
            tints[wire_dark as usize][0],
            Some(0x507A32),
            "the canonical dark forest grass through the fallback"
        );
        // the SYNCED table is the unchallenged truth: a sync of one LIE
        // entry keeps the accessor at len 1 (no built-in top-up) and the
        // name walk gets no rescue either
        let mut synced = WorldModel::new();
        synced.registry.biomes = vec![biome_entry("mock:lie")];
        assert_eq!(synced.biome_tints().len(), 1, "the sync wins, even when it lies");
        assert_eq!(
            synced.biome_name(0, 0, 0),
            None,
            "an id the sync never named gets no built-in rescue"
        );
    }

    #[test]
    fn the_built_in_clock_tables_are_the_skip_sync_fallback() {
        // NO registry data at all (the claim-accepted skip-sync state):
        // the built-in dimension table names the overworld's clock, the
        // wire-order table maps that name to the id the vanilla Set Time
        // packet carries
        let mut m = WorldModel::new();
        m.current_dimension = Some("minecraft:overworld".into());
        let p = mc_protocol::play::test_support::set_time_packet(1, &[(0, 1_002_006_000, 0.0, 1.0)]);
        let ts = mc_protocol::play::parse_set_time(&p).expect("set time parses");
        assert!(m.apply(PlayEvent::Time(ts)));
        assert_eq!(
            m.day_tick(),
            Some(6_000),
            "built-in default_clock -> wire-order id 0 -> the overworld clock's ticks"
        );
        // the end: the built-in dimension names ITS clock — the
        // wire-order id 1, whose own state serves (never the
        // overworld's ticks, whatever the arrival order)
        m.current_dimension = Some("minecraft:the_end".into());
        assert_eq!(m.day_tick(), None, "the end's clock state never arrived: no opinion");
        let p2 = mc_protocol::play::test_support::set_time_packet(2, &[(1, 999_999, 0.0, 1.0)]);
        let ts2 = mc_protocol::play::parse_set_time(&p2).unwrap();
        assert!(m.apply(PlayEvent::Time(ts2)));
        assert_eq!(m.day_tick(), Some(15_999), "the end's own clock through wire-order id 1");
        // the nether: the built-in dimension names NO clock — the
        // documented first-clock fallback shape (the only state, the
        // end's, serves — the arrival-order guess)
        m.current_dimension = Some("minecraft:the_nether".into());
        assert_eq!(m.day_tick(), Some(15_999), "no clock named: the arrival-order fallback");
        // a mock dimension name: unknown in both tables — the same
        // documented fallback (the bare-mock contract, unchanged)
        m.current_dimension = Some("mock:test".into());
        assert_eq!(m.day_tick(), Some(15_999));
        // a sync that DID arrive gets NO built-in rescue: a partial
        // dimension sync without the overworld key keeps the
        // first-clock fallback, the pre-fallback contract
        let mut partial = WorldModel::new();
        partial.current_dimension = Some("minecraft:overworld".into());
        partial.registry.dimension_types = vec![DimensionTypeEntry {
            name: "minecraft:the_nether".into(),
            min_y: 0,
            height: 256,
            default_clock: None,
            timelines: None,
        ambient_light_color: None,
        skybox: None,
        fog_start_distance: None,
        fog_end_distance: None,
        cloud_fog_end_distance: None,
        block_light_tint: None,
        sky_light_color: None,
        sky_light_factor: None,
        night_vision_color: None,
        }];
        let p3 =
            mc_protocol::play::test_support::set_time_packet(3, &[(0, 1_002_006_000, 0.0, 1.0)]);
        let ts3 = mc_protocol::play::parse_set_time(&p3).unwrap();
        assert!(partial.apply(PlayEvent::Time(ts3)));
        assert_eq!(
            partial.day_tick(),
            Some(6_000),
            "the arrived sync is the only truth — no key, no rescue, first clock serves"
        );
    }

    #[test]
    fn light_queries_walk_the_nibble_grid_across_the_engine_layers() {
        let mut m = WorldModel::new();
        m.min_y = Some(-64);
        // a 2-section column parsed from a REAL light-carrying packet
        // (mc-protocol's own mock builder — the wire shape, not a
        // hand-assembled struct): light bits 0..=3 = below / own
        // section 0 / own section 1 / above (the engine's -1/+1 layers)
        let mut sec0 = vec![0u8; 2048];
        sec0[0] = 0x5A; // idx 0 (x0,y0,z0) low = 0xA; idx 1 (x1) high = 0x5
        sec0[1] = 0x34; // idx 2 (x2) low = 0x4
        sec0[16] = 0x0F; // idx 32 (z2) low = 0xF
        sec0[128] = 0x21; // idx 256 (y1) low = 0x1
        let mut below = vec![0u8; 2048];
        below[0] = 0x0F; // idx 0 low = 0xF under the world's floor
        let above = vec![0xFFu8; 2048]; // full-bright layer above the top
        let mut block_arr = vec![0u8; 2048];
        block_arr[0] = 0x23; // idx 0 low = 3; idx 1 high = 2
        let p = mc_protocol::play::test_support::chunk_packet_with_light(
            3,
            -2,
            &[stone_dirt_section(2), air_section()],
            &[(0, below), (1, sec0), (3, above)],
            &[2], // the column's own section 1 is KNOWN-EMPTY sky
            &[(1, block_arr)],
            &[],
        );
        let col = mc_protocol::play::parse_chunk_data(&p).expect("chunk parses");
        m.apply(PlayEvent::Chunk(col));
        // chunk (3, -2) spans world x 48..=63, z -32..=-17; min_y -64
        // puts section 0 at world y -64..=-49
        assert_eq!(
            m.sky_light(48, -64, -32),
            Some(10),
            "local (0,0,0), the LOW nibble of 0x5A"
        );
        assert_eq!(
            m.sky_light(49, -64, -32),
            Some(5),
            "local (1,0,0), the HIGH nibble of 0x5A"
        );
        assert_eq!(m.sky_light(50, -64, -32), Some(4), "idx 2, low of 0x34");
        assert_eq!(m.sky_light(48, -63, -32), Some(1), "idx 256 (y stride), low of 0x21");
        assert_eq!(m.sky_light(48, -64, -30), Some(15), "idx 32 (z stride), low of 0x0F");
        // the engine's extra layers: one section BELOW the column
        // (bit 0) and one ABOVE the top (bit 3, world y -32)
        assert_eq!(m.sky_light(48, -80, -32), Some(15), "world -80 is the under-world layer");
        assert_eq!(m.sky_light(48, -32, -32), Some(15), "world -32 is the above-top layer");
        // a known-empty section (the empty mask) answers ZERO — the
        // server said so, it is not an unknown
        assert_eq!(m.sky_light(48, -48, -32), Some(0), "section 1 is Empty, not unknown");
        // beyond the engine's light range — the honest unknown
        assert_eq!(m.sky_light(48, -96, -32), None, "two sections below: no such bit");
        assert_eq!(m.sky_light(48, -16, -32), None, "one section above the above-layer");
        // the block-light layer rides the same walk
        assert_eq!(m.block_light(48, -64, -32), Some(3), "low nibble of 0x23");
        assert_eq!(m.block_light(49, -64, -32), Some(2), "high nibble");
        assert_eq!(m.block_light(48, -48, -32), None, "no block data for section 1");
        // an unloaded column is unknown; a hand-built lightless column
        // (ChunkLight::default) too — no light tail, no invented values
        assert_eq!(m.sky_light(0, -64, 0), None);
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 7,
            z: 7,
            sections: vec![parse_section_buffer(&air_section())],
            light: ChunkLight::default(),
        }));
        assert_eq!(m.sky_light(112, -64, 112), None, "no light tail in the column");
    }

    #[test]
    fn light_update_replaces_the_columns_light_on_the_live_wire() {
        let mut m = WorldModel::new();
        m.min_y = Some(-64);
        // the join column: sky bit 1 data (0x5A in byte 0), bit 2 empty
        let p = mc_protocol::play::test_support::chunk_packet_with_light(
            3,
            -2,
            &[stone_dirt_section(2)],
            &[(1, vec![0x5Au8; 2048])],
            &[2],
            &[],
            &[],
        );
        m.apply(PlayEvent::Chunk(
            mc_protocol::play::parse_chunk_data(&p).expect("chunk parses"),
        ));
        assert_eq!(m.sky_light(48, -64, -32), Some(10));
        assert_eq!(m.sky_light(48, -48, -32), Some(0), "bit 2 is Empty");
        assert_eq!(m.sky_light(48, -80, -32), None, "bit 0 unmentioned");
        // an update re-sends the column's COMPLETE light state: bit 1
        // full-bright, bit 0 marked Empty, bit 2 UNMENTIONED (drops to
        // the honest None — the vanilla masks are the engine's full
        // current storage, not a delta)
        let mut arr = vec![0u8; 2048];
        arr[0] = 0xFF;
        let up = mc_protocol::play::test_support::update_light_packet(
            3,
            -2,
            &[(1, arr)],
            &[0],
            &[],
            &[],
        );
        let (ux, uz, ulight) = mc_protocol::play::parse_update_light(&up).unwrap();
        assert_eq!((ux, uz), (3, -2));
        assert!(m.apply(PlayEvent::Light { x: ux, z: uz, light: ulight }));
        assert_eq!(m.sky_light(48, -64, -32), Some(15), "the update's array wins");
        assert_eq!(m.sky_light(48, -80, -32), Some(0), "bit 0 is Empty now");
        assert_eq!(m.sky_light(48, -48, -32), None, "unmentioned -> the honest None");
        // an update for a column that never landed is refused
        let up2 = mc_protocol::play::test_support::update_light_packet(9, 9, &[], &[], &[], &[]);
        let (_, _, l2) = mc_protocol::play::parse_update_light(&up2).unwrap();
        assert!(!m.apply(PlayEvent::Light { x: 9, z: 9, light: l2 }));
        // a documented bit outside the column's light range is drift:
        // the WHOLE update is refused, the old light survives intact
        let mut arr3 = vec![0u8; 2048];
        arr3[0] = 0xEE;
        let up3 = mc_protocol::play::test_support::update_light_packet(
            3,
            -2,
            &[(7, arr3)], // a 1-section column spans light bits 0..=2
            &[],
            &[],
            &[],
        );
        let (_, _, l3) = mc_protocol::play::parse_update_light(&up3).unwrap();
        assert!(
            !m.apply(PlayEvent::Light { x: 3, z: -2, light: l3 }),
            "out-of-range bit refused"
        );
        assert_eq!(m.sky_light(48, -64, -32), Some(15), "the old light intact");
    }

    #[test]
    fn set_time_feeds_the_day_tick_and_sky_factor() {
        let mut m = WorldModel::new();
        // no clock opinion yet: day, honestly
        assert_eq!(m.day_tick(), None);
        assert_eq!(m.sky_factor(), 1.0);
        // the server's Set Time: game time + the dimension's default
        // clock at midnight (total ticks 18_000 -> day tick 18_000)
        let p = mc_protocol::play::test_support::set_time_packet(
            7_777,
            &[(0, 1_002_018_000, 0.0, 1.0)],
        );
        let ts = mc_protocol::play::parse_set_time(&p).expect("set time parses");
        assert!(m.apply(PlayEvent::Time(ts)), "time touches world state");
        assert_eq!(m.game_time, 7_777);
        assert_eq!(m.day_tick(), Some(18_000), "total ticks mod the day period");
        assert_eq!(m.sky_factor(), 0.24, "midnight is the night plateau");
        // a re-sync REPLACES the clocks (the current state, not a delta)
        let p2 = mc_protocol::play::test_support::set_time_packet(7_778, &[(0, 6_000, 0.0, 1.0)]);
        let ts2 = mc_protocol::play::parse_set_time(&p2).unwrap();
        assert!(m.apply(PlayEvent::Time(ts2)));
        assert_eq!(m.day_tick(), Some(6_000), "noon after the re-sync");
        assert_eq!(m.sky_factor(), 1.0, "noon is full day");
    }

    #[test]
    fn the_game_event_words_the_weather_levels_speak() {
        // THE DRY FLOOR: the world with no weather opinion is dry — the
        // golden scenes' honest default
        let mut m = WorldModel::new();
        assert_eq!(m.rain_level(), 0.0);
        assert_eq!(m.thunder_level(), 0.0);
        // THE START/STOP PAIR (javap handleGameEvent: the two arms the
        // `setRainLevel(1.0F)` / `setRainLevel(0.0F)` calls keep — the
        // param those events carry the client never reads)
        assert!(m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: mc_protocol::play::game_event::START_RAINING,
            param: 0.25,
        })), "start-rain touches state");
        assert_eq!(m.rain_level(), 1.0, "the 1.0F arm speaks");
        assert!(m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: mc_protocol::play::game_event::STOP_RAINING,
            param: 0.9,
        })));
        assert_eq!(m.rain_level(), 0.0, "the 0.0F arm speaks");
        // THE LEVEL-CHANGE PAIR (the param arms — each
        // `Mth.clamp(param, 0.0, 1.0)` the setter keeps; the out-of-range
        // params the clamp folds)
        assert!(m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: mc_protocol::play::game_event::RAIN_LEVEL_CHANGE,
            param: 0.6875,
        })));
        assert_eq!(m.rain_level(), 0.6875);
        assert!(m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: mc_protocol::play::game_event::RAIN_LEVEL_CHANGE,
            param: 1.7,
        })));
        assert_eq!(m.rain_level(), 1.0, "the clamp's ceiling");
        assert!(m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: mc_protocol::play::game_event::RAIN_LEVEL_CHANGE,
            param: -0.3,
        })));
        assert_eq!(m.rain_level(), 0.0, "the clamp's floor");
        // THE THUNDER LEVEL (the setter's own clamp; the EFFECTIVE value
        // the getThunderLevel walk multiplies by the rain level — the
        // dry world's thunder is 0.0 whatever the state stores)
        assert!(m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: mc_protocol::play::game_event::THUNDER_LEVEL_CHANGE,
            param: 0.5,
        })));
        assert_eq!(m.thunder_level(), 0.0, "the rain multiply gates it");
        assert!(m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: mc_protocol::play::game_event::RAIN_LEVEL_CHANGE,
            param: 1.0,
        })));
        assert_eq!(m.thunder_level(), 0.5, "the rain arms the thunder");
        // THE HONEST SKIPS: the vocabulary's other words keep no state —
        // the win-screen arms this client does not own
        assert!(!m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: mc_protocol::play::game_event::WIN_GAME,
            param: 0.0,
        })), "the win arm is no-opinion");
        // THE GAME-MODE ECHO (the CHANGE_GAME_MODEL arm the mode
        // re-sync speaks — the `floor(param + 0.5)` fold the id
        // computes, the model's word the refresh ships)
        assert!(m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: mc_protocol::play::game_event::CHANGE_GAME_MODEL,
            param: 1.0,
        })), "the echo the mode re-sync speaks");
        assert_eq!(
            m.local_game_mode(),
            Some(1),
            "floor(1.0 + 0.5) = 1 — the CREATIVE id banks"
        );
        // the FOLD'S OWN ROUNDING (param 2.4 → floor(2.9) = 2 — the
        // ADVENTURE id the next echo banks)
        assert!(m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: mc_protocol::play::game_event::CHANGE_GAME_MODEL,
            param: 2.4,
        })));
        assert_eq!(m.local_game_mode(), Some(2), "the fold's rounding speaks");
        // an UNDEFINED event id the server could still send: the same
        // honest skip (the decode admits the word, the state keeps none)
        assert!(!m.apply(PlayEvent::GameEvent(mc_protocol::play::GameEvent {
            event: 99,
            param: 1.0,
        })));
        assert_eq!(m.rain_level(), 1.0, "the skips never touch the rain");
    }

    #[test]
    fn the_abilities_echo_the_server_word_banks() {
        // THE BANK (the PlayerAbilities arm — the server's own word
        // the abilities wire rides, the full word the model keeps
        // verbatim: the four flag bits and the two speeds the read
        // ctor folded)
        let mut m = WorldModel::new();
        assert!(
            m.abilities_word().is_none(),
            "the fresh model the word never carries"
        );
        let word = mc_protocol::play::PlayerAbilitiesWord {
            invulnerable: true,
            flying: true,
            mayfly: true,
            instabuild: false,
            flying_speed: 0.07,
            walking_speed: 0.12,
        };
        assert!(
            m.apply(PlayEvent::PlayerAbilities(word.clone())),
            "the echo the view's re-ship earns"
        );
        let banked = m.abilities_word().expect("the word the model banks");
        assert_eq!(banked, &word, "the full word the bank keeps verbatim");
        // THE REPLACE WALK (the newest echo wins — the handler's own
        // unconditional re-bank the later packet speaks)
        let newer = mc_protocol::play::PlayerAbilitiesWord {
            invulnerable: false,
            flying: false,
            mayfly: false,
            instabuild: false,
            flying_speed: 0.05,
            walking_speed: 0.1,
        };
        assert!(m.apply(PlayEvent::PlayerAbilities(newer.clone())));
        assert_eq!(
            m.abilities_word(),
            Some(&newer),
            "the newest word the bank replaces"
        );
    }

    #[test]
    fn day_tick_resolves_the_dimension_default_clock_by_name() {
        let mut m = WorldModel::new();
        m.current_dimension = Some("minecraft:overworld".into());
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![
                DimensionTypeEntry {
                    name: "minecraft:overworld".into(),
                    min_y: -64,
                    height: 384,
                    default_clock: Some("minecraft:overworld".into()),
                    timelines: None,
                ambient_light_color: None,
                skybox: None,
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: None,
                block_light_tint: None,
                sky_light_color: None,
                sky_light_factor: None,
                night_vision_color: None,
                },
                DimensionTypeEntry {
                    name: "minecraft:the_nether".into(),
                    min_y: 0,
                    height: 256,
                    default_clock: None, // no day cycle in the nether
                    timelines: None,
                ambient_light_color: None,
                skybox: None,
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: None,
                block_light_tint: None,
                sky_light_color: None,
                sky_light_factor: None,
                night_vision_color: None,
                },
            ],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            // the synced id table: overworld = 0, the_end = 1
            world_clocks: vec!["minecraft:overworld".to_string(), "minecraft:the_end".to_string()],
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        // BOTH clocks sync in ONE packet, the END's FIRST: the naive
        // first-clock rule would take the end's ticks; the named
        // resolution takes the overworld's (1_002_006_000 mod 24000 =
        // 6000, noon)
        let p = mc_protocol::play::test_support::set_time_packet(
            1,
            &[(1, 999_999, 0.0, 1.0), (0, 1_002_006_000, 0.0, 1.0)],
        );
        let ts = mc_protocol::play::parse_set_time(&p).expect("set time parses");
        assert!(m.apply(PlayEvent::Time(ts)));
        assert_eq!(
            m.day_tick(),
            Some(6_000),
            "the dimension's named clock wins over arrival order"
        );
        assert_eq!(m.sky_factor(), 1.0, "noon");
        // the nether: no default_clock on its element — the documented
        // bare-mock fallback (first clock = the end's 999_999 mod 24000)
        m.current_dimension = Some("minecraft:the_nether".into());
        assert_eq!(m.day_tick(), Some(15_999), "no day cycle named: the fallback serves");
        // back in the overworld with only THE END's state synced: the
        // named clock is known but absent — the honest no-opinion, never
        // another clock's ticks
        m.current_dimension = Some("minecraft:overworld".into());
        let p2 = mc_protocol::play::test_support::set_time_packet(2, &[(1, 999_999, 0.0, 1.0)]);
        let ts2 = mc_protocol::play::parse_set_time(&p2).unwrap();
        assert!(m.apply(PlayEvent::Time(ts2)));
        assert_eq!(m.day_tick(), None, "the overworld clock never arrived: no opinion");
        assert_eq!(m.sky_factor(), 1.0, "no opinion renders as day");
    }

    #[test]
    fn sky_factor_reads_the_synced_timeline_registry_through_the_dimension_tag() {
        let mut m = WorldModel::new();
        m.current_dimension = Some("minecraft:overworld".into());
        // the registry knowledge a real vanilla server streams: the
        // overworld dimension with its default_clock AND its
        // `timelines` tag reference; FOUR timelines in arrival order
        // with an OFF-TAG curve-carrier FIRST (a clock-name filter
        // would wrongly take it); the tag names ids 1..=3 only
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![DimensionTypeEntry {
                name: "minecraft:overworld".into(),
                min_y: -64,
                height: 384,
                default_clock: Some("minecraft:overworld".into()),
                timelines: Some("#minecraft:in_overworld".into()),
                ambient_light_color: None,
                skybox: None,
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: None,
                block_light_tint: None,
                sky_light_color: None,
                sky_light_factor: None,
                night_vision_color: None,
            }],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: vec!["minecraft:overworld".to_string()],
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: vec![
                // id 0: carries a curve but is NOT in the dimension's
                // tag — vanilla merges only the tag's members, so this
                // half-day plateau must never serve
                mc_protocol::TimelineEntry {
                    name: "minecraft:test_alt".into(),
                    clock: "minecraft:overworld".into(),
                    period_ticks: Some(24_000),
                    sky_light_factor: Some(vec![(0, 1.0), (12_000, 0.5)]),
                },
                // id 1: THE curve-carrier (the real day.json keyframes)
                mc_protocol::TimelineEntry {
                    name: "minecraft:day".into(),
                    clock: "minecraft:overworld".into(),
                    period_ticks: Some(24_000),
                    sky_light_factor: Some(vec![
                        (730, 1.0),
                        (11_270, 1.0),
                        (13_140, 0.24),
                        (22_860, 0.24),
                    ]),
                },
                // id 2: the moon — overworld clock, NO visual track
                mc_protocol::TimelineEntry {
                    name: "minecraft:moon".into(),
                    clock: "minecraft:overworld".into(),
                    period_ticks: Some(192_000),
                    sky_light_factor: None,
                },
                // id 3: early_game — no period at all
                mc_protocol::TimelineEntry {
                    name: "minecraft:early_game".into(),
                    clock: "minecraft:overworld".into(),
                    period_ticks: None,
                    sky_light_factor: None,
                },
            ],
            timeline_tags: vec![("minecraft:in_overworld".to_string(), vec![1, 2, 3])],
        };
        // the overworld clock at midnight: the TAG path picks
        // minecraft:day -> the night plateau 0.24, NOT test_alt's 0.5
        let p = mc_protocol::play::test_support::set_time_packet(1, &[(0, 18_000, 0.0, 1.0)]);
        let ts = mc_protocol::play::parse_set_time(&p).expect("set time parses");
        assert!(m.apply(PlayEvent::Time(ts)));
        assert_eq!(
            m.sky_factor(),
            0.24,
            "the tag narrowed to minecraft:day — the off-tag carrier never serves"
        );
        // noon through the registry curve
        let p2 = mc_protocol::play::test_support::set_time_packet(2, &[(0, 6_000, 0.0, 1.0)]);
        let ts2 = mc_protocol::play::parse_set_time(&p2).unwrap();
        assert!(m.apply(PlayEvent::Time(ts2)));
        assert_eq!(m.sky_factor(), 1.0, "noon is full day through the registry curve");
        // back to midnight for the fallback contracts
        let p3 = mc_protocol::play::test_support::set_time_packet(3, &[(0, 18_000, 0.0, 1.0)]);
        let ts3 = mc_protocol::play::parse_set_time(&p3).unwrap();
        assert!(m.apply(PlayEvent::Time(ts3)));
        // the tag UNKNOWN (its Update Tags payload never arrived): the
        // narrowing cannot happen, so the compile-time day.json
        // snapshot serves — the jar's own overworld curve, NOT a
        // blind clock filter that could resurrect an off-tag carrier
        // (the exact thing the tag exists to prevent)
        m.registry.timeline_tags.clear();
        assert_eq!(
            m.sky_factor(),
            0.24,
            "unknown tag: the snapshot curve serves, the off-tag carrier never does"
        );
        // NO registry knowledge at all (a bare mock server): the same
        // snapshot — the same values bit-for-bit
        m.registry.timelines.clear();
        assert_eq!(m.sky_factor(), 0.24, "midnight through the snapshot's curve");
        // and with neither table the snapshot is the day.json constants
        let mut bare = WorldModel::new();
        bare.current_dimension = Some("minecraft:overworld".into());
        let p4 = mc_protocol::play::test_support::set_time_packet(4, &[(0, 18_000, 0.0, 1.0)]);
        let ts4 = mc_protocol::play::parse_set_time(&p4).unwrap();
        assert!(bare.apply(PlayEvent::Time(ts4)));
        assert_eq!(bare.sky_factor(), 0.24, "the snapshot IS the jar's day.json curve");
    }

    #[test]
    fn the_built_in_timeline_tables_are_the_skip_sync_curve() {
        // a claim-accepted vanilla server streams NO registry — the
        // timeline slice of the skip-sync fallback (the last registry
        // surface without one): the curve resolves through the BUILT-IN
        // jar tables (dimension_type -> the `#minecraft:in_overworld`
        // tag -> nested `#universal` expansion -> the first carrier
        // `minecraft:day`), and for the real 26.2 jar the answer is the
        // SAME values the compile-time snapshot serves — the snapshot
        // has been byte-pinned to the generated TL_DAY_SKY_LIGHT_FACTOR
        // since the timeline arc, so the two sources cannot disagree
        let mut m = WorldModel::new();
        m.current_dimension = Some("minecraft:overworld".into());
        let p = mc_protocol::play::test_support::set_time_packet(1, &[(0, 18_000, 0.0, 1.0)]);
        let ts = mc_protocol::play::parse_set_time(&p).expect("set time parses");
        assert!(m.apply(PlayEvent::Time(ts)));
        assert_eq!(m.sky_factor(), 0.24, "midnight through the built-in tag chain");
        let p2 = mc_protocol::play::test_support::set_time_packet(2, &[(0, 6_000, 0.0, 1.0)]);
        let ts2 = mc_protocol::play::parse_set_time(&p2).unwrap();
        assert!(m.apply(PlayEvent::Time(ts2)));
        assert_eq!(m.sky_factor(), 1.0, "noon through the built-in tag chain");
        // overworld_caves names the SAME tag — the same curve
        let mut caves = WorldModel::new();
        caves.current_dimension = Some("minecraft:overworld_caves".into());
        let p3 = mc_protocol::play::test_support::set_time_packet(3, &[(0, 18_000, 0.0, 1.0)]);
        let ts3 = mc_protocol::play::parse_set_time(&p3).unwrap();
        assert!(caves.apply(PlayEvent::Time(ts3)));
        assert_eq!(caves.sky_factor(), 0.24);
        // THE END: `in_end` expands to `#universal` -> villager_schedule
        // only — NO timeline carries the track, so the resolution comes
        // up empty and the snapshot serves (at the end's own ticks —
        // the v0.3.56 day-tick fallback's wire-order id 1). AND the
        // fold's stated base multiplies UNDER the track: the end's
        // built-in `visual/sky_light_factor` is **0.0** (this round's
        // jar discovery — the raw dimension JSON), so the composed
        // factor is ZERO whatever the track says: vanilla pins the
        // end's sky term to zero.
        let mut end = WorldModel::new();
        end.current_dimension = Some("minecraft:the_end".into());
        let p4 = mc_protocol::play::test_support::set_time_packet(4, &[(1, 15_999, 0.0, 1.0)]);
        let ts4 = mc_protocol::play::parse_set_time(&p4).unwrap();
        assert!(end.apply(PlayEvent::Time(ts4)));
        assert_eq!(
            end.sky_light_factor(),
            Some(0.0),
            "the end's stated factor — the jar's own verdict"
        );
        assert_eq!(
            end.sky_factor(),
            0.0,
            "0.0 (the stated base) x 0.24 (the 15999 plateau) = 0.0 — no sky light \
             reaches the end's lightmap"
        );
        // a PARTIAL sync (dimension_types arrived, the current dimension
        // did not) gets NO built-in rescue — the v0.3.56 scoping
        // mirrored; for the real jar the snapshot answers the same
        // number the chain would (convergence), and the assertion pins
        // that the gate holds: a future jar whose tables drift from the
        // snapshot must NOT leak into a partial-sync world
        let mut partial = WorldModel::new();
        partial.current_dimension = Some("minecraft:overworld".into());
        partial.registry.dimension_types = vec![mc_protocol::DimensionTypeEntry {
            name: "minecraft:mock_dim".into(),
            min_y: 0,
            height: 256,
            default_clock: None,
            timelines: None,
        ambient_light_color: None,
        skybox: None,
        fog_start_distance: None,
        fog_end_distance: None,
        cloud_fog_end_distance: None,
        block_light_tint: None,
        sky_light_color: None,
        sky_light_factor: None,
        night_vision_color: None,
        }];
        let p5 = mc_protocol::play::test_support::set_time_packet(5, &[(0, 18_000, 0.0, 1.0)]);
        let ts5 = mc_protocol::play::parse_set_time(&p5).unwrap();
        assert!(partial.apply(PlayEvent::Time(ts5)));
        assert_eq!(partial.sky_factor(), 0.24, "the snapshot, not a built-in rescue");
    }

    #[test]
    fn the_built_in_tag_path_reads_the_registry_flatten() {
        // the skip-sync tag branch reads the REGISTRY's flatten (the
        // vanilla built-tag verdict — file order, depth-first `#`
        // expansion, first-occurrence dedup, fail-closed refusals —
        // pinned on synthetics in mc-registry), so the client keeps NO
        // private walk: ONE expansion truth for every tag table. The
        // member order the curve resolution feeds on is pinned here
        // through the resolved TimelineDefs, and the chain resolves
        // end-to-end: overworld's tag picks the DAY curve (the first
        // carrier of visual/sky_light_factor among the four), the
        // end's tag (villager_schedule only) carries NO track — the
        // chain-miss the snapshot fallback serves.
        use mc_registry::generated::worldgen::timeline;
        let members = mc_registry::flatten_timeline_tag("minecraft:in_overworld")
            .expect("the jar's overworld tag loads");
        let names: Vec<&str> = members
            .iter()
            .map(|m| timeline(m).expect("the flatten validated every member").name)
            .collect();
        assert_eq!(
            names,
            vec![
                "minecraft:villager_schedule", // via #universal, expanded FIRST (list order)
                "minecraft:day",
                "minecraft:moon",
                "minecraft:early_game",
            ],
            "the nested #universal ref expands in the jar's stored order"
        );
        // in_end / in_nether: the single-ref form — only universal's
        // member; and the unknown tag REFUSES whole (the vanilla
        // TagLoader verdict the registry owns, no partial candidates)
        for tag in ["minecraft:in_end", "minecraft:in_nether"] {
            let flat = mc_registry::flatten_timeline_tag(tag).expect(tag);
            assert_eq!(flat.len(), 1, "{tag}");
            assert_eq!(
                timeline(flat[0]).unwrap().name,
                "minecraft:villager_schedule",
                "{tag}"
            );
        }
        assert_eq!(mc_registry::flatten_timeline_tag("minecraft:mock_tag"), None);
        // the chain end-to-end on the model: the overworld curve IS the
        // day curve (keys byte-equal the generated table's carrier, the
        // period the tag arc pinned); the end has none
        let mut m = WorldModel::new();
        m.current_dimension = Some("minecraft:overworld".into());
        let (keys, period) = m
            .built_in_timeline_curve()
            .expect("the overworld tag resolves a curve");
        assert_eq!(
            keys,
            timeline("minecraft:day").expect("day").sky_light_factor,
            "the first carrier of the track is the day timeline"
        );
        assert_eq!(period, 24_000);
        let mut end = WorldModel::new();
        end.current_dimension = Some("minecraft:the_end".into());
        assert!(
            end.built_in_timeline_curve().is_none(),
            "no timeline in the end's tag carries the track"
        );
    }

    #[test]
    fn snapshot_adoption_fills_the_model() {
        let mut m = WorldModel::new();
        let info = WorldInfo {
            entity_id: 17,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        };
        let player = PlayerPosition {
            teleport_id: 42,
            x: 8.5,
            y: 64.0,
            z: -3.25,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 90.0,
            pitch: 0.0,
            flags: 0,
        };
        let col = ChunkColumn {
            x: 3,
            z: -2,
            sections: vec![parse_section_buffer(&stone_dirt_section(2))],
            light: ChunkLight::default(),
        };
        m.adopt_snapshot(WorldSnapshot {
            info: info.clone(),
            player: player.clone(),
            chunks: vec![col],
            registry: RegistryData {
                command_argument_types: Vec::new(),
                chat_types: Vec::new(),
                dimension_types: vec![DimensionTypeEntry {
                    name: "minecraft:overworld".to_string(),
                    min_y: -64,
                    height: 384,
                    default_clock: Some("minecraft:overworld".into()),
                    timelines: None,
                ambient_light_color: None,
                skybox: None,
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: None,
                block_light_tint: None,
                sky_light_color: None,
                sky_light_factor: None,
                night_vision_color: None,
                }],
                biomes: Vec::new(),
                biome_tags: Vec::new(),
                world_clocks: Vec::new(),
                entity_types: Vec::new(),
                entity_type_tags: Vec::new(),
                sound_events: Vec::new(),
                chicken_variants: Vec::new(),
                pig_variants: Vec::new(),
                cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
                timelines: Vec::new(),
                timeline_tags: Vec::new(),
            },
        });
        assert_eq!(m.loaded_chunks(), 1);
        assert_eq!(m.info.as_ref().unwrap().entity_id, 17);
        assert_eq!(m.player.as_ref().unwrap().teleport_id, 42);
        // the login's current dimension + the registry knowledge rode
        // along: min_y is LIVE, so block() takes WORLD-y now — the stone
        // at the column's local (0,0,0) sits at world y -64
        assert_eq!(m.min_y, Some(-64));
        assert_eq!(m.block(48, -64, -32), Some(1), "world-y query");
        assert_eq!(m.block(48, 0, -32), None, "local y no longer matches");
        // the registry knowledge rode along: the decoded dimension-type
        // vertical layout answers by registry key
        assert_eq!(m.dimension_min_y("minecraft:overworld"), Some(-64));
        assert_eq!(m.dimension_min_y("minecraft:the_end"), None);
        // a later sync position updates in place (the model holds the truth)
        let newer = PlayerPosition { y: 65.0, ..player };
        assert!(m.apply(PlayEvent::Position(newer)));
        assert_eq!(m.player.as_ref().unwrap().y, 65.0);
        // and a late world info does not clobber what is already known
        // unless it is a NEW packet (apply always stores — it IS the newest)
        let old_info = WorldInfo { entity_id: 1, ..info };
        assert!(m.apply(PlayEvent::WorldInfo(old_info)));
        assert_eq!(m.info.as_ref().unwrap().entity_id, 1);
    }

    #[test]
    fn registry_table_replaces_wholesale_with_each_adopted_snapshot() {
        let mut m = WorldModel::new();
        // no sync ever arrived: the BUILT-IN table (the same jar data a
        // matching server would sync) answers the vanilla names — the
        // skip-sync fallback. The wholesale rule below is about SYNCED
        // tables: once one arrives it is the ONLY truth.
        assert_eq!(
            m.dimension_min_y("minecraft:overworld"),
            Some(-64),
            "the built-in table serves before any join"
        );
        let snap = |names: &[(&str, i32, i32)]| WorldSnapshot {
            info: WorldInfo {
                entity_id: 1,
                hardcore: false,
                dimensions: vec!["minecraft:overworld".to_string()],
                view_distance: 8,
                simulation_distance: 8,
                portal_cooldown: 300,
                current_dimension: "minecraft:overworld".to_string(),
            },
            player: PlayerPosition {
                teleport_id: 1,
                x: 0.0,
                y: 0.0,
                z: 0.0,
                dx: 0.0,
                dy: 0.0,
                dz: 0.0,
                yaw: 0.0,
                pitch: 0.0,
                flags: 0,
            },
            chunks: vec![],
            registry: RegistryData {
                command_argument_types: Vec::new(),
                chat_types: Vec::new(),
                dimension_types: names
                    .iter()
                    .map(|(n, min_y, height)| DimensionTypeEntry {
                        name: n.to_string(),
                        min_y: *min_y,
                        height: *height,
                        default_clock: None,
                        timelines: None,
                    ambient_light_color: None,
                    skybox: None,
                    fog_start_distance: None,
                    fog_end_distance: None,
                    cloud_fog_end_distance: None,
                    block_light_tint: None,
                    sky_light_color: None,
                    sky_light_factor: None,
                    night_vision_color: None,
                    })
                    .collect(),
                biomes: Vec::new(),
                biome_tags: Vec::new(),
                world_clocks: Vec::new(),
                entity_types: Vec::new(),
                entity_type_tags: Vec::new(),
                sound_events: Vec::new(),
                chicken_variants: Vec::new(),
                pig_variants: Vec::new(),
                cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
                timelines: Vec::new(),
                timeline_tags: Vec::new(),
            },
        };
        m.adopt_snapshot(snap(&[
            ("minecraft:overworld", -64, 384),
            ("minecraft:the_nether", 0, 256),
        ]));
        assert_eq!(m.dimension_min_y("minecraft:overworld"), Some(-64));
        assert_eq!(m.dimension_min_y("minecraft:the_nether"), Some(0));
        // a re-join REPLACES the table wholesale — no stale entries linger
        m.adopt_snapshot(snap(&[("minecraft:flat", 0, 16)]));
        assert_eq!(m.dimension_min_y("minecraft:overworld"), None, "the sync speaks, the built-in table stays silent");
        assert_eq!(m.dimension_min_y("minecraft:the_nether"), None);
        assert_eq!(m.dimension_min_y("minecraft:flat"), Some(0));
    }

    #[test]
    fn the_built_in_dimension_table_is_the_skip_sync_fallback() {
        // NO registry data at all: the generator's built-in table — the
        // SAME jar data a matching server would sync — answers the
        // vanilla dimension names with their real layouts
        let mut m = WorldModel::new();
        assert_eq!(m.dimension_min_y("minecraft:overworld"), Some(-64));
        assert_eq!(m.dimension_min_y("minecraft:overworld_caves"), Some(-64));
        assert_eq!(m.dimension_min_y("minecraft:the_nether"), Some(0));
        assert_eq!(m.dimension_min_y("minecraft:the_end"), Some(0));
        assert_eq!(
            m.dimension_min_y("minecraft:flat"),
            None,
            "a name the vanilla jar does not build stays unknown"
        );
        // the REAL flow through the fallback: a bare server's Login pins
        // min_y -64 and block() reads world-y with zero sync
        let info = WorldInfo {
            entity_id: 7,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        };
        assert!(m.apply(PlayEvent::WorldInfo(info)));
        assert_eq!(m.min_y, Some(-64), "the login pin resolves through the built-in table");
        // a Respawn re-pins through the fallback on the dimension change
        let spawn = SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        };
        assert!(m.apply(PlayEvent::Respawn(spawn)));
        assert_eq!(m.min_y, Some(0), "the nether re-pin: floor 0 from the built-in table");
        // an ARRIVED sync is the only truth: a table without the key gets
        // None (the server runs its own dimension set), and a key the sync
        // DOES carry wins even when it disagrees with the jar
        let snap = |names: &[(&str, i32, i32)]| WorldSnapshot {
            info: WorldInfo {
                entity_id: 7,
                hardcore: false,
                dimensions: vec!["minecraft:overworld".to_string()],
                view_distance: 8,
                simulation_distance: 8,
                portal_cooldown: 300,
                current_dimension: "minecraft:overworld".to_string(),
            },
            player: PlayerPosition {
                teleport_id: 1,
                x: 0.0,
                y: 0.0,
                z: 0.0,
                dx: 0.0,
                dy: 0.0,
                dz: 0.0,
                yaw: 0.0,
                pitch: 0.0,
                flags: 0,
            },
            chunks: vec![],
            registry: RegistryData {
                command_argument_types: Vec::new(),
                chat_types: Vec::new(),
                dimension_types: names
                    .iter()
                    .map(|(n, min_y, height)| DimensionTypeEntry {
                        name: n.to_string(),
                        min_y: *min_y,
                        height: *height,
                        default_clock: None,
                        timelines: None,
                    ambient_light_color: None,
                    skybox: None,
                    fog_start_distance: None,
                    fog_end_distance: None,
                    cloud_fog_end_distance: None,
                    block_light_tint: None,
                    sky_light_color: None,
                    sky_light_factor: None,
                    night_vision_color: None,
                    })
                    .collect(),
                biomes: Vec::new(),
                biome_tags: Vec::new(),
                world_clocks: Vec::new(),
                entity_types: Vec::new(),
                entity_type_tags: Vec::new(),
                sound_events: Vec::new(),
                chicken_variants: Vec::new(),
                pig_variants: Vec::new(),
                cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
                timelines: Vec::new(),
                timeline_tags: Vec::new(),
            },
        };
        m.adopt_snapshot(snap(&[("minecraft:overworld", 0, 128)]));
        assert_eq!(
            m.dimension_min_y("minecraft:overworld"),
            Some(0),
            "the synced entry wins over the built-in -64"
        );
        m.adopt_snapshot(snap(&[("minecraft:custom", -32, 64)]));
        assert_eq!(
            m.dimension_min_y("minecraft:overworld"),
            None,
            "a sync that omits the key gets NO built-in rescue"
        );
    }

    #[test]
    fn dimension_pins_at_login_and_respawn_repins_min_y() {
        let mut m = WorldModel::new();
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![
                DimensionTypeEntry { name: "minecraft:overworld".into(), min_y: -64, height: 384, default_clock: Some("minecraft:overworld".into()) , timelines: None, ambient_light_color: None, skybox: None, fog_start_distance: None, fog_end_distance: None, cloud_fog_end_distance: None, block_light_tint: None, sky_light_color: None, sky_light_factor: None, night_vision_color: None },
                DimensionTypeEntry { name: "minecraft:the_nether".into(), min_y: 0, height: 256, default_clock: None , timelines: None, ambient_light_color: None, skybox: None, fog_start_distance: None, fog_end_distance: None, cloud_fog_end_distance: None, block_light_tint: None, sky_light_color: None, sky_light_factor: None, night_vision_color: None },
            ],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: Vec::new(),
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        assert_eq!(m.min_y, None, "no knowledge before the server speaks");
        // the login carries the current dimension (CommonPlayerSpawnInfo)
        assert!(m.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 17,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        })));
        assert_eq!(m.current_dimension.as_deref(), Some("minecraft:overworld"));
        assert_eq!(m.min_y, Some(-64), "the registry layout goes live at login");
        // block() now answers WORLD-y: section 0 of a column starts at -64
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![parse_section_buffer(&stone_dirt_section(2))],
            light: ChunkLight::default(),
        }));
        assert_eq!(m.block(0, -64, 0), Some(1), "world -64 = section 0 local 0");
        assert_eq!(m.block(3, -63, 2), Some(10), "local (3,1,2) at world -63");
        // Respawn: a dimension change re-pins from the same registry AND
        // CLEARS the loaded columns — the vanilla level recreation. The
        // overworld column re-read under the nether min_y would be pure
        // fiction (its blocks sit at world -64..-49, far below the nether
        // floor 0); the honest state is an empty map.
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 1,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 60,
        })));
        assert_eq!(m.current_dimension.as_deref(), Some("minecraft:the_nether"));
        assert_eq!(m.min_y, Some(0), "nether layout takes over");
        assert_eq!(m.loaded_chunks(), 0, "the old dimension's level dropped");
        assert_eq!(m.block(0, 0, 0), None, "no stale column survives the crossing");
        // the NEW dimension's own chunks land fresh — nether world 0 is
        // section 0's bottom now
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![parse_section_buffer(&stone_dirt_section(2))],
            light: ChunkLight::default(),
        }));
        assert_eq!(m.block(0, 0, 0), Some(1), "nether world 0 = section 0 local 0");
        assert_eq!(m.block(0, -1, 0), None, "below the nether floor: outside");
        // an unknown dimension key keeps the previous min_y — never a
        // guess — but STILL clears the level (any Respawn is a crossing)
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 9,
            dimension: "minecraft:custom_dim".to_string(),
            portal_cooldown: 0,
        })));
        assert_eq!(m.current_dimension.as_deref(), Some("minecraft:custom_dim"));
        assert_eq!(m.min_y, Some(0), "stale-but-honest: unknown key, layout unchanged");
        assert_eq!(m.loaded_chunks(), 0, "the level drops on every dimension change");
    }

    #[test]
    fn entities_spawn_move_and_despawn_in_place() {
        let mut m = WorldModel::new();
        // the spawn packet CREATES: the snapshot is the spawn's own
        // facts — the registry-typed existence, the full-precision
        // position, the byte angles unpacked the vanilla way
        assert!(m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 7,
            uuid: 0x0011_2233_4455_6677_8899_AABB_CCDD_EEFF,
            type_id: 2101,
            x: -12.5,
            y: 64.0,
            z: 3.25,
            movement: [0.0; 3],
            x_rot: -180.0, // the spawn byte -128 unpacked
            y_rot: 178.59375, // the spawn byte 127 unpacked
            y_head_rot: 0.0,
            data: 7,
        })));
        let e = m.entity(7).expect("the spawn created the entity");
        assert_eq!(
            (e.type_id, e.x, e.y, e.z, e.y_rot, e.x_rot),
            (2101, -12.5, 64.0, 3.25, 178.59375, -180.0)
        );
        // re-spawning the SAME id REPLACES (the vanilla lookup is keyed
        // by id — the map does not grow a second entry)
        assert!(m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 7,
            uuid: 42,
            type_id: 1,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 0.0,
            y_head_rot: 0.0,
            data: 0,
        })));
        assert_eq!(m.entity(7).unwrap().x, 0.0, "the newer spawn won");
        // a second id keeps both alive
        assert!(m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 8,
            uuid: 43,
            type_id: 2,
            x: 1.0,
            y: 2.0,
            z: 3.0,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 0.0,
            y_head_rot: 0.0,
            data: 0,
        })));
        // the sync MOVES in place: the authoritative correction rewrites
        // the position AND the exact float angles
        assert!(m.apply(PlayEvent::EntityMoved(EntityPositionSync {
            id: 7,
            x: 100.5,
            y: -64.25,
            z: 300.75,
            delta: [0.1, 0.2, 0.3],
            y_rot: 45.0,
            x_rot: -10.0,
            on_ground: true,
        })));
        let e = m.entity(7).expect("the sync moved, not replaced");
        assert_eq!(
            (e.x, e.y, e.z, e.y_rot, e.x_rot),
            (100.5, -64.25, 300.75, 45.0, -10.0)
        );
        assert_eq!(m.entity(8).unwrap().x, 1.0, "the neighbor is untouched");
        // a sync for an UNKNOWN id is refused — the spawn packet stays
        // the only creator (no phantom entities from a correction)
        assert!(!m.apply(PlayEvent::EntityMoved(EntityPositionSync {
            id: 999,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            delta: [0.0; 3],
            y_rot: 0.0,
            x_rot: 0.0,
            on_ground: false,
        })));
        assert!(m.entity(999).is_none(), "no phantom spawned");
        // the batch removal: whatever existed is gone, unknowns are
        // honest no-ops, and the report is whether ANYTHING changed
        assert!(m.apply(PlayEvent::EntitiesRemoved(vec![7, 999])));
        assert!(m.entity(7).is_none());
        assert!(m.entity(8).is_some(), "the unlisted neighbor survives");
        // nothing left in the batch existed — the no-op reports false
        assert!(!m.apply(PlayEvent::EntitiesRemoved(vec![7, 999])));
    }

    #[test]
    fn the_vehicles_belief_rides_the_packet_law() {
        let mut m = WorldModel::new();
        m.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 1,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        }));
        // the world's population: the local player's entity (id 1),
        // two boats (ids 7 and 9) — the belief's own cast
        for (id, type_id) in [(1, 1), (7, 2101), (9, 2102)] {
            m.apply(PlayEvent::Spawn(EntitySpawn {
                id,
                uuid: id as u128,
                type_id,
                x: 0.0,
                y: 64.0,
                z: 0.0,
                movement: [0.0; 3],
                x_rot: 0.0,
                y_rot: 90.0,
                y_head_rot: 0.0,
                data: 0,
            }));
        }
        // THE UNKNOWN VEHICLE REFUSES (the handler's own warn-and-
        // return): level.getEntity(vehicle) == null lands NOTHING —
        // the belief stays silent, the world never learns the lie
        assert!(!m.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 99,
            passengers: vec![1],
        })));
        assert_eq!(m.vehicle_of(1), None, "an unknown mount carries nobody");

        // THE MOUNT: the boat 7 carries the player (the FULL list the
        // packet speaks, the belief re-baselines at it)
        assert!(m.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 7,
            passengers: vec![1],
        })));
        assert_eq!(m.vehicle_of(1), Some(7));
        assert_eq!(m.vehicle_passengers(7).map(|r| r.as_slice()), Some(&[1][..]));

        // THE REMOUNT'S EJECT WALK: the player switches to boat 9 —
        // the startRiding's own stopRiding empties boat 7's seat
        // before boat 9's list lands
        assert!(m.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 9,
            passengers: vec![1],
        })));
        assert_eq!(m.vehicle_of(1), Some(9));
        assert_eq!(
            m.vehicle_passengers(7).map(|r| r.as_slice()),
            None,
            "the old vehicle's seat emptied (no riders, no entry)"
        );

        // THE MULTI-RIDER LIST: boat 9 carries the player and a mate —
        // the seat index the attachment law reads
        m.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 9,
            passengers: vec![7, 1],
        }));
        assert_eq!(m.passenger_index(1), Some(1), "the second seat");
        assert_eq!(m.passenger_index(7), Some(0), "the front seat");

        // THE DISMOUNT BROADCAST: the empty list drops the entry — the
        // walk owns the player again
        assert!(m.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 9,
            passengers: vec![],
        })));
        assert_eq!(m.vehicle_of(1), None, "the dismount is the freedom");
        assert_eq!(m.passenger_index(7), None);

        // THE DEAD MOUNT PRUNES: the vehicle's removal takes its
        // riders' belief away (the removal IS the auto-dismount)
        m.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 7,
            passengers: vec![1],
        }));
        assert_eq!(m.vehicle_of(1), Some(7));
        assert!(m.apply(PlayEvent::EntitiesRemoved(vec![7])));
        assert_eq!(m.vehicle_of(1), None, "the dead boat carries nobody");

        // THE RESPAWN CLEARS THE BELIEF (the vehicles ARE the
        // entities — the crossed dimension's mounts never follow)
        m.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 9,
            passengers: vec![1],
        }));
        assert_eq!(m.vehicle_of(1), Some(9));
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 1,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 60,
        })));
        assert_eq!(m.vehicle_of(1), None, "the ride belief dies with the level");
    }

    /// THE ARROW SYNC'S OWN MAILBOX (the EntityMoved walk's arrow arm):
    /// the arrow-type sync arms the mailbox (the drain ships the word
    /// once, the second drain the spent silence keeps), the non-arrow
    /// sync stays outside (the mobs ride the store's own projection
    /// law), the unknown id is refused AND arms nothing, and the level
    /// change empties the box (the same-ids law the voices' own walk
    /// keeps).
    #[test]
    fn the_arrow_sync_words_the_mailbox() {
        let mut m = WorldModel::new();
        // the synced registry: type id 0 = zombie (the mob lane's own
        // body), id 1 = arrow (the shot the sim lane serves)
        m.registry.entity_types = vec![
            "minecraft:zombie".to_string(),
            "minecraft:arrow".to_string(),
        ];
        // the arrow rides the store beside a zombie (the lane's own
        // contrast)
        for (id, type_id) in [(7, 0), (8, 1)] {
            assert!(m.apply(PlayEvent::Spawn(EntitySpawn {
                id,
                uuid: id as u128,
                type_id,
                x: 10.0,
                y: 64.0,
                z: 10.0,
                movement: [0.0; 3],
                x_rot: 0.0,
                y_rot: 0.0,
                y_head_rot: 0.0,
                data: 0,
            })));
        }
        // THE ZOMBIE'S SYNC: the store re-poses, the mailbox stays
        // silent (the mobs never enter the arrows' own lane)
        assert!(m.apply(PlayEvent::EntityMoved(EntityPositionSync {
            id: 7,
            x: 12.0,
            y: 64.0,
            z: 10.0,
            delta: [0.0; 3],
            y_rot: 0.0,
            x_rot: 0.0,
            on_ground: true,
        })));
        assert!(m.arrow_syncs.is_empty(), "the mob stays outside");
        // THE ARROW'S SYNC: the store re-poses AND the mailbox arms
        // (the sim lane's wire word — the position and the exact
        // float angles; the delta word never rides)
        assert!(m.apply(PlayEvent::EntityMoved(EntityPositionSync {
            id: 8,
            x: 14.5,
            y: 65.25,
            z: 9.75,
            delta: [0.1, 0.2, 0.3],
            y_rot: -45.5,
            x_rot: 10.0,
            on_ground: false,
        })));
        assert_eq!(m.entity(8).unwrap().x, 14.5, "the store's own truth");
        assert_eq!(m.arrow_syncs.len(), 1, "the word armed");
        assert_eq!(m.arrow_syncs[0].id, 8);
        assert_eq!(m.arrow_syncs[0].x, 14.5);
        assert_eq!(m.arrow_syncs[0].y_rot, -45.5);
        // THE DRAIN: the word ships exactly once, the spent silence
        // ships nothing
        let drained = m.drain_arrow_syncs();
        assert_eq!(drained.len(), 1, "the drain ships the word");
        assert!(m.arrow_syncs.is_empty(), "the mailbox spent");
        assert!(
            m.drain_arrow_syncs().is_empty(),
            "the drained silence ships nothing"
        );
        // THE UNKNOWN ID: refused AND armed nothing (the spawn packet
        // stays the only creator)
        assert!(!m.apply(PlayEvent::EntityMoved(EntityPositionSync {
            id: 999,
            x: 1.0,
            y: 1.0,
            z: 1.0,
            delta: [0.0; 3],
            y_rot: 0.0,
            x_rot: 0.0,
            on_ground: false,
        })));
        assert!(m.arrow_syncs.is_empty(), "no phantom word");
        // THE LEVEL CHANGE: the mailbox dies with the old level (the
        // same-ids law the voices' own walk keeps)
        assert!(m.apply(PlayEvent::EntityMoved(EntityPositionSync {
            id: 8,
            x: 15.0,
            y: 65.0,
            z: 9.0,
            delta: [0.0; 3],
            y_rot: 0.0,
            x_rot: 0.0,
            on_ground: false,
        })));
        assert_eq!(m.arrow_syncs.len(), 1, "the word armed again");
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".to_string(),
            portal_cooldown: 60,
        })));
        assert!(m.arrow_syncs.is_empty(), "the box dies with the level");
    }

    #[test]
    fn the_position_sync_folds_the_calculate_absolute_law() {
        let mut m = WorldModel::new();
        // THE MOUNT WORD (an absolute pose — flags 0 — lands verbatim:
        // the vanilla teleport's own shape, every replace arm keeps)
        let base = PlayerPosition {
            teleport_id: 1,
            x: 10.0,
            y: 64.0,
            z: 20.0,
            dx: 0.1,
            dy: 0.2,
            dz: 0.3,
            yaw: 30.0,
            pitch: -45.0,
            flags: 0,
        };
        assert!(m.apply(PlayEvent::Position(base.clone())));
        let p = m.player.as_ref().expect("the pose landed");
        assert_eq!((p.x, p.y, p.z), (10.0, 64.0, 20.0));
        assert_eq!((p.yaw, p.pitch), (30.0, -45.0));
        assert_eq!((p.dx, p.dy, p.dz), (0.1, 0.2, 0.3));

        // THE RELATIVE X ARM (bit 0): the stored x the fold reads, the
        // packet's delta the add speaks; EVERY absent arm replaces —
        // the angles, the other axes, and the velocity word too
        let rel = PlayerPosition {
            teleport_id: 2,
            x: 0.5,
            y: 70.0,
            z: 21.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 90.0,
            pitch: 10.0,
            flags: RELATIVE_X,
        };
        assert!(m.apply(PlayEvent::Position(rel)));
        let p = m.player.as_ref().expect("the fold landed");
        assert_eq!(p.x, 10.5, "the relative x added");
        assert_eq!((p.y, p.z), (70.0, 21.0), "the absent bits replaced");
        assert_eq!((p.yaw, p.pitch), (90.0, 10.0), "the angles replaced too");
        assert_eq!((p.dx, p.dy, p.dz), (0.0, 0.0, 0.0), "the delta replaced");
        assert_eq!(p.flags, RELATIVE_X, "the flags ride verbatim");

        // THE RELATIVE YAW ARM (bit 3) + THE PITCH'S OWN BAND: the yaw
        // adds (90 + 15), the pitch adds THROUGH the ±90 clamp
        // (Mth.clamp the fold's own walk keeps — 10 + 100 = 110 speaks 90)
        let rel = PlayerPosition {
            teleport_id: 3,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 15.0,
            pitch: 100.0,
            flags: RELATIVE_Y_ROT | RELATIVE_X_ROT,
        };
        assert!(m.apply(PlayEvent::Position(rel)));
        let p = m.player.as_ref().expect("the fold landed");
        assert_eq!(p.yaw, 105.0, "the relative yaw added");
        assert_eq!(p.pitch, 90.0, "the band held the + side");

        // THE ABSOLUTE RESET (the walk's known baseline: the pose, the
        // pitch -45 the band's - side reads, the delta (0,0,1) the
        // rotation spins)
        let reset = PlayerPosition {
            teleport_id: 4,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            dx: 0.0,
            dy: 0.0,
            dz: 1.0,
            yaw: 0.0,
            pitch: -45.0,
            flags: 0,
        };
        assert!(m.apply(PlayEvent::Position(reset.clone())));
        let p = m.player.as_ref().expect("landed");
        assert_eq!(p.pitch, -45.0);
        assert_eq!(p.dz, 1.0);

        // THE BAND'S OTHER SIDE: -45 - 60 = -105 speaks -90
        let rel = PlayerPosition {
            teleport_id: 5,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 0.0,
            pitch: -60.0,
            flags: RELATIVE_X_ROT,
        };
        assert!(m.apply(PlayEvent::Position(rel)));
        assert_eq!(m.player.as_ref().expect("landed").pitch, -90.0);

        // THE CUMULATIVE FOLD: two relative y syncs add twice — the
        // stored pose IS the next fold's current
        let rel = PlayerPosition {
            teleport_id: 6,
            x: 0.0,
            y: 1.0,
            z: 0.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: RELATIVE_Y,
        };
        assert!(m.apply(PlayEvent::Position(rel.clone())));
        assert_eq!(m.player.as_ref().expect("landed").y, 1.0, "0 + 1");
        assert!(m.apply(PlayEvent::Position(rel)));
        assert_eq!(m.player.as_ref().expect("landed").y, 2.0, "1 + 1");

        // THE ABSOLUTE RESET (the delta (0,0,1) and the yaw 0 the
        // rotation's own walk reads)
        let reset = PlayerPosition {
            teleport_id: 7,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            dx: 0.0,
            dy: 0.0,
            dz: 1.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
        };
        assert!(m.apply(PlayEvent::Position(reset)));

        // THE VELOCITY WORD: the ROTATE_DELTA bit spins the current
        // delta FIRST (Vec3.xRot then yRot — the Mth table the angles
        // walk), then the axis bits calculateDelta's. The yaw bit adds
        // 30 (0 -> 30): the dYRot the current-minus-new speaks
        // (-30deg) rotates the (0,0,1) onto (-0.5, 0, sqrt(3)/2) —
        // the xRot arm the unchanged pitch keeps identity (cos 0, sin
        // 0). The axis bits RELATIVE-add, the rotated word survives.
        let rotated = PlayerPosition {
            teleport_id: 8,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 30.0,
            pitch: 0.0,
            flags: RELATIVE_Y_ROT
                | RELATIVE_ROTATE_DELTA
                | RELATIVE_DELTA_X
                | RELATIVE_DELTA_Y
                | RELATIVE_DELTA_Z,
        };
        assert!(m.apply(PlayEvent::Position(rotated)));
        let p = m.player.as_ref().expect("landed");
        // THE TABLE'S OWN WORDS (the quantization the index walk keeps:
        // the -30deg the dYRot speaks lands at the f32 angle
        // -0.5235988, the *10430.378... index -5461.33 the d2l
        // truncates to -5461 -> the cells 60075/10922 — the
        // floor's own -0.0055deg the verdicts carry, bit-for-bit the
        // jar's SIN array at those cells)
        assert_eq!(
            p.dx, -0.49997231364250183,
            "the yRot walk carried the table's sin cell"
        );
        assert_eq!(p.dy, 0.0);
        assert_eq!(
            p.dz, 0.8659934401512146,
            "the z walked the table's cos cell"
        );
    }

    #[test]
    fn the_position_sync_never_lands_while_the_ride_holds() {
        let mut m = WorldModel::new();
        m.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 1,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        }));
        m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 9,
            uuid: 9,
            type_id: 2101,
            x: 0.0,
            y: 64.0,
            z: 0.0,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 0.0,
            y_head_rot: 0.0,
            data: 0,
        }));
        // THE MOUNT WORD: the server's teleport spoke first — the pose
        // lands free-standing (the gate the no-ride belief keeps open)
        let mount = PlayerPosition {
            teleport_id: 1,
            x: 10.0,
            y: 64.0,
            z: 20.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 30.0,
            pitch: 0.0,
            flags: 0,
        };
        assert!(m.apply(PlayEvent::Position(mount.clone())));
        assert_eq!(m.player.as_ref().expect("landed").y, 64.0);
        // THE MOUNT: the boat's list claims the player — the belief
        // the gate reads now holds
        m.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 9,
            passengers: vec![1],
        }));
        // THE SYNC WHILE RIDING: consumed (true — the confirm still
        // spoke inside the pump) but the pose NEVER lands — the boat's
        // own syncs carry the rider, the stored word stays the mount's
        let mid = PlayerPosition { teleport_id: 2, y: 70.0, ..mount };
        assert!(m.apply(PlayEvent::Position(mid)));
        assert_eq!(
            m.player.as_ref().expect("the mount word kept").y,
            64.0,
            "the ride holds the pose"
        );
        // THE DISMOUNT: the empty list drops the belief — the gate
        // opens, the dismount teleport's own word lands (the vanilla
        // server's dismountTo the position lane always speaks)
        m.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 9,
            passengers: vec![],
        }));
        let land = PlayerPosition {
            teleport_id: 3,
            x: 11.0,
            y: 70.5,
            z: 21.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 30.0,
            pitch: 0.0,
            flags: 0,
        };
        assert!(m.apply(PlayEvent::Position(land)));
        let p = m.player.as_ref().expect("the dismount teleport landed");
        assert_eq!((p.x, p.y, p.z), (11.0, 70.5, 21.0));
    }

    #[test]
    fn entities_follow_the_dimension_swap() {
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 7,
            uuid: 1,
            type_id: 2101,
            x: 0.0,
            y: 64.0,
            z: 0.0,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 0.0,
            y_head_rot: 0.0,
            data: 0,
        }));
        assert!(m.entity(7).is_some());
        // the Respawn crossing clears the population with the columns —
        // the vanilla client swaps the whole level; the old dimension's
        // mobs do not follow the player through the portal
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 1,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 60,
        })));
        assert!(m.entity(7).is_none(), "no entity crosses the dimension swap");
        // the new dimension's spawns land fresh in the cleared registry
        assert!(m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 7,
            uuid: 2,
            type_id: 3,
            x: 8.0,
            y: 32.0,
            z: 8.0,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 0.0,
            y_head_rot: 0.0,
            data: 0,
        })));
        assert_eq!(m.entity(7).unwrap().x, 8.0);
    }

    /// The delta walk IS the vanilla codec, not a naive `+=`: a
    /// non-zero lane moves the QUANTIZED base (the position snaps to
    /// the 1/4096 grid), a zero lane keeps its axis EXACT, and the
    /// all-zero payload is the codec's own early return — nothing
    /// moved, nothing changed. The spawn's fractional positions prove
    /// both halves: the zero lane survives UNQUANTIZED while the moved
    /// axes land on the grid.
    #[test]
    fn entity_delta_walk_is_the_vanilla_codec() {
        let mut m = WorldModel::new();
        spawn_at(&mut m, 7, 8.0001, 32.0, -8.0001);
        // a real step: the moved axes quantize, the untouched axis does
        // not — (round(8.0001*4096) + 1)/4096 = 32769/4096 on the x,
        // the exact 32.0 on the y, the grid-snapped -32769/4096 on the z
        assert!(m.apply(PlayEvent::EntityDeltaPos(EntityDeltaPos {
            id: 7,
            dx: 1,
            dy: 0,
            dz: -1,
            on_ground: true,
        })));
        let e = m.entity(7).unwrap();
        assert_eq!(e.x, 32769.0 / 4096.0, "the moved axis snapped to the grid");
        assert_eq!(e.y, 32.0, "the zero lane kept the axis EXACT (unquantized)");
        assert_eq!(e.z, -32769.0 / 4096.0, "the negative axis walked the grid");
        // the all-zero payload is the codec's early return: the base
        // comes back UNCHANGED — including its off-grid fraction
        spawn_at(&mut m, 9, 0.0001, 64.0002, -0.0003);
        let before = m.entity(9).unwrap().clone();
        assert!(
            !m.apply(PlayEvent::EntityDeltaPos(EntityDeltaPos {
                id: 9,
                dx: 0,
                dy: 0,
                dz: 0,
                on_ground: false,
            })),
            "the all-zero walk moved nothing"
        );
        assert_eq!(m.entity(9).unwrap(), &before, "no quantization on the idle walk");
    }

    /// The walk's round direction is JAVA's `Math.round` — TIES TOWARD
    /// POSITIVE INFINITY. Rust's `f64::round` rounds halves away from
    /// zero, so on a negative half-grid base the naive port lands ONE
    /// 1/4096 GRID STEP LOW: `Math.round(-0.5)` is 0, not -1. The
    /// spawn position is `2^-13` (exact in f64, `* 4096` = -0.5 on the
    /// tie) — the vanilla direction walks to 1/4096, the wrong one to
    /// 0.0, and the test pins the vanilla one.
    #[test]
    fn the_delta_walk_rounds_ties_toward_positive_infinity() {
        let mut m = WorldModel::new();
        spawn_at(&mut m, 7, -0.0001220703125, 0.0, 0.0); // = -2^-13
        assert!(m.apply(PlayEvent::EntityDeltaPos(EntityDeltaPos {
            id: 7,
            dx: 1,
            dy: 0,
            dz: 0,
            on_ground: true,
        })));
        let e = m.entity(7).unwrap();
        assert_eq!(
            e.x,
            1.0 / 4096.0,
            "Math.round(-0.5) is 0 (ties to +inf), then +1 delta = 1/4096"
        );
    }

    /// The rotation family: the pos-rot packet walks AND turns (the
    /// angle bytes are the Add Entity quantization), the rot-only
    /// packet turns WITHOUT touching the position, and the pos-rot
    /// walk's rotation is asserted even on an all-zero walk.
    #[test]
    fn the_rotation_family_turns_the_body_without_moving_it() {
        let mut m = WorldModel::new();
        spawn_at(&mut m, 7, 8.0, 32.0, -8.0);
        // pos + rot: the walk AND the angles land
        assert!(m.apply(PlayEvent::EntityDeltaPosRot(EntityDeltaPosRot {
            id: 7,
            dx: 16,
            dy: 0,
            dz: 0,
            y_rot: -180.0,
            x_rot: 178.59375,
            on_ground: true,
        })));
        let e = m.entity(7).unwrap();
        assert_eq!(e.x, 8.00390625, "the 16-unit grid delta walked");
        assert_eq!((e.y_rot, e.x_rot), (-180.0, 178.59375));
        // rot-only: the position is untouched, the body turns in place
        assert!(m.apply(PlayEvent::EntityDeltaRot(EntityDeltaRot {
            id: 7,
            y_rot: 45.0,
            x_rot: -45.0,
            on_ground: false,
        })));
        let e = m.entity(7).unwrap();
        assert_eq!(e.x, 8.00390625, "the rot packet carries no position lane");
        assert_eq!((e.y_rot, e.x_rot), (45.0, -45.0));
        // the all-zero pos-rot walk still asserts its rotation
        assert!(m.apply(PlayEvent::EntityDeltaPosRot(EntityDeltaPosRot {
            id: 7,
            dx: 0,
            dy: 0,
            dz: 0,
            y_rot: 90.0,
            x_rot: 0.0,
            on_ground: false,
        })));
        let e = m.entity(7).unwrap();
        assert_eq!(e.x, 8.00390625, "no walk on the zero lanes");
        assert_eq!(e.y_rot, 90.0, "the rotation landed anyway");
    }

    /// The head yaw is its OWN channel: the spawn seeds it (the spawn
    /// byte unpacked the vanilla way), Rotate Head keeps it current,
    /// and the body packets NEVER touch it. An unknown id is refused
    /// by every mover — the spawn packet stays the ONLY creator.
    #[test]
    fn the_head_yaw_is_its_own_channel() {
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 7,
            uuid: 1,
            type_id: 3,
            x: 0.0,
            y: 32.0,
            z: 0.0,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 10.0,
            y_head_rot: 20.0,
            data: 0,
        }));
        assert_eq!(m.entity(7).unwrap().y_head_rot, 20.0, "the spawn seeds the head");
        // the head turns WITHOUT the body following
        assert!(m.apply(PlayEvent::EntityHeadRot(EntityHeadRot {
            id: 7,
            y_head_rot: -90.0,
        })));
        let e = m.entity(7).unwrap();
        assert_eq!(e.y_head_rot, -90.0);
        assert_eq!(e.y_rot, 10.0, "the body's yaw is a different channel");
        // every mover refuses an unknown id — no phantom entities
        assert!(!m.apply(PlayEvent::EntityDeltaPos(EntityDeltaPos {
            id: 99,
            dx: 1,
            dy: 0,
            dz: 0,
            on_ground: true,
        })));
        assert!(!m.apply(PlayEvent::EntityDeltaPosRot(EntityDeltaPosRot {
            id: 99,
            dx: 0,
            dy: 0,
            dz: 0,
            y_rot: 0.0,
            x_rot: 0.0,
            on_ground: false,
        })));
        assert!(!m.apply(PlayEvent::EntityDeltaRot(EntityDeltaRot {
            id: 99,
            y_rot: 0.0,
            x_rot: 0.0,
            on_ground: false,
        })));
        assert!(!m.apply(PlayEvent::EntityHeadRot(EntityHeadRot {
            id: 99,
            y_head_rot: 0.0,
        })));
        assert!(m.entity(99).is_none(), "no mover creates");
        assert!(m.entity(7).is_some(), "the neighbor is untouched");
    }

    /// Set Entity Data merges into the entity's metadata store: the
    /// spawn starts it empty, each packet's entries REPLACE the listed
    /// accessors and leave the unmentioned ones (the assignValues
    /// walk), a null lookup DROPS the packet (vanilla's handle — no
    /// phantom entity, no panic), and a re-spawn replaces the whole
    /// snapshot — the metadata store with it.
    #[test]
    fn entity_data_merges_into_the_metadata_store() {
        let mut m = WorldModel::new();
        spawn_at(&mut m, 7, 0.0, 32.0, 0.0);
        // the first set seeds the store
        assert!(m.apply(PlayEvent::EntityData(EntityDataSet {
            id: 7,
            entries: vec![
                SetEntityDataEntry {
                    index: 0,
                    serializer: 0,
                    value: EntityDataEntry::Byte(1),
                },
                SetEntityDataEntry {
                    index: 6,
                    serializer: 20,
                    value: EntityDataEntry::Pose(6),
                },
            ],
        })));
        let e = m.entity(7).unwrap();
        assert_eq!(e.data.get(&0), Some(&EntityDataEntry::Byte(1)));
        assert_eq!(e.data.get(&6), Some(&EntityDataEntry::Pose(6)));
        // the second set replaces ONE accessor and leaves the other —
        // never a wholesale reset (the vanilla assignValues walk)
        assert!(m.apply(PlayEvent::EntityData(EntityDataSet {
            id: 7,
            entries: vec![SetEntityDataEntry {
                index: 0,
                serializer: 0,
                value: EntityDataEntry::Byte(0),
            }],
        })));
        let e = m.entity(7).unwrap();
        assert_eq!(e.data.get(&0), Some(&EntityDataEntry::Byte(0)), "replaced");
        assert_eq!(e.data.get(&6), Some(&EntityDataEntry::Pose(6)), "untouched");
        // a null lookup DROPS the packet: no phantom entity, the real
        // one untouched (the vanilla `ifnull -> return`)
        assert!(!m.apply(PlayEvent::EntityData(EntityDataSet {
            id: 99,
            entries: vec![SetEntityDataEntry {
                index: 0,
                serializer: 0,
                value: EntityDataEntry::Byte(1),
            }],
        })));
        assert!(m.entity(99).is_none());
        assert_eq!(m.entity(7).unwrap().data.get(&0), Some(&EntityDataEntry::Byte(0)));
        // the re-spawn replaces the WHOLE snapshot — the store with it
        spawn_at(&mut m, 7, 1.0, 32.0, 1.0);
        assert!(m.entity(7).unwrap().data.is_empty(), "the re-spawn resets the store");
    }

    #[test]
    fn the_night_vision_const_is_pinned_to_the_generated_registry_table() {
        // the round-120 probe became round-122's GENERATED table: the
        // checked-in DumpMobEffects.java boots the official jar's real
        // registries and mc-registry carries the 40-entry wire order.
        // The const must name the table's slot — a regeneration that
        // moves night_vision (a future version's registration change)
        // fails HERE, not on the wire.
        use mc_registry::generated::mob_effects::MOB_EFFECT_ORDER;
        assert_eq!(MOB_EFFECT_ORDER.len(), 40);
        assert_eq!(
            MOB_EFFECT_ORDER[MOB_EFFECT_NIGHT_VISION as usize],
            "minecraft:night_vision",
            "the const indexes the generated table's night_vision slot"
        );
        // the probe's other landmarks ride the same table (the round-120
        // verdict, now generated code)
        assert_eq!(MOB_EFFECT_ORDER[0], "minecraft:speed");
        assert_eq!(MOB_EFFECT_ORDER[28], "minecraft:conduit_power");
        assert_eq!(MOB_EFFECT_ORDER[39], "minecraft:breath_of_the_nautilus");
    }

    /// THE EQUIPMENT PACKET THE STORE MERGES (the Set Equipment lane's
    /// own model walk): the snapshot's slots the merge REPLACES, the
    /// absent slots the partial packet KEEPS (the vanilla client's own
    /// merge — the gear the snapshot never spoke rides), the empty
    /// stack the slot CLEARS (the drop the wire speaks), the MAINHAND
    /// word the registry walk resolves (the iron sword the bare name
    /// speaks — the render's own model walk reads it), the
    /// never-spoken entity the honest None keeps.
    #[test]
    fn the_equipment_packet_the_store_merges() {
        use mc_protocol::play::{EntityEquipment, EquipmentEntry, EquipmentSlot, ItemStack};
        let mut m = WorldModel::new();
        // the never-spoken entity the honest None keeps
        assert_eq!(m.entity_mainhand_word(7), None);
        // the FIRST packet: the mainhand the iron sword arms (the id
        // the registry's wire order names — the packet decode's own
        // byIdOrThrow table the reader's walk shares)
        let sword_id = mc_registry::item_id_by_name("iron_sword").expect("the registry names");
        assert!(m.apply(PlayEvent::EntityEquipped(EntityEquipment {
            entity_id: 7,
            slots: vec![EquipmentEntry {
                slot: EquipmentSlot::Mainhand,
                stack: ItemStack {
                    count: 1,
                    item_id: sword_id,
                    patch: Default::default(),
                },
            }],
        })));
        assert_eq!(m.entity_mainhand_word(7), Some("iron_sword"));
        // the PARTIAL packet: the offhand alone — the mainhand the
        // merge KEEPS (the absent slots the snapshot never spoke ride)
        let shield_id = mc_registry::item_id_by_name("shield").expect("the registry names");
        assert!(m.apply(PlayEvent::EntityEquipped(EntityEquipment {
            entity_id: 7,
            slots: vec![EquipmentEntry {
                slot: EquipmentSlot::Offhand,
                stack: ItemStack {
                    count: 3,
                    item_id: shield_id,
                    patch: Default::default(),
                },
            }],
        })));
        assert_eq!(
            m.entity_mainhand_word(7),
            Some("iron_sword"),
            "the absent slot the merge keeps"
        );
        // the REPLACE: the mainhand the second packet's own word
        // overrides (the slot the snapshot spoke replaces)
        let shovel_id = mc_registry::item_id_by_name("iron_shovel").expect("the registry names");
        assert!(m.apply(PlayEvent::EntityEquipped(EntityEquipment {
            entity_id: 7,
            slots: vec![EquipmentEntry {
                slot: EquipmentSlot::Mainhand,
                stack: ItemStack {
                    count: 1,
                    item_id: shovel_id,
                    patch: Default::default(),
                },
            }],
        })));
        assert_eq!(
            m.entity_mainhand_word(7),
            Some("iron_shovel"),
            "the spoken slot the merge replaces"
        );
        // the EMPTY stack the slot CLEARS (the drop the wire speaks —
        // the OPTIONAL codec's count<=0 the empty ItemStack decodes)
        assert!(m.apply(PlayEvent::EntityEquipped(EntityEquipment {
            entity_id: 7,
            slots: vec![EquipmentEntry {
                slot: EquipmentSlot::Mainhand,
                stack: ItemStack {
                    count: 0,
                    item_id: 0,
                    patch: Default::default(),
                },
            }],
        })));
        assert_eq!(
            m.entity_mainhand_word(7),
            None,
            "the drop the empty stack speaks"
        );
        // the DIMENSION SWAP the store dies with (the same ids — the
        // new level's entities never wear the old one's gear)
        let zombie_id = mc_registry::item_id_by_name("rotten_flesh").expect("the registry names");
        assert!(m.apply(PlayEvent::EntityEquipped(EntityEquipment {
            entity_id: 9,
            slots: vec![EquipmentEntry {
                slot: EquipmentSlot::Mainhand,
                stack: ItemStack {
                    count: 2,
                    item_id: zombie_id,
                    patch: Default::default(),
                },
            }],
        })));
        assert_eq!(m.entity_mainhand_word(9), Some("rotten_flesh"));
        m.apply(PlayEvent::Respawn(mc_protocol::play::SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".to_string(),
            portal_cooldown: 0,
        }));
        assert!(
            m.entity_equipment.is_empty(),
            "the swap the store dies with"
        );
    }

    /// THE OFFHAND WORD (the store's slot-1 read — the LEFT arm the
    /// vanilla client renders): the shield the slot arms, the mainhand
    /// the same packet the OTHER slot keeps (the two words the two
    /// readers split), the offhand's own clear the mainhand never
    /// touches, the never-spoken slot the honest None keeps.
    #[test]
    fn the_offhand_word_the_slot_one_reads() {
        use mc_protocol::play::{EntityEquipment, EquipmentEntry, EquipmentSlot, ItemStack};
        let mut m = WorldModel::new();
        assert_eq!(m.entity_offhand_word(7), None, "the silence the honest empty");
        let sword_id = mc_registry::item_id_by_name("iron_sword").expect("the registry names");
        let shield_id = mc_registry::item_id_by_name("shield").expect("the registry names");
        // the ONE packet, BOTH hands: the slots the merge splits
        assert!(m.apply(PlayEvent::EntityEquipped(EntityEquipment {
            entity_id: 7,
            slots: vec![
                EquipmentEntry {
                    slot: EquipmentSlot::Mainhand,
                    stack: ItemStack { count: 1, item_id: sword_id, patch: Default::default() },
                },
                EquipmentEntry {
                    slot: EquipmentSlot::Offhand,
                    stack: ItemStack { count: 1, item_id: shield_id, patch: Default::default() },
                },
            ],
        })));
        assert_eq!(
            m.entity_mainhand_word(7),
            Some("iron_sword"),
            "the mainhand its own slot reads"
        );
        assert_eq!(
            m.entity_offhand_word(7),
            Some("shield"),
            "the offhand the slot 1 reads"
        );
        // the offhand's own clear: the mainhand rides (the slots the
        // merges split — the drop the packet's own slot speaks)
        assert!(m.apply(PlayEvent::EntityEquipped(EntityEquipment {
            entity_id: 7,
            slots: vec![EquipmentEntry {
                slot: EquipmentSlot::Offhand,
                stack: ItemStack { count: 0, item_id: 0, patch: Default::default() },
            }],
        })));
        assert_eq!(
            m.entity_offhand_word(7),
            None,
            "the drop the empty stack speaks"
        );
        assert_eq!(
            m.entity_mainhand_word(7),
            Some("iron_sword"),
            "the other slot the clear never touches"
        );
    }

    /// THE ARMOR WORDS (the store's FEET/LEGS/CHEST/HEAD ordinals 2..5
    /// — `EquipmentSlot.values()`'s own declaration order): the one
    /// packet the FOUR rows the merge arms (the reader's own walk
    /// order the array speaks), the partial packet the spoken slot
    /// alone REPLACES, the empty stack the slot CLEARS, the
    /// never-spoken rows the honest None keeps, the dimension swap the
    /// store dies with.
    #[test]
    fn the_armor_rows_the_store_merges() {
        use mc_protocol::play::{EntityEquipment, EquipmentEntry, EquipmentSlot, ItemStack};
        let mut m = WorldModel::new();
        // the never-spawned entity the honest bare keeps
        assert_eq!(m.entity_armor_words(7), [None; 4], "the silence the bare");
        // the FIRST packet: the boots + the chestplate arm (the ids the
        // registry's wire order names, the two rows the reader's array
        // carries IN THE STORE'S OWN WALK ORDER — FEET row 0, HEAD row 3)
        let boots_id = mc_registry::item_id_by_name("iron_boots").expect("the registry names");
        let chest_id =
            mc_registry::item_id_by_name("golden_chestplate").expect("the registry names");
        assert!(m.apply(PlayEvent::EntityEquipped(EntityEquipment {
            entity_id: 7,
            slots: vec![
                EquipmentEntry {
                    slot: EquipmentSlot::Feet,
                    stack: ItemStack { count: 1, item_id: boots_id, patch: Default::default() },
                },
                EquipmentEntry {
                    slot: EquipmentSlot::Chest,
                    stack: ItemStack { count: 1, item_id: chest_id, patch: Default::default() },
                },
            ],
        })));
        let words = m.entity_armor_words(7);
        assert_eq!(words[0], Some("iron_boots"), "the FEET row the wire 2 reads");
        assert_eq!(words[1], None, "the LEGS row the silence keeps bare");
        assert_eq!(
            words[2],
            Some("golden_chestplate"),
            "the CHEST row the wire 4 reads"
        );
        assert_eq!(words[3], None, "the HEAD row the silence keeps bare");
        // the REPLACE: the boots the second packet's own word overrides
        let diamond_boots_id =
            mc_registry::item_id_by_name("diamond_boots").expect("the registry names");
        assert!(m.apply(PlayEvent::EntityEquipped(EntityEquipment {
            entity_id: 7,
            slots: vec![EquipmentEntry {
                slot: EquipmentSlot::Feet,
                stack: ItemStack {
                    count: 1,
                    item_id: diamond_boots_id,
                    patch: Default::default(),
                },
            }],
        })));
        let words = m.entity_armor_words(7);
        assert_eq!(
            words[0],
            Some("diamond_boots"),
            "the spoken slot the merge replaces"
        );
        assert_eq!(
            words[2],
            Some("golden_chestplate"),
            "the absent row the partial packet keeps"
        );
        // the EMPTY stack the slot CLEARS (the drop the wire speaks)
        assert!(m.apply(PlayEvent::EntityEquipped(EntityEquipment {
            entity_id: 7,
            slots: vec![EquipmentEntry {
                slot: EquipmentSlot::Chest,
                stack: ItemStack { count: 0, item_id: 0, patch: Default::default() },
            }],
        })));
        let words = m.entity_armor_words(7);
        assert_eq!(words[2], None, "the drop the empty stack speaks");
        assert_eq!(
            words[0],
            Some("diamond_boots"),
            "the other row the clear never touches"
        );
        // the DIMENSION SWAP the store dies with (the same ids — the
        // new level's entities never wear the old one's armor)
        m.apply(PlayEvent::Respawn(mc_protocol::play::SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".to_string(),
            portal_cooldown: 0,
        }));
        assert!(m.entity_equipment.is_empty(), "the swap the store dies with");
        assert_eq!(m.entity_armor_words(7), [None; 4], "the swap the rows bare keep");
    }

    #[test]
    fn the_vitals_lanes_store_the_wires_truth() {
        // the hurt/health arc's own model walk: the Set Health snapshot
        // replaces the store, the Hurt Animation's own-entity filter
        // keeps the LOCAL player's flash (the animateHurt 10-tick arm
        // + the yaw), the Combat Kill lands the death message, and the
        // Respawn's world recreation clears the death (the vanilla
        // client closes the DeathScreen when the new level lands).
        use mc_protocol::HurtAnimation;
        let mut m = WorldModel::new();
        // no info pinned yet: the flash has no owner — the foreign
        // arm treats it as the mobs' mailbox's own lane (the ids the
        // level names are all foreign until the login pins the own)
        assert!(m.apply(PlayEvent::Hurt(HurtAnimation { entity_id: 17, yaw: -45.5 })));
        assert_eq!(m.entity_hurts.get(&17), Some(&10), "the foreign arm");
        m.entity_hurts.clear();
        // the login pins the own id
        assert!(m.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 17,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        })));
        // SET HEALTH: the vitals snapshot lands verbatim; the repeat
        // that carries the same triple is the no-op (the CenterChunk
        // convention)
        let vitals = SetHealth { health: 6.5, food: 18, saturation: 5.0 };
        assert!(m.apply(PlayEvent::Health(vitals)));
        assert_eq!(m.health, Some(vitals));
        assert!(!m.apply(PlayEvent::Health(vitals)), "the repeat is a no-op");
        // HURT ANIMATION: the own id lands on the player's store, the
        // foreign id arms the MOBS' mailbox (the animateHurt window
        // the mob lane's own render speaks — the flash traffic no
        // longer drops, it speaks)
        assert!(m.apply(PlayEvent::Hurt(HurtAnimation { entity_id: 17, yaw: -45.5 })));
        assert_eq!(m.hurt, Some((10, -45.5)), "the animateHurt arm");
        assert!(m.entity_hurts.is_empty(), "the own arm never touches the mobs' lane");
        assert!(m.apply(PlayEvent::Hurt(HurtAnimation { entity_id: 99, yaw: 0.0 })));
        assert_eq!(m.entity_hurts.get(&99), Some(&10), "the mobs' own mailbox arm");
        assert_eq!(m.drain_entity_hurts().get(&99), Some(&10), "the drain ships once");
        assert!(m.entity_hurts.is_empty(), "the mailbox spent");
        assert!(m.drain_entity_hurts().is_empty(), "the drained silence ships nothing");
        // THE MOBS' OWN VOICES (the Entity Sound packets): the registry
        // arm resolves through the synced sound_event table, the id
        // outside it ships UNNAMED (the honest no-voice), the drain
        // ships each voice once, the respawn clears the mailbox
        m.registry.sound_events = vec![
            "minecraft:entity.zombie.ambient".to_string(),
            "minecraft:entity.zombie.hurt".to_string(),
        ];
        assert!(m.apply(PlayEvent::EntitySound(mc_protocol::EntitySoundPacket {
            entity_id: 99,
            sound: mc_protocol::ConsumableSound::Registry(0),
            volume: 1.0,
            pitch: 0.9,
            seed: 7,
        })));
        assert!(m.apply(PlayEvent::EntitySound(mc_protocol::EntitySoundPacket {
            entity_id: 99,
            sound: mc_protocol::ConsumableSound::Registry(41), // outside the table
            volume: 1.0,
            pitch: 1.0,
            seed: 7,
        })));
        assert!(m.apply(PlayEvent::EntitySound(mc_protocol::EntitySoundPacket {
            entity_id: 99,
            sound: mc_protocol::ConsumableSound::Direct {
                id: "minecraft:entity.zombie.death".to_string(),
                range: 16.0,
            },
            volume: 1.0,
            pitch: 1.0,
            seed: 7,
        })));
        assert_eq!(m.entity_sounds.len(), 3, "the three arms ride in wire order");
        assert_eq!(
            m.entity_sounds[0].name.as_deref(),
            Some("minecraft:entity.zombie.ambient"),
            "the registry arm resolved"
        );
        assert_eq!(m.entity_sounds[1].name, None, "the unknown id ships unnamed");
        assert_eq!(
            m.entity_sounds[2].name.as_deref(),
            Some("minecraft:entity.zombie.death"),
            "the direct lane's own location"
        );
        assert_eq!(m.entity_sounds[0].volume, 1.0);
        assert_eq!(m.entity_sounds[0].pitch, 0.9);
        let drained = m.drain_entity_sounds();
        assert_eq!(drained.len(), 3, "the drain ships the whole mailbox");
        assert!(m.entity_sounds.is_empty(), "the mailbox spent");
        assert!(m.drain_entity_sounds().is_empty(), "the drained silence ships nothing");
        // THE WORLD'S OWN VOICES (the Sound packet's global lane): the
        // arms ride in wire order, the registry arm resolves, the
        // unknown id ships UNNAMED, the direct lane's own location
        // speaks, the spot rides verbatim, the drain empties the box
        m.registry.sound_events = vec!["minecraft:block.wooden_door.open".to_string()];
        assert!(m.apply(PlayEvent::WorldSound(mc_protocol::WorldSoundPacket {
            source: 4,
            sound: mc_protocol::ConsumableSound::Registry(0),
            x: 96.0,
            y: 65.0,
            z: -128.0,
            volume: 1.0,
            pitch: 0.9,
            seed: 7,
        })));
        assert!(m.apply(PlayEvent::WorldSound(mc_protocol::WorldSoundPacket {
            source: 0,
            sound: mc_protocol::ConsumableSound::Registry(41), // outside the table
            x: 0.0,
            y: 64.0,
            z: 0.0,
            volume: 0.5,
            pitch: 1.0,
            seed: 7,
        })));
        assert!(m.apply(PlayEvent::WorldSound(mc_protocol::WorldSoundPacket {
            source: 3,
            sound: mc_protocol::ConsumableSound::Direct {
                id: "minecraft:ambient.weather.thunder".to_string(),
                range: 16.0,
            },
            x: -8.5,
            y: 90.125,
            z: 12.0,
            volume: 2.0,
            pitch: 1.0,
            seed: 7,
        })));
        assert_eq!(m.world_sounds.len(), 3, "the three arms ride in wire order");
        assert_eq!(
            m.world_sounds[0].name.as_deref(),
            Some("minecraft:block.wooden_door.open"),
            "the registry arm resolved"
        );
        assert_eq!(m.world_sounds[1].name, None, "the unknown id ships unnamed");
        assert_eq!(
            m.world_sounds[2].name.as_deref(),
            Some("minecraft:ambient.weather.thunder"),
            "the direct lane's own location"
        );
        assert_eq!((m.world_sounds[0].x, m.world_sounds[0].y, m.world_sounds[0].z),
            (96.0, 65.0, -128.0), "the spot the packet carried verbatim");
        assert_eq!(m.world_sounds[0].pitch, 0.9);
        let drained = m.drain_world_sounds();
        assert_eq!(drained.len(), 3, "the drain ships the whole mailbox");
        assert!(m.world_sounds.is_empty(), "the mailbox spent");
        assert!(m.drain_world_sounds().is_empty(), "the drained silence ships nothing");
        // COMBAT KILL: the message lands on the own id
        assert!(m.apply(PlayEvent::CombatKill(mc_protocol::PlayerCombatKill {
            player_id: 17,
            message: "Player fell from a high place".to_string(),
        })));
        assert_eq!(m.death_message.as_deref(), Some("Player fell from a high place"));
        // the RESPAWN recreates the level: the death screen's message
        // dies with it
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        })));
        assert!(m.death_message.is_none(), "the respawn closed the death");
        assert!(m.hurt.is_none(), "the flash dies with the level");
        // the mobs' mailbox dies with the level too (the arms name the
        // OLD population's ids — the new level never reads them)
        assert!(m.apply(PlayEvent::Hurt(HurtAnimation { entity_id: 99, yaw: 0.0 })));
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_end".to_string(),
            portal_cooldown: 0,
        })));
        assert!(m.entity_hurts.is_empty(), "the mailbox dies with the level");
        assert!(m.entity_sounds.is_empty(), "the voices die with the level");
        // the vitals ride the player (NOT cleared — the health the
        // server speaks crosses the dimension like the effects do)
        assert_eq!(m.health, Some(vitals), "the vitals ride the player");
    }

    #[test]
    fn the_level_event_the_growth_word_the_mailbox_arms() {
        // THE LEVEL EVENT'S OWN GATE (the vocabulary the render lane
        // speaks): the GROWTH_EFFECT word the mailbox arms — the
        // anchor + the data verbatim — every OTHER id the honest
        // `false` keeps (the event touched nothing this model speaks),
        // the drain ships the arms exactly once, and the Respawn
        // recreates the level the mailbox dies with it.
        let mut m = WorldModel::new();
        // the growth word arms (the bone-meal walk's own event)
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::GROWTH_EFFECT,
            x: 12,
            y: 64,
            z: -7,
            data: 15,
            global: false,
        })));
        assert_eq!(m.growth_events.len(), 1, "the arm rides");
        assert_eq!(m.growth_events[0].x, 12);
        assert_eq!(m.growth_events[0].y, 64);
        assert_eq!(m.growth_events[0].z, -7);
        assert_eq!(m.growth_events[0].data, 15, "the data word verbatim");
        // a SECOND growth word rides in wire order (the bone meal the
        // server's own broadcast pair speaks)
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::GROWTH_EFFECT,
            x: 3,
            y: 70,
            z: 9,
            data: 15,
            global: false,
        })));
        assert_eq!(m.growth_events.len(), 2, "the second arm rides after");
        assert_eq!(m.growth_events[1].x, 3);
        // an unknown effect id: the honest refusal — no arm, no state
        // touched (the mob-spawner flames, the wither's broadcast, the
        // rest of the constants class the vocabulary never speaks)
        assert!(!m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: 1030,
            x: 0,
            y: 0,
            z: 0,
            data: 0,
            global: false,
        })));
        assert_eq!(m.growth_events.len(), 2, "the foreign word never arms");
        // a foreign GLOBAL word the same refusal keeps
        assert!(!m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: 1032,
            x: 0,
            y: 0,
            z: 0,
            data: 0,
            global: true,
        })));
        assert_eq!(m.growth_events.len(), 2);
        // the drain ships the whole mailbox exactly once
        let drained = m.drain_growth_events();
        assert_eq!(drained.len(), 2, "the drain ships the whole mailbox");
        assert_eq!(drained[0].data, 15);
        assert!(m.growth_events.is_empty(), "the mailbox spent");
        assert!(m.drain_growth_events().is_empty(), "the drained silence ships nothing");
        // a fresh word after the drain arms again
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::GROWTH_EFFECT,
            x: 1,
            y: 2,
            z: 3,
            data: 15,
            global: false,
        })));
        assert_eq!(m.growth_events.len(), 1, "the fresh word re-arms");
        // the Respawn recreates the level: the mailbox dies with it
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        })));
        assert!(m.growth_events.is_empty(), "the mailbox dies with the level");
    }

    #[test]
    fn the_level_event_the_break_word_the_state_walk_arms() {
        // THE BLOCK-BREAK EFFECT'S OWN ARM (javap the case offset 1561
        // — the positioned route the GLOBAL switch never banks): the
        // data word the GLOBAL STATE id carries (`Block.stateById` the
        // registry ranges the resolution speaks), the SOUND the `isAir`
        // gate refuses, the BURST the palette kind banks (the unbanked
        // kinds the honest absence keeps), the drain ships the arms
        // exactly once, and the Respawn recreates the level the
        // mailbox dies with it.
        let mut m = WorldModel::new();
        // the STONE state (the registry range's own head — the state id
        // 1 the `Block.stateById` walk answers): the voice + the burst
        // both speak
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::DESTROY_BLOCK_EFFECT,
            x: -40,
            y: 12,
            z: 77,
            data: 1,
            global: false,
        })));
        assert_eq!(m.break_events.len(), 1, "the arm rides");
        assert_eq!(
            m.break_events[0].sound_name,
            Some("stone"),
            "the stripped name the sound table speaks"
        );
        assert_eq!(
            m.break_events[0].burst_kind,
            Some(crate::world::STONE),
            "the palette kind the burst tile reads"
        );
        // the GLOBAL route the arm refuses (the `globalLevelEvent`
        // switch the 1023/1028/1038 trio the band alone carries — the
        // 2001 id the positioned lane's own word)
        assert!(!m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::DESTROY_BLOCK_EFFECT,
            x: 0,
            y: 0,
            z: 0,
            data: 1,
            global: true,
        })));
        assert_eq!(m.break_events.len(), 1, "the global word never arms");
        // the AIR state (the state id 0 the table's own head): the
        // voice the isAir gate keeps, the kind the palette's own head
        // answers (the render lane's tile verdict the no-op keeps)
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::DESTROY_BLOCK_EFFECT,
            x: 1,
            y: 2,
            z: 3,
            data: 0,
            global: false,
        })));
        assert_eq!(m.break_events.len(), 2);
        assert_eq!(
            m.break_events[1].sound_name,
            None,
            "the air state the voice never plays"
        );
        assert_eq!(m.break_events[1].burst_kind, Some(crate::world::AIR));
        // the OUT-OF-RANGE data the `Blocks.AIR.defaultBlockState()`
        // fallback answers (the byId null the jar's own word keeps):
        // both lanes the silent walk keeps
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::DESTROY_BLOCK_EFFECT,
            x: 0,
            y: 0,
            z: 0,
            data: 999_999,
            global: false,
        })));
        assert_eq!(m.break_events.len(), 3);
        assert_eq!(m.break_events[2].sound_name, None);
        assert_eq!(m.break_events[2].burst_kind, None);
        // the UNBANKED kind (the anvil's range head the palette never
        // banks): the sound the honest ride keeps, the burst the tile's
        // own absence refuses
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::DESTROY_BLOCK_EFFECT,
            x: 4,
            y: 5,
            z: 6,
            data: 11195,
            global: false,
        })));
        assert_eq!(m.break_events.len(), 4);
        assert_eq!(m.break_events[3].sound_name, Some("anvil"));
        assert_eq!(
            m.break_events[3].burst_kind,
            None,
            "the unbanked kind the burst never finds"
        );
        // the drain ships the whole mailbox exactly once
        let drained = m.drain_break_events();
        assert_eq!(drained.len(), 4, "the drain ships the whole mailbox");
        assert!(m.break_events.is_empty(), "the mailbox spent");
        assert!(m.drain_break_events().is_empty(), "the drained silence ships nothing");
        // the Respawn recreates the level: the mailbox dies with it
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::DESTROY_BLOCK_EFFECT,
            x: 7,
            y: 8,
            z: 9,
            data: 1,
            global: false,
        })));
        assert_eq!(m.break_events.len(), 1, "the fresh word re-arms");
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        })));
        assert!(m.break_events.is_empty(), "the mailbox dies with the level");
    }

    #[test]
    fn the_level_event_the_white_smoke_word_the_facing_rides() {
        // THE WHITE-SMOKE EFFECT'S OWN ARM (javap the case offset 919
        // — the positioned route the GLOBAL switch never banks): the
        // SAME `shootParticles` walk the 2000 word wears (the
        // WHITE_SMOKE type the one arg's delta), the data word the
        // DIRECTION id carries (`Direction.from3DDataValue` the render
        // lane's own read), the anchor + the raw id the mailbox keeps
        // verbatim, the drain ships the arms exactly once, and the
        // Respawn recreates the level the mailbox dies with it.
        let mut m = WorldModel::new();
        // the EAST-facing crafter's own word (get3DDataValue 5 the
        // eject's broadcast carries): the arm rides the raw id
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SHOOT_WHITE_SMOKE_EFFECT,
            x: -40,
            y: 12,
            z: 77,
            data: 5,
            global: false,
        })));
        assert_eq!(m.white_smoke_events.len(), 1, "the arm rides");
        assert_eq!(m.white_smoke_events[0].x, -40);
        assert_eq!(m.white_smoke_events[0].y, 12);
        assert_eq!(m.white_smoke_events[0].z, 77);
        assert_eq!(
            m.white_smoke_events[0].data, 5,
            "the raw direction id the render lane folds"
        );
        // the GLOBAL route the arm refuses (the `globalLevelEvent`
        // switch the 1023/1028/1038 trio the band alone carries — the
        // 2010 id the positioned lane's own word)
        assert!(!m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SHOOT_WHITE_SMOKE_EFFECT,
            x: 0,
            y: 0,
            z: 0,
            data: 5,
            global: true,
        })));
        assert_eq!(m.white_smoke_events.len(), 1, "the global word never arms");
        // the second word arms beside the first (the mailbox keeps the
        // wire order — the DOWN-facing puff the data-0 word carries)
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SHOOT_WHITE_SMOKE_EFFECT,
            x: 1,
            y: 2,
            z: 3,
            data: 0,
            global: false,
        })));
        assert_eq!(m.white_smoke_events.len(), 2);
        assert_eq!(m.white_smoke_events[1].data, 0);
        // the drain ships the whole mailbox exactly once
        let drained = m.drain_white_smoke_events();
        assert_eq!(drained.len(), 2, "the drain ships the whole mailbox");
        assert!(m.white_smoke_events.is_empty(), "the mailbox spent");
        assert!(
            m.drain_white_smoke_events().is_empty(),
            "the drained silence ships nothing"
        );
        // the Respawn recreates the level: the mailbox dies with it
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SHOOT_WHITE_SMOKE_EFFECT,
            x: 7,
            y: 8,
            z: 9,
            data: 3,
            global: false,
        })));
        assert_eq!(m.white_smoke_events.len(), 1, "the fresh word re-arms");
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        })));
        assert!(m.white_smoke_events.is_empty(), "the mailbox dies with the level");
    }

    #[test]
    fn the_level_event_the_smoke_word_the_facing_rides() {
        // THE SHOOT-SMOKE EFFECT'S OWN ARM (javap the case offset 933
        // — the positioned route the GLOBAL switch never banks): the
        // data word the DIRECTION id carries (`Direction
        // .from3DDataValue` the render lane's own read), the anchor +
        // the raw id the mailbox keeps verbatim (the facing fold the
        // render lane's own law speaks), the drain ships the arms
        // exactly once, and the Respawn recreates the level the
        // mailbox dies with it.
        let mut m = WorldModel::new();
        // the EAST-facing dispenser's own word (get3DDataValue 5 the
        // `playDefaultAnimation` broadcast carries): the arm rides the
        // raw id
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SHOOT_SMOKE_EFFECT,
            x: -40,
            y: 12,
            z: 77,
            data: 5,
            global: false,
        })));
        assert_eq!(m.shoot_smoke_events.len(), 1, "the arm rides");
        assert_eq!(m.shoot_smoke_events[0].x, -40);
        assert_eq!(m.shoot_smoke_events[0].y, 12);
        assert_eq!(m.shoot_smoke_events[0].z, 77);
        assert_eq!(
            m.shoot_smoke_events[0].data, 5,
            "the raw direction id the render lane folds"
        );
        // the GLOBAL route the arm refuses (the `globalLevelEvent`
        // switch the 1023/1028/1038 trio the band alone carries — the
        // 2000 id the positioned lane's own word)
        assert!(!m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SHOOT_SMOKE_EFFECT,
            x: 0,
            y: 0,
            z: 0,
            data: 5,
            global: true,
        })));
        assert_eq!(m.shoot_smoke_events.len(), 1, "the global word never arms");
        // the second word arms beside the first (the mailbox keeps the
        // wire order — the DOWN-facing puff the data-0 word carries)
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SHOOT_SMOKE_EFFECT,
            x: 1,
            y: 2,
            z: 3,
            data: 0,
            global: false,
        })));
        assert_eq!(m.shoot_smoke_events.len(), 2);
        assert_eq!(m.shoot_smoke_events[1].data, 0);
        // the drain ships the whole mailbox exactly once
        let drained = m.drain_shoot_smoke_events();
        assert_eq!(drained.len(), 2, "the drain ships the whole mailbox");
        assert!(m.shoot_smoke_events.is_empty(), "the mailbox spent");
        assert!(
            m.drain_shoot_smoke_events().is_empty(),
            "the drained silence ships nothing"
        );
        // the Respawn recreates the level: the mailbox dies with it
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SHOOT_SMOKE_EFFECT,
            x: 7,
            y: 8,
            z: 9,
            data: 3,
            global: false,
        })));
        assert_eq!(m.shoot_smoke_events.len(), 1, "the fresh word re-arms");
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        })));
        assert!(
            m.shoot_smoke_events.is_empty(),
            "the mailbox dies with the level"
        );
    }

    #[test]
    fn the_level_event_the_spawn_word_the_puff_rides() {
        // THE MOB-SPAWN CLOUD'S OWN ARM (javap the case offset 1689 —
        // the positioned route the GLOBAL switch never banks): the
        // anchor the mailbox keeps verbatim, the data word the case's
        // own body NEVER reads (no `iload_3` the arm's bytecode speaks
        // — the arm the anchor alone carries)
        let mut m = crate::world_model::WorldModel::new();
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::MOB_SPAWN_EFFECT,
            x: -40,
            y: 12,
            z: 77,
            data: 0,
            global: false,
        })));
        assert_eq!(m.mob_spawn_events.len(), 1, "the arm rides");
        assert_eq!(m.mob_spawn_events[0].x, -40);
        assert_eq!(m.mob_spawn_events[0].y, 12);
        assert_eq!(m.mob_spawn_events[0].z, 77);
        // the GLOBAL route the arm refuses (the `globalLevelEvent`
        // switch the 1023/1028/1038 trio the band alone carries — the
        // 2004 id the positioned lane's own word)
        assert!(!m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::MOB_SPAWN_EFFECT,
            x: 0,
            y: 0,
            z: 0,
            data: 0,
            global: true,
        })));
        assert_eq!(m.mob_spawn_events.len(), 1, "the global word never arms");
        // the second word arms beside the first (the mailbox keeps the
        // wire order — the data word the walk never reads, any value
        // the same anchor-only arm speaks)
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::MOB_SPAWN_EFFECT,
            x: 1,
            y: 2,
            z: 3,
            data: 9999,
            global: false,
        })));
        assert_eq!(m.mob_spawn_events.len(), 2);
        assert_eq!(m.mob_spawn_events[1].x, 1);
        // the drain ships the whole mailbox exactly once
        let drained = m.drain_mob_spawn_events();
        assert_eq!(drained.len(), 2, "the drain ships the whole mailbox");
        assert!(m.mob_spawn_events.is_empty(), "the mailbox spent");
        assert!(
            m.drain_mob_spawn_events().is_empty(),
            "the drained silence ships nothing"
        );
        // the Respawn recreates the level: the mailbox dies with it
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::MOB_SPAWN_EFFECT,
            x: 7,
            y: 8,
            z: 9,
            data: 0,
            global: false,
        })));
        assert_eq!(m.mob_spawn_events.len(), 1, "the fresh word re-arms");
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        })));
        assert!(
            m.mob_spawn_events.is_empty(),
            "the mailbox dies with the level"
        );
    }

    #[test]
    fn the_level_event_the_splash_word_the_color_rides() {
        // THE SPLASH-POTION EFFECT'S OWN ARM (javap the case offset
        // 1199 — the positioned route the GLOBAL switch never banks,
        // BOTH ids the one arm wears): the data word the raw color
        // carries (the RGB decode the render lane's own law speaks),
        // the instant flag the type's own split banks (the `2007` word
        // the `if_icmpne` branch answers), the anchor verbatim, the
        // drain ships the arms exactly once, and the Respawn recreates
        // the level the mailbox dies with it.
        let mut m = WorldModel::new();
        // the regular splash's own word (the ARGB color the server's
        // option carries — the red the >>16 band, the green the >>8,
        // the blue the >>0): the arm rides the RAW word
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SPLASH_POTION_EFFECT,
            x: -12,
            y: 34,
            z: -56,
            data: 0x37FA_19D5,
            global: false,
        })));
        assert_eq!(m.splash_events.len(), 1, "the arm rides");
        assert_eq!(m.splash_events[0].x, -12);
        assert_eq!(m.splash_events[0].y, 34);
        assert_eq!(m.splash_events[0].z, -56);
        assert_eq!(
            m.splash_events[0].data, 0x37FA_19D5,
            "the raw color word the render lane folds"
        );
        assert!(
            !m.splash_events[0].instant,
            "the regular splash the type split keeps"
        );
        // the instant word arms beside it — the SAME arm the one flag
        // splits (the `InstantEffect` type the branch answers)
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::INSTANT_SPLASH_EFFECT,
            x: 1,
            y: 2,
            z: 3,
            data: 0x00FF_8800,
            global: false,
        })));
        assert_eq!(m.splash_events.len(), 2);
        assert!(m.splash_events[1].instant, "the instant word the flag banks");
        // the GLOBAL route the arm refuses (the `globalLevelEvent`
        // switch the 1023/1028/1038 trio the band alone carries —
        // neither id the global route's own word)
        assert!(!m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SPLASH_POTION_EFFECT,
            x: 0,
            y: 0,
            z: 0,
            data: 0xFF,
            global: true,
        })));
        assert!(
            !m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
                effect: mc_protocol::INSTANT_SPLASH_EFFECT,
                x: 0,
                y: 0,
                z: 0,
                data: 0xFF,
                global: true,
            }))
        );
        assert_eq!(m.splash_events.len(), 2, "the global words never arm");
        // the drain ships the whole mailbox exactly once
        let drained = m.drain_splash_events();
        assert_eq!(drained.len(), 2, "the drain ships the whole mailbox");
        assert!(m.splash_events.is_empty(), "the mailbox spent");
        assert!(
            m.drain_splash_events().is_empty(),
            "the drained silence ships nothing"
        );
        // the Respawn recreates the level: the mailbox dies with it
        assert!(m.apply(PlayEvent::LevelEvent(mc_protocol::LevelEventPacket {
            effect: mc_protocol::SPLASH_POTION_EFFECT,
            x: 7,
            y: 8,
            z: 9,
            data: 0x11_22_33,
            global: false,
        })));
        assert_eq!(m.splash_events.len(), 1, "the fresh word re-arms");
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        })));
        assert!(
            m.splash_events.is_empty(),
            "the mailbox dies with the level"
        );
    }

    #[test]
    fn the_local_player_effects_drive_the_night_vision_factor() {
        // the effect state's CONSUMER walk: the model's own-entity
        // filter keeps the LOCAL player's effects (the lightmap's
        // night-vision transport), the foreign traffic is dropped, the
        // upsert replaces, and Respawn does NOT clear the store (the
        // effects ride the player across the level recreation — the
        // vanilla client keeps LocalPlayer effects through a dimension
        // change, unlike the columns and the entity registry).
        let mut m = WorldModel::new();
        // no info pinned yet: the own-entity filter cannot match, the
        // effect traffic is honestly dropped (no phantom player)
        assert!(!m.apply(PlayEvent::MobEffectApplied(MobEffectUpdate {
            entity_id: 17,
            effect_id: MOB_EFFECT_NIGHT_VISION,
            amplifier: 0,
            duration: 600,
            flags: 0b0110,
        })));
        assert_eq!(m.night_vision_factor(), 0.0, "no info, no effect: the floor");
        // the login pins the own id — the own-entity traffic lands and
        // the factor lifts to the nightVisionScale plateau
        assert!(m.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 17,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        })));
        assert!(m.apply(PlayEvent::MobEffectApplied(MobEffectUpdate {
            entity_id: 17,
            effect_id: MOB_EFFECT_NIGHT_VISION,
            amplifier: 2,
            duration: -1,
            flags: 0b0110,
        })));
        assert_eq!(m.night_vision_factor(), 1.0, "the nightVisionScale plateau");
        let st = m.mob_effect(MOB_EFFECT_NIGHT_VISION).unwrap();
        assert_eq!(st.amplifier, 2, "the state retained verbatim");
        assert_eq!(st.duration, -1, "the infinite sentinel rides verbatim");
        assert_eq!(st.flags, 0b0110, "the visible|show-icon bits");
        // a FOREIGN entity's effect traffic never touches the store
        assert!(!m.apply(PlayEvent::MobEffectApplied(MobEffectUpdate {
            entity_id: 99,
            effect_id: 28, // conduit_power — not ours
            amplifier: 0,
            duration: 200,
            flags: 0,
        })));
        assert!(m.mob_effect(28).is_none(), "the foreign packet stored nothing");
        assert_eq!(m.night_vision_factor(), 1.0, "and changed nothing");
        // the upsert REPLACES the stored state
        assert!(m.apply(PlayEvent::MobEffectApplied(MobEffectUpdate {
            entity_id: 17,
            effect_id: MOB_EFFECT_NIGHT_VISION,
            amplifier: 0,
            duration: 100,
            flags: 0,
        })));
        assert_eq!(m.mob_effect(MOB_EFFECT_NIGHT_VISION).unwrap().amplifier, 0, "replaced");
        // Respawn keeps the effects (the player carries them across)
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        })));
        assert_eq!(m.night_vision_factor(), 1.0, "effects survive the dimension change");
        // the removal drops the entry (the floor returns); a second
        // removal is an honest no-op
        assert!(m.apply(PlayEvent::MobEffectRemoved(MobEffectRemove {
            entity_id: 17,
            effect_id: MOB_EFFECT_NIGHT_VISION,
        })));
        assert_eq!(m.night_vision_factor(), 0.0, "the floor returns");
        assert!(!m.apply(PlayEvent::MobEffectRemoved(MobEffectRemove {
            entity_id: 17,
            effect_id: MOB_EFFECT_NIGHT_VISION,
        })));
    }

    /// The effect state's NAME lane — the debug surface riding the
    /// generated mob-effect table into the model (the candidate the
    /// effect rounds queued): the listing resolves each stored id
    /// through `mc_registry::mob_effect_name`, the known ids name
    /// themselves, an id PAST the table (a newer server's registry)
    /// names None while the state still rides, the own-entity filter
    /// keeps the listing LOCAL, and the removal drops the pair.
    #[test]
    fn the_effect_debug_surface_resolves_the_generated_names() {
        let mut m = WorldModel::new();
        assert!(m.active_effects_debug().is_empty(), "no effects, no listing");
        // the login pins the own id — the own-entity traffic lands
        assert!(m.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 7,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        })));
        // two of ours: a known id and one PAST the 26.2 table (the
        // wire's VarInt lane accepts any id — a newer server's
        // registry can carry effects this jar's boot never had)
        assert!(m.apply(PlayEvent::MobEffectApplied(MobEffectUpdate {
            entity_id: 7,
            effect_id: MOB_EFFECT_NIGHT_VISION,
            amplifier: 0,
            duration: 600,
            flags: 0b0110,
        })));
        assert!(m.apply(PlayEvent::MobEffectApplied(MobEffectUpdate {
            entity_id: 7,
            effect_id: 40,
            amplifier: 1,
            duration: -1,
            flags: 0,
        })));
        // a FOREIGN entity's effect never surfaces in the listing
        assert!(!m.apply(PlayEvent::MobEffectApplied(MobEffectUpdate {
            entity_id: 99,
            effect_id: 1, // slowness — not ours
            amplifier: 0,
            duration: 200,
            flags: 0,
        })));
        // the listing's order is the store's (the map's) — sort for
        // the pin (the HUD/debug-screen rounds own any sort)
        let mut listing = m.active_effects_debug();
        listing.sort_by_key(|(id, _, _)| *id);
        assert_eq!(listing.len(), 2, "the foreign packet's effect never surfaced");
        assert_eq!(listing[0].0, MOB_EFFECT_NIGHT_VISION);
        assert_eq!(listing[0].1, Some("minecraft:night_vision"), "the generated table's slot 15");
        assert_eq!(listing[0].2.amplifier, 0, "the state rides the triple verbatim");
        assert_eq!(listing[0].2.duration, 600);
        assert_eq!(listing[1].0, 40);
        assert_eq!(listing[1].1, None, "the honest unknown — never a guessed name");
        assert_eq!(listing[1].2.amplifier, 1, "the unresolvable id's STATE still rides");
        assert_eq!(listing[1].2.duration, -1);
        // the removal drops the pair from the listing
        assert!(m.apply(PlayEvent::MobEffectRemoved(MobEffectRemove {
            entity_id: 7,
            effect_id: 40,
        })));
        let listing = m.active_effects_debug();
        assert_eq!(listing.len(), 1);
        assert_eq!(listing[0].1, Some("minecraft:night_vision"));
    }

    /// The player-inventory surface — the first consumer the container
    /// trilogy's window-0 traffic has been waiting for. The audited
    /// consumer split drives the store: window 0 IS the player's own
    /// `inventoryMenu` (the full snapshot REPLACES, the single-slot
    /// delta upserts in place, the counter advances with whichever
    /// packet spoke last), other windows ride the open-container lane
    /// and have no model surface yet. The growth policy (a slot past
    /// the current end pads with empties) and the negative-slot ignore
    /// are the retention line; the hotbar helpers carry the audit's
    /// quirk (the helper answers true for the shield slot 45).
    #[test]
    fn the_player_inventory_store_feeds_from_window_zero() {
        use mc_protocol::play::data_component_types as t;
        use mc_protocol::play::{clientbound, parse_set_container_content, parse_set_container_slot};
        let mut m = WorldModel::new();
        assert_eq!(m.inventory().slots.len(), 0, "empty until the wire speaks");
        // the full window-0 snapshot: the patched stack, the CLEARED
        // slot, the patch-less stack, and the cursor stack
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 0); // window 0: the player's own inventory
        write_varint(&mut p, 7); // state id
        write_varint(&mut p, 3); // the collection count
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
        write_varint(&mut p, 2); // the cursor: a small stack
        write_varint(&mut p, 10);
        write_varint(&mut p, 0);
        write_varint(&mut p, 0);
        let c = parse_set_container_content(&p).unwrap();
        assert!(m.apply(PlayEvent::ContainerFilled(c)));
        assert_eq!(m.inventory().state_id, 7);
        assert_eq!(m.inventory().slots.len(), 3);
        assert_eq!(m.inventory().carried.count, 2, "the cursor stack rides");
        assert_eq!(m.inventory_slot(0).unwrap().count, 12);
        assert!(m.inventory_slot(1).unwrap().count <= 0, "the cleared slot");
        assert_eq!(m.inventory_slot(2).unwrap().item_id, 33);
        // the single-slot delta upserts in place and advances the
        // counter; slot 44 (a USE_ROW slot) grows the store past the
        // snapshot's three slots — the retention line
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 8);
        p.extend_from_slice(&44i16.to_be_bytes()); // a hotbar slot
        write_varint(&mut p, 5);
        write_varint(&mut p, 900);
        write_varint(&mut p, 0);
        write_varint(&mut p, 0);
        let s = parse_set_container_slot(&p).unwrap();
        assert!(m.apply(PlayEvent::ContainerSlotSet(s)));
        assert_eq!(m.inventory().state_id, 8);
        assert_eq!(m.inventory().slots.len(), 45, "grown to address slot 44");
        assert_eq!(m.inventory_slot(44).unwrap().item_id, 900);
        assert!(m.inventory_slot(36).unwrap().count <= 0, "the growth padding");
        // a negative slot cannot index anything: the counter advances,
        // the slots do not move
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 9);
        p.extend_from_slice(&(-2i16).to_be_bytes());
        write_varint(&mut p, 0);
        let s = parse_set_container_slot(&p).unwrap();
        assert!(m.apply(PlayEvent::ContainerSlotSet(s)));
        assert_eq!(m.inventory().state_id, 9);
        assert_eq!(m.inventory().slots.len(), 45);
        assert_eq!(m.inventory_slot(0).unwrap().count, 12, "untouched");
        // the other windows ride the OPEN-container lane (matched by id
        // against containerMenu): no model surface yet — the honest skip
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 200); // a chest's window
        write_varint(&mut p, 1);
        write_varint(&mut p, 0);
        write_varint(&mut p, 0); // the carried item: empty
        let c = parse_set_container_content(&p).unwrap();
        assert!(!m.apply(PlayEvent::ContainerFilled(c)));
        assert_eq!(m.inventory().state_id, 9, "the chest's window wrote nothing");
        // the audited hotbar facts: the SLICE is the nine USE_ROW slots,
        // the SHIELD slot rides right past it, and the HELPER answers
        // true for 45 too (the javap quirk — both facts kept separate)
        assert_eq!(USE_ROW_SLOT_END - USE_ROW_SLOT_START, 9);
        assert_eq!(SHIELD_SLOT, USE_ROW_SLOT_END);
        assert!(is_hotbar_slot(36) && is_hotbar_slot(44));
        assert!(is_hotbar_slot(45), "the helper's audited quirk");
        assert!(!is_hotbar_slot(35) && !is_hotbar_slot(46));
    }

    /// The hotbar's selection — the Set Held Slot packet's audited
    /// gate: the SELECTION-relative 0..9 moves the store's selected
    /// slot (the window slot it names is `USE_ROW_SLOT_START +
    /// selected`), a repeat naming the held slot is the no-op, the
    /// gate DROPS the window numbering (36 — a USE_ROW slot in the
    /// InventoryMenu numbering but not a selection index) and the
    /// negatives before they reach the inventory, and the container
    /// snapshot does NOT reset the selection (vanilla's
    /// initializeContents sets slots/carried/stateId only).
    #[test]
    fn the_held_slot_moves_the_selection_through_the_gate() {
        use mc_protocol::play::{clientbound, parse_set_container_content, parse_set_held_slot};
        let mut m = WorldModel::new();
        assert_eq!(m.selected_slot(), 0, "vanilla's initial selection");
        // the selection moves
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        write_varint(&mut p, 4);
        let s = parse_set_held_slot(&p).unwrap();
        assert!(m.apply(PlayEvent::HeldSlotSet(s)));
        assert_eq!(m.selected_slot(), 4);
        // a repeat naming the held slot is the no-op
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        write_varint(&mut p, 4);
        let s = parse_set_held_slot(&p).unwrap();
        assert!(!m.apply(PlayEvent::HeldSlotSet(s)));
        // the GATE drops the window numbering: 36 is a USE_ROW slot in
        // the InventoryMenu numbering but NOT a selection-relative
        // hotbar slot — the two isHotbarSlot helpers are different
        // truths
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        write_varint(&mut p, 36);
        let s = parse_set_held_slot(&p).unwrap();
        assert!(!m.apply(PlayEvent::HeldSlotSet(s)));
        assert_eq!(m.selected_slot(), 4, "the gate dropped 36");
        // and the negative
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        write_varint(&mut p, -2);
        let s = parse_set_held_slot(&p).unwrap();
        assert!(!m.apply(PlayEvent::HeldSlotSet(s)));
        assert_eq!(m.selected_slot(), 4);
        // the window-0 snapshot REPLACES slots/carried/stateId and
        // leaves the selection alone (the initializeContents field
        // list, javap-pinned)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 0); // window 0
        write_varint(&mut p, 3); // state id
        write_varint(&mut p, 1); // one slot
        write_varint(&mut p, 7); // stack count
        write_varint(&mut p, 123); // item registry id
        write_varint(&mut p, 0); // patch: none added
        write_varint(&mut p, 0); // patch: none removed
        write_varint(&mut p, 0); // the carried item: empty
        let c = parse_set_container_content(&p).unwrap();
        assert!(m.apply(PlayEvent::ContainerFilled(c)));
        assert_eq!(m.selected_slot(), 4, "the snapshot left the selection");
        assert_eq!(m.inventory().slots.len(), 1);
        assert_eq!(m.inventory_slot(0).unwrap().item_id, 123);
        // the window slot the selection names rides the USE_ROW mapping
        assert_eq!(USE_ROW_SLOT_START + m.selected_slot(), 40);
    }

    /// The player's OWN hotbar selection (the number keys' local
    /// prediction): the SELECTION-relative 0..8 gate mirrors the Set
    /// Held Slot packet's, the repeat naming the held slot is the
    /// no-op, and the move is the client's own belief — the hand's
    /// projection reads the moved slot (`USE_ROW_SLOT_START +
    /// selected`).
    #[test]
    fn the_local_selection_moves_through_its_gate() {
        let mut m = WorldModel::new();
        assert_eq!(m.selected_slot(), 0, "vanilla's initial selection");
        // the move is the client's own belief
        assert!(m.select_hotbar(7));
        assert_eq!(m.selected_slot(), 7);
        // the repeat naming the held slot is the no-op
        assert!(!m.select_hotbar(7));
        assert_eq!(m.selected_slot(), 7);
        // the gate drops the out-of-domain (the window numbering's 36
        // is NOT a selection-relative slot — the same truth the Set
        // Held Slot gate pins)
        assert!(!m.select_hotbar(36));
        assert!(!m.select_hotbar(9));
        assert_eq!(m.selected_slot(), 7, "the gate dropped them all");
        // and the move lands on the hand's projection slot
        assert_eq!(USE_ROW_SLOT_START + m.selected_slot(), 43);
    }

    /// The open-container store's life cycle (the container GUI's
    /// consumer, the audited `containerMenu` lane): the Open Screen
    /// packet births it, the MATCHING window's container traffic feeds
    /// it (the snapshot REPLACES, the slot delta UPSERTS through the
    /// shared [`store_slot_write`]), a MISMATCHED window is dropped
    /// (the fresh javap: the `if_icmpne` falls through to return), a
    /// re-open RESETS the store (the fresh menu), and the Container
    /// Close drops it wholesale (the audited close is ONE assignment
    /// with NO id check) leaving the window-0 inventory store
    /// untouched.
    #[test]
    fn the_open_container_store_receives_the_chests_traffic() {
        use mc_protocol::play::{clientbound, parse_container_close, parse_open_screen,
            parse_set_container_content, parse_set_container_slot};
        let mut m = WorldModel::new();
        assert!(m.open_container().is_none(), "no screen, no store");

        // Open Screen: window 5, the chest (generic_9x3 = 2), the
        // title a minimal anonymous-root compound riding verbatim (the
        // 1.20.5+ root has NO name — the root byte then the end tag)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::OPEN_SCREEN);
        write_varint(&mut p, 5);
        write_varint(&mut p, MENU_TYPE_CHEST);
        p.extend_from_slice(&[0x0A, 0x00]); // the minimal empty root
        let o = parse_open_screen(&p).unwrap();
        assert!(m.apply(PlayEvent::ScreenOpened(o)));
        let oc = m.open_container().unwrap();
        assert_eq!(oc.id, 5);
        assert_eq!(oc.menu_type_id, MENU_TYPE_CHEST, "generic_9x3, the chest");
        assert_eq!(oc.title_nbt, vec![0x0A, 0x00], "the title rides verbatim");
        assert!(oc.slots.is_empty(), "the contents ride the container packets");

        // the matching window's snapshot REPLACES the store
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 5); // the chest's window
        write_varint(&mut p, 11); // state id
        write_varint(&mut p, 2); // the collection count
        write_varint(&mut p, 3);
        write_varint(&mut p, 745);
        write_varint(&mut p, 0); // patch: none added
        write_varint(&mut p, 0); // patch: none removed
        write_varint(&mut p, 0); // slot 1: the EMPTY stack
        write_varint(&mut p, 0); // the carried item: empty
        let c = parse_set_container_content(&p).unwrap();
        assert!(m.apply(PlayEvent::ContainerFilled(c)));
        let oc = m.open_container().unwrap();
        assert_eq!(oc.state_id, 11);
        assert_eq!(oc.slots.len(), 2);
        assert_eq!(oc.slots[0].item_id, 745);

        // the matching window's slot delta UPSERTS through the shared
        // policy — slot 4 grows the store past the snapshot's two
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 5);
        write_varint(&mut p, 12);
        p.extend_from_slice(&4i16.to_be_bytes());
        write_varint(&mut p, 9);
        write_varint(&mut p, 33);
        write_varint(&mut p, 0);
        write_varint(&mut p, 0);
        let s = parse_set_container_slot(&p).unwrap();
        assert!(m.apply(PlayEvent::ContainerSlotSet(s)));
        let oc = m.open_container().unwrap();
        assert_eq!(oc.state_id, 12);
        assert_eq!(oc.slots.len(), 5, "grown to address slot 4");
        assert_eq!(oc.slots[4].item_id, 33);

        // a MISMATCHED window is dropped: window 6 while 5 is open —
        // the packet rode the wire, the store never hears of it
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_CONTENT);
        write_varint(&mut p, 6); // another window
        write_varint(&mut p, 99);
        write_varint(&mut p, 0);
        write_varint(&mut p, 0);
        let c = parse_set_container_content(&p).unwrap();
        assert!(!m.apply(PlayEvent::ContainerFilled(c)));
        assert_eq!(m.open_container().unwrap().state_id, 12, "untouched");

        // the inventory store (window 0) is a SEPARATE lane — it keeps
        // feeding while the chest is open
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 0);
        write_varint(&mut p, 20);
        p.extend_from_slice(&40i16.to_be_bytes());
        write_varint(&mut p, 2);
        write_varint(&mut p, 777);
        write_varint(&mut p, 0);
        write_varint(&mut p, 0);
        let s = parse_set_container_slot(&p).unwrap();
        assert!(m.apply(PlayEvent::ContainerSlotSet(s)));
        assert_eq!(m.inventory().state_id, 20);
        assert_eq!(m.inventory_slot(40).unwrap().item_id, 777);

        // a re-open RESETS the store (the fresh menu has fresh slots)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::OPEN_SCREEN);
        write_varint(&mut p, 5);
        write_varint(&mut p, MENU_TYPE_CHEST);
        p.extend_from_slice(&[0x0A, 0x00]);
        let o = parse_open_screen(&p).unwrap();
        assert!(m.apply(PlayEvent::ScreenOpened(o)));
        let oc = m.open_container().unwrap();
        assert!(oc.slots.is_empty(), "the fresh menu resets the store");
        assert_eq!(oc.state_id, 0);

        // the CLOSE drops the store wholesale (the audited ONE
        // assignment, no id check — the closing id rides verbatim)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_CLOSE);
        write_varint(&mut p, 5);
        let c = parse_container_close(&p).unwrap();
        assert!(m.apply(PlayEvent::ContainerClosed(c)));
        assert!(m.open_container().is_none(), "the close drops the store");
        assert_eq!(m.inventory().state_id, 20, "the window-0 lane untouched");

        // a close with nothing open is the no-op repeat
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_CLOSE);
        write_varint(&mut p, 0);
        let c = parse_container_close(&p).unwrap();
        assert!(!m.apply(PlayEvent::ContainerClosed(c)));
    }

    /// Spawn ONE entity at the named position (the full-precision
    /// doubles are exact — the walk tests rely on that).
    fn spawn_at(m: &mut WorldModel, id: i32, x: f64, y: f64, z: f64) {
        m.apply(PlayEvent::Spawn(EntitySpawn {
            id,
            uuid: 1,
            type_id: 3,
            x,
            y,
            z,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 0.0,
            y_head_rot: 0.0,
            data: 0,
        }));
    }

    #[test]
    fn the_ambient_light_color_reads_the_built_in_table() {
        // the built-in visual/ambient_light_color table through the
        // model's pinned dimension: the three vanilla worlds' stated
        // floors (the generator emitted them from the jar's dimension
        // JSONs — #0a0a0a / #302821 / #3f473f), and the honest None for
        // no dimension pinned yet / an unknown custom key — the
        // bytecode-default black, never a guess.
        let mut m = WorldModel::new();
        assert_eq!(m.ambient_light_color(), None, "no dimension: no opinion");
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.ambient_light_color(), Some([0x0a, 0x0a, 0x0a]));
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 1,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.ambient_light_color(), Some([0x30, 0x28, 0x21]));
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 2,
            dimension: "minecraft:the_end".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.ambient_light_color(), Some([0x3f, 0x47, 0x3f]));
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 9,
            dimension: "minecraft:custom_dim".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(
            m.ambient_light_color(),
            None,
            "unknown custom key: the black default, never a guess"
        );
    }

    #[test]
    fn the_ambient_light_color_prefers_the_synced_truth() {
        // the dimension_min_y discipline, ambient edition: the synced
        // registry is the server's truth (its stated value wins even
        // over the built-in, its SILENCE reads the bytecode default —
        // black, the vanilla EnvironmentAttribute fallback); the
        // built-in table serves ONLY when the sync carried no dimension
        // table at all (the skip-sync fallback); a sync that arrived
        // but omits the key keeps None, never a guess.
        let mut m = WorldModel::new();
        m.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 17,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        }));
        // no sync yet: the built-in table's overworld floor
        assert_eq!(
            m.ambient_light_color(),
            Some([0x0a, 0x0a, 0x0a]),
            "skip-sync fallback: the built-in table serves"
        );
        // the sync arrives WITH the attribute: its value wins, even
        // where it differs from the built-in
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![DimensionTypeEntry {
                name: "minecraft:overworld".into(),
                min_y: -64,
                height: 384,
                default_clock: Some("minecraft:overworld".into()),
                timelines: None,
                ambient_light_color: Some([0x11, 0x22, 0x33]),
                skybox: None,
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: None,
                block_light_tint: None,
                sky_light_color: None,
                sky_light_factor: None,
                night_vision_color: None,
            }],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: Vec::new(),
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        assert_eq!(
            m.ambient_light_color(),
            Some([0x11, 0x22, 0x33]),
            "the synced value is the server's truth, not the built-in"
        );
        // a synced entry that states NONE of the attributes: the
        // vanilla default (black) — NOT the built-in's opinion
        m.registry.dimension_types[0].ambient_light_color = None;
        assert_eq!(
            m.ambient_light_color(),
            None,
            "the sync arrived and is silent: the bytecode-default black, its own truth"
        );
        // a sync that arrived but omits the KEY entirely: None — the
        // server's truth is "not a vanilla built-in", never a guess
        m.registry.dimension_types[0].name = "minecraft:custom_dim".into();
        assert_eq!(
            m.ambient_light_color(),
            None,
            "a live sync without the key: no built-in fallback"
        );
    }

    #[test]
    fn the_skybox_reads_the_built_in_table() {
        // the built-in skybox column through the model's pinned
        // dimension: the vanilla worlds' verdicts (the generator
        // emitted them from the jar's dimension JSONs — overworld
        // Overworld, the nether None, the end End), and the Overworld
        // sky path for no dimension pinned yet / an unknown custom key
        // — the codec's own default, never a claimed truth.
        use mc_registry::generated::worldgen::Skybox;
        let mut m = WorldModel::new();
        assert_eq!(
            m.skybox(),
            Skybox::Overworld,
            "no dimension: the default sky path"
        );
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.skybox(), Skybox::Overworld);
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 1,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.skybox(), Skybox::None, "the nether ships no sky at all");
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 2,
            dimension: "minecraft:the_end".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.skybox(), Skybox::End, "the end ships the end sky");
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 9,
            dimension: "minecraft:custom_dim".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(
            m.skybox(),
            Skybox::Overworld,
            "unknown custom key: the vanilla default, never a guess"
        );
    }

    #[test]
    fn the_skybox_prefers_the_synced_truth() {
        // the ambient discipline, skybox edition: the synced registry
        // is the server's truth (its STATED skybox wins even over the
        // built-in, its SILENCE reads the codec's optionalFieldOf
        // default — Overworld, the vanilla binding); the built-in
        // table serves ONLY when the sync carried no dimension table
        // at all (the skip-sync fallback); a sync that arrived but
        // omits the key keeps the codec default, never the built-in's
        // opinion.
        use mc_registry::generated::worldgen::Skybox;
        let mut m = WorldModel::new();
        m.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 17,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        }));
        // no sync yet: the built-in table's overworld sky
        assert_eq!(
            m.skybox(),
            Skybox::Overworld,
            "skip-sync fallback: the built-in table serves"
        );
        // the sync arrives WITH the key: its value wins, even where it
        // differs from the built-in (a custom overworld whose element
        // states the nether's none)
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![DimensionTypeEntry {
                name: "minecraft:overworld".into(),
                min_y: -64,
                height: 384,
                default_clock: Some("minecraft:overworld".into()),
                timelines: None,
                ambient_light_color: None,
                skybox: Some(mc_protocol::Skybox::None),
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: None,
                block_light_tint: None,
                sky_light_color: None,
                sky_light_factor: None,
                night_vision_color: None,
            }],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: Vec::new(),
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        assert_eq!(
            m.skybox(),
            Skybox::None,
            "the synced value is the server's truth, not the built-in"
        );
        // a synced entry that states NOTHING: the codec default
        // (Overworld) — NOT the built-in's opinion, and never the
        // built-in nether's None
        m.registry.dimension_types[0].skybox = None;
        assert_eq!(
            m.skybox(),
            Skybox::Overworld,
            "the sync arrived and is silent: the codec's own default"
        );
        // a sync that arrived but omits the KEY for the dimension
        // entirely: the codec default — never a built-in fallback for
        // a dimension the server named itself
        m.registry.dimension_types[0].name = "minecraft:custom_dim".into();
        assert_eq!(
            m.skybox(),
            Skybox::Overworld,
            "a live sync without the key: no built-in fallback"
        );
    }

    #[test]
    fn the_fog_distances_read_the_stated_pair() {
        // the nether's real 10/96 (the fogged walls) through the model:
        // the built-in table answers on the skip-sync path, the SYNCED
        // entry wins when it states the pair, the silence and the
        // defaults' shape read None (the window treatment's verdict —
        // the bytecode defaults 0/1024 are far beyond the window), and
        // a one-sided statement is the honest absence (the
        // pair-or-nothing rule).
        let mut m = WorldModel::new();
        assert_eq!(m.fog_distances(), None, "no dimension: no opinion");
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(
            m.fog_distances(),
            None,
            "the overworld states no pair — the defaults' shape, the window treatment"
        );
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 1,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(
            m.fog_distances(),
            Some((10.0, 96.0)),
            "the built-in table's nether pair (the skip-sync fallback)"
        );
        // the synced truth wins: a custom server's own stated pair
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![DimensionTypeEntry {
                name: "minecraft:the_nether".into(),
                min_y: 0,
                height: 256,
                default_clock: None,
                timelines: None,
                ambient_light_color: None,
                skybox: Some(mc_protocol::Skybox::None),
                fog_start_distance: Some(5.0),
                fog_end_distance: Some(48.0),
                cloud_fog_end_distance: None,
                block_light_tint: None,
                sky_light_color: None,
                sky_light_factor: None,
                night_vision_color: None,
            }],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: Vec::new(),
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        assert_eq!(
            m.fog_distances(),
            Some((5.0, 48.0)),
            "the synced value is the server's truth, not the built-in"
        );
        // the sync arrives and is silent: None — the defaults' shape, the
        // window treatment, NOT the built-in's nether pair
        m.registry.dimension_types[0].fog_start_distance = None;
        m.registry.dimension_types[0].fog_end_distance = None;
        assert_eq!(
            m.fog_distances(),
            None,
            "the sync arrived and is silent: no pair, never the built-in's"
        );
        // a one-sided statement: the pair-or-nothing rule — the honest
        // absence, never a half guess
        m.registry.dimension_types[0].fog_end_distance = Some(64.0);
        assert_eq!(
            m.fog_distances(),
            None,
            "start without end is not a real vanilla shape"
        );
    }

    #[test]
    fn the_cloud_fog_end_reads_the_stated_float() {
        // the cloud-fog end is the fog pair's single-attribute sibling:
        // no vanilla dimension states it (the built-in table is None for
        // all four — the consumer's 2048.0 bytecode default covers the
        // absence), a CUSTOM server's stated FLOAT is the synced truth,
        // and the sync's silence is the honest absence, never a guess.
        let mut m = WorldModel::new();
        assert_eq!(m.cloud_fog_end_distance(), None, "no dimension: no opinion");
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(
            m.cloud_fog_end_distance(),
            None,
            "the built-in nether states no cloud-fog end — the default's shape"
        );
        // the synced truth wins: a custom server's own stated FLOAT
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![DimensionTypeEntry {
                name: "minecraft:the_nether".into(),
                min_y: 0,
                height: 256,
                default_clock: None,
                timelines: None,
                ambient_light_color: None,
                skybox: Some(mc_protocol::Skybox::None),
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: Some(512.0),
                block_light_tint: None,
                sky_light_color: None,
                sky_light_factor: None,
                night_vision_color: None,
            }],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: Vec::new(),
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        assert_eq!(
            m.cloud_fog_end_distance(),
            Some(512.0),
            "the synced value is the server's truth"
        );
        // the sync arrives and is silent: None — the bytecode default's
        // shape, the sync's own silence, never a built-in fallback
        m.registry.dimension_types[0].cloud_fog_end_distance = None;
        assert_eq!(
            m.cloud_fog_end_distance(),
            None,
            "the sync arrived and is silent: no stated value, never a guess"
        );
    }

    #[test]
    fn the_block_light_tint_reads_the_stated_attribute() {
        // the block-light tint is the lightmap's stated attribute: no
        // vanilla dimension states it (the built-in table is None for
        // all four — the consumer's #FFD88C bytecode default covers the
        // absence), a CUSTOM server's stated rgb_color is the synced
        // truth, and the sync's silence is the honest absence, never a
        // guess. The three-branch precedence, the fog pair's shape.
        let mut m = WorldModel::new();
        assert_eq!(m.block_light_tint(), None, "no dimension: no opinion");
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(
            m.block_light_tint(),
            None,
            "the built-in nether states no tint — the default's shape"
        );
        // the synced truth wins: a custom server's own stated rgb_color
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![DimensionTypeEntry {
                name: "minecraft:the_nether".into(),
                min_y: 0,
                height: 256,
                default_clock: None,
                timelines: None,
                ambient_light_color: None,
                skybox: Some(mc_protocol::Skybox::None),
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: None,
                block_light_tint: Some([0x30, 0x28, 0x21]),
                sky_light_color: None,
                sky_light_factor: None,
                night_vision_color: None,
            }],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: Vec::new(),
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        assert_eq!(
            m.block_light_tint(),
            Some([0x30, 0x28, 0x21]),
            "the synced value is the server's truth"
        );
        // the sync arrives and is silent: None — the bytecode default's
        // shape (#FFD88C, the corrected blue 140), the sync's own
        // silence, never a guess
        m.registry.dimension_types[0].block_light_tint = None;
        assert_eq!(
            m.block_light_tint(),
            None,
            "the sync arrived and is silent: no stated value, never a guess"
        );
    }

    #[test]
    fn the_stated_sky_light_factor_folds_under_the_timeline_track() {
        // the sky-light factor's consumer contract is the attribute-system
        // FOLD (javap EnvironmentAttributeSystem: the dimension's constant
        // layer overrides the registered default 1.0, the timeline's
        // multiply track applies last) — and the raw jar JSON pins the
        // discovery: the END and the NETHER state 0.0, their sky term
        // ZERO. The built-in table carries that shape on skip-sync.
        let mut m = WorldModel::new();
        assert_eq!(m.sky_light_factor(), None, "no dimension: the default");
        assert_eq!(m.sky_factor(), 1.0, "no clock, no statement: the default 1.0");
        // the built-in nether: the stated 0.0 — the sky term pinned to
        // zero with no clock and no track (the fold's override alone)
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(
            m.sky_light_factor(),
            Some(0.0),
            "the built-in nether STATES 0.0 — this round's jar discovery"
        );
        assert_eq!(
            m.sky_factor(),
            0.0,
            "no clock: the stated 0.0 overrides the default, no track to multiply"
        );
        // back to the overworld: unstated, the identity returns
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.sky_light_factor(), None);
        assert_eq!(m.sky_factor(), 1.0);
        // a CUSTOM server's stated 0.5: the multiply-BASE under the day
        // curve — noon 0.5 x 1.0, midnight 0.5 x 0.24 = 0.12
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![DimensionTypeEntry {
                name: "minecraft:overworld".into(),
                min_y: -64,
                height: 384,
                default_clock: Some("minecraft:overworld".into()),
                timelines: Some("#minecraft:in_overworld".into()),
                ambient_light_color: None,
                skybox: None,
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: None,
                block_light_tint: None,
                sky_light_color: None,
                sky_light_factor: Some(0.5),
                night_vision_color: None,
            }],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: vec!["minecraft:overworld".to_string()],
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        // noon: the compile-time day snapshot (no synced timeline) gives
        // 1.0, the stated base halves it
        let p = mc_protocol::play::test_support::set_time_packet(1, &[(0, 1_002_006_000, 0.0, 1.0)]);
        let ts = mc_protocol::play::parse_set_time(&p).expect("set time parses");
        assert!(m.apply(PlayEvent::Time(ts)));
        assert_eq!(
            m.sky_factor(),
            0.5,
            "the stated base scales the day track: 0.5 x 1.0"
        );
        // midnight: 0.5 x 0.24
        let p2 =
            mc_protocol::play::test_support::set_time_packet(2, &[(0, 1_002_018_000, 0.0, 1.0)]);
        let ts2 = mc_protocol::play::parse_set_time(&p2).expect("set time parses");
        assert!(m.apply(PlayEvent::Time(ts2)));
        let night = crate::timeline::sky_light_factor(18_000);
        assert_eq!(
            m.sky_factor(),
            0.5 * night,
            "the stated base scales the night plateau too"
        );
        // the sync's silence restores the default: the exact identity
        m.registry.dimension_types[0].sky_light_factor = None;
        assert_eq!(m.sky_factor(), night, "silent sync: the default 1.0 x track");
    }

    #[test]
    fn the_sky_light_color_reads_the_stated_attribute() {
        // the sky-light color is the lightmap's stated attribute: the
        // overworlds state none (the built-in None — the -1 = #FFFFFF
        // white bytecode default covers the absence, the ARGB multiply
        // identity), the END and the NETHER state their moonlight pairs
        // (#ac60cd / #7a7aff — the generated table's honest rows, the
        // raw jar JSON), and a custom server's stated rgb_color is the
        // synced truth. The three-branch precedence, the factor's shape.
        let mut m = WorldModel::new();
        assert_eq!(m.sky_light_color(), None, "no dimension: no opinion");
        // the built-in end: the stated #ac60cd — the real jar's verdict
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_end".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(
            m.sky_light_color(),
            Some([0xac, 0x60, 0xcd]),
            "the built-in end STATES #ac60cd — the moonlight pair"
        );
        // the built-in nether: #7a7aff
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(
            m.sky_light_color(),
            Some([0x7a, 0x7a, 0xff]),
            "the built-in nether STATES #7a7aff"
        );
        // the overworld: unstated — the white default's shape
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.sky_light_color(), None);
        // the synced truth wins: a custom server's own stated rgb_color
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![DimensionTypeEntry {
                name: "minecraft:overworld".into(),
                min_y: -64,
                height: 384,
                default_clock: Some("minecraft:overworld".into()),
                timelines: Some("#minecraft:in_overworld".into()),
                ambient_light_color: None,
                skybox: None,
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: None,
                block_light_tint: None,
                sky_light_color: Some([0x30, 0x28, 0x21]),
                sky_light_factor: None,
                night_vision_color: None,
            }],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: Vec::new(),
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        assert_eq!(
            m.sky_light_color(),
            Some([0x30, 0x28, 0x21]),
            "the synced value is the server's truth"
        );
        // the sync arrives and is silent: None — the white default's
        // shape, the sync's own silence, never a guess
        m.registry.dimension_types[0].sky_light_color = None;
        assert_eq!(
            m.sky_light_color(),
            None,
            "the sync arrived and is silent: no stated value, never a guess"
        );
    }

    #[test]
    fn the_night_vision_color_reads_the_stated_attribute() {
        // the night-vision color is the lightmap's LAST stated
        // attribute: no vanilla dimension states it (the built-in rows
        // all None — the -6710887 = #999999 gray bytecode default
        // covers the absence; the end/nether state their sky pairs,
        // not this), and a custom server's stated rgb_color is the
        // synced truth. The shader consumes it UNDER the night-vision
        // FACTOR — the player's EFFECT state (NIGHT_VISION /
        // conduit power + water vision), 0.0 without (javap
        // LightmapRenderStateExtractor.extract: the probe reads the
        // attribute unconditionally, the effect state gates the term).
        // The three-branch precedence, the color's shape.
        let mut m = WorldModel::new();
        assert_eq!(m.night_vision_color(), None, "no dimension: no opinion");
        // the built-in end: unstated — None (the honest all-None shape,
        // the sky pairs ride OTHER columns)
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_end".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(
            m.night_vision_color(),
            None,
            "the built-in end does NOT state it — the gray default's shape"
        );
        // the built-in nether: likewise unstated
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.night_vision_color(), None);
        // the overworld: unstated — the gray default's shape
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".to_string(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.night_vision_color(), None);
        // the synced truth wins: a custom server's own stated rgb_color
        m.registry = RegistryData {
            command_argument_types: Vec::new(),
            chat_types: Vec::new(),
            dimension_types: vec![DimensionTypeEntry {
                name: "minecraft:overworld".into(),
                min_y: -64,
                height: 384,
                default_clock: Some("minecraft:overworld".into()),
                timelines: Some("#minecraft:in_overworld".into()),
                ambient_light_color: None,
                skybox: None,
                fog_start_distance: None,
                fog_end_distance: None,
                cloud_fog_end_distance: None,
                block_light_tint: None,
                sky_light_color: None,
                sky_light_factor: None,
                night_vision_color: Some([0x30, 0x28, 0x21]),
            }],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: Vec::new(),
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
            sound_events: Vec::new(),
            chicken_variants: Vec::new(),
            pig_variants: Vec::new(),
            cow_variants: Vec::new(),
            chicken_sound_variants: Vec::new(),
            pig_sound_variants: Vec::new(),
            cow_sound_variants: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        assert_eq!(
            m.night_vision_color(),
            Some([0x30, 0x28, 0x21]),
            "the synced value is the server's truth"
        );
        // the sync arrives and is silent: None — the gray default's
        // shape, the sync's own silence, never a guess
        m.registry.dimension_types[0].night_vision_color = None;
        assert_eq!(
            m.night_vision_color(),
            None,
            "the sync arrived and is silent: no stated value, never a guess"
        );
    }

    #[test]
    fn the_scoreboard_trio_rides_the_belief() {
        let mut m = WorldModel::new();

        // SET OBJECTIVE add: the belief stores the display + the type
        assert!(m.apply(PlayEvent::Objective(mc_protocol::ObjectiveUpdate {
            objective: "money".into(),
            method: mc_protocol::OBJECTIVE_METHOD_ADD,
            display: Some("Coins".into()),
            render_type: mc_protocol::RENDER_TYPE_INTEGER,
        })));
        assert_eq!(m.objectives["money"].display, "Coins");

        // SET DISPLAY OBJECTIVE: the sidebar slot names the objective
        assert!(m.apply(PlayEvent::DisplayObjective(mc_protocol::DisplayObjectiveUpdate {
            slot: 1,
            objective: "money".into(),
        })));
        // ... and the repeat is a no-op (the changed answer's own law)
        assert!(!m.apply(PlayEvent::DisplayObjective(mc_protocol::DisplayObjectiveUpdate {
            slot: 1,
            objective: "money".into(),
        })));

        // SET SCORE: an UNKNOWN objective is the warn + drop (no state)
        assert!(!m.apply(PlayEvent::Score(mc_protocol::ScoreUpdate {
            owner: "Zed".into(),
            objective: "ghost".into(),
            score: 5,
            display: None,
        })));
        assert!(m.scores.is_empty(), "the gate refused the ghost row");
        // the known one upserts
        assert!(m.apply(PlayEvent::Score(mc_protocol::ScoreUpdate {
            owner: "Zed".into(),
            objective: "money".into(),
            score: 42,
            display: Some("Zed Inc.".into()),
        })));
        assert_eq!(m.scores[&("Zed".into(), "money".into())].value, 42);
        assert_eq!(
            m.scores[&("Zed".into(), "money".into())].display.as_deref(),
            Some("Zed Inc.")
        );

        // SET DISPLAY OBJECTIVE with an UNKNOWN name CLEARS the slot
        // (the handler's null-objective set)
        assert!(m.apply(PlayEvent::DisplayObjective(mc_protocol::DisplayObjectiveUpdate {
            slot: 1,
            objective: "ghost".into(),
        })));
        assert!(!m.display_slots.contains_key(&1), "the unknown name cleared");
        // ... and the re-set restores
        assert!(m.apply(PlayEvent::DisplayObjective(mc_protocol::DisplayObjectiveUpdate {
            slot: 1,
            objective: "money".into(),
        })));

        // OBJECTIVE UPDATE (method 2): the display + the type re-speak
        assert!(m.apply(PlayEvent::Objective(mc_protocol::ObjectiveUpdate {
            objective: "money".into(),
            method: mc_protocol::OBJECTIVE_METHOD_UPDATE,
            display: Some("Gold".into()),
            render_type: mc_protocol::RENDER_TYPE_HEARTS,
        })));
        assert_eq!(m.objectives["money"].display, "Gold");
        assert_eq!(m.objectives["money"].render_type, mc_protocol::RENDER_TYPE_HEARTS);

        // RESET SCORE: a named reset is GATED on the objective existing
        assert!(!m.apply(PlayEvent::ResetScore(mc_protocol::ResetScore {
            owner: "Zed".into(),
            objective: Some("ghost".into()),
        })));
        // the known one removes the row
        assert!(m.apply(PlayEvent::Score(mc_protocol::ScoreUpdate {
            owner: "Amy".into(),
            objective: "money".into(),
            score: 7,
            display: None,
        })));
        assert!(m.apply(PlayEvent::ResetScore(mc_protocol::ResetScore {
            owner: "Amy".into(),
            objective: Some("money".into()),
        })));
        assert!(!m.scores.contains_key(&("Amy".into(), "money".into())));

        // OBJECTIVE REMOVE: the slots pointing at it clear, the scores
        // keyed by it die (the removeObjective cleanup)
        assert!(m.apply(PlayEvent::Objective(mc_protocol::ObjectiveUpdate {
            objective: "money".into(),
            method: mc_protocol::OBJECTIVE_METHOD_REMOVE,
            display: None,
            render_type: mc_protocol::RENDER_TYPE_INTEGER,
        })));
        assert!(!m.objectives.contains_key("money"), "the objective died");
        assert!(!m.display_slots.contains_key(&1), "the slot cleared");
        assert!(m.scores.is_empty(), "the objective's rows died with it");

        // the whole-owner reset (the NULL objective walk)
        assert!(m.apply(PlayEvent::Objective(mc_protocol::ObjectiveUpdate {
            objective: "a".into(),
            method: mc_protocol::OBJECTIVE_METHOD_ADD,
            display: Some("A".into()),
            render_type: mc_protocol::RENDER_TYPE_INTEGER,
        })));
        assert!(m.apply(PlayEvent::Objective(mc_protocol::ObjectiveUpdate {
            objective: "b".into(),
            method: mc_protocol::OBJECTIVE_METHOD_ADD,
            display: Some("B".into()),
            render_type: mc_protocol::RENDER_TYPE_INTEGER,
        })));
        for obj in ["a", "b"] {
            assert!(m.apply(PlayEvent::Score(mc_protocol::ScoreUpdate {
                owner: "Zed".into(),
                objective: obj.into(),
                score: 1,
                display: None,
            })));
        }
        assert!(m.apply(PlayEvent::ResetScore(mc_protocol::ResetScore {
            owner: "Zed".into(),
            objective: None,
        })));
        assert!(m.scores.is_empty(), "the holder's whole ledger died");

        // THE RESPAWN SURVIVES (the scoreboard is the CONNECTION's own
        // object — the dimension swap clears the level's columns and
        // the mailboxes but NEVER the sidebar's belief)
        assert!(m.apply(PlayEvent::DisplayObjective(mc_protocol::DisplayObjectiveUpdate {
            slot: 1,
            objective: "a".into(),
        })));
        assert!(m.apply(PlayEvent::Score(mc_protocol::ScoreUpdate {
            owner: "Zed".into(),
            objective: "a".into(),
            score: 9,
            display: None,
        })));
        assert!(m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        })));
        assert_eq!(m.scores[&("Zed".into(), "a".into())].value, 9, "the belief rides");
        assert_eq!(
            m.display_slots.get(&1).map(String::as_str),
            Some("a"),
            "the slot survives the swap"
        );
    }

    #[test]
    fn the_string_uuid_rides_the_holder_law() {
        // the known Java UUID.toString shape: the 8-4-4-4-12 lowercase
        // hyphenated words, the most-significant long first (the wire's
        // own pack order the u128's high half keeps)
        let u = (0x550e8400e29b41d4u128 << 64) | 0xa716446655440000u128;
        assert_eq!(string_uuid(u), "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(string_uuid(0), "00000000-0000-0000-0000-000000000000");
        // the SPAWN keeps the wire's own bits (the Add Entity packet's
        // 128 bits the scoreboard holder's string renders from)
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 7,
            uuid: u,
            type_id: 0,
            x: 1.0,
            y: 2.0,
            z: 3.0,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 0.0,
            y_head_rot: 0.0,
            data: 0,
        }));
        assert_eq!(m.entity(7).unwrap().uuid, u, "the bits the holder rides");
    }

    #[test]
    fn the_player_info_lanes_keep_the_tab_belief() {
        // THE BIRTH WALK (the putIfAbsent law): a NAMED patch creates
        // the entry — the vanilla PlayerInfo ctor's own defaults ride
        // the fields the mask never spoke (latency -1, unlisted, the
        // default game mode)
        let mut m = WorldModel::new();
        let steve = (0x5500_0000u128 << 96) | 0x55AAu128;
        assert!(m.apply(PlayEvent::PlayerInfoUpdate(mc_protocol::PlayerInfoUpdate {
            entries: vec![mc_protocol::PlayerEntryPatch {
                uuid: steve,
                name: Some("Steve".into()),
                game_mode: None,
                listed: Some(true),
                latency: None,
                display_name: None,
                list_order: Some(5),
            }],
        })));
        let e = m.players.get(&steve).unwrap();
        assert_eq!(e.name, "Steve");
        assert_eq!(e.game_mode, 0, "the ADD-alone's default mode");
        assert!(e.listed);
        assert_eq!(e.latency, -1, "the ctor's own default latency");
        assert_eq!(e.list_order, 5);

        // THE OVERLAY WALK: the second update touches the SAME entry
        // in place (the fields the mask spoke change, the rest ride)
        assert!(m.apply(PlayEvent::PlayerInfoUpdate(mc_protocol::PlayerInfoUpdate {
            entries: vec![mc_protocol::PlayerEntryPatch {
                uuid: steve,
                name: None,
                game_mode: Some(3),
                listed: None,
                latency: Some(77),
                display_name: None,
                list_order: None,
            }],
        })));
        let e = m.players.get(&steve).unwrap();
        assert_eq!((e.game_mode, e.latency, e.name.as_str()), (3, 77, "Steve"));
        // the IDENTICAL walk is the honest no-op (no change)
        assert!(!m.apply(PlayEvent::PlayerInfoUpdate(mc_protocol::PlayerInfoUpdate {
            entries: vec![mc_protocol::PlayerEntryPatch {
                uuid: steve,
                name: None,
                game_mode: Some(3),
                listed: None,
                latency: Some(77),
                display_name: None,
                list_order: None,
            }],
        })));

        // an UPDATE-ONLY patch for an unknown uuid is the silent no-op
        // (the map walk's own guard)
        assert!(!m.apply(PlayEvent::PlayerInfoUpdate(mc_protocol::PlayerInfoUpdate {
            entries: vec![mc_protocol::PlayerEntryPatch {
                uuid: 9,
                name: None,
                game_mode: None,
                listed: Some(true),
                latency: None,
                display_name: None,
                list_order: None,
            }],
        })));
        assert!(!m.players.contains_key(&9), "no ghost entry");

        // THE REMOVAL: the entry dies, the unknown uuid no-ops
        assert!(m.apply(PlayEvent::PlayerInfoRemove(mc_protocol::PlayerInfoRemove {
            uuids: vec![steve]
        })));
        assert!(m.players.is_empty());
        assert!(!m.apply(PlayEvent::PlayerInfoRemove(mc_protocol::PlayerInfoRemove {
            uuids: vec![steve]
        })));

        // THE TAB LIST: the header/footer land verbatim (the
        // handleTabListCustomisation walk), the identical re-speak is
        // the honest no-op
        assert!(m.apply(PlayEvent::TabList(mc_protocol::TabListUpdate {
            header: "The Top".into(),
            footer: "The Bottom".into(),
        })));
        assert_eq!(
            (m.tab_header.as_str(), m.tab_footer.as_str()),
            ("The Top", "The Bottom")
        );
        assert!(!m.apply(PlayEvent::TabList(mc_protocol::TabListUpdate {
            header: "The Top".into(),
            footer: "The Bottom".into(),
        })));

        // THE RESPAWN SURVIVES (the scoreboard's own law mirrored): the
        // tab is the CONNECTION's own object — the vanilla map lives on
        // the ClientPacketListener one level above the level the
        // Respawn recreates, the entries and the bands ride the swap
        assert!(m.apply(PlayEvent::PlayerInfoUpdate(mc_protocol::PlayerInfoUpdate {
            entries: vec![mc_protocol::PlayerEntryPatch {
                uuid: steve,
                name: Some("Steve".into()),
                game_mode: None,
                listed: Some(true),
                latency: None,
                display_name: None,
                list_order: None,
            }],
        })));
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".into(),
            portal_cooldown: 0,
        }));
        assert_eq!(m.players.get(&steve).unwrap().name, "Steve", "the tab rides the swap");
        assert_eq!(m.tab_header, "The Top", "the band rides too");
    }

    #[test]
    fn the_built_in_fallback_speaks_the_frozen_oceans_modifier() {
        // THE SKIP-SYNC FALLBACK's own climate slice: the registry
        // empty (no sync at all), the BUILT-IN jar tables serve — the
        // generated rows' temperature_modifier word the SAME mapping
        // the synced arm keeps now drives (the limitation the 344
        // round documented, the generator field the round retires).
        let m = WorldModel::new();
        assert!(m.registry.biomes.is_empty(), "the mock boot ships no sync");
        let climates = m.biome_climates();
        // the WIRE order the slice indexes: find the frozen pair by
        // the registry-order name chain the fallback itself walks
        use mc_registry::generated::registry_order::BIOME_ORDER;
        use mc_registry::generated::worldgen::biome as built_in;
        for (i, name) in BIOME_ORDER.iter().enumerate() {
            let def = built_in(name).expect("the built-in chain resolves every order row");
            let climate = climates[i];
            assert_eq!(climate.has_precipitation, def.has_precipitation);
            assert_eq!(climate.temperature, def.temperature);
            match def.temperature_modifier {
                Some("frozen") => {
                    assert_eq!(
                        climate.temperature_modifier,
                        crate::world::TemperatureModifier::Frozen,
                        "{name}: the frozen word the table carries the enum wears"
                    );
                }
                _ => assert_eq!(
                    climate.temperature_modifier,
                    crate::world::TemperatureModifier::None,
                    "{name}: the absence IS the codec's default"
                ),
            }
        }
        // THE RAIN-IN-FROZEN ANSWER the fallback too speaks: the
        // built-in frozen_ocean climate through the weather lane's
        // precipitation pick — the 0.2F patchwork's opened gate at
        // (0, 0) warmens the 0.0 base past the 0.15F snow line, the
        // same word the modifier's own test pins end to end
        use mc_registry::generated::registry_order::BIOME_ORDER as ORDER;
        let idx = ORDER
            .iter()
            .position(|n| *n == "minecraft:frozen_ocean")
            .expect("the wire order carries the frozen ocean");
        let frozen = climates[idx];
        assert_eq!(
            frozen.temperature_modifier,
            crate::world::TemperatureModifier::Frozen
        );
        assert_eq!(
            crate::weather::precipitation(frozen, 0, 64, 0, 63),
            crate::weather::Precipitation::Rain,
            "the fallback climate's patchwork RAINS at the warm patch"
        );
    }
}

//! Configuration (protocol state 3): the phase both sides enter after
//! Login Success (via the serverbound Login Acknowledged). The vanilla
//! client introduces itself (Client Information + the `minecraft:brand`
//! plugin message), answers Keep Alive / Ping probes, Select Known Packs
//! and Cookie Requests, skips the bulk feature/resource-pack traffic and
//! acknowledges the server's Finish Configuration — then the play state
//! begins.
//!
//! Runtime-true shapes for 1.20.5+ / 26.x (the wire IDS are GENERATED —
//! `mc2rs-gen dump-packet-ids` binds the vanilla ConfigurationProtocols
//! and dumps the live dispatch codec's id map; the wiki-era table they
//! replaced had drifted: every clientbound id from disconnect onward was
//! one LOW, the wiki table forgetting cookie_request's head slot —
//! registry_data is 0x07 on the real 26.2 wire, not 0x06):
//! - Login Acknowledged (serverbound login 0x03): empty payload — from the
//!   NEXT packet on, both sides speak configuration.
//! - Client Information (serverbound config 0x00): javap-proven 26.2
//!   shape — ServerboundClientInformationPacket delegates to
//!   ClientInformation.write: Locale (String 16), View Distance (BYTE,
//!   not a VarInt), Chat Mode (VarInt enum 0 = enabled), Chat Colors
//!   (Bool), Displayed Skin Parts (Byte 0x7F = all), Main Hand (VarInt
//!   1 = right), Disable Text Filtering (Bool), Allow Server Listings
//!   (Bool), Particle Status (VarInt enum 0 = full) — the wiki-era
//!   encoder was one field SHORT and used the wrong primitive for the
//!   view distance.
//! - Registry Data (clientbound 0x07, one registry per packet):
//!   javap-proven 26.2 shape — ClientboundRegistryDataPacket is the
//!   record (ResourceKey registry, List<PackedRegistryEntry>):
//!   Identifier + VarInt count (max 256) + per entry { Identifier id,
//!   optional network NBT tag }. The wiki-era "Identifier + whole NBT"
//!   shape would have read the entry count as an NBT root and FAILED on
//!   a real server — the same self-consistent-mock mask the id drift
//!   had. The `minecraft:dimension_type` registry is DECODED — every
//!   entry's optional tag IS the dimension element (min_y, height at
//!   the top level, the name from the entry id); every other registry
//!   (biomes, worldgen, banners...) is walked and skipped.
//! - Plugin Message (serverbound config 0x02): Channel (String) + Data —
//!   the brand channel carries one UTF string ("mc2rs").
//! - Keep Alive (clientbound config 0x04 / serverbound 0x04): i64 id,
//!   echoed verbatim.
//! - Ping/Pong (clientbound 0x05 / serverbound 0x05): i32 id, echoed.
//! - Disconnect (clientbound config 0x02): chat JSON reason.
//! - Finish Configuration (clientbound 0x03) → Acknowledge Finish
//!   Configuration (serverbound 0x03): both empty; after the ack the play
//!   state begins.
//! - Select Known Packs (clientbound 0x0E → serverbound 0x07, both
//!   GENERATED): javap-proven 26.2 shape — a list of KnownPack records,
//!   each THREE UTF STRINGS (namespace, id, version — the version is a
//!   string, not a VarInt; `ByteBufCodecs.list(64)` caps the serverbound
//!   reply). The vanilla server BLOCKS on this reply before finishing
//!   configuration — an unanswered packet stalls a real join until the
//!   30s timeout kicks it. The client replies with what it truly knows:
//!   an EMPTY list (claiming a pack version we have not extracted would
//!   make the server skip registry traffic we never received; empty
//!   means the server streams its registries, which the Registry Data
//!   parser already walks).
//! - Cookie Request (clientbound 0x00 → serverbound Cookie Response 0x01,
//!   both GENERATED): javap-proven 26.2 shapes — the request is one
//!   Identifier key; the response is the key echoed + a nullable
//!   byte-array payload (`ByteBufCodecs.byteArray(5120)` under the
//!   nullable). The vanilla client ALWAYS answers — `Map.get` returning
//!   null encodes as the absent payload — and the client stores no
//!   server cookies, so the reply is key + Bool 0.
//! - Everything else (Feature Flags, Resource Pack Push/Pop, Custom
//!   Report Details, Server Links) is read and SKIPPED — the client has
//!   no data-pack or resource-pack surface yet; the vanilla server
//!   finishes configuration fine without answering those.

use std::io::{Read, Write};

use anyhow::{Context, Result};

use crate::framing::{encode_handshake, PacketConn};
use crate::login::{login_conn, LoginOutcome};
use crate::nbt::{read_network_nbt, Nbt};
use crate::varint::{read_string, read_varint, write_string, write_varint};
use crate::{DEFAULT_TIMEOUT_SECS, NEXT_STATE_LOGIN};

/// Serverbound login packet id (the state switch happens at the server),
/// GENERATED from the official jar's real login protocol.
pub mod loginbound {
    pub use crate::generated::packet_ids::LSB_LOGIN_ACKNOWLEDGED as LOGIN_ACKNOWLEDGED;
}

/// Serverbound configuration packet ids, GENERATED from the official
/// jar's real configuration protocol (see the module notes for the drift
/// story). The vanilla names moved on (plugin_message → custom_payload,
/// acknowledge_finish → finish_configuration); the public names are the
/// client's own vocabulary, aliased onto the generated truth.
pub mod serverbound {
    pub use crate::generated::packet_ids::{
        CSB_CLIENT_INFORMATION as CLIENT_INFORMATION,
        CSB_COOKIE_RESPONSE as COOKIE_RESPONSE,
        CSB_CUSTOM_PAYLOAD as PLUGIN_MESSAGE,
        CSB_FINISH_CONFIGURATION as ACKNOWLEDGE_FINISH, CSB_KEEP_ALIVE as KEEP_ALIVE,
        CSB_PONG as PONG, CSB_SELECT_KNOWN_PACKS as SELECT_KNOWN_PACKS,
    };
}

/// Clientbound configuration packet ids, GENERATED from the official
/// jar's real configuration protocol — the ids the wiki-era table had
/// ALL one low (disconnect 0x01 → 0x02, finish 0x02 → 0x03, keep alive
/// 0x03 → 0x04, ping 0x04 → 0x05, registry data 0x06 → 0x07): the
/// vanilla flow grew cookie_request at its head and everything after it
/// shifted. A real 26.2 join would have failed to recognize ANY of
/// these packets; the e2e mocks were self-consistent with the old table
/// and masked it (the same mask the play-id drift had).
pub mod clientbound {
    pub use crate::generated::packet_ids::{
        CCB_COOKIE_REQUEST as COOKIE_REQUEST, CCB_DISCONNECT as DISCONNECT,
        CCB_FINISH_CONFIGURATION as FINISH, CCB_KEEP_ALIVE as KEEP_ALIVE,
        CCB_PING as PING, CCB_REGISTRY_DATA as REGISTRY_DATA,
        CCB_SELECT_KNOWN_PACKS as SELECT_KNOWN_PACKS, CCB_UPDATE_TAGS as UPDATE_TAGS,
    };
}

/// One registered dimension type: the world's vertical layout, decoded
/// from the `minecraft:dimension_type` registry the server streams during
/// configuration (a real vanilla server sends overworld -64/384, nether
/// 0/256, end 0/256).
#[derive(Debug, Clone, PartialEq)]
pub struct DimensionTypeEntry {
    /// Registry key (e.g. `minecraft:overworld`).
    pub name: String,
    /// World Y of the bottom section.
    pub min_y: i32,
    /// Total height in blocks (a multiple of 16).
    pub height: i32,
    /// The dimension's day/night clock (`default_clock` element field:
    /// `minecraft:overworld` for the overworld, `minecraft:the_end` for
    /// the end, ABSENT for the nether — it has no day cycle). The Set
    /// Time clock states resolve through this name via the synced
    /// `minecraft:world_clock` table.
    pub default_clock: Option<String>,
    /// The dimension's timelines reference, VERBATIM from the element
    /// (the overworld carries the TAG form `#minecraft:in_overworld` —
    /// the field is a HolderSet and a tag reference rides the leading
    /// `#`). The tag's member ids resolve through the Update Tags
    /// packet's `minecraft:timeline` payload; without the `#` it is a
    /// direct timeline id.
    pub timelines: Option<String>,
}

/// One synced timeline: the 26.2 data-driven time system's element
/// (javap `Timeline.DIRECT_CODEC`: required string `clock`, optional
/// int `period_ticks`, optional `tracks` map whose values are
/// KeyframeTrack records — required `keyframes` list of {int ticks,
/// float value} + optional `ease` defaulting to LINEAR + optional
/// `modifier`). The client consumes ONE track today —
/// `minecraft:visual/sky_light_factor` — so only THAT track's
/// keyframes are decoded; every other track (bools, strings,
/// gameplay curves) is walked past. Real data: `minecraft:day`
/// (overworld clock, period 24000) is the only timeline carrying the
/// track — `minecraft:moon` (period 192000) and `minecraft:early_game`
/// (no period) do not.
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineEntry {
    /// Registry key (e.g. `minecraft:day`).
    pub name: String,
    /// The clock the timeline binds to (required by the element
    /// codec — the overworld's timelines all name
    /// `minecraft:overworld`).
    pub clock: String,
    /// The cycle the keyframes repeat over (day: 24000). A timeline
    /// without one is non-cyclic (early_game: a one-shot ramp).
    pub period_ticks: Option<i32>,
    /// The `visual/sky_light_factor` keyframes, (tick, value) in
    /// arrival order — `None` when the timeline carries no such track.
    pub sky_light_factor: Option<Vec<(i64, f32)>>,
}

/// The grass-tint modifier one synced biome may carry (javap:
/// `BiomeSpecialEffects$GrassColorModifier` — exactly THREE constants,
/// serialized by their lowercase names `none`/`swamp`/`dark_forest`,
/// pinned from both the bytecode and the class constant pool). Vanilla
/// binds it as optionalFieldOf("grass_color_modifier", NONE) — ABSENT
/// and `none` are the SAME verdict, so the decode collapses both into
/// [`GrassColorModifier::None`] and the field is not an Option. An
/// unknown name is the drift it is (vanilla's own enum codec refuses
/// it the same way).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrassColorModifier {
    None,
    Swamp,
    DarkForest,
}

impl GrassColorModifier {
    fn from_wire(name: &str) -> Option<Self> {
        match name {
            "none" => Some(Self::None),
            "swamp" => Some(Self::Swamp),
            "dark_forest" => Some(Self::DarkForest),
            _ => None,
        }
    }
}

/// The tint-relevant slice of one synced biome's `effects` compound
/// (javap `BiomeSpecialEffects.CODEC`, 26.2 — the class carries ONLY
/// the water color + THREE optional color overrides + the grass
/// modifier; sky/fog/water-fog colors left the biome in the 26.x
/// environment-attributes migration and ride the optional
/// `attributes` compound instead, which the decode walks past — the
/// generator's own worldgen data agrees: overworld fog_color comes
/// out 0, the `minecraft:visual/fog_color` ATTRIBUTE carries it).
/// Colors ride the wire as the vanilla RGB ints verbatim (e.g. water
/// 4159204 = 0x3F76E4).
#[derive(Debug, Clone, PartialEq)]
pub struct BiomeEffects {
    pub water_color: u32,
    pub grass_color: Option<u32>,
    pub foliage_color: Option<u32>,
    pub dry_foliage_color: Option<u32>,
    pub grass_color_modifier: GrassColorModifier,
}

/// One synced biome element — javap `Biome.NETWORK_CODEC` (the codec
/// `RegistryDataLoader`'s networked list binds for the synced
/// `minecraft:worldgen/biome` registry — NOT the disk DIRECT_CODEC,
/// which additionally carries generation/mob settings): a record of
/// exactly THREE — the climate settings (required `has_precipitation`
/// bool, `temperature` float, optional `temperature_modifier` string,
/// `downfall` float), the OPTIONAL `attributes` compound (the
/// environment-attribute overrides — real overworld biomes carry
/// `minecraft:visual/sky_color`, `minecraft:visual/fog_color` and
/// `minecraft:audio/background_music` there; walked past: nothing
/// consumes them yet), and the required `effects` compound decoded
/// into [`BiomeEffects`]. A tagless entry is the drift it is — every
/// codec field is required or defaulted — the same way vanilla's own
/// registry loader refuses malformed data.
#[derive(Debug, Clone, PartialEq)]
pub struct BiomeEntry {
    /// Registry key (e.g. `minecraft:plains`); the ARRIVAL ORDER of
    /// the entries IS the network id the chunk biome palettes
    /// reference.
    pub name: String,
    pub has_precipitation: bool,
    pub temperature: f32,
    /// The vanilla enum is `none`/`frozen`; kept verbatim — nothing
    /// consumes it yet.
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    pub effects: BiomeEffects,
}

/// The registry knowledge the client retained from the configuration
/// phase. `Default` is the honest empty table — a server that never sent
/// Registry Data yields no entries.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RegistryData {
    pub dimension_types: Vec<DimensionTypeEntry>,
    /// The `minecraft:worldgen/biome` entries in ARRIVAL ORDER — the
    /// index IS the network id the chunk section biome palettes
    /// reference (the vanilla synced-registry contract, the same order
    /// the `dimension_type_id` index already relies on through
    /// CommonPlayerSpawnInfo). The element's climate + effects colors
    /// are decoded (the tint foundation); the `attributes` compound is
    /// walked past — nothing consumes it yet.
    pub biomes: Vec<BiomeEntry>,
    /// The `minecraft:worldgen/biome` TAG contents from the Update
    /// Tags packet, tag name -> member entry ids (the same wire shape
    /// as `timeline_tags` — `TagNetworkSerialization$NetworkPayload`).
    pub biome_tags: Vec<(String, Vec<i32>)>,
    /// The `minecraft:world_clock` entries in ARRIVAL ORDER — the id
    /// table the Set Time clock states resolve through (javap: the Set
    /// Time map keys are `holderRegistry(WORLD_CLOCK)` VarInt ids; the
    /// built-in clocks are `minecraft:overworld` and
    /// `minecraft:the_end`). Names only: the element codec is UNIT
    /// (`WorldClock.DIRECT_CODEC = MapCodec.unitCodec`), so real
    /// entries ride WITHOUT a tag — nothing decodable beyond the name.
    pub world_clocks: Vec<String>,
    /// The `minecraft:timeline` entries in ARRIVAL ORDER (javap:
    /// `Registries.TIMELINE` = the "minecraft:timeline" registry key,
    /// in `RegistryDataLoader`'s synced list). The sky-light curve the
    /// renderer consumes resolves through this table + the tags below.
    pub timelines: Vec<TimelineEntry>,
    /// The `minecraft:timeline` TAG contents from the Update Tags
    /// packet — javap: Registry Data itself is ELEMENTS ONLY
    /// (`ClientboundRegistryDataPacket` is the record (registry,
    /// entries); the client's `handleUpdateTags` feeds a SEPARATE
    /// common-phase `ClientboundUpdateTagsPacket` whose map value is
    /// `TagNetworkSerialization$NetworkPayload` = a readMap of tag
    /// name -> VarInt id list). Tag name -> member entry ids.
    pub timeline_tags: Vec<(String, Vec<i32>)>,
}

impl RegistryData {
    /// The entry ids one timeline tag names (e.g.
    /// `minecraft:in_overworld` -> the day/moon/early_game ids on a
    /// real server) — `None` for a tag the server never sent.
    pub fn timeline_tag_ids(&self, tag: &str) -> Option<&[i32]> {
        self.timeline_tags
            .iter()
            .find(|(name, _)| name == tag)
            .map(|(_, ids)| ids.as_slice())
    }
    /// The vertical layout of one dimension, by registry key.
    pub fn dimension(&self, name: &str) -> Option<&DimensionTypeEntry> {
        self.dimension_types.iter().find(|d| d.name == name)
    }

    /// The registry name of one synced biome id (the palette values in
    /// a chunk section resolve through this) — `None` for an id the
    /// server never streamed (the honest "unknown biome").
    pub fn biome_name(&self, id: u32) -> Option<&str> {
        self.biomes.get(id as usize).map(|b| b.name.as_str())
    }

    /// One synced biome element, by network id — the tint path's
    /// lookup (palette id -> element -> effects colors).
    pub fn biome(&self, id: u32) -> Option<&BiomeEntry> {
        self.biomes.get(id as usize)
    }

    /// The entry ids one biome tag names (e.g. `minecraft:is_overworld`
    /// -> the plains/forest/... ids on a real server) — `None` for a
    /// tag the server never sent.
    pub fn biome_tag_ids(&self, tag: &str) -> Option<&[i32]> {
        self.biome_tags
            .iter()
            .find(|(name, _)| name == tag)
            .map(|(_, ids)| ids.as_slice())
    }

    /// The network id of one clock, by registry name — the reverse of
    /// the arrival-order table (the Set Time clock states carry ids;
    /// the dimension's `default_clock` carries a name).
    pub fn world_clock_id(&self, name: &str) -> Option<i32> {
        self.world_clocks
            .iter()
            .position(|n| n == name)
            .map(|i| i as i32)
    }

    /// Absorb one decoded Registry Data packet (vanilla streams one
    /// registry per packet; entries append in arrival order).
    fn merge(&mut self, other: RegistryData) {
        self.dimension_types.extend(other.dimension_types);
        self.biomes.extend(other.biomes);
        self.biome_tags.extend(other.biome_tags);
        self.world_clocks.extend(other.world_clocks);
        self.timelines.extend(other.timelines);
        self.timeline_tags.extend(other.timeline_tags);
    }
}

/// One pack identity the server offers and the client may claim to know:
/// the javap-proven 26.2 `KnownPack` record — THREE UTF STRINGS on the
/// wire (the version is a string like "26.2", NOT a VarInt).
#[derive(Debug, Clone, PartialEq)]
pub struct KnownPack {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

/// The pack-version claim — the ONE record the vanilla client sends when
/// the server offers Select Known Packs. The javap-proven 26.2 chain
/// (`tools/client.jar`):
/// - `BuiltInPackSource.<clinit>`: `CORE_PACK_INFO = KnownPack.vanilla("core")`
/// - `KnownPack.vanilla(String id)`:
///   `new KnownPack("minecraft", id, SharedConstants.getCurrentVersion().id())`
/// — so the record is `("minecraft", "core", WorldVersion.id())` and the
/// version string is the jar's own version.json `id` (generated:
/// [`generated::pack_claim::VERSION_ID`]), NOT the pack-format number.
///
/// The safety valve is the server's own comparison: a server whose pack
/// version differs from ours does not recognize the claim and streams its
/// registries anyway (the pre-claim behavior); a server that DOES skip the
/// sync agrees with us by construction — the built-in tables' wire ids are
/// the runtime-dumped vanilla registration order (v0.3.53's evidence) and
/// the built-in dimension layouts answer the world-height lookups
/// (v0.3.52's skip-sync fallback).
pub fn vanilla_known_packs() -> Vec<KnownPack> {
    vec![KnownPack {
        namespace: "minecraft".to_string(),
        id: "core".to_string(),
        version: crate::generated::pack_claim::VERSION_ID.to_string(),
    }]
}

/// The brand the vanilla server reports through its player list and logs
/// (the vanilla client sends "vanilla", modded clients their loader name).
pub const BRAND: &str = "mc2rs";

/// How the configuration phase ended.
#[derive(Debug, Clone, PartialEq)]
pub enum PlayOutcome {
    /// Finish Configuration acknowledged — the play state begins.
    Joined {
        /// The server-assigned profile UUID (equals the requested one in
        /// offline mode).
        uuid: [u8; 16],
        username: String,
        /// The registry knowledge retained during the phase (the
        /// dimension-type vertical layout table).
        registry: RegistryData,
        /// The skin model the login profile's `textures` property named
        /// (`Some("slim")` is the only client-visible value; `None` is
        /// the classic wide default — absent property, absent metadata,
        /// an unknown model value, or malformed input). See
        /// [`crate::login::profile_skin_model`].
        skin_model: Option<String>,
    },
    /// The server refused the login (Login Disconnect) or kicked the
    /// client during configuration (config Disconnect); carries the
    /// flattened chat-component reason.
    Disconnected { reason: String },
}

/// The vanilla-shaped client introduction (Client Information defaults a
/// fresh vanilla install sends: en_us locale, all skin parts, right hand).
pub struct ClientInfo {
    pub locale: String,
    /// Wire type: a single BYTE (vanilla writeByte).
    pub view_distance: i32,
    /// 0 = enabled, 1 = commands only, 2 = hidden.
    pub chat_mode: i32,
    pub chat_colors: bool,
    pub skin_parts: u8,
    /// 0 = left, 1 = right.
    pub main_hand: i32,
    pub disable_text_filtering: bool,
    pub allow_server_listings: bool,
    /// 0 = all, 1 = decreased, 2 = minimal — the ninth ClientInformation
    /// field the wiki-era shape never knew about.
    pub particle_status: i32,
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self {
            locale: "en_us".to_string(),
            view_distance: 8,
            chat_mode: 0,
            chat_colors: true,
            skin_parts: 0x7F,
            main_hand: 1,
            disable_text_filtering: false,
            allow_server_listings: true,
            particle_status: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// packet builders
// ---------------------------------------------------------------------------

/// Login Acknowledged (serverbound LOGIN 0x03): empty payload.
pub fn encode_login_acknowledged() -> Vec<u8> {
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, loginbound::LOGIN_ACKNOWLEDGED);
    p
}

/// Client Information (serverbound config 0x00), the javap-proven 26.2
/// field order and primitives: locale String, view distance BYTE, chat
/// mode VarInt enum, chat colors Bool, skin parts Byte, main hand VarInt
/// enum, text filtering Bool, server listings Bool, particle status
/// VarInt enum.
pub fn encode_client_information(info: &ClientInfo) -> Vec<u8> {
    let mut p = Vec::with_capacity(24);
    write_varint(&mut p, serverbound::CLIENT_INFORMATION);
    write_string(&mut p, &info.locale);
    p.push(info.view_distance as u8); // writeByte, NOT a varint
    write_varint(&mut p, info.chat_mode);
    p.push(u8::from(info.chat_colors));
    p.push(info.skin_parts);
    write_varint(&mut p, info.main_hand);
    p.push(u8::from(info.disable_text_filtering));
    p.push(u8::from(info.allow_server_listings));
    write_varint(&mut p, info.particle_status);
    p
}

/// Plugin Message (serverbound config 0x02) — the brand channel carries a
/// single UTF string payload.
pub fn encode_plugin_brand(brand: &str) -> Vec<u8> {
    let mut p = Vec::with_capacity(24 + brand.len());
    write_varint(&mut p, serverbound::PLUGIN_MESSAGE);
    write_string(&mut p, "minecraft:brand");
    // the brand payload is itself a length-prefixed string
    write_string(&mut p, brand);
    p
}

/// Keep Alive response (serverbound config 0x04): the i64 id verbatim.
pub fn encode_config_keep_alive_response(id: i64) -> Vec<u8> {
    let mut p = Vec::with_capacity(10);
    write_varint(&mut p, serverbound::KEEP_ALIVE);
    p.extend_from_slice(&id.to_be_bytes());
    p
}

/// Pong (serverbound config 0x05): the i32 id verbatim.
pub fn encode_config_pong(id: i32) -> Vec<u8> {
    let mut p = Vec::with_capacity(6);
    write_varint(&mut p, serverbound::PONG);
    p.extend_from_slice(&id.to_be_bytes());
    p
}

/// Acknowledge Finish Configuration (serverbound config 0x03): empty.
pub fn encode_config_ack() -> Vec<u8> {
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, serverbound::ACKNOWLEDGE_FINISH);
    p
}

/// Select Known Packs reply (serverbound config 0x07 — generated): the
/// javap-proven 26.2 shape — VarInt count (`ByteBufCodecs.list(64)`, the
/// vanilla server's own cap) + per pack THREE UTF strings.
pub fn encode_select_known_packs_reply(known: &[KnownPack]) -> Result<Vec<u8>> {
    anyhow::ensure!(
        known.len() <= 64,
        "{} known packs exceeds the vanilla serverbound cap of 64",
        known.len()
    );
    let mut p = Vec::with_capacity(2);
    write_varint(&mut p, serverbound::SELECT_KNOWN_PACKS);
    write_varint(&mut p, known.len() as i32);
    for k in known {
        write_string(&mut p, &k.namespace);
        write_string(&mut p, &k.id);
        write_string(&mut p, &k.version);
    }
    Ok(p)
}

/// Cookie Response (serverbound config 0x01 — generated): the javap-proven
/// 26.2 shape — Identifier key + a NULLABLE byte-array payload
/// (`ByteBufCodecs.byteArray(5120)` under the nullable). `None` is what
/// the vanilla client sends when it has no stored cookie for the key —
/// `Map.get` returning null encodes as Bool false — and the client
/// stores no server cookies, so the driver always passes `None`.
pub fn encode_cookie_response(key: &str, payload: Option<&[u8]>) -> Result<Vec<u8>> {
    if let Some(bytes) = payload {
        anyhow::ensure!(
            bytes.len() <= 5120,
            "cookie payload of {} bytes exceeds the vanilla cap of 5120",
            bytes.len()
        );
    }
    let mut p = Vec::with_capacity(4 + key.len());
    write_varint(&mut p, serverbound::COOKIE_RESPONSE);
    write_string(&mut p, key);
    match payload {
        None => p.push(0), // nullable absent — the vanilla no-cookie reply
        Some(bytes) => {
            p.push(1);
            write_varint(&mut p, bytes.len() as i32);
            p.extend_from_slice(bytes);
        }
    }
    Ok(p)
}

// ---------------------------------------------------------------------------
// packet parsers (full payload including the packet id)
// ---------------------------------------------------------------------------

/// Parse Keep Alive (clientbound config 0x04 — generated): i64 id.
pub fn parse_config_keep_alive(payload: &[u8]) -> Result<i64> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::KEEP_ALIVE,
        "not a config keep-alive packet (0x{id:02X})"
    );
    anyhow::ensure!(c.len() >= 8, "config keep-alive truncated");
    Ok(i64::from_be_bytes(c[..8].try_into()?))
}

/// Parse Ping (clientbound config 0x05 — generated): i32 id.
pub fn parse_config_ping(payload: &[u8]) -> Result<i32> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::PING, "not a config ping packet (0x{id:02X})");
    anyhow::ensure!(c.len() >= 4, "config ping truncated");
    Ok(i32::from_be_bytes(c[..4].try_into()?))
}

/// Parse Disconnect (clientbound config 0x02 — generated): chat JSON
/// reason, flattened to plain text (raw string kept when the JSON does
/// not parse).
pub fn parse_config_disconnect(payload: &[u8]) -> Result<String> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::DISCONNECT,
        "not a config disconnect packet (0x{id:02X})"
    );
    let reason = read_string(&mut c)?;
    let flat = serde_json::from_str::<serde_json::Value>(&reason)
        .map(|v| crate::status::extract_text(&v))
        .unwrap_or(reason);
    Ok(flat)
}

/// Parse Registry Data (clientbound config 0x07 — generated): the
/// javap-proven 26.2 shape — registry Identifier + VarInt entry count
/// (vanilla readCount max 256) + per entry { Identifier id, optional
/// network NBT tag }. `minecraft:dimension_type` and
/// `minecraft:worldgen/biome` are decoded (the entry tag IS the
/// element: min_y/height, resp. the NETWORK_CODEC climate + effects —
/// the name from the entry id); `minecraft:world_clock` and
/// `minecraft:timeline` likewise have their own shapes; every other
/// registry's entries are walked and dropped. Strict on the decoded
/// registries: an entry WITH a tag missing a required field, or a
/// tagless entry where the codec requires one, is a protocol drift
/// worth erroring on, the same way vanilla's own registry loader
/// refuses malformed data.
pub fn parse_registry_data(payload: &[u8]) -> Result<RegistryData> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::REGISTRY_DATA,
        "not a registry data packet (0x{id:02X})"
    );
    let registry_name = read_string(&mut c)?;
    let count = read_varint(&mut c)?;
    anyhow::ensure!(
        count <= 256,
        "registry {registry_name} carries {count} entries (vanilla max 256)"
    );
    let mut out = RegistryData::default();
    for _ in 0..count {
        let entry_id = read_string(&mut c)?;
        anyhow::ensure!(
            !c.is_empty(),
            "registry {registry_name} entry {entry_id} truncated (tag flag)"
        );
        let present = c[0] != 0;
        c = &c[1..];
        if registry_name == "minecraft:worldgen/biome" {
            // every biome entry occupies one id slot in arrival order;
            // the NETWORK_CODEC's climate + effects fields are required,
            // so a tagless entry would be the drift it is — error, like
            // the dimension and timeline elements. The optional
            // `attributes` compound (the 26.x home of sky/fog/music)
            // is walked past by simply not reading it.
            anyhow::ensure!(
                present,
                "biome {entry_id} rides without its element tag (the codec requires climate + effects)"
            );
            let element = read_network_nbt(&mut c)
                .with_context(|| format!("decode biome {entry_id} element nbt"))?;
            let has_precipitation = element
                .get("has_precipitation")
                .and_then(Nbt::as_i32)
                .with_context(|| format!("biome {entry_id} without has_precipitation"))?
                != 0;
            let temperature = element
                .get("temperature")
                .and_then(Nbt::as_f32)
                .with_context(|| format!("biome {entry_id} without temperature"))?;
            let temperature_modifier = element
                .get("temperature_modifier")
                .and_then(Nbt::as_str)
                .map(String::from);
            let downfall = element
                .get("downfall")
                .and_then(Nbt::as_f32)
                .with_context(|| format!("biome {entry_id} without downfall"))?;
            let effects = element
                .get("effects")
                .with_context(|| format!("biome {entry_id} without effects"))?;
            let effects = read_biome_effects(effects)
                .with_context(|| format!("biome {entry_id} carries a malformed effects compound"))?;
            out.biomes.push(BiomeEntry {
                name: entry_id,
                has_precipitation,
                temperature,
                temperature_modifier,
                downfall,
                effects,
            });
            continue;
        }
        if registry_name == "minecraft:world_clock" {
            // same arrival-order id table (the Set Time map keys are
            // these ids); the element codec is UNIT so real vanilla
            // entries ride tagless — a tag, if one ever rides, is
            // walked and skipped
            out.world_clocks.push(entry_id);
            if present {
                crate::play::skip_network_nbt(&mut c)
                    .with_context(|| format!("skip {registry_name} entry nbt"))?;
            }
            continue;
        }
        if registry_name == "minecraft:timeline" {
            // every timeline entry occupies one id slot in arrival order;
            // the element codec's `clock` is REQUIRED, so a tagless entry
            // would be the drift it is — error, like the dimension elements
            anyhow::ensure!(
                present,
                "timeline {entry_id} rides without its element tag (the codec requires `clock`)"
            );
            let element = read_network_nbt(&mut c)
                .with_context(|| format!("decode timeline {entry_id} element nbt"))?;
            let clock = element
                .get("clock")
                .and_then(Nbt::as_str)
                .with_context(|| format!("timeline {entry_id} without `clock`"))?
                .to_string();
            let period_ticks = element.get("period_ticks").and_then(Nbt::as_i32);
            let sky_light_factor = element
                .get("tracks")
                .and_then(|tracks| tracks.get("minecraft:visual/sky_light_factor"))
                .and_then(read_sky_light_track);
            out.timelines.push(TimelineEntry { name: entry_id, clock, period_ticks, sky_light_factor });
            continue;
        }
        if !present {
            // an entry without data: nothing to decode, honestly skipped
            continue;
        }
        if registry_name != "minecraft:dimension_type" {
            crate::play::skip_network_nbt(&mut c)
                .with_context(|| format!("skip {registry_name} entry {entry_id} nbt"))?;
            continue;
        }
        let element = read_network_nbt(&mut c)
            .with_context(|| format!("decode dimension_type {entry_id} element nbt"))?;
        let min_y = element
            .get("min_y")
            .and_then(Nbt::as_i32)
            .with_context(|| format!("dimension_type {entry_id} without min_y"))?;
        let height = element
            .get("height")
            .and_then(Nbt::as_i32)
            .with_context(|| format!("dimension_type {entry_id} without height"))?;
        // the day/night clock name: present for the overworld (and the
        // end), ABSENT for the nether — no day cycle there
        let default_clock = element.get("default_clock").and_then(Nbt::as_str).map(String::from);
        // the timelines reference, VERBATIM (the overworld's tag form
        // "#minecraft:in_overworld"; a direct id rides without the #)
        let timelines = element.get("timelines").and_then(Nbt::as_str).map(String::from);
        out.dimension_types.push(DimensionTypeEntry {
            name: entry_id,
            min_y,
            height,
            default_clock,
            timelines,
        });
    }
    anyhow::ensure!(
        c.is_empty(),
        "trailing bytes after the {registry_name} registry entries"
    );
    Ok(out)
}

/// The `visual/sky_light_factor` track's keyframes, (tick, value): the
/// track compound's required `keyframes` list of {int ticks, float
/// value}. `ease` (default LINEAR — the only easing the client's
/// evaluation implements) and `modifier` are walked past. A malformed
/// keyframe yields None — the honest no-curve, the same way an absent
/// track is no-curve (the model falls back to the compile-time day
/// curve; a half-decoded curve would be a guess).
fn read_sky_light_track(track: &Nbt) -> Option<Vec<(i64, f32)>> {
    let frames = track.get("keyframes")?.as_list()?;
    let mut out = Vec::with_capacity(frames.len());
    for frame in frames {
        let ticks = frame.get("ticks")?.as_i32()? as i64;
        let value = frame.get("value")?.as_f32()?;
        out.push((ticks, value));
    }
    Some(out)
}

/// The biome `effects` compound — javap `BiomeSpecialEffects.CODEC`:
/// required `water_color` int; the THREE optional color overrides
/// (`grass_color`, `foliage_color`, `dry_foliage_color`); optional
/// `grass_color_modifier` (absent ≡ "none", vanilla's own default —
/// an unknown name refuses the decode, like the vanilla enum codec).
fn read_biome_effects(effects: &Nbt) -> Option<BiomeEffects> {
    let water_color = effects.get("water_color")?.as_i32()? as u32;
    let color = |key: &str| effects.get(key).and_then(Nbt::as_i32).map(|v| v as u32);
    let grass_color_modifier = match effects.get("grass_color_modifier") {
        Some(name) => GrassColorModifier::from_wire(name.as_str()?)?,
        None => GrassColorModifier::None,
    };
    Some(BiomeEffects {
        water_color,
        grass_color: color("grass_color"),
        foliage_color: color("foliage_color"),
        dry_foliage_color: color("dry_foliage_color"),
        grass_color_modifier,
    })
}

/// Parse Update Tags (the common-phase clientbound packet — javap:
/// `ClientboundUpdateTagsPacket` = a readMap of { Identifier registry
/// name -> `TagNetworkSerialization$NetworkPayload` }, each payload a
/// readMap of { Identifier tag name -> VarInt id list }). Registry
/// Data is ELEMENTS ONLY — the tags ride THIS packet, and the
/// client's `handleUpdateTags` feeds them to the registry collector
/// alongside the elements. The `minecraft:timeline` AND
/// `minecraft:worldgen/biome` payloads are retained (the two
/// registries whose tags the client resolves references through —
/// the dimension's `timelines` reference, and the biome tag
/// membership the tint/tint-adjacent consumers resolve through);
/// every other registry's tags are walked past.
pub fn parse_update_tags(payload: &[u8]) -> Result<RegistryData> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(id == clientbound::UPDATE_TAGS, "not an update-tags packet (0x{id:02X})");
    let registry_count = read_varint(&mut c)?;
    anyhow::ensure!(
        registry_count <= 256,
        "update tags carries {registry_count} registries (past any sane flow)"
    );
    let mut out = RegistryData::default();
    for _ in 0..registry_count {
        let registry_name = read_string(&mut c)?;
        let tag_count = read_varint(&mut c)?;
        // the block/item tag maps number in the thousands on real
        // servers — anything past 65536 is the drift it is
        anyhow::ensure!(
            tag_count <= 65_536,
            "update tags registry {registry_name} carries {tag_count} tags"
        );
        for _ in 0..tag_count {
            let tag_name = read_string(&mut c)?;
            let value_count = read_varint(&mut c)?;
            anyhow::ensure!(
                value_count <= 65_536,
                "update tags tag {registry_name}:{tag_name} carries {value_count} values"
            );
            let mut ids = Vec::with_capacity(value_count.min(4096) as usize);
            for _ in 0..value_count {
                ids.push(read_varint(&mut c)?);
            }
            if registry_name == "minecraft:timeline" {
                out.timeline_tags.push((tag_name, ids));
            } else if registry_name == "minecraft:worldgen/biome" {
                out.biome_tags.push((tag_name, ids));
            }
        }
    }
    anyhow::ensure!(c.is_empty(), "trailing bytes after the update tags");
    Ok(out)
}

/// Parse Select Known Packs (clientbound config 0x0E — generated): the
/// javap-proven 26.2 shape — VarInt pack count + per pack THREE UTF
/// strings (namespace, id, version — the version is a STRING, not a
/// VarInt). Vanilla's clientbound list uses the default collection cap
/// (no small bound in the codec); real configuration flows carry a
/// handful, so anything past 1024 is treated as the drift it is rather
/// than allocated for.
pub fn parse_select_known_packs(payload: &[u8]) -> Result<Vec<KnownPack>> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::SELECT_KNOWN_PACKS,
        "not a select-known-packs packet (0x{id:02X})"
    );
    let count = read_varint(&mut c)?;
    anyhow::ensure!(
        (0..=1024).contains(&count),
        "select-known-packs carries {count} packs (past any sane flow)"
    );
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let namespace = read_string(&mut c)?;
        let id = read_string(&mut c)?;
        let version = read_string(&mut c)?;
        out.push(KnownPack { namespace, id, version });
    }
    anyhow::ensure!(c.is_empty(), "trailing bytes after the known packs");
    Ok(out)
}

/// Parse Cookie Request (clientbound config 0x00 — generated): the
/// javap-proven 26.2 shape — one Identifier key, nothing else.
pub fn parse_cookie_request(payload: &[u8]) -> Result<String> {
    let mut c: &[u8] = payload;
    let id = read_varint(&mut c)?;
    anyhow::ensure!(
        id == clientbound::COOKIE_REQUEST,
        "not a cookie request packet (0x{id:02X})"
    );
    let key = read_string(&mut c)?;
    anyhow::ensure!(c.is_empty(), "trailing bytes after the cookie key");
    Ok(key)
}

// ---------------------------------------------------------------------------
// the configuration driver
// ---------------------------------------------------------------------------

/// Answer the server's configuration probes until Finish Configuration is
/// acknowledged. Registry Data is decoded for the dimension-type vertical
/// layout (everything else in it — other registries — is skipped); Select
/// Known Packs gets the pack-version claim (the vanilla-shaped one-record
/// reply — a server that recognizes it skips the registry sync, one that
/// does not streams anyway) and Cookie Requests the no-cookie one (the
/// server BLOCKS otherwise); feature/resource-pack traffic is skipped
/// wholesale; a config Disconnect surfaces as `PlayOutcome::Disconnected`.
fn run_configuration<S: Read + Write>(
    conn: &mut PacketConn<S>,
    uuid: [u8; 16],
    username: String,
    skin_model: Option<String>,
) -> Result<PlayOutcome> {
    let mut registry = RegistryData::default();
    loop {
        let pkt = conn.read_packet().context("read configuration packet")?;
        anyhow::ensure!(!pkt.is_empty(), "empty configuration packet");
        let mut c: &[u8] = &pkt;
        let id = read_varint(&mut c)?;
        match id {
            clientbound::KEEP_ALIVE => {
                let probe = parse_config_keep_alive(&pkt)?;
                conn.send_packet(&encode_config_keep_alive_response(probe))?;
            }
            clientbound::PING => {
                let probe = parse_config_ping(&pkt)?;
                conn.send_packet(&encode_config_pong(probe))?;
            }
            clientbound::REGISTRY_DATA => {
                registry.merge(parse_registry_data(&pkt)?);
            }
            clientbound::UPDATE_TAGS => {
                // the common-phase tag payload the Registry Data
                // elements reference (the timeline tags among them)
                registry.merge(parse_update_tags(&pkt)?);
            }
            clientbound::SELECT_KNOWN_PACKS => {
                // validated, then answered with the pack-version claim:
                // the vanilla-shaped one-record reply ("minecraft",
                // "core", the jar's own version.json id — see
                // vanilla_known_packs). A server that recognizes the
                // claim skips the registry sync (our built-in tables
                // carry the wire ids + layouts); one that does not
                // streams its registries as before.
                parse_select_known_packs(&pkt)?;
                conn.send_packet(&encode_select_known_packs_reply(&vanilla_known_packs())?)?;
            }
            clientbound::COOKIE_REQUEST => {
                let key = parse_cookie_request(&pkt)?;
                // the vanilla client answers unconditionally; with no
                // stored cookie the payload is the absent one (Bool 0)
                conn.send_packet(&encode_cookie_response(&key, None)?)?;
            }
            clientbound::FINISH => {
                conn.send_packet(&encode_config_ack())?;
                return Ok(PlayOutcome::Joined { uuid, username, registry, skin_model });
            }
            clientbound::DISCONNECT => {
                let reason = parse_config_disconnect(&pkt)?;
                return Ok(PlayOutcome::Disconnected { reason });
            }
            // feature flags / resource packs / report details /
            // server links — read and skip
            _ => {}
        }
    }
}

/// Handshake + offline login + full configuration phase on an ALREADY
/// open packet connection (play.rs continues on the same wire after the
/// finish-configuration ack — the vanilla client never reopens the
/// socket between the phases).
pub fn join_play_conn<S: Read + Write>(
    conn: &mut PacketConn<S>,
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<PlayOutcome> {
    conn.send_packet(&encode_handshake(protocol_version, host, port, NEXT_STATE_LOGIN))?;
    let uuid = crate::login::offline_uuid(username);
    match login_conn(conn, username, &uuid)? {
        LoginOutcome::Disconnected { reason } => Ok(PlayOutcome::Disconnected { reason }),
        LoginOutcome::Success { properties, .. } => {
            // the profile's skin model (the textures property's metadata)
            // rides the session: the slim/wide selection the first-person
            // arm renders with. `None` = the classic wide default (the
            // offline shape, the malformed shape, every unknown value).
            let skin_model = crate::login::profile_skin_model(&properties);
            // switch both sides to configuration
            conn.send_packet(&encode_login_acknowledged())?;
            // the vanilla client's mandatory introductions
            conn.send_packet(&encode_client_information(&ClientInfo::default()))?;
            conn.send_packet(&encode_plugin_brand(BRAND))?;
            run_configuration(conn, uuid, username.to_string(), skin_model)
        }
    }
}

/// Handshake + offline login + full configuration phase on one connection
/// (the vanilla client never reopens the socket between the phases).
pub fn join_play<S: Read + Write>(
    stream: S,
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<PlayOutcome> {
    let mut conn = PacketConn::new(stream);
    join_play_conn(&mut conn, host, port, protocol_version, username)
}

/// Connect + full join (login + configuration) with the vanilla offline
/// profile UUID for `username`.
pub fn connect_and_join_play(
    host: &str,
    port: u16,
    protocol_version: i32,
    username: &str,
) -> Result<PlayOutcome> {
    let addr = format!("{host}:{port}");
    let timeout = std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS);
    let stream = std::net::TcpStream::connect(&addr).with_context(|| format!("tcp connect to {addr}"))?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    join_play(stream, host, port, protocol_version, username)
}

/// Shared e2e mock fixtures. The same-crate mock servers reuse these,
/// and downstream crates' mock servers do too (the GUI join e2e streams
/// the SAME registry bytes the mc-protocol arc mock does — one truth for
/// the mock registry, not hand-duplicated NBT in every mock).
pub mod test_support {
    use super::clientbound;
    use crate::nbt::Nbt;
    use crate::varint::{write_string, write_varint};

    /// A REAL `minecraft:worldgen/biome` registry data packet: ONE entry
    /// (the named biome, id 0 in the chunk biome palettes), carrying the
    /// REAL 26.2 `Biome.NETWORK_CODEC` element shape (climate + effects
    /// + an `attributes` compound the decode walks past). Mock servers
    /// that exercise biome music stream this alongside
    /// [`registry_packet`] — one truth for the mock registries, not
    /// hand-duplicated bytes in every mock.
    pub fn biome_registry_packet(biome: &str) -> Vec<u8> {
        biome_registry_packet_multi(&[biome])
    }

    /// The MULTI-entry form: a real vanilla server streams the WHOLE
    /// `minecraft:worldgen/biome` registry in one packet — both
    /// dimensions' biomes together, the ARRIVAL ORDER being the network
    /// id the chunk biome palettes reference. Mock servers that exercise
    /// dimension crossings stream two entries so the new dimension's
    /// chunks can carry a DIFFERENT biome id (the crossing re-fires the
    /// music only when the resolved name actually changes). The element
    /// is the real NETWORK_CODEC shape — required climate, the effects
    /// compound with its water color, and the optional `attributes`
    /// compound (the 26.x home of sky/fog/music) riding along to
    /// exercise the decode's walk-past.
    pub fn biome_registry_packet_multi(names: &[&str]) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REGISTRY_DATA);
        write_string(&mut p, "minecraft:worldgen/biome");
        write_varint(&mut p, names.len() as i32);
        for biome in names {
            write_string(&mut p, biome);
            p.push(1); // tag present — the codec requires the element
            crate::nbt::write_network_nbt(&real_biome_element(), &mut p)
                .expect("biome element writes");
        }
        p
    }

    /// A REAL registry data packet: the javap-proven 26.2 shape —
    /// registry Identifier + VarInt entry count + per entry { Identifier
    /// id, optional network NBT tag }. `dimension` selects between the
    /// decoded `minecraft:dimension_type` registry (overworld -64/384,
    /// nether 0/256 — the vanilla values; the entry tag IS the element)
    /// and the decoded `minecraft:worldgen/biome` registry (plains, the
    /// real NETWORK_CODEC element shape).
    pub fn registry_packet(dimension: bool) -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REGISTRY_DATA);
        if dimension {
            write_string(&mut p, "minecraft:dimension_type");
            write_varint(&mut p, 2); // two entries
            // entry 1: overworld — the optional tag IS the element
            // (the vanilla element carries default_clock: the overworld
            // clock name the Set Time states resolve through)
            write_string(&mut p, "minecraft:overworld");
            p.push(1); // tag present
            let ow = Nbt::Compound(vec![
                ("min_y".into(), Nbt::Int(-64)),
                ("height".into(), Nbt::Int(384)),
                ("has_skylight".into(), Nbt::Byte(1)),
                ("default_clock".into(), Nbt::String("minecraft:overworld".into())),
                // the dimension's timelines reference, VERBATIM from the
                // vanilla overworld element (the tag form)
                ("timelines".into(), Nbt::String("#minecraft:in_overworld".into())),
            ]);
            crate::nbt::write_network_nbt(&ow, &mut p).expect("overworld element writes");
            // entry 2: nether — NO default_clock (no day cycle), the
            // vanilla element's fixed_time instead
            write_string(&mut p, "minecraft:the_nether");
            p.push(1);
            let nether = Nbt::Compound(vec![
                ("min_y".into(), Nbt::Int(0)),
                ("height".into(), Nbt::Int(256)),
                ("fixed_time".into(), Nbt::Long(18_000)),
            ]);
            crate::nbt::write_network_nbt(&nether, &mut p).expect("nether element writes");
        } else {
            write_string(&mut p, "minecraft:worldgen/biome");
            write_varint(&mut p, 1); // one entry
            write_string(&mut p, "minecraft:plains");
            p.push(1); // tag present — the codec requires the element
            crate::nbt::write_network_nbt(&real_biome_element(), &mut p)
                .expect("biome element writes");
        }
        p
    }

    /// The REAL 26.2 biome element as the `Biome.NETWORK_CODEC` decodes
    /// it (the plains values): required climate (has_precipitation/
    /// temperature/downfall), the required effects compound (water_color
    /// 4159204 = 0x3F76E4 — the vanilla plains water), and the OPTIONAL
    /// attributes compound with the keys the real overworld biomes
    /// carry (`minecraft:visual/sky_color`, `minecraft:visual/fog_color`,
    /// `minecraft:audio/background_music` — the 26.x environment-
    /// attribute migration's home; the decode walks past them).
    fn real_biome_element() -> Nbt {
        Nbt::Compound(vec![
            ("has_precipitation".into(), Nbt::Byte(1)),
            ("temperature".into(), Nbt::Float(0.8)),
            ("downfall".into(), Nbt::Float(0.4)),
            (
                "effects".into(),
                Nbt::Compound(vec![("water_color".into(), Nbt::Int(4_159_204))]),
            ),
            (
                "attributes".into(),
                Nbt::Compound(vec![
                    ("minecraft:visual/sky_color".into(), Nbt::Int(7_907_327)),
                    ("minecraft:visual/fog_color".into(), Nbt::Int(12_638_463)),
                    (
                        "minecraft:audio/background_music".into(),
                        Nbt::Compound(vec![
                            ("sound".into(), Nbt::String("minecraft:music.game".into())),
                            ("min_delay".into(), Nbt::Int(12_000)),
                            ("max_delay".into(), Nbt::Int(24_000)),
                        ]),
                    ),
                ]),
            ),
        ])
    }

    /// The synced `minecraft:world_clock` registry as real vanilla
    /// streams it: BOTH built-in clocks in bootstrap order, entries
    /// TAGLESS — the element codec is UNIT
    /// (`WorldClock.DIRECT_CODEC = MapCodec.unitCodec`), so the wire
    /// carries names only and the arrival order IS the id table.
    pub fn world_clock_packet() -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REGISTRY_DATA);
        write_string(&mut p, "minecraft:world_clock");
        write_varint(&mut p, 2);
        write_string(&mut p, "minecraft:overworld");
        p.push(0); // no tag — the unit element
        write_string(&mut p, "minecraft:the_end");
        p.push(0);
        p
    }

    /// A `minecraft:timeline` Registry Data packet: the FOUR built-in
    /// overworld timelines in the in_overworld tag's id reality —
    /// `minecraft:day` (overworld clock, period 24000, the REAL
    /// day.json visual/sky_light_factor keyframes), `minecraft:moon`
    /// (period 192000, a moon_phase track only — no visual track),
    /// `minecraft:early_game` (NO period, a gameplay bool track),
    /// `minecraft:villager_schedule` (a gameplay track). The other
    /// tracks ride the shapes the real data uses (string keyframes,
    /// bool keyframes, an ease field) so the parse's walk-past
    /// discipline is exercised against honest shapes.
    pub fn timeline_packet() -> Vec<u8> {
        let day_kfs = Nbt::List(vec![
            Nbt::Compound(vec![("ticks".into(), Nbt::Int(730)), ("value".into(), Nbt::Float(1.0))]),
            Nbt::Compound(vec![("ticks".into(), Nbt::Int(11_270)), ("value".into(), Nbt::Float(1.0))]),
            Nbt::Compound(vec![("ticks".into(), Nbt::Int(13_140)), ("value".into(), Nbt::Float(0.24))]),
            Nbt::Compound(vec![("ticks".into(), Nbt::Int(22_860)), ("value".into(), Nbt::Float(0.24))]),
        ]);
        let day = Nbt::Compound(vec![
            ("clock".into(), Nbt::String("minecraft:overworld".into())),
            ("period_ticks".into(), Nbt::Int(24_000)),
            (
                "tracks".into(),
                Nbt::Compound(vec![(
                    "minecraft:visual/sky_light_factor".into(),
                    Nbt::Compound(vec![("keyframes".into(), day_kfs)]),
                )]),
            ),
        ]);
        // the moon timeline: a STRING-valued track (the real
        // moon_phase keyframes) — walked past, no visual track
        let moon = Nbt::Compound(vec![
            ("clock".into(), Nbt::String("minecraft:overworld".into())),
            ("period_ticks".into(), Nbt::Int(192_000)),
            (
                "tracks".into(),
                Nbt::Compound(vec![(
                    "minecraft:visual/moon_phase".into(),
                    Nbt::Compound(vec![(
                        "keyframes".into(),
                        Nbt::List(vec![
                            Nbt::Compound(vec![
                                ("ticks".into(), Nbt::Int(0)),
                                ("value".into(), Nbt::String("minecraft:full_moon".into())),
                            ]),
                            Nbt::Compound(vec![
                                ("ticks".into(), Nbt::Int(24000)),
                                ("value".into(), Nbt::String("minecraft:waning_gibbous".into())),
                            ]),
                        ]),
                    )]),
                )]),
            ),
        ]);
        // early_game: NO period, a BOOL track with an ease field — the
        // non-cyclic + walk-past shapes in one entry
        let early = Nbt::Compound(vec![
            ("clock".into(), Nbt::String("minecraft:overworld".into())),
            (
                "tracks".into(),
                Nbt::Compound(vec![(
                    "minecraft:gameplay/can_pillager_patrol_spawn".into(),
                    Nbt::Compound(vec![
                        ("ease".into(), Nbt::String("constant".into())),
                        (
                            "keyframes".into(),
                            Nbt::List(vec![
                                Nbt::Compound(vec![
                                    ("ticks".into(), Nbt::Int(0)),
                                    ("value".into(), Nbt::Byte(0)),
                                ]),
                                Nbt::Compound(vec![
                                    ("ticks".into(), Nbt::Int(120_000)),
                                    ("value".into(), Nbt::Byte(1)),
                                ]),
                            ]),
                        ),
                    ]),
                )]),
            ),
        ]);
        let villager = Nbt::Compound(vec![
            ("clock".into(), Nbt::String("minecraft:overworld".into())),
            ("period_ticks".into(), Nbt::Int(24_000)),
        ]);
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REGISTRY_DATA);
        write_string(&mut p, "minecraft:timeline");
        write_varint(&mut p, 4);
        for (name, element) in [
            ("minecraft:day", &day),
            ("minecraft:moon", &moon),
            ("minecraft:early_game", &early),
            ("minecraft:villager_schedule", &villager),
        ] {
            write_string(&mut p, name);
            p.push(1); // the element codec requires `clock` — tag present
            crate::nbt::write_network_nbt(element, &mut p).expect("timeline element writes");
        }
        p
    }

    /// An Update Tags packet: THREE registries — `minecraft:timeline`
    /// with the REAL built-in tag contents (in_overworld = the day/
    /// moon/early_game ids, universal = the villager_schedule id),
    /// `minecraft:worldgen/biome` whose tag the client RETAINS (the
    /// tint consumers' membership table), and a walked-past block tag
    /// registry (the shape a real server's huge block tag map rides
    /// in).
    pub fn update_tags_packet() -> Vec<u8> {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::UPDATE_TAGS);
        write_varint(&mut p, 3); // three registries
        // registry 1: the timeline tags the client retains
        write_string(&mut p, "minecraft:timeline");
        write_varint(&mut p, 2); // two tags
        write_string(&mut p, "minecraft:in_overworld");
        write_varint(&mut p, 3);
        write_varint(&mut p, 0); // day
        write_varint(&mut p, 1); // moon
        write_varint(&mut p, 2); // early_game
        write_string(&mut p, "minecraft:universal");
        write_varint(&mut p, 1);
        write_varint(&mut p, 3); // villager_schedule
        // registry 2: the biome tags the client retains
        write_string(&mut p, "minecraft:worldgen/biome");
        write_varint(&mut p, 1);
        write_string(&mut p, "minecraft:is_overworld");
        write_varint(&mut p, 1);
        write_varint(&mut p, 0); // plains
        // registry 3: walked past (nothing consumes block tags)
        write_string(&mut p, "minecraft:block");
        write_varint(&mut p, 1);
        write_string(&mut p, "minecraft:mineable/pickaxe");
        write_varint(&mut p, 1);
        write_varint(&mut p, 42);
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test_support::{registry_packet, timeline_packet, update_tags_packet, world_clock_packet};
    use crate::login::{encode_login_disconnect, encode_login_success, encode_set_compression};
    use crate::varint::write_varint;
    use std::net::{TcpListener, TcpStream};
    use std::thread::JoinHandle;

    #[test]
    fn packet_shapes() {
        assert_eq!(encode_login_acknowledged(), vec![0x03]);
        assert_eq!(encode_config_ack(), vec![0x03]);
        // client information: the javap-proven 26.2 shape — locale "en_us",
        // vd 8 (a BYTE, not a varint), chat 0, colors 1, parts 0x7F, hand
        // 1, filters 0, listings 1, particle status 0 (the ninth field)
        let p = encode_client_information(&ClientInfo::default());
        assert_eq!(p[0], 0x00);
        assert_eq!(p[1], 5, "string length prefix");
        assert_eq!(&p[2..7], b"en_us");
        assert_eq!(p[7], 8, "view distance is a raw byte");
        assert_eq!(
            &p[p.len() - 6..],
            &[0x01, 0x7F, 0x01, 0x00, 0x01, 0x00],
            "skin parts .. particle status"
        );
        // brand: channel + nested string payload
        let b = encode_plugin_brand("mc2rs");
        assert_eq!(b[0], 0x02);
        assert!(b.windows(15).any(|w| w == b"minecraft:brand"), "channel");
        assert!(b.windows(5).any(|w| w == b"mc2rs"), "nested payload");
        // keep alive / pong roundtrips
        assert_eq!(encode_config_keep_alive_response(0x0123456789ABCDEF), {
            let mut p = vec![0x04];
            p.extend_from_slice(&0x0123456789ABCDEFi64.to_be_bytes());
            p
        });
        // parse expects the CLIENTBOUND id (4 on the real 26.2 wire —
        // generated), so build that shape here
        let mut ka = Vec::new();
        write_varint(&mut ka, clientbound::KEEP_ALIVE);
        ka.extend_from_slice(&(-42i64).to_be_bytes());
        assert_eq!(parse_config_keep_alive(&ka).unwrap(), -42);
        // the real wire has the clientbound ping and the serverbound pong
        // at the SAME id (both 5, generated) — different flows, no wire
        // ambiguity; the parser distinguishes within ONE flow: a
        // keep-alive id (4) is rejected by the ping parser (5)
        assert_eq!(
            clientbound::PING,
            crate::generated::packet_ids::CSB_PONG,
            "cross-flow id coincidence, the real 26.2 wire"
        );
        assert!(parse_config_ping(&encode_config_keep_alive_response(777)).is_err());
        let ok = {
            let mut p = Vec::new();
            write_varint(&mut p, clientbound::PING);
            p.extend_from_slice(&777i32.to_be_bytes());
            p
        };
        assert_eq!(parse_config_ping(&ok).unwrap(), 777);
        // disconnect flattening
        let mut d = Vec::new();
        write_varint(&mut d, clientbound::DISCONNECT);
        write_string(&mut d, r#"{"text":"shut","extra":[{"text":"down"}]}"#);
        assert_eq!(parse_config_disconnect(&d).unwrap(), "shutdown");
    }

    /// Mock server: login (with compression) -> configuration -> play.
    /// Asserts the client's introductions and answers server-side, then
    /// returns the client-visible log of serverbound packet ids it read.
    fn spawn_join_mock() -> (u16, JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
        let port = listener.local_addr().unwrap().port();
        let handle = std::thread::spawn(move || {
            let (stream, _) = listener.accept().expect("accept");
            let mut srv = PacketConn::new(stream);
            // handshake
            let hs = srv.read_packet().expect("handshake");
            let mut c: &[u8] = &hs;
            assert_eq!(read_varint(&mut c).unwrap(), 0x00);
            assert_eq!(read_varint(&mut c).unwrap(), 776, "REAL protocol number");
            let _host = read_string(&mut c).unwrap();
            let mut port_b = [0u8; 2];
            std::io::Read::read_exact(&mut c, &mut port_b).unwrap();
            assert_eq!(read_varint(&mut c).unwrap(), NEXT_STATE_LOGIN);
            // login start FIRST (the client has not seen Set Compression
            // yet — it travels uncompressed), then the compression switch
            let start = srv.read_packet().expect("login start");
            let mut c: &[u8] = &start;
            assert_eq!(read_varint(&mut c).unwrap(), 0x00);
            let name = read_string(&mut c).unwrap();
            srv.send_packet(&encode_set_compression(256)).unwrap();
            srv.enable_compression(256);
            srv.send_packet(&encode_login_success(
                &crate::login::offline_uuid(&name),
                &name,
            ))
            .unwrap();
            let mut log = vec![format!("login:{name}")];
            // the introductions: login acknowledged (still login state),
            // client information, plugin brand
            for expected in ["ack:0x03", "info:0x00", "brand:0x02"] {
                let pkt = srv.read_packet().expect(expected);
                let mut c: &[u8] = &pkt;
                log.push(format!("{expected}:0x{:02X}", read_varint(&mut c).unwrap()));
            }
            // probes + bulk traffic + finish
            let mut ka = Vec::new();
            write_varint(&mut ka, clientbound::KEEP_ALIVE);
            ka.extend_from_slice(&0x1122334455667788u64.to_be_bytes());
            srv.send_packet(&ka).unwrap();
            let mut pg = Vec::new();
            write_varint(&mut pg, clientbound::PING);
            pg.extend_from_slice(&77i32.to_be_bytes());
            srv.send_packet(&pg).unwrap();
            // replies arrive inline in read order: keep-alive echo + pong
            let resp = srv.read_packet().expect("keep alive response");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::KEEP_ALIVE);
            assert_eq!(i64::from_be_bytes(c[..8].try_into().unwrap()), 0x1122334455667788);
            let resp = srv.read_packet().expect("pong");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::PONG);
            assert_eq!(i32::from_be_bytes(c[..4].try_into().unwrap()), 77);
            // the BLOCKING probes: select known packs (the vanilla server
            // waits for the reply before finishing configuration — the
            // vanilla-era skip would stall a real join for 30s) + a cookie
            // request (the vanilla client answers unconditionally)
            let mut skp = Vec::new();
            write_varint(&mut skp, clientbound::SELECT_KNOWN_PACKS);
            write_varint(&mut skp, 1); // one offered pack
            write_string(&mut skp, "minecraft");
            write_string(&mut skp, "core");
            write_string(&mut skp, "26.2");
            srv.send_packet(&skp).unwrap();
            let mut crq = Vec::new();
            write_varint(&mut crq, clientbound::COOKIE_REQUEST);
            write_string(&mut crq, "minecraft:join_cookie");
            srv.send_packet(&crq).unwrap();
            let resp = srv.read_packet().expect("select known packs reply");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::SELECT_KNOWN_PACKS);
            // the pack-version claim: the client echoes the offered pack
            // (the mock offers exactly what the vanilla client claims —
            // minecraft:core at the jar's own version.json id), which on
            // a real vanilla server is the skip-the-registry-sync trigger
            assert_eq!(read_varint(&mut c).unwrap(), 1, "the one-record claim");
            assert_eq!(read_string(&mut c).unwrap(), "minecraft", "the vanilla namespace");
            assert_eq!(read_string(&mut c).unwrap(), "core", "the vanilla pack id");
            assert_eq!(
                read_string(&mut c).unwrap(),
                "26.2",
                "the claimed version is the jar's own WorldVersion id"
            );
            assert!(c.is_empty(), "nothing after the one record");
            let resp = srv.read_packet().expect("cookie response");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::COOKIE_RESPONSE);
            assert_eq!(read_string(&mut c).unwrap(), "minecraft:join_cookie");
            assert_eq!(c, &[0x00], "no stored cookie -> the absent payload");
            // bulk registry traffic: the dimension_type registry is
            // DECODED by the client, a second registry is walked+skipped
            srv.send_packet(&registry_packet(true)).unwrap();
            srv.send_packet(&registry_packet(false)).unwrap();
            let mut fin = Vec::new();
            write_varint(&mut fin, clientbound::FINISH);
            srv.send_packet(&fin).unwrap();
            // the finish ack closes the phase
            let resp = srv.read_packet().expect("ack");
            let mut c: &[u8] = &resp;
            assert_eq!(read_varint(&mut c).unwrap(), serverbound::ACKNOWLEDGE_FINISH);
            assert!(c.is_empty(), "ack payload must be empty");
            log
        });
        (port, handle)
    }


    #[test]
    fn registry_data_decodes_dimensions_keeps_biomes_and_skips_other_registries() {
        // the decoded registry: both dimensions land with their layouts
        let data = parse_registry_data(&registry_packet(true)).expect("dimension_type parses");
        assert_eq!(data.dimension_types.len(), 2);
        let ow = data.dimension("minecraft:overworld").expect("overworld");
        assert_eq!((ow.min_y, ow.height), (-64, 384));
        assert_eq!(
            ow.default_clock.as_deref(),
            Some("minecraft:overworld"),
            "the element's default_clock name is retained"
        );
        assert_eq!(
            ow.timelines.as_deref(),
            Some("#minecraft:in_overworld"),
            "the element's timelines reference is retained VERBATIM (the tag form)"
        );
        let nether = data.dimension("minecraft:the_nether").expect("nether");
        assert_eq!((nether.min_y, nether.height), (0, 256));
        assert_eq!(
            nether.default_clock, None,
            "the nether has no day cycle — no default_clock on the element"
        );
        assert_eq!(nether.timelines, None, "the nether references no timelines either");
        assert!(data.dimension("minecraft:the_end").is_none());
        // the biome registry: the element DECODED (the NETWORK_CODEC
        // climate + effects — plains' real values), the arrival order
        // still the id table the chunk biome palettes resolve through,
        // the `attributes` compound walked past without an error
        let biomes = parse_registry_data(&registry_packet(false)).expect("biome parses");
        assert_eq!(biomes.biome_name(0), Some("minecraft:plains"));
        assert_eq!(biomes.biome_name(1), None, "an id the server never streamed");
        let plains = biomes.biome(0).expect("the one decoded element");
        assert!(plains.has_precipitation);
        assert_eq!(plains.temperature, 0.8);
        assert_eq!(plains.downfall, 0.4);
        assert_eq!(plains.temperature_modifier, None);
        assert_eq!(plains.effects.water_color, 4_159_204);
        assert_eq!(plains.effects.grass_color, None, "plains overrides nothing");
        assert_eq!(plains.effects.foliage_color, None);
        assert_eq!(plains.effects.dry_foliage_color, None);
        assert_eq!(plains.effects.grass_color_modifier, GrassColorModifier::None);
        assert!(biomes.dimension_types.is_empty(), "no dimension knowledge rides along");
        // the world_clock registry: the tagless unit entries land as the
        // arrival-order id table (the Set Time map keys resolve here)
        let clocks = parse_registry_data(&world_clock_packet()).expect("world_clock parses");
        assert_eq!(
            clocks.world_clocks,
            vec!["minecraft:overworld".to_string(), "minecraft:the_end".to_string()]
        );
        assert_eq!(clocks.world_clock_id("minecraft:overworld"), Some(0));
        assert_eq!(clocks.world_clock_id("minecraft:the_end"), Some(1));
        assert_eq!(clocks.world_clock_id("minecraft:nowhere"), None);
        assert!(clocks.biomes.is_empty() && clocks.dimension_types.is_empty());
        // the timeline registry: FOUR entries in arrival order, only
        // `minecraft:day` carrying the visual/sky_light_factor track —
        // the string/bool tracks and the ease field are walked past
        let timelines = parse_registry_data(&timeline_packet()).expect("timeline parses");
        assert_eq!(
            timelines.timelines.iter().map(|t| t.name.as_str()).collect::<Vec<_>>(),
            vec![
                "minecraft:day",
                "minecraft:moon",
                "minecraft:early_game",
                "minecraft:villager_schedule",
            ]
        );
        let day = &timelines.timelines[0];
        assert_eq!(day.clock, "minecraft:overworld");
        assert_eq!(day.period_ticks, Some(24_000));
        assert_eq!(
            day.sky_light_factor.as_deref(),
            Some(&[
                (730i64, 1.0f32),
                (11_270, 1.0),
                (13_140, 0.24),
                (22_860, 0.24),
            ][..]),
            "the REAL day.json keyframes decode"
        );
        let moon = &timelines.timelines[1];
        assert_eq!(moon.period_ticks, Some(192_000));
        assert_eq!(
            moon.sky_light_factor, None,
            "the moon's string-valued track is walked past, not misread"
        );
        assert_eq!(timelines.timelines[2].period_ticks, None, "early_game is non-cyclic");
        assert_eq!(timelines.timelines[2].sky_light_factor, None, "bool track walked past");
        assert_eq!(timelines.timelines[3].sky_light_factor, None);
        // a timeline entry WITHOUT its element tag is the drift it is —
        // the element codec requires `clock`
        let mut tagless = Vec::new();
        write_varint(&mut tagless, clientbound::REGISTRY_DATA);
        write_string(&mut tagless, "minecraft:timeline");
        write_varint(&mut tagless, 1);
        write_string(&mut tagless, "minecraft:day");
        tagless.push(0); // NO tag — required-element drift
        assert!(
            parse_registry_data(&tagless).is_err(),
            "a tagless timeline entry errors, never guesses"
        );
        // wrong packet id is rejected, not guessed
        let mut wrong = Vec::new();
        write_varint(&mut wrong, 0x99);
        assert!(parse_registry_data(&wrong).is_err());
        // a dimension entry WITH a tag but no min_y is a hard error
        let mut broken = Vec::new();
        write_varint(&mut broken, clientbound::REGISTRY_DATA);
        write_string(&mut broken, "minecraft:dimension_type");
        write_varint(&mut broken, 1);
        write_string(&mut broken, "minecraft:overworld");
        broken.push(1); // tag present
        let broken_tree = Nbt::Compound(vec![]);
        crate::nbt::write_network_nbt(&broken_tree, &mut broken).unwrap();
        assert!(parse_registry_data(&broken).is_err());
        // an entry with NO tag is skipped (nothing decodable, not drift)
        let mut absent = Vec::new();
        write_varint(&mut absent, clientbound::REGISTRY_DATA);
        write_string(&mut absent, "minecraft:dimension_type");
        write_varint(&mut absent, 2);
        write_string(&mut absent, "minecraft:overworld");
        absent.push(1);
        let ow = Nbt::Compound(vec![
            ("min_y".into(), Nbt::Int(-64)),
            ("height".into(), Nbt::Int(384)),
        ]);
        crate::nbt::write_network_nbt(&ow, &mut absent).unwrap();
        write_string(&mut absent, "minecraft:the_end");
        absent.push(0); // NO tag — the honest skip
        let data = parse_registry_data(&absent).expect("absent-tag entry skipped");
        assert_eq!(data.dimension_types.len(), 1, "only the tagged entry");
        assert_eq!(data.dimension("minecraft:overworld").unwrap().min_y, -64);
        // a truncated entry stream (count says 2, one entry) errors
        let mut short = Vec::new();
        write_varint(&mut short, clientbound::REGISTRY_DATA);
        write_string(&mut short, "minecraft:dimension_type");
        write_varint(&mut short, 2);
        write_string(&mut short, "minecraft:overworld");
        short.push(0);
        assert!(parse_registry_data(&short).is_err(), "count lies -> error");
    }

    #[test]
    fn biome_registries_retain_the_id_to_name_table_in_arrival_order() {
        // three FULL elements in arrival order (every id slot carries
        // the element its codec requires): the middle one is the real
        // swamp — the grass_color_modifier "swamp" decode rides along —
        // and all three resolve through their arrival-order ids
        let element = |modifier: Option<&str>| {
            let mut effects = vec![("water_color".into(), Nbt::Int(4_159_204))];
            if let Some(m) = modifier {
                effects.push(("grass_color_modifier".into(), Nbt::String(m.into())));
            }
            Nbt::Compound(vec![
                ("has_precipitation".into(), Nbt::Byte(1)),
                ("temperature".into(), Nbt::Float(0.8)),
                ("downfall".into(), Nbt::Float(0.4)),
                ("effects".into(), Nbt::Compound(effects)),
            ])
        };
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REGISTRY_DATA);
        write_string(&mut p, "minecraft:worldgen/biome");
        write_varint(&mut p, 3);
        for (name, modifier) in [
            ("minecraft:the_void", None),
            ("minecraft:swamp", Some("swamp")),
            ("minecraft:plains", None),
        ] {
            write_string(&mut p, name);
            p.push(1); // the codec requires the element
            crate::nbt::write_network_nbt(&element(modifier), &mut p).unwrap();
        }
        let data = parse_registry_data(&p).expect("three biomes parse");
        assert_eq!(data.biome_name(0), Some("minecraft:the_void"));
        assert_eq!(data.biome_name(1), Some("minecraft:swamp"));
        assert_eq!(data.biome_name(2), Some("minecraft:plains"));
        assert_eq!(data.biome_name(3), None, "past the streamed ids: unknown");
        assert_eq!(
            data.biome(1).expect("swamp").effects.grass_color_modifier,
            GrassColorModifier::Swamp,
            "the modifier rides the wire name and decodes"
        );
        // a biome entry with a MALFORMED tag is a hard error, not a
        // silent swallow — the walk must stay strict
        let mut broken = Vec::new();
        write_varint(&mut broken, clientbound::REGISTRY_DATA);
        write_string(&mut broken, "minecraft:worldgen/biome");
        write_varint(&mut broken, 1);
        write_string(&mut broken, "minecraft:swamp");
        broken.push(1);
        broken.extend_from_slice(&[0xFF, 0xFF]); // garbage tag bytes
        assert!(parse_registry_data(&broken).is_err(), "broken tag -> error");
        // a TAGLESS entry is now the drift it is (the codec requires
        // climate + effects — vanilla's loader refuses it too)
        let mut bare = Vec::new();
        write_varint(&mut bare, clientbound::REGISTRY_DATA);
        write_string(&mut bare, "minecraft:worldgen/biome");
        write_varint(&mut bare, 1);
        write_string(&mut bare, "minecraft:plains");
        bare.push(0);
        assert!(parse_registry_data(&bare).is_err(), "tagless element -> error");
        // a LYING count (two entries, one sent) errors the same way
        let mut short = Vec::new();
        write_varint(&mut short, clientbound::REGISTRY_DATA);
        write_string(&mut short, "minecraft:worldgen/biome");
        write_varint(&mut short, 2);
        write_string(&mut short, "minecraft:plains");
        short.push(0);
        assert!(parse_registry_data(&short).is_err(), "count lies -> error");
    }

    #[test]
    fn biome_effects_decode_the_overrides_and_refuse_the_drift() {
        // the FULL effects surface: all three optional overrides + the
        // dark_forest modifier (the real dark_forest biome's verdict)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::REGISTRY_DATA);
        write_string(&mut p, "minecraft:worldgen/biome");
        write_varint(&mut p, 1);
        write_string(&mut p, "minecraft:dark_forest");
        p.push(1);
        let element = Nbt::Compound(vec![
            ("has_precipitation".into(), Nbt::Byte(1)),
            ("temperature".into(), Nbt::Float(0.7)),
            ("downfall".into(), Nbt::Float(0.8)),
            (
                "effects".into(),
                Nbt::Compound(vec![
                    ("water_color".into(), Nbt::Int(4_159_204)),
                    ("grass_color".into(), Nbt::Int(5_066_409)),
                    ("foliage_color".into(), Nbt::Int(5_066_409)),
                    ("dry_foliage_color".into(), Nbt::Int(4_683_620)),
                    ("grass_color_modifier".into(), Nbt::String("dark_forest".into())),
                ]),
            ),
        ]);
        crate::nbt::write_network_nbt(&element, &mut p).unwrap();
        let data = parse_registry_data(&p).expect("dark_forest parses");
        let effects = &data.biome(0).expect("dark_forest").effects;
        assert_eq!(effects.water_color, 4_159_204);
        assert_eq!(effects.grass_color, Some(5_066_409));
        assert_eq!(effects.foliage_color, Some(5_066_409));
        assert_eq!(effects.dry_foliage_color, Some(4_683_620));
        assert_eq!(effects.grass_color_modifier, GrassColorModifier::DarkForest);
        // the "none" name decodes like the absence does (vanilla's
        // optionalFieldOf default — the SAME verdict)
        let none_named = {
            let mut p = Vec::new();
            write_varint(&mut p, clientbound::REGISTRY_DATA);
            write_string(&mut p, "minecraft:worldgen/biome");
            write_varint(&mut p, 1);
            write_string(&mut p, "minecraft:plains");
            p.push(1);
            let element = Nbt::Compound(vec![
                ("has_precipitation".into(), Nbt::Byte(0)),
                ("temperature".into(), Nbt::Float(0.5)),
                ("downfall".into(), Nbt::Float(0.5)),
                (
                    "effects".into(),
                    Nbt::Compound(vec![
                        ("water_color".into(), Nbt::Int(4_159_204)),
                        ("grass_color_modifier".into(), Nbt::String("none".into())),
                    ]),
                ),
            ]);
            crate::nbt::write_network_nbt(&element, &mut p).unwrap();
            parse_registry_data(&p).expect("none-named parses")
        };
        assert_eq!(
            none_named.biome(0).expect("plains").effects.grass_color_modifier,
            GrassColorModifier::None
        );
        // the DRIFTS, each refused like vanilla's own loader refuses
        // them: an unknown modifier name, a missing water_color, a
        // missing effects compound, a missing climate field
        let build = |effects: Nbt, climate: bool| {
            let mut fields = vec![("effects".into(), effects)];
            if climate {
                fields.push(("has_precipitation".into(), Nbt::Byte(1)));
                fields.push(("temperature".into(), Nbt::Float(0.5)));
                fields.push(("downfall".into(), Nbt::Float(0.5)));
            }
            let mut p = Vec::new();
            write_varint(&mut p, clientbound::REGISTRY_DATA);
            write_string(&mut p, "minecraft:worldgen/biome");
            write_varint(&mut p, 1);
            write_string(&mut p, "minecraft:swamp");
            p.push(1);
            crate::nbt::write_network_nbt(&Nbt::Compound(fields), &mut p).unwrap();
            p
        };
        let full = || {
            Nbt::Compound(vec![
                ("water_color".into(), Nbt::Int(4_159_204)),
                ("grass_color_modifier".into(), Nbt::String("swamp".into())),
            ])
        };
        let mut unknown = full();
        if let Nbt::Compound(fields) = &mut unknown {
            fields[1].1 = Nbt::String("modded_whatever".into());
        }
        for (name, effects) in [
            ("unknown modifier", unknown),
            ("no water_color", Nbt::Compound(vec![])),
        ] {
            assert!(
                parse_registry_data(&build(effects, true)).is_err(),
                "{name} -> the drift it is"
            );
        }
        let no_effects = Nbt::Compound(vec![("water_color".into(), Nbt::Int(1))]);
        assert!(
            parse_registry_data(&build(no_effects, false)).is_err(),
            "missing effects + climate -> error"
        );
    }

    #[test]
    fn configuration_reaches_the_play_state() {
        let (port, srv) = spawn_join_mock();
        let stream = TcpStream::connect(("127.0.0.1", port)).expect("connect");
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        stream
            .set_write_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let out = join_play(stream, "127.0.0.1", port, 776, "mc2rs").expect("join play");
        match out {
            PlayOutcome::Joined { username, uuid, registry } => {
                assert_eq!(username, "mc2rs");
                assert_eq!(uuid, crate::login::offline_uuid("mc2rs"));
                // the registry knowledge survived the phase: both
                // dimension layouts decoded from the streamed NBT
                let ow = registry.dimension("minecraft:overworld").expect("overworld");
                assert_eq!((ow.min_y, ow.height), (-64, 384));
                let nether = registry.dimension("minecraft:the_nether").expect("nether");
                assert_eq!((nether.min_y, nether.height), (0, 256));
                // the biome registry rode the same phase: the id -> name
                // table survives into the play state (the merge path),
                // the element's effects decoded with it
                assert_eq!(registry.biome_name(0), Some("minecraft:plains"));
                assert_eq!(
                    registry.biome(0).expect("plains").effects.water_color,
                    4_159_204
                );
            }
            other => panic!("expected Joined, got {other:?}"),
        }
        let log = srv.join().unwrap();
        assert_eq!(log.len(), 4, "full vanilla introduction order: {log:?}");
        assert_eq!(log[0], "login:mc2rs");
        assert_eq!(log[1], "ack:0x03:0x03");
        assert_eq!(log[2], "info:0x00:0x00");
        assert_eq!(log[3], "brand:0x02:0x02");
    }

    #[test]
    fn update_tags_narrow_the_timelines_and_walk_the_rest() {
        // the common-phase tag payload: the timeline AND biome tags are
        // retained, the block registry's payload is walked past
        let tags = parse_update_tags(&update_tags_packet()).expect("update tags parse");
        assert_eq!(
            tags.timeline_tags,
            vec![
                ("minecraft:in_overworld".to_string(), vec![0, 1, 2]),
                ("minecraft:universal".to_string(), vec![3]),
            ]
        );
        assert_eq!(
            tags.timeline_tag_ids("minecraft:in_overworld"),
            Some(&[0, 1, 2][..]),
            "the tag the overworld dimension's `timelines` reference names"
        );
        assert_eq!(tags.timeline_tag_ids("minecraft:in_the_void"), None);
        assert!(tags.timelines.is_empty(), "tags carry names+ids, never elements");
        // the biome tags are retained the same way (the tint consumers'
        // membership table); the block registry's payload is walked past
        assert_eq!(
            tags.biome_tags,
            vec![("minecraft:is_overworld".to_string(), vec![0])]
        );
        assert_eq!(
            tags.biome_tag_ids("minecraft:is_overworld"),
            Some(&[0][..]),
            "the plains id the fixture's biome table resolves"
        );
        assert_eq!(tags.biome_tag_ids("minecraft:is_forest"), None);
        assert!(tags.biomes.is_empty(), "tags carry names+ids, never elements");
        // wrong packet id is rejected, not guessed
        let mut wrong = Vec::new();
        write_varint(&mut wrong, 0x99);
        assert!(parse_update_tags(&wrong).is_err());
        // a truncated tag stream (value count lies) errors
        let mut short = Vec::new();
        write_varint(&mut short, clientbound::UPDATE_TAGS);
        write_varint(&mut short, 1);
        write_string(&mut short, "minecraft:timeline");
        write_varint(&mut short, 1);
        write_string(&mut short, "minecraft:in_overworld");
        write_varint(&mut short, 5); // says five values...
        write_varint(&mut short, 0);
        assert!(parse_update_tags(&short).is_err(), "...and delivers one");
        // the merged flow: registry packets AND update tags land in ONE
        // RegistryData (the run_configuration accumulation)
        let mut merged = parse_registry_data(&timeline_packet()).expect("timelines");
        merged.merge(parse_update_tags(&update_tags_packet()).expect("tags"));
        assert_eq!(merged.timelines.len(), 4);
        assert_eq!(merged.timeline_tags.len(), 2);
        // the full chain the model resolves through: tag -> ids ->
        // arrival-order names
        let names: Vec<&str> = merged
            .timeline_tag_ids("minecraft:in_overworld")
            .expect("tag")
            .iter()
            .filter_map(|&id| merged.timelines.get(id as usize))
            .map(|t| t.name.as_str())
            .collect();
        assert_eq!(names, vec!["minecraft:day", "minecraft:moon", "minecraft:early_game"]);
        // the biome knowledge merges the same way: the element table AND
        // the tag payload accumulate into one RegistryData
        assert_eq!(merged.biome_tags, vec![("minecraft:is_overworld".to_string(), vec![0])]);
        assert!(merged.biomes.is_empty(), "this flow streamed no biome elements");
        let mut with_biomes = parse_registry_data(&registry_packet(false)).expect("biomes");
        with_biomes.merge(merged);
        assert_eq!(with_biomes.biome_name(0), Some("minecraft:plains"));
        assert_eq!(
            with_biomes.biome_tag_ids("minecraft:is_overworld"),
            Some(&[0][..]),
            "tag -> ids -> arrival-order names, for biomes too"
        );
    }

    #[test]
    fn select_known_packs_and_cookies_roundtrip_with_the_proven_bounds() {
        // clientbound select known packs: two packs, THREE strings each
        // (the version is a string, not a VarInt — javap-proven)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SELECT_KNOWN_PACKS);
        write_varint(&mut p, 2);
        write_string(&mut p, "minecraft");
        write_string(&mut p, "core");
        write_string(&mut p, "26.2");
        write_string(&mut p, "example_mod");
        write_string(&mut p, "example");
        write_string(&mut p, "7");
        let packs = parse_select_known_packs(&p).expect("packs parse");
        assert_eq!(
            packs,
            vec![
                KnownPack {
                    namespace: "minecraft".into(),
                    id: "core".into(),
                    version: "26.2".into()
                },
                KnownPack {
                    namespace: "example_mod".into(),
                    id: "example".into(),
                    version: "7".into()
                }
            ]
        );
        // drift guards: wrong id, lying count, trailing bytes
        let mut wrong = Vec::new();
        write_varint(&mut wrong, 0x77);
        assert!(parse_select_known_packs(&wrong).is_err());
        let mut liar = Vec::new();
        write_varint(&mut liar, clientbound::SELECT_KNOWN_PACKS);
        write_varint(&mut liar, 3);
        write_string(&mut liar, "minecraft");
        write_string(&mut liar, "core");
        write_string(&mut liar, "26.2");
        assert!(parse_select_known_packs(&liar).is_err(), "count lies -> error");
        let mut trailing = p.clone();
        trailing.push(0xFF);
        assert!(parse_select_known_packs(&trailing).is_err());
        // the encoder primitive also encodes the empty list (id 7 + 0) —
        // the shape a server whose pack version we do not match still gets
        // nothing about (the claim is ONE record, not a blanket always)
        assert_eq!(encode_select_known_packs_reply(&[]).unwrap(), vec![0x07, 0x00]);
        // a non-empty reply is the same three-string shape
        let reply = encode_select_known_packs_reply(&packs).unwrap();
        assert_eq!(reply[0], serverbound::SELECT_KNOWN_PACKS as u8);
        assert_eq!(reply[1], 2, "pack count");
        assert!(reply.windows(4).any(|w| w == b"26.2"), "version is a plain string");
        // the vanilla serverbound cap of 64 is enforced
        let mut too_many = Vec::new();
        for _ in 0..65 {
            too_many.push(KnownPack {
                namespace: "minecraft".into(),
                id: "core".into(),
                version: "26.2".into(),
            });
        }
        assert!(encode_select_known_packs_reply(&too_many).is_err());
        // cookie request: one Identifier key, nothing else
        let mut crq = Vec::new();
        write_varint(&mut crq, clientbound::COOKIE_REQUEST);
        write_string(&mut crq, "minecraft:join_cookie");
        assert_eq!(parse_cookie_request(&crq).unwrap(), "minecraft:join_cookie");
        crq.push(0x01);
        assert!(parse_cookie_request(&crq).is_err(), "trailing byte -> error");
        let mut wrong_key = Vec::new();
        write_varint(&mut wrong_key, 0x66);
        write_string(&mut wrong_key, "minecraft:nope");
        assert!(parse_cookie_request(&wrong_key).is_err());
        // cookie response: the key echoed + the absent payload (Bool 0) —
        // exactly the vanilla client's no-cookie answer
        assert_eq!(
            encode_cookie_response("minecraft:join_cookie", None).unwrap(),
            {
                let mut p = vec![0x01];
                write_string(&mut p, "minecraft:join_cookie");
                p.push(0);
                p
            }
        );
        // a payload rides the nullable form; 5120 is the vanilla cap
        let with_payload = encode_cookie_response("minecraft:join_cookie", Some(&[9, 9])).unwrap();
        assert_eq!(
            &with_payload[with_payload.len() - 4..],
            &[0x01, 0x02, 9, 9],
            "present flag, length, bytes"
        );
        let mut big = vec![0u8; 5121];
        big[5120] = 1;
        assert!(encode_cookie_response("minecraft:join_cookie", Some(&big)).is_err());
    }

    #[test]
    fn the_pack_version_claim_is_the_vanilla_one_record_reply() {
        // the javap-proven chain: BuiltInPackSource.<clinit> binds
        // CORE_PACK_INFO = KnownPack.vanilla("core"), and vanilla(id) is
        // new KnownPack("minecraft", id, WorldVersion.id()) — the version
        // string is the jar's version.json id, generated into
        // pack_claim.rs, NOT the pack-format number
        let claim = vanilla_known_packs();
        assert_eq!(claim.len(), 1, "the vanilla client claims exactly ONE pack");
        let k = &claim[0];
        assert_eq!(k.namespace, "minecraft", "the vanilla namespace");
        assert_eq!(k.id, "core", "the vanilla core pack id");
        assert!(
            !k.version.is_empty(),
            "a non-empty reply IS the claim — the empty list could never trigger the skip"
        );
        assert_eq!(
            k.version,
            crate::generated::pack_claim::VERSION_ID,
            "the claimed version IS the generated WorldVersion id"
        );
        assert_eq!(
            k.version,
            crate::generated::pack_claim::VERSION_NAME,
            "26.2's id and name coincide (the generated surface pins both)"
        );
        // the generated manifest surface itself: the values the real jar's
        // version.json states (regenerating must reproduce these)
        assert_eq!(crate::generated::pack_claim::WORLD_VERSION, 4903);
        assert_eq!(crate::generated::pack_claim::PROTOCOL_VERSION, 776);
        assert_eq!(
            (
                crate::generated::pack_claim::PACK_VERSION_RESOURCE_MAJOR,
                crate::generated::pack_claim::PACK_VERSION_RESOURCE_MINOR,
                crate::generated::pack_claim::PACK_VERSION_DATA_MAJOR,
                crate::generated::pack_claim::PACK_VERSION_DATA_MINOR
            ),
            (88, 0, 107, 1),
            "the four-field 26.2 pack_version schema — NOT the claim string"
        );
        // the claim encodes to the same wire shape the e2e mock asserts
        let reply = encode_select_known_packs_reply(&claim).unwrap();
        assert_eq!(reply[0], serverbound::SELECT_KNOWN_PACKS as u8);
        assert_eq!(reply[1], 1, "one record");
        let mut echo = Vec::new();
        write_varint(&mut echo, serverbound::SELECT_KNOWN_PACKS);
        write_varint(&mut echo, 1);
        write_string(&mut echo, "minecraft");
        write_string(&mut echo, "core");
        write_string(&mut echo, "26.2");
        assert_eq!(
            reply, echo,
            "the claim replies with EXACTLY the pack a real vanilla server offers"
        );
    }

    #[test]
    fn configuration_disconnect_surfaces_the_reason() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let srv = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut conn = PacketConn::new(stream);
            let _hs = conn.read_packet().unwrap();
            let _start = conn.read_packet().unwrap();
            conn.send_packet(&encode_set_compression(256)).unwrap();
            conn.enable_compression(256);
            conn.send_packet(&encode_login_success(&[9u8; 16], "mc2rs")).unwrap();
            // read the introductions, then kick during configuration
            for _ in 0..3 {
                conn.read_packet().unwrap();
            }
            let mut d = Vec::new();
            write_varint(&mut d, clientbound::DISCONNECT);
            write_string(&mut d, r#"{"text":"You are ","extra":[{"text":"banned","color":"red"}]}"#);
            conn.send_packet(&d).unwrap();
        });
        let stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let out = join_play(stream, "127.0.0.1", port, 776, "mc2rs").expect("join flow");
        assert_eq!(
            out,
            PlayOutcome::Disconnected { reason: "You are banned".to_string() }
        );
        srv.join().unwrap();
    }

    #[test]
    fn login_refusal_surfaces_through_join_play() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let srv = std::thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut conn = PacketConn::new(stream);
            let _hs = conn.read_packet().unwrap();
            let _start = conn.read_packet().unwrap();
            conn.send_packet(&encode_login_disconnect(r#"{"text":"invalid session"}"#))
                .unwrap();
        });
        let stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let out = join_play(stream, "127.0.0.1", port, 776, "mc2rs").expect("join flow");
        assert_eq!(out, PlayOutcome::Disconnected { reason: "invalid session".to_string() });
        srv.join().unwrap();
    }

    #[test]
    fn connect_and_join_play_end_to_end() {
        let (port, srv) = spawn_join_mock();
        let out = connect_and_join_play("127.0.0.1", port, 776, "Steve").expect("connect");
        assert!(matches!(out, PlayOutcome::Joined { ref username, .. } if username == "Steve"));
        srv.join().unwrap();
    }

    #[test]
    fn pre_play_ids_are_the_generated_runtime_facts() {
        // The configuration/login tables are GENERATED from the official
        // jar's own bootstrapped protocols (`mc2rs-gen dump-packet-ids`
        // binds LoginProtocols + ConfigurationProtocols + the rest and
        // dumps the live dispatch codecs' id maps). These pins come from
        // the real 26.2 runtime dump — the wiki-era constants they
        // replaced had EVERY clientbound config id one low (registry
        // data was 0x06 there, 0x07 on the real wire): a real join would
        // have failed to recognize any of these packets, and the e2e
        // mocks were self-consistent with the old table and masked it.
        use crate::generated::packet_ids as gen;
        // clientbound configuration: the +1 drift wave
        assert_eq!(clientbound::DISCONNECT, gen::CCB_DISCONNECT);
        assert_eq!(clientbound::DISCONNECT, 2, "disconnect — the wiki-era 0x01 was one low");
        assert_eq!(clientbound::FINISH, 3, "finish_configuration — wiki-era 0x02");
        assert_eq!(clientbound::KEEP_ALIVE, 4, "wiki-era 0x03");
        assert_eq!(clientbound::PING, 5, "wiki-era 0x04");
        assert_eq!(clientbound::REGISTRY_DATA, 7, "registry_data — wiki-era 0x06");
        // the answered configuration probes (this run's javap proofs)
        assert_eq!(clientbound::SELECT_KNOWN_PACKS, 14, "minecraft:select_known_packs");
        assert_eq!(clientbound::COOKIE_REQUEST, 0, "minecraft:cookie_request — the flow head");
        assert_eq!(serverbound::SELECT_KNOWN_PACKS, 7, "minecraft:select_known_packs");
        assert_eq!(serverbound::COOKIE_RESPONSE, 1, "minecraft:cookie_response");
        // serverbound configuration: numerically stable, names moved
        assert_eq!(serverbound::CLIENT_INFORMATION, 0);
        assert_eq!(serverbound::PLUGIN_MESSAGE, 2, "minecraft:custom_payload");
        assert_eq!(serverbound::ACKNOWLEDGE_FINISH, 3, "minecraft:finish_configuration");
        assert_eq!(serverbound::KEEP_ALIVE, 4);
        assert_eq!(serverbound::PONG, 5);
        // login acknowledged lives in the LOGIN serverbound flow
        assert_eq!(loginbound::LOGIN_ACKNOWLEDGED, gen::LSB_LOGIN_ACKNOWLEDGED);
        assert_eq!(loginbound::LOGIN_ACKNOWLEDGED, 3);
        // the full flows are dense: the last id is the entry count - 1
        for (flow, count) in [
            (&gen::CONFIG_CLIENTBOUND, 20),
            (&gen::CONFIG_SERVERBOUND, 10),
            (&gen::LOGIN_CLIENTBOUND, 6),
            (&gen::LOGIN_SERVERBOUND, 5),
            (&gen::STATUS_CLIENTBOUND, 2),
            (&gen::STATUS_SERVERBOUND, 2),
            (&gen::HANDSHAKE_SERVERBOUND, 1),
        ] {
            assert_eq!(flow.last().unwrap().0, (flow.len() - 1) as u32, "dense flow");
            assert_eq!(flow.len(), count, "flow size pinned from the 26.2 dump");
        }
        // the status/handshake facts the framers now reference
        assert_eq!(gen::TCB_STATUS_RESPONSE, 0);
        assert_eq!(gen::TSB_STATUS_REQUEST, 0);
        assert_eq!(gen::HSB_INTENTION, 0);
        assert_eq!(gen::LSB_HELLO, 0, "login start is hello");
    }
}

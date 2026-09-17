//! World view — the bridge from the network world model to the renderer.
//!
//! `materialize` projects a window of the client's `WorldModel` (chunk
//! columns with resolved global state ids) into the voxel grid the mesher
//! and raster already consume (`world::World`). `materialize_snapshot_view`
//! is the frame-loop entry the join worker calls: the world-load
//! milestone's snapshot adopts into a fresh model and the player-centered
//! window lands on the world screen — the first world VIEW from network
//! chunks. The stream is LIVE: `LiveView` keeps feeding the SAME model
//! for the session's whole life (re-sent chunks, teleports, probes) and
//! every refresh re-anchors the window on the latest pose.
//!
//! Documented decisions (all deliberate, no silent guesses):
//! - The global-state-id -> runtime-palette mapping is a REQUIRED caller
//!   parameter. The bytecode-true numeric id table now EXISTS — GENERATED
//!   by `mc2rs-gen dump-block-states` from the official jar's real
//!   bootstrapped registry (mc-registry's block_ids) — and `runtime_mapper`
//!   is the GUI's policy through it; tests inject other mappers freely.
//!   Nothing bakes in remembered constants.
//! - The vertical copy is PLAYER-ANCHORED when the dimension layout is
//!   known: the join teleport's feet land at grid y [`PLAYER_GRID_Y`]
//!   (`snapshot_y_base` — min_y + player.y, clamped at the world floor),
//!   so a real overworld join shows the surface the player actually
//!   stands on instead of the first 48 blocks of deepslate above bedrock
//!   (the bottom-anchored copy a live min_y made WRONG: world y -64..-16
//!   is deep underground, the player at y 64 far outside the window).
//!   Without layout knowledge world coordinates are meaningless, so the
//!   copy stays column-local (the deterministic bottom anchor) — never a
//!   guess. The top is deliberately unclamped: a player high above the
//!   terrain truthfully sees sky in a window that follows them, not a
//!   surface the code guessed.
//! - The materialized view ships the player's CAMERA too: the join
//!   teleport's exact pose (feet + the vanilla standing eye height, the
//!   server-directed look direction converted from the vanilla angle
//!   conventions) in the grid frame — the world screen renders FROM the
//!   player, not from a stand-in vantage. Without dimension layout
//!   knowledge there is no pose (the grid is column-local then and the
//!   player's world y means nothing in it) — the deterministic flight
//!   camera stands instead; never a guess.
//! - Columns missing from the model stay AIR (the vanilla client would
//!   still be streaming them — holes are the truthful render).

use crate::camera::Camera;
use crate::math::Vec3;
use crate::world::{
    BiomeLayer, LightGrid, World, AIR, ANDESITE, BEDROCK, BIOME_UNKNOWN, CHUNK_X, CHUNK_Y, CHUNK_Z,
    COAL_ORE, COBBLED_DEEPSLATE, COPPER_ORE, DEEPSLATE, DIAMOND_ORE, DIORITE, DIRT, EMERALD_ORE,
    GOLD_ORE, GRANITE, GRASS, GRAVEL, IRON_ORE, LAPIS_ORE, LEAF_LITTER, LEAVES, LIGHT_UNKNOWN, LOG,
    NETHERRACK, PALE_LEAVES, REDSTONE_ORE, SAND, STONE, WATER,
};
use crate::world_model::{USE_ROW_SLOT_END, USE_ROW_SLOT_START, WorldModel};
use mc_protocol::{PlayEvent, PlayerPosition, WorldSnapshot};

/// Project the window of `side * side` chunks starting at chunk
/// `(origin.0, origin.1)` into a fresh renderer grid. `y_base` is the
/// vertical anchor: the y coordinate grid row 0 represents — WORLD-y when
/// the model's `min_y` is known (the snapshot path passes
/// [`snapshot_y_base`]), column-local y when it is not (pass 0; the only
/// meaningful copy then). `mapper` converts every resolved global state
/// id (air included) to the runtime palette; blocks outside a column's
/// vertical extent and columns the server has not sent (yet) stay air.
/// The view also carries the model's LIGHT — sky and block separately,
/// every cell in a sent column's extent (air included), the honest
/// [`LIGHT_UNKNOWN`] where the engine never answered — the raw material
/// the mesher's per-corner smooth lighting consumes. The BIOME layer
/// rides the same window: per-voxel registry ids through the model's
/// proven biome queries (the honest [`BIOME_UNKNOWN`] where it never
/// answers) plus the model's tint quads (the synced registry, or the
/// built-in wire-order tables when no sync arrived) — the raw material
/// the mesher's tint fold consumes.
pub fn materialize(
    model: &WorldModel,
    origin: (i32, i32),
    side: usize,
    y_base: i32,
    mut mapper: impl FnMut(u32) -> u16,
) -> World {
    // the biome layer rides the SAME window: the tint quads resolve
    // through the model's accessor (the synced registry's elements, or
    // the built-in wire-order tables when no sync arrived — the
    // skip-sync fallback) and the per-voxel ids copy during the walk
    // below; the layer lands on the world after it
    let tints: Vec<[Option<u32>; 4]> = model.biome_tints();
    let mut biome_ids = vec![vec![BIOME_UNKNOWN; CHUNK_X * CHUNK_Y * CHUNK_Z]; side * side];
    let mut world = World {
        chunks_per_side: side,
        seed: 0,
        chunks: vec![vec![AIR; CHUNK_X * CHUNK_Y * CHUNK_Z]; side * side],
        light: Some(LightGrid {
            sky: vec![vec![LIGHT_UNKNOWN; CHUNK_X * CHUNK_Y * CHUNK_Z]; side * side],
            block: vec![vec![LIGHT_UNKNOWN; CHUNK_X * CHUNK_Y * CHUNK_Z]; side * side],
        }),
        // the clock's day tick rides the SAME snapshot: the model's
        // Set Time state decides day or night; no clock yet = None, the
        // honest day
        day_tick: model.day_tick(),
        // filled after the walk (the ids copy needs the mutable local)
        biomes: None,
        // the per-cell STATE-VARIANT grid rides the SAME window (the 14th
        // audit): decoded from the raw global state id through the
        // generated per-state property table; zeros where the block
        // carries no variant (the mesher's no-opinion look)
        variants: Some(vec![vec![0u8; CHUNK_X * CHUNK_Y * CHUNK_Z]; side * side]),
        // the lightmap's AmbientColor base rides the SAME snapshot: the
        // model's dimension resolves through the built-in
        // visual/ambient_light_color table; None = no dimension pinned (or
        // an unknown custom one) — the bytecode-default black
        ambient_light: model.ambient_light_color(),
        // the lightmap's BlockLightTint rides the SAME snapshot: the
        // model's dimension resolves through the built-in
        // visual/block_light_tint column; None = no dimension pinned (or
        // an unknown custom one) — the mesher reads the bytecode default
        // #FFD88C (the corrected blue 140)
        block_light_tint: model.block_light_tint(),
        // the stated sky-light factor rides the SAME snapshot (the
        // attribute-system fold's multiply-base): the model's dimension
        // resolves through the built-in visual/sky_light_factor column —
        // the real jar's end/nether carry Some(0.0), the overworlds
        // None (the mesher's multiply identity)
        sky_light_factor: model.sky_light_factor(),
        // the stated sky-light color rides the SAME snapshot (the
        // lightmap family's transport): the model's dimension resolves
        // through the built-in visual/sky_light_color column — the real
        // jar's end (#ac60cd) / nether (#7a7aff) carry their moonlight
        // bases, the overworlds None (the mesher's white identity)
        sky_light_color: model.sky_light_color(),
        // the stated night-vision color rides the SAME snapshot (the
        // lightmap family's transport): the model's dimension resolves
        // through the built-in visual/night_vision_color column — no
        // vanilla dimension states it (all four rows None), the mesher
        // reads the bytecode default #999999 gray UNDER the night-vision
        // factor (the player's effect state, 0.0 at this client)
        night_vision_color: model.night_vision_color(),
        // the night-vision FACTOR rides the SAME snapshot (the effect
        // state's transport): the model's own-effect store — the
        // Update/Remove Mob Effect packets' filtered truth — answers
        // 0.0 with the effect absent (every golden scene) and the 1.0
        // plateau with it riding
        night_vision_factor: model.night_vision_factor(),
    };
    let side_i = side as i32;
    for gz in 0..side_i {
        for gx in 0..side_i {
            let (cx, cz) = (origin.0 + gx, origin.1 + gz);
            let Some(_col) = model.column(cx, cz) else {
                continue; // unsent column -> the grid's fresh air + unknown light
            };
            let grid = &mut world.chunks[(gx + gz * side_i) as usize];
            let variants = world.variants.as_mut().expect("just built");
            // the caller anchors the copy; the model answers column-local
            // y when min_y is unknown and world-y when it is — querying
            // through block() keeps ONE source of truth for the index
            // conventions
            for gy in 0..CHUNK_Y as i32 {
                let wy = y_base + gy;
                for lz in 0..CHUNK_Z as i32 {
                    for lx in 0..CHUNK_X as i32 {
                        let wx = cx * CHUNK_X as i32 + lx;
                        let wz = cz * CHUNK_Z as i32 + lz;
                        let Some(id) = model.block(wx, wy, wz) else {
                            continue; // above/below the column's extent -> air
                        };
                        let mapped = mapper(id);
                        if mapped != AIR {
                            grid[lx as usize + lz as usize * CHUNK_X + gy as usize * CHUNK_X * CHUNK_Z] =
                                mapped;
                        }
                        // the variant rides the SAME walk (the 14th audit):
                        // the state's properties, decoded once here — the
                        // mesher reads the byte, never the wire id
                        let variant = leaf_litter_variant(id);
                        if variant != 0 {
                            variants[(gx + gz * side_i) as usize]
                                [lx as usize + lz as usize * CHUNK_X + gy as usize * CHUNK_X * CHUNK_Z] =
                                variant;
                        }
                    }
                }
            }
            // the light layer rides the SAME walk — every cell in the
            // column's extent (air INCLUDED: the mesher's face cells are
            // usually air), the model's honest None left as LIGHT_UNKNOWN —
            // and the biome ids ride it too, expanded from the wire's
            // 4×4×4 cell grids through the model's proven queries
            let li = (gx + gz * side_i) as usize;
            for gy in 0..CHUNK_Y as i32 {
                let wy = y_base + gy;
                for lz in 0..CHUNK_Z as i32 {
                    for lx in 0..CHUNK_X as i32 {
                        let wx = cx * CHUNK_X as i32 + lx;
                        let wz = cz * CHUNK_Z as i32 + lz;
                        let idx =
                            lx as usize + lz as usize * CHUNK_X + gy as usize * CHUNK_X * CHUNK_Z;
                        let light = world.light.as_mut().expect("just built");
                        light.sky[li][idx] = model.sky_light(wx, wy, wz).unwrap_or(LIGHT_UNKNOWN);
                        light.block[li][idx] = model.block_light(wx, wy, wz).unwrap_or(LIGHT_UNKNOWN);
                        biome_ids[li][idx] = model.biome(wx, wy, wz).unwrap_or(BIOME_UNKNOWN);
                    }
                }
            }
        }
    }
    world.biomes = Some(BiomeLayer { ids: biome_ids, tints });
    world
}

/// The window origin that centers the PLAYER's chunk: a vanilla server
/// streams chunks around the player, so the view window anchors on the
/// chunk the player stands in (`floor(x) >> 4` — the floor before the
/// arithmetic shift keeps the negative half-plane honest). With an odd
/// side the player's chunk is the exact window center; the world screen's
/// camera scans the window's center column.
pub fn snapshot_window_origin(player: &PlayerPosition, side: usize) -> (i32, i32) {
    let half = (side / 2) as i32;
    let pcx = (player.x.floor() as i32) >> 4;
    let pcz = (player.z.floor() as i32) >> 4;
    (pcx - half, pcz - half)
}

/// The grid y the player's feet anchor to: a third of the window up —
/// 16 rows of world below the feet, sky above (the vanilla spawn spirit:
/// ground in view, room to see the sky).
pub const PLAYER_GRID_Y: i32 = crate::world::CHUNK_Y as i32 / 3;

/// The vanilla standing eye height over the feet — the same offset the
/// procedural flight camera adds over its scanned surface (one constant,
/// two users: every first-person eye sits 1.62 above its feet).
pub const PLAYER_EYE_HEIGHT: f32 = 1.62;

/// The vertical anchor of the snapshot window: the world-y that grid row
/// 0 will represent. With the dimension layout known (`min_y`), the
/// window anchors on the PLAYER — the join teleport's feet land at grid
/// y [`PLAYER_GRID_Y`], clamped at the world floor so a spawn near the
/// bottom never wastes rows on void (the feet slide up the grid
/// instead). Without layout knowledge world coordinates are meaningless:
/// the deterministic column-local copy (0) stands. The top is
/// deliberately unclamped — see the module notes.
pub fn snapshot_y_base(player: &PlayerPosition, min_y: Option<i32>) -> i32 {
    let Some(min_y) = min_y else { return 0 };
    (player.y.floor() as i32 - PLAYER_GRID_Y).max(min_y)
}

/// The materialized join view: the renderer grid AND the camera pose
/// computed from the SAME window decisions (origin, y_base, layout
/// knowledge) — the world screen renders the grid from the pose and the
/// two can never disagree about where the player stands. The view also
/// carries the biome AT THE PLAYER's feet (the synced registry name) —
/// the situation key the biome music resolves through.
pub struct SnapshotView {
    pub world: World,
    /// The teleport's exact pose in the grid frame, or `None` without
    /// dimension layout knowledge (see [`snapshot_camera`]).
    pub camera: Option<Camera>,
    /// The biome at the player's feet (full registry id, e.g.
    /// "minecraft:plains"), or `None` without a pose / outside the
    /// loaded columns / for an id the server never named — the honest
    /// unknown, never a guess.
    pub biome: Option<String>,
    /// The OPEN foreign container window's UI-facing projection — the
    /// chest's live slots the networked screen renders (see
    /// [`LiveContainer`]). `None` while no foreign window is open.
    pub container: Option<LiveContainer>,
    /// The store's SELECTED hotbar stack as `(item_id, count)` — the
    /// first-person hand's own lane (the vanilla renderItemInHand's
    /// projection: the selected slot's stack, the count the GUI's
    /// business). The emptiness (a count-0 slot, or a slot the store
    /// never spoke) is `None` — the bare arm renders.
    pub held: Option<(i32, i32)>,
    /// The store's WHOLE hotbar row as `(item_id, count)` pairs — the
    /// HUD's own lane (the vanilla renderHotbar's projection: the nine
    /// slots the hotbar sprite carries, the icons and the counts drawn
    /// over it). The row is `USE_ROW_SLOT_START..USE_ROW_SLOT_END`
    /// verbatim (the inventory window's own numbering, NOT re-ordered
    /// by the selection — the selection is the highlight's business),
    /// and a slot the store never spoke stays `(0, 0)` — the wire's
    /// own emptiness the HUD skips, never a guessed stack.
    pub hotbar: [(i32, i32); 9],
    /// The LOCAL player's vitals — the health row's own projection
    /// (see [`HealthView`]).
    pub health: HealthView,
    /// The wire's MOB population — the Add Entity store's own
    /// projection through the synced entity-type registry (see
    /// [`WireMob`]). Zombie-only v1: the type ids the registry names
    /// `minecraft:zombie` render through the mob lane's own avatar
    /// model; every other type id stays honest data the screen
    /// doesn't draw yet.
    pub mobs: Vec<WireMob>,
    /// The wire's BOAT population (see [`WireBoat`]) — the boat-family
    /// entities the registry named, the hulls the render lane draws.
    /// The empty list the honest silence (no boat the server spawned),
    /// the shape every lane rides.
    pub boats: Vec<WireBoat>,
    /// The wire mobs' PENDING VOICES — the Entity Sound mailbox the
    /// model resolved (see [`WireVoice`]): the sounds the SERVER
    /// played for its mobs, shipped once with the refresh that
    /// carried them (the worker's own drain empties the mailbox
    /// after — each voice rides exactly one ship).
    pub voices: Vec<WireVoice>,
    /// The world's PENDING VOICES — the Sound mailbox the model
    /// resolved (see [`WireWorldVoice`]): the GLOBAL sounds the
    /// SERVER played at a world spot, shipped once with the refresh
    /// that carried them (the worker's own drain empties the mailbox
    /// after — each voice rides exactly one ship).
    pub world_voices: Vec<WireWorldVoice>,
    /// The SIDEBAR slot's own projection (see [`WireSidebar`]) — the
    /// scoreboard the wire trio keeps, sorted and capped the display
    /// law keeps. `None` while the slot is silent (no display set, or
    /// its objective gone) — the HUD renders nothing, the honest
    /// absence.
    pub sidebar: Option<WireSidebar>,
    /// The TAB overlay's own projection (see [`WirePlayerList`]) —
    /// the listed rows the player-info store keeps, sorted and
    /// capped the comparator law keeps, plus the header/footer texts.
    /// The EMPTY overlay (no rows, no bands) is the honest silence —
    /// the join's default, and the shape the TAB key's gate renders
    /// when the server never spoke.
    pub player_list: WirePlayerList,
    /// THE RIDE ANCHOR (see [`WireRide`]) — the carried position the
    /// vehicle's own snapshot keeps (the boat family only), `None`
    /// while nothing carries the local player (the walk keeps the
    /// body's own physics — the ride's silence IS the freedom).
    pub ride: Option<WireRide>,
    /// The player's OWN inventory window (the window-0 store)
    /// projected for the UI — the full 46-slot shape in the wire's
    /// own numbering (result 0, craft 1..5, armor 5..9, the main
    /// grid 9..36, the hotbar 36..45, the offhand 45), keyed by the
    /// window's OWN menu id (the index IS the id). A slot the store
    /// never spoke stays `(0, 0)` — the wire's own emptiness the
    /// render skips, never a guessed stack.
    pub inventory_slots: [(i32, i32); INVENTORY_MENU_SLOTS],
    /// The window-0 store's OWN bundle rows (the set-slot sync's
    /// projection the chest lane shares) — the inventory screen's
    /// scroll lane's feeder, keyed by the window's own menu id.
    pub inventory_bundles: Vec<(i32, Vec<(i32, i32)>)>,
    /// The window-0 store's revision (the `state_id` the server's
    /// Set Content / Set Slot packets keep) — the click lane's echo
    /// input (the ServerboundContainerClickPacket's own stateId field
    /// the vanilla client fills from its menu's last-seen revision).
    pub inventory_state_id: i32,
    /// The window-0 store's GUI cursor (the Set Content packet's
    /// carriedItem — the stack riding the mouse) — the click lane's
    /// believed cursor the prediction walks speak.
    pub inventory_carried: (i32, i32),
}

/// One wire mob's played voice — the Entity Sound packet's own
/// landing (the vanilla `ClientLevel.handleEntitySound`'s data): the
/// sounding entity's id (the mob lane's match key), the sound event's
/// NAME resolved through the synced sound_event registry (the
/// verbatim registry name — the audio edge strips its namespace
/// prefix), the volume and the pitch verbatim.
#[derive(Debug, Clone, PartialEq)]
pub struct WireVoice {
    pub entity_id: i32,
    pub name: String,
    pub volume: f32,
    pub pitch: f32,
}

/// One wire WORLD voice — the Sound packet's own landing (the global
/// channel the entity voices' sibling answers): the sound event's
/// NAME resolved through the synced sound_event registry (the
/// verbatim registry name — the audio edge strips its namespace
/// prefix), the WORLD SPOT the packet carried (the ear's own
/// distance law folds the gain from it), the volume and the pitch
/// verbatim.
#[derive(Debug, Clone, PartialEq)]
pub struct WireWorldVoice {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub volume: f32,
    pub pitch: f32,
}

/// The DisplaySlot id the sidebar renders from (the
/// `extractScoreboardSidebar` walk's own slot — the player-team
/// color slots the extract reads first never arm in the house's
/// laneless belief).
pub const SIDEBAR_SLOT: u8 = 1;

/// The sidebar's own row cap (the `displayScoreboardSidebar` walk's
/// `limit(15L)` — the stream's own Long speaks it).
pub const SIDEBAR_ROW_CAP: usize = 15;

/// One sidebar row — the row's NAME (the display override the Set
/// Score packet carried, else the holder's own string — the
/// ownerName walk) and the value.
#[derive(Debug, Clone, PartialEq)]
pub struct WireSidebarLine {
    pub name: String,
    pub score: i32,
}

/// The SIDEBAR slot's own projection (the Hud
/// extractScoreboardSidebar's stand-in): the objective's flattened
/// display as the title, the rows the display law keeps — the
/// non-hidden holders sorted value-DESCENDING with the owner's
/// case-insensitive tiebreak (the SCORE_DISPLAY_ORDER comparator),
/// capped at 15 (the stream's own limit).
#[derive(Debug, Clone, PartialEq)]
pub struct WireSidebar {
    pub title: String,
    pub lines: Vec<WireSidebarLine>,
}

/// The TAB overlay's own row cap (the `getPlayerInfos` walk's
/// `limit(80L)` — the stream's own Long the constant pool speaks).
pub const PLAYER_ROW_CAP: usize = 80;

/// One TAB row as the renderer consumes it (the vanilla
/// `PlayerTabOverlay.PlayerInfo` walk's own words): the profile name
/// the ADD_PLAYER carried, the game mode's ordinal verbatim (the 3
/// the spectator dim speaks), the latency milliseconds (the bar
/// lanes' law reads — the icon lane the house's no-sprite render
/// keeps honest data for), and the list order (the sort's first
/// key). The LIST-objective score rides when the scoreboard speaks
/// for the player's NAME (the ScoreHolder.fromGameProfile law — the
/// players' rows score by name, the entities' by stringUUID);
/// `None` = no row on the list objective.
#[derive(Debug, Clone, PartialEq)]
pub struct WirePlayerRow {
    pub name: String,
    pub game_mode: u8,
    pub latency: i32,
    pub list_order: i32,
    pub score: Option<i32>,
}

/// The TAB overlay's own projection (the Hud extractTabList →
/// PlayerTabOverlay.extractRenderState's stand-in): the header and
/// footer texts (empty = the no-band silence the vanilla nulls
/// keep), and the rows the display law keeps — the LISTED entries
/// sorted by (list order ASC, spectator LAST, profile name ASC —
/// the PLAYER_COMPARATOR's own chain, the house's no-team lane the
/// team-name tiebreak skips), capped at 80.
#[derive(Debug, Clone, PartialEq)]
pub struct WirePlayerList {
    pub header: String,
    pub footer: String,
    pub rows: Vec<WirePlayerRow>,
}

/// THE RIDE ANCHOR (the vanilla rideTick's own word — the position the
/// vehicle carries the LOCAL player at): the vehicle's world-space
/// position plus the passenger attachment (the
/// AbstractBoat.getPassengerAttachmentPoint walk), and the vehicle's
/// yaw the mount snap reads (the handleSetEntityPassengersPacket's own
/// yRotOF/setYRot/setYHeadRot fold). The camera rides the feet plus
/// the eye height; the body's own pose rides the feet verbatim.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WireRide {
    pub feet_x: f64,
    pub feet_y: f64,
    pub feet_z: f64,
    /// The vehicle's body yaw in degrees (the mount snap's source).
    pub vehicle_yaw: f32,
}

/// THE BOAT FAMILY'S OWN DIMENSIONS (the vanilla EntityType constants
/// the client bakes: BOAT / CHEST_BOAT / RAFT / CHEST_RAFT all scale
/// 1.375 wide × 0.562 tall — the registry never ships dimensions, the
/// type's own constant the vanilla source keeps). The ride height the
/// AbstractBoat.rideHeight override speaks: `dimensions.height() / 3`
/// (the Boat/ChestBoat/Raft/ChestRaft bytecode's own fdiv).
pub const BOAT_DIMENSIONS_HEIGHT: f32 = 0.562;

/// The multi-rider seat offsets the AbstractBoat
/// getPassengerAttachmentPoint bytecode keeps: the first seat of a
/// MULTI-rider list sits at +0.2, every later seat at −0.6 (the
/// single-rider boat's own x offset is 0.0 — the
/// getSinglePassengerXOffset base the Boat/Raft family never
/// overrides). Animal passengers add +0.2 more — a branch the local
/// player can never ride (the rider's own seat law only).
pub const BOAT_FRONT_SEAT_OFFSET: f32 = 0.2;
pub const BOAT_BACK_SEAT_OFFSET: f32 = -0.6;

/// THE RIDE PROJECTION: the LOCAL player's carried position, from the
/// Set Passengers belief + the vehicle's own entity snapshot. The
/// chain (each link the honest None — no belief, no vehicle entity,
/// no boat family, no seat): the Login's entity id -> the vehicle
/// that carries it -> the vehicle's snapshot -> the type name's boat
/// family gate (`boat` / `chest_boat` / `raft` / `chest_raft` — the
/// registry names the ends-with walk keeps, the 26.2 boat family's
/// own four) -> the attachment: `Vec3(0, height/3, seat_x).yRot(
/// -yaw·π/180)` added to the vehicle's position (the
/// getPassengerAttachmentPoint's own return, the yRot the vanilla
/// Vec3's own cos/sin fold).
pub fn ride_view(model: &WorldModel) -> Option<WireRide> {
    let info = model.info.as_ref()?;
    let vehicle_id = model.vehicle_of(info.entity_id)?;
    let vehicle = model.entity(vehicle_id)?;
    let name = model.entity_type_name(vehicle.type_id)?;
    let is_boat_family = name.ends_with("boat") || name.ends_with("raft");
    if !is_boat_family {
        return None;
    }
    let ride_height = BOAT_DIMENSIONS_HEIGHT / 3.0;
    let seat_x = match model.passenger_index(info.entity_id) {
        Some(0) => {
            // the front seat: +0.2 only when the boat carries MORE than
            // one (the size>1 branch the bytecode gates; the single
            // rider keeps the 0.0 base)
            match model.vehicle_passengers(vehicle_id) {
                Some(riders) if riders.len() > 1 => BOAT_FRONT_SEAT_OFFSET,
                _ => 0.0,
            }
        }
        Some(_) => BOAT_BACK_SEAT_OFFSET,
        None => 0.0,
    };
    // the vanilla Vec3.yRot(-yaw * π/180) walk: x' = x·cos + z·sin,
    // z' = −x·sin + z·cos — the attachment's local x rides the seat,
    // the y the ride height
    let theta = -(vehicle.y_rot as f64) * std::f64::consts::PI / 180.0;
    let (sin, cos) = theta.sin_cos();
    let (lx, lz) = (seat_x as f64, 0.0f64);
    let ax = lx * cos + lz * sin;
    let az = -lx * sin + lz * cos;
    Some(WireRide {
        feet_x: vehicle.x + ax,
        feet_y: vehicle.y + ride_height as f64,
        feet_z: vehicle.z + az,
        vehicle_yaw: vehicle.y_rot,
    })
}

/// One wire boat as the renderer consumes it — the Add Entity snapshot's
/// pose verbatim (the server's own word, the same lane the mobs ride) plus
/// the boat family's own shape bits the registry name parses: the model
/// kind (`_raft` the RaftModel's two-plank deck picks, `_boat` the
/// BoatModel's five-piece hull), the chest bit (`_chest_` the variant's
/// three cubes + the 128-tall texture band pick), and the wood the
/// texture resolves through.
#[derive(Debug, Clone, PartialEq)]
pub struct WireBoat {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// The boat's body yaw in degrees (the submit fold's `180 − yRot`
    /// source).
    pub yaw: f32,
    /// the RaftModel hull (the `*_raft` names) — false rides the
    /// BoatModel's railed hull.
    pub raft: bool,
    /// the chest variant (`*_chest_boat` / `*_chest_raft`) — the chest
    /// cubes + the 128×128 texture band.
    pub chest: bool,
    /// the wood the registry named (`oak`, `bamboo`, ... — the texture
    /// path's own word, the unknown wood the load gate refuses).
    pub wood: String,
}

/// THE BOAT FAMILY'S OWN NAME PARSE (the 26.2 registry's per-wood entity
/// types — the wood the name itself speaks, the variant accessor the
/// AbstractBoat clinit never defined): `minecraft:oak_boat` ->
/// ("boat", false, "oak"), `minecraft:bamboo_chest_raft` ->
/// ("raft", true, "bamboo"). The CHEST suffixes read FIRST (every
/// `*_chest_boat` ends with `*_boat` too — the plain walk the wood
/// would corrupt). The bare `boat`/`chest_boat` names (no wood — the
/// pre-split naming) refuse: no wood, no texture, no render — the
/// honest unknown the ride anchor still carries (the anchor's math
/// the wood never reads).
pub fn boat_family_parts(name: &str) -> Option<(&'static str, bool, &str)> {
    let body = name.strip_prefix("minecraft:")?;
    let (kind, chest, wood) = if let Some(wood) = body.strip_suffix("_chest_boat") {
        ("boat", true, wood)
    } else if let Some(wood) = body.strip_suffix("_chest_raft") {
        ("raft", true, wood)
    } else if let Some(wood) = body.strip_suffix("_boat") {
        ("boat", false, wood)
    } else if let Some(wood) = body.strip_suffix("_raft") {
        ("raft", false, wood)
    } else {
        return None;
    };
    // the bare `chest_boat` / `chest_raft` (no wood — the pre-split
    // naming) fall through: the chest arm's suffix needs the leading
    // underscore the bare body lacks, the plain arm leaves the word
    // "chest" — and the bare `boat` leaves it empty. All three refuse:
    // no wood, no texture, no render.
    if wood.is_empty() || wood == "chest" {
        return None;
    }
    Some((kind, chest, wood))
}

/// THE BOAT PROJECTION: the entity store's boat-family bodies, the
/// renderer's own shape bits the name parse keeps. The boat's pose rides
/// the snapshot verbatim (the same authority the mobs speak) — no lerp
/// walk, no death flip: the hull the server's word carries.
pub fn boats_view(model: &WorldModel) -> Vec<WireBoat> {
    model
        .entities()
        .filter_map(|(id, e)| {
            let name = model.entity_type_name(e.type_id)?;
            let (kind, chest, wood) = boat_family_parts(name)?;
            Some(WireBoat {
                id,
                x: e.x,
                y: e.y,
                z: e.z,
                yaw: e.y_rot,
                raft: kind == "raft",
                chest,
                wood: wood.to_string(),
            })
        })
        .collect()
}

/// One wire mob as the renderer consumes it — the Add Entity
/// snapshot's pose verbatim (the server's own authority, the
/// `lerpTo` law the client walks): the numeric id (the update key
/// the refresh lane matches), the world doubles, the body yaw, the
/// head pitch, the ABSOLUTE head yaw (`yHeadRot` — the offset the
/// render's netHeadYaw extraction derives), and the registry NAME
/// the type id resolved through (`minecraft:zombie` renders).
#[derive(Debug, Clone, PartialEq)]
pub struct WireMob {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub head_yaw: f32,
    /// The metadata's own health word (LivingEntity's DATA_HEALTH_ID 9
    /// — the FLOAT accessor the Set Entity Data store keeps, the
    /// clinit defineId walk's own order FLAGS 8 then HEALTH 9):
    /// `None` while the server never spoke the accessor — the honest
    /// no-opinion the death walk never mistakes for a kill.
    pub health: Option<f32>,
    /// The hurt flash's remaining window (the Hurt Animation packet's
    /// own arm — the animateHurt 10-tick law the mailbox shipped for
    /// this id), 0 when the server never spoke the flash. The
    /// mailbox's shipped value: the mob lane's own tick counts it
    /// down after the landing (the client entity's own clock).
    pub hurt_time: i32,
    /// The metadata's own BURNING WORD (the shared-flags accessor —
    /// index 0, the BYTE serializer, bit 0: `Entity.isOnFire`'s own
    /// `getSharedFlag(0)` the 26.2 bytecode speaks): true while the
    /// server says the mob burns, false while the accessor never
    /// spoke (the honest unburnt — the silence never ignites).
    pub on_fire: bool,
    /// The metadata's own CROUCH WORD (the same shared-flags accessor
    /// — bit 1: `Entity.isCrouching`'s own `getSharedFlag(0) & 2` the
    /// vanilla client's render state speaks): true while the server
    /// says the mob bends, false while the accessor never spoke (the
    /// honest stand — the silence never crouches).
    pub crouching: bool,
    /// The metadata's own WOOL BYTE (Sheep's DATA_WOOL_ID 17 — the
    /// accessor the class tree lands there, the raw byte verbatim):
    /// the sheared bit + the dye nibble the merge decodes through the
    /// Sheep.isSheared/getColor laws. 0 = the SynchedEntityData ctor's
    /// own default (unsheared, WHITE) — the silence the vanilla
    /// client's local default speaks, and the honest 0 the other
    /// bodies' projections carry (the wool word never asks for their
    /// index-17 traffic).
    pub wool_byte: u8,
    /// The metadata's own CUSTOM NAME (Entity's DATA_CUSTOM_NAME 2 —
    /// the accessor the class tree lands at 2, the component's plain
    /// text the projection extracts: the TAG_String verbatim, the
    /// structured form's `text` key): the name MobRenderer's
    /// checkMagicName reads for every body — the jeb_ sheep's arm
    /// rides here, the accessor never spoken is the honest unnamed
    /// (None). The read is the ENTITY's own (the name is generic
    /// entity data — the sheep's rainbow gate is only one consumer).
    pub custom_name: Option<String>,
    /// THE NAMEPLATE'S VISIBILITY BIT (the accessor 3 the same clinit
    /// walk lands — the Boolean serializer's own word). The
    /// shouldShowName gate's first arm is `isCustomNameVisible()`:
    /// the byte raised arms the floating text, the silent byte keeps
    /// the name unsaid (a named body whose server never raised the
    /// bit renders no tag — the honest gate the vanilla client rides).
    pub name_visible: bool,
    /// The synced registry's own name (`minecraft:zombie` v1 renders;
    /// anything else is the honest not-yet-drawn type).
    pub type_name: String,
    /// THE BELOW-NAME LINE (the `Entity.belowNameDisplay` walk's own
    /// projection): the BELOW_NAME slot's objective + THIS entity's
    /// row (the holder the stringUUID law keeps) joined as
    /// `"score display"` — the NO_STYLE component the vanilla text
    /// builds. `None` while the slot is silent, the objective's row
    /// for this UUID is absent, or the ten-block gate the RENDER's
    /// distance law keeps refuses (the distance is the camera's own
    /// business — the extract law runs per frame, not per ship).
    pub score_text: Option<String>,
}

/// The OPEN container window as the UI consumes it — the
/// protocol-agnostic projection of the model's `OpenContainerState`
/// (the screens stay out of the wire's types, the JoinOutcome slice's
/// own convention). The slot list rides the MENU's own numbering
/// verbatim: a chest window's slots 0..27 are the container grid, the
/// tail is the player-inventory mirror — the screen's layout lanes
/// index both out of this one list. The ids are the ITEM registry
/// order's (the same table the flat icon textures resolve through).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveContainer {
    /// The window id the server assigned (the match key the container
    /// traffic rides; retained for the future clientbound echoes).
    pub id: i32,
    /// The MenuType registry id (generic_9x3 = 2, the chest — the
    /// parse's doc table names vanilla's built-in order). The screen
    /// side routes on it: the chest is the wired menu, other menus
    /// stay honest drops until their own rounds.
    pub menu_type_id: i32,
    /// The menu's slots as `(item_id, count)` pairs, the menu's own
    /// numbering, verbatim from the store (an empty stack is the
    /// wire's own `(0, 0)` — the screen's emptiness sentinel).
    pub slots: Vec<(i32, i32)>,
    /// THE BUNDLE CONTENTS' OWN FEED (the set-slot sync the screens'
    /// map rides): the slots whose `bundle_contents` component patch
    /// the store's stacks carry — the MENU slot id keyed with the
    /// entries' `(item_id, count)` pairs in wire order. The selected
    /// index the wire never speaks (the wrap's -1 default — the
    /// screens' own belief, any CHANGED component's reset).
    pub bundle_rows: Vec<(i32, Vec<(i32, i32)>)>,
    /// The GUI cursor stack as `(item_id, count)` (the Set Content
    /// packet's carriedItem).
    pub carried: (i32, i32),
    /// The menu's revision counter (the store's `state_id` — whichever
    /// packet spoke last): the value the player's own clicks echo back
    /// (the click packet's stateId lane; the server uses it to spot a
    /// stale client and correct it).
    pub state_id: i32,
}

/// The click's input modes, the UI-facing mirror of the wire's own
/// `ContainerInput` enum (the 26.2 registration order — the official
/// codec's `getEnumConstants()` order, dumped by the checked-in
/// `ReflectContainerClick.java`). The player's left press speaks
/// Pickup; the rest are the menu-interaction rounds' own lines.
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

/// One clicked slot, as the screen emits it (the protocol-agnostic
/// record the window's translation lane turns into the wire's own
/// click). The window id and the revision ride the LIVE store's
/// last-known values — the projection's own fields, not the screen's
/// guesses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerClick {
    /// The live window's id (the store's open container).
    pub window_id: i32,
    /// The menu's revision the client last saw.
    pub state_id: i32,
    /// The clicked slot, the MENU's own numbering (the chest grid
    /// 0..27, the player mirror 27..63).
    pub slot: i16,
    /// The button (the left press's 0; the right press is the
    /// split-stack round's own lane).
    pub button: i8,
    /// The click's input mode (Pickup for the plain left press,
    /// QuickMove for the shift-click — the vanilla hasShiftDown query
    /// overrides the click type for either button).
    pub input: ContainerInput,
    /// The client's believed per-slot deltas — the prediction's own
    /// output, the menu's slot numbering with the post-click (item_id,
    /// count) pairs (the (0, 0) sentinel rides the wire as the
    /// HashedStack's own emptiness). Empty without a prediction.
    pub changed: Vec<(i16, (i32, i32))>,
    /// The client's believed cursor AFTER the click (the (0, 0)
    /// emptiness sentinel).
    pub carried: (i32, i32),
}

/// The player's container command as the screen emits it: a clicked
/// slot, or the player's OWN close (the Escape/E echo the clientbound
/// Container Close lane mirrors). The command lane rides the live
/// session's worker — the frame thread emits, the worker speaks the
/// wire (the session's send path).
#[derive(Debug, Clone, PartialEq)]
pub enum ContainerCommand {
    Click(ContainerClick),
    Close { window_id: i32 },
    /// The player's OWN hotbar selection (the vanilla keyPress's lane):
    /// the SELECTION-relative 0..8 slot the number keys speak. The
    /// worker moves the store's selection (the local prediction — the
    /// hand follows through the store's own projection) and echoes the
    /// serverbound Set Carried Item (the dump's short-shaped slot).
    SelectHotbar { selection: u8 },
    /// THE MIDDLE-CLICK PICK's wire half (the vanilla
    /// `MultiPlayerGameMode.handlePickItemFromBlock`'s own lane — the
    /// packet send ALONE, the inventory law server-side): the clip's
    /// hit voxel and the Ctrl flag the packet's includeData slot
    /// rides. The client predicts NOTHING here (the offline store walk
    /// the pick's own lane keeps is the OFFLINE world's answer) — the
    /// server's verdict rides the clientbound Set Held Slot echo back
    /// through its gate, the store the server's to move.
    PickBlock {
        pos: (i32, i32, i32),
        include_data: bool,
    },
    /// THE BUNDLE SELECTION's wire half (the vanilla
    /// `BundleMouseActions.toggleSelectedBundleItem`'s own echo — the
    /// ServerboundSelectBundleItemPacket the scroll's tail sends after
    /// the local set): the hovered slot's MENU id (the walk's own
    /// numbering — the chest's grid/mirror the rects keep, the
    /// inventory's menu order) and the selection the walk moved (the
    /// wrap the `[0, n)` domain lands). The worker speaks the packet
    /// through the session's send path.
    SelectBundleItem {
        slot_id: i32,
        selected_item_index: i32,
    },
    /// The right-mouse USE (the vanilla gameMode.useItem's own lane):
    /// the camera's angles converted to the vanilla wire degrees
    /// ([`wire_angles`]), the MAIN_HAND ordinal the worker speaks.
    UseItem { yaw: f32, pitch: f32 },
    /// The use ON A BLOCK (the vanilla gameMode.useItemOn's own lane —
    /// the block half of the use arc): the client clip's hit (the hit
    /// voxel, the entry face's Direction ordinal, the cursor floats the
    /// wire's writeBlockHitResult speaks — the hit location minus the
    /// block origin — and the eye-inside flag). The worker speaks the
    /// MAIN_HAND ordinal and the same monotonic prediction sequence the
    /// use/release echo (the serverbound Use Item On packet).
    UseItemOn {
        pos: (i32, i32, i32),
        face: u8,
        cursor: (f32, f32, f32),
        inside: bool,
    },
    /// The use RELEASE (the vanilla gameMode.releaseUsingItem's own
    /// echo — the RELEASE_USE_ITEM action at BlockPos.ZERO / DOWN).
    ReleaseUse,
    /// The movement wire (the body's sendPosition dispatch): one of the
    /// four ServerboundMovePlayer packets the tick named. The body's
    /// own bookkeeping decides WHICH (the 2.0E-4 law, the 20-tick
    /// reminder, the ground/collision StatusOnly); the worker speaks
    /// the packet through the session's send path.
    MovePosRot {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
        horizontal_collision: bool,
    },
    MovePos {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool,
        horizontal_collision: bool,
    },
    MoveRot {
        yaw: f32,
        pitch: f32,
        on_ground: bool,
        horizontal_collision: bool,
    },
    MoveStatus {
        on_ground: bool,
        horizontal_collision: bool,
    },
    /// THE DIG ARC's wire send (the vanilla gameMode's destroy half —
    /// the Player Action packet's START/ABORT/STOP actions the dig state
    /// machine produced). The worker speaks the packet through the SAME
    /// monotonic prediction sequence the use arc carries.
    DigAction {
        action: u8,
        pos: (i32, i32, i32),
        face: u8,
    },
    /// The arm SWING (the vanilla LocalPlayer.swing's echo — the attack
    /// press's own beat and every mining beat the hold speaks).
    Swing,
    /// THE WIRE BODY'S OWN ATTACK (the vanilla gameMode.attack's lane):
    /// the server's entity id the 26.2's dedicated Attack packet
    /// carries — the record's ONE VarInt the clinit composite speaks.
    /// The worker speaks the packet BEFORE the swing beat (the dump's
    /// own order), the wire body's health the server's to keep (the
    /// Hurt echo the hurt lane renders the verdict the client never
    /// predicts).
    Attack { entity_id: i32 },
    /// THE WIRE BODY'S OWN INTERACT (the vanilla gameMode.interact's
    /// lane): the flat Interact record the right press ON A BODY
    /// speaks — the server's entity id, the CURSOR the hit location
    /// minus the body's origin (the LpVec3 the location field rides),
    /// the shift the isShiftKeyDown carries. The worker speaks the
    /// packet; the interaction the server's to resolve (the local
    /// prediction the wire body never runs — the echo the entity-event
    /// lane owes).
    Interact {
        entity_id: i32,
        cursor: (f64, f64, f64),
        shift: bool,
    },
    /// The death screen's RESPAWN (the vanilla DeathScreen's own
    /// serverbound echo: the Client Command packet's PERFORM_RESPAWN
    /// action — the server respawns the player and re-streams the
    /// position and the vitals).
    Respawn,
    /// THE INPUT WIRE (the vanilla `LocalPlayer.tick`'s own echo —
    /// `lastSentInput != input.keyPresses()` sends one
    /// ServerboundPlayerInputPacket): the seven booleans' record the
    /// movement lane rebuilt, the worker speaks the ONE byte. The
    /// riding lanes read it server-side (the boat's steering, the
    /// shift's dismount ask).
    PlayerInput {
        forward: bool,
        backward: bool,
        left: bool,
        right: bool,
        jump: bool,
        shift: bool,
        sprint: bool,
    },
}

/// The camera's pose in the VANILLA wire degrees — the inverse of
/// [`snapshot_camera`]'s own conversion (camera yaw = vanilla yaw rad +
/// π; camera pitch = −vanilla pitch rad): the use-item packet's yRot
/// and xRot the server's own math speaks.
pub fn wire_angles(cam: &crate::camera::Camera) -> (f32, f32) {
    let yaw = (cam.yaw - std::f32::consts::PI).to_degrees();
    let pitch = (-cam.pitch).to_degrees();
    (yaw, pitch)
}

/// The stack-size ceiling the prediction merges against — the
/// (item_id, count) store carries no components, so the per-item true
/// max (the tools' 1, the snowballs' 16) is unknown locally; the
/// vanilla default 64 is the honest belief for the common cargo, and
/// the server's authoritative correction repairs any overfill (the
/// prediction is a BELIEF, never the truth — the container lane's own
/// contract). The store's future component lanes carry the real maxes.
pub const ITEM_MAX_STACK_DEFAULT: i32 = 64;

/// THE EQUIPPABLE FOLD (javap 26.2 `LivingEntity.isEquippableInSlot` +
/// `getEquipmentSlotForItem` — the verdict the armor slots' `mayPlace`
/// keeps): the stack's `EQUIPPABLE` component names its
/// `EquipmentSlot`; a component-less stack folds to the MAINHAND
/// verdict (the `ifnonnull` else-arm — armor slots refuse it). The
/// client's (id, count) stacks carry no components, so the component
/// itself rides the registry's item names — the armor family the
/// vanilla datapack's default components speak: the four piece
/// suffixes, the carved pumpkin and the head family (HEAD), the
/// elytra (CHEST). Returns the ARMOR LOOP index the item equips —
/// the ctor's SLOT_IDS order (HEAD=0, CHEST=1, LEGS=2, FEET=3, the
/// container indices 39-k) — `None` for the MAINHAND verdict (and
/// the OFFHAND verdict no base item's component ever names — the
/// quickMoveStack's offhand arm the bytecode keeps stands honest).
pub fn equippable_loop_k(id: i32) -> Option<usize> {
    let name = mc_registry::item_name(id)?;
    if name.ends_with("_helmet")
        || name == "carved_pumpkin"
        || matches!(
            name,
            "player_head"
                | "zombie_head"
                | "creeper_head"
                | "skeleton_skull"
                | "wither_skeleton_skull"
                | "dragon_head"
                | "piglin_head"
        )
    {
        return Some(0); // HEAD — container 39, menu slot 5
    }
    if name.ends_with("_chestplate") || name == "elytra" {
        return Some(1); // CHEST — container 38, menu slot 6
    }
    if name.ends_with("_leggings") {
        return Some(2); // LEGS — container 37, menu slot 7
    }
    if name.ends_with("_boots") {
        return Some(3); // FEET — container 36, menu slot 8
    }
    None
}

/// The armor slot's PICKUP prediction (the doClick PICKUP arm over an
/// ArmorSlot — the `mayPlace` gate the `isEquippableInSlot` verdict
/// keeps, the `getMaxStackSize()` 1 the slot's own constant speaks):
/// an empty cursor takes the whole piece (the count's own 1 — the
/// half-rounded-up arm the secondary speaks lands the same); a held
/// stack the matching piece PLACES (the safeInsert the slot's own
/// max caps — count 1), the same item's merge arm the full-slot
/// silence keeps (room = 1 - 1 = 0), a DIFFERENT matching-piece
/// arm... the swap the doClick's `mayPlace`-true arm speaks — the
/// worn piece and the held trade places; ANY other item the gate
/// refuses (the stack stays on the cursor — the real game's own
/// feel).
pub fn predict_pickup_armor(
    slot: (i32, i32),
    carried: (i32, i32),
    loop_k: usize,
) -> ((i32, i32), (i32, i32)) {
    let slot_empty = slot.1 <= 0;
    let carried_empty = carried.1 <= 0;
    match (slot_empty, carried_empty) {
        // both empty — nothing to do
        (true, true) => (slot, carried),
        // an empty cursor over a filled slot — the piece comes off
        (false, true) => ((0, 0), slot),
        // a held stack: the mayPlace gate answers FIRST
        _ => {
            if equippable_loop_k(carried.0) != Some(loop_k) {
                return (slot, carried); // the gate's refusal — silence
            }
            if slot_empty {
                // the place arm — the slot's own max (1) caps the take
                let take = carried.1.min(1);
                let rest = carried.1 - take;
                ((carried.0, take), if rest > 0 { (carried.0, rest) } else { (0, 0) })
            } else if slot.0 == carried.0 {
                // the same item — the full slot takes nothing (max 1)
                (slot, carried)
            } else {
                // the matching piece — the swap the mayPlace arm speaks
                (carried, slot)
            }
        }
    }
}

/// The vanilla `AbstractContainerMenu.doClick` PICKUP prediction for
/// the left press (button 0) — the client's own menu logic, run
/// locally so the click's believed deltas and the held cursor ride the
/// packet (the vocabulary `HashedStack::item` provides). Takes the
/// believed (item_id, count) pair of the clicked slot and the believed
/// cursor; returns the believed pair AFTER the click, slot first. The
/// semantics (the vanilla pickup table): an empty slot + a held stack
/// PLACES it; a filled slot + an empty cursor PICKS UP the whole
/// stack; the SAME item MERGES into the slot up to the stack ceiling
/// (the remainder stays carried; a full slot takes nothing); a
/// DIFFERENT item SWAPS. The prediction is the client's belief only —
/// the server's authoritative corrections ride the container lane
/// back and replace it wholesale.
pub fn predict_pickup(slot: (i32, i32), carried: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let slot_empty = slot.1 <= 0;
    let carried_empty = carried.1 <= 0;
    match (slot_empty, carried_empty) {
        // both empty — nothing to do
        (true, true) => (slot, carried),
        // a held stack over an empty slot — place it all
        (true, false) => (carried, (0, 0)),
        // an empty cursor over a filled slot — pick up the whole stack
        (false, true) => ((0, 0), slot),
        // both filled
        (false, false) => {
            if slot.0 == carried.0 {
                // the same item: merge what fits, the remainder rides
                let room = ITEM_MAX_STACK_DEFAULT - slot.1;
                if room <= 0 {
                    (slot, carried) // a full slot takes nothing
                } else {
                    let moved = room.min(carried.1);
                    let rest = carried.1 - moved;
                    (
                        (slot.0, slot.1 + moved),
                        if rest > 0 { (carried.0, rest) } else { (0, 0) },
                    )
                }
            } else {
                // different items: the vanilla swap
                (carried, slot)
            }
        }
    }
}

/// The vanilla `AbstractContainerMenu.doClick` PICKUP prediction for the
/// RIGHT press (button 1) — the same PICKUP input mode on the wire, the
/// button byte separates the tables. The semantics (the doClick
/// buttonnum != 0 branch): an empty cursor over a filled slot takes the
/// CEILING HALF (5 -> 3 taken, 2 left; 1 -> the whole single); a held
/// stack PLACES ONE (the cursor's last item empties it); the same item
/// with room MOVES ONE; a different item (or a full slot, or both
/// empty) does NOTHING — the right press never swaps. The belief laws
/// are the local prediction's own (the server's corrections replace
/// the belief wholesale — the container lane's contract).
pub fn predict_pickup_right(slot: (i32, i32), carried: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let slot_empty = slot.1 <= 0;
    let carried_empty = carried.1 <= 0;
    match (slot_empty, carried_empty) {
        // both empty — nothing to do
        (true, true) => (slot, carried),
        // a held stack over an empty slot — place ONE
        (true, false) => (
            (carried.0, 1),
            if carried.1 > 1 {
                (carried.0, carried.1 - 1)
            } else {
                (0, 0)
            },
        ),
        // an empty cursor over a filled slot — take the ceiling half
        (false, true) => {
            let take = (slot.1 + 1) / 2;
            let leave = slot.1 - take;
            (
                if leave > 0 { (slot.0, leave) } else { (0, 0) },
                (slot.0, take),
            )
        }
        // both filled
        (false, false) => {
            if slot.0 == carried.0 && slot.1 < ITEM_MAX_STACK_DEFAULT {
                // the same item with room — move ONE
                (
                    (slot.0, slot.1 + 1),
                    if carried.1 > 1 {
                        (carried.0, carried.1 - 1)
                    } else {
                        (0, 0)
                    },
                )
            } else {
                // a different item or a full slot — the right press
                // never swaps, never overfills
                (slot, carried)
            }
        }
    }
}

/// The vanilla `AbstractContainerMenu.doClick` QUICK_MOVE prediction —
/// the SHIFT-click (the `hasShiftDown()` query overrides the click type
/// for the left AND the right press; the button byte keeps its press's
/// own value). The whole clicked stack moves to the OTHER section (the
/// ChestMenu's own `quickMoveStack` → `moveItemStackTo`): the chest grid
/// (0..27) feeds the player mirror (27..63) from its END backwards (the
/// vanilla `fillFromEnd` — the hotbar first, the menu's 62 down to 27),
/// the mirror feeds the grid slot 0 forward. TWO PASSES, the vanilla's
/// own order: the SAME-item stacks with room take what fits up to the
/// ceiling first, then the EMPTY slots take the remainder. The cursor is
/// NOT the quick-move's lane — the held stack rides out untouched. Takes
/// the FULL believed menu (the screen's grid + mirror, the menu's own
/// numbering) and the clicked slot; returns the changed `(index, pair)`
/// entries — the source first (emptied, or the unmovable remainder),
/// then the destinations in application order. NO entries means the
/// belief did not move (the empty source, or no room anywhere) — the
/// click still rides the wire and the server's correction decides. The
/// belief laws are the local prediction's own (the ceiling is the
/// documented ITEM_MAX_STACK_DEFAULT 64 — the server repairs any
/// overfill; the corrections replace the belief wholesale).
pub fn predict_quick_move(slots: &[(i32, i32)], src: usize) -> Vec<(usize, (i32, i32))> {
    let src_item = slots[src];
    if src_item.1 <= 0 {
        return Vec::new(); // nothing to move
    }
    let order: Vec<usize> = if src < 27 {
        (27..63).rev().collect() // the grid -> the mirror, the hotbar first
    } else {
        (0..27).collect() // the mirror -> the grid, slot 0 forward
    };
    let mut slots = slots.to_vec();
    let mut remaining = src_item.1;
    let mut changed: Vec<(usize, (i32, i32))> = Vec::new();
    // pass 1: the same-item merge into the stacks with room
    for &i in &order {
        if remaining == 0 {
            break;
        }
        let d = slots[i];
        if d.1 > 0 && d.0 == src_item.0 && d.1 < ITEM_MAX_STACK_DEFAULT {
            let take = (ITEM_MAX_STACK_DEFAULT - d.1).min(remaining);
            slots[i] = (d.0, d.1 + take);
            remaining -= take;
            changed.push((i, slots[i]));
        }
    }
    // pass 2: the empty slots take the remainder
    if remaining > 0 {
        for &i in &order {
            if remaining == 0 {
                break;
            }
            if slots[i].1 <= 0 {
                let take = remaining.min(ITEM_MAX_STACK_DEFAULT);
                slots[i] = (src_item.0, take);
                remaining -= take;
                changed.push((i, slots[i]));
            }
        }
    }
    let src_new = if remaining > 0 { (src_item.0, remaining) } else { (0, 0) };
    if src_new != src_item || !changed.is_empty() {
        changed.insert(0, (src, src_new));
    }
    changed
}

/// The quickMoveStack walk over the player's own store (javap 26.2
/// `InventoryMenu.quickMoveStack` — the arms the bytecode speaks in
/// order). The sources are the STORE rows (menu 9..35 the main grid,
/// menu 36..44 the hotbar; the armor/offhand sources ride
/// [`predict_quick_move_to_store`]):
///
/// 1. THE ARMOR ARM (bytecode 135-200 — it fires for EVERY source
///    past the menu's own early arms): `getEquipmentSlotForItem`
///    names HUMANOID_ARMOR and the matching menu slot (8 - the
///    equipment index) is EMPTY → the move rides
///    `moveItemStackTo(target, target+1, false)` and the walk ENDS
///    (the `ifne -> 325` the tail's own goto keeps) — the remainder
///    stays in the source, the hotbar routing never speaks. The
///    moveItemStackTo consults `mayPlace` — the matching piece's own
///    slot takes it, nothing else does.
/// 2. THE OFFHAND ARM (bytecode 203-246): `getEquipmentSlotForItem`
///    == OFFHAND and menu 45 empty → menu 45. The vanilla registry's
///    base items' components never name OFFHAND (the verdict folds
///    MAINHAND for component-less stacks) — the arm stands honest in
///    the bytecode, the registry keeps it silent.
/// 3. THE SECTION ARM (bytecode 247-306): a main-row source moves to
///    the hotbar rows (`moveItemStackTo(36, 45, false)` — menu 36..44
///    ascending = containers 0..8), a hotbar source to the main rows
///    (`moveItemStackTo(9, 36, false)`). The two passes
///    moveItemStackTo keeps: the merge pass over the whole range
///    first, the first-empty pass after, 64 the slot's max.
///
/// Returns the believed CHANGES as (container_index, stack) pairs —
/// the items' own 0..35, the armor list at 36..39 (loop k = index -
/// 36), the offhand at 40 — the source first (its remainder). An
/// empty vec means nothing moved.
pub fn predict_quick_move_inventory(
    slots: &[(i32, i32); 36],
    armor: &[(i32, i32); 4],
    offhand: &(i32, i32),
    src: usize,
) -> Vec<(usize, (i32, i32))> {
    let src_item = slots[src];
    if src_item.1 <= 0 {
        return Vec::new(); // nothing to move
    }
    // THE ARMOR ARM FIRST (the bytecode 135 gate the section arms
    // precede — the matching empty piece slot takes the whole stack,
    // capped at the slot's own max 1, and the walk ends)
    if let Some(k) = equippable_loop_k(src_item.0) {
        if armor[k].1 <= 0 {
            let take = src_item.1.min(1);
            let rest = src_item.1 - take;
            let mut changed = vec![(36 + k, (src_item.0, take))];
            let src_new = if rest > 0 { (src_item.0, rest) } else { (0, 0) };
            if src_new != src_item {
                changed.push((src, src_new));
            }
            return changed;
        }
    }
    // THE OFFHAND ARM — the registry never names it (see the doc);
    // the fallthrough the bytecode keeps speaks next.
    let _ = offhand;
    let order: Vec<usize> = if src < 9 {
        (9..36).collect() // the hotbar -> the main rows, store 9 forward
    } else {
        (0..9).collect() // the main rows -> the hotbar, store 0 forward
    };
    let mut slots = *slots;
    let mut remaining = src_item.1;
    let mut changed: Vec<(usize, (i32, i32))> = Vec::new();
    // pass 1: the same-item merge into the stacks with room
    for &i in &order {
        if remaining == 0 {
            break;
        }
        let d = slots[i];
        if d.1 > 0 && d.0 == src_item.0 && d.1 < ITEM_MAX_STACK_DEFAULT {
            let take = (ITEM_MAX_STACK_DEFAULT - d.1).min(remaining);
            slots[i] = (d.0, d.1 + take);
            remaining -= take;
            changed.push((i, slots[i]));
        }
    }
    // pass 2: the empty slots take the remainder
    if remaining > 0 {
        for &i in &order {
            if remaining == 0 {
                break;
            }
            if slots[i].1 <= 0 {
                let take = remaining.min(ITEM_MAX_STACK_DEFAULT);
                slots[i] = (src_item.0, take);
                remaining -= take;
                changed.push((i, slots[i]));
            }
        }
    }
    let src_new = if remaining > 0 { (src_item.0, remaining) } else { (0, 0) };
    if src_new != src_item || !changed.is_empty() {
        changed.insert(0, (src, src_new));
    }
    changed
}

/// The quickMoveStack walk for the ARMOR and OFFHAND sources (the
/// bytecode's own fallthrough — menu 5..8 and menu 45 match no
/// earlier section arm, so the final
/// `moveItemStackTo(9, 45, false)` answers): the [9, 45) FORWARD
/// range over the store's rows — menu 9..35 ascending = the main
/// rows 9..35, menu 36..44 ascending = the hotbar 0..8 — the merge
/// pass over the whole range first, the first-empty pass after, 64
/// the slot's max. Returns the believed changes (the store index
/// pairs) and the remainder that stayed in the source slot.
pub fn predict_quick_move_to_store(
    slots: &[(i32, i32); 36],
    id: i32,
    count: i32,
) -> (Vec<(usize, (i32, i32))>, i32) {
    let order: Vec<usize> = (9..36).chain(0..9).collect();
    let mut slots = *slots;
    let mut remaining = count;
    let mut changed: Vec<(usize, (i32, i32))> = Vec::new();
    // pass 1: the same-item merge into the stacks with room
    for &i in &order {
        if remaining == 0 {
            break;
        }
        let d = slots[i];
        if d.1 > 0 && d.0 == id && d.1 < ITEM_MAX_STACK_DEFAULT {
            let take = (ITEM_MAX_STACK_DEFAULT - d.1).min(remaining);
            slots[i] = (d.0, d.1 + take);
            remaining -= take;
            changed.push((i, slots[i]));
        }
    }
    // pass 2: the empty slots take the remainder
    if remaining > 0 {
        for &i in &order {
            if remaining == 0 {
                break;
            }
            if slots[i].1 <= 0 {
                let take = remaining.min(ITEM_MAX_STACK_DEFAULT);
                slots[i] = (id, take);
                remaining -= take;
                changed.push((i, slots[i]));
            }
        }
    }
    (changed, remaining)
}

/// The component patch's OWN bundle rows (the set-slot sync's
/// projection, shared by both container lanes): the slots whose
/// component patch carries the `bundle_contents` value — the slot's
/// OWN index keyed with the entries' (id, count) pairs, the wire
/// order kept. The patches the store already parsed (the value
/// codec's own lane); the selection index the wire never carries,
/// the screens' own belief.
fn bundle_rows_of(slots: &[mc_protocol::play::ItemStack]) -> Vec<(i32, Vec<(i32, i32)>)> {
    slots
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            let value = s.patch.added.iter().find_map(|(ty, v)| {
                if *ty == mc_protocol::play::data_component_types::BUNDLE_CONTENTS {
                    Some(v)
                } else {
                    None
                }
            })?;
            match value {
                mc_protocol::play::ComponentValue::BundleContents(entries) => Some((
                    i as i32,
                    entries
                        .iter()
                        .map(|e| (e.item_id, e.count))
                        .collect::<Vec<_>>(),
                )),
                _ => None,
            }
        })
        .collect()
}

/// The model's open-container store projected for the UI — `None`
/// while no foreign window is open (the join's own state: no screen
/// is up until the server opens one).
fn open_container_view(model: &WorldModel) -> Option<LiveContainer> {
    let oc = model.open_container()?;
    // THE BUNDLE ROWS' EXTRACTION (the set-slot sync's projection):
    // the shared walk over the window's own slots — the menu slot id
    // keyed with the entries' (id, count) pairs, the wire order kept.
    let bundle_rows = bundle_rows_of(&oc.slots);
    Some(LiveContainer {
        id: oc.id,
        menu_type_id: oc.menu_type_id,
        slots: oc.slots.iter().map(|s| (s.item_id, s.count)).collect(),
        bundle_rows,
        carried: (oc.carried.item_id, oc.carried.count),
        state_id: oc.state_id,
    })
}

/// The player's OWN inventory window's full slot shape (the wire's
/// own numbering — result 0, craft 1..5, armor 5..9, the main grid
/// 9..36, the hotbar 36..45, the offhand 45; the `InventoryMenu`
/// constants' audited layout).
pub const INVENTORY_MENU_SLOTS: usize = crate::world_model::SHIELD_SLOT + 1;

/// The window-0 store projected for the UI — the pairs the screen's
/// live store lane renders and hits, keyed by the window's OWN menu
/// id (the index IS the id, the store's Vec numbering verbatim). A
/// slot the store never spoke stays `(0, 0)` — the wire's own
/// emptiness the render skips, never a guessed stack (the hotbar
/// row's own law).
fn live_inventory_view(
    model: &WorldModel,
) -> (
    [(i32, i32); INVENTORY_MENU_SLOTS],
    Vec<(i32, Vec<(i32, i32)>)>,
    i32,
    (i32, i32),
) {
    let inv = model.inventory();
    let mut slots = [(0i32, 0i32); INVENTORY_MENU_SLOTS];
    for (i, s) in inv.slots.iter().enumerate().take(INVENTORY_MENU_SLOTS) {
        slots[i] = (s.item_id, s.count);
    }
    // THE BUNDLE ROWS' EXTRACTION (the set-slot sync's projection):
    // the shared walk over the window's own slots — the menu slot id
    // keyed with the entries' (id, count) pairs, the wire order kept.
    let bundles = bundle_rows_of(&inv.slots);
    // the revision and the GUI cursor — the click lane's own inputs
    // (the stateId the server's packets keep, the carriedItem the
    // Set Content's last word held)
    let state_id = inv.state_id;
    let carried = (inv.carried.item_id, inv.carried.count);
    (slots, bundles, state_id, carried)
}

/// The store's SELECTED hotbar stack projected for the UI — the
/// first-person hand's own lane. The hotbar slice is the inventory
/// window's `USE_ROW_SLOT_START..USE_ROW_SLOT_END` (the audited
/// layout), the selection the Set Held Slot packet's gate keeps;
/// `(0, 0)`-emptiness and a slot the store never spoke are `None` —
/// the bare arm, never a guessed stack.
fn held_item_view(model: &WorldModel) -> Option<(i32, i32)> {
    let inv = model.inventory();
    inv.slots
        .get(USE_ROW_SLOT_START + inv.selected)
        .filter(|s| s.count > 0)
        .map(|s| (s.item_id, s.count))
}

/// The store's hotbar ROW projected for the HUD — the same audited
/// slice the hand's lane reads (`USE_ROW_SLOT_START..USE_ROW_SLOT_END`),
/// but the whole row verbatim: the nine pairs ride the inventory
/// window's own order, the never-spoken tail stays `(0, 0)` — the
/// emptiness sentinel the HUD skips, exactly like the chest grid's.
/// The SIDEBAR slot's own view (the scoreboard wire trio's projection
/// — the model's belief through the display law the extract keeps):
/// the SIDEBAR slot's objective is looked up first (no slot, no
/// objective — `None`), then the objective's scores walk the
/// display law's shape: the hidden holders (the `#` prefix the
/// PlayerScoreEntry.isHidden law keeps) drop, the rest sort
/// value-DESCENDING with the owner's case-insensitive tiebreak (the
/// SCORE_DISPLAY_ORDER comparator's own walk), the rows cap at 15
/// (the stream's own limit), each row's name the display override
/// (else the holder — the ownerName walk).
/// THE BELOW-NAME LINE (the `Entity.belowNameDisplay` walk's own
/// projection): the BELOW_NAME slot's objective is looked up first
/// (the silent slot is the honest `None`), then the entity's own row
/// — the holder the stringUUID law formats (the vanilla
/// getScoreboardName's `return this.stringUUID`) — and the text the
/// walk builds: `score + ' ' + objective display` (the
/// `Component.empty().append(value).append(SPACE).append(displayName)`
/// chain, the NO_STYLE default the format keeps).
fn below_name_view(model: &WorldModel, uuid: u128) -> Option<String> {
    let objective = model.display_slots.get(&mc_protocol::DISPLAY_SLOT_BELOW_NAME)?;
    let obj = model.objectives.get(objective)?;
    let state = model.scores.get(&(crate::world_model::string_uuid(uuid), objective.clone()))?;
    Some(format!("{} {}", state.value, obj.display))
}

fn sidebar_view(model: &WorldModel) -> Option<WireSidebar> {
    let objective = model.display_slots.get(&SIDEBAR_SLOT)?;
    let obj = model.objectives.get(objective)?;
    let mut rows: Vec<(&String, i32, String)> = model
        .scores
        .iter()
        .filter(|((_, obj_name), _)| obj_name == objective)
        .filter(|((owner, _), _)| !owner.starts_with('#'))
        .map(|((owner, _), state)| {
            (owner, state.value, state.display.clone().unwrap_or_else(|| owner.clone()))
        })
        .collect();
    rows.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| a.0.to_lowercase().cmp(&b.0.to_lowercase()))
    });
    rows.truncate(SIDEBAR_ROW_CAP);
    Some(WireSidebar {
        title: obj.display.clone(),
        lines: rows
            .into_iter()
            .map(|(_, score, name)| WireSidebarLine { name, score })
            .collect(),
    })
}

/// The TAB overlay's own projection (the getPlayerInfos +
/// extractRenderState walks the wire's store feeds — see
/// [`WirePlayerList`]). The score column walks the LIST slot: the
/// objective the slot names (the Hud extractTabList's own lookup),
/// each listed entry's NAME the score holder (the
/// ScoreHolder.fromGameProfile law), the row's value the score store
/// keeps — the honest `None` when the objective is silent or the
/// player has no row on it.
fn player_list_view(model: &WorldModel) -> WirePlayerList {
    let list_objective = model.display_slots.get(&0);
    let score_of = |name: &str| {
        let obj_name = list_objective?;
        model.scores.get(&(name.to_string(), obj_name.clone())).map(|s| s.value)
    };
    let mut rows: Vec<WirePlayerRow> = model
        .players
        .values()
        .filter(|e| e.listed)
        .map(|e| WirePlayerRow {
            name: e.name.clone(),
            game_mode: e.game_mode,
            latency: e.latency,
            list_order: e.list_order,
            score: score_of(&e.name),
        })
        .collect();
    // THE PLAYER_COMPARATOR's own chain (the lambda$static$0..3 walk):
    // the list order ascending, the spectator LAST (the
    // gameMode == SPECTATOR ? 1 : 0 law — the flag the comparator
    // sorts ascending, the 1 the spectator owns sinks), the profile
    // name ascending in the String.compareTo's own case-sensitive
    // order (the team-name tiebreak the house's no-team lane skips —
    // every entry's team is the same empty string, the order never
    // speaks).
    rows.sort_by(|a, b| {
        a.list_order
            .cmp(&b.list_order)
            .then_with(|| (a.game_mode == 3).cmp(&(b.game_mode == 3)))
            .then_with(|| a.name.cmp(&b.name))
    });
    rows.truncate(PLAYER_ROW_CAP);
    WirePlayerList {
        header: model.tab_header.clone(),
        footer: model.tab_footer.clone(),
        rows,
    }
}

fn hotbar_row_view(model: &WorldModel) -> [(i32, i32); 9] {
    let inv = model.inventory();
    let mut row = [(0i32, 0i32); 9];
    for (i, slot) in inv
        .slots
        .iter()
        .skip(USE_ROW_SLOT_START)
        .take(USE_ROW_SLOT_END - USE_ROW_SLOT_START)
        .enumerate()
    {
        row[i] = (slot.item_id, slot.count);
    }
    row
}

/// The LOCAL player's vitals projection — the HUD's health row's own
/// lane (the vanilla Gui.renderHearts reads the LocalPlayer's health,
/// food and hurt state; this is the wire's store the same way the
/// hotbar row is the inventory's). The DEFAULTS speak the vanilla
/// first-join shape: 20 health / 20 food / no saturation (the player
/// the server never named), the flash unhurt, the death unwritten —
/// the honest defaults, never a guessed hurt.
#[derive(Debug, Clone, PartialEq)]
pub struct HealthView {
    pub health: f32,
    pub food: i32,
    pub saturation: f32,
    /// The hurt flash's remaining window (the animateHurt 10-tick arm
    /// the Hurt Animation packet spoke), 0 when unhurt.
    pub hurt_time: i32,
    /// The tilt yaw the packet carried (the future camera roll's fact).
    pub hurt_yaw: f32,
    /// The death screen's message, `None` while alive.
    pub death_message: Option<String>,
    /// The LOCAL player's active mob effects (the model's own-effect
    /// store's snapshot — the Update/Remove Mob Effect packets'
    /// filtered truth): the (registry id, amplifier, duration, flags)
    /// quads the HUD's effect lane speaks. Empty in every golden scene.
    pub effects: Vec<crate::world_model::MobEffectWireState>,
}

impl HealthView {
    pub fn default_view() -> Self {
        HealthView {
            health: 20.0,
            food: 20,
            saturation: 0.0,
            hurt_time: 0,
            hurt_yaw: 0.0,
            death_message: None,
            effects: Vec::new(),
        }
    }
}

fn health_view(model: &WorldModel) -> HealthView {
    let mut v = HealthView::default_view();
    if let Some(h) = &model.health {
        v.health = h.health;
        v.food = h.food;
        v.saturation = h.saturation;
    }
    if let Some((t, yaw)) = model.hurt {
        v.hurt_time = t;
        v.hurt_yaw = yaw;
    }
    if let Some(m) = &model.death_message {
        v.death_message = Some(m.clone());
    }
    // THE EFFECTS' OWN SNAPSHOT (the model's store verbatim — the
    // activeEffectsMap's walk; the BTreeMap-ordered copy keeps the
    // lane deterministic)
    v.effects = model.active_effects_debug().into_iter()
        .map(|(id, _, state)| crate::world_model::MobEffectWireState {
            effect_id: id,
            amplifier: state.amplifier,
            duration: state.duration,
            flags: state.flags,
        })
        .collect();
    v
}

/// The grid-local first-person camera for the materialized window: the
/// join teleport's EXACT pose — the vanilla standing eye height over the
/// feet, the server-directed look direction converted from the vanilla
/// angle conventions (yaw degrees, clockwise, 0 = facing south/+Z;
/// pitch degrees, positive = down) into the camera crate's (yaw radians,
/// 0 = -Z, counterclockwise; pitch positive = up): `yaw_cam =
/// yaw_mc + pi`, `pitch_cam = -pitch_mc`. `None` without dimension
/// layout knowledge: the window is column-local then and the player's
/// world y has no meaning in it — the honest state is "no pose", never a
/// guess (the world screen keeps its deterministic flight camera).
pub fn snapshot_camera(player: &PlayerPosition, min_y: Option<i32>, side: usize) -> Option<Camera> {
    let min_y = min_y?;
    let origin = snapshot_window_origin(player, side);
    let y_base = snapshot_y_base(player, Some(min_y));
    let feet = Vec3::new(
        player.x as f32 - (origin.0 * CHUNK_X as i32) as f32,
        player.y as f32 - y_base as f32,
        player.z as f32 - (origin.1 * CHUNK_Z as i32) as f32,
    );
    Some(Camera {
        pos: feet.add(Vec3::new(0.0, PLAYER_EYE_HEIGHT, 0.0)),
        yaw: player.yaw.to_radians() + std::f32::consts::PI,
        pitch: -player.pitch.to_radians(),
    })
}

/// The biome AT THE PLAYER's feet: the model's biome walk (the 4×4×4
/// cell grid, world-y through the dimension layout) resolved through the
/// synced registry names. `None` without a pose, outside the loaded
/// columns, or for an id the server never streamed — the honest unknown.
fn player_biome(model: &WorldModel) -> Option<String> {
    let p = model.player.as_ref()?;
    model
        .biome_name(p.x.floor() as i32, p.y.floor() as i32, p.z.floor() as i32)
        .map(|s| s.to_string())
}

/// The full join view: the materialized grid PLUS the camera pose from
/// the same window decisions. The join worker calls this off the frame
/// thread and ships all three parts in the join result — the world screen
/// renders the grid FROM the pose and the two can never disagree about
/// where the player stands.
pub fn materialize_snapshot_view(
    snap: WorldSnapshot,
    side: usize,
    mapper: impl FnMut(u32) -> u16,
) -> SnapshotView {
    let origin = snapshot_window_origin(&snap.player, side);
    let mut model = WorldModel::new();
    model.adopt_snapshot(snap);
    let player = model.player.as_ref().expect("adopt_snapshot pins the player");
    let y_base = snapshot_y_base(player, model.min_y);
    let camera = snapshot_camera(player, model.min_y, side);
    let biome = player_biome(&model);
    // the join's own store — the window-0 lane reads the adopted
    // state the same way the hand's lane does (empty until the wire
    // speaks)
    let (inventory_slots, inventory_bundles, inventory_state_id, inventory_carried) =
        live_inventory_view(&model);
    SnapshotView {
        world: materialize(&model, origin, side, y_base, mapper),
        camera,
        biome,
        // a fresh join opens no window — the container lane starts
        // closed (the Open Screen packet is the only opener)
        container: None,
        // and the inventory rides until the store speaks — the bare
        // arm, the join's own state (the helper reads the store)
        held: held_item_view(&model),
        hotbar: hotbar_row_view(&model),
        health: health_view(&model),
        // the join snapshot's own population (a fresh world carries
        // none — the empty lane the projection walks)
        mobs: Vec::new(),
        // and no boats (a fresh join carries no spawned hulls — the
        // empty lane the render skips)
        boats: Vec::new(),
        // and no voices yet (the join snapshot carries no played sounds
        // — the empty mailbox the projection walks)
        voices: Vec::new(),
        world_voices: Vec::new(),
        // the join snapshot carries no scoreboard belief — the wire
        // trio's packets are the only source (the empty lane)
        sidebar: None,
        // the join snapshot carries no tab belief either — the
        // player-info packets are the only source (the empty overlay:
        // no rows, no bands — the render's gate keeps it silent)
        player_list: WirePlayerList { header: String::new(), footer: String::new(), rows: Vec::new() },
        // and no ride belief — the Set Passengers packets are the only
        // source (the join's feet stand on their own, nothing carries)
        ride: None,
        // the join's own store — the window-0 lane reads the adopted
        // state the same way the hand's lane does (empty until the
        // wire speaks)
        inventory_slots,
        inventory_bundles,
        inventory_state_id,
        inventory_carried,
    }
}

/// The LIVE world view: one `WorldModel` fed by the play stream for the
/// session's whole life and re-materialized into the renderer window on
/// demand — the SAME model the screen renders, not a one-shot snapshot.
/// The window decisions (origin, y base, camera) are recomputed from the
/// LATEST player pose on every [`LiveView::refresh`]: a new teleport
/// re-anchors the grid AND the pose (the view follows the player like
/// the original), a re-sent chunk replaces its column in place, an
/// unload drops it — the play state's whole vocabulary lands here.
pub struct LiveView {
    /// The live model — the session's world state, exactly as streamed.
    pub model: WorldModel,
    side: usize,
}

impl LiveView {
    /// Adopt the join milestone as the live model's starting state (the
    /// session that produced it keeps pumping INTO this view — the first
    /// [`LiveView::refresh`] reproduces the one-shot snapshot view
    /// exactly, same window decisions, same grid, same pose).
    pub fn from_snapshot(snap: WorldSnapshot, side: usize) -> Self {
        let mut model = WorldModel::new();
        model.adopt_snapshot(snap);
        LiveView { model, side }
    }

    /// Feed one play event into the live model. Returns `true` when the
    /// event touched world state (info, position, chunk, unload, center)
    /// — the caller re-materializes only then; probes and skipped
    /// packets never dirty the view.
    pub fn apply(&mut self, event: PlayEvent) -> bool {
        self.model.apply(event)
    }

    /// The CURRENT view: the model's chunks projected into the renderer
    /// window plus the camera pose from the SAME window decisions — both
    /// recomputed from the latest player pose (the view follows the
    /// player, the vanilla behavior). No pose known (no teleport ever
    /// arrived): the deterministic window (origin 0,0, the layout-known
    /// y base or column-local 0) with no camera — the honest state,
    /// never a guess.
    pub fn refresh(&self, mapper: impl FnMut(u32) -> u16) -> SnapshotView {
        // the mob projection first (both arms ride the same walk —
        // the wire's population is pose data, never a guess)
        let mobs = self
            .model
            .entities()
            .filter_map(|(id, e)| {
                let type_name = self.model.entity_type_name(e.type_id)?.to_string();
                if type_name != "minecraft:zombie"
                    && type_name != "minecraft:pig"
                    && type_name != "minecraft:cow"
                    && type_name != "minecraft:sheep"
                {
                    return None; // not-yet-drawn types stay honest data
                }
                Some(WireMob {
                    id,
                    x: e.x,
                    y: e.y,
                    z: e.z,
                    yaw: e.y_rot,
                    pitch: e.x_rot,
                    head_yaw: e.y_head_rot,
                    // THE HEALTH ACCESSOR'S OWN INDEX — LivingEntity's
                    // clinit defineId walk speaks DATA_HEALTH_ID at 9
                    // (FLAGS 8, HEALTH 9, PARTICLES 10, AMBIENCE 11,
                    // ARROWS 12, STINGERS 13, SLEEPING 14 — the 8 the
                    // lane once read is the FLAGS byte's own slot, the
                    // Float match never spoke, the word never landed)
                    health: match e.data.get(&9) {
                        Some(mc_protocol::play::EntityDataEntry::Float(h)) => Some(*h),
                        _ => None,
                    },
                    // the hurt mailbox's shipped arm (the animateHurt
                    // window the packet spoke for THIS id — the ids
                    // the wire never hit stay unhurt, the honest 0)
                    hurt_time: self.model.entity_hurts.get(&id).copied().unwrap_or(0),
                    // the burning word (the shared-flags accessor's
                    // own bit 0 — Entity.isOnFire's getSharedFlag(0);
                    // the accessor never spoken is the honest unburnt)
                    on_fire: matches!(
                        e.data.get(&0),
                        Some(mc_protocol::play::EntityDataEntry::Byte(b)) if b & 1 != 0
                    ),
                    // the crouch word (the same accessor's bit 1 —
                    // Entity.isCrouching's own query; the accessor
                    // never spoken is the honest stand)
                    crouching: matches!(
                        e.data.get(&0),
                        Some(mc_protocol::play::EntityDataEntry::Byte(b)) if b & 2 != 0
                    ),
                    // THE WOOL BYTE (Sheep's own DATA_WOOL_ID 17 — the
                    // accessor the class tree lands at 17): the RAW
                    // word the merge decodes through the
                    // isSheared/getColor laws. The read is the SHEEP's
                    // OWN gate (the index belongs to the byte's own
                    // class — other bodies' index-17 traffic is data
                    // the wool word never asks for), the silence the
                    // SynchedEntityData ctor's own default (byte 0:
                    // unsheared, WHITE).
                    wool_byte: if type_name == "minecraft:sheep" {
                        match e.data.get(&crate::mob::SHEEP_DATA_WOOL_ID) {
                            // the byte verbatim (the wire's own unsigned
                            // word — the i8 the store carries re-reads
                            // as the bit pattern the decode asks)
                            Some(mc_protocol::play::EntityDataEntry::Byte(b)) => *b as u8,
                            _ => 0,
                        }
                    } else {
                        0
                    },
                    // THE CUSTOM NAME (Entity's DATA_CUSTOM_NAME 2 —
                    // the OPTIONAL_COMPONENT the store keeps, the
                    // plain text the projection extracts: the
                    // TAG_String verbatim, the structured form's
                    // `text` key the compound carries; the accessor
                    // never spoken, the None default is the honest
                    // unnamed)
                    custom_name: match e.data.get(&crate::mob::DATA_CUSTOM_NAME_ID) {
                        Some(mc_protocol::play::EntityDataEntry::OptionalComponent(Some(
                            nbt,
                        ))) => match nbt {
                            mc_protocol::nbt::Nbt::String(s) => Some(s.clone()),
                            mc_protocol::nbt::Nbt::Compound(pairs) => pairs
                                .iter()
                                .find(|(k, _)| k == "text")
                                .and_then(|(_, v)| match v {
                                    mc_protocol::nbt::Nbt::String(s) => Some(s.clone()),
                                    _ => None,
                                }),
                            _ => None,
                        },
                        _ => None,
                    },
                    // THE VISIBILITY BIT (the accessor 3's Boolean —
                    // the shouldShowName gate's own first arm, the
                    // byte the server raised the tag rides)
                    name_visible: match e.data.get(&crate::mob::DATA_CUSTOM_NAME_VISIBLE) {
                        Some(mc_protocol::play::EntityDataEntry::Bool(b)) => *b,
                        _ => false,
                    },
                    // THE BELOW-NAME LINE (the belowNameDisplay walk —
                    // the slot's objective + the entity's own row, the
                    // holder the stringUUID law formats, the text the
                    // score + space + the objective's display speak)
                    score_text: below_name_view(&self.model, e.uuid),
                    type_name,
                })
            })
            .collect();
        // the voices' own projection (the mailbox's arms verbatim — the
        // UNNAMED arm (the id outside the sound_event table) is the
        // honest no-voice the projection drops; the named ones ship
        // once with this view)
        let voices = self
            .model
            .entity_sounds
            .iter()
            .filter_map(|arm| {
                Some(WireVoice {
                    entity_id: arm.entity_id,
                    name: arm.name.clone()?,
                    volume: arm.volume,
                    pitch: arm.pitch,
                })
            })
            .collect();
        // the world sounds' own projection (the same walk the voices'
        // keep — the UNNAMED arm the id outside the sound_event table
        // ships is the honest no-voice the projection drops; the named
        // ones ship once with this view, the spot verbatim)
        let world_voices = self
            .model
            .world_sounds
            .iter()
            .filter_map(|arm| {
                Some(WireWorldVoice {
                    name: arm.name.clone()?,
                    x: arm.x,
                    y: arm.y,
                    z: arm.z,
                    volume: arm.volume,
                    pitch: arm.pitch,
                })
            })
            .collect();
        // the window-0 store's own projection rides every refresh —
        // the inventory screen follows the store exactly like the
        // hand and the HUD's row do (the pose-less arm included)
        let (inventory_slots, inventory_bundles, inventory_state_id, inventory_carried) =
            live_inventory_view(&self.model);
        let Some(player) = self.model.player.as_ref() else {
            return SnapshotView {
                world: materialize(
                    &self.model,
                    (0, 0),
                    self.side,
                    self.model.min_y.unwrap_or(0),
                    mapper,
                ),
                camera: None,
                biome: None,
                // no player pose ≠ no container: the window can be
                // open before any teleport arrived — read the store
                container: open_container_view(&self.model),
                // nor does it mean no inventory — the hand's lane
                // reads the store the same way (and the HUD's row)
                held: held_item_view(&self.model),
                hotbar: hotbar_row_view(&self.model),
                // the window-0 store's own projection — the screen's
                // live lane reads the store exactly like the hand does
                inventory_slots,
                inventory_bundles,
                inventory_state_id,
                inventory_carried,
                health: health_view(&self.model),
                mobs,
                // the boats' belief rides the pose-less arm too — the
                // hulls are the connection's own population, not the
                // pose's
                boats: boats_view(&self.model),
                voices,
                world_voices,
                // the scoreboard's belief rides the pose-less arm too —
                // the sidebar is the session's own truth, not the
                // pose's
                sidebar: sidebar_view(&self.model),
                // the tab's belief rides the same arm — the listed
                // rows the projection sorted and capped (the TAB key's
                // screen the overlay renders)
                player_list: player_list_view(&self.model),
                // the ride's belief rides the pose-less arm too — the
                // mount is the connection's own word, not the pose's
                ride: ride_view(&self.model),
            };
        };
        let origin = snapshot_window_origin(player, self.side);
        let y_base = snapshot_y_base(player, self.model.min_y);
        let camera = snapshot_camera(player, self.model.min_y, self.side);
        let biome = player_biome(&self.model);
        SnapshotView {
            world: materialize(&self.model, origin, self.side, y_base, mapper),
            camera,
            biome,
            // the live window state rides EVERY refresh — the chest's
            // slots update in place exactly like the terrain does
            container: open_container_view(&self.model),
            // the selected hotbar stack rides every refresh too — the
            // hand follows the store exactly like the chest does (and
            // the HUD's whole row rides with it)
            held: held_item_view(&self.model),
            hotbar: hotbar_row_view(&self.model),
            // the vitals ride every refresh too — the hearts follow the
            // store exactly like the hotbar row does
            health: health_view(&self.model),
            // the wire's own population rides with them — the mobs the
            // server spawned follow the store exactly like the hearts do
            mobs,
            // the hulls ride with the population — the boats the server
            // spawned follow the store exactly like the mobs do
            boats: boats_view(&self.model),
            voices,
            world_voices,
            // the scoreboard's belief rides every refresh too — the
            // sidebar follows the wire trio's store exactly like the
            // vitals do
            sidebar: sidebar_view(&self.model),
            // the tab's belief rides every refresh too — the listed
            // rows follow the player-info store exactly like the
            // scoreboard does
            player_list: player_list_view(&self.model),
            // the ride's anchor rides every refresh too — the carried
            // position follows the vehicle's own snapshot exactly like
            // the mobs' poses do
            ride: ride_view(&self.model),
            // the player's OWN store rides with them — the inventory
            // screen's live lane follows the store exactly like the
            // chest's slots do
            inventory_slots,
            inventory_bundles,
            inventory_state_id,
            inventory_carried,
        }
    }
}

/// The per-cell VARIANT byte the palette kind alone cannot carry: the
/// STATE's properties, decoded through the generated per-state table
/// (`props_of`, the 14th audit — `mc2rs-gen block-ids` / the
/// `dump-block-states` arc). Currently leaf_litter only: `facing`
/// (north=0, east=1, south=2, west=3 — the blockstate's y-rotation
/// order) x `segment_amount` (1..=4) packs into
/// `1 + facing + (segment_amount - 1) * 4` (1..=16). Every other block —
/// and any props string that fails to parse — yields 0: the mesher's
/// no-opinion full-footprint look.
pub fn leaf_litter_variant(id: u32) -> u8 {
    if mc_registry::generated::block_ids::block_name_of(id) != Some("minecraft:leaf_litter") {
        return 0;
    }
    let Some(props) = mc_registry::generated::block_ids::props_of(id) else {
        return 0;
    };
    let mut facing = 0u8;
    let mut segment = 1u8;
    for pair in props.split(',') {
        let Some((k, v)) = pair.split_once('=') else {
            continue;
        };
        match k {
            "facing" => {
                facing = match v {
                    "north" => 0,
                    "east" => 1,
                    "south" => 2,
                    "west" => 3,
                    _ => return 0,
                };
            }
            "segment_amount" => {
                segment = match v {
                    "1" => 1,
                    "2" => 2,
                    "3" => 3,
                    "4" => 4,
                    _ => return 0,
                };
            }
            _ => {}
        }
    }
    1 + facing + (segment - 1) * 4
}

/// The REAL id mapper, GENERATED from the official jar's bootstrapped
/// runtime registry (`mc2rs-gen dump-block-states` -> the block_ids
/// table): a global state id resolves to its block NAME, and the runtime
/// palette carries the block when we render it. The palette covers the
/// survival classics plus the generated members (stone variants, deepslate,
/// ores, netherrack); ids the palette has no member for (glass, wool,
/// slabs, ...) map to AIR — the truthful holes — until the palette itself
/// grows generated members. Nothing here is remembered: the table is
/// regenerated from the jar.
pub fn runtime_mapper(id: u32) -> u16 {
    match mc_registry::generated::block_ids::block_name_of(id) {
        Some("minecraft:stone") => STONE,
        Some("minecraft:dirt") => DIRT,
        Some("minecraft:grass_block") => GRASS,
        Some("minecraft:oak_log") => LOG,
        Some("minecraft:oak_leaves") => LEAVES,
        Some("minecraft:water") => WATER,
        Some("minecraft:sand") => SAND,
        Some("minecraft:granite") => GRANITE,
        Some("minecraft:diorite") => DIORITE,
        Some("minecraft:andesite") => ANDESITE,
        Some("minecraft:deepslate") => DEEPSLATE,
        Some("minecraft:cobbled_deepslate") => COBBLED_DEEPSLATE,
        Some("minecraft:gravel") => GRAVEL,
        Some("minecraft:bedrock") => BEDROCK,
        Some("minecraft:coal_ore") => COAL_ORE,
        Some("minecraft:copper_ore") => COPPER_ORE,
        Some("minecraft:iron_ore") => IRON_ORE,
        Some("minecraft:gold_ore") => GOLD_ORE,
        Some("minecraft:redstone_ore") => REDSTONE_ORE,
        Some("minecraft:lapis_ore") => LAPIS_ORE,
        Some("minecraft:diamond_ore") => DIAMOND_ORE,
        Some("minecraft:emerald_ore") => EMERALD_ORE,
        Some("minecraft:netherrack") => NETHERRACK,
        Some("minecraft:pale_oak_leaves") => PALE_LEAVES,
        // the thirteenth-audit kind: ALL 16 leaf_litter states (facing x
        // segment_amount, ids 30339..=30355 in the generated table) ride
        // ONE kind — the full-footprint flat ground cover. Per-segment
        // coverage (quarter/half-strip/full quads from the multipart
        // blockstate) needs state-property extraction: documented
        // follow-up.
        Some("minecraft:leaf_litter") => LEAF_LITTER,
        _ => AIR, // air and every block the palette doesn't render yet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{DIRT, NAME_OF, PALE_LEAVES, STONE};
    use crate::world_model::WorldModel;
    use mc_protocol::play::clientbound;
    use mc_registry::generated::atlas;
    use mc_protocol::{
        BiomeEffects, BiomeEntry, ChunkColumn, ChunkLight, GrassColorModifier, PlayEvent,
        RegistryData, SpawnInfo, WorldInfo,
        varint::write_varint,
    };

    /// One synced biome element under the mock tables (the real
    /// NETWORK_CODEC shape, plains' climate — the name is what the
    /// assertions resolve).
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

    /// One section whose block states ride the DIRECT palette (15 bits:
    /// the entries ARE the global state ids — vanilla's own encoding for
    /// big palettes). leaf_litter north, segment_amount 1 — the real
    /// state 30339, the 14th audit's decode input — at (2,3,4).
    fn leaf_litter_section() -> mc_protocol::ChunkSection {
        let idx = |x: usize, y: usize, z: usize| (y << 8) | (z << 4) | x;
        let mut entries = vec![0u32; 4096];
        entries[idx(2, 3, 4)] = 30339;
        let mut s = Vec::new();
        s.extend_from_slice(&1i16.to_be_bytes());
        s.push(15); // direct: global ids, no palette
        s.extend_from_slice(&mc_protocol::pack_entries(&entries, 15));
        s.push(0); // biomes: single valued
        write_varint(&mut s, 1);
        parse_one_section(s)
    }

    /// Decode a section buffer through the REAL parser (the same wire
    /// shape the world_model tests use): stone at (0,0,0), dirt at (3,1,2).
    fn stone_dirt_section() -> mc_protocol::ChunkSection {
        let palette = [0u32, 1, 10]; // air, stone, dirt — EXAMPLE ids
        let idx = |x: usize, y: usize, z: usize| (y << 8) | (z << 4) | x;
        let mut entries = vec![0u32; 4096];
        entries[idx(0, 0, 0)] = 1;
        entries[idx(3, 1, 2)] = 2;
        let mut s = Vec::new();
        s.extend_from_slice(&2i16.to_be_bytes());
        s.push(5); // bits per entry -> indirect palette
        write_varint(&mut s, palette.len() as i32);
        for id in palette {
            write_varint(&mut s, id as i32);
        }
        s.extend_from_slice(&mc_protocol::pack_entries(&entries, 5));
        s.push(0); // biomes: single valued
        write_varint(&mut s, 1);
        parse_one_section(s)
    }

    /// One all-air section through the REAL parser (single-valued air,
    /// the exact shape the world_model tests package).
    fn air_section() -> mc_protocol::ChunkSection {
        let mut s = Vec::new();
        s.extend_from_slice(&0i16.to_be_bytes());
        s.push(0); // bits per entry: single valued
        write_varint(&mut s, 0); // the single value: air
        s.push(0); // biomes: single valued
        write_varint(&mut s, 187);
        parse_one_section(s)
    }

    /// One all-air section whose biomes ride a 2-bit indirect palette
    /// (the vanilla 4×4×4 cell array — 64 entries of PALETTE INDICES),
    /// through the REAL parser like every other test section here.
    fn biome_cells_section(palette: &[u32], cells: &[u32; 64]) -> mc_protocol::ChunkSection {
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
        parse_one_section(s)
    }

    /// Package one section buffer into a one-section chunk packet and
    /// parse it, so tests run against the actual wire shape.
    fn parse_one_section(buffer: Vec<u8>) -> mc_protocol::ChunkSection {
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CHUNK_DATA);
        p.extend_from_slice(&0i32.to_be_bytes());
        p.extend_from_slice(&0i32.to_be_bytes());
        p.push(0x0A);
        p.push(0x00); // empty heightmaps compound
        write_varint(&mut p, buffer.len() as i32);
        p.extend_from_slice(&buffer);
        write_varint(&mut p, 0); // no block entities
        p.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // light masks
        let col = mc_protocol::play::parse_chunk_data(&p).expect("chunk parses");
        col.sections.into_iter().next().expect("one section")
    }

    /// The example mapper: the ids the test columns carry, to the runtime
    /// palette. Unknown ids map to AIR — the caller's explicit choice.
    fn example_mapper(id: u32) -> u16 {
        match id {
            1 => STONE,
            10 => DIRT,
            _ => AIR,
        }
    }

    #[test]
    fn materialize_carries_the_models_light() {
        use mc_protocol::LightData;
        let mut m = WorldModel::new();
        // one section, column-local y 0..16; the sky light layer for that
        // section (light bit 1 = section sy 0 + 1) is all 0xFF -> sky 15
        // everywhere in it; NO block layer at all (the honest unknown)
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight {
                sky: vec![None, Some(LightData::Data(vec![0xFF; 2048])), None],
                block: vec![None, None, None],
            },
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        let light = world.light.as_ref().expect("the view carries a light layer");
        // inside the section: the model's sky 15 landed; the block layer
        // stays unknown — the mesher's partial-layer rule consumes exactly
        // the sky opinion
        let (sky, block) = light.cell(1, 0, 5, 0);
        assert_eq!(sky, Some(15), "the section's sky light landed in the grid");
        assert_eq!(block, None, "no block layer was ever sent");
        // the stone cell itself (y 0) is also lit sky-15 (light fills air AND blocks)
        let (sky, _) = light.cell(1, 0, 0, 0);
        assert_eq!(sky, Some(15));
        // above the column's extent: the honest unknown — never a guess
        let (sky, block) = light.cell(1, 0, 20, 0);
        assert_eq!(sky, None, "outside the column: unknown, not zero");
        assert_eq!(block, None);
        // below: unknown too
        let (sky, _) = light.cell(1, 0, -1, 0);
        assert_eq!(sky, None);
        // and a cell in an UNSENT window column: unknown (the grid's fresh state)
        let (sky, _) = light.cell(1, 20, 5, 20);
        assert_eq!(sky, None, "unsent column: unknown");
    }

    /// The mob projection: the Add Entity store's zombies, pigs, cows
    /// AND sheep ride the refresh (the synced entity-type registry
    /// names the id), the unknown types stay honest data, the pose
    /// verbatim (the server's own authority the lerpTo law will walk).
    #[test]
    fn refresh_projects_the_wire_zombies() {
        use mc_protocol::play::EntitySpawn;
        let mut m = WorldModel::new();
        // the synced registry: type id 0 = pig, id 1 = zombie, id 2 = cow, id 3 = sheep
        m.registry.entity_types = vec![
            "minecraft:pig".to_string(),
            "minecraft:zombie".to_string(),
            "minecraft:cow".to_string(),
            "minecraft:sheep".to_string(),
        ];
        m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 7,
            uuid: 1,
            type_id: 1,
            x: 10.0,
            y: 64.0,
            z: 12.0,
            movement: [0.0; 3],
            x_rot: 5.0,
            y_rot: 90.0,
            y_head_rot: 60.0,
            data: 0,
        }));
        // a pig: the SECOND drawn body (the quadruped mesh the kind's
        // own branch speaks — the registry name rides the projection)
        m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 8,
            uuid: 2,
            type_id: 0,
            x: 30.0,
            y: 64.0,
            z: 32.0,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 0.0,
            y_head_rot: 0.0,
            data: 0,
        }));
        let mut view = LiveView {
            model: m,
            side: 1,
        };
        // a cow: the THIRD drawn body (the bovine mesh the kind's own
        // branch speaks — the registry name rides the projection)
        view.model.apply(PlayEvent::Spawn(EntitySpawn {
            id: 9,
            uuid: 3,
            type_id: 2,
            x: 50.0,
            y: 64.0,
            z: 52.0,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 180.0,
            y_head_rot: 180.0,
            data: 0,
        }));
        // a sheep: the FOURTH drawn body (the ovine mesh + the eat
        // fold the kind's own branch speaks — the registry name rides
        // the projection)
        view.model.apply(PlayEvent::Spawn(EntitySpawn {
            id: 10,
            uuid: 4,
            type_id: 3,
            x: 70.0,
            y: 64.0,
            z: 72.0,
            movement: [0.0; 3],
            x_rot: 0.0,
            y_rot: 270.0,
            y_head_rot: 270.0,
            data: 0,
        }));
        let upd = view.refresh(example_mapper);
        assert_eq!(upd.mobs.len(), 4, "the zombie AND the pig AND the cow AND the sheep render v2");
        // THE ID KEYS THE LOOKUP (the entities' walk order is the
        // store's own — the id names the body, never the position)
        let z = upd.mobs.iter().find(|m| m.id == 7).unwrap();
        assert_eq!(z.type_name, "minecraft:zombie");
        assert_eq!((z.x, z.y, z.z), (10.0, 64.0, 12.0));
        assert_eq!((z.yaw, z.pitch, z.head_yaw), (90.0, 5.0, 60.0));
        assert_eq!(z.health, None, "no accessor spoken: the honest no-opinion");
        // the pig's own word rides the SAME projection (the type name
        // verbatim, the pose the server's own)
        let p = upd.mobs.iter().find(|m| m.id == 8).unwrap();
        assert_eq!(p.type_name, "minecraft:pig");
        assert_eq!((p.x, p.y, p.z), (30.0, 64.0, 32.0));
        // the cow's own word rides the SAME projection (the type name
        // verbatim, the pose the server's own)
        let c = upd.mobs.iter().find(|m| m.id == 9).unwrap();
        assert_eq!(c.type_name, "minecraft:cow");
        assert_eq!((c.x, c.y, c.z), (50.0, 64.0, 52.0));
        assert_eq!((c.yaw, c.head_yaw), (180.0, 180.0));
        // the sheep's own word rides the SAME projection (the type
        // name verbatim, the pose the server's own)
        let s = upd.mobs.iter().find(|m| m.id == 10).unwrap();
        assert_eq!(s.type_name, "minecraft:sheep");
        assert_eq!((s.x, s.y, s.z), (70.0, 64.0, 72.0));
        assert_eq!((s.yaw, s.head_yaw), (270.0, 270.0));
        // the metadata's own health word (LivingEntity's DATA_HEALTH_ID
        // 9 — the FLOAT accessor the Set Entity Data store keeps; the
        // clinit defineId walk's own order FLAGS 8 then HEALTH 9)
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 9,
                serializer: 3,
                value: mc_protocol::play::EntityDataEntry::Float(6.0),
            }],
        }));
        let upd = view.refresh(example_mapper);
        let z = upd.mobs.iter().find(|m| m.id == 7).unwrap();
        assert_eq!(z.health, Some(6.0), "the server's word rides");
        // THE WOOL BYTE'S OWN RIDE (Sheep's DATA_WOOL_ID 17 — the raw
        // byte the projection carries, the SHEEP's own gate the read
        // speaks): the zombie's index-17 traffic is data the wool word
        // never asks for, the sheep's own byte rides verbatim
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 17,
                serializer: 0,
                value: mc_protocol::play::EntityDataEntry::Byte(0x7F),
            }],
        }));
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 10,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 17,
                serializer: 0,
                value: mc_protocol::play::EntityDataEntry::Byte(0x1E),
            }],
        }));
        let upd = view.refresh(example_mapper);
        let z = upd.mobs.iter().find(|m| m.id == 7).unwrap();
        assert_eq!(z.wool_byte, 0, "the zombie's index-17 traffic the wool never asks for");
        let s = upd.mobs.iter().find(|m| m.id == 10).unwrap();
        assert_eq!(s.wool_byte, 0x1E, "the sheep's own byte rides verbatim");
        // THE CUSTOM NAME'S OWN RIDE (Entity's DATA_CUSTOM_NAME 2 — the
        // OPTIONAL_COMPONENT the store keeps, the plain text the
        // projection extracts): the zombie's accessor-2 traffic is the
        // ENTITY's own word (the name is generic data — checkMagicName
        // is every body's read), the sheep's structured form's `text`
        // key the same extract speaks
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 2,
                serializer: 6,
                value: mc_protocol::play::EntityDataEntry::OptionalComponent(Some(
                    mc_protocol::nbt::Nbt::String("Dinnerbone".into()),
                )),
            }],
        }));
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 10,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 2,
                serializer: 6,
                value: mc_protocol::play::EntityDataEntry::OptionalComponent(Some(
                    mc_protocol::nbt::Nbt::Compound(vec![(
                        "text".into(),
                        mc_protocol::nbt::Nbt::String("jeb_".into()),
                    )]),
                )),
            }],
        }));
        let upd = view.refresh(example_mapper);
        let z = upd.mobs.iter().find(|m| m.id == 7).unwrap();
        assert_eq!(
            z.custom_name.as_deref(),
            Some("Dinnerbone"),
            "the TAG_String the projection carries verbatim"
        );
        let s = upd.mobs.iter().find(|m| m.id == 10).unwrap();
        assert_eq!(
            s.custom_name.as_deref(),
            Some("jeb_"),
            "the compound's text key the extract speaks"
        );
        // THE UNNAMED'S OWN SILENCE (the accessor never spoken, the
        // None default the projection carries — the pig's word)
        let p = upd.mobs.iter().find(|m| m.id == 8).unwrap();
        assert_eq!(p.custom_name, None, "the silence the unnamed speaks");
        // the absent-optional arm (the flag byte alone — the server
        // unnamed the entity): the None rides
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 2,
                serializer: 6,
                value: mc_protocol::play::EntityDataEntry::OptionalComponent(None),
            }],
        }));
        let upd = view.refresh(example_mapper);
        let z = upd.mobs.iter().find(|m| m.id == 7).unwrap();
        assert_eq!(z.custom_name, None, "the empty optional the None speaks");
        // THE NAMEPLATE'S VISIBILITY BIT (the accessor 3's Boolean —
        // Entity.shouldShowName's own first arm the raw walk speaks:
        // `isCustomNameVisible()` IS the gate): the raised bit arms
        // the tag, the absent byte keeps the named body's silence,
        // the false the disarm the byte speaks
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 3,
                serializer: 8,
                value: mc_protocol::play::EntityDataEntry::Bool(true),
            }],
        }));
        let upd = view.refresh(example_mapper);
        let z = upd.mobs.iter().find(|m| m.id == 7).unwrap();
        assert!(z.name_visible, "the raised bit the projection carries");
        let s = upd.mobs.iter().find(|m| m.id == 10).unwrap();
        assert!(
            !s.name_visible,
            "the sheep's absent byte the named silence keeps"
        );
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 3,
                serializer: 8,
                value: mc_protocol::play::EntityDataEntry::Bool(false),
            }],
        }));
        let upd = view.refresh(example_mapper);
        let z = upd.mobs.iter().find(|m| m.id == 7).unwrap();
        assert!(!z.name_visible, "the false the disarm speaks");
        // THE MOBS' OWN FLASH (the Hurt Animation's other arm — the
        // mailbox the foreign ids' packets arm): the zombie's id ships
        // the 10-tick window, the pig's silence stays the honest 0
        view.model.apply(PlayEvent::Hurt(mc_protocol::HurtAnimation {
            entity_id: 7,
            yaw: -20.0,
        }));
        view.model.apply(PlayEvent::Hurt(mc_protocol::HurtAnimation {
            entity_id: 8,
            yaw: 0.0,
        }));
        view.model.apply(PlayEvent::Hurt(mc_protocol::HurtAnimation {
            entity_id: 7,
            yaw: -20.0,
        }));
        let upd = view.refresh(example_mapper);
        let z = upd.mobs.iter().find(|m| m.id == 7).unwrap();
        assert_eq!(z.hurt_time, 10, "the packet's own window");
        // the pig's own flash rides ITS lane (the hit's own id — the
        // two windows never mix)
        let p = upd.mobs.iter().find(|m| m.id == 8).unwrap();
        assert_eq!(p.hurt_time, 10, "the pig's own hit ships its own window");
        // the drain speaks after the ship (the mailbox's own law —
        // the mob's tick owns the countdown, the stale arm never
        // re-arms the flash the window already spent)
        assert_eq!(view.model.drain_entity_hurts().get(&7), Some(&10));
        assert!(view.model.entity_hurts.is_empty(), "the mailbox spent");
        let upd = view.refresh(example_mapper);
        let z = upd.mobs.iter().find(|m| m.id == 7).unwrap();
        assert_eq!(z.hurt_time, 0, "the shipped arm never re-arms");
        // THE VOICES' OWN SHIP (the Entity Sound mailbox): the named
        // arm rides the view (the registry name verbatim), the id
        // outside the sound_event table ships UNNAMED and the
        // projection honestly drops it, the drain empties the mailbox
        view.model.registry.sound_events =
            vec!["minecraft:entity.zombie.ambient".to_string()];
        view.model.apply(PlayEvent::EntitySound(mc_protocol::EntitySoundPacket {
            entity_id: 7,
            sound: mc_protocol::ConsumableSound::Registry(0),
            volume: 1.0,
            pitch: 0.85,
            seed: 9,
        }));
        view.model.apply(PlayEvent::EntitySound(mc_protocol::EntitySoundPacket {
            entity_id: 7,
            sound: mc_protocol::ConsumableSound::Registry(17), // outside the table
            volume: 1.0,
            pitch: 1.0,
            seed: 9,
        }));
        let upd = view.refresh(example_mapper);
        assert_eq!(upd.voices.len(), 1, "the unnamed arm is the honest no-voice");
        assert_eq!(upd.voices[0].entity_id, 7);
        assert_eq!(upd.voices[0].name, "minecraft:entity.zombie.ambient");
        assert_eq!(upd.voices[0].volume, 1.0);
        assert_eq!(upd.voices[0].pitch, 0.85);
        view.model.drain_entity_sounds();
        let upd = view.refresh(example_mapper);
        assert!(upd.voices.is_empty(), "the shipped voice never re-ships");
    }

    /// THE BURNING WORD'S PROJECTION (the shared-flags accessor —
    /// index 0, the BYTE serializer, bit 0: `Entity.isOnFire`'s own
    /// `getSharedFlag(0)` the 26.2 bytecode speaks): the flag rides
    /// the projection verbatim, the other bits never ignite, the
    /// accessor never spoken is the honest unburnt.
    #[test]
    fn the_projection_carries_the_burning_word() {
        use mc_protocol::play::EntitySpawn;
        let mut m = WorldModel::new();
        m.registry.entity_types = vec!["minecraft:zombie".to_string()];
        m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 7,
            uuid: 1,
            type_id: 0,
            x: 10.0,
            y: 64.0,
            z: 12.0,
            movement: [0.0; 3],
            x_rot: 5.0,
            y_rot: 90.0,
            y_head_rot: 60.0,
            data: 0,
        }));
        let mut view = LiveView {
            model: m,
            side: 1,
        };
        // no accessor spoken: the honest unburnt (the silence never
        // ignites)
        let upd = view.refresh(example_mapper);
        assert!(!upd.mobs[0].on_fire, "the silence never ignites");
        // the shared-flags accessor's bit 0: the burning word rides
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 0,
                serializer: 0,
                value: mc_protocol::play::EntityDataEntry::Byte(0x01),
            }],
        }));
        let upd = view.refresh(example_mapper);
        assert!(upd.mobs[0].on_fire, "the flag's own bit speaks");
        // the OTHER bits never ignite (bit 1 crouching, bit 3 sprinting
        // — the byte's own word, the bit-0 mask the query speaks)
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 0,
                serializer: 0,
                value: mc_protocol::play::EntityDataEntry::Byte(0x0A),
            }],
        }));
        let upd = view.refresh(example_mapper);
        assert!(!upd.mobs[0].on_fire, "the other bits never ignite");
        // the douse: bit 0 clear — the flag's own drop
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 0,
                serializer: 0,
                value: mc_protocol::play::EntityDataEntry::Byte(0x00),
            }],
        }));
        let upd = view.refresh(example_mapper);
        assert!(!upd.mobs[0].on_fire, "the douse rides the same lane");
    }

    /// THE CROUCH WORD'S PROJECTION (the same shared-flags accessor —
    /// bit 1: `Entity.isCrouching`'s own `getSharedFlag(0) & 2`): the
    /// flag rides the projection verbatim, the FIRE's own bit never
    /// bends the mob, the accessor never spoken is the honest stand.
    #[test]
    fn the_projection_carries_the_crouch_word() {
        use mc_protocol::play::EntitySpawn;
        let mut m = WorldModel::new();
        m.registry.entity_types = vec!["minecraft:zombie".to_string()];
        m.apply(PlayEvent::Spawn(EntitySpawn {
            id: 7,
            uuid: 1,
            type_id: 0,
            x: 10.0,
            y: 64.0,
            z: 12.0,
            movement: [0.0; 3],
            x_rot: 5.0,
            y_rot: 90.0,
            y_head_rot: 60.0,
            data: 0,
        }));
        let mut view = LiveView {
            model: m,
            side: 1,
        };
        // no accessor spoken: the honest stand (the silence never
        // crouches)
        let upd = view.refresh(example_mapper);
        assert!(!upd.mobs[0].crouching, "the silence never crouches");
        // the shared-flags accessor's bit 1: the crouch word rides
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 0,
                serializer: 0,
                value: mc_protocol::play::EntityDataEntry::Byte(0x02),
            }],
        }));
        let upd = view.refresh(example_mapper);
        assert!(upd.mobs[0].crouching, "the flag's own bit bends");
        // the FIRE's own bit never bends the mob (bit 0 — the byte's
        // own word, the bit-1 mask the query speaks)
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 0,
                serializer: 0,
                value: mc_protocol::play::EntityDataEntry::Byte(0x01),
            }],
        }));
        let upd = view.refresh(example_mapper);
        assert!(!upd.mobs[0].crouching, "the fire's bit never bends");
        // both words at once: each query reads its own mask (0x03 —
        // burning AND crouching, the bits' independence)
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 0,
                serializer: 0,
                value: mc_protocol::play::EntityDataEntry::Byte(0x03),
            }],
        }));
        let upd = view.refresh(example_mapper);
        assert!(upd.mobs[0].crouching, "the two words ride together");
        assert!(upd.mobs[0].on_fire, "the fire's word rides too");
        // the stand: bit 1 clear — the flag's own drop
        view.model.apply(PlayEvent::EntityData(mc_protocol::play::EntityDataSet {
            id: 7,
            entries: vec![mc_protocol::play::SetEntityDataEntry {
                index: 0,
                serializer: 0,
                value: mc_protocol::play::EntityDataEntry::Byte(0x00),
            }],
        }));
        let upd = view.refresh(example_mapper);
        assert!(!upd.mobs[0].crouching, "the stand rides the same lane");
    }

    fn materialize_carries_the_models_day_tick() {
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        // no clock yet: the view is day (None), and the grid is fresh
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(world.day_tick, None, "no Set Time: the honest day");
        // the server's Set Time lands in the view: midnight total ticks
        m.apply(PlayEvent::Time(mc_protocol::play::parse_set_time(
            &mc_protocol::play::test_support::set_time_packet(5, &[(0, 18_000, 0.0, 1.0)]),
        ).expect("set time parses")));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(
            world.day_tick,
            Some(18_000),
            "the view carries the clock's day tick for the mesher's SkyFactor"
        );
    }

    #[test]
    fn materialize_carries_the_models_ambient_light() {
        // the lightmap's AmbientColor base rides the SAME snapshot: the
        // model's dimension resolves through the built-in
        // visual/ambient_light_color table; no dimension pinned (or an
        // unknown custom one) keeps None — the mesher reads the
        // bytecode-default black, the pre-ambient arithmetic.
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(world.ambient_light, None, "no dimension: no opinion");
        // the overworld's stated floor lands on the view
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:overworld".to_string(),
            portal_cooldown: 0,
        }));
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(
            world.ambient_light,
            Some([0x0a, 0x0a, 0x0a]),
            "the view carries the dimension's ambient base for the mesher"
        );
    }

    #[test]
    fn materialize_carries_the_models_block_light_tint() {
        // the lightmap's BlockLightTint rides the SAME snapshot (the
        // ambient's sibling): the model's dimension resolves through the
        // built-in visual/block_light_tint column; no dimension pinned
        // (or an unknown custom one) keeps None — the mesher reads the
        // bytecode default #FFD88C (the corrected blue 140). No vanilla
        // dimension states the tint, so the stamped verdict is the
        // absent shape end to end; a stated one rides the synced entry.
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(world.block_light_tint, None, "no dimension: no opinion");
        // the built-in nether lands the None shape (no vanilla statement)
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        }));
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(
            world.block_light_tint, None,
            "the built-in table states no tint — the default's shape"
        );
        // a custom server's stated tint lands on the view verbatim
        m.registry.dimension_types.push(mc_protocol::DimensionTypeEntry {
            name: "minecraft:custom".into(),
            min_y: -64,
            height: 384,
            default_clock: None,
            timelines: None,
            ambient_light_color: None,
            skybox: None,
            fog_start_distance: None,
            fog_end_distance: None,
            cloud_fog_end_distance: None,
            block_light_tint: Some([0x30, 0x28, 0x21]),
            sky_light_color: None,
            sky_light_factor: None,
            night_vision_color: None,
        });
        m.current_dimension = Some("minecraft:custom".to_string());
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(
            world.block_light_tint,
            Some([0x30, 0x28, 0x21]),
            "the view carries the stated tint for the mesher"
        );
    }

    #[test]
    fn materialize_carries_the_models_sky_light_factor() {
        // the stated sky-light factor rides the SAME snapshot (the
        // fold's multiply-base): the model's dimension resolves through
        // the built-in visual/sky_light_factor column. THE VANILLA
        // SHAPE this round discovered: the built-in NETHER states 0.0
        // (its sky term pinned to zero), the overworlds state None —
        // the mesher's exact f32 identity.
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(
            world.sky_light_factor, None,
            "no dimension: the default 1.0's shape"
        );
        // the built-in nether lands the stated 0.0 — the jar's own verdict
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 0,
        }));
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(
            world.sky_light_factor,
            Some(0.0),
            "the view carries the nether's stated 0.0 for the mesher's fold"
        );
    }

    #[test]
    fn materialize_carries_the_models_sky_light_color() {
        // the stated sky-light color rides the SAME snapshot (the
        // lightmap family's transport): the model's dimension resolves
        // through the built-in visual/sky_light_color column. THE REAL
        // JAR'S SHAPE: the built-in END states #ac60cd (the moonlight
        // pair), the overworlds state None — the mesher's white
        // identity.
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(
            world.sky_light_color, None,
            "no dimension: the white default's shape"
        );
        // the built-in end lands the stated #ac60cd — the jar's own
        // verdict, raw channel bytes for the mesher's integer fold
        m.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 0,
            dimension: "minecraft:the_end".to_string(),
            portal_cooldown: 0,
        }));
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(
            world.sky_light_color,
            Some([0xac, 0x60, 0xcd]),
            "the view carries the end's stated #ac60cd for the mesher's fold"
        );
    }

    #[test]
    fn materialize_carries_the_models_night_vision_color() {
        // the stated night-vision color rides the SAME snapshot (the
        // lightmap family's transport): the model's dimension resolves
        // through the built-in visual/night_vision_color column. NO
        // vanilla dimension states it — the built-in rows are all None —
        // so the view's absent shape is the bytecode default's verdict
        // (#999999 gray, read by the mesher UNDER the factor 0.0), and
        // a custom server's stated entry lands VERBATIM.
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(
            world.night_vision_color, None,
            "no dimension: the #999999 default's shape"
        );
        // a synced custom entry is the server's truth: it lands on the
        // view raw, the shader's stated base for the day the factor lifts
        m.registry.dimension_types.push(mc_protocol::DimensionTypeEntry {
            name: "minecraft:custom".into(),
            min_y: -64,
            height: 384,
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
            night_vision_color: Some([0x18, 0x10, 0x28]),
        });
        m.current_dimension = Some("minecraft:custom".to_string());
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        assert_eq!(
            world.night_vision_color,
            Some([0x18, 0x10, 0x28]),
            "the view carries the stated custom night-vision color verbatim"
        );
    }

    #[test]
    fn materialize_carries_the_models_biome_layer() {
        let mut m = WorldModel::new();
        // the synced table: id 0 plains (the baked water default), id 1
        // a swamp-named element with the REAL swamp climate (0.8, 0.9)
        // and water color — the wire id IS the arrival order
        let mut swamp = biome_entry("minecraft:swamp");
        swamp.downfall = 0.9;
        swamp.effects.water_color = 0x617B64;
        m.registry.biomes = vec![biome_entry("minecraft:plains"), swamp];
        // one column arrives (single-valued biome wire id 1 = the SECOND
        // element, the mock sections' convention)
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, example_mapper);
        let layer = world.biomes.as_ref().expect("the biome layer rides the window");
        assert_eq!(layer.tints.len(), 2, "one quad per synced element");
        assert_eq!(
            layer.tints[0],
            [Some(0x91BD59), Some(0x77AB2F), Some(4_159_204), Some(0xA37546)],
            "plains: the climate sample IS the baked constants (the equivalence); dry rides the climate verdict"
        );
        assert_eq!(
            layer.tints[1],
            [Some(0x6AC44E), Some(0x45B421), Some(0x617B64), Some(0xA36946)],
            "swamp climate (0.8, 0.9): the REAL grid samples (the modifier stays unconsumed); dry rides its climate"
        );
        // the column's vertical extent (one section, y 0..15) resolved the
        // single-valued id everywhere; above it the honest UNKNOWN
        for y in 0..16usize {
            let idx = y * CHUNK_X * CHUNK_Z;
            assert_eq!(layer.ids[0][idx], 1, "y {y} inside the column");
        }
        assert_eq!(
            layer.ids[0][20 * CHUNK_X * CHUNK_Z],
            BIOME_UNKNOWN,
            "above the column's extent: no opinion"
        );
        // the tint is one face_tint away from the mesher's fold
        assert_eq!(
            world.face_tint(WATER, 0, 0, 8, 0),
            Some(0x617B64),
            "wire id 1 -> the second element's water color (differs from baked)"
        );
        assert_eq!(
            world.face_tint(WATER, 0, 0, 20, 0),
            None,
            "above the extent: the baked look"
        );
    }

    #[test]
    fn runtime_mapper_resolves_through_the_generated_table() {
        // GENERATED facts from the official 26.2 registry (block_ids.rs):
        // the real global state ids of the palette blocks
        assert_eq!(runtime_mapper(0), AIR, "air");
        assert_eq!(runtime_mapper(1), STONE, "stone = global id 1, generated");
        assert_eq!(runtime_mapper(9), GRASS, "grass_block snowy=false (its default state)");
        assert_eq!(runtime_mapper(10), DIRT, "dirt = global id 10, generated");
        assert_eq!(runtime_mapper(86), WATER, "water level=0");
        assert_eq!(runtime_mapper(118), SAND);
        assert_eq!(runtime_mapper(137), LOG, "oak_log axis=y");
        assert_eq!(runtime_mapper(252), LEAVES);
        // state details collapse: any state of a palette block maps
        assert_eq!(runtime_mapper(90), WATER, "water level=4");
        assert_eq!(runtime_mapper(136), LOG, "oak_log axis=x");
        // the generated members — real ids pinned from block_ids.rs
        assert_eq!(runtime_mapper(2), GRANITE, "granite = global id 2, generated");
        assert_eq!(runtime_mapper(4), DIORITE, "diorite = global id 4");
        assert_eq!(runtime_mapper(6), ANDESITE, "andesite = global id 6");
        assert_eq!(runtime_mapper(85), BEDROCK, "bedrock = global id 85");
        assert_eq!(runtime_mapper(124), GRAVEL, "gravel = global id 124");
        assert_eq!(runtime_mapper(133), COAL_ORE, "coal_ore = global id 133");
        assert_eq!(runtime_mapper(131), IRON_ORE, "iron_ore = global id 131");
        assert_eq!(runtime_mapper(129), GOLD_ORE, "gold_ore = global id 129");
        assert_eq!(runtime_mapper(27_790), COPPER_ORE, "copper_ore = global id 27790");
        assert_eq!(runtime_mapper(563), LAPIS_ORE, "lapis_ore = global id 563");
        assert_eq!(runtime_mapper(5_307), DIAMOND_ORE, "diamond_ore = global id 5307");
        assert_eq!(runtime_mapper(9_573), EMERALD_ORE, "emerald_ore = global id 9573");
        assert_eq!(runtime_mapper(6_997), NETHERRACK, "netherrack = global id 6997");
        // the twelfth-audit kind: pale_oak_leaves (states 448..476) maps
        // and carries NO tint slot (the audited BlockColors verdict)
        assert_eq!(runtime_mapper(448), PALE_LEAVES, "pale_oak_leaves distance=0 persistent=true");
        assert_eq!(runtime_mapper(475), PALE_LEAVES, "any state collapses");
        // the thirteenth-audit kind: ALL 16 leaf_litter states (facing x
        // segment_amount, the half-open 30339..30355 range in the generated
        // table) ride ONE kind — the flat ground cover
        assert_eq!(runtime_mapper(30_339), LEAF_LITTER, "leaf_litter first state");
        assert_eq!(runtime_mapper(30_354), LEAF_LITTER, "leaf_litter last state");
        assert_eq!(runtime_mapper(30_347), LEAF_LITTER, "any state collapses");
        assert_eq!(runtime_mapper(30_355), AIR, "the range is half-open: moss_block");
        // deepslate carries an axis property — any state collapses (the
        // same policy as oak_log); redstone_ore has the lit state
        assert_eq!(runtime_mapper(30_416), DEEPSLATE, "deepslate axis=x");
        assert_eq!(runtime_mapper(30_418), DEEPSLATE, "deepslate axis=z");
        assert_eq!(runtime_mapper(30_419), COBBLED_DEEPSLATE, "cobbled_deepslate");
        assert_eq!(runtime_mapper(6_882), REDSTONE_ORE, "redstone_ore lit=true");
        // blocks the palette has no member for stay truthful holes
        assert_eq!(runtime_mapper(3), AIR, "polished_granite renders as air");
        assert_eq!(runtime_mapper(14), AIR, "cobblestone renders as air");
        assert_eq!(runtime_mapper(32_365), AIR, "past the table edge: air");
    }

    #[test]
    fn generated_palette_members_carry_real_atlas_tiles_and_mesh() {
        // the regenerated atlas carries official textures for every new
        // member (all full cubes — the mesher treats them as opaque)
        for kind in [
            GRANITE, DIORITE, ANDESITE, DEEPSLATE, COBBLED_DEEPSLATE, GRAVEL, BEDROCK,
            COAL_ORE, COPPER_ORE, IRON_ORE, GOLD_ORE, REDSTONE_ORE, LAPIS_ORE, DIAMOND_ORE,
            EMERALD_ORE, NETHERRACK,
        ] {
            assert!((kind as usize) < atlas::N_KINDS, "{} inside the atlas", NAME_OF[kind as usize]);
            for face in 0..6 {
                assert_ne!(
                    atlas::FACE_TILES[kind as usize][face],
                    atlas::TILE_NONE,
                    "{} face {} has a real tile",
                    NAME_OF[kind as usize],
                    face
                );
            }
        }
        // a world with the new blocks meshes through the textured path:
        // place them ABOVE the generated terrain (heights 8..=14) so every
        // face borders air and is actually meshed
        let mut w = World::new(1, 7);
        for (i, &kind) in [
            GRANITE, DIORITE, ANDESITE, DEEPSLATE, COBBLED_DEEPSLATE, GRAVEL, BEDROCK,
            COAL_ORE, COPPER_ORE, IRON_ORE, GOLD_ORE, REDSTONE_ORE, LAPIS_ORE, DIAMOND_ORE,
            EMERALD_ORE, NETHERRACK,
        ]
        .iter()
        .enumerate()
        {
            // the chunks field is pub; World::set is scene-generation-internal
            w.chunks[0][i + 0 * CHUNK_X + 20 * CHUNK_X * CHUNK_Z] = kind;
        }
        let quads = crate::mesher::mesh_world(&w, None);
        assert!(quads.iter().any(|q| q.tile != atlas::TILE_NONE
            && q.pts.iter().all(|p| (p[1] - 21.0).abs() < f32::EPSILON)),
            "new members' open-sky top faces mesh with real atlas tiles");
    }

    #[test]
    fn name_of_matches_the_palette_order() {
        // world.rs NAME_OF and the generated atlas KIND_NAMES describe the
        // same ids in the same order — the compile-time mirror
        assert_eq!(NAME_OF, mc_registry::generated::atlas::KIND_NAMES);
    }

    #[test]
    fn leaf_litter_variant_decodes_the_real_state_ids() {
        // the generated table's real ids: facing OUTER (north, south,
        // west, east — the registry order) x segment_amount INNER; the
        // packing is 1 + facing + (segment - 1) * 4 with the blockstate's
        // y-rotation order north/east/south/west = 0..=3
        assert_eq!(leaf_litter_variant(30339), 1, "north, segment 1");
        assert_eq!(leaf_litter_variant(30340), 5, "north, segment 2");
        assert_eq!(leaf_litter_variant(30341), 9, "north, segment 3");
        assert_eq!(leaf_litter_variant(30342), 13, "north, segment 4");
        assert_eq!(leaf_litter_variant(30343), 3, "south, segment 1");
        assert_eq!(leaf_litter_variant(30346), 15, "south, segment 4");
        assert_eq!(leaf_litter_variant(30347), 4, "west, segment 1");
        assert_eq!(leaf_litter_variant(30350), 16, "west, segment 4");
        assert_eq!(leaf_litter_variant(30351), 2, "east, segment 1");
        assert_eq!(leaf_litter_variant(30354), 14, "east, segment 4");
        // outside the block: the no-opinion byte
        assert_eq!(leaf_litter_variant(1), 0, "stone");
        assert_eq!(leaf_litter_variant(0), 0, "air");
    }

    #[test]
    fn materialize_decodes_the_leaf_litter_variant_into_the_grid() {
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![leaf_litter_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 1, 0, runtime_mapper);
        assert_eq!(world.get(2, 3, 4), LEAF_LITTER, "the kind through the mapper");
        let idx = 2 + 4 * CHUNK_X + 3 * CHUNK_X * CHUNK_Z;
        assert_eq!(
            world.variants.as_ref().expect("materialize always builds it")[0][idx],
            1,
            "north, segment_amount 1 -> the variant byte 1"
        );
        // every other cell: the no-opinion byte
        assert_eq!(world.variants.as_ref().unwrap()[0][0], 0, "air carries none");
    }

    #[test]
    fn materialize_maps_blocks_through_the_callers_mapper() {
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 3,
            z: -2,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        // window origin (1,-3): chunk (3,-2) sits at grid (2, 1)
        let world = materialize(&m, (1, -3), 4, 0, example_mapper);
        assert_eq!(world.chunks_per_side, 4);
        // column-local stone (0,0,0) -> grid chunk (2,1) local (0,0,0)
        // -> world grid coords (2*16, 0, 1*16)
        assert_eq!(world.get(32, 0, 16), STONE, "stone through the mapper");
        assert_eq!(world.get(35, 1, 18), DIRT, "dirt (3,1,2) of the column");
        assert_eq!(world.get(33, 0, 16), AIR, "air stays air");
        // a column outside the window never appears
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 10,
            z: 10,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (1, -3), 4, 0, example_mapper);
        assert_eq!(world.get(32, 0, 16), STONE, "window content unchanged");
        assert_eq!(world.get(0, 0, 0), AIR, "outside window stays air");
    }

    #[test]
    fn missing_columns_and_short_columns_stay_air() {
        let mut m = WorldModel::new();
        // a one-section column: 16 blocks tall, sky above in the grid
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        let world = materialize(&m, (0, 0), 2, 0, example_mapper);
        assert_eq!(world.get(0, 0, 0), STONE);
        assert_eq!(world.get(0, 15, 0), AIR, "column top slice is air blocks");
        assert_eq!(world.get(0, 16, 0), AIR, "above the short column: sky");
        assert_eq!(world.get(0, 47, 0), AIR, "grid top: sky");
        // an empty model: the whole window is fresh air
        let empty = WorldModel::new();
        let world = materialize(&empty, (5, 5), 2, 0, example_mapper);
        assert!(world
            .chunks
            .iter()
            .all(|c| c.iter().all(|&b| b == AIR)));
    }

    #[test]
    fn mapper_sees_raw_ids_and_y_base_drives_the_copy() {
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        }));
        // the mapper receives every resolved id verbatim (air included) —
        // nothing is pre-filtered, the caller owns the full decision
        let mut seen = Vec::new();
        let world = materialize(&m, (0, 0), 1, 0, |id| {
            if !seen.contains(&id) {
                seen.push(id);
            }
            example_mapper(id)
        });
        seen.sort_unstable();
        assert_eq!(seen, vec![0, 1, 10], "raw global ids reach the mapper");
        assert_eq!(world.get(0, 0, 0), STONE);
        // the caller owns the vertical anchor. With min_y known, an
        // explicit y_base = min_y reproduces the old bottom-anchored copy
        // (world-y queries shift back through block() — the min_y-stable
        // property, now spelled out at the call site)
        m.min_y = Some(-64);
        let world = materialize(&m, (0, 0), 1, -64, example_mapper);
        assert_eq!(world.get(0, 0, 0), STONE, "y_base = min_y: bottom anchor");
        assert_eq!(world.get(3, 1, 2), DIRT);
        assert_eq!(world.get(0, 16, 0), AIR);
        // anchor ABOVE the one-section column: the window slid up, the
        // column content is below the view (the truthful render)
        let world = materialize(&m, (0, 0), 1, -48, example_mapper);
        assert_eq!(world.get(0, 0, 0), AIR, "world -48..0: the section is below");
        // anchor BELOW it: the content slides up into view
        let world = materialize(&m, (0, 0), 1, -80, example_mapper);
        assert_eq!(world.get(0, 16, 0), STONE, "world -64 at grid 16");
        assert_eq!(world.get(3, 17, 2), DIRT);
    }

    #[test]
    fn snapshot_y_base_anchors_the_player_and_clamps_to_the_world_floor() {
        let p = |y: f64| PlayerPosition {
            teleport_id: 0,
            x: 0.0,
            y,
            z: 0.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
        };
        // no layout knowledge: the deterministic column-local copy (0),
        // whatever the teleport says — never a guess
        assert_eq!(snapshot_y_base(&p(64.0), None), 0);
        assert_eq!(snapshot_y_base(&p(-59.0), None), 0);
        // overworld: feet 64 anchor at grid 16 -> the window base 48
        assert_eq!(snapshot_y_base(&p(64.0), Some(-64)), 48);
        assert_eq!(snapshot_y_base(&p(64.7), Some(-64)), 48, "floor before the math");
        // near the world floor the clamp holds the window on solid ground:
        // feet -60 cannot sit at grid 16 (base -76 < min_y) -> base -64,
        // the feet land at grid 4 instead
        assert_eq!(snapshot_y_base(&p(-60.0), Some(-64)), -64);
        assert_eq!(snapshot_y_base(&p(-48.0), Some(-64)), -64, "exactly the anchor's reach");
        // nether layout (min_y 0): the same math over its own floor
        assert_eq!(snapshot_y_base(&p(40.0), Some(0)), 24);
    }

    #[test]
    fn snapshot_camera_converts_the_vanilla_pose_into_the_grid_frame() {
        let pose = |yaw: f32, pitch: f32| PlayerPosition {
            teleport_id: 1,
            x: 8.5,
            y: 64.0,
            z: -3.25,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw,
            pitch,
            flags: 0,
        };
        // without layout knowledge there is NO pose: the grid is
        // column-local then and the player's world y means nothing in it
        // — the honest state is "no pose", never a guess
        assert!(snapshot_camera(&pose(0.0, 0.0), None, 5).is_none());

        // grid-local placement: (8.5, -3.25) sits in chunk (0,-1), so the
        // 5x5 window origin is (-2,-3) and the eye lands at
        // (8.5+32, feet 64 - base 48 + eye, -3.25+48)
        let cam = snapshot_camera(&pose(0.0, 0.0), Some(-64), 5).expect("pose with layout");
        assert_eq!(cam.pos.x, 8.5 - (-2 * 16) as f32, "grid x over the window origin");
        assert_eq!(cam.pos.y, 64.0 - 48.0 + PLAYER_EYE_HEIGHT, "feet anchor 16 up + the standing eye");
        assert_eq!(cam.pos.z, -3.25 - (-3 * 16) as f32, "grid z over the window origin");
        // vanilla yaw 0 = facing south (+Z) -> camera yaw pi (0 = -Z);
        // the camera then looks exactly along +Z
        assert_eq!(cam.yaw, std::f32::consts::PI);
        assert_eq!(cam.pitch, 0.0);
        assert_eq!(cam.forward().z, 1.0, "south is +Z");
        assert!(cam.forward().x.abs() < 1e-6, "no sideways drift");
        // vanilla yaw 270 = east (+X): the camera faces +X exactly like
        // the procedural camera's FRAC_PI_2 east-facing default
        let cam = snapshot_camera(&pose(270.0, 0.0), Some(-64), 5).unwrap();
        assert_eq!(cam.yaw, 270.0f32.to_radians() + std::f32::consts::PI);
        assert_eq!(cam.forward().x, 1.0, "east is +X");
        assert!(cam.forward().z.abs() < 1e-6);
        // vanilla yaw 180 = north (-Z) -> camera yaw 0
        let cam = snapshot_camera(&pose(180.0, 0.0), Some(-64), 5).unwrap();
        assert_eq!(cam.forward().z, -1.0, "north is -Z");
        // vanilla pitch positive = DOWN: camera pitch flips the sign
        let cam = snapshot_camera(&pose(0.0, 30.0), Some(-64), 5).unwrap();
        assert_eq!(cam.pitch, -30.0f32.to_radians());
        assert!(cam.forward().y < 0.0, "vanilla +30 looks 30 degrees down");
        // near the world floor the clamp slides the feet up the grid
        // (y_base clamped at min_y) and the eye rides along
        let low = PlayerPosition { y: -60.0, ..pose(0.0, 0.0) };
        let cam = snapshot_camera(&low, Some(-64), 5).unwrap();
        assert_eq!(
            cam.pos.y,
            -60.0 - -64.0 as f32 + PLAYER_EYE_HEIGHT,
            "feet at grid 4 through the clamp, eye 1.62 above"
        );
    }

    #[test]
    fn materialize_snapshot_anchors_the_window_on_the_player_feet() {
        // the join_snapshot with the registry knowledge (overworld -64)
        // and a vanilla-shaped column: 7 air sections below, the
        // stone/dirt section at index 7 — the terrain sits at world y
        // 48..63, right under the teleport's feet y 64 (a real vanilla
        // join burst is exactly this: surface terrain under the spawn)
        let mut snap = join_snapshot();
        // a REAL overworld join: the vanilla dimension name rides with the
        // synced vanilla layout (the base fixture's mock name would not
        // resolve through this table)
        snap.info.dimensions = vec!["minecraft:overworld".to_string()];
        snap.info.current_dimension = "minecraft:overworld".to_string();
        snap.registry = RegistryData {
            dimension_types: vec![mc_protocol::DimensionTypeEntry {
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
            sound_events: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        snap.chunks[0].sections = (0..7)
            .map(|_| air_section())
            .chain(std::iter::once(stone_dirt_section()))
            .collect();
        // base = 64 - 16 = 48: the stone surface lands at grid y 0 and
        // the feet row (world 64) is sky
        let world = materialize_snapshot_view(snap.clone(), 5, example_mapper).world;
        assert_eq!(world.get(32, 0, 32), STONE, "world y 48 = feet 64 - anchor 16");
        assert_eq!(world.get(35, 1, 34), DIRT, "dirt one above the stone");
        assert_eq!(world.get(32, 16, 32), AIR, "the feet row is sky in this mock");
        assert_eq!(world.get(32, 47, 32), AIR, "sky up to the grid top");
        // the window FOLLOWS the player: a teleport 7 blocks higher slides
        // the view up — the surface slides out below (the truthful render,
        // no surface guess) and the real generated mapper resolves the
        // mock's example ids the same way
        let mut higher = snap;
        higher.player.y = 71.0;
        let world = materialize_snapshot_view(higher, 5, runtime_mapper).world;
        assert_eq!(world.get(32, 0, 32), AIR, "world 48 is now 7 below the window");
        assert_eq!(world.get(35, 1, 34), AIR);
    }

    /// A join milestone shaped like the e2e mock's: one real section at
    /// chunk (3,-2) (stone@(0,0,0), dirt@(3,1,2)) and the player standing
    /// inside the centered chunk (a real vanilla join burst is exactly
    /// this — the spawn position inside the view-distance center).
    fn join_snapshot() -> WorldSnapshot {
        WorldSnapshot {
            info: WorldInfo {
                entity_id: 17,
                hardcore: false,
                // a MOCK layout (one bare section) — NOT vanilla-shaped, so
                // it must not claim the vanilla name: the built-in
                // dimension table now answers a bare "minecraft:overworld"
                // with the real min_y -64 (the skip-sync fallback) and the
                // window would shift under the terrain
                dimensions: vec!["mock:test".to_string()],
                view_distance: 8,
                simulation_distance: 8,
                portal_cooldown: 300,
                current_dimension: "mock:test".to_string(),
            },
            player: PlayerPosition {
                teleport_id: 42,
                x: 56.5,
                y: 64.0,
                z: -27.25,
                dx: 0.0,
                dy: 0.0,
                dz: 0.0,
                yaw: 90.0,
                pitch: 0.0,
                flags: 0,
            },
            chunks: vec![ChunkColumn {
                x: 3,
                z: -2,
                sections: vec![stone_dirt_section()],
                light: ChunkLight::default(),
            }],
            registry: RegistryData::default(),
        }
    }

    #[test]
    fn snapshot_window_origin_centers_the_player_chunk() {
        let p = |x: f64, z: f64| PlayerPosition {
            teleport_id: 0,
            x,
            y: 64.0,
            z,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
        };
        // floor before the shift: the negative half-plane stays honest
        assert_eq!(
            snapshot_window_origin(&p(56.5, -27.25), 5),
            (1, -4),
            "player chunk (3,-2) centers the 5x5 window"
        );
        assert_eq!(
            snapshot_window_origin(&p(-0.5, 15.9), 5),
            (-3, -2),
            "floor(-0.5) = -1 -> chunk (-1,0); floor(15.9) = 15 -> chunk (0,0)"
        );
        assert_eq!(snapshot_window_origin(&p(0.0, 0.0), 5), (-2, -2));
    }

    #[test]
    fn materialize_snapshot_builds_the_player_centered_join_world() {
        // player at (56.5, -27.25) -> chunk (3,-2) -> origin (1,-4): the
        // mock column sits at grid chunk (2,2)
        let world = materialize_snapshot_view(join_snapshot(), 5, example_mapper).world;
        assert_eq!(world.chunks_per_side, 5);
        assert_eq!(world.get(32, 0, 32), STONE, "stone of column (3,-2)");
        assert_eq!(world.get(35, 1, 34), DIRT, "dirt at column-local (3,1,2)");
        // the REAL generated mapper resolves the mock's ids too — the
        // mock's example ids (1=stone, 10=dirt) coincide with the real
        // registry facts, and the mapper now proves it from the table
        let real_world = materialize_snapshot_view(join_snapshot(), 5, runtime_mapper).world;
        assert_eq!(real_world.get(32, 0, 32), STONE, "stone through the generated table");
        assert_eq!(real_world.get(35, 1, 34), DIRT, "dirt through the generated table");
    }

    /// The join milestone shaped like the e2e mock's AND like a real
    /// overworld join: the registry knowledge (overworld min_y -64) plus
    /// a vanilla-shaped column — 7 air sections below, the stone/dirt
    /// section at index 7, so the terrain sits at world y 48..63 right
    /// under the teleport's feet y 64 (the player-anchored window then
    /// shows the surface, not deepslate).
    fn surface_snapshot() -> WorldSnapshot {
        let mut snap = join_snapshot();
        // a REAL overworld join: the vanilla dimension name rides with the
        // synced vanilla layout (the base fixture's mock name would not
        // resolve through this table)
        snap.info.dimensions = vec!["minecraft:overworld".to_string()];
        snap.info.current_dimension = "minecraft:overworld".to_string();
        snap.registry = RegistryData {
            dimension_types: vec![mc_protocol::DimensionTypeEntry {
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
            sound_events: Vec::new(),
            timelines: Vec::new(),
            timeline_tags: Vec::new(),
        };
        snap.chunks[0].sections = (0..7)
            .map(|_| air_section())
            .chain(std::iter::once(stone_dirt_section()))
            .collect();
        snap
    }

    #[test]
    fn live_view_first_refresh_equals_the_one_shot_snapshot_view() {
        // the live view's FIRST refresh is exactly the one-shot snapshot
        // view: same window decisions, same grid, same pose — going live
        // never changes the join frame's shape (the pixel contract of the
        // first world screen survives the live view switch bit for bit)
        let snap = surface_snapshot();
        let one_shot = materialize_snapshot_view(snap.clone(), 5, example_mapper);
        let live = LiveView::from_snapshot(snap, 5);
        let first = live.refresh(example_mapper);
        assert_eq!(first.world.chunks_per_side, one_shot.world.chunks_per_side);
        assert_eq!(
            first.world.chunks, one_shot.world.chunks,
            "bit-identical grid: the live model's first refresh IS the snapshot"
        );
        let (a, b) = (first.camera.expect("pose"), one_shot.camera.expect("pose"));
        assert_eq!((a.pos.x, a.pos.y, a.pos.z), (b.pos.x, b.pos.y, b.pos.z));
        assert_eq!((a.yaw, a.pitch), (b.yaw, b.pitch));
    }

    #[test]
    fn the_snapshot_view_carries_the_open_container() {
        // the join's own state: no window is open — the container lane
        // rides `None` (the one-shot and the live first refresh agree)
        let snap = surface_snapshot();
        let one_shot = materialize_snapshot_view(snap.clone(), 5, example_mapper);
        assert!(one_shot.container.is_none(), "a fresh join opens no window");
        let mut live = LiveView::from_snapshot(snap, 5);
        assert!(live.refresh(example_mapper).container.is_none());

        // the server opens the chest (window 5, generic_9x3) and fills
        // it — the projection rides EVERY refresh from here on
        live.apply(PlayEvent::ScreenOpened(mc_protocol::play::OpenScreen {
            container_id: 5,
            menu_type_id: crate::world_model::MENU_TYPE_CHEST, // generic_9x3, the chest
            title_nbt: vec![0x0A, 0x00],
        }));
        live.apply(PlayEvent::ContainerFilled(mc_protocol::play::ContainerContent {
            container_id: 5,
            state_id: 11,
            items: vec![
                mc_protocol::play::ItemStack {
                    count: 3,
                    item_id: 745,
                    patch: mc_protocol::play::ComponentPatch::default(),
                },
                mc_protocol::play::ItemStack {
                    count: 0,
                    item_id: 0,
                    patch: mc_protocol::play::ComponentPatch::default(),
                },
            ],
            carried_item: mc_protocol::play::ItemStack {
                count: 9,
                item_id: 33,
                patch: mc_protocol::play::ComponentPatch::default(),
            },
        }));
        let view = live.refresh(example_mapper);
        let c = view.container.expect("the open window rides the view");
        assert_eq!(c.id, 5);
        assert_eq!(c.menu_type_id, 2, "the chest's menu id rides verbatim");
        assert_eq!(c.slots, vec![(745, 3), (0, 0)], "the store's slots, verbatim pairs");
        assert_eq!(c.carried, (33, 9), "the cursor stack rides the projection");

        // the store DROPS (the Container Close): the lane closes — the
        // next refresh reports no window, the screen side's cue
        live.apply(PlayEvent::ContainerClosed(mc_protocol::play::ContainerClose {
            container_id: 5,
        }));
        assert!(live.refresh(example_mapper).container.is_none(), "the close lands");
    }

    #[test]
    fn the_snapshot_view_extracts_the_bundle_rows() {
        // the set-slot sync's projection: the slots whose component
        // patch carries the `bundle_contents` value ride the view's
        // bundle_rows — the menu slot id keyed with the entries'
        // (id, count) pairs in wire order; the plain slots and the
        // absence of the component stay rowless
        let mut live = LiveView::from_snapshot(surface_snapshot(), 5);
        live.apply(PlayEvent::ScreenOpened(mc_protocol::play::OpenScreen {
            container_id: 5,
            menu_type_id: crate::world_model::MENU_TYPE_CHEST,
            title_nbt: vec![0x0A, 0x00],
        }));
        let bundle = mc_registry::item_id_by_name("bundle").expect("the bundle the table keeps");
        let bundle_patch = mc_protocol::play::ComponentPatch {
            added: vec![(
                mc_protocol::play::data_component_types::BUNDLE_CONTENTS,
                mc_protocol::play::ComponentValue::BundleContents(vec![
                    mc_protocol::play::BundleEntry {
                        item_id: 926,
                        count: 12,
                        patch: mc_protocol::play::ComponentPatch::default(),
                    },
                    mc_protocol::play::BundleEntry {
                        item_id: 921,
                        count: 5,
                        patch: mc_protocol::play::ComponentPatch::default(),
                    },
                ]),
            )],
            removed: Vec::new(),
        };
        let empty_bundle_patch = mc_protocol::play::ComponentPatch {
            added: vec![(
                mc_protocol::play::data_component_types::BUNDLE_CONTENTS,
                mc_protocol::play::ComponentValue::BundleContents(Vec::new()),
            )],
            removed: Vec::new(),
        };
        let stack = |count, item_id, patch| mc_protocol::play::ItemStack {
            count,
            item_id,
            patch,
        };
        live.apply(PlayEvent::ContainerFilled(mc_protocol::play::ContainerContent {
            container_id: 5,
            state_id: 11,
            items: vec![
                stack(0, 0, Default::default()), // the empty grid heads
                stack(0, 0, Default::default()),
                stack(0, 0, Default::default()),
                stack(0, 0, Default::default()),
                stack(1, bundle, bundle_patch),   // slot 4: the loaded bundle
                stack(7, 745, Default::default()), // slot 5: the plain stack
                stack(1, bundle, empty_bundle_patch), // slot 6: the EMPTY bundle
            ],
            carried_item: stack(0, 0, Default::default()),
        }));
        let view = live.refresh(example_mapper);
        let c = view.container.expect("the open window rides the view");
        assert_eq!(
            c.bundle_rows,
            vec![(4, vec![(926, 12), (921, 5)]), (6, Vec::new())],
            "the bundle-bearing slots the extraction keys, wire order"
        );
    }

    #[test]
    fn the_window0_store_projection_rides_the_view() {
        // the player's OWN inventory window: the store the window-0
        // sync feeds projects BOTH lanes — the slots keyed by the
        // menu id (the index IS the id, the never-spoken tail the
        // (0, 0) emptiness keeps) and the bundle rows the patches
        // carry (the same extraction the chest lane speaks)
        let mut live = LiveView::from_snapshot(surface_snapshot(), 5);
        let bundle = mc_registry::item_id_by_name("bundle").expect("the bundle the table keeps");
        let bundle_patch = mc_protocol::play::ComponentPatch {
            added: vec![(
                mc_protocol::play::data_component_types::BUNDLE_CONTENTS,
                mc_protocol::play::ComponentValue::BundleContents(vec![
                    mc_protocol::play::BundleEntry {
                        item_id: 926,
                        count: 12,
                        patch: mc_protocol::play::ComponentPatch::default(),
                    },
                ]),
            )],
            removed: Vec::new(),
        };
        let stack = |count, item_id, patch| mc_protocol::play::ItemStack {
            count,
            item_id,
            patch,
        };
        // the window-0 full snapshot: the loaded bundle rides the
        // main grid's second cell (menu id 10), a plain stack the
        // first cell (menu id 9) — the InventoryMenu numbering, the
        // never-spoken heads the empty pairs keep
        let mut items = vec![stack(0, 0, Default::default()); 9];
        items.push(stack(7, 745, Default::default())); // slot 9: the plain stack
        items.push(stack(1, bundle, bundle_patch)); // slot 10: the loaded bundle
        live.apply(PlayEvent::ContainerFilled(mc_protocol::play::ContainerContent {
            container_id: crate::world_model::INVENTORY_WINDOW_ID,
            state_id: 3,
            items,
            carried_item: stack(0, 0, Default::default()),
        }));
        let view = live.refresh(example_mapper);
        assert_eq!(
            view.inventory_slots[9],
            (745, 7),
            "the plain stack the menu id keys"
        );
        assert_eq!(
            view.inventory_slots[10],
            (bundle, 1),
            "the bundle the menu id keys"
        );
        assert_eq!(
            view.inventory_slots[40],
            (0, 0),
            "the never-spoken tail the emptiness keeps"
        );
        assert_eq!(
            view.inventory_bundles,
            vec![(10, vec![(926, 12)])],
            "the bundle rows the patches carry"
        );
    }

    #[test]
    fn live_view_streams_chunks_and_follows_the_player() {
        let mut live = LiveView::from_snapshot(surface_snapshot(), 5);
        // the join view: the mock column at grid chunk (2,2), stone at
        // (32, 0, 32) (world y 48 = feet 64 - anchor 16), pose from
        // teleport 42
        let view = live.refresh(example_mapper);
        assert_eq!(view.world.get(32, 0, 32), STONE);
        let cam0 = view.camera.expect("join pose");
        assert_eq!(cam0.pos.x, 56.5 - 16.0, "grid x over origin (1,-4)");

        // a second column streams in NEXT DOOR (chunk (3,-1) -> grid
        // (2,3)): the SAME model absorbs it, the refreshed window shows it
        let second_col = ChunkColumn {
            x: 3,
            z: -1,
            sections: (0..7)
                .map(|_| air_section())
                .chain(std::iter::once(stone_dirt_section()))
                .collect(),
            light: ChunkLight::default(),
        };
        assert!(live.apply(PlayEvent::Chunk(second_col)));
        let w = live.refresh(example_mapper).world;
        assert_eq!(w.get(32, 0, 32), STONE, "the join column is still there");
        assert_eq!(w.get(32, 0, 48), STONE, "the streamed column landed next door");

        // an unload drops it again — the model is the CURRENT world
        assert!(live.apply(PlayEvent::UnloadChunk { x: 3, z: -1 }));
        assert_eq!(live.refresh(example_mapper).world.get(32, 0, 48), AIR);

        // probes never dirty the view (nothing world-state-shaped arrived)
        assert!(!live.apply(PlayEvent::KeepAlive(3)));
        assert!(!live.apply(PlayEvent::ChunkBatch));

        // a new teleport re-anchors the window AND the pose: the player
        // moves one chunk east (x 56.5 -> 72.5, chunk 3 -> 4) — the
        // window origin slides to (2,-4), the join column moves to grid
        // chunk (1,2) and the camera rides the server's new pose
        assert!(live.apply(PlayEvent::Position(PlayerPosition {
            teleport_id: 43,
            x: 72.5,
            y: 64.0,
            z: -27.25,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 180.0,
            pitch: 0.0,
            flags: 0,
        })));
        let view = live.refresh(example_mapper);
        assert_eq!(
            view.world.get(16, 0, 32),
            STONE,
            "the join column slid one grid chunk west with the window"
        );
        let cam = view.camera.expect("the pose follows the teleport");
        assert_eq!(cam.pos.x, 72.5 - 32.0, "grid x over the NEW origin (2,-4)");
        assert_eq!(
            cam.yaw,
            180.0f32.to_radians() + std::f32::consts::PI,
            "the server's new look direction"
        );
    }

    #[test]
    fn snapshot_view_carries_the_biome_at_the_player() {
        // the synced biome table (ids 0/1) + the player's section
        // (index 8, world y 64..79): the player's cell (2,0,1) — local
        // x 8, z 5, y row 0 — carries swamp (id 1), everything else
        // plains (id 0)
        let mut cells = [0u32; 64];
        cells[6] = 1;
        let mut snap = surface_snapshot();
        snap.registry.biomes = vec![biome_entry("minecraft:plains"), biome_entry("minecraft:swamp")];
        snap.chunks[0].sections.push(biome_cells_section(&[0, 1], &cells));
        // the one-shot materialize fills the SAME field (the join
        // worker's milestone ship reads it)
        let one_shot = materialize_snapshot_view(snap.clone(), 5, example_mapper);
        assert_eq!(
            one_shot.biome.as_deref(),
            Some("minecraft:swamp"),
            "the biome at the teleport's feet, through the synced names"
        );
        let mut live = LiveView::from_snapshot(snap, 5);
        let first = live.refresh(example_mapper);
        assert_eq!(first.biome, one_shot.biome, "first refresh == one-shot, biome too");
        // four blocks up: the NEXT y cell (local y 6 -> cell row 1) —
        // the biome follows the pose like the window and the camera do
        let pose = live.model.player.clone().expect("pose");
        assert!(live.apply(PlayEvent::Position(PlayerPosition {
            teleport_id: 43,
            y: 70.0,
            ..pose
        })));
        let second = live.refresh(example_mapper);
        assert_eq!(
            second.biome.as_deref(),
            Some("minecraft:plains"),
            "a new teleport re-reads the biome at the new pose"
        );
        // the synced names gone: the model is in the SKIP-SYNC state —
        // the bare vanilla id 0 now walks the BUILT-IN wire order and
        // names badlands, the same fallback the tint quads ride. The
        // honest None survives for ids the vanilla order never names and
        // for syncs that DID arrive (world_model pins both).
        live.model.registry.biomes.clear();
        assert_eq!(
            live.refresh(example_mapper).biome.as_deref(),
            Some("minecraft:badlands"),
            "the skip-sync fallback names the bare vanilla wire id"
        );
    }

    /// The first-person hand's lane: the store's SELECTED hotbar stack
    /// rides the view — the filled-but-not-selected slots do not, the
    /// selection's move re-reads the slice, the empty slot is the bare
    /// arm's None.
    #[test]
    fn the_selected_hotbar_rides_the_view_as_the_held_stack() {
        use mc_protocol::play::{parse_set_container_slot, parse_set_held_slot};
        let mut live = LiveView::from_snapshot(surface_snapshot(), 5);
        // the store silent: the bare arm
        assert_eq!(live.refresh(example_mapper).held, None);
        // a diamond lands at the hotbar's slot 3 (the USE_ROW's window
        // numbering 39) — but the selection still sits at slot 0
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 0); // window 0 — the player's inventory
        write_varint(&mut p, 1); // state id
        p.extend_from_slice(&(USE_ROW_SLOT_START as i16 + 3).to_be_bytes());
        write_varint(&mut p, 12); // the count
        write_varint(&mut p, 926); // the diamond's wire id
        write_varint(&mut p, 0); // patch: none added
        write_varint(&mut p, 0); // patch: none removed
        let s = parse_set_container_slot(&p).unwrap();
        assert!(live.apply(PlayEvent::ContainerSlotSet(s)));
        assert_eq!(
            live.refresh(example_mapper).held,
            None,
            "filled but not selected — the hand does not guess"
        );
        // the selection moves to slot 3: the diamond rides the view
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        write_varint(&mut p, 3);
        let s = parse_set_held_slot(&p).unwrap();
        assert!(live.apply(PlayEvent::HeldSlotSet(s)));
        assert_eq!(live.refresh(example_mapper).held, Some((926, 12)));
        // the selection moves to the EMPTY slot 4: the bare arm returns
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        write_varint(&mut p, 4);
        let s = parse_set_held_slot(&p).unwrap();
        assert!(live.apply(PlayEvent::HeldSlotSet(s)));
        assert_eq!(live.refresh(example_mapper).held, None);
    }

    /// The HUD's lane: the store's WHOLE hotbar row rides the view —
    /// verbatim in the inventory window's own order (never re-ordered
    /// by the selection: the highlight is the highlight's business),
    /// the never-spoken tail the `(0, 0)` emptiness, and a clearing
    /// Set Slot updates the row in place.
    #[test]
    fn the_hotbar_row_rides_the_view_for_the_hud() {
        use mc_protocol::play::{parse_set_container_slot, parse_set_held_slot};
        let mut live = LiveView::from_snapshot(surface_snapshot(), 5);
        // the store silent: the all-empty row (the chrome-only HUD)
        assert_eq!(live.refresh(example_mapper).hotbar, [(0, 0); 9]);
        // the wire speaks: an apple lands at hotbar slot 0 (the USE_ROW's
        // window numbering 36), a diamond at slot 3 (39) — the row reads
        // them at their OWN indexes while the selection still sits at 0
        for (window_slot, wire_id, count) in
            [(USE_ROW_SLOT_START, 921i32, 5i32), (USE_ROW_SLOT_START + 3, 926, 12)]
        {
            let mut p = Vec::new();
            write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
            write_varint(&mut p, 0); // window 0 — the player's inventory
            write_varint(&mut p, 1); // state id
            p.extend_from_slice(&(window_slot as i16).to_be_bytes());
            write_varint(&mut p, count);
            write_varint(&mut p, wire_id);
            write_varint(&mut p, 0); // patch: none added
            write_varint(&mut p, 0); // patch: none removed
            let s = parse_set_container_slot(&p).unwrap();
            assert!(live.apply(PlayEvent::ContainerSlotSet(s)));
        }
        let row = live.refresh(example_mapper).hotbar;
        assert_eq!(row[0], (921, 5), "the apple at its own index");
        assert_eq!(row[3], (926, 12), "the diamond at its own index");
        assert_eq!(row[8], (0, 0), "the never-spoken tail stays empty");
        // the selection moves to slot 3: the ROW DOES NOT MOVE — the
        // highlight is the selection's own business (the held lane's
        // test pins the hand's re-read; the HUD's order is the store's)
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::SET_HELD_SLOT);
        write_varint(&mut p, 3);
        let s = parse_set_held_slot(&p).unwrap();
        assert!(live.apply(PlayEvent::HeldSlotSet(s)));
        let row = live.refresh(example_mapper).hotbar;
        assert_eq!(row[0], (921, 5));
        assert_eq!(row[3], (926, 12));
        // a clearing Set Slot empties the diamond's seat in place
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CONTAINER_SET_SLOT);
        write_varint(&mut p, 0); // window 0
        write_varint(&mut p, 2); // state id
        p.extend_from_slice(&((USE_ROW_SLOT_START + 3) as i16).to_be_bytes());
        write_varint(&mut p, 0); // the wire's own emptiness
        write_varint(&mut p, 0);
        write_varint(&mut p, 0);
        write_varint(&mut p, 0);
        let s = parse_set_container_slot(&p).unwrap();
        assert!(live.apply(PlayEvent::ContainerSlotSet(s)));
        let row = live.refresh(example_mapper).hotbar;
        assert_eq!(row[3], (0, 0), "the cleared slot empties in place");
        assert_eq!(row[0], (921, 5), "the rest of the row stands");
    }

    #[test]
    fn live_view_respawn_crosses_the_dimension() {
        // the vanilla crossing: Respawn re-pins the layout (the nether's
        // min_y 0), DROPS the old dimension's level (the columns clear —
        // overworld blocks re-read under the nether min_y would be pure
        // fiction) and the window truthfully shows air until the new
        // dimension's own chunks stream in, which then anchor on the new
        // pose — the full arc, the same model the screen renders
        let mut snap = surface_snapshot();
        snap.registry.dimension_types.push(mc_protocol::DimensionTypeEntry {
            name: "minecraft:the_nether".to_string(),
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
        });
        // the synced biome table BOTH dimensions share (a real vanilla
        // server streams the whole worldgen/biome registry): plains 0,
        // crimson_forest 1 — and the player's own overworld section
        // (world y 64..79) carries the overworld biome's cells
        snap.registry.biomes = vec![
            biome_entry("minecraft:plains"),
            biome_entry("minecraft:crimson_forest"),
        ];
        snap.chunks[0]
            .sections
            .push(biome_cells_section(&[0], &[0u32; 64]));
        let mut live = LiveView::from_snapshot(snap, 5);
        // the overworld join view stands (stone at grid y 0, feet-anchored)
        // and the biome at the player's feet is the spawn biome's NAME
        let join = live.refresh(example_mapper);
        assert_eq!(join.world.get(32, 0, 32), STONE);
        assert_eq!(
            join.biome.as_deref(),
            Some("minecraft:plains"),
            "the overworld biome resolves through the synced names"
        );
        assert_eq!(live.model.min_y, Some(-64));
        assert_eq!(live.model.loaded_chunks(), 1);
        // THE CROSSING: Respawn to the nether
        assert!(live.apply(PlayEvent::Respawn(SpawnInfo {
            dimension_type_id: 1,
            dimension: "minecraft:the_nether".to_string(),
            portal_cooldown: 60,
        })));
        assert_eq!(live.model.min_y, Some(0), "the nether layout re-pins");
        assert_eq!(live.model.loaded_chunks(), 0, "the overworld level dropped");
        let mid = live.refresh(example_mapper);
        assert!(
            mid.world.chunks.iter().all(|c| c.iter().all(|&b| b == AIR)),
            "the mid-transition window is honestly all air (the loading state)"
        );
        assert!(mid.biome.is_none(), "no biome without a loaded column");
        // the nether's own chunk + spawn teleport land: the window
        // re-anchors on the NEW dimension — the stone at nether world 0
        // sits at grid y 0 again (feet 0, anchor 0 through the clamp)
        assert!(live.apply(PlayEvent::Chunk(ChunkColumn {
            x: 3,
            z: -2,
            sections: vec![stone_dirt_section()],
            light: ChunkLight::default(),
        })));
        assert!(live.apply(PlayEvent::Position(PlayerPosition {
            teleport_id: 44,
            x: 56.5,
            y: 0.0,
            z: -27.25,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            yaw: 90.0,
            pitch: 0.0,
            flags: 0,
        })));
        let nether = live.refresh(example_mapper);
        assert_eq!(
            nether.world.get(32, 0, 32),
            STONE,
            "the nether column under the nether min_y, feet-anchored like the join"
        );
        let cam = nether.camera.expect("the nether pose drives the camera");
        assert_eq!(cam.pos.y, PLAYER_EYE_HEIGHT, "feet world 0, base 0, the standing eye");
        assert_eq!(
            nether.biome.as_deref(),
            Some("minecraft:crimson_forest"),
            "the crossing resolves the NETHER's own biome at the new pose — \
             the shipped change re-fires the music situation (screens layer)"
        );
    }

    #[test]
    fn live_view_without_a_pose_materializes_the_honest_window() {
        // no teleport ever arrived: the deterministic window (origin
        // 0,0), the layout-known y base or column-local 0, NO camera —
        // the honest state, never a guess
        let mut live = LiveView::from_snapshot(surface_snapshot(), 5);
        live.model.player = None; // the model field is pub: tests may strip the pose
        let view = live.refresh(example_mapper);
        assert!(view.camera.is_none(), "no pose known, no pose shipped");
        assert!(view.biome.is_none(), "no pose known, no biome shipped");
        // origin (0,0) covers chunks 0..=4 / 0..=4 — the join column at
        // (3,-2) is OUTSIDE it: the window is honestly all air
        assert!(view
            .world
            .chunks
            .iter()
            .all(|c| c.iter().all(|&b| b == AIR)));
    }

    /// The pickup prediction's semantics table — the vanilla
    /// `AbstractContainerMenu.doClick` PICKUP (button 0) over the
    /// believed (item_id, count) pairs: place, pick up, merge-to-ceiling,
    /// the full-slot refusal, the swap, and the both-empty no-op. These
    /// are the LOCAL belief's laws (the server's corrections replace it
    /// wholesale — the container lane's own contract).
    #[test]
    fn the_pickup_prediction_rides_the_vanilla_table() {
        // both empty — nothing to do
        assert_eq!(predict_pickup((0, 0), (0, 0)), ((0, 0), (0, 0)));
        // an empty cursor picks up the whole stack
        assert_eq!(predict_pickup((926, 12), (0, 0)), ((0, 0), (926, 12)));
        // a held stack places whole into an empty slot
        assert_eq!(predict_pickup((0, 0), (926, 12)), ((926, 12), (0, 0)));
        // the same item merges up to the ceiling — the remainder rides
        assert_eq!(predict_pickup((926, 60), (926, 12)), ((926, 64), (926, 8)));
        // the merge that empties the cursor is the (0, 0) sentinel
        assert_eq!(predict_pickup((926, 60), (926, 4)), ((926, 64), (0, 0)));
        // a full slot takes nothing (the vanilla pickup's own refusal)
        assert_eq!(predict_pickup((926, 64), (926, 5)), ((926, 64), (926, 5)));
        // different items swap
        assert_eq!(predict_pickup((926, 12), (921, 3)), ((921, 3), (926, 12)));
        // the ceiling is the default 64 — the component-carried true
        // maxes are the store's future lanes (the doc's own honesty)
        assert_eq!(ITEM_MAX_STACK_DEFAULT, 64);
    }

    /// The right-press prediction's semantics table — the doClick
    /// PICKUP buttonnum != 0 branch over the believed pairs: the
    /// CEILING-half take, the place-one, the move-one, and the
    /// refusals (a different item never swaps on the right press).
    #[test]
    fn the_right_press_prediction_rides_the_vanilla_table() {
        // both empty — nothing to do
        assert_eq!(predict_pickup_right((0, 0), (0, 0)), ((0, 0), (0, 0)));
        // an empty cursor takes the CEILING half: 12 -> 6 + 6
        assert_eq!(predict_pickup_right((926, 12), (0, 0)), ((926, 6), (926, 6)));
        // the odd count rounds UP: 5 -> 3 taken, 2 left
        assert_eq!(predict_pickup_right((926, 5), (0, 0)), ((926, 2), (926, 3)));
        // the single empties the slot
        assert_eq!(predict_pickup_right((926, 1), (0, 0)), ((0, 0), (926, 1)));
        // a held stack places ONE — the last item empties the cursor
        assert_eq!(predict_pickup_right((0, 0), (926, 12)), ((926, 1), (926, 11)));
        assert_eq!(predict_pickup_right((0, 0), (926, 1)), ((926, 1), (0, 0)));
        // the same item with room moves ONE
        assert_eq!(predict_pickup_right((926, 12), (926, 5)), ((926, 13), (926, 4)));
        // the move that empties the cursor is the sentinel
        assert_eq!(predict_pickup_right((926, 12), (926, 1)), ((926, 13), (0, 0)));
        // a different item NEVER swaps on the right press
        assert_eq!(predict_pickup_right((926, 12), (921, 3)), ((926, 12), (921, 3)));
        // a full slot takes nothing
        assert_eq!(predict_pickup_right((926, 64), (926, 5)), ((926, 64), (926, 5)));
    }

    /// The QUICK_MOVE (shift-click) prediction's semantics table — the
    /// doClick third branch over the FULL believed menu: the
    /// grid-to-mirror lane fills from the END (the hotbar first), the
    /// mirror-to-grid lane fills slot 0 forward, the same-item merge
    /// precedes the empty-slot fill, and the cursor never moves.
    #[test]
    fn the_quick_move_prediction_rides_the_vanilla_table() {
        // the empty source: nothing to move, no belief entries
        let m = vec![(0, 0); 63];
        assert!(predict_quick_move(&m, 2).is_empty());
        // the grid -> the mirror, the hotbar first: the empty mirror's
        // first tried slot is the menu's 62 (the fillFromEnd order)
        let mut m = vec![(0, 0); 63];
        m[2] = (926, 30);
        assert_eq!(
            predict_quick_move(&m, 2),
            vec![(2, (0, 0)), (62, (926, 30))],
            "the whole stack rides, the source emptied"
        );
        // the merge pass precedes the empty fill: the same-item stack
        // takes what fits up to the ceiling
        let mut m = vec![(0, 0); 63];
        m[2] = (926, 24);
        m[50] = (926, 40);
        assert_eq!(
            predict_quick_move(&m, 2),
            vec![(2, (0, 0)), (50, (926, 64))],
            "the merge filled the ceiling, no empty slot touched"
        );
        // the merge then the remainder: what the ceiling refused rides
        // the hotbar's last slot (the pass order's own number)
        let mut m = vec![(0, 0); 63];
        m[2] = (926, 50);
        m[50] = (926, 40);
        assert_eq!(
            predict_quick_move(&m, 2),
            vec![(2, (0, 0)), (50, (926, 64)), (62, (926, 26))],
            "the merge, then the remainder's empty fill"
        );
        // a different item is refused by the merge pass — the empty
        // fill takes the whole stack
        let mut m = vec![(0, 0); 63];
        m[2] = (926, 30);
        m[55] = (921, 3);
        assert_eq!(
            predict_quick_move(&m, 2),
            vec![(2, (0, 0)), (62, (926, 30))],
            "the apple's slot untouched"
        );
        // the mirror -> the grid, slot 0 forward: the vanilla's own
        // fillFromEnd=false order
        let mut m = vec![(0, 0); 63];
        m[30] = (921, 12);
        assert_eq!(predict_quick_move(&m, 30), vec![(30, (0, 0)), (0, (921, 12))]);
        // the mirror's merge fills the grid's low slots first
        let mut m = vec![(0, 0); 63];
        m[30] = (921, 12);
        m[5] = (921, 60);
        assert_eq!(
            predict_quick_move(&m, 30),
            vec![(30, (0, 0)), (5, (921, 64)), (0, (921, 8))],
            "the merge into the grid's slot 5, the remainder at slot 0"
        );
        // no room anywhere (every destination slot is a different full
        // item): the stack stays, the belief did not move
        let mut m = vec![(921, 64); 63];
        m[2] = (926, 64);
        assert!(predict_quick_move(&m, 2).is_empty());
        // the overfill cap: the empty slot takes at most the ceiling —
        // the remainder STAYS in the source (the server repairs the
        // store's future component maxes)
        let mut m = vec![(0, 0); 63];
        m[2] = (926, 64);
        m[50] = (926, 60);
        assert_eq!(
            predict_quick_move(&m, 2),
            vec![(2, (0, 0)), (50, (926, 64)), (62, (926, 60))],
            "the merge took 4, the empty fill took the 60"
        );
    }

    #[test]
    fn the_sidebar_projection_sorts_and_caps() {
        use crate::world_model::{ObjectiveState, ScoreState};
        use mc_protocol::{
            DisplayObjectiveUpdate, ObjectiveUpdate, PlayEvent, ScoreUpdate, OBJECTIVE_METHOD_ADD,
            RENDER_TYPE_INTEGER,
        };
        let mut model = WorldModel::new();
        assert!(
            model.apply(PlayEvent::Objective(ObjectiveUpdate {
                objective: "money".into(),
                method: OBJECTIVE_METHOD_ADD,
                display: Some("Coins".into()),
                render_type: RENDER_TYPE_INTEGER,
            })),
            "the objective add"
        );
        assert!(
            model.apply(PlayEvent::DisplayObjective(DisplayObjectiveUpdate {
                slot: 1,
                objective: "money".into(),
            })),
            "the sidebar slot set"
        );
        assert!(model.apply(PlayEvent::Score(ScoreUpdate {
            owner: "Amy".into(),
            objective: "money".into(),
            score: 5,
            display: None,
        })));
        assert!(model.apply(PlayEvent::Score(ScoreUpdate {
            owner: "bob".into(),
            objective: "money".into(),
            score: 10,
            display: None,
        })));
        assert!(model.apply(PlayEvent::Score(ScoreUpdate {
            owner: "Zed".into(),
            objective: "money".into(),
            score: 10,
            display: Some("Zed Inc.".into()),
        })));
        // the hidden holder (the '#' convention) drops; a foreign
        // objective's row never rides this sidebar
        model.objectives.insert(
            "hidden".into(),
            ObjectiveState { display: String::new(), render_type: 0 },
        );
        model.scores.insert(
            ("#server_ticks".into(), "money".into()),
            ScoreState { value: 999, display: None },
        );
        model.scores.insert(
            ("Amy".into(), "hidden".into()),
            ScoreState { value: 777, display: None },
        );

        let sb = sidebar_view(&model).expect("the slot speaks");
        assert_eq!(sb.title, "Coins");
        let names: Vec<(&str, i32)> =
            sb.lines.iter().map(|l| (l.name.as_str(), l.score)).collect();
        // the value-DESCENDING order; the 10-tie breaks case-insensitive
        // on the OWNER ascending (bob < zed — the comparator's own law)
        assert_eq!(
            names,
            vec![("bob", 10), ("Zed Inc.", 10), ("Amy", 5)],
            "the display override names the row, the tie rides the owner ci"
        );

        // the cap: 17 rows walk out 15 (the stream's own limit)
        for i in 0..17 {
            model.scores.insert(
                (format!("holder{i:02}"), "money".into()),
                ScoreState { value: i, display: None },
            );
        }
        let sb = sidebar_view(&model).unwrap();
        assert_eq!(sb.lines.len(), crate::world_view::SIDEBAR_ROW_CAP, "the cap");
        assert_eq!(sb.lines[0].score, 16, "the top row is the max");

        // the silent slot: the display cleared (or the objective gone)
        // is the honest None — never an empty guess
        model.display_slots.remove(&1);
        assert!(sidebar_view(&model).is_none(), "no slot, no sidebar");
        model.display_slots.insert(1, "ghost".into());
        assert!(sidebar_view(&model).is_none(), "no objective, no sidebar");
    }

    #[test]
    fn the_ride_projection_keeps_the_boat_anchor() {
        use mc_protocol::play::EntitySpawn;
        use mc_protocol::{PlayEvent, SetPassengers};
        let mut model = WorldModel::new();
        // the login pins the local entity id; the registry names the
        // population the family gate reads (type id 0 = the boat, id 1
        // = the zombie — the honest contrast the gate needs)
        model.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 1,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        }));
        model.registry.entity_types =
            vec!["minecraft:boat".to_string(), "minecraft:zombie".to_string()];
        for (id, type_id) in [(1, 1), (7, 0), (8, 1), (9, 0)] {
            model.apply(PlayEvent::Spawn(EntitySpawn {
                id,
                uuid: id as u128,
                type_id,
                x: 10.0,
                y: 64.0,
                z: -20.0,
                movement: [0.0; 3],
                x_rot: 0.0,
                y_rot: 90.0,
                y_head_rot: 0.0,
                data: 0,
            }));
        }
        // THE HONEST SILENCE: no belief yet — the walk owns the player
        assert_eq!(ride_view(&model), None, "nothing carries yet");

        // THE FAMILY GATE: the mount lands on the ZOMBIE (the belief
        // accepts it — the vanilla handler only checks existence) but
        // the projection refuses the anchor (the boat family only)
        model.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 8,
            passengers: vec![1],
        }));
        assert_eq!(ride_view(&model), None, "a zombie is no boat");

        // THE SINGLE-RIDER ANCHOR: boat 7 carries the player — the
        // attachment = Vec3(0, height/3, 0.0), the seat x 0.0, the yaw
        // rotation of a zero offset never moves it
        model.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 7,
            passengers: vec![1],
        }));
        let ride = ride_view(&model).expect("the boat carries");
        let ride_height = (BOAT_DIMENSIONS_HEIGHT / 3.0) as f64;
        assert_eq!(
            (ride.feet_x, ride.feet_y, ride.feet_z),
            (10.0, 64.0 + ride_height, -20.0),
            "the single rider sits over the keel, the ride height above"
        );
        assert_eq!(ride.vehicle_yaw, 90.0, "the mount snap's source");

        // THE MULTI-RIDER SEATS: boat 9 carries boat 7's mate in the
        // front seat and the player behind — the front seat of a
        // multi-rider list at +0.2, the player's second seat at −0.6,
        // the offsets rotated by −yaw (90° folds the local x into the
        // world z, the cos/sin the vanilla Vec3.yRot speaks)
        model.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 9,
            passengers: vec![7, 1],
        }));
        let ride = ride_view(&model).expect("the boat carries two");
        let theta = -90.0f64 * std::f64::consts::PI / 180.0;
        let (sin, cos) = theta.sin_cos();
        let lx = BOAT_BACK_SEAT_OFFSET as f64;
        assert!(
            (ride.feet_x - 10.0).abs() < 1e-9,
            "the rotated x: x·cos + z·sin with x=−0.6, z=0, cos(−90°)=0"
        );
        assert!(
            (ride.feet_y - (64.0 + ride_height)).abs() < 1e-9,
            "the ride height the height/3 law keeps"
        );
        assert!(
            (ride.feet_z - (-20.0 + (-lx) * sin)).abs() < 1e-9,
            "the rotated z: −x·sin + z·cos (sin(−90°) = −1 flips the sign)"
        );

        // THE FRONT SEAT'S OWN OFFSET (the index-0 rider's +0.2 the
        // projection ships when the PLAYER holds it)
        model.apply(PlayEvent::SetPassengers(SetPassengers {
            vehicle: 9,
            passengers: vec![1, 7],
        }));
        let ride = ride_view(&model).expect("the player holds the front seat");
        let lx = BOAT_FRONT_SEAT_OFFSET as f64;
        assert!(
            (ride.feet_z - (-20.0 + (-lx) * sin)).abs() < 1e-9,
            "the +0.2 front seat rotated with the boat"
        );
    }

    #[test]
    fn the_boat_name_parse_keeps_the_family_bits() {
        use crate::world_view::boat_family_parts;
        // THE FOUR SHAPES (the 26.2 registry's per-wood entity types —
        // the wood the name itself speaks)
        assert_eq!(
            boat_family_parts("minecraft:oak_boat"),
            Some(("boat", false, "oak"))
        );
        assert_eq!(
            boat_family_parts("minecraft:bamboo_raft"),
            Some(("raft", false, "bamboo"))
        );
        assert_eq!(
            boat_family_parts("minecraft:oak_chest_boat"),
            Some(("boat", true, "oak"))
        );
        assert_eq!(
            boat_family_parts("minecraft:bamboo_chest_raft"),
            Some(("raft", true, "bamboo"))
        );
        // THE CHEST SUFFIX READS FIRST (every *_chest_boat ends with
        // *_boat too — the plain walk the wood would corrupt)
        let (_, chest, wood) = boat_family_parts("minecraft:dark_oak_chest_boat").unwrap();
        assert!(chest, "the chest bit the suffix speaks");
        assert_eq!(wood, "dark_oak", "the wood the chest-first walk keeps");
        // THE BARE NAMES REFUSE (the pre-split naming — no wood, no
        // texture, no render; the ride anchor still carries them)
        assert_eq!(boat_family_parts("minecraft:boat"), None);
        assert_eq!(boat_family_parts("minecraft:chest_boat"), None);
        // no prefix, no family — the registry's own names verbatim
        assert_eq!(boat_family_parts("boat"), None);
        assert_eq!(boat_family_parts("minecraft:zombie"), None);
        assert_eq!(boat_family_parts("minecraft:pig"), None);
    }

    #[test]
    fn the_boat_projection_walks_the_family_store() {
        use crate::world_view::boats_view;
        use mc_protocol::play::EntitySpawn;
        use mc_protocol::{PlayEvent, WorldInfo};
        let mut model = WorldModel::new();
        model.apply(PlayEvent::WorldInfo(WorldInfo {
            entity_id: 1,
            hardcore: false,
            dimensions: vec!["minecraft:overworld".to_string()],
            view_distance: 8,
            simulation_distance: 8,
            portal_cooldown: 300,
            current_dimension: "minecraft:overworld".to_string(),
        }));
        // the registry names the family: 0 the oak boat, 1 the zombie
        // (the honest contrast), 2 the bamboo raft, 3 the oak chest
        // boat, 4 the BARE boat (the pre-split name the render refuses)
        model.registry.entity_types = vec![
            "minecraft:oak_boat".to_string(),
            "minecraft:zombie".to_string(),
            "minecraft:bamboo_raft".to_string(),
            "minecraft:oak_chest_boat".to_string(),
            "minecraft:boat".to_string(),
        ];
        for (id, type_id, y, yaw) in [
            (7, 0, 64.0, 90.0f32),
            (8, 1, 65.0, 0.0),
            (9, 2, 66.0, 180.0),
            (10, 3, 67.0, 270.0),
            (11, 4, 68.0, 0.0),
        ] {
            model.apply(PlayEvent::Spawn(EntitySpawn {
                id,
                uuid: id as u128,
                type_id,
                x: 10.0,
                y,
                z: -20.0,
                movement: [0.0; 3],
                x_rot: 0.0,
                y_rot: yaw,
                y_head_rot: 0.0,
                data: 0,
            }));
        }
        let boats = boats_view(&model);
        assert_eq!(boats.len(), 3, "the zombie AND the bare boat stay out");
        let oak = boats.iter().find(|b| b.id == 7).unwrap();
        assert_eq!(
            (oak.x, oak.y, oak.z, oak.yaw),
            (10.0, 64.0, -20.0, 90.0),
            "the pose the snapshot verbatim keeps"
        );
        assert!(!oak.raft, "the oak boat rides the railed hull");
        assert!(!oak.chest, "the plain boat carries no chest");
        assert_eq!(oak.wood, "oak", "the wood the name speaks");
        let raft = boats.iter().find(|b| b.id == 9).unwrap();
        assert!(raft.raft, "the bamboo raft rides the plank deck");
        assert_eq!(raft.wood, "bamboo", "the raft's own wood");
        let chest = boats.iter().find(|b| b.id == 10).unwrap();
        assert!(chest.chest, "the chest variant's own bit");
        assert!(!chest.raft, "the chest boat rides the railed hull");
        assert_eq!(chest.wood, "oak", "the chest boat's own wood");
    }

    #[test]
    fn the_below_name_projection_rides_the_holder() {
        use crate::world_model::string_uuid;
        use mc_protocol::{
            DisplayObjectiveUpdate, ObjectiveUpdate, PlayEvent, ScoreUpdate, OBJECTIVE_METHOD_ADD,
            RENDER_TYPE_INTEGER,
        };
        let mut model = WorldModel::new();
        let mob_uuid = (0x550e8400e29b41d4u128 << 64) | 0xa716446655440000u128;
        assert!(
            model.apply(PlayEvent::Objective(ObjectiveUpdate {
                objective: "deaths".into(),
                method: OBJECTIVE_METHOD_ADD,
                display: Some("Deaths".into()),
                render_type: RENDER_TYPE_INTEGER,
            })),
            "the objective add"
        );
        // the silent slot: no BELOW_NAME display — the honest None
        assert_eq!(below_name_view(&model, mob_uuid), None, "no slot, no line");
        // the slot speaks but the entity's row is absent — still None
        assert!(
            model.apply(PlayEvent::DisplayObjective(DisplayObjectiveUpdate {
                slot: mc_protocol::DISPLAY_SLOT_BELOW_NAME,
                objective: "deaths".into(),
            })),
            "the below-name slot set"
        );
        assert_eq!(
            below_name_view(&model, mob_uuid),
            None,
            "no row, no line"
        );
        // the row lands (the holder the stringUUID law formats): the
        // text the score + space + the objective's display speak
        assert!(model.apply(PlayEvent::Score(ScoreUpdate {
            owner: string_uuid(mob_uuid),
            objective: "deaths".into(),
            score: 42,
            display: None,
        })));
        assert_eq!(
            below_name_view(&model, mob_uuid).as_deref(),
            Some("42 Deaths"),
            "the NO_STYLE component's own words"
        );
        // a foreign holder's row never rides this entity's line
        assert!(model.apply(PlayEvent::Score(ScoreUpdate {
            owner: "someone-else".into(),
            objective: "deaths".into(),
            score: 999,
            display: None,
        })));
        assert_eq!(below_name_view(&model, mob_uuid).as_deref(), Some("42 Deaths"));
    }

    #[test]
    fn the_player_list_projection_sorts_and_caps() {
        use mc_protocol::{
            DisplayObjectiveUpdate, ObjectiveUpdate, PlayEvent, PlayerEntryPatch, PlayerInfoUpdate,
            ScoreUpdate, OBJECTIVE_METHOD_ADD, RENDER_TYPE_INTEGER,
        };
        let mut model = WorldModel::new();
        // the silent overlay: the join's own empty shape
        let empty = player_list_view(&model);
        assert_eq!(empty.rows.len(), 0);
        assert_eq!(empty.header, "");
        // the entries: order asc rides first, the spectator sinks, the
        // unlisted never render, the names tiebreak case-sensitive
        let patch = |uuid: u128, name: &str, gm: Option<u8>, listed: bool| PlayerEntryPatch {
            uuid,
            name: Some(name.into()),
            game_mode: gm,
            listed: Some(listed),
            latency: None,
            display_name: None,
            list_order: Some(0),
        };
        assert!(model.apply(PlayEvent::PlayerInfoUpdate(PlayerInfoUpdate {
            entries: vec![
                patch(1, "Zed", None, true),
                patch(2, "SpectatorBot", Some(3), true),
                patch(3, "Alpha", None, true),
                patch(4, "Hidden", None, false),
            ],
        })));
        // the list-order key: Alpha re-sorts FIRST on order 3
        assert!(model.apply(PlayEvent::PlayerInfoUpdate(PlayerInfoUpdate {
            entries: vec![PlayerEntryPatch {
                uuid: 3,
                name: None,
                game_mode: None,
                listed: None,
                latency: None,
                display_name: None,
                list_order: Some(-3),
            }],
        })));
        let list = player_list_view(&model);
        let names: Vec<&str> = list.rows.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["Alpha", "Zed", "SpectatorBot"],
            "the order asc, the non-spectators' name asc, the spectator sinks, the unlisted drops"
        );
        assert_eq!(list.rows[2].game_mode, 3, "the spectator's mode rides the row");

        // THE SCORE COLUMN (the LIST slot's own lane): the objective
        // the slot names, the row the player's NAME holds (the
        // ScoreHolder.fromGameProfile law — players score by name)
        assert!(model.apply(PlayEvent::Objective(ObjectiveUpdate {
            objective: "kills".into(),
            method: OBJECTIVE_METHOD_ADD,
            display: Some("Kills".into()),
            render_type: RENDER_TYPE_INTEGER,
        })));
        assert!(model.apply(PlayEvent::DisplayObjective(DisplayObjectiveUpdate {
            slot: 0, // the LIST slot
            objective: "kills".into(),
        })));
        assert!(model.apply(PlayEvent::Score(ScoreUpdate {
            owner: "Zed".into(),
            objective: "kills".into(),
            score: 7,
            display: None,
        })));
        let list = player_list_view(&model);
        let zed = list.rows.iter().find(|r| r.name == "Zed").unwrap();
        assert_eq!(zed.score, Some(7), "the name-holder's row rides");
        let alpha = list.rows.iter().find(|r| r.name == "Alpha").unwrap();
        assert_eq!(alpha.score, None, "no row, no score — the honest None");

        // THE CAP (the limit(80L) the stream speaks): 85 listed entries
        // truncate to 80
        let mut flood = Vec::new();
        for i in 0..85u128 {
            flood.push(patch(100 + i, &format!("Player{i:03}"), None, true));
        }
        model.apply(PlayEvent::PlayerInfoUpdate(PlayerInfoUpdate { entries: flood }));
        assert_eq!(player_list_view(&model).rows.len(), 80, "the stream's own limit");
    }
}

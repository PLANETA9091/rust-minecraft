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

use std::collections::HashMap;

use mc_protocol::{
    ChunkColumn, ChunkLight, ClockUpdate, LightData, PlayEvent, PlayerPosition, RegistryData,
    WorldInfo, WorldSnapshot,
};

/// The model's view of ONE entity — the facts the entity packets have
/// actually delivered (the spawn's registry-typed existence, the latest
/// authoritative position). Rotation rides the spawn's byte-unpacked
/// degrees until a sync overrides it (the sync's floats are exact;
/// the spawn's bytes are the vanilla `b * 360 / 256` quantization).
#[derive(Debug, Clone, PartialEq)]
pub struct EntitySnapshot {
    /// The entity type's registry id (resolved by the synced registry).
    pub type_id: i32,
    /// World position (the latest full-precision doubles).
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// Yaw in degrees (the latest sync's float, else the spawn byte
    /// unpacked the vanilla way).
    pub y_rot: f32,
    /// Pitch in degrees (the latest sync's float, else the spawn byte
    /// unpacked the vanilla way).
    pub x_rot: f32,
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
}

impl Default for WorldModel {
    fn default() -> Self {
        Self::new()
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
            clocks: Vec::new(),
            columns: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    /// One entity's current snapshot, by id.
    pub fn entity(&self, id: i32) -> Option<&EntitySnapshot> {
        self.entities.get(&id)
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
                self.pin_dimension(&spawn.dimension);
                true
            }
            PlayEvent::Position(p) => {
                self.player = Some(p);
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
                        x: e.x,
                        y: e.y,
                        z: e.z,
                        y_rot: e.y_rot,
                        x_rot: e.x_rot,
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
                }
                any
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
                true
            }
            PlayEvent::CenterChunk { x, z } => {
                if self.center != (x, z) {
                    self.center = (x, z);
                    true
                } else {
                    false
                }
            }
            // probes and skipped packets carry no world state
            PlayEvent::KeepAlive(_) | PlayEvent::Ping(_) | PlayEvent::ChunkBatch => false,
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
    use mc_protocol::{BiomeEffects, BiomeEntry, GrassColorModifier};

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
    fn day_tick_resolves_the_dimension_default_clock_by_name() {
        let mut m = WorldModel::new();
        m.current_dimension = Some("minecraft:overworld".into());
        m.registry = RegistryData {
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
            dimension_types: vec![
                DimensionTypeEntry { name: "minecraft:overworld".into(), min_y: -64, height: 384, default_clock: Some("minecraft:overworld".into()) , timelines: None, ambient_light_color: None, skybox: None, fog_start_distance: None, fog_end_distance: None, cloud_fog_end_distance: None, block_light_tint: None, sky_light_color: None, sky_light_factor: None, night_vision_color: None },
                DimensionTypeEntry { name: "minecraft:the_nether".into(), min_y: 0, height: 256, default_clock: None , timelines: None, ambient_light_color: None, skybox: None, fog_start_distance: None, fog_end_distance: None, cloud_fog_end_distance: None, block_light_tint: None, sky_light_color: None, sky_light_factor: None, night_vision_color: None },
            ],
            biomes: Vec::new(),
            biome_tags: Vec::new(),
            world_clocks: Vec::new(),
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
}

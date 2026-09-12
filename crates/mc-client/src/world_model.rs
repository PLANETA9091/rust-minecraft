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

    /// The decoded vertical layout of one dimension (by registry key),
    /// from the configuration registry knowledge. `None` when the server
    /// never sent that key (or no registry data at all).
    pub fn dimension_min_y(&self, name: &str) -> Option<i32> {
        self.registry.dimension(name).map(|d| d.min_y)
    }

    /// How many chunk columns are currently loaded.
    pub fn loaded_chunks(&self) -> usize {
        self.columns.len()
    }

    /// The stored column at a chunk coordinate, if loaded.
    pub fn column(&self, cx: i32, cz: i32) -> Option<&ChunkColumn> {
        self.columns.get(&(cx, cz))
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
    /// (the arrival-order ids the server synced). `None` when the
    /// position is outside the loaded world OR the id is one the server
    /// never named — the honest "unknown biome", never a guess.
    pub fn biome_name(&self, x: i32, y: i32, z: i32) -> Option<&str> {
        let id = self.biome(x, y, z)?;
        self.registry.biome_name(id)
    }

    /// The current dimension's day tick — the synced clock's total
    /// ticks mod the day timeline's period — or `None` before the
    /// server's first Set Time (the honest state: no opinion, the
    /// renderer stays at day). The clock used is the first entry the
    /// server sent: a vanilla overworld server syncs its dimension's
    /// default clock, and multi-clock disambiguation needs the
    /// WORLD_CLOCK registry names (future work).
    pub fn day_tick(&self) -> Option<i64> {
        self.clocks
            .first()
            .map(|c| c.total_ticks.rem_euclid(crate::timeline::DAY_PERIOD_TICKS))
    }

    /// The renderer's sky factor for the current clock state — the day
    /// timeline's `visual/sky_light_factor` at [`WorldModel::day_tick`]
    /// (jar-extracted keyframes: 1.0 day / 0.24 night). `1.0` before
    /// any Set Time: the world with no clock opinion is day, and the
    /// factor multiplies the sky brightness term exactly as the
    /// vanilla lightmap shader's `SkyFactor` does.
    pub fn sky_factor(&self) -> f32 {
        self.day_tick()
            .map(crate::timeline::sky_light_factor)
            .unwrap_or(1.0)
    }

    /// Sky light (0..=15) at (x, y, z)
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
            }],
            // the synced biome table: ids 0/1/2, arrival order
            biomes: vec![
                "minecraft:plains".to_string(),
                "minecraft:swamp".to_string(),
                "minecraft:desert".to_string(),
            ],
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
                }],
                biomes: Vec::new(),
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
        assert_eq!(m.dimension_min_y("minecraft:overworld"), None, "empty before any join");
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
                    })
                    .collect(),
                biomes: Vec::new(),
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
        assert_eq!(m.dimension_min_y("minecraft:overworld"), None);
        assert_eq!(m.dimension_min_y("minecraft:the_nether"), None);
        assert_eq!(m.dimension_min_y("minecraft:flat"), Some(0));
    }

    #[test]
    fn dimension_pins_at_login_and_respawn_repins_min_y() {
        let mut m = WorldModel::new();
        m.registry = RegistryData {
            dimension_types: vec![
                DimensionTypeEntry { name: "minecraft:overworld".into(), min_y: -64, height: 384 },
                DimensionTypeEntry { name: "minecraft:the_nether".into(), min_y: 0, height: 256 },
            ],
            biomes: Vec::new(),
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
}

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
//! - `min_y` is `None` until the world's CURRENT dimension is pinned:
//!   the configuration Registry Data is now DECODED (the dimension-type
//!   vertical layout table lives in `registry`, see `dimension_min_y`),
//!   but the play-state Login no longer names the current dimension —
//!   vanilla conveys it via the Respawn packet, whose id still needs
//!   proof against real servers. Until then `block()` takes
//!   COLUMN-LOCAL y (0 = bottom of the lowest section) — the honest
//!   convention, no guessed world offset. Once `min_y` is known,
//!   world-Y queries subtract it before the section math.
//! - This module is wired into the frame loop when the mesher input lands
//!   (the next step); until then it is exercised by its own tests only.

use std::collections::HashMap;

use mc_protocol::{
    ChunkColumn, DimensionTypeEntry, PlayEvent, PlayerPosition, RegistryData, WorldInfo,
    WorldSnapshot,
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
    /// The dimension-type vertical layout table the configuration phase
    /// decoded (registry key -> min_y/height). The `min_y` field above
    /// pins the CURRENT dimension only once the Respawn packet is parsed.
    pub registry: RegistryData,
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
            registry: RegistryData::default(),
            columns: HashMap::new(),
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
                self.info = Some(w);
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
        self.info = Some(info);
        self.player = Some(player);
        self.registry = registry;
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
}

#[cfg(test)]
mod tests {
    use super::*;
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
        };
        assert!(m.apply(PlayEvent::Chunk(col_a)));
        assert_eq!(m.loaded_chunks(), 1);
        // a re-sent column for the same key REPLACES (newest wins) —
        // the old content is gone, not merged, and the map stays size 1
        let col_b = ChunkColumn {
            x: 3,
            z: -2,
            sections: vec![parse_section_buffer(&air_section())],
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
        };
        m.apply(PlayEvent::Chunk(neg));
        assert_eq!(m.block(-1, -64, -1), Some(0), "world (-1,-64,-1) is local (15,0,15): air");
        assert_eq!(m.block(-16, -64, -16), Some(1), "world (-16,-64,-16) is local (0,0,0): stone");
        assert_eq!(m.block(-13, -63, -14), Some(10), "local (3,1,2) of chunk (-1,-1)");
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
        };
        let player = PlayerPosition {
            x: 8.5,
            y: 64.0,
            z: -3.25,
            yaw: 90.0,
            pitch: 0.0,
            flags: 0,
            teleport_id: 42,
        };
        let col = ChunkColumn {
            x: 3,
            z: -2,
            sections: vec![parse_section_buffer(&stone_dirt_section(2))],
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
            },
        });
        assert_eq!(m.loaded_chunks(), 1);
        assert_eq!(m.info.as_ref().unwrap().entity_id, 17);
        assert_eq!(m.player.as_ref().unwrap().teleport_id, 42);
        assert_eq!(m.block(48, 0, -32), Some(1));
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
            },
            player: PlayerPosition {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                yaw: 0.0,
                pitch: 0.0,
                flags: 0,
                teleport_id: 1,
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
}

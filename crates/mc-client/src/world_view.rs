//! World view — the bridge from the network world model to the renderer.
//!
//! `materialize` projects a window of the client's `WorldModel` (chunk
//! columns with resolved global state ids) into the voxel grid the mesher
//! and raster already consume (`world::World`). This is the renderer-side
//! half of the first world VIEW from network chunks: the window/camera
//! wiring into the frame loop is the NEXT step, until then the module is
//! exercised by its own tests only.
//!
//! Documented decisions (all deliberate, no silent guesses):
//! - The global-state-id -> runtime-palette mapping is a REQUIRED caller
//!   parameter. The project has no bytecode-true numeric id table yet
//!   (mc-registry carries names, not vanilla's registration-order ids),
//!   so nothing here bakes in remembered constants — clean-room
//!   discipline. The future generated table slots in as another mapper.
//! - The copy is BOTTOM-ANCHORED: grid y 0 is the column's bottom section
//!   bottom, columns shorter than the grid leave sky above. Placing the
//!   surface nicely needs the dimension's real vertical layout (min_y /
//!   height) plus a surface scan — future work; the honest default is the
//!   deterministic one.
//! - Columns missing from the model stay AIR (the vanilla client would
//!   still be streaming them — holes are the truthful render).

use crate::world::{World, AIR, CHUNK_X, CHUNK_Y, CHUNK_Z};
use crate::world_model::WorldModel;

/// Project the window of `side * side` chunks starting at chunk
/// `(origin.0, origin.1)` into a fresh renderer grid. `mapper` converts
/// every resolved global state id (air included) to the runtime palette;
/// blocks outside a column's vertical extent and columns the server has
/// not sent (yet) stay air.
#[allow(dead_code)] // wired into the frame loop when the view lands (next step)
pub fn materialize(
    model: &WorldModel,
    origin: (i32, i32),
    side: usize,
    mapper: impl Fn(u32) -> u16,
) -> World {
    let mut world = World {
        chunks_per_side: side,
        seed: 0,
        chunks: vec![vec![AIR; CHUNK_X * CHUNK_Y * CHUNK_Z]; side * side],
    };
    let side_i = side as i32;
    for gz in 0..side_i {
        for gx in 0..side_i {
            let (cx, cz) = (origin.0 + gx, origin.1 + gz);
            let Some(_col) = model.column(cx, cz) else {
                continue; // unsent column -> the grid's fresh air
            };
            let grid = &mut world.chunks[(gx + gz * side_i) as usize];
            // bottom-anchored copy: the model answers column-local y when
            // min_y is unknown and world-y when it is — querying through
            // block() keeps ONE source of truth for the index conventions
            let y_base = model.min_y.unwrap_or(0);
            for gy in 0..CHUNK_Y as i32 {
                let wy = y_base + gy;
                for lz in 0..CHUNK_Z as i32 {
                    for lx in 0..CHUNK_X as i32 {
                        let Some(id) = model.block(cx * CHUNK_X as i32 + lx, wy, cz * CHUNK_Z as i32 + lz)
                        else {
                            continue; // above/below the column's extent -> air
                        };
                        let mapped = mapper(id);
                        if mapped != AIR {
                            grid[lx as usize + lz as usize * CHUNK_X + gy as usize * CHUNK_X * CHUNK_Z] =
                                mapped;
                        }
                    }
                }
            }
        }
    }
    world
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{DIRT, STONE};
    use crate::world_model::WorldModel;
    use mc_protocol::play::clientbound;
    use mc_protocol::{ChunkColumn, PlayEvent, varint::write_varint};

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
        // package into a one-section chunk packet and parse for real
        let mut p = Vec::new();
        write_varint(&mut p, clientbound::CHUNK_DATA);
        p.extend_from_slice(&0i32.to_be_bytes());
        p.extend_from_slice(&0i32.to_be_bytes());
        p.push(0x0A);
        p.push(0x00); // empty heightmaps compound
        write_varint(&mut p, s.len() as i32);
        p.extend_from_slice(&s);
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
    fn materialize_maps_blocks_through_the_callers_mapper() {
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 3,
            z: -2,
            sections: vec![stone_dirt_section()],
        }));
        // window origin (1,-3): chunk (3,-2) sits at grid (2, 1)
        let world = materialize(&m, (1, -3), 4, example_mapper);
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
        }));
        let world = materialize(&m, (1, -3), 4, example_mapper);
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
        }));
        let world = materialize(&m, (0, 0), 2, example_mapper);
        assert_eq!(world.get(0, 0, 0), STONE);
        assert_eq!(world.get(0, 15, 0), AIR, "column top slice is air blocks");
        assert_eq!(world.get(0, 16, 0), AIR, "above the short column: sky");
        assert_eq!(world.get(0, 47, 0), AIR, "grid top: sky");
        // an empty model: the whole window is fresh air
        let empty = WorldModel::new();
        let world = materialize(&empty, (5, 5), 2, example_mapper);
        assert!(world
            .chunks
            .iter()
            .all(|c| c.iter().all(|&b| b == AIR)));
    }

    #[test]
    fn mapper_sees_raw_ids_and_bottom_anchor_survives_min_y() {
        let mut m = WorldModel::new();
        m.apply(PlayEvent::Chunk(ChunkColumn {
            x: 0,
            z: 0,
            sections: vec![stone_dirt_section()],
        }));
        // the mapper receives every resolved id verbatim (air included) —
        // nothing is pre-filtered, the caller owns the full decision
        let mut seen = Vec::new();
        let world = materialize(&m, (0, 0), 1, |id| {
            if !seen.contains(&id) {
                seen.push(id);
            }
            example_mapper(id)
        });
        seen.sort_unstable();
        assert_eq!(seen, vec![0, 1, 10], "raw global ids reach the mapper");
        assert_eq!(world.get(0, 0, 0), STONE);
        // with a known min_y the SAME bottom-anchored copy results: the
        // materializer queries world-y (min_y + gy), block() shifts back
        m.min_y = Some(-64);
        let world = materialize(&m, (0, 0), 1, example_mapper);
        assert_eq!(world.get(0, 0, 0), STONE, "bottom anchor is min_y stable");
        assert_eq!(world.get(3, 1, 2), DIRT);
        assert_eq!(world.get(0, 16, 0), AIR);
    }
}

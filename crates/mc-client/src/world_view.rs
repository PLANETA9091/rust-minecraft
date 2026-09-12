//! World view — the bridge from the network world model to the renderer.
//!
//! `materialize` projects a window of the client's `WorldModel` (chunk
//! columns with resolved global state ids) into the voxel grid the mesher
//! and raster already consume (`world::World`). `materialize_snapshot` is
//! the frame-loop entry the join worker calls: the world-load milestone's
//! snapshot adopts into a fresh model and the player-centered window
//! lands on the world screen — the first world VIEW from network chunks.
//! The stream itself stays one-shot for now (the join pump returns at the
//! milestone); a live, continuously-updated view is future work.
//!
//! Documented decisions (all deliberate, no silent guesses):
//! - The global-state-id -> runtime-palette mapping is a REQUIRED caller
//!   parameter. The bytecode-true numeric id table now EXISTS — GENERATED
//!   by `mc2rs-gen dump-block-states` from the official jar's real
//!   bootstrapped registry (mc-registry's block_ids) — and `runtime_mapper`
//!   is the GUI's policy through it; tests inject other mappers freely.
//!   Nothing bakes in remembered constants.
//! - The copy is BOTTOM-ANCHORED: grid y 0 is the column's bottom section
//!   bottom, columns shorter than the grid leave sky above. Placing the
//!   surface nicely needs the dimension's real vertical layout (min_y /
//!   height) plus a surface scan — future work; the honest default is the
//!   deterministic one.
//! - Columns missing from the model stay AIR (the vanilla client would
//!   still be streaming them — holes are the truthful render).

use crate::world::{
    World, AIR, ANDESITE, BEDROCK, CHUNK_X, CHUNK_Y, CHUNK_Z, COAL_ORE, COBBLED_DEEPSLATE,
    COPPER_ORE, DEEPSLATE, DIAMOND_ORE, DIORITE, DIRT, EMERALD_ORE, GOLD_ORE, GRANITE, GRASS,
    GRAVEL, IRON_ORE, LAPIS_ORE, LEAVES, LOG, NETHERRACK, REDSTONE_ORE, SAND, STONE, WATER,
};
use crate::world_model::WorldModel;
use mc_protocol::{PlayerPosition, WorldSnapshot};

/// Project the window of `side * side` chunks starting at chunk
/// `(origin.0, origin.1)` into a fresh renderer grid. `mapper` converts
/// every resolved global state id (air included) to the runtime palette;
/// blocks outside a column's vertical extent and columns the server has
/// not sent (yet) stay air.
pub fn materialize(
    model: &WorldModel,
    origin: (i32, i32),
    side: usize,
    mut mapper: impl FnMut(u32) -> u16,
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

/// Materialize the join milestone's world: the snapshot adopts into a
/// fresh `WorldModel` (replace-on-collision semantics included) and the
/// player-centered window projects into the renderer grid. `mapper` is
/// the caller's id policy (see the module notes) — the GUI join path uses
/// `placeholder_mapper` today; tests pin the mapping with concrete ids.
pub fn materialize_snapshot(
    snap: WorldSnapshot,
    side: usize,
    mapper: impl FnMut(u32) -> u16,
) -> World {
    let origin = snapshot_window_origin(&snap.player, side);
    let mut model = WorldModel::new();
    model.adopt_snapshot(snap);
    materialize(&model, origin, side, mapper)
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
        _ => AIR, // air and every block the palette doesn't render yet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{DIRT, NAME_OF, STONE};
    use crate::world_model::WorldModel;
    use mc_protocol::play::clientbound;
    use mc_registry::generated::atlas;
    use mc_protocol::{
        ChunkColumn, PlayEvent, RegistryData, WorldInfo, varint::write_varint,
    };

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

    /// A join milestone shaped like the e2e mock's: one real section at
    /// chunk (3,-2) (stone@(0,0,0), dirt@(3,1,2)) and the player standing
    /// inside the centered chunk (a real vanilla join burst is exactly
    /// this — the spawn position inside the view-distance center).
    fn join_snapshot() -> WorldSnapshot {
        WorldSnapshot {
            info: WorldInfo {
                entity_id: 17,
                hardcore: false,
                dimensions: vec!["minecraft:overworld".to_string()],
                view_distance: 8,
                simulation_distance: 8,
                portal_cooldown: 300,
            },
            player: PlayerPosition {
                x: 56.5,
                y: 64.0,
                z: -27.25,
                yaw: 90.0,
                pitch: 0.0,
                flags: 0,
                teleport_id: 42,
            },
            chunks: vec![ChunkColumn {
                x: 3,
                z: -2,
                sections: vec![stone_dirt_section()],
            }],
            registry: RegistryData::default(),
        }
    }

    #[test]
    fn snapshot_window_origin_centers_the_player_chunk() {
        let p = |x: f64, z: f64| PlayerPosition {
            x,
            y: 64.0,
            z,
            yaw: 0.0,
            pitch: 0.0,
            flags: 0,
            teleport_id: 0,
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
        let world = materialize_snapshot(join_snapshot(), 5, example_mapper);
        assert_eq!(world.chunks_per_side, 5);
        assert_eq!(world.get(32, 0, 32), STONE, "stone of column (3,-2)");
        assert_eq!(world.get(35, 1, 34), DIRT, "dirt at column-local (3,1,2)");
        // the REAL generated mapper resolves the mock's ids too — the
        // mock's example ids (1=stone, 10=dirt) coincide with the real
        // registry facts, and the mapper now proves it from the table
        let real_world = materialize_snapshot(join_snapshot(), 5, runtime_mapper);
        assert_eq!(real_world.get(32, 0, 32), STONE, "stone through the generated table");
        assert_eq!(real_world.get(35, 1, 34), DIRT, "dirt through the generated table");
    }
}

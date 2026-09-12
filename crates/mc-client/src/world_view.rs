//! World view — the bridge from the network world model to the renderer.
//!
//! `materialize` projects a window of the client's `WorldModel` (chunk
//! columns with resolved global state ids) into the voxel grid the mesher
//! and raster already consume (`world::World`). `materialize_snapshot_view`
//! is the frame-loop entry the join worker calls: the world-load
//! milestone's snapshot adopts into a fresh model and the player-centered
//! window lands on the world screen — the first world VIEW from network
//! chunks. The stream itself stays one-shot for now (the join pump returns
//! at the
//! milestone); a live, continuously-updated view is future work.
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
    World, AIR, ANDESITE, BEDROCK, CHUNK_X, CHUNK_Y, CHUNK_Z, COAL_ORE, COBBLED_DEEPSLATE,
    COPPER_ORE, DEEPSLATE, DIAMOND_ORE, DIORITE, DIRT, EMERALD_ORE, GOLD_ORE, GRANITE, GRASS,
    GRAVEL, IRON_ORE, LAPIS_ORE, LEAVES, LOG, NETHERRACK, REDSTONE_ORE, SAND, STONE, WATER,
};
use crate::world_model::WorldModel;
use mc_protocol::{PlayerPosition, WorldSnapshot};

/// Project the window of `side * side` chunks starting at chunk
/// `(origin.0, origin.1)` into a fresh renderer grid. `y_base` is the
/// vertical anchor: the y coordinate grid row 0 represents — WORLD-y when
/// the model's `min_y` is known (the snapshot path passes
/// [`snapshot_y_base`]), column-local y when it is not (pass 0; the only
/// meaningful copy then). `mapper` converts every resolved global state
/// id (air included) to the runtime palette; blocks outside a column's
/// vertical extent and columns the server has not sent (yet) stay air.
pub fn materialize(
    model: &WorldModel,
    origin: (i32, i32),
    side: usize,
    y_base: i32,
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
            // the caller anchors the copy; the model answers column-local
            // y when min_y is unknown and world-y when it is — querying
            // through block() keeps ONE source of truth for the index
            // conventions
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
/// two can never disagree about where the player stands.
pub struct SnapshotView {
    pub world: World,
    /// The teleport's exact pose in the grid frame, or `None` without
    /// dimension layout knowledge (see [`snapshot_camera`]).
    pub camera: Option<Camera>,
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

/// The full join view: the materialized grid PLUS the camera pose from
/// the same window decisions. The join worker calls this off the frame
/// thread and ships both parts in the join result — the world screen
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
    SnapshotView {
        world: materialize(&model, origin, side, y_base, mapper),
        camera,
    }
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
        snap.registry = RegistryData {
            dimension_types: vec![mc_protocol::DimensionTypeEntry {
                name: "minecraft:overworld".to_string(),
                min_y: -64,
                height: 384,
            }],
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
                dimensions: vec!["minecraft:overworld".to_string()],
                view_distance: 8,
                simulation_distance: 8,
                portal_cooldown: 300,
                current_dimension: "minecraft:overworld".to_string(),
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
}

//! net_fixture — the NETWORK world's deterministic golden scene.
//!
//! Every CPU golden so far renders the PROCEDURAL world (`World::new`):
//! its pixels pin the mesher/raster math, but its world is born in Rust,
//! never on the wire. The light layer (0.3.37) only rides the network
//! path, so no golden pinned ITS pixels — until this fixture. The scene
//! is built exactly the way a real join is: full Chunk Data packets
//! (the shared `test_support` writer — one truth for the wire shape),
//! decoded by the real `parse_chunk_data`, applied to a `WorldModel`,
//! then materialized through [`materialize`] with the generated
//! [`runtime_mapper`]. What the mesher consumes is byte-for-byte the
//! network pipeline's output.
//!
//! The terrain is a pure function of grid coordinates (no RNG, no
//! floats): a stepped hill (heights 25..=30) with bedrock, ore
//! sprinkles and a grass skin — plus ONE structure chosen to make the
//! light layer VISIBLE from the fixed orbit camera: a floating stone
//! shelf whose footprint casts a hard sky shadow (the server's arrays
//! say sky 0 under it), with a torch underneath painting a warm block
//! light 14..0 gradient over the shadowed ground. Open terrain stays
//! sky 15 — brightness(15) is exactly 1.0, so it renders with the
//! legacy look, proving the ramp's top end while the shelf patch pins
//! the dark end and the gradient.

use crate::world::World;
use crate::world_model::WorldModel;
use crate::world_view::{materialize, runtime_mapper};
use mc_protocol::varint::write_varint;
use mc_protocol::{pack_entries, play::parse_chunk_data, play::test_support::chunk_packet_with_light, PlayEvent};
use mc_registry::generated::block_ids::BLOCK_RANGES;

/// The grid the CPU goldens frame: 5x5 chunks = 80x80 blocks (the
/// default `--chunks`), so the orbit camera/frustum/fog match the
/// procedural goldens' geometry exactly.
#[cfg_attr(not(test), expect(dead_code))]
pub const FIXTURE_SIDE: usize = 5;

/// The torch cell (grid x, world y, grid z): under the shelf, mid-gap
/// above the ground skin — the emitter whose gradient the arrays carry.
const TORCH: (i32, i32, i32) = (48, 15, 48);

/// The shelf: footprint (grid x/z ranges) and the solid slab rows. The
/// sky arrays mark every cell inside the footprint at or below
/// [`SHELF_TOP`] as sky 0 — the server's shadow. The slab floats high
/// enough (five rows of headroom) that the fixed orbit camera looks
/// UNDER it: the shadowed ground and the torch glow face the lens
/// through the open near corner.
const SHELF_X: (i32, i32) = (44, 52);
const SHELF_Z: (i32, i32) = (44, 52);
const SHELF_BOTTOM: i32 = 20;
const SHELF_TOP: i32 = 22;

/// One block's first global state id, resolved FROM the generated table
/// (never hardcoded): if the jar's registry drifts, the fixture panics
/// here instead of silently rendering the wrong block.
fn state_id(name: &str) -> u32 {
    for &(start, _end, n) in BLOCK_RANGES {
        if n == name {
            return start;
        }
    }
    panic!("net fixture: {name} is not in the generated block table");
}

/// Write one 4-bit light level into a 2048-byte engine array at the
/// section-local YZX cell (javap: `DataLayer.getIndex = (y << 8) |
/// (z << 4) | x`, even index in the LOW half-byte).
fn set_nibble(arr: &mut [u8; 2048], x: usize, y: usize, z: usize, level: u8) {
    let i = (y << 8) | (z << 4) | x;
    let (byte, nib) = (i >> 1, i & 1);
    if nib == 0 {
        arr[byte] = (arr[byte] & 0xF0) | (level & 0x0F);
    } else {
        arr[byte] = (arr[byte] & 0x0F) | (level << 4);
    }
}

/// The stepped hill: a Chebyshev falloff from the grid center, integer
/// only — 14 at the center, 9 at the rim, one step per 8 blocks. The
/// whole scene sits BELOW the orbit camera's eye (y 18.5, fixed by the
/// CPU goldens' framing): a taller world would swallow the camera in
/// solid stone.
fn terrain_height(gx: i32, gz: i32) -> i32 {
    14 - (gx - 40).abs().max((gz - 40).abs()) / 8
}

fn on_shelf(gx: i32, gz: i32) -> bool {
    gx >= SHELF_X.0 && gx <= SHELF_X.1 && gz >= SHELF_Z.0 && gz <= SHELF_Z.1
}

/// The sky rule, cell-precise: full daylight above the terrain skin
/// EXCEPT under the shelf (the solid slab blocks the vertical path —
/// the server's BFS verdict is 0 there), and nothing below ground.
fn sky_level(gx: i32, wy: i32, gz: i32) -> u8 {
    let open = wy >= terrain_height(gx, gz) && !(on_shelf(gx, gz) && wy <= SHELF_TOP);
    if open {
        15
    } else {
        0
    }
}

/// The torch gradient: vanilla block light falls by 1 per manhattan
/// step, clamped at 0 — here as a pure function of the cell.
fn block_level(gx: i32, wy: i32, gz: i32) -> u8 {
    let d = (gx - TORCH.0).abs() + (wy - TORCH.1).abs() + (gz - TORCH.2).abs();
    (14 - d).clamp(0, 15) as u8
}

/// One section's wire buffer: indirect palette (5 bits — every fixture
/// palette fits), the packed 4096-entry grid (YZX, same convention the
/// parser proves), single-valued biomes. `block_count` is the real
/// non-air count — the parser retains it, a lie would be drift.
fn build_section(palette: &[u32], entries: &[u32; 4096]) -> Vec<u8> {
    let block_count = entries.iter().filter(|&&e| e != 0).count() as i16;
    let mut s = Vec::new();
    s.extend_from_slice(&block_count.to_be_bytes());
    s.push(5); // bits per entry — indirect
    write_varint(&mut s, palette.len() as i32);
    for &id in palette {
        write_varint(&mut s, id as i32);
    }
    s.extend_from_slice(&pack_entries(entries, 5));
    s.push(0); // biomes: single valued
    // 200 = an id the vanilla wire order (66 biomes) never names — the
    // fixture never branches on biome, and a mock that sent NO registry
    // must not carry a bare wire id the skip-sync fallback would
    // (correctly, for a claim-accepting vanilla server) resolve through
    // BIOME_ORDER; the same fixture honesty the "mock:test" dimension
    // name rides (a non-vanilla layout never claims a vanilla key)
    write_varint(&mut s, 200);
    s
}

/// One column's FULL Chunk Data packet: three sections (terrain skin +
/// shelf, open sky, empty sky) + the light tail with the engine layers.
/// Sky: bit 1 mixed (open air above the rim skin, 0 underground and
/// under the shelf), bit 2 mixed (the shelf's shadow rows), bit 3 (the
/// empty sky section) full 15, bit 4 (above the column) full 15, bit 0
/// known-zero. Block: bits 1..=2 carry the torch gradient (it straddles
/// the section border), the rest known-zero. Count == popcount holds by
/// construction (the shared writer enforces it).
fn fixture_packet(cx: i32, cz: i32) -> Vec<u8> {
    let air = 0u32;
    let stone = state_id("minecraft:stone");
    let bedrock = state_id("minecraft:bedrock");
    let coal = state_id("minecraft:coal_ore");
    let iron = state_id("minecraft:iron_ore");
    let andesite = state_id("minecraft:andesite");
    let grass = state_id("minecraft:grass_block");
    let dirt = state_id("minecraft:dirt");

    let idx = |x: usize, y: usize, z: usize| (y << 8) | (z << 4) | x;
    // entries carry PALETTE INDICES (the wire's indirect palettes), the
    // palette carries the global state ids
    let pidx = |p: &[u32], id: u32| p.iter().position(|&x| x == id).expect("id in its palette") as u32;

    // section 0 (rows 0..=15): bedrock floor AND the terrain skin — the
    // hill tops out at 14, so grass/dirt/stone-with-ores all live here;
    // the pillars start here too
    let p0 = [air, stone, bedrock, coal, iron, andesite, grass, dirt];
    let mut e0 = [0u32; 4096];
    for y in 0..16usize {
        for z in 0..16usize {
            for x in 0..16usize {
                let (wy, wx, wz) = (y as i32, cx * 16 + x as i32, cz * 16 + z as i32);
                let h = terrain_height(wx, wz);
                let id = if (wx == SHELF_X.0 && wz == SHELF_Z.1 || wx == SHELF_X.1 && wz == SHELF_Z.0)
                    && (9..=SHELF_BOTTOM - 1).contains(&wy)
                {
                    stone // the two support pillars, ground to slab
                } else if y == 0 {
                    bedrock
                } else if wy > h {
                    air
                } else if wy == h {
                    grass
                } else if wy >= h - 3 {
                    dirt
                } else if (wx * 7 + wy * 13 + wz * 29) % 53 == 0 {
                    coal
                } else if (wx * 11 + wy * 5 + wz * 17) % 71 == 5 {
                    iron
                } else if (wx * 3 + wy * 19 + wz * 7) % 89 == 11 {
                    andesite
                } else {
                    stone
                };
                e0[idx(x, y, z)] = pidx(&p0, id);
            }
        }
    }

    // section 1 (rows 16..=31): the shelf slab (rows 20..=22) and the
    // pillars' upper runs, open sky around them
    let p1 = [air, stone];
    let mut e1 = [0u32; 4096];
    for y in 0..16usize {
        for z in 0..16usize {
            for x in 0..16usize {
                let (wy, wx, wz) = (16 + y as i32, cx * 16 + x as i32, cz * 16 + z as i32);
                let pillar = wx == SHELF_X.0 && wz == SHELF_Z.1
                    || wx == SHELF_X.1 && wz == SHELF_Z.0;
                if on_shelf(wx, wz) && (SHELF_BOTTOM..=SHELF_TOP).contains(&wy)
                    || pillar && (16..=SHELF_BOTTOM - 1).contains(&wy)
                {
                    e1[idx(x, y, z)] = pidx(&p1, stone);
                }
            }
        }
    }

    // section 2: open sky — every cell air (block_count 0 parses fine)
    let s0 = build_section(&p0, &e0);
    let s1 = build_section(&p1, &e1);
    let s2 = build_section(&[air], &[0u32; 4096]);

    // light arrays for the column's own sections (bits 1..=3) — the
    // torch's gradient and the open-air rows straddle sections 0 and 1,
    // so BOTH carry real data; the engine layers: bit 0 below =
    // known-zero, bit 4 above = full 15
    let mut sky0 = [0u8; 2048];
    let mut block0 = [0u8; 2048];
    let mut sky1 = [0u8; 2048];
    let mut sky2 = [0u8; 2048];
    let mut block1 = [0u8; 2048];
    for y in 0..16usize {
        for z in 0..16usize {
            for x in 0..16usize {
                let (wx, wz) = (cx * 16 + x as i32, cz * 16 + z as i32);
                set_nibble(&mut sky0, x, y, z, sky_level(wx, y as i32, wz));
                set_nibble(&mut block0, x, y, z, block_level(wx, y as i32, wz));
                let wy1 = 16 + y as i32;
                set_nibble(&mut sky1, x, y, z, sky_level(wx, wy1, wz));
                set_nibble(&mut block1, x, y, z, block_level(wx, wy1, wz));
                set_nibble(&mut sky2, x, y, z, sky_level(wx, 32 + y as i32, wz));
            }
        }
    }

    chunk_packet_with_light(
        cx,
        cz,
        &[s0, s1, s2],
        &[
            (1, sky0.to_vec()),
            (2, sky1.to_vec()),
            (3, sky2.to_vec()),
            (4, vec![0xFF; 2048]),
        ],
        &[0],
        &[(1, block0.to_vec()), (2, block1.to_vec())],
        &[0, 3, 4],
    )
}

/// The fixture's [`WorldModel`]: every column of the `side * side`
/// window built as a real packet and decoded by the REAL parser — the
/// model state is byte-for-byte what the play pump would hold.
pub fn fixture_model(side: usize) -> WorldModel {
    let mut m = WorldModel::new();
    for cz in 0..side as i32 {
        for cx in 0..side as i32 {
            let col = parse_chunk_data(&fixture_packet(cx, cz))
                .expect("the fixture's own packets must parse");
            m.apply(PlayEvent::Chunk(col));
        }
    }
    m
}

/// The fixture's renderer world: the model materialized over the full
/// window (origin (0,0), column-local y — the model never learned a
/// dimension layout, so the grid rows ARE the column rows 0..48) with
/// the generated runtime mapper. The light layer comes along: this is
/// the network view the mesher's smooth lighting consumes.
pub fn fixture_world(side: usize) -> World {
    let model = fixture_model(side);
    materialize(&model, (0, 0), side, 0, runtime_mapper)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{BEDROCK, GRASS, LIGHT_UNKNOWN, STONE};

    #[test]
    fn fixture_world_materializes_the_parsed_wire_state() {
        let w = fixture_world(FIXTURE_SIDE);
        // the light layer rode the whole chain: packets -> parse -> model
        // -> materialize
        let light = w.light.as_ref().expect("the fixture carries its light");
        // grass skin at the hill's center (h(40,40) = 14)
        assert_eq!(w.get(40, 14, 40), GRASS);
        // bedrock floor straight from the packet's first section
        assert_eq!(w.get(40, 0, 40), BEDROCK);
        // the shelf slab is stone, floating above the skin
        assert_eq!(w.get(48, 21, 48), STONE);
        // open terrain: sky 15 (the ramp's top end — legacy-identical)
        let (sky, block) = light.cell(FIXTURE_SIDE, 40, 40, 40);
        assert_eq!(sky, Some(15));
        assert_eq!(block, Some(0));
        // under the shelf the server's shadow is HARD: sky 0 …
        let (sky, block) = light.cell(FIXTURE_SIDE, 48, 13, 48);
        assert_eq!(sky, Some(0));
        // … and the torch (48,15,48) paints the gradient: manhattan 2
        // below the emitter -> light 12
        assert_eq!(block, Some(12));
        // an air gap cell in the shadow, off the emitter: manhattan 4
        // -> light 10
        let (sky, block) = light.cell(FIXTURE_SIDE, 46, 15, 50);
        assert_eq!(sky, Some(0));
        assert_eq!(block, Some(10));
        // above the shelf the sky opens again
        let (sky, _) = light.cell(FIXTURE_SIDE, 48, 26, 48);
        assert_eq!(sky, Some(15));
    }

    #[test]
    fn fixture_is_deterministic_and_fully_known() {
        let a = fixture_world(FIXTURE_SIDE);
        let b = fixture_world(FIXTURE_SIDE);
        assert_eq!(a.chunks, b.chunks, "same packets, same blocks");
        assert_eq!(a.light, b.light, "same packets, same light");
        // every sent column's cells were answered: no unknowns anywhere
        // in the window (the fixture documents the full-coverage case;
        // the unknown-cell honesty is the mesher tests' business)
        let light = a.light.as_ref().expect("light");
        for chunk in &light.sky {
            assert!(
                chunk.iter().all(|&v| v != LIGHT_UNKNOWN),
                "the fixture's sky arrays cover the whole window"
            );
        }
    }
}

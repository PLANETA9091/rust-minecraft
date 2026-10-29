#!/usr/bin/env python3
"""the autopush-334 round's mesher edits: the enchanting table's own
pedestal lane (the beacon 319 shape the same walk wears — the derive
bake + the named-tile lane + the dispatch branch + the REAL mesh
test)."""
import sys

P = "crates/mc-client/src/mesher.rs"
s = open(P).read()


def rep(old, new, want=1, tag=""):
    global s
    n = s.count(old)
    if n == 0 and new in s:
        print(f"  [{tag}] already inserted")
        return
    if n != want:
        sys.exit(f"  [{tag}] anchor count {n} != {want}")
    s = s.replace(old, new)
    print(f"  [{tag}] inserted")


# ---- 1. the fn pair (before the bed models doc) ----
rep(
    "/// THE BED MODELS' OWN CACHED BAKES (the dev-194 round",
    '''/// THE ENCHANTING TABLE'S OWN CACHED BAKE (the autopush-334 round —
/// the jar's block/enchanting_table model the derive walk resolves
/// ONCE, the ONE element the six quads speak: the 12-px pedestal
/// [0,0,0]->[16,12,16] — the down the #bottom full-uv face, the up
/// the #top face, the four sides the #side rows the uv [0,4,16,16]
/// window the lower 12 rows serve — the book the BE renderer's own
/// lane the vanilla's own render split speaks, the extractBook's
/// animated glyphs the block lane never draws). The failed derive the
/// honest empty slice keeps (the 314 hopper law).
fn enchant_table_model_quads() -> &'static [DerivedQuad] {
    static QUADS: std::sync::OnceLock<Vec<DerivedQuad>> = std::sync::OnceLock::new();
    QUADS.get_or_init(|| derive_model_inherited("block/enchanting_table").unwrap_or_default())
}

/// THE ENCHANTING TABLE'S OWN NAMED TILE (the pool's own rows the
/// name index answers — the mint's 1067/1068/1069 the stems keep,
/// the unknown stem the None the TILE_NONE fallback keeps [the
/// beacon lane's own cache pattern the walk shares]).
fn enchant_table_named_tile(tex: &str) -> u16 {
    static TILES: std::sync::OnceLock<std::collections::HashMap<&'static str, u16>> =
        std::sync::OnceLock::new();
    let m = TILES.get_or_init(|| {
        atlas::TILE_NAMES
            .iter()
            .enumerate()
            .map(|(i, name)| (*name, i as u16))
            .collect()
    });
    let stem = tex.strip_prefix("block/").unwrap_or(tex);
    m.get(stem).copied().unwrap_or(atlas::TILE_NONE)
}

/// THE BED MODELS' OWN CACHED BAKES (the dev-194 round''',
    1,
    "fn pair",
)

# ---- 2. the dispatch branch (after the beacon branch, before the
# end-gateway doc) ----
rep(
    """                    continue;
                }
                // THE END GATEWAY'S OWN INVISIBLE LANE (the autopush-310""",
    '''                    continue;
                }
                // THE ENCHANTING TABLE'S OWN PEDESTAL LANE (the autopush-334
                // round — the jar's block/enchanting_table model the
                // derive walk resolves: the generic cube the ONE-element
                // 12-px pedestal retires, the sides the uv [0,4,16,16]
                // window the lower 12 rows serve, the book the BE
                // renderer's own lane the vanilla's own render split
                // speaks — the opaque law the whole bake wears, the
                // cullface'd down the beacon lane's own no-filter walk
                // keeps [the depth test the floor seat resolves])
                if crate::world::is_enchanting_table(id) {
                    let sky_factor = world.sky_light_factor.unwrap_or(1.0)
                        * world
                            .day_tick
                            .map(crate::timeline::sky_light_factor)
                            .unwrap_or(1.0);
                    let sky_tint = match world.sky_light_color {
                        None => world
                            .day_tick
                            .map(crate::timeline::sky_light_color)
                            .unwrap_or([1.0; 3]),
                        Some(stated) => {
                            let track = world
                                .day_tick
                                .map(crate::timeline::sky_light_color_bytes)
                                .unwrap_or([255; 3]);
                            [
                                ((stated[0] as u32 * track[0] as u32) / 255) as f32 / 255.0,
                                ((stated[1] as u32 * track[1] as u32) / 255) as f32 / 255.0,
                                ((stated[2] as u32 * track[2] as u32) / 255) as f32 / 255.0,
                            ]
                        }
                    };
                    let ambient = world.ambient_light.map_or([0.0f32; 3], |a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let block_tint = world.block_light_tint.map(|a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let night_vision = world.night_vision_color.map(|a| {
                        [
                            a[0] as f32 / 255.0,
                            a[1] as f32 / 255.0,
                            a[2] as f32 / 255.0,
                        ]
                    });
                    let (cell_sky, cell_block) = match light {
                        Some(l) => {
                            let (s, b) = cell_layers(l, world.chunks_per_side, wx, wy, wz);
                            (s.map(|v| v as f32), b.map(|v| v as f32))
                        }
                        None => (None, None),
                    };
                    for q in enchant_table_model_quads() {
                        // THE REF'S OWN TILE (the inherited derive the
                        // three refs resolves in place — "block/
                        // enchanting_table_bottom" the pool tail's 1067
                        // row answers, "_top" the 1068 keeps, "_side"
                        // the 1069 serves — the mint the stems banked)
                        let tile = enchant_table_named_tile(q.texture);
                        if tile == atlas::TILE_NONE {
                            continue;
                        }
                        let face_shade = if q.shade {
                            match q.direction {
                                mc_registry::element_quads::Direction::Up => 1.0f32,
                                mc_registry::element_quads::Direction::Down => 0.5,
                                mc_registry::element_quads::Direction::North
                                | mc_registry::element_quads::Direction::South => 0.8,
                                mc_registry::element_quads::Direction::West
                                | mc_registry::element_quads::Direction::East => 0.6,
                            }
                        } else {
                            1.0
                        };
                        let lm = lightmap_combine(
                            cell_sky,
                            cell_block,
                            sky_factor,
                            sky_tint,
                            ambient,
                            block_tint,
                            night_vision,
                            world.night_vision_factor,
                            world.darkness_scale,
                        );
                        let mut pts = [[0f32; 3]; 4];
                        let mut colors = [[0f32; 3]; 4];
                        let mut uvs = [[0f32; 2]; 4];
                        for i in 0..4 {
                            let v = q.vertices[i];
                            pts[i] = [wx as f32 + v[0], wy as f32 + v[1], wz as f32 + v[2]];
                            uvs[i] = [q.uvs[i][0] / 16.0, q.uvs[i][1] / 16.0];
                            colors[i] =
                                [lm[0] * face_shade, lm[1] * face_shade, lm[2] * face_shade];
                        }
                        // THE OPAQUE LAW (the three masters the alpha
                        // {255} map the decode pins — the cutout lane
                        // the glass shell alone wears)
                        out.push(Quad {
                            pts,
                            colors,
                            uvs,
                            tile,
                            translucent: false,
                            per_texel_alpha: false,
                        });
                    }
                    continue;
                }
                // THE END GATEWAY'S OWN INVISIBLE LANE (the autopush-310''',
    1,
    "dispatch branch",
)

open(P, "w").write(s)
print("mesher edits done")

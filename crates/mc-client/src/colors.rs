//! Face shading + block palette colors (defaults + real-texture averages).

use crate::world::*;
use std::sync::OnceLock;

/// Face order: 0:+X 1:-X 2:+Y 3:-Y 4:+Z 5:-Z (classic MC light multipliers).
pub const FACE_SHADE: [f32; 6] = [0.6, 0.6, 1.0, 0.5, 0.8, 0.8];

pub const FACE_NAMES: [&str; 6] = ["east", "west", "top", "bottom", "south", "north"];

/// The mesher's face order -> the registry Cull order. The mesher walks
/// [east, west, top, bottom, south, north] (+X, -X, +Y, -Y, +Z, -Z); the
/// palette reports model-space slots in the Cull enum order
/// [Down, Up, North, South, West, East].
const FACE_TO_CULL: [usize; 6] = [5, 4, 1, 0, 3, 2];

/// The blockstate-driven palette CONSUMER (the data half landed in the
/// registry as `palette::block_face_colors`): the flat world colors now
/// come from the REAL blockstate -> model -> texture chain instead of the
/// hand-written `default_color` guesses, with the runtime-texture LUT
/// demoted to the fallback (the LUT still serves what the chain cannot —
/// water, air, and any block outside the blockstate table).
///
/// The chain walk (parse + select + derive + resolve) is far too heavy
/// for the mesh hot path, so the table is computed ONCE over the world's
/// NAME_OF kinds and then indexed.
///
/// The states are the PROCEDURAL world's canonical ones (the world
/// generator carries no live block state yet — the same states the
/// palette census pins): grass_block snowy=false, the logs axis=y,
/// leaf_litter facing=north + segment_amount=1. A live-state world
/// swaps them at this table, nothing else changes.
fn palette_table() -> &'static Vec<Option<[[f32; 3]; 6]>> {
    static TABLE: OnceLock<Vec<Option<[[f32; 3]; 6]>>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let states: &[(&str, &[(&str, &str)])] = &[
            ("grass_block", &[("snowy", "false")]),
            ("oak_log", &[("axis", "y")]),
            ("deepslate", &[("axis", "y")]),
            ("leaf_litter", &[("facing", "north"), ("segment_amount", "1")]),
        ];
        NAME_OF
            .iter()
            .map(|name| {
                let state = states
                    .iter()
                    .find(|(n, _)| n == name)
                    .map(|(_, s)| *s)
                    .unwrap_or(&[]);
                mc_registry::palette::block_face_colors(name, state).map(|faces| {
                    // Cull-ordered palette entries -> the mesher's face order
                    let mut slots: [Option<[u8; 3]>; 6] = [None; 6];
                    for fc in faces {
                        slots[fc.face as usize] = Some(fc.color);
                    }
                    let mut out: [[f32; 3]; 6] = [[0.0, 0.0, 0.0]; 6];
                    for (m, &c) in FACE_TO_CULL.iter().enumerate() {
                        if let Some(rgb) = slots[c] {
                            out[m] = [
                                f32::from(rgb[0]) / 255.0,
                                f32::from(rgb[1]) / 255.0,
                                f32::from(rgb[2]) / 255.0,
                            ];
                        }
                    }
                    out
                })
            })
            .collect()
    })
}

/// The flat world color of one block face from the blockstate-driven
/// palette. `None` = the chain cannot color this kind (air, water) —
/// the caller falls back to the LUT / the defaults.
pub fn palette_color(kind: u16, face: usize) -> Option<[f32; 3]> {
    let table = palette_table();
    let row = table.get(kind as usize)?.as_ref()?;
    let rgb = row.get(face)?; // the mesher's 0..6 face order
    if *rgb == [0.0, 0.0, 0.0] {
        None // a face with no contributing quads (never painted)
    } else {
        Some(*rgb)
    }
}

/// The leaf-litter VARIANT->MODEL palette mapping (the item banked since
/// v0.3.78): the mesher's leaf-litter flat fallback must follow the
/// variant the way the textured path does. The blockstate is a multipart
/// whose `segment_amount` selects the template model(s) (1->{m1},
/// 2->{m2}, 3->{m2,m3} -- the OR-list `2|3` plus `3`, the multipart
/// COLLECT rule -- and 4->{m4}), and each template samples a DIFFERENT
/// uv rect of the same 16x16 `block/leaf_litter` texture, so the average
/// color differs per segment.
///
/// `facing` is deliberately pinned to north everywhere: the blockstate's
/// y-rotation turns the element AND its glued uv together, and the quad
/// rect is the rotation-invariant min/max -- the average CANNOT change
/// with the facing. Only segment_amount reaches the color.
///
/// The oracle numbers (scripts/palette_oracle.py over the real jar, the
/// opaque-pixel rule: leaf_litter.png is a grayscale cutout, 117/256
/// opaque): segment 1 = the NW quarter rect (165,165,165) [27/64
/// opaque], segment 2 = the west strip (171,171,171) [55/128], segment 3
/// = the equal-weight compose of m2's strip (171) + m3's SE quarter
/// (170) -> half-up (171,171,171), segment 4 = the full cover (171,171,
/// 171) [the full-texture average]. Three of the four segments coincide
/// on THIS texture -- the mapping is still required: segment 1 differs,
/// and a future texture is free to split them.
///
/// The templates carry exactly up+down faces sampling the SAME rect
/// (`#texture`, tintindex 0 -- the palette averages RAW texels, the dry
/// fold tint multiplies at the consumer on the textured path), so up and
/// down share one color and the four side slots stay empty.
fn leaf_litter_table() -> &'static [Option<[[f32; 3]; 6]>; 4] {
    static TABLE: OnceLock<[Option<[[f32; 3]; 6]>; 4]> = OnceLock::new();
    TABLE.get_or_init(|| {
        (1u8..=4)
            .map(|segment| {
                let state = [
                    ("facing", "north"),
                    ("segment_amount", match segment {
                        1 => "1",
                        2 => "2",
                        3 => "3",
                        _ => "4",
                    }),
                ];
                mc_registry::palette::block_face_colors("leaf_litter", &state).map(|faces| {
                    let mut slots: [Option<[u8; 3]>; 6] = [None; 6];
                    for fc in faces {
                        slots[fc.face as usize] = Some(fc.color);
                    }
                    let mut out: [[f32; 3]; 6] = [[0.0, 0.0, 0.0]; 6];
                    for (m, &c) in FACE_TO_CULL.iter().enumerate() {
                        if let Some(rgb) = slots[c] {
                            out[m] = [
                                f32::from(rgb[0]) / 255.0,
                                f32::from(rgb[1]) / 255.0,
                                f32::from(rgb[2]) / 255.0,
                            ];
                        }
                    }
                    out
                })
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("segments 1..=4")
    })
}

/// The flat world color of one leaf-litter face for a given SEGMENT (the
/// mesher's variant encoding: `segment = (variant >> 2) + 1`, and the
/// no-op variant 0 renders as segment 4 -- the full cover). `None` =
/// the chain cannot color this segment/face (a side face, an
/// out-of-domain segment) -- the caller falls back to the LUT / the
/// defaults, exactly the cube path's precedence.
pub fn palette_leaf_litter_color(segment: u8, face: usize) -> Option<[f32; 3]> {
    let row = leaf_litter_table()
        .get(usize::from(segment).checked_sub(1)?)?
        .as_ref()?;
    let rgb = row.get(face)?; // the mesher's 0..6 face order
    if *rgb == [0.0, 0.0, 0.0] {
        None // the templates have no side faces
    } else {
        Some(*rgb)
    }
}

/// Default palette (used when game textures are not available locally).
pub fn default_color(kind: u16, face: usize) -> [f32; 3] {
    let top = [0.40, 0.66, 0.26];
    let grass_side = [0.52, 0.42, 0.30];
    let dirt = [0.55, 0.42, 0.30];
    let stone = [0.58, 0.58, 0.58];
    let log_side = [0.43, 0.34, 0.20];
    let log_top = [0.62, 0.51, 0.31];
    let leaves = [0.24, 0.50, 0.21];
    let water = [0.28, 0.48, 0.74];
    let sand = [0.84, 0.79, 0.61];
    let granite = [0.58, 0.44, 0.38];
    let diorite = [0.85, 0.85, 0.86];
    let andesite = [0.62, 0.63, 0.63];
    let deepslate = [0.32, 0.32, 0.35];
    let cobbled_deepslate = [0.35, 0.35, 0.37];
    let gravel = [0.48, 0.45, 0.44];
    let bedrock = [0.35, 0.35, 0.35];
    let coal_ore = [0.52, 0.52, 0.52];
    let copper_ore = [0.60, 0.50, 0.45];
    let iron_ore = [0.65, 0.55, 0.48];
    let gold_ore = [0.63, 0.57, 0.42];
    let redstone_ore = [0.55, 0.40, 0.38];
    let lapis_ore = [0.45, 0.50, 0.62];
    let diamond_ore = [0.58, 0.66, 0.66];
    let emerald_ore = [0.52, 0.62, 0.48];
    let netherrack = [0.42, 0.20, 0.20];
    // The lava's own orange (the lava_still first frame's average — the
    // flat fallback when the atlas is absent; the real pixels ride the
    // textured path's tile)
    let lava = [0.77, 0.36, 0.08];
    match kind {
        GRASS => match face {
            2 => top,
            3 => dirt,
            _ => grass_side,
        },
        DIRT => dirt,
        STONE => stone,
        LOG => match face {
            2 | 3 => log_top,
            _ => log_side,
        },
        LEAVES => leaves,
        WATER => water,
        SAND => sand,
        GRANITE => granite,
        DIORITE => diorite,
        ANDESITE => andesite,
        DEEPSLATE => deepslate,
        COBBLED_DEEPSLATE => cobbled_deepslate,
        GRAVEL => gravel,
        BEDROCK => bedrock,
        COAL_ORE => coal_ore,
        COPPER_ORE => copper_ore,
        IRON_ORE => iron_ore,
        GOLD_ORE => gold_ore,
        REDSTONE_ORE => redstone_ore,
        LAPIS_ORE => lapis_ore,
        DIAMOND_ORE => diamond_ore,
        EMERALD_ORE => emerald_ore,
        NETHERRACK => netherrack,
        LAVA => lava,
        _ => [1.0, 0.0, 1.0], // magenta = missing block (debug-friendly)
    }
}

/// Official texture file stem per block kind + face (without directory and
/// extension). The LUT joins it into `minecraft/textures/block/<stem>.png`.
pub fn texture_stem(kind: u16, face: usize) -> Option<&'static str> {
    match kind {
        GRASS => match face {
            2 => Some("grass_block_top"),
            3 => Some("dirt"),
            _ => Some("grass_block_side"),
        },
        DIRT => Some("dirt"),
        STONE => Some("stone"),
        LOG => match face {
            2 | 3 => Some("oak_log_top"),
            _ => Some("oak_log"),
        },
        LEAVES => Some("oak_leaves"),
        WATER => Some("water_still"),
        SAND => Some("sand"),
        GRANITE => Some("granite"),
        DIORITE => Some("diorite"),
        ANDESITE => Some("andesite"),
        DEEPSLATE => match face {
            2 | 3 => Some("deepslate_top"),
            _ => Some("deepslate"),
        },
        COBBLED_DEEPSLATE => Some("cobbled_deepslate"),
        GRAVEL => Some("gravel"),
        BEDROCK => Some("bedrock"),
        COAL_ORE => Some("coal_ore"),
        COPPER_ORE => Some("copper_ore"),
        IRON_ORE => Some("iron_ore"),
        GOLD_ORE => Some("gold_ore"),
        REDSTONE_ORE => Some("redstone_ore"),
        LAPIS_ORE => Some("lapis_ore"),
        DIAMOND_ORE => Some("diamond_ore"),
        EMERALD_ORE => Some("emerald_ore"),
        NETHERRACK => Some("netherrack"),
        // the SECOND fluid member: the still strip the atlas crops to
        // its first frame (the water_still arm's own shape, tint-free)
        LAVA => Some("lava_still"),
        _ => None,
    }
}

#[cfg(test)]
mod palette_consumer_tests {
    use super::*;

    /// The flat world colors are the blockstate-driven chain's numbers —
    /// the oracle pins from the palette round, scaled to the 0..1 domain:
    /// stone (126,126,126) on every face, grass top (147,147,147) (the
    /// untinted grass_block_top), grass sides the composed two-quad
    /// (141,131,111), grass bottom dirt (134,96,67).
    #[test]
    fn the_palette_colors_are_the_chain_ones() {
        if !mc_registry::textures_table::real() {
            return; // the no-cache boot has no texels to average
        }
        for face in 0..6usize {
            assert_eq!(
                palette_color(STONE, face),
                Some([126.0 / 255.0; 3]),
                "stone face {face}"
            );
        }
        assert_eq!(palette_color(GRASS, 2), Some([147.0 / 255.0; 3]), "grass top");
        assert_eq!(palette_color(GRASS, 3), Some([134.0 / 255.0, 96.0 / 255.0, 67.0 / 255.0]), "grass bottom = dirt");
        for face in [0usize, 1, 4, 5] {
            assert_eq!(
                palette_color(GRASS, face),
                Some([141.0 / 255.0, 131.0 / 255.0, 111.0 / 255.0]),
                "grass side = the #side + #overlay equal-weight compose, face {face}"
            );
        }
    }

    /// The palette PRECEDES the LUT and the defaults: where it has a
    /// color it wins (the grass side is the compose, not the old
    /// hand-written guess), and where it cannot (water, air) it stays
    /// None so the old chain still serves.
    #[test]
    fn the_palette_precedes_the_lut_and_the_defaults() {
        if !mc_registry::textures_table::real() {
            return; // the precedence needs a real palette to precede
        }
        assert_ne!(
            palette_color(GRASS, 0),
            Some(default_color(GRASS, 0)),
            "the hand-written grass side is gone from the flat path"
        );
        assert_eq!(palette_color(WATER, 2), None, "water is a particle template");
        assert_eq!(palette_color(AIR, 2), None, "air has no blockstate");
        // the face-order mapping: east(+X) -> Cull::East carries the SIDE
        // color, top(+Y) -> Cull::Up the top color
        assert_eq!(palette_color(GRASS, 0), palette_color(GRASS, 5), "the four sides share one color");
        assert_ne!(palette_color(GRASS, 0), palette_color(GRASS, 2), "sides differ from top");
    }

    /// The leaf-litter variant->model mapping is the CHAIN's numbers,
    /// oracle-pinned (scripts/palette_oracle.py over the real jar):
    /// segment 1 (the NW-quarter template) reads (165,165,165), segments
    /// 2/3/4 read (171,171,171) on this texture (2 = the west strip,
    /// 3 = the half-up compose of the strip 171 + the SE quarter 170,
    /// 4 = the full cover). Up and down share one color (the templates'
    /// two faces sample the SAME rect), the four side slots are None
    /// (the templates have no side faces), and out-of-domain segments
    /// fail closed.
    #[test]
    fn the_leaf_litter_palette_follows_the_variant() {
        if !mc_registry::textures_table::real() {
            return; // the no-cache boot has no texels to average
        }
        for (segment, gray) in [(1u8, 165.0f32), (2, 171.0), (3, 171.0), (4, 171.0)] {
            let expect = [gray / 255.0; 3];
            for face in [2usize, 3] {
                assert_eq!(
                    palette_leaf_litter_color(segment, face),
                    Some(expect),
                    "segment {segment} face {face}"
                );
            }
            // down == up: both template faces sample the same rect
            assert_eq!(
                palette_leaf_litter_color(segment, 3),
                palette_leaf_litter_color(segment, 2),
                "segment {segment} up/down share the rect"
            );
            for face in [0usize, 1, 4, 5] {
                assert_eq!(
                    palette_leaf_litter_color(segment, face),
                    None,
                    "segment {segment} has no side faces, face {face}"
                );
            }
        }
        // the out-of-domain edges fail closed to the caller's fallback
        assert_eq!(palette_leaf_litter_color(0, 2), None, "segment 0 is not a state");
        assert_eq!(palette_leaf_litter_color(5, 2), None, "the blockstate caps at 4");
    }

    /// The variant table genuinely DIVERGES from the cube table's
    /// canonical entry: the cube table pins leaf_litter at
    /// segment_amount=1, so the mesher's leaf-litter fallback could not
    /// have reused `palette_color` -- segment 4 (the full cover, what
    /// the no-op variant 0 renders) is a different color. And segment 1
    /// MUST match the canonical entry bit-for-bit (the same chain call).
    #[test]
    fn the_leaf_litter_variant_table_diverges_where_the_segment_differs() {
        if !mc_registry::textures_table::real() {
            return; // the divergence needs real colors on both sides
        }
        assert_eq!(
            palette_leaf_litter_color(1, 2),
            palette_color(LEAF_LITTER, 2),
            "segment 1 IS the canonical state"
        );
        assert_ne!(
            palette_leaf_litter_color(4, 2),
            palette_color(LEAF_LITTER, 2),
            "segment 4 (the full cover) differs from the segment-1 canonical entry"
        );
    }
}

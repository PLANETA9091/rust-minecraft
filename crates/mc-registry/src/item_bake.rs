//! The 26.2 ItemModelGenerator sprite-extrusion bake — the held-item
//! geometry link: the generated item models (item/generated's chain) are
//! not element models; their quads are BAKED from the layer sprites'
//! opaque-pixel outline. Pure data over the generated textures table,
//! zero dependencies, zero render-path contact.
//!
//! Every rule is javap evidence from the 26.2 client jar
//! (net.minecraft.client.resources.model.cuboid.ItemModelGenerator +
//! ItemLayerKey/SideDirection/SideFace, SpriteContents, BakedQuad$MaterialInfo):
//!
//! - Domain: a model whose parent chain contains `item/generated`
//!   (whose own parent is the `builtin/generated` GENERATED_ITEM_MODEL_ID
//!   the generator serves). 1231 of 3928 models.
//! - LAYERS = ["layer0".."layer4"]; the layers resolve through the
//!   texture_slots chain merge (TextureSlots.getMaterial). The private
//!   bake() loops the layers and BREAKS at the first null material
//!   (`if (material == null) goto end`) — layers must be contiguous from
//!   layer0; 9 domain models have no layer0 at all and bake EMPTY
//!   (item/spear_in_hand, item/bundle/open_front/back — element or
//!   display-only carriers).
//! - MaterialInfo.of(material, transparency, layerIndex, true, 0):
//!   tintIndex = the LAYER INDEX (0..4), shade = TRUE, lightEmission = 0.
//! - SpriteContents.isTransparent(frame, x, y) = ARGB.alpha(pixel) == 0 —
//!   any NONZERO alpha is opaque. A texture missing from the namespace
//!   bakes from the missingno checkerboard (generateMissingImage 16x16,
//!   fully opaque — sprite_uv::missing_pixel).
//! - The front/back: ONE unculled quad each over the whole sprite,
//!   from (0,0,7.5) to (16,16,8.5) (MIN_Z/MAX_Z — the extrusion sits
//!   7.5/16..8.5/16), SOUTH with UVs(0,0,16,16), NORTH with the mirrored
//!   UVs(16,0,0,16), Quadrant.R0.
//! - The sides: for every opaque pixel and each SideDirection (UP, DOWN,
//!   LEFT, RIGHT — mapped to Direction.UP, DOWN, EAST, WEST — the enum's
//!   LEFT carries EAST and RIGHT carries WEST), checkTransition records
//!   (dir, x, y) when the NEIGHBOR (x - stepX, y - stepY) is transparent
//!   (out of bounds counts transparent). Each recorded edge emits ONE
//!   unculled quad — 26.2 does NOT merge pixel runs (SideFace is a
//!   Record(facing, x, y)):
//!   ```
//!   f14 = x + 0.1; f15 = x + 1 - 0.1              // UV_SHRINK = 0.1f
//!   LEFT/RIGHT: f16 = y + 0.1;     f17 = y + 1 - 0.1
//!   UP/DOWN:    f16 = y + 1 - 0.1; f17 = y + 0.1  // the V is REVERSED
//!   corners: f18 = x; f19 = y; f20 = x; f21 = y; then
//!     UP:    f20 += 1
//!     DOWN:  f20 += 1; f19 += 1; f21 += 1
//!     LEFT:  f21 += 1
//!     RIGHT: f18 += 1; f20 += 1; f21 += 1
//!   f18 *= 16/w; f20 *= 16/w; f19 *= 16/h; f21 *= 16/h  (whole sprites
//!     like the 32x32 spears scale here)
//!   f19 = 16 - f19; f21 = 16 - f21                // the y flip
//!   UP:    from (f18, f19, 7.5)  to (f20, f19, 8.5)
//!   DOWN:  from (f18, f21, 7.5)  to (f20, f21, 8.5)
//!   LEFT:  from (f18, f19, 7.5)  to (f18, f21, 8.5)
//!   RIGHT: from (f20, f19, 7.5)  to (f20, f21, 8.5)
//!   uv = UVs(f14*16/w, f16*16/h, f15*16/w, f17*16/h)
//!   ```
//!   All arithmetic is f32; the +0.1 insets are the f32 literals (7.0+0.1f
//!   and 13.0+0.1f round to DIFFERENT f32 values — the oracle pins both).
//! - The animated-sprite note: vanilla scans every unique frame
//!   (SpriteContents.getUniqueFrames) and unions the exposed edges; the
//!   generated textures table stores the FIRST-FRAME crop only (v0.3.70),
//!   so this bake uses frame 0's alpha for every frame. The census
//!   measures the approximation: exactly ONE domain layer texture is
//!   animated (block/sculk_vein, referenced by item/sculk_vein).
//! - Ordering: vanilla accumulates the side edges in a HashSet (JVM
//!   order is arbitrary); this port emits the deterministic scan order —
//!   y outer, x inner, then UP, DOWN, LEFT, RIGHT (the lambda's
//!   checkTransition call order), layers in index order, front before
//!   back before sides.
//!
//! The bit-exact oracle: scripts/item_bake_census.py runs the same bake
//! in python (f32-strict via struct) over the jar's real PNG pixels
//! (PIL decode); the census pins the domain numbers and the fixtures
//! (iron_sword 82 quads, copper_torch 26, bow 98, spear_in_hand empty).

use crate::element_quads::{
    calculate_facing, faceinfo_corners, recalculate_winding, vertex_uvs, Direction, Quad,
};
use crate::generated::models::MODELS;
use crate::generated::textures::{texture_index, texture_rgba, texture_size};
use crate::texture_slots;

/// The extrusion depth (ItemModelGenerator's MIN_Z/MAX_Z constants).
const MIN_Z: f32 = 7.5;
const MAX_Z: f32 = 8.5;
/// The UV inset (ItemModelGenerator.UV_SHRINK).
const UV_SHRINK: f32 = 0.1;
/// LAYERS = List.of("layer0".."layer4").
pub const LAYERS: [&str; 5] = ["layer0", "layer1", "layer2", "layer3", "layer4"];

/// The faceinfo slot per SideDirection — the enum's Direction carries:
/// UP -> UP, DOWN -> DOWN, LEFT -> EAST, RIGHT -> WEST (the 26.2 enum
/// ctor arguments, verbatim).
fn side_slot(side: u8) -> Direction {
    match side {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::East,
        _ => Direction::West,
    }
}

fn lookup(model_id: &str) -> Option<&'static str> {
    MODELS
        .iter()
        .find(|(id, ..)| *id == model_id)
        .map(|(id, ..)| *id)
}

/// The parent-chain walk (bare ids); the domain test.
fn chain_has_generated(model_id: &'static str) -> bool {
    let mut cur = model_id;
    for _ in 0..32 {
        let Some((_, parent, ..)) = MODELS.iter().find(|(id, ..)| *id == cur) else {
            return false;
        };
        let parent = match parent {
            Some(p) if !p.starts_with("builtin/") => *p,
            // builtin/generated IS the generated root the generator serves
            Some("builtin/generated") => return true,
            _ => return false,
        };
        if parent == "item/generated" {
            return true;
        }
        cur = parent;
    }
    false
}

/// The one-pixel alpha probe: the SpriteContents.isTransparent rule —
/// alpha exactly 0 is transparent (any nonzero alpha is opaque).
fn pixel_alpha(texels: &[u8], w: u16, x: u32, y: u32) -> u8 {
    let i = ((y * w as u32) + x) as usize * 4 + 3;
    texels.get(i).copied().unwrap_or(0)
}

/// The exposed-edge scan (lambda$getSideFaces$0): every opaque pixel
/// checks its four neighbors; a transparent (or out-of-bounds) neighbor
/// records (dir, x, y) in the scan order.
fn exposed_edges(texels: &[u8], w: u16, h: u16) -> Vec<(u8, u32, u32)> {
    let (w32, h32) = (w as u32, h as u32);
    // the Direction STEP vectors (Mojang's getStepX/getStepY):
    // UP = (0,+1), DOWN = (0,-1), EAST = (+1,0), WEST = (-1,0) — the
    // checkTransition neighbor is (x - stepX, y - stepY), so UP probes
    // the pixel ABOVE (y-1), DOWN the one BELOW (y+1), LEFT (carrying
    // EAST) probes x-1, and RIGHT (carrying WEST) probes x+1.
    let steps: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut out = Vec::new();
    for y in 0..h32 {
        for x in 0..w32 {
            if pixel_alpha(texels, w, x, y) == 0 {
                continue; // only OPAQUE pixels expose edges
            }
            for (d, (sx, sy)) in steps.iter().enumerate() {
                let nx = x as i32 - sx;
                let ny = y as i32 - sy;
                let neighbor_transparent = nx < 0
                    || ny < 0
                    || nx >= w32 as i32
                    || ny >= h32 as i32
                    || pixel_alpha(texels, w, nx as u32, ny as u32) == 0;
                if neighbor_transparent {
                    out.push((d as u8, x, y));
                }
            }
        }
    }
    out
}

/// One layer sprite -> the quads. `texels` is the RGBA8 pixels (the
/// first-frame crop; the missingno checkerboard for the missing ids).
fn bake_layer(texels: &[u8], w: u16, h: u16, texture: &'static str, layer: usize) -> Vec<Quad> {
    let mut quads = Vec::new();
    let mut emit = |slot: Direction, from: [f32; 3], to: [f32; 3], uv: [f32; 4]| {
        let mut vertices = faceinfo_corners(slot, from, to);
        let mut uvs = vertex_uvs(uv, 0);
        let facing = calculate_facing(&vertices);
        if let Some(f) = facing {
            recalculate_winding(&mut vertices, &mut uvs, f);
        }
        quads.push(Quad {
            vertices,
            uvs,
            slot,
            direction: facing.unwrap_or(Direction::Up),
            texture,
            cullface: None,
            tintindex: Some(layer as u8),
            uv_rotation: 0,
            shade: true,
            light_emission: None,
        });
    };
    // the front/back over the whole sprite
    emit(
        Direction::South,
        [0.0, 0.0, MIN_Z],
        [16.0, 16.0, MAX_Z],
        [0.0, 0.0, 16.0, 16.0],
    );
    emit(
        Direction::North,
        [0.0, 0.0, MIN_Z],
        [16.0, 16.0, MAX_Z],
        [16.0, 0.0, 0.0, 16.0],
    );
    // the per-edge sides (the f5/f6 scale = 16/sprite dims)
    let fx = 16.0f32 / w as f32;
    let fy = 16.0f32 / h as f32;
    for (side, x, y) in exposed_edges(texels, w, h) {
        let xf = x as f32;
        let yf = y as f32;
        let f14 = xf + UV_SHRINK;
        let f15 = (xf + 1.0) - UV_SHRINK;
        let (f16, f17) = match side {
            2 | 3 => (yf + UV_SHRINK, (yf + 1.0) - UV_SHRINK), // LEFT/RIGHT
            _ => ((yf + 1.0) - UV_SHRINK, yf + UV_SHRINK),     // UP/DOWN: V reversed
        };
        let (mut f18, mut f19, mut f20, mut f21) = (xf, yf, xf, yf);
        match side {
            0 => f20 += 1.0,
            1 => {
                f20 += 1.0;
                f19 += 1.0;
                f21 += 1.0;
            }
            2 => f21 += 1.0,
            _ => {
                f18 += 1.0;
                f20 += 1.0;
                f21 += 1.0;
            }
        }
        f18 *= fx;
        f20 *= fx;
        f19 *= fy;
        f21 *= fy;
        f19 = 16.0 - f19;
        f21 = 16.0 - f21;
        let uv = [f14 * fx, f16 * fy, f15 * fx, f17 * fy];
        let (from, to) = match side {
            0 => ([f18, f19, MIN_Z], [f20, f19, MAX_Z]),
            1 => ([f18, f21, MIN_Z], [f20, f21, MAX_Z]),
            2 => ([f18, f19, MIN_Z], [f18, f21, MAX_Z]),
            _ => ([f20, f19, MIN_Z], [f20, f21, MAX_Z]),
        };
        emit(side_slot(side), from, to, uv);
    }
    quads
}

/// The quads of one generated item model, by id. None when the id is not
/// in the bake domain (the parent chain never reaches item/generated);
/// Some(empty) for the domain models with no layer0 (the element or
/// display-only carriers — item/spear_in_hand, the bundle sides).
pub fn derive_item_model(model_id: &str) -> Option<Vec<Quad>> {
    let root = lookup(model_id)?;
    if !chain_has_generated(root) {
        return None;
    }
    let slots = texture_slots::resolve_model(model_id)?;
    let mut quads = Vec::new();
    for (layer, name) in LAYERS.iter().enumerate() {
        // the layer material through the resolved slots; the FIRST null
        // BREAKS the loop (the bake() bytecode — no gaps allowed)
        let Some(id) = slots.material(name) else {
            break;
        };
        let bare = id.strip_prefix("minecraft:").unwrap_or(id);
        let (texels, w, h, texture) = match texture_index(bare).filter(|i| !crate::generated::textures::is_missing(*i)) {
            Some(idx) => (texture_rgba(idx).to_vec(), texture_size(idx).0, texture_size(idx).1, {
                // the Quad.texture is the bare id (a 'static str from the
                // table; strip the namespace like sprite_uv's join does)
                bare
            }),
            // the missing material: the missingno checkerboard 16x16
            None => {
                let mut t = Vec::with_capacity(16 * 16 * 4);
                for y in 0u32..16 {
                    for x in 0u32..16 {
                        t.extend_from_slice(&crate::sprite_uv::missing_pixel(x, y, 16, 16));
                    }
                }
                (t, 16, 16, "missingno")
            }
        };
        quads.extend(bake_layer(&texels, w, h, texture, layer));
    }
    Some(quads)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_the_iron_sword_extrusion() {
        // the oracle (scripts/item_bake_census.py, strict-f32): 82 quads.
        let quads = derive_item_model("item/iron_sword")
            .expect("in the bake domain");
        assert_eq!(quads.len(), 82, "2 whole-sprite faces + 80 exposed edges");
        // quad[0]: the SOUTH face over the whole sprite
        let q = &quads[0];
        assert_eq!(q.slot, Direction::South);
        assert_eq!(q.direction, Direction::South);
        assert_eq!(q.texture, "item/iron_sword");
        assert_eq!(q.tintindex, Some(0));
        assert!(q.shade);
        assert_eq!(q.cullface, None);
        assert_eq!(q.uv_rotation, 0);
        assert_eq!(q.light_emission, None);
        assert_eq!(
            q.uvs.map(|uv| uv.map(f32::to_bits)),
            [
                [0x00000000, 0x00000000],
                [0x41800000, 0x00000000],
                [0x41800000, 0x41800000],
                [0x00000000, 0x41800000],
            ]
        );
        // quad[1]: the NORTH face carries the MIRRORED UVs (16,0,0,16)
        let q = &quads[1];
        assert_eq!(q.slot, Direction::North);
        assert_eq!(
            q.uvs.map(|uv| uv.map(f32::to_bits)),
            [
                [0x41800000, 0x00000000],
                [0x00000000, 0x00000000],
                [0x00000000, 0x41800000],
                [0x41800000, 0x41800000],
            ]
        );
        // quad[2]: the first exposed edge — a DOWN side at pixel (13, 0),
        // the V REVERSED (minV 0.9 > maxV 0.1), the 0.1f insets
        // (13.0f + 0.1f rounds to 0x4151999a — the f32 tie the oracle
        // pins; 7.0f + 0.1f rounds differently: 0x40e33333)
        let q = &quads[2];
        assert_eq!(q.slot, Direction::Down);
        assert_eq!(q.direction, Direction::Down);
        assert_eq!(
            q.uvs.map(|uv| uv.map(f32::to_bits)),
            [
                [0x4151999a, 0x3f666666],
                [0x4151999a, 0x3dcccccd],
                [0x415e6666, 0x3dcccccd],
                [0x415e6666, 0x3f666666],
            ]
        );
        // quad[3]: a RIGHT edge — the 26.2 enum carries WEST (LEFT->EAST,
        // RIGHT->WEST, the ctor arguments verbatim)
        let q = &quads[3];
        assert_eq!(q.slot, Direction::West);
        assert_eq!(
            q.uvs.map(|uv| uv.map(f32::to_bits)),
            [
                [0x4151999a, 0x3dcccccd],
                [0x4151999a, 0x3f666666],
                [0x415e6666, 0x3f666666],
                [0x415e6666, 0x3dcccccd],
            ]
        );
        // every quad is axis-aligned: the computed facing equals the slot
        let drift = quads.iter().filter(|q| q.direction != q.slot).count();
        assert_eq!(drift, 0, "no facing drift on the axis-aligned slabs");
        // the extrusion depth: z in [7.5/16, 8.5/16] everywhere
        for q in &quads {
            for v in &q.vertices {
                assert!((0.0..=1.0).contains(&v[0]));
                assert!((0.0..=1.0).contains(&v[1]));
                assert!((MIN_Z / 16.0..=MAX_Z / 16.0).contains(&v[2]));
            }
        }
    }

    #[test]
    fn derives_the_copper_torch_and_bow() {
        // copper_torch: the MINIMUM fixture (26 quads); its 0.1f insets
        // round to DIFFERENT bits than iron_sword's (7.0f + 0.1f)
        let quads = derive_item_model("item/copper_torch")
            .expect("in the bake domain");
        assert_eq!(quads.len(), 26);
        assert_eq!(quads[0].texture, "block/copper_torch");
        assert_eq!(
            quads[2].uvs.map(|uv| uv.map(f32::to_bits)),
            [
                [0x40e33333, 0x40dccccd],
                [0x40e33333, 0x40c33333],
                [0x40fccccd, 0x40c33333],
                [0x40fccccd, 0x40dccccd],
            ]
        );
        // bow: 98 quads
        let quads = derive_item_model("item/bow")
            .expect("in the bake domain");
        assert_eq!(quads.len(), 98);
    }

    #[test]
    fn empty_bakes_for_the_layerless_carriers() {
        // the bake() bytecode BREAKS at the first null material; models
        // with no layer keys at all bake EMPTY (still Some — the domain)
        let empty = derive_item_model("item/spear_in_hand")
            .expect("in the bake domain");
        assert!(empty.is_empty());
        let empty = derive_item_model("item/bundle/open_back")
            .expect("in the bake domain");
        assert!(empty.is_empty());
        // a block model whose chain never reaches item/generated -> None
        assert!(derive_item_model("block/stone").is_none());
        assert!(derive_item_model("block/stairs").is_none());
        assert!(derive_item_model("no/such/model").is_none());
    }

    #[test]
    fn the_missing_layer_bakes_the_missingno_checkerboard() {
        // item/chainmail_boots_amethyst_trim: layer0 = item/chainmail_boots
        // (66 quads), layer1 = trims/items/boots_trim_amethyst — MISSING
        // from the namespace -> the missingno checkerboard (16x16, fully
        // opaque -> 64 border edges + 2 = 66), texture "missingno",
        // tintindex = the layer index 1
        let quads = derive_item_model("item/chainmail_boots_amethyst_trim")
            .expect("in the bake domain");
        assert_eq!(quads.len(), 132, "66 + 66");
        assert_eq!(quads[0].texture, "item/chainmail_boots");
        assert_eq!(quads[65].texture, "item/chainmail_boots");
        assert_eq!(quads[66].texture, "missingno");
        assert_eq!(quads[66].tintindex, Some(1));
        // the checkerboard is fully opaque -> the first edge is DOWN at
        // pixel (0,0): from (0,15,7.5) to (1,15,8.5), uv (0.1, 0.9, 0.9, 0.1)
        assert_eq!(quads[66].slot, Direction::Down);
        assert_eq!(
            quads[66].uvs.map(|uv| uv.map(f32::to_bits)),
            [
                [0x3dcccccd, 0x3f666666],
                [0x3dcccccd, 0x3dcccccd],
                [0x3f666666, 0x3dcccccd],
                [0x3f666666, 0x3f666666],
            ]
        );
        let vertices = quads[66].vertices;
        assert_eq!(
            vertices.map(|v| v.map(f32::to_bits)),
            [
                [0x3f000000, 0x3f800000, 0x3ef00000],
                [0x3e800000, 0x3f800000, 0x3ef00000],
                [0x3e800000, 0x3f800000, 0x3f088889],
                [0x3f000000, 0x3f800000, 0x3f088889],
            ],
            "the Down corner walk over from (0,15,7.5) to (1,15,8.5), block units"
        );
    }

    #[test]
    fn the_registry_wide_bake_census_holds() {
        // scripts/item_bake_census.py over the real jar
        let mut domain = 0usize;
        let mut layer_models = [0usize; 5];
        let mut missing_layer_textures = std::collections::BTreeSet::new();
        let mut quad_total = 0usize;
        for (id, ..) in MODELS.iter() {
            // the domain test: the chain reaches item/generated
            if !chain_has_generated(id) {
                continue;
            }
            domain += 1;
            let Some(quads) = derive_item_model(id) else {
                panic!("{id}: the domain never fails");
            };
            quad_total += quads.len();
            let slots = texture_slots::resolve_model(id).expect("a table row");
            for (i, name) in LAYERS.iter().enumerate() {
                let Some(t) = slots.material(name) else {
                    break; // the contiguous-layers rule
                };
                layer_models[i] += 1;
                let bare = t.strip_prefix("minecraft:").unwrap_or(t);
                let missing = texture_index(bare)
                    .map(|idx| crate::generated::textures::is_missing(idx))
                    .unwrap_or(true);
                if missing {
                    missing_layer_textures.insert(bare);
                }
            }
        }
        assert_eq!(domain, 1231, "the bake domain (chains through item/generated)");
        assert_eq!(layer_models, [1222, 332, 44, 0, 0], "contiguous layer presence");
        assert_eq!(
            missing_layer_textures.len(),
            64,
            "the trims family, all missing"
        );
        assert!(
            missing_layer_textures
                .iter()
                .all(|t| t.starts_with("trims/items/")),
            "64 missing = the trims/items family"
        );
        assert_eq!(
            quad_total, 80464,
            "the total baked quads over the domain (all layers)"
        );
    }
}

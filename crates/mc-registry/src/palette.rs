//! The blockstate-driven palette (the DATA half): a block state -> the
//! per-face average color of its REAL model's REAL textures, assembled
//! entirely from the generated tables — the data-driven replacement for
//! the hand-written default-color guessing. Zero dependencies, zero
//! render-path contact (the mesher consumes this in the NEXT round).
//!
//! The chain, every link already census-validated:
//!   BLOCKSTATES (v0.3.62) -> blockstate_lookup's vanilla selection
//!   semantics (variants first-match / multipart collect, v0.3.69's
//!   when-census) -> the applies' model refs -> element_quads::derive_model
//!   (the FaceBakery quads, v0.3.68) -> texture_slots::resolve_model
//!   (the TextureSlots fixpoint, v0.3.69) -> generated::textures pixels
//!   (the jar PNGs decoded at generation time, v0.3.70).
//!
//! Semantics pinned here:
//! - The color of one quad is the average of the OPAQUE pixels (alpha
//!   != 0 — vanilla SpriteContents.isTransparent is `alpha == 0`, so
//!   any nonzero alpha counts) of its UV RECT: the min/max of the four
//!   vertex uvs (the 0..16 texture space), scaled to the texture's real
//!   pixel size. The uv rotation only permutes the four corners, so the
//!   min/max rect is rotation-invariant.
//! - One face's color is the EQUAL-WEIGHT average of its quads' rect
//!   colors (a face split across quads, e.g. the fence posts, averages
//!   its pieces; a weighted-by-area refinement stays with the consumer,
//!   which owns the geometry).
//! - The faces are keyed by quad.slot — the model's OWN face slots. The
//!   blockstate apply rotations (x/y, always multiples of 90 in 26.2)
//!   are NOT folded in here: they remap model slots to world faces at
//!   the consumer, and the palette reports the model-space truth.
//! - Tinted quads (tintindex) average their RAW texels — the biome tint
//!   (grass/foliage/water) multiplies at the consumer, exactly like the
//!   quads carry it.
//! - Quads whose texture ref fails to resolve (the vanilla-warned
//!   dangling slots of template models) or whose texture is MISSING
//!   (the missingno family, width 0) contribute nothing; a face with no
//!   contributing quads is absent from the result.
//! - Unknown blocks / unparseable definitions return None — fail-closed,
//!   the consumer keeps its previous color.

use crate::blockstate_lookup;
use crate::element_quads;
use crate::generated::elements::Cull;
use crate::generated::textures::{texture_index, texture_rgba, texture_size};
use crate::texture_slots;

/// One model-space face's average color (the slot, the RGB average).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaceColor {
    pub face: Cull,
    pub color: [u8; 3],
}

/// The per-face average colors of a block state's selected model(s).
/// `state` is the property pairs in BLOCKS order (empty for the
/// propertyless blocks). `None` = unknown block, unparseable column, no
/// selected model, or nothing resolvable — the consumer keeps whatever
/// it had.
pub fn block_face_colors(block_name: &str, state: &[(&str, &str)]) -> Option<Vec<FaceColor>> {
    let parsed = blockstate_lookup::parse(block_name)?;
    let parsed = parsed.ok()?;
    // The by-value selection: the applies come OUT of the parsed
    // definition ('static data — no borrows survive the parse).
    let applies = blockstate_lookup::select_applies(parsed, state)?;
    if applies.is_empty() {
        return None;
    }
    // (face, sum, count) accumulation over every contributing quad.
    let mut acc: Vec<(Cull, [u64; 3], u32)> = Vec::new();
    for apply in &applies {
        let model = bare(apply.model);
        // resolve_model and derive_model walk the same MODELS table; a
        // model that is not a model file (or has no slots) contributes
        // nothing rather than failing the whole block.
        let (Some(quads), Some(slots)) =
            (element_quads::derive_model(model), texture_slots::resolve_model(model))
        else {
            continue;
        };
        for q in &quads {
            let Some(color) = quad_average(&slots, q.texture, &q.uvs) else {
                continue;
            };
            match acc.iter_mut().find(|(f, _, _)| *f == q.slot) {
                Some((_, sum, n)) => {
                    for i in 0..3 {
                        sum[i] += color[i] as u64;
                    }
                    *n += 1;
                }
                None => acc.push((q.slot, [color[0] as u64, color[1] as u64, color[2] as u64], 1)),
            }
        }
    }
    if acc.is_empty() {
        return None;
    }
    // Stable Cull order (the enum order — Down..East) for a
    // deterministic result regardless of apply order.
    let mut out: Vec<FaceColor> = acc
        .into_iter()
        .map(|(face, sum, n)| FaceColor {
            face,
            color: [
                (sum[0] / n as u64) as u8,
                (sum[1] / n as u64) as u8,
                (sum[2] / n as u64) as u8,
            ],
        })
        .collect();
    let rank = |f: Cull| f as u32;
    out.sort_by_key(|fc| rank(fc.face));
    Some(out)
}

/// The average opaque-pixel color of one quad's UV rect over the REAL
/// texture the slot resolution names. `None` = unresolvable ref,
/// missing material, or a fully transparent rect.
fn quad_average(
    slots: &texture_slots::ResolvedSlots,
    texture_ref: &str,
    uvs: &[[f32; 2]; 4],
) -> Option<[u8; 3]> {
    let material = slots.material(texture_ref)?;
    let idx = texture_index(bare(material))?;
    let rgba = texture_rgba(idx);
    let (w, h) = texture_size(idx);
    if w == 0 || h == 0 {
        return None; // the missingno family carries an empty texel span
    }
    // The rotation-invariant rect (the corners only permute).
    let (mut u0, mut v0, mut u1, mut v1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for [u, v] in uvs {
        u0 = u0.min(*u);
        v0 = v0.min(*v);
        u1 = u1.max(*u);
        v1 = v1.max(*v);
    }
    // 0..16 texture space -> the texture's own pixel grid, clamped.
    let (wf, hf) = (f32::from(w), f32::from(h));
    let x0 = ((u0 / 16.0 * wf).round() as i64).clamp(0, i64::from(w));
    let x1 = ((u1 / 16.0 * wf).round() as i64).clamp(0, i64::from(w));
    let y0 = ((v0 / 16.0 * hf).round() as i64).clamp(0, i64::from(h));
    let y1 = ((v1 / 16.0 * hf).round() as i64).clamp(0, i64::from(h));
    let (mut sr, mut sg, mut sb, mut n) = (0u64, 0u64, 0u64, 0u64);
    for y in y0..y1 {
        for x in x0..x1 {
            let o = ((y as u32 * w as u32 + x as u32) * 4) as usize;
            // vanilla SpriteContents.isTransparent: alpha == 0 is the
            // only transparent — any nonzero alpha is opaque
            if rgba[o + 3] == 0 {
                continue;
            }
            sr += u64::from(rgba[o]);
            sg += u64::from(rgba[o + 1]);
            sb += u64::from(rgba[o + 2]);
            n += 1;
        }
    }
    if n == 0 {
        return None;
    }
    Some([(sr / n) as u8, (sg / n) as u8, (sb / n) as u8])
}

/// The one-`minecraft:`-prefix strip (sprite_uv's namespace rule — the
/// slots' values ride as written, the tables index the bare paths).
fn bare(id: &str) -> &str {
    id.strip_prefix("minecraft:").unwrap_or(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The oracle: a hand-written python PNG reader over the real jar
    /// (scripts/palette_oracle.py) pins the full-texture averages —
    /// stone (126,126,126), dirt (134,96,67), grass_block_side
    /// (127,107,66). A 16x16 texture's whole-image average IS one
    /// quad-average with the default 0..16 uv, so the palette must
    /// reproduce these numbers bit-exactly from the generated texels.
    #[test]
    fn stone_is_six_faces_of_the_oracle_gray() {
        let colors = block_face_colors("stone", &[]).expect("stone has a blockstate");
        assert_eq!(colors.len(), 6, "cube_all contributes every face slot");
        let faces = [Cull::Down, Cull::Up, Cull::North, Cull::South, Cull::West, Cull::East];
        for (fc, face) in colors.iter().zip(faces) {
            assert_eq!(fc.face, face);
            assert_eq!(fc.color, [126, 126, 126], "the oracle gray, face {face:?}");
        }
    }

    /// grass_block's variants model splits the faces: up is the UNTINTED
    /// grass_block_top (a grayscale texture — the biome tint rides the
    /// quads, not the texels), down is dirt, the sides are
    /// grass_block_side. Each face must equal its own oracle average.
    #[test]
    fn grass_block_splits_top_bottom_and_sides() {
        let colors = block_face_colors("grass_block", &[]).expect("grass_block parses");
        let get = |f: Cull| colors.iter().find(|fc| fc.face == f).map(|fc| fc.color);
        assert_eq!(get(Cull::Up), Some([147, 147, 147]), "the untinted top gray");
        assert_eq!(get(Cull::Down), Some([134, 96, 67]), "dirt");
        let side = [127, 107, 66];
        for f in [Cull::North, Cull::South, Cull::West, Cull::East] {
            assert_eq!(get(f), Some(side), "grass_block_side, face {f:?}");
        }
        assert_ne!(get(Cull::Up), get(Cull::Down), "top and bottom differ");
    }

    /// The state selector rides: snowy_grass_block (snowy=true) swaps
    /// ALL six faces to snow. And the fail-closed edges: unknown blocks
    /// and multipart fences behave.
    #[test]
    fn the_state_selector_and_the_edges_hold() {
        // snowy=true selects the snow variant
        let colors = block_face_colors("grass_block", &[("snowy", "true")]).expect("snowy parses");
        assert!(colors.iter().all(|fc| fc.color == [250, 250, 250]), "all snow");
        // unknown block
        assert!(block_face_colors("definitely_not_a_block", &[]).is_none());
        // multipart: the oak_fence definition has no full-cube faces but
        // its parts still carry resolvable colors
        let fence = block_face_colors("oak_fence", &[]).expect("fence parses");
        assert!(!fence.is_empty(), "the fence parts contribute colors");
        // air has no blockstate file at all
        assert!(block_face_colors("air", &[]).is_none());
    }

    /// The palette covers the procedural world's renderable blocks
    /// (the consumer's contract for the next round): every NAME_OF id
    /// except air resolves to at least one face color. Water and the
    /// leaves ride untinted here (the tint is the consumer's).
    #[test]
    fn the_palette_covers_the_procedural_world_blocks() {
        let world_blocks = [
            "stone", "dirt", "grass_block", "oak_log", "oak_leaves", "water", "sand",
            "granite", "diorite", "andesite", "deepslate", "cobbled_deepslate", "gravel",
            "bedrock", "coal_ore", "copper_ore", "iron_ore", "gold_ore", "redstone_ore",
            "lapis_ore", "diamond_ore", "emerald_ore", "netherrack", "pale_oak_leaves",
            "leaf_litter",
        ];
        for name in world_blocks {
            let colors = block_face_colors(name, &[])
                .unwrap_or_else(|| panic!("{name} must resolve a palette"));
            assert!(!colors.is_empty(), "{name} must contribute faces");
            for fc in colors {
                assert!(
                    fc.color.iter().any(|&c| c != 0),
                    "{name:?} face {fc:?} must not be all-black (a broken rect)"
                );
            }
        }
        assert!(block_face_colors("air", &[]).is_none(), "air stays unpainted");
    }
}

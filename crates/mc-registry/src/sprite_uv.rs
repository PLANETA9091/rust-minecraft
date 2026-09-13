//! The vanilla sprite-uv consumer — the quads' 0..16 uv space mapped to
//! texels over generated::textures, with the vanilla missingno
//! checkerboard for the missing materials. Pure data, zero render-path
//! contact: this is the held-item pixel link between element_quads'
//! quads, texture_slots' resolved materials, and the texture namespace
//! (the v0.3.70 pixels).
//!
//! Every rule is javap evidence from the 26.2 client jar:
//! - CuboidFace.getU/getV (net.minecraft.client.resources.model.cuboid.
//!   CuboidFace): UVs.getVertexU(Quadrant.rotateVertexIndex(v)) / 16.0f —
//!   the face's quadrant-rotated 0..16 uv (the walk element_quads
//!   already applied) divides by 16 into the per-sprite [0,1] domain.
//!   Mirrored rects (maxU < minU) ride through as the raw floats.
//! - FaceBakery.bakeVertex: the normalized uv feeds
//!   TextureAtlasSprite.getU/getV DIRECTLY (f18 = f15 when the ModelState
//!   matrix is identity); the non-identity branch rotates the uv around
//!   (0.5, 0.5) by ModelState.inverseFaceTransformation — the blockstate
//!   carrier, identity for standalone element models.
//! - TextureAtlasSprite.getU(u) = u0 + (u1 - u0) * u over the
//!   atlas-normalized span (u0 = (x+pad)/atlasW, u1 = u0 + w/atlasW);
//!   a standalone per-id texture is the whole image (u0 = 0, u1 = 1) so
//!   the mapping is the EXACT f32 identity ((1.0 - 0.0) * u + 0.0 == u;
//!   the /16 and *16 are exponent shifts — no rounding either way).
//! - UVPair.pack: Float.floatToIntBits — the baked quad's uv roundtrips
//!   bit-exact, no quantization.
//! - The atlas sampler (TextureAtlas): SamplerCache.getClampToEdge(
//!   FilterMode.NEAREST) — NEAREST + CLAMP_TO_EDGE: the texel at u is
//!   clamp(floor(u * size), 0, size - 1); u = 1.0 lands on the LAST
//!   texel (no REPEAT wrap), negative/mirrored uv clamps to the edge.
//! - MissingTextureAtlasSprite: the missing material is the literal
//!   `missingno` sprite; generateMissingImage() is 16x16 with magenta
//!   -524040 = 0xFFF800F8 -> RGBA (248, 0, 248, 255) and black
//!   -16777216 -> (0, 0, 0, 255); the pattern (y < h/2) ^ (x < w/2) ->
//!   magenta puts the magenta in the top-right and bottom-left
//!   quadrants. The general generateMissingImage(int, int) keeps the
//!   same XOR at any dims; the atlas materializes 16x16.

use crate::generated::textures::{is_missing, texture_index, texture_rgba, texture_size};

/// The missingno sprite's edge (generateMissingImage() default).
pub const MISSING_SIZE: u32 = 16;

/// 0xFFF800F8 — the missing texture's magenta (NativeImage RGBA byte order).
pub const MISSING_MAGENTA: [u8; 4] = [248, 0, 248, 255];

/// 0xFF000000 — the missing texture's black.
pub const MISSING_BLACK: [u8; 4] = [0, 0, 0, 255];

/// CuboidFace.getU/getV: the quadrant-rotated 0..16 uv -> the per-sprite
/// [0,1] domain. The division is an exact exponent shift in f32.
#[inline]
pub fn normalize_uv(uv: f32) -> f32 {
    uv / 16.0
}

/// TextureAtlasSprite.getU/getV over a standalone id's span (u0 = 0,
/// u1 = 1): spelled out as the vanilla formula — an exact f32 identity
/// for every input (the one exception: -0.0 normalizes to +0.0, which
/// the sampler treats identically).
#[inline]
pub fn sprite_map(u01: f32) -> f32 {
    0.0 + (1.0 - 0.0) * u01
}

/// The NEAREST + CLAMP_TO_EDGE texel index at the normalized coordinate
/// u: clamp(floor(u * size), 0, size - 1). Mirrored (negative) and
/// over-range uv clamp to the edge texels; NaN fails safe to 0.
#[inline]
pub fn nearest_texel(u: f32, size: u32) -> u32 {
    if size == 0 {
        return 0;
    }
    let scaled = (u * size as f32).floor();
    if !(scaled > 0.0) {
        0
    } else if scaled >= (size - 1) as f32 {
        size - 1
    } else {
        scaled as u32
    }
}

/// The vanilla checkerboard pixel at texel (x, y) of a w x h missing
/// sprite — the general generateMissingImage(int, int) XOR; the atlas
/// materializes the 16x16 form.
#[inline]
pub fn missing_pixel(x: u32, y: u32, w: u32, h: u32) -> [u8; 4] {
    if (y < h / 2) ^ (x < w / 2) {
        MISSING_MAGENTA
    } else {
        MISSING_BLACK
    }
}

/// One sampled pixel: the namespace hit's texel, the missing material's
/// checkerboard texel, or Unknown for an id outside the models' domain
/// (a caller bug — the slots' materials ARE the namespace).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sample {
    /// A present texture's RGBA8 texel.
    Pixel([u8; 4]),
    /// A missing id (the 65: the trims/items family + missingno) — the
    /// vanilla checkerboard texel.
    Missing([u8; 4]),
    /// The id is not in the namespace at all (never model-referenced).
    Unknown,
}

/// The namespace key: the slots' values ride AS WRITTEN ("minecraft:x"
/// namespaced — 10287 of 11094 — or bare "x" — 807), while the
/// generated TEXTURE_IDS are the bare jar-path form; the lookup strips
/// exactly one leading "minecraft:".
fn namespace_key(material: &str) -> &str {
    material.strip_prefix("minecraft:").unwrap_or(material)
}

/// The quad-corner fetch: the 0..16 uv -> texel -> RGBA8 over the
/// texture namespace (or the checkerboard for the 65 missing
/// materials). The tint is NOT applied — tintindex rides the quad and
/// the consumer multiplies at draw time (grass_block_top returns the
/// raw grayscale master, never the atlas bake).
pub fn sample_uv(id: &str, u: f32, v: f32) -> Sample {
    let Some(idx) = texture_index(namespace_key(id)) else {
        return Sample::Unknown;
    };
    if is_missing(idx) {
        // The missingno sprite binds in place of the absent PNG: the
        // same /16 -> getU path, over its 16x16 checkerboard.
        let tx = nearest_texel(sprite_map(normalize_uv(u)), MISSING_SIZE);
        let ty = nearest_texel(sprite_map(normalize_uv(v)), MISSING_SIZE);
        return Sample::Missing(missing_pixel(tx, ty, MISSING_SIZE, MISSING_SIZE));
    }
    let (w, h) = texture_size(idx);
    let tx = nearest_texel(sprite_map(normalize_uv(u)), w as u32);
    let ty = nearest_texel(sprite_map(normalize_uv(v)), h as u32);
    let texels = texture_rgba(idx);
    let at = ((ty as usize) * (w as usize) + (tx as usize)) * 4;
    Sample::Pixel([texels[at], texels[at + 1], texels[at + 2], texels[at + 3]])
}

/// The held-item flow in one call: the model's resolved slot material
/// for the face's texture ref, then the uv fetch. None is vanilla's
/// warn+drop — a dangling template ref (block/button's "#texture") or
/// an unknown model binds no sprite and nothing renders.
pub fn sample_material(model_id: &str, texture_ref: &str, u: f32, v: f32) -> Option<Sample> {
    let slots = crate::texture_slots::resolve_model(model_id)?;
    let material = slots.material(texture_ref)?;
    Some(sample_uv(material, u, v))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element_quads::{self, Direction};
    use crate::generated::atlas::TILE_NAMES;
    use crate::texture_slots;

    #[test]
    fn normalizes_the_0_16_space_and_maps_the_standalone_span_exactly() {
        // CuboidFace.getU: the /16 is an exact exponent shift in f32
        assert_eq!(normalize_uv(16.0).to_bits(), 1.0f32.to_bits());
        assert_eq!(normalize_uv(8.0).to_bits(), 0.5f32.to_bits());
        assert_eq!(normalize_uv(0.0).to_bits(), 0.0f32.to_bits());
        assert_eq!(normalize_uv(15.5).to_bits(), 0.96875f32.to_bits());
        assert_eq!(normalize_uv(-1.0).to_bits(), (-0.0625f32).to_bits());
        // TextureAtlasSprite.getU over the standalone span (u0=0, u1=1):
        // the exact identity for every normalized uv
        for uv in [0.0f32, 0.25, 0.5, 0.96875, 1.0, 7.5 / 16.0, 16.0] {
            let u01 = normalize_uv(uv);
            assert_eq!(sprite_map(u01).to_bits(), u01.to_bits());
        }
    }

    #[test]
    fn the_nearest_sampler_clamps_to_edge() {
        assert_eq!(nearest_texel(0.0, 16), 0);
        assert_eq!(nearest_texel(0.5, 16), 8);
        assert_eq!(nearest_texel(0.9375, 16), 15);
        assert_eq!(nearest_texel(1.0, 16), 15, "u = 1.0 lands on the LAST texel (no REPEAT wrap)");
        assert_eq!(nearest_texel(-0.0625, 16), 0, "mirrored uv clamps to the edge");
        assert_eq!(nearest_texel(1.0625, 16), 15);
        assert_eq!(nearest_texel(0.5, 32), 16);
        assert_eq!(nearest_texel(1.0, 32), 31);
        assert_eq!(nearest_texel(f32::NAN, 16), 0, "fail-safe");
        assert_eq!(nearest_texel(1.0, 1), 0);
    }

    #[test]
    fn the_checkerboard_follows_the_vanilla_xor() {
        // 16x16: black TL, magenta TR, magenta BL, black BR
        assert_eq!(missing_pixel(0, 0, 16, 16), MISSING_BLACK);
        assert_eq!(missing_pixel(15, 0, 16, 16), MISSING_MAGENTA);
        assert_eq!(missing_pixel(0, 15, 16, 16), MISSING_MAGENTA);
        assert_eq!(missing_pixel(15, 15, 16, 16), MISSING_BLACK);
        assert_eq!(missing_pixel(7, 7, 16, 16), MISSING_BLACK);
        assert_eq!(missing_pixel(8, 8, 16, 16), MISSING_BLACK);
        // the parameterized generateMissingImage(int, int)
        assert_eq!(missing_pixel(16, 4, 32, 32), MISSING_MAGENTA);
        assert_eq!(missing_pixel(4, 4, 32, 32), MISSING_BLACK);
    }

    #[test]
    fn samples_the_heavy_core_quads_end_to_end() {
        // quads (v0.3.68) -> slots (v0.3.69) -> pixels (v0.3.70): the
        // held-item pixel link against the independent PIL decode of the
        // jar's block/heavy_core.png (16x16 cutout with a hole)
        let quads = element_quads::derive_model("block/heavy_core").expect("an element model");
        let slots = texture_slots::resolve_model("block/heavy_core").expect("a model row");
        let up = quads.iter().find(|q| q.slot == Direction::Up).expect("the up quad");
        assert_eq!(up.texture, "all", "the bare ref rides as written");
        assert_eq!(slots.material(up.texture), Some("block/heavy_core"), "the BARE value");
        // the canonical corner (minX, maxY, minZ) carries uv (0, 0)
        let i = up
            .vertices
            .iter()
            .position(|v| *v == [0.25, 0.5, 0.25])
            .expect("the canonical corner");
        assert_eq!(up.uvs[i], [0.0, 0.0]);
        assert_eq!(
            sample_uv("block/heavy_core", up.uvs[i][0], up.uvs[i][1]),
            Sample::Pixel([146, 153, 171, 255]),
            "PIL pin: the texture's top-left texel"
        );
        // the hole's transparent center: uv (8, 8) -> texel (8, 8)
        assert_eq!(sample_uv("block/heavy_core", 8.0, 8.0), Sample::Pixel([0, 0, 0, 0]));
        // the down face's far uv edge (16, 0): u01 = 1.0 CLAMPS to texel 15
        let down = quads.iter().find(|q| q.slot == Direction::Down).expect("the down quad");
        let j = down
            .vertices
            .iter()
            .position(|v| *v == [0.75, 0.0, 0.75])
            .expect("the far corner");
        assert_eq!(down.uvs[j], [16.0, 0.0]);
        assert_eq!(
            sample_uv("block/heavy_core", down.uvs[j][0], down.uvs[j][1]),
            Sample::Pixel([49, 52, 54, 255]),
            "PIL pin: texel (15, 0) via the CLAMP_TO_EDGE boundary"
        );
    }

    #[test]
    fn missing_materials_sample_the_missingno_checkerboard() {
        // the 16x16 missingno sprite: black TL / magenta TR / magenta BL / black BR
        assert_eq!(sample_uv("missingno", 4.0, 4.0), Sample::Missing(MISSING_BLACK));
        assert_eq!(sample_uv("missingno", 12.0, 4.0), Sample::Missing(MISSING_MAGENTA));
        assert_eq!(sample_uv("missingno", 4.0, 12.0), Sample::Missing(MISSING_MAGENTA));
        assert_eq!(sample_uv("missingno", 12.0, 12.0), Sample::Missing(MISSING_BLACK));
        // one of the 64 missing armor-trim textures rides the same sprite
        assert_eq!(
            sample_uv("trims/items/boots_trim_amethyst", 0.0, 0.0),
            Sample::Missing(MISSING_BLACK)
        );
        // the clamp applies to the missing sprite too: uv 16 -> texel 15
        assert_eq!(sample_uv("missingno", 16.0, 0.0), Sample::Missing(MISSING_MAGENTA));
    }

    #[test]
    fn the_namespace_forms_lookup_identically() {
        let bare = sample_uv("block/heavy_core", 0.0, 0.0);
        let namespaced = sample_uv("minecraft:block/heavy_core", 0.0, 0.0);
        assert_eq!(bare, namespaced, "10287 namespaced vs 807 bare values: one lookup");
        assert_eq!(bare, Sample::Pixel([146, 153, 171, 255]));
        // outside the models' domain: the skin PNG is a jar texture but
        // never model-referenced -> Unknown (a caller bug, not a sprite)
        assert_eq!(sample_uv("entity/player/wide/steve", 0.0, 0.0), Sample::Unknown);
        assert_eq!(sample_uv("block/not_a_texture", 0.0, 0.0), Sample::Unknown);
    }

    #[test]
    fn the_consumer_returns_the_raw_master_no_baked_tint() {
        // grass_block_top: the raw grayscale master — the world atlas
        // bakes the 0x91BD59 plains tint; this consumer does NOT (the
        // tintindex multiply is the draw-time step)
        assert_eq!(
            sample_uv("block/grass_block_top", 0.0, 0.0),
            Sample::Pixel([148, 148, 148, 255])
        );
        assert_eq!(
            sample_uv("block/grass_block_top", 8.0, 8.0),
            Sample::Pixel([179, 179, 179, 255])
        );
        let atlas_idx = TILE_NAMES
            .iter()
            .position(|n| *n == "grass_block_top")
            .expect("the world atlas bakes grass_block_top");
        let baked = ((148u16 * 0x91 + 127) / 255) as u8;
        assert_eq!(
            crate::generated::atlas::TILES[atlas_idx][0], baked,
            "the atlas really carries the bake the consumer must NOT return"
        );
        assert_ne!(
            sample_uv("block/grass_block_top", 0.0, 0.0),
            Sample::Pixel([baked, baked, baked, 255]),
            "the consumer must NOT return the tint-baked atlas pixel"
        );
    }

    #[test]
    fn sample_material_composes_the_slots_and_the_sampler() {
        // heavy_core's bare value, sampled through the full chain
        assert_eq!(
            sample_material("block/heavy_core", "all", 0.0, 0.0),
            Some(Sample::Pixel([146, 153, 171, 255]))
        );
        // the namespaced value form lands identically
        assert_eq!(
            sample_material("block/acacia_shelf", "all", 0.0, 0.0),
            Some(sample_uv("block/acacia_shelf", 0.0, 0.0)),
        );
        // a dangling template ref: vanilla's warn+drop -> no sprite
        assert_eq!(sample_material("block/button", "particle", 0.0, 0.0), None);
        // an unknown model: no slots at all
        assert_eq!(sample_material("block/not_a_model", "all", 0.0, 0.0), None);
    }
}

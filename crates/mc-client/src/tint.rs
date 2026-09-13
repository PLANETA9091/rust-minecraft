//! The biome tint consumers — the FIRST render-facing use of the synced
//! biome table: the decoded `BiomeEntry` effects colors resolved onto the
//! three tinted textures the runtime palette renders (grass_block_top,
//! oak_leaves, water_still).
//!
//! The vanilla chain (26.2): the client resolves the biome color at the
//! block's position — grass/foliage through the biome's colormap lookup
//! (climate temperature/downfall into grass.png/foliage.png) UNLESS
//! `BiomeSpecialEffects` carries an override, water through the REQUIRED
//! `water_color` — and folds it into the quad's VERTEX COLOR before the
//! shader's texel multiply (`vertexColor * texelColor` in the solid /
//! cutout pipelines). This module resolves the same colors from the
//! synced table; the mesher folds them into the same vertex carrier.
//!
//! THE COLORMAP LOOKUP IS CONSUMED (since v0.3.46): an override-less
//! biome now samples the REAL jar grids through the verbatim
//! `ColorMapColorUtil.get` port (`crate::colormap`) — the caller-side
//! `Mth.clamp(0,1)` included, the raw climate field (the audited path
//! ignores `temperature_modifier`). The PLAINS EQUIVALENCE pins the
//! chain: the sample at the plains climate IS the baked
//! TINT_GRASS/TINT_FOLIAGE, so plains stays a no-op downstream while a
//! desert/jungle/snowy climate now folds its REAL color.
//!
//! THE NO-OP RULE (the exactness guarantee): our generated atlas BAKES
//! the vanilla plains constants into the tile pixels at generation time
//! (`mc2rs-gen atlas`: the grayscale masters multiplied by
//! TINT_GRASS/TINT_FOLIAGE/TINT_WATER along the models' tintindex flags),
//! and the generated `TINTS` table records them per (kind, face). A
//! resolved tint EQUAL to the baked default stays unfolded — the pixels
//! already carry that exact color, and a fold would only replay it in
//! float. The fold therefore fires ONLY when the synced table actually
//! changes the color: every existing scene (procedural worlds, the net
//! fixture's registry-less plains) stays bit-identical BY CONSTRUCTION.
//!
//! Documented deviations (deliberate, no silent guesses):
//! - `grass_color_modifier` (swamp / dark_forest) is not consumed:
//!   swamp's exact color rides the biome noise (unreachable noise-free),
//!   dark_forest's is a documented blend — both when the modifier
//!   consumer lands. (Vanilla applies it AFTER this resolution.)
//! - `dry_foliage_color` is decoded in the table but the runtime palette
//!   has no dry-foliage texture to tint yet — carried, unconsumed.
//! - `temperature_modifier` does not feed the lookup — the audited
//!   vanilla colormap path reads the RAW climate field.
//! - A no-jar fallback build (`REAL_GRIDS = false`, zeroed grids) keeps
//!   the pre-colormap verdicts: an override-less biome then reports no
//!   opinion and stays on the baked plains look.

use crate::colormap;
use crate::world::{GRASS, LEAVES, WATER};
use mc_protocol::BiomeEntry;

/// Slot order of the per-biome tint triple: the grass_block_top color
/// (`None` = no opinion — only on a no-jar fallback build; the real
/// grids resolve every climate through the colormap, plains onto the
/// baked constant exactly).
pub const GRASS_SLOT: usize = 0;
/// Slot order of the per-biome tint triple: the oak_leaves color.
pub const FOLIAGE_SLOT: usize = 1;
/// Slot order of the per-biome tint triple: the water color (REQUIRED by
/// the network codec — always present on a decoded entry).
pub const WATER_SLOT: usize = 2;

/// Resolve one synced biome element into the renderer's tint triple.
/// Vanilla's chain, verbatim: the `BiomeSpecialEffects` override wins;
/// without one the climate samples the REAL jar colormap grids (the
/// caller-side clamp inside `grass_from_climate`/`foliage_from_climate`,
/// the raw temperature/downfall fields — `temperature_modifier` is not
/// in the audited path). The water color is the codec's required field
/// (the decoder refuses an entry without it, so `Some` is unconditional).
pub fn entry_tints(entry: &BiomeEntry) -> [Option<u32>; 3] {
    let (temp, downfall) = (entry.temperature, entry.downfall);
    [
        entry
            .effects
            .grass_color
            .or_else(|| colormap::grass_from_climate(temp, downfall)),
        entry
            .effects
            .foliage_color
            .or_else(|| colormap::foliage_from_climate(temp, downfall)),
        Some(entry.effects.water_color),
    ]
}

/// The tint slot a (kind, face) consumes — `None` for every untinted
/// texture (the atlas' tintindex flags: grass BLOCK TOP only, leaves on
/// all faces, water on all faces; a grass SIDE keeps its untinted
/// dirt-and-root look, exactly the blockstate models' verdict).
pub fn slot_for(kind: u16, face: usize) -> Option<usize> {
    if kind == WATER {
        Some(WATER_SLOT)
    } else if kind == GRASS && face == 2 {
        Some(GRASS_SLOT)
    } else if kind == LEAVES {
        Some(FOLIAGE_SLOT)
    } else {
        None
    }
}

/// Unpack a 0xRRGGBB tint into the vertex-color components 0..1.
pub fn unpack(tint: u32) -> [f32; 3] {
    [
        ((tint >> 16) & 0xFF) as f32 / 255.0,
        ((tint >> 8) & 0xFF) as f32 / 255.0,
        (tint & 0xFF) as f32 / 255.0,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::STONE;
    use mc_protocol::{BiomeEffects, GrassColorModifier};

    /// One decoded element with the given effects surface (the real
    /// NETWORK_CODEC shape).
    fn entry(water: u32, grass: Option<u32>, foliage: Option<u32>) -> BiomeEntry {
        BiomeEntry {
            name: "test:biome".into(),
            has_precipitation: true,
            temperature: 0.8,
            temperature_modifier: None,
            downfall: 0.4,
            effects: BiomeEffects {
                water_color: water,
                grass_color: grass,
                foliage_color: foliage,
                dry_foliage_color: None,
                grass_color_modifier: GrassColorModifier::None,
            },
        }
    }

    #[test]
    fn entry_tints_resolve_the_effects_surface() {
        // plains: no overrides — the climate samples the REAL grids and
        // lands exactly on the baked constants (the plains equivalence).
        let plains = entry(4_159_204, None, None);
        assert_eq!(
            entry_tints(&plains),
            [Some(0x91BD59), Some(0x77AB2F), Some(4_159_204)]
        );
        // the badlands family carries REAL overrides in vanilla data —
        // the override wins over the climate sample
        let mesa = entry(0x3F76E4, Some(0x90814D), Some(0x9E814D));
        assert_eq!(
            entry_tints(&mesa),
            [Some(0x90814D), Some(0x9E814D), Some(0x3F76E4)]
        );
    }

    #[test]
    fn override_less_climates_sample_the_real_grids() {
        // desert (temp clamps 2.0 -> 1.0) and jungle — the colormap
        // verdicts the baked atlas could never produce
        let mut desert = entry(0x3F76E4, None, None);
        desert.temperature = 2.0;
        desert.downfall = 0.0;
        assert_eq!(
            entry_tints(&desert),
            [Some(0xBFB755), Some(0xAEA42A), Some(0x3F76E4)]
        );
        let mut jungle = entry(0x3F76E4, None, None);
        jungle.temperature = 0.95;
        jungle.downfall = 0.9;
        assert_eq!(
            entry_tints(&jungle),
            [Some(0x59C93C), Some(0x30BB0B), Some(0x3F76E4)]
        );
    }

    #[test]
    fn slot_for_follows_the_tintindex_flags() {
        assert_eq!(slot_for(WATER, 0), Some(WATER_SLOT));
        assert_eq!(slot_for(WATER, 5), Some(WATER_SLOT), "all water faces");
        assert_eq!(slot_for(GRASS, 2), Some(GRASS_SLOT), "grass top only");
        assert_eq!(slot_for(GRASS, 0), None, "grass side stays untinted");
        assert_eq!(slot_for(GRASS, 3), None, "grass bottom (dirt) untinted");
        assert_eq!(slot_for(LEAVES, 2), Some(FOLIAGE_SLOT));
        assert_eq!(slot_for(LEAVES, 4), Some(FOLIAGE_SLOT), "all leaf faces");
        assert_eq!(slot_for(STONE, 2), None, "untinted texture");
    }

    #[test]
    fn unpack_divides_by_255() {
        assert_eq!(unpack(0xFF0000), [1.0, 0.0, 0.0]);
        let (r, g, b) = (0x91, 0xBD, 0x59);
        assert_eq!(
            unpack(0x91BD59),
            [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
        );
    }
}

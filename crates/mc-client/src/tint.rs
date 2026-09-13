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
//! - The dry-foliage RESOLUTION is consumed since v0.3.48 (the twelfth
//!   javap audit: `Biome.getDryFoliageColor()` = override `orElseGet`
//!   the climate lookup; `DryFoliageColor.get` = the SAME util, default
//!   0x5C3C32). The remaining gap is the FOLD consumer: vanilla's only
//!   registered block source is `BlockTintSources.dryFoliage()` ->
//!   LEAF_LITTER — a non-cube ground cover whose mesh path doesn't exist
//!   yet, so no runtime kind selects `DRY_SLOT` (carried, ready).
//! - pale_oak_leaves carries NO tint slot by AUDITED VERDICT: vanilla's
//!   `BlockColors.createDefault()` registers no source for it (the
//!   model's tintindex falls back to BLANK_LAYER — constant white) and
//!   the probe showed the texture is PRE-COLORED (zero pure-gray
//!   pixels). It renders as authored, exactly vanilla.
//! - `temperature_modifier` does not feed the lookup — the audited
//!   vanilla colormap path reads the RAW climate field.
//! - `grass_color_modifier` IS consumed since v0.3.47 — but only the
//!   position-free DARK_FOREST blend; SWAMP's exact color rides
//!   `Biome.BIOME_INFO_NOISE` at (x·0.0225, z·0.0225) (two fixed
//!   verdicts, position-dependent) — unreachable without per-position
//!   noise, the base passes through (documented).
//! - A no-jar fallback build (`REAL_GRIDS = false`, zeroed grids) keeps
//!   the pre-colormap verdicts: an override-less biome then reports no
//!   opinion and stays on the baked plains look.

use crate::colormap;
use crate::world::{GRASS, LEAVES, PALE_LEAVES, WATER};
use mc_protocol::{BiomeEntry, GrassColorModifier};

/// Slot order of the per-biome tint quad: the grass_block_top color
/// (`None` = no opinion — only on a no-jar fallback build; the real
/// grids resolve every climate through the colormap, plains onto the
/// baked constant exactly).
pub const GRASS_SLOT: usize = 0;
/// Slot order of the per-biome tint quad: the oak_leaves color.
pub const FOLIAGE_SLOT: usize = 1;
/// Slot order of the per-biome tint quad: the water color (REQUIRED by
/// the network codec — always present on a decoded entry).
pub const WATER_SLOT: usize = 2;
/// Slot order of the per-biome tint quad: the dry-foliage color (the
/// twelfth javap audit's resolution — override `orElseGet` the climate
/// through the dry_foliage grid). No runtime kind selects it yet:
/// vanilla's sole fold consumer is leaf_litter, a non-cube ground cover
/// (the mesh path is future work) — the slot is carried, ready.
pub const DRY_SLOT: usize = 3;

/// Resolve one synced biome element into the renderer's tint quad.
/// Vanilla's chain, verbatim: the `BiomeSpecialEffects` override wins;
/// without one the climate samples the REAL jar colormap grids (the
/// caller-side clamp inside `grass_from_climate`/`foliage_from_climate`/
/// `dry_foliage_from_climate`, the raw temperature/downfall fields —
/// `temperature_modifier` is not in the audited path). The grass verdict
/// then runs through the `grass_color_modifier` (vanilla wraps the BASE
/// in `modifyColor(x, z, base)` — after this resolution). The water
/// color is the codec's required field (the decoder refuses an entry
/// without it, so `Some` is unconditional). The dry-foliage verdict
/// mirrors `Biome.getDryFoliageColor()`'s `orElseGet` shape exactly.
pub fn entry_tints(entry: &BiomeEntry) -> [Option<u32>; 4] {
    let (temp, downfall) = (entry.temperature, entry.downfall);
    let grass = entry
        .effects
        .grass_color
        .or_else(|| colormap::grass_from_climate(temp, downfall))
        .map(|base| apply_grass_modifier(entry.effects.grass_color_modifier, base));
    [
        grass,
        entry
            .effects
            .foliage_color
            .or_else(|| colormap::foliage_from_climate(temp, downfall)),
        Some(entry.effects.water_color),
        entry
            .effects
            .dry_foliage_color
            .or_else(|| colormap::dry_foliage_from_climate(temp, downfall)),
    ]
}

/// The vanilla `grass_color_modifier` over the RESOLVED base color —
/// `BiomeSpecialEffects.GrassColorModifier.modifyColor(x, z, base)` (the
/// eleventh javap audit, 26.2; the enum's declaration order is NONE,
/// DARK_FOREST, SWAMP — the numbered subclasses map accordingly).
///
/// DARK_FOREST is pure arithmetic on the whole int (the carry between
/// channels is part of the vanilla result, reproduced verbatim):
///
/// ```java
/// ARGB.opaque(((grassColor & 0xFEFEFE) + 0x28340A) >> 1)
/// ```
///
/// `ARGB.opaque(i) = i | 0xFF000000` — the alpha OR is outside our
/// 0xRRGGBB tint representation, so the stored verdict is the blend
/// itself. SWAMP is NOT portable to this per-entry triple: it samples
/// `Biome.BIOME_INFO_NOISE` (a static PerlinSimplexNoise) at
/// (x·0.0225, z·0.0225) and returns one of TWO FIXED colors
/// (`< -0.1` -> the packed 0xFF4C763C, else the packed 0xFF6A7039) —
/// genuinely position-dependent, so the base passes through and the
/// deviation stays documented. NONE is the identity (the bytecode
/// returns the input unchanged).
pub fn apply_grass_modifier(modifier: GrassColorModifier, base: u32) -> u32 {
    match modifier {
        GrassColorModifier::DarkForest => ((base & 0xFEFEFE) + DARK_FOREST_BLEND) >> 1,
        GrassColorModifier::Swamp | GrassColorModifier::None => base,
    }
}

/// The dark_forest blend's vanilla constant (`ldc 2634762`).
const DARK_FOREST_BLEND: u32 = 0x28340A;

/// The tint slot a (kind, face) consumes — `None` for every untinted
/// texture (the atlas' tintindex flags: grass BLOCK TOP only, leaves on
/// all faces, water on all faces; a grass SIDE keeps its untinted
/// dirt-and-root look, exactly the blockstate models' verdict).
///
/// pale_oak_leaves is `None` BY AUDITED VERDICT (the twelfth javap
/// audit): `BlockColors.createDefault()` registers no tint source for it
/// — the model's tintindex falls back to BLANK_LAYER (constant white)
/// and the PRE-COLORED texture renders as authored (probe-verified:
/// zero pure-gray pixels). The dry-foliage consumer is leaf_litter, a
/// non-cube ground cover outside the mesh vocabulary so far.
pub fn slot_for(kind: u16, face: usize) -> Option<usize> {
    if kind == WATER {
        Some(WATER_SLOT)
    } else if kind == GRASS && face == 2 {
        Some(GRASS_SLOT)
    } else if kind == LEAVES {
        Some(FOLIAGE_SLOT)
    } else {
        // PALE_LEAVES lands here deliberately — no tint, exactly vanilla
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
        // lands exactly on the baked constants (the plains equivalence);
        // the dry slot samples its grid too (no plains involvement there
        // — the honest climate verdict, probe-pinned 0xA37546).
        let plains = entry(4_159_204, None, None);
        assert_eq!(
            entry_tints(&plains),
            [Some(0x91BD59), Some(0x77AB2F), Some(4_159_204), Some(0xA37546)]
        );
        // the badlands family carries REAL overrides in vanilla data —
        // the override wins over the climate sample (dry stays
        // override-less -> the climate verdict)
        let mesa = entry(0x3F76E4, Some(0x90814D), Some(0x9E814D));
        assert_eq!(
            entry_tints(&mesa),
            [Some(0x90814D), Some(0x9E814D), Some(0x3F76E4), Some(0xA37546)]
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
            [Some(0xBFB755), Some(0xAEA42A), Some(0x3F76E4), Some(0xA38046)]
        );
        let mut jungle = entry(0x3F76E4, None, None);
        jungle.temperature = 0.95;
        jungle.downfall = 0.9;
        assert_eq!(
            entry_tints(&jungle),
            [Some(0x59C93C), Some(0x30BB0B), Some(0x3F76E4), Some(0xA36346)]
        );
    }

    #[test]
    fn dry_foliage_follows_the_audited_ors_get_chain() {
        // The REAL pale_garden entry (the jar's biome json): the dry
        // override #A0A69C wins — `Biome.getDryFoliageColor()`'s
        // orElseGet never consults the climate. The same entry's
        // grass/foliage overrides (also present in the json) ride their
        // own slots; the modifier is NONE here (pale_garden declares no
        // grass_color_modifier).
        let mut pale = entry(0x76889D, Some(0x778272), Some(0x878D76));
        pale.temperature = 0.7;
        pale.downfall = 0.8;
        pale.effects.dry_foliage_color = Some(0xA0A69C);
        let tints = entry_tints(&pale);
        assert_eq!(tints[0], Some(0x778272), "grass override");
        assert_eq!(tints[1], Some(0x878D76), "foliage override");
        assert_eq!(tints[2], Some(0x76889D), "required water");
        assert_eq!(tints[3], Some(0xA0A69C), "dry override wins over the climate");
        // override-less: the climate through the dry grid (the orElseGet
        // leg — the plains verdict probe-pinned at 0xA37546)
        let no_override = entry(0x3F76E4, None, None);
        assert_eq!(entry_tints(&no_override)[3], Some(0xA37546));
    }

    #[test]
    fn dark_forest_blends_the_resolved_base() {
        // The audited blend over the plains sample: (0x91BD59 & 0xFEFEFE)
        // + 0x28340A >> 1 = 0x5C7831.
        let mut dark = entry(0x3F76E4, None, None);
        dark.effects.grass_color_modifier = GrassColorModifier::DarkForest;
        assert_eq!(entry_tints(&dark)[0], Some(0x5C7831));
        // the whole-int arithmetic is verbatim — a high-red base CARRIES
        // into bit 24 and the shift folds it back (0xFE0000 + 0x28340A =
        // 0x126340A >> 1 = 0x931A05, not a per-channel average)
        assert_eq!(apply_grass_modifier(GrassColorModifier::DarkForest, 0xFE0000), 0x931A05);
        // the override is the BASE too — the blend runs over whatever
        // the resolution produced (vanilla wraps modifyColor around it)
        let mut overridden = entry(0x3F76E4, Some(0x91BD59), Some(0x9E814D));
        overridden.effects.grass_color_modifier = GrassColorModifier::DarkForest;
        assert_eq!(entry_tints(&overridden)[0], Some(0x5C7831));
        // foliage and water ride untouched — the modifier is grass-only
        assert_eq!(entry_tints(&dark)[1], Some(0x77AB2F));
        assert_eq!(entry_tints(&dark)[2], Some(0x3F76E4));
    }

    #[test]
    fn dark_forest_climate_lands_the_canonical_color() {
        // The END-TO-END vanilla verdict: the real grid sample at the
        // dark forest climate (0.7, 0.8) is 0x79C05A; the blend turns it
        // into 0x507A32 — the canonical dark forest grass color (probe-
        // verified against the jar before any client code existed).
        let mut dark = entry(0x3F76E4, None, None);
        dark.temperature = 0.7;
        dark.downfall = 0.8;
        dark.effects.grass_color_modifier = GrassColorModifier::DarkForest;
        assert_eq!(entry_tints(&dark)[0], Some(0x507A32));
    }

    #[test]
    fn swamp_modifier_passes_the_base_through() {
        // The documented deviation: swamp's verdict is BIOME_INFO_NOISE
        // at the block's (x, z) — a per-position noise this per-entry
        // triple cannot express; the base passes through (the climate
        // sample here, the same verdict as v0.3.46).
        let mut swamp = entry(0x617B64, None, None);
        swamp.temperature = 0.8;
        swamp.downfall = 0.9;
        swamp.effects.grass_color_modifier = GrassColorModifier::Swamp;
        assert_eq!(entry_tints(&swamp)[0], Some(0x6AC44E));
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
        // the AUDITED verdict: pale oak leaves have NO tint source in
        // vanilla's BlockColors — every face untinted, the pre-colored
        // texture renders as authored (despite the model's tintindex)
        assert_eq!(slot_for(PALE_LEAVES, 2), None, "pale oak: no source");
        assert_eq!(slot_for(PALE_LEAVES, 0), None, "pale oak: no source");
        assert_eq!(slot_for(PALE_LEAVES, 5), None, "pale oak: no source");
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

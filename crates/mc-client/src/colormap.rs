//! The vanilla biome colormap consumer — the climate-driven grass/foliage
//! lookup, ported verbatim from the 26.2 bytecode (the tenth javap audit,
//! official jar, sha1 2dc72797).
//!
//! THE AUDIT: `net.minecraft.world.level.ColorMapColorUtil.get(double temp,
//! double rain, int[] pixels, int defaultMapColor)` disassembles to
//!
//! ```java
//! rain = rain * temp;                        // downfall scaled by temperature FIRST
//! int i = (int)((1.0 - temp) * 255.0);       // column
//! int j = (int)((1.0 - rain) * 255.0);       // row
//! int index = (j << 8) | i;                  // row-major, 256-wide
//! return index >= pixels.length ? defaultMapColor : pixels[index];
//! ```
//!
//! The CALLER clamps: `Biome.getGrassColorFromTexture` runs
//! `Mth.clamp(climateSettings.temperature, 0.0F, 1.0F)` (and the same for
//! downfall) on FLOATS and widens with f2d — the util itself does NOT
//! clamp. `GrassColor.get` passes the grid + default 0xFF00FF (the
//! not-loaded magenta); `FoliageColor.get` the same shape with default
//! 0xFF48B518. Grass then runs the `grass_color_modifier` OVER the base
//! (override-or-texture) color; foliage has no modifier.
//!
//! THE GRIDS live in `mc-registry/src/generated/colormap.rs`: the jar's
//! grass.png/foliage.png/dry_foliage.png decoded row-major to opaque
//! 0xRRGGBB. A no-jar fallback build emits `REAL_GRIDS = false` with zeroed
//! grids — the climate lookup then reports no opinion and the resolution
//! keeps the baked-plains look (the same degraded mode the TILE_NONE atlas
//! fallback drives).
//!
//! THE PLAINS EQUIVALENCE (the chain's end-to-end proof, pinned as a
//! test): the atlas' baked constants ARE the vanilla colormap samples at
//! the plains climate — get(0.8, 0.4) == 0x91BD59 (grass) and 0x77AB2F
//! (foliage). Verified against the real jar grids before a line of this
//! module existed; it validates the PNG decode, the packing, the
//! row-major indexing and the `rain *= temp` ordering in one shot.
//!
//! Deviations (deliberate, no silent guesses):
//! - Vanilla CRASHES (negative array index) on unclamped inputs; this
//!   port returns the default for an out-of-range index. Unreachable
//!   through the clamped caller — strictly safer, never observed.
//! - The i64 index math differs from Java's wrapping `(j << 8) | i` only
//!   for inputs where vanilla crashes anyway (clamped inputs stay in
//!   [0, 65535] where the forms are exact).
//! - The `grass_color_modifier` remains decoded-but-position-bound in
//!   tint.rs (swamp's per-position noise); `temperature_modifier` does
//!   NOT feed this lookup — the audited vanilla path reads the RAW
//!   climate field (the modifier steers freeze/snow logic elsewhere).
//! - THE DRY FOLIAGE CHAIN IS CONSUMED (since v0.3.48, the twelfth
//!   javap audit): `Biome.getDryFoliageColor()` is
//!   `effects.dryFoliageColorOverride().orElseGet(climate lookup)`, the
//!   climate lookup byte-identical to its foliage sibling (the raw
//!   fields, the caller-side `Mth.clamp`, f2d widening), and
//!   `DryFoliageColor.get` delegates the SAME `ColorMapColorUtil.get`
//!   with the not-loaded default -10732494 (0xFF5C3C32). Vanilla's only
//!   RENDER consumer is leaf_litter (a non-cube ground cover — its mesh
//!   path is future work); pale_oak_leaves carries NO tint source at all
//!   (the audited `BlockColors.createDefault()` — the model's tintindex
//!   falls back to BLANK_LAYER and the pre-colored texture renders as
//!   authored). The resolution surface is complete either way.

use mc_registry::generated::colormap::{DRY_FOLIAGE, FOLIAGE, GRASS, REAL_GRIDS};

/// The vanilla not-loaded defaults — unreachable for clamped inputs (the
/// generated grids hold 65536 entries), carried verbatim from the
/// bytecode (`ldc -65281` / `ldc -12012264` / the twelfth audit's
/// `ldc -10732494`, the `FOLIAGE_DRY_DEFAULT` ConstantValue — the RGB
/// part of the 0xFF5C3C32 argb word).
pub const GRASS_DEFAULT: u32 = 0xFF00FF;
pub const FOLIAGE_DEFAULT: u32 = 0xFF48B518;
pub const DRY_FOLIAGE_DEFAULT: u32 = 0x5C3C32;

/// `Mth.clamp(v, 0.0F, 1.0F)` widened to f64 — the audited caller-side
/// clamp. The comparison chain passes NaN through (exactly vanilla); the
/// util's `(int)` cast lands NaN at 0 downstream, identically.
fn clamp01(v: f32) -> f64 {
    if v < 0.0 {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v as f64
    }
}

/// The verbatim `ColorMapColorUtil.get`. Takes an ALREADY-clamped climate
/// (the util does not clamp — the caller does, vanilla and here).
pub fn get(temp: f64, rain: f64, grid: &[u32], default: u32) -> u32 {
    let rain = rain * temp;
    // Java d2i truncates toward zero — Rust's `as` matches (NaN -> 0,
    // saturating at the int bounds).
    let i = ((1.0 - temp) * 255.0) as i32;
    let j = ((1.0 - rain) * 255.0) as i32;
    let idx = ((j as i64) << 8) | i as i64;
    if idx < 0 || idx as usize >= grid.len() {
        default
    } else {
        grid[idx as usize]
    }
}

/// `Biome.getGrassColorFromTexture`: the clamped climate into the grass
/// grid. `None` only on a fallback (no-jar) build — the caller then keeps
/// the baked plains look (the pre-colormap behavior).
pub fn grass_from_climate(temp: f32, downfall: f32) -> Option<u32> {
    if !REAL_GRIDS {
        return None;
    }
    Some(get(clamp01(temp), clamp01(downfall), &GRASS, GRASS_DEFAULT))
}

/// `Biome.getFoliageColorFromTexture`: the clamped climate into the
/// foliage grid (no modifier in vanilla either).
pub fn foliage_from_climate(temp: f32, downfall: f32) -> Option<u32> {
    if !REAL_GRIDS {
        return None;
    }
    Some(get(
        clamp01(temp),
        clamp01(downfall),
        &FOLIAGE,
        FOLIAGE_DEFAULT,
    ))
}

/// `Biome.getDryFoliageColorFromTexture` (the twelfth javap audit):
/// byte-identical to the foliage sibling except the grid and the default
/// — the RAW climate fields, the caller-side `Mth.clamp(0,1)` with f2d
/// widening, the SAME `ColorMapColorUtil.get`. The override is resolved
/// by the caller (`Biome.getDryFoliageColor`'s `orElseGet` — tint.rs
/// carries that shape).
pub fn dry_foliage_from_climate(temp: f32, downfall: f32) -> Option<u32> {
    if !REAL_GRIDS {
        return None;
    }
    Some(get(
        clamp01(temp),
        clamp01(downfall),
        &DRY_FOLIAGE,
        DRY_FOLIAGE_DEFAULT,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plains_climate_resolves_the_baked_constants() {
        // THE chain proof: the atlas bake (TINT_GRASS/TINT_FOLIAGE) was
        // generated from the vanilla plains constants; the real grids
        // MUST reproduce them at the plains climate or the no-op rule
        // would be fiction.
        assert_eq!(grass_from_climate(0.8, 0.4), Some(0x91BD59));
        assert_eq!(foliage_from_climate(0.8, 0.4), Some(0x77AB2F));
    }

    #[test]
    fn known_biome_climates_sample_the_known_constants() {
        // Vanilla-canonical colors, verified against the jar grids before
        // pinning (the probe script): desert grass #BFB755, jungle grass
        // #59C93C, snowy plains grass #80B497.
        assert_eq!(grass_from_climate(2.0, 0.0), Some(0xBFB755), "desert");
        assert_eq!(grass_from_climate(0.95, 0.9), Some(0x59C93C), "jungle");
        assert_eq!(grass_from_climate(0.0, 0.5), Some(0x80B497), "snowy plains");
        assert_eq!(
            foliage_from_climate(0.95, 0.9),
            Some(0x30BB0B),
            "jungle foliage"
        );
        // The clamping is the caller's job: 2.0 lands at 1.0 — the desert
        // sample IS the (1.0, 0.0) corner.
        assert_eq!(
            get(clamp01(2.0), clamp01(0.0), &GRASS, GRASS_DEFAULT),
            get(1.0, 0.0, &GRASS, GRASS_DEFAULT)
        );
    }

    #[test]
    fn rain_is_scaled_by_temperature_before_the_row_pick() {
        // The util's first op: rain *= temp. At (0.5, 1.0) the row comes
        // from rain*temp = 0.5 -> j = 127, NOT the downfall-only j = 0.
        // i = (1-0.5)*255 = 127 (Java truncation) -> grid[127*256 + 127].
        assert_eq!(get(0.5, 1.0, &GRASS, GRASS_DEFAULT), GRASS[127 * 256 + 127]);
        assert_ne!(get(0.5, 1.0, &GRASS, GRASS_DEFAULT), GRASS[127]);
    }

    #[test]
    fn get_matches_a_hand_indexed_grid_position() {
        // (0.25, 0.75): i = (int)(0.75*255) = 191, rain = 0.1875,
        // j = (int)(0.8125*255) = 207 -> idx 207*256 + 191.
        assert_eq!(get(0.25, 0.75, &FOLIAGE, FOLIAGE_DEFAULT), FOLIAGE[207 * 256 + 191]);
    }

    #[test]
    fn clamp01_mirrors_mth_clamp() {
        assert_eq!(clamp01(-0.25), 0.0);
        assert_eq!(clamp01(2.0), 1.0);
        assert_eq!(
            clamp01(0.8),
            0.8f32 as f64,
            "in-range value widens unchanged (f32 -> f64 exact)"
        );
        assert!(
            clamp01(f32::NAN).is_nan(),
            "NaN passes through, like Mth.clamp's comparison chain"
        );
        // and the util's d2i lands NaN at 0 — the vanilla (int) cast
        assert_eq!(((1.0 - clamp01(f32::NAN)) * 255.0) as i32, 0);
    }

    #[test]
    fn out_of_range_index_takes_the_default() {
        // Unreachable through the clamped callers (idx <= 65535 < len);
        // the guard replaces vanilla's negative-index crash. Unclamped
        // temp 2.0: i = -255 — the Java OR folds the sign (456960 | -255
        // is negative) and the array load CRASHES; the port's idx < 0
        // guard takes the default instead.
        let tiny = [0x112233u32; 4];
        assert_eq!(get(2.0, -3.0, &tiny, 0xABCDEF), 0xABCDEF);
        // NaN temp lands at (int)0 both in Java and here: rain = NaN,
        // i = j = 0 -> idx 0 -> IN range -> the first cell (a crash-free
        // vanilla-identical landing, not the default).
        assert_eq!(get(f64::NAN, 0.5, &tiny, 0xABCDEF), 0x112233);
    }

    #[test]
    fn real_builds_report_climate_opinions() {
        // The committed grids are the REAL jar data; a fallback build
        // (REAL_GRIDS=false, zeroed) reports no opinion instead of black.
        if REAL_GRIDS {
            assert!(grass_from_climate(0.8, 0.4).is_some());
            assert!(foliage_from_climate(0.8, 0.4).is_some());
        } else {
            assert_eq!(grass_from_climate(0.8, 0.4), None);
            assert_eq!(foliage_from_climate(0.8, 0.4), None);
        }
        assert_eq!(
            mc_registry::generated::colormap::COLORMAP_VERSION,
            "26.2"
        );
        assert_eq!(GRASS.len(), 65536);
        assert_eq!(FOLIAGE.len(), 65536);
        assert_eq!(DRY_FOLIAGE.len(), 65536);
    }

    #[test]
    fn dry_foliage_climates_sample_the_probed_constants() {
        // Probe-verified against the real jar grid BEFORE any client code
        // (scripts/probe_dry_foliage.py, the same
        // ColorMapColorUtil.get arithmetic): the dry grid's palette is a
        // narrow autumn band (all samples in the 0xA3xx4x range).
        assert_eq!(dry_foliage_from_climate(0.8, 0.4), Some(0xA37546), "plains climate");
        assert_eq!(
            dry_foliage_from_climate(0.7, 0.8),
            Some(0xA36D46),
            "pale_garden climate (the override wins in-world; the grid sample is the no-override verdict)"
        );
        assert_eq!(dry_foliage_from_climate(1.0, 0.0), Some(0xA38046), "desert corner");
        assert_eq!(dry_foliage_from_climate(0.95, 0.9), Some(0xA36346), "jungle");
        // the util is the SAME audited arithmetic — a hand-indexed cell
        // pins the ordering (rain *= temp) on the dry grid too:
        // (0.5, 1.0): rain=0.5, i = (int)(0.5*255) = 127, j = 127 ->
        assert_eq!(
            get(0.5, 1.0, &DRY_FOLIAGE, DRY_FOLIAGE_DEFAULT),
            DRY_FOLIAGE[127 * 256 + 127]
        );
    }
}

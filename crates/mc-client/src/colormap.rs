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
//! THE GRIDS live in the runtime cache (the generated/ -> runtime-load
//! migration's fourth lane — the 1.8MB decoded grids the compiled source
//! no longer carries): the jar's grass.png/foliage.png/dry_foliage.png the
//! cache keeps VERBATIM (`assets-cache/jar/assets/minecraft/textures/
//! colormap/` — the generator's own copy walk, the jar entry the truth),
//! the loader the only decoder (the generator's decode walk verbatim —
//! EXPAND|STRIP_16, the 256x256 RGBA8 shape law, the alpha-opaque check,
//! the row-major 0xRRGGBB pack). The absent/broken cache the honest
//! `real = false` keeps — the climate lookup then reports no opinion and
//! the resolution keeps the baked-plains look (the same degraded mode the
//! TILE_NONE atlas fallback drives). The OnceLock the load-once law keeps.
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

use anyhow::Context as _;

/// The vanilla not-loaded defaults — unreachable for clamped inputs (the
/// loaded grids hold 65536 entries), carried verbatim from the
/// bytecode (`ldc -65281` / `ldc -12012264` / the twelfth audit's
/// `ldc -10732494`, the `FOLIAGE_DRY_DEFAULT` ConstantValue — the RGB
/// part of the 0xFF5C3C32 argb word).
pub const GRASS_DEFAULT: u32 = 0xFF00FF;
pub const FOLIAGE_DEFAULT: u32 = 0xFF48B518;
pub const DRY_FOLIAGE_DEFAULT: u32 = 0x5C3C32;

/// THE LOADED GRIDS (the generated/ -> runtime-load migration's fourth
/// lane — the 1.8MB decoded grids the compiled source no longer carries):
/// the jar's three colormap PNGs decoded row-major to opaque 0xRRGGBB.
/// `real` false when the cache is absent/broken — the climate lookups
/// then report no opinion and the resolution keeps the baked-plains look
/// (the same degraded mode the old `REAL_GRIDS = false` fallback build
/// spoke, the TILE_NONE atlas fallback's sibling).
#[derive(Debug, Clone, PartialEq)]
pub struct ColormapGrids {
    pub grass: Vec<u32>,
    pub foliage: Vec<u32>,
    pub dry_foliage: Vec<u32>,
    pub real: bool,
}

/// The vanilla colormap side (a 256x256 grid, one 0xRRGGBB per cell).
const COLORMAP_SIDE: usize = 256;

/// THE DECODE WALK (the generator's own `decode_rgba` verbatim — the same
/// png 0.17 the workspace pins, the same EXPAND|STRIP_16 transformations):
/// any official PNG into RGBA8 (palette/grayscale expanded, 16-bit
/// stripped).
fn decode_rgba(bytes: &[u8]) -> anyhow::Result<(u32, u32, Vec<u8>)> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().context("png read_info")?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).context("png next_frame")?;
    let (w, h) = (info.width, info.height);
    let samples = info.color_type.samples();
    let n = (w as usize) * (h as usize);
    let mut rgba = Vec::with_capacity(n * 4);
    for px in 0..n {
        let off = px * samples;
        match info.color_type {
            png::ColorType::Grayscale => {
                let g = buf[off];
                rgba.extend_from_slice(&[g, g, g, 255]);
            }
            png::ColorType::GrayscaleAlpha => {
                let (g, a) = (buf[off], buf[off + 1]);
                rgba.extend_from_slice(&[g, g, g, a]);
            }
            png::ColorType::Rgb => {
                rgba.extend_from_slice(&[buf[off], buf[off + 1], buf[off + 2], 255])
            }
            png::ColorType::Rgba => {
                rgba.extend_from_slice(&[buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
            }
            png::ColorType::Indexed => anyhow::bail!("indexed survived EXPAND"),
        }
    }
    Ok((w, h, rgba))
}

/// THE PARSE LAW (the old generator's emit walk verbatim — the same shape
/// checks, the same opaque rule, the same row-major pack). Any violation
/// the honest None keeps — the boot then walks the next cache root (or
/// the baked-plains fallback answers).
fn parse_grid(bytes: &[u8]) -> Option<Vec<u32>> {
    let (w, h, rgba) = decode_rgba(bytes).ok()?;
    let n = COLORMAP_SIDE * COLORMAP_SIDE;
    if w as usize != COLORMAP_SIDE || h as usize != COLORMAP_SIDE || rgba.len() != n * 4 {
        return None;
    }
    let mut grid = Vec::with_capacity(n);
    for px in rgba.chunks_exact(4) {
        if px[3] != 255 {
            return None;
        }
        grid.push(((px[0] as u32) << 16) | ((px[1] as u32) << 8) | px[2] as u32);
    }
    Some(grid)
}

/// THE CACHE WALK (the splash/lang/sounds loaders' own roots): the
/// workspace root the binary's cwd keeps, the crate dir the cargo test
/// cwd one and two steps below. All three grids must parse at one root —
/// a partial cache the next root's chance keeps.
fn load_grids() -> ColormapGrids {
    for prefix in [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ] {
        let dir = std::path::Path::new(prefix).join("assets/minecraft/textures/colormap");
        let (Some(grass), Some(foliage), Some(dry_foliage)) = (
            std::fs::read(dir.join("grass.png")).ok().and_then(|b| parse_grid(&b)),
            std::fs::read(dir.join("foliage.png")).ok().and_then(|b| parse_grid(&b)),
            std::fs::read(dir.join("dry_foliage.png"))
                .ok()
                .and_then(|b| parse_grid(&b)),
        ) else {
            continue;
        };
        return ColormapGrids {
            grass,
            foliage,
            dry_foliage,
            real: true,
        };
    }
    ColormapGrids {
        grass: Vec::new(),
        foliage: Vec::new(),
        dry_foliage: Vec::new(),
        real: false,
    }
}

/// THE LOADED GRIDS (the OnceLock the load-once law keeps — the 'static
/// borrow the sound/lang loaders' own shape answers).
fn grids() -> &'static ColormapGrids {
    static GRIDS: std::sync::OnceLock<ColormapGrids> = std::sync::OnceLock::new();
    GRIDS.get_or_init(load_grids)
}

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
/// grid. `None` only when the cache is absent/broken — the caller then
/// keeps the baked plains look (the pre-colormap behavior).
pub fn grass_from_climate(temp: f32, downfall: f32) -> Option<u32> {
    let g = grids();
    if !g.real {
        return None;
    }
    Some(get(clamp01(temp), clamp01(downfall), &g.grass, GRASS_DEFAULT))
}

/// `Biome.getFoliageColorFromTexture`: the clamped climate into the
/// foliage grid (no modifier in vanilla either).
pub fn foliage_from_climate(temp: f32, downfall: f32) -> Option<u32> {
    let g = grids();
    if !g.real {
        return None;
    }
    Some(get(
        clamp01(temp),
        clamp01(downfall),
        &g.foliage,
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
    let g = grids();
    if !g.real {
        return None;
    }
    Some(get(
        clamp01(temp),
        clamp01(downfall),
        &g.dry_foliage,
        DRY_FOLIAGE_DEFAULT,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An in-memory PNG encoder for the parse-law walk (the same png
    /// encoder the shots/ writer speaks, a Vec sink instead of a file).
    fn encode_png(w: u32, h: u32, color: png::ColorType, data: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, w, h);
            enc.set_color(color);
            enc.set_depth(png::BitDepth::Eight);
            let mut writer = enc.write_header().unwrap();
            writer.write_image_data(data).unwrap();
            writer.finish().unwrap();
        }
        out
    }

    /// The identity grid: cell i holds i — every hand-indexed law reads
    /// the arithmetic straight off the value (cache-free, the CI no-cache
    /// boot runs it too).
    fn identity_grid() -> Vec<u32> {
        (0..COLORMAP_SIDE * COLORMAP_SIDE).map(|i| i as u32).collect()
    }

    #[test]
    fn the_parse_law_keeps_the_png_walk() {
        // THE happy path: a 256x256 RGB PNG (the jar's own colormap
        // shape), three of the four corners pinned, the pack row-major.
        let mut px = vec![0u8; 256 * 256 * 3];
        let mut set = |px: &mut [u8], x: usize, y: usize, rgb: [u8; 3]| {
            let o = (y * 256 + x) * 3;
            px[o] = rgb[0];
            px[o + 1] = rgb[1];
            px[o + 2] = rgb[2];
        };
        set(&mut px, 0, 0, [0x12, 0x34, 0x56]);
        set(&mut px, 255, 0, [0x78, 0x9A, 0xBC]);
        set(&mut px, 0, 255, [0xDE, 0xF0, 0x12]);
        let grid = parse_grid(&encode_png(256, 256, png::ColorType::Rgb, &px)).unwrap();
        assert_eq!(grid.len(), 65536);
        assert_eq!(grid[0], 0x123456);
        assert_eq!(grid[255], 0x789ABC);
        assert_eq!(grid[255 * 256], 0xDEF012);
        assert_eq!(grid[65535], 0, "untouched pixel packs zero");

        // THE SHAPE LAW: wrong dims the None keeps (the old generator's
        // expected-{s}x{s} bail, the boot's next-root walk the same arm).
        assert_eq!(
            parse_grid(&encode_png(2, 2, png::ColorType::Rgba, &[1; 16])),
            None
        );

        // THE OPAQUE LAW: an RGBA pixel at alpha 200 the None keeps.
        let mut alpha = vec![255u8; 256 * 256 * 4];
        alpha[3] = 200;
        assert_eq!(
            parse_grid(&encode_png(256, 256, png::ColorType::Rgba, &alpha)),
            None
        );

        // THE BYTES LAW: broken bytes the None keeps (the honest
        // fallback, never a panic).
        assert_eq!(parse_grid(b"not a png at all"), None);
    }

    #[test]
    fn the_indexing_laws_walk_the_synthetic_grid() {
        // The laws are about INDEXING, not the jar's values — the identity
        // grid pins them without the cache. The util's first op: rain *=
        // temp. At (0.5, 1.0) the row comes from rain*temp = 0.5 -> j =
        // 127, NOT the downfall-only j = 0; i = (1-0.5)*255 = 127 (Java
        // truncation) -> idx 127*256 + 127.
        let g = identity_grid();
        assert_eq!(get(0.5, 1.0, &g, GRASS_DEFAULT), 127 * 256 + 127);
        assert_ne!(get(0.5, 1.0, &g, GRASS_DEFAULT), 127);
        // (0.25, 0.75): i = (int)(0.75*255) = 191, rain = 0.1875,
        // j = (int)(0.8125*255) = 207 -> idx 207*256 + 191.
        assert_eq!(get(0.25, 0.75, &g, FOLIAGE_DEFAULT), 207 * 256 + 191);
        // The clamping is the caller's job: 2.0 lands at 1.0 — the desert
        // corner IS the (1.0, 0.0) cell: i = 0, j = 255 -> idx 255*256.
        assert_eq!(
            get(clamp01(2.0), clamp01(0.0), &g, GRASS_DEFAULT),
            get(1.0, 0.0, &g, GRASS_DEFAULT)
        );
        assert_eq!(get(1.0, 0.0, &g, GRASS_DEFAULT), 255 * 256);
    }

    #[test]
    fn plains_climate_resolves_the_baked_constants() {
        // THE chain proof — cache-present boots (the local suite, the
        // goldens pipeline, the CI gui-pixel job) pin the REAL jar values:
        // the atlas bake (TINT_GRASS/TINT_FOLIAGE) was generated from the
        // vanilla plains constants; the decoded grids MUST reproduce them
        // at the plains climate or the no-op rule would be fiction. The
        // no-cache boot the honest None arms speak (the baked-plains look
        // the callers keep).
        if !grids().real {
            assert_eq!(grass_from_climate(0.8, 0.4), None);
            assert_eq!(foliage_from_climate(0.8, 0.4), None);
            return;
        }
        assert_eq!(grass_from_climate(0.8, 0.4), Some(0x91BD59));
        assert_eq!(foliage_from_climate(0.8, 0.4), Some(0x77AB2F));
    }

    #[test]
    fn known_biome_climates_sample_the_known_constants() {
        // Vanilla-canonical colors, verified against the jar grids before
        // pinning (the probe script): desert grass #BFB755, jungle grass
        // #59C93C, snowy plains grass #80B497. Cache-present boots only.
        if !grids().real {
            return;
        }
        assert_eq!(grass_from_climate(2.0, 0.0), Some(0xBFB755), "desert");
        assert_eq!(grass_from_climate(0.95, 0.9), Some(0x59C93C), "jungle");
        assert_eq!(grass_from_climate(0.0, 0.5), Some(0x80B497), "snowy plains");
        assert_eq!(
            foliage_from_climate(0.95, 0.9),
            Some(0x30BB0B),
            "jungle foliage"
        );
        // The clamping is the caller's job: the desert sample IS the
        // (1.0, 0.0) corner of the REAL grid.
        let g = grids();
        assert_eq!(
            get(clamp01(2.0), clamp01(0.0), &g.grass, GRASS_DEFAULT),
            get(1.0, 0.0, &g.grass, GRASS_DEFAULT)
        );
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
    fn the_loaded_grids_report_the_boot_shape() {
        // The loader's own contract: a cache-present boot holds the three
        // full grids (the old 65536-entry law) and the lookups opine; an
        // absent/broken cache the honest empty + real=false keeps — the
        // lookups then report no opinion instead of black.
        let g = grids();
        if g.real {
            assert_eq!(g.grass.len(), 65536);
            assert_eq!(g.foliage.len(), 65536);
            assert_eq!(g.dry_foliage.len(), 65536);
            assert!(grass_from_climate(0.8, 0.4).is_some());
            assert!(foliage_from_climate(0.8, 0.4).is_some());
        } else {
            assert!(g.grass.is_empty() && g.foliage.is_empty() && g.dry_foliage.is_empty());
            assert_eq!(grass_from_climate(0.8, 0.4), None);
            assert_eq!(foliage_from_climate(0.8, 0.4), None);
            assert_eq!(dry_foliage_from_climate(0.8, 0.4), None);
        }
    }

    #[test]
    fn dry_foliage_climates_sample_the_probed_constants() {
        // Probe-verified against the real jar grid BEFORE any client code
        // (scripts/probe_dry_foliage.py, the same
        // ColorMapColorUtil.get arithmetic): the dry grid's palette is a
        // narrow autumn band (all samples in the 0xA3xx4x range).
        // Cache-present boots only — the no-cache boot the shape test's
        // None arms speak.
        let g = grids();
        if !g.real {
            return;
        }
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
            get(0.5, 1.0, &g.dry_foliage, DRY_FOLIAGE_DEFAULT),
            g.dry_foliage[127 * 256 + 127]
        );
    }
}

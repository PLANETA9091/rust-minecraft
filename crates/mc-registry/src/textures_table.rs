//! THE TEXTURE TABLE'S OWN LOADER (the generated/ -> runtime-load migration's
//! sixth and last lane — the 10MB decoded texel body the compiled source no
//! longer carries): every concrete texture id the models table references,
//! decoded to RAW RGBA8 by the generator's jar walk (the model-domain census,
//! the first-frame animation crop, the special renderers' pre-tinted entity
//! textures), the runtime cache keeps the result as the generator's own
//! binary emit (`assets-cache/gen/textures.bin` — the second generator-DERIVED
//! cache file beside font.bin: not a jar byte, the decode law the jar walk
//! produces), THIS loader the only parser. THE FORMAT (the generator's
//! `emit_cache` and the bridge that proved the migration speak the same law):
//! the `MC2RSTEX1` magic, the record count (the compiled `N_TEXTURES` the
//! drift-detection pins — a stale cache from another version refuses), then
//! the records in the compiled `TEXTURE_IDS` order (12 bytes + texels each:
//! width u16, height u16, texel len u32 — `w*h*4` the shape law demands, 0
//! for the 65 missing materials — then the RGBA8 bytes). The absent/broken
//! file the honest empty table keeps — the vanilla checkerboard the missing
//! materials' own law answers (the same degraded mode the no-cache boots
//! already speak for the font pixels and the colormap grids; the WORLD atlas
//! is a different, compiled lane — the terrain keeps its real tiles). The
//! OnceLock the load-once law keeps — the 'static borrows the font/colormap
//! loaders' own shape answers.

/// The record count must equal the compiled namespace (the stale-cache
/// refusal: a bin from another version would misalign every id).
use crate::generated::textures::N_TEXTURES;

#[derive(Debug, Clone, PartialEq)]
pub struct TextureTable {
    /// The concatenated RGBA8 texels, row-major, unpremultiplied alpha.
    pub texels: Vec<u8>,
    /// (start, end) byte span per id — the compiled OFFSETS law rebuilt.
    pub spans: Vec<(u32, u32)>,
    pub widths: Vec<u16>,
    pub heights: Vec<u16>,
    /// false when the cache is absent/broken — the honest empty table
    /// (the vanilla checkerboard every consumer's missing arm keeps).
    pub real: bool,
}

impl TextureTable {
    /// The honest empty table (the no-cache boot's degraded mode).
    fn empty() -> Self {
        Self {
            texels: Vec::new(),
            spans: Vec::new(),
            widths: Vec::new(),
            heights: Vec::new(),
            real: false,
        }
    }
}

/// THE PARSE LAW (the generator's emit_cache walk verbatim, mirrored):
/// the magic, the count (the caller's expectation — production pins the
/// compiled `N_TEXTURES`), the records in order, each span exactly
/// `w*h*4` (`0` iff the missing `w==h==0`), the record region exactly
/// consumed. Any violation the honest None keeps — the boot then walks
/// the next cache root (or the empty fallback answers).
fn parse_table(bytes: &[u8], expected_n: usize) -> Option<TextureTable> {
    if bytes.len() < 13 || &bytes[..9] != b"MC2RSTEX1" {
        return None;
    }
    if u32::from_le_bytes(bytes[9..13].try_into().unwrap()) as usize != expected_n {
        return None; // the stale-cache refusal
    }
    let mut o = 13usize;
    let mut widths = Vec::with_capacity(expected_n);
    let mut heights = Vec::with_capacity(expected_n);
    let mut spans = Vec::with_capacity(expected_n);
    let mut texels = Vec::new();
    let mut acc: u32 = 0;
    for i in 0..expected_n {
        let head_end = o + 8;
        if head_end > bytes.len() {
            return None; // the truncated head bail
        }
        let w = u16::from_le_bytes(bytes[o..o + 2].try_into().unwrap());
        let h = u16::from_le_bytes(bytes[o + 2..o + 4].try_into().unwrap());
        let len = u32::from_le_bytes(bytes[o + 4..o + 8].try_into().unwrap()) as usize;
        o = head_end;
        if len != w as usize * h as usize * 4 {
            return None; // the shape law
        }
        if (w == 0) != (len == 0) {
            return None; // the missing-material law (w==h==0 iff empty)
        }
        let end = o + len;
        if end > bytes.len() {
            return None; // the truncated texels bail
        }
        texels.extend_from_slice(&bytes[o..end]);
        o = end;
        widths.push(w);
        heights.push(h);
        spans.push((acc, acc + len as u32));
        acc += len as u32;
        let _ = i;
    }
    if o != bytes.len() {
        return None; // the record region exactly consumed
    }
    Some(TextureTable {
        texels,
        spans,
        widths,
        heights,
        real: true,
    })
}

fn load_table() -> TextureTable {
    // The roots the font/colormap loaders' own walk speaks, the gen cache
    // home the generator's emit walk keeps (assets-cache/gen/ — the DERIVED
    // files' lane beside the jar-verbatim tree).
    for prefix in ["assets-cache/gen", "../assets-cache/gen", "../../assets-cache/gen"] {
        if let Ok(bytes) = std::fs::read(std::path::Path::new(prefix).join("textures.bin")) {
            if let Some(t) = parse_table(&bytes, N_TEXTURES) {
                return t;
            }
        }
    }
    TextureTable::empty()
}

fn table() -> &'static TextureTable {
    static TABLE: std::sync::OnceLock<TextureTable> = std::sync::OnceLock::new();
    TABLE.get_or_init(load_table)
}

/// True when the runtime cache carried the real decoded table (the
/// boot-shape flag the value tests gate on — the no-cache CI boot
/// walks the honest-empty arms).
pub fn real() -> bool {
    table().real
}

/// The id's RGBA8 texels (empty for a missing material — or for every
/// id at the honest no-cache boot).
pub fn texture_rgba(idx: u16) -> &'static [u8] {
    let t = table();
    if !t.real {
        return &[];
    }
    let (s, e) = t.spans[idx as usize];
    &t.texels[s as usize..e as usize]
}

/// The id's pixel dims (0, 0) for a missing material.
pub fn texture_size(idx: u16) -> (u16, u16) {
    let t = table();
    if !t.real {
        return (0, 0);
    }
    (t.widths[idx as usize], t.heights[idx as usize])
}

/// True when the id has no PNG in the jar (the vanilla checkerboard) —
/// and the whole table's degraded mode when the cache is absent.
pub fn is_missing(idx: u16) -> bool {
    let t = table();
    if !t.real {
        return true;
    }
    t.widths[idx as usize] == 0
}

/// Kept for the compiled accessor's shape: true exactly when the real
/// decoded table booted (the old `N_TEXTURES > 0` law the pixels now
/// answer at runtime).
pub fn has_textures() -> bool {
    real()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The synthetic encoder the parse law pins (the generator's
    /// emit_cache walk verbatim).
    fn encode(records: &[(u16, u16, &[u8])], n: u32) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"MC2RSTEX1");
        out.extend_from_slice(&n.to_le_bytes());
        for (w, h, rgba) in records {
            out.extend_from_slice(&w.to_le_bytes());
            out.extend_from_slice(&h.to_le_bytes());
            out.extend_from_slice(&(rgba.len() as u32).to_le_bytes());
            out.extend_from_slice(rgba);
        }
        out
    }

    #[test]
    fn the_parse_law_keeps_the_binary_walk() {
        let two = &[(1u16, 1u16, &[1u8, 2, 3, 4][..]), (0, 0, &[][..])];
        // the happy path: two records, the second the missing material
        let t = parse_table(&encode(two, 2), 2).expect("the happy walk parses");
        assert!(t.real);
        assert_eq!(t.widths, vec![1, 0]);
        assert_eq!(t.heights, vec![1, 0]);
        assert_eq!(t.spans, vec![(0, 4), (4, 4)]);
        assert_eq!(t.texels, vec![1, 2, 3, 4]);

        // the magic bail
        let mut bad = encode(two, 2);
        bad[0] = b'X';
        assert!(parse_table(&bad, 2).is_none(), "wrong magic refuses");

        // the count expectation bail (the stale-cache refusal)
        assert!(parse_table(&encode(two, 2), 3).is_none(), "wrong count refuses");

        // the truncated-head bail
        for cut in [13, 17, 21] {
            assert!(
                parse_table(&encode(two, 2)[..cut], 2).is_none(),
                "cut at {cut} refuses"
            );
        }

        // the shape-law bail: len != w*h*4
        let mut bad = encode(two, 2);
        bad[17] = 9; // the first record's len byte (4 -> 9, impossible for 1x1)
        assert!(parse_table(&bad, 2).is_none(), "len/shape disagreement refuses");

        // the missing-material law: nonzero dims with an empty span
        let bad = &[(16u16, 16u16, &[1u8, 2, 3, 4][..]), (16, 0, &[][..])];
        assert!(parse_table(&encode(bad, 2), 2).is_none(), "16x0 with no texels refuses");

        // the trailing-garbage bail (the record region exactly consumed)
        let mut bad = encode(two, 2);
        bad.push(0);
        assert!(parse_table(&bad, 2).is_none(), "trailing byte refuses");
    }

    #[test]
    fn the_loaded_table_reports_the_boot_shape() {
        // The both-arms contract: the cache-present boot carries the real
        // census (2172 records, the texel total the bridge/generator pin);
        // the no-cache boot the honest empties (the checkerboard mode the
        // callers' missing arms answer).
        if !real() {
            assert!(texture_rgba(0).is_empty());
            assert_eq!(texture_size(0), (0, 0));
            assert!(is_missing(0));
            assert!(!has_textures());
            return;
        }
        assert!(real());
        assert!(has_textures());
        assert_eq!(table().spans.len(), 2172);
        assert_eq!(table().widths.len(), 2172);
        assert_eq!(table().texels.len(), 3_345_408, "the census texel total");
        let present = (0..2172).filter(|i| !is_missing(*i)).count();
        assert_eq!(present, 2107, "the census present count");
    }

    #[test]
    fn the_missing_materials_stay_missing_the_cache_way() {
        if !real() {
            return;
        }
        // missingno is a namespace MEMBER pinned missing: an empty span the
        // consumers' checkerboard arm answers
        let air = crate::generated::textures::texture_index("missingno").expect("the literal ref");
        assert!(is_missing(air));
        assert_eq!(texture_size(air), (0, 0));
        assert!(texture_rgba(air).is_empty());
        // and the first present id spans real pixels
        let stone =
            crate::generated::textures::texture_index("block/stone").expect("stone in the table");
        assert_eq!(texture_size(stone), (16, 16));
        assert_eq!(texture_rgba(stone).len(), 16 * 16 * 4);
        assert!(!is_missing(stone));
    }
}

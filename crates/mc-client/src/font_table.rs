//! THE FONT TABLE'S OWN LOADER (the generated/ -> runtime-load migration's
//! fifth lane — the 224KB glyph table the compiled source no longer
//! carries): the glyph metrics the generator MEASURES from the real font
//! textures (the provider chain walk — the vanilla trim rule), the runtime
//! cache keeps the result as the generator's own binary emit
//! (`assets-cache/gen/font.bin` — the first generator-DERIVED cache file:
//! not a jar byte, the measured law the jar walk produces), the client's
//! loader the only parser. THE FORMAT (the generator's `emit_cache` and
//! the bridge that proved the migration speak the same law): the
//! `MC2RSFONT1` magic, the texture list (the short paths the renderer's
//! cache walk joins — `font/ascii.png` under the runtime root), the
//! skipped-provider count, then the glyph records sorted by codepoint
//! (18 bytes each: ch u32, tex u16, u u16, v u16, gw u16, gh u16,
//! ascent u16, space_advance i16 — the sorted order the binary_search
//! lookup walks). The absent/broken file the honest empty table keeps —
//! the every call site's Option arm answers (the fallback pen advance the
//! renderer already speaks), the same degraded mode the TILE_NONE atlas
//! fallback and the absent font-texture pixels drive at a no-cache boot
//! (the glyphs existed but the pixels did not — the text was already
//! invisible there; the empty table changes nothing visually). The
//! OnceLock the load-once law keeps — the 'static borrows the sound/lang
//! loaders' own shape answers.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlyphDef {
    pub ch: u32,
    pub tex: u16,
    pub u: u16,
    pub v: u16,
    pub gw: u16,
    pub gh: u16,
    pub ascent: u16,
    pub space_advance: i16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontTable {
    pub textures: Vec<String>,
    pub glyphs: Vec<GlyphDef>,
    pub skipped_providers: usize,
    /// false when the cache is absent/broken — the honest empty table
    /// (the Option arms the renderer keeps answer everywhere).
    pub real: bool,
}

/// THE PARSE LAW (the generator's emit_cache walk verbatim, mirrored):
/// the magic, the texture list, the skipped count, the glyph records in
/// STRICTLY INCREASING codepoint order (the binary_search's own
/// precondition), the record region exactly consumed. Any violation the
/// honest None keeps — the boot then walks the next cache root (or the
/// empty fallback answers).
fn parse_font(bytes: &[u8]) -> Option<FontTable> {
    if bytes.len() < 10 || &bytes[..10] != b"MC2RSFONT1" {
        return None;
    }
    let mut o = 10usize;
    let u32_at = |o: &mut usize| -> Option<u32> {
        let end = *o + 4;
        if end > bytes.len() {
            return None;
        }
        let v = u32::from_le_bytes(bytes[*o..end].try_into().unwrap());
        *o = end;
        Some(v)
    };
    let texture_count = u32_at(&mut o)? as usize;
    let mut textures = Vec::with_capacity(texture_count.min(64));
    for _ in 0..texture_count {
        let len = u32_at(&mut o)? as usize;
        let end = o + len;
        if end > bytes.len() {
            return None;
        }
        textures.push(String::from_utf8(bytes[o..end].to_vec()).ok()?);
        o = end;
    }
    let skipped_providers = u32_at(&mut o)? as usize;
    let glyph_count = u32_at(&mut o)? as usize;
    let end = o + glyph_count * 18;
    if end > bytes.len() {
        return None;
    }
    let mut glyphs = Vec::with_capacity(glyph_count);
    let mut prev_ch = 0u32;
    for rec in bytes[o..end].chunks_exact(18) {
        let ch = u32::from_le_bytes(rec[0..4].try_into().unwrap());
        if ch <= prev_ch && !glyphs.is_empty() {
            return None; // the sorted order the lookup law requires
        }
        prev_ch = ch;
        glyphs.push(GlyphDef {
            ch,
            tex: u16::from_le_bytes(rec[4..6].try_into().unwrap()),
            u: u16::from_le_bytes(rec[6..8].try_into().unwrap()),
            v: u16::from_le_bytes(rec[8..10].try_into().unwrap()),
            gw: u16::from_le_bytes(rec[10..12].try_into().unwrap()),
            gh: u16::from_le_bytes(rec[12..14].try_into().unwrap()),
            ascent: u16::from_le_bytes(rec[14..16].try_into().unwrap()),
            space_advance: i16::from_le_bytes(rec[16..18].try_into().unwrap()),
        });
    }
    if end != bytes.len() {
        return None; // the record region exactly consumed
    }
    Some(FontTable {
        textures,
        glyphs,
        skipped_providers,
        real: true,
    })
}

fn load_table() -> FontTable {
    // The roots the sound/colormap loaders' own walk speaks, the gen
    // cache home the generator's emit walk keeps (assets-cache/gen/ —
    // the DERIVED files' lane beside the jar-verbatim tree).
    for prefix in ["assets-cache/gen", "../assets-cache/gen", "../../assets-cache/gen"] {
        if let Ok(bytes) = std::fs::read(std::path::Path::new(prefix).join("font.bin")) {
            if let Some(t) = parse_font(&bytes) {
                return t;
            }
        }
    }
    FontTable {
        textures: Vec::new(),
        glyphs: Vec::new(),
        skipped_providers: 0,
        real: false,
    }
}

/// THE LOADED TABLE (the OnceLock the load-once law keeps — the 'static
/// borrows the static's own lifetime answers).
fn table() -> &'static FontTable {
    static TABLE: std::sync::OnceLock<FontTable> = std::sync::OnceLock::new();
    TABLE.get_or_init(load_table)
}

/// Binary search the glyph table (sorted by codepoint — the parse law
/// enforces it, the old generated glyph() the same walk spoke).
pub fn glyph(ch: char) -> Option<&'static GlyphDef> {
    table()
        .glyphs
        .binary_search_by_key(&(ch as u32), |g| g.ch)
        .ok()
        .map(|i| &table().glyphs[i])
}

/// THE UNBOLDED ADVANCE (javap `GlyphInfo.getAdvance(false)` the port's
/// integer law speaks: the space the provider's explicit advance keeps,
/// the bitmap glyph the width+1 wear — the same law the draw/width
/// lanes already walk).
pub fn glyph_advance(d: &GlyphDef) -> i32 {
    if d.space_advance != 0 {
        d.space_advance as i32
    } else {
        d.gw as i32 + 1
    }
}

/// THE MISSING GLYPH'S OWN ADVANCE (javap `FontSet$1.info` =
/// `SpecialGlyphs.MISSING` — the 5x8 image the `getWidth() + 1 = 6`
/// advance wears (the raw walk: `getWidth(); iconst_1; iadd; i2f`),
/// the `createGlyph` the null render keeps — the INVISIBLE missing
/// glyph the 26.2 clinit banks). The obfuscated walk's width class
/// when the ORIGINAL codepoint the table misses: vanilla still picks
/// a random glyph from the missing glyph's own class.
pub const MISSING_GLYPH_ADVANCE: i32 = 6;

/// THE WIDTH-CLASS MAP (javap `lambda$selectProviders$0` — every
/// codepoint the active providers answer joins `glyphsByWidth[Mth.ceil
/// (info.getAdvance(false))]`, the list the bake order keeps; the
/// port's codepoint-sorted table the same membership wears — integer
/// advances make ceil the identity). THE OBFUSCATED WALK'S OWN POOL:
/// the random pick the class list draws.
fn glyphs_by_width_map() -> std::collections::HashMap<i32, Vec<u32>> {
    let mut map: std::collections::HashMap<i32, Vec<u32>> = std::collections::HashMap::new();
    for d in &table().glyphs {
        map.entry(glyph_advance(d)).or_default().push(d.ch);
    }
    map
}

/// The loaded map (the OnceLock the load-once law keeps — the font
/// table's own shape the static borrow answers).
pub fn glyphs_by_width() -> &'static std::collections::HashMap<i32, Vec<u32>> {
    static MAP: std::sync::OnceLock<std::collections::HashMap<i32, Vec<u32>>> =
        std::sync::OnceLock::new();
    MAP.get_or_init(glyphs_by_width_map)
}

/// THE OBFUSCATED PICK (javap `FontSet.getRandomGlyph(RandomSource, I)`
/// — the class list the `random.nextInt(loc3.size())` draw walks, the
/// `getInt` the codepoint answers, the `getGlyph(cp).nonFishy().get()`
/// re-resolution the pick wears; the EMPTY class the missing glyph
/// returns — the port's None the degraded empty-table boot keeps; the
/// nonFishy filter the default font's plain bitmap set the honest
/// no-op keeps — every glyph's bold advance the +1 offset wears, no
/// fishy advance the bake sees). `roll` the uniform draw — the caller
/// folds (seed, position) into it, the pure derivation the shadow
/// pass's SAME glyphs answer (the vanilla preparation's ONE pick the
/// shadow copy renders).
pub fn obfuscated_pick(class: i32, roll: u64) -> Option<&'static GlyphDef> {
    let list = glyphs_by_width().get(&class)?;
    let cp = list[(roll % list.len() as u64) as usize];
    glyph(char::from_u32(cp)?)
}

/// The referenced font textures, in provider order (the short paths the
/// renderer's cache walk joins — `font/ascii.png` under the runtime root).
pub fn font_textures() -> &'static [String] {
    &table().textures
}

/// Providers skipped by the generator (ttf/legacy, unifont is empty in
/// current versions).
#[cfg_attr(not(test), expect(dead_code))]
pub fn skipped_providers() -> usize {
    table().skipped_providers
}

/// True when the real measured table loaded (the test gates the jar-value
/// pins on it — the no-cache CI boot the honest arms walk).
#[cfg_attr(not(test), expect(dead_code))]
pub fn real() -> bool {
    table().real
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The format law's own encoder (the generator's emit_cache mirrored —
    /// two glyphs, one texture, the exact byte layout).
    fn encode(textures: &[&str], skipped: usize, glyphs: &[(u32, u16, u16, u16, u16, u16, u16, i16)]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"MC2RSFONT1");
        out.extend_from_slice(&(textures.len() as u32).to_le_bytes());
        for t in textures {
            out.extend_from_slice(&(t.len() as u32).to_le_bytes());
            out.extend_from_slice(t.as_bytes());
        }
        out.extend_from_slice(&(skipped as u32).to_le_bytes());
        out.extend_from_slice(&(glyphs.len() as u32).to_le_bytes());
        for (ch, tex, u, v, gw, gh, asc, sp) in glyphs {
            out.extend_from_slice(&ch.to_le_bytes());
            out.extend_from_slice(&tex.to_le_bytes());
            out.extend_from_slice(&u.to_le_bytes());
            out.extend_from_slice(&v.to_le_bytes());
            out.extend_from_slice(&gw.to_le_bytes());
            out.extend_from_slice(&gh.to_le_bytes());
            out.extend_from_slice(&asc.to_le_bytes());
            out.extend_from_slice(&sp.to_le_bytes());
        }
        out
    }

    #[test]
    fn the_parse_law_keeps_the_binary_walk() {
        // THE happy path: two glyphs (sorted), one texture, the fields
        // round-trip exactly.
        let bytes = encode(
            &["font/ascii.png"],
            2,
            &[(65, 2, 8, 32, 5, 8, 7, 0), (66, 2, 16, 32, 5, 8, 7, 0)],
        );
        let t = parse_font(&bytes).unwrap();
        assert!(t.real);
        assert_eq!(t.textures, vec!["font/ascii.png".to_string()]);
        assert_eq!(t.skipped_providers, 2);
        assert_eq!(t.glyphs.len(), 2);
        assert_eq!(
            t.glyphs[0],
            GlyphDef { ch: 65, tex: 2, u: 8, v: 32, gw: 5, gh: 8, ascent: 7, space_advance: 0 }
        );

        // THE MAGIC LAW: wrong magic the None keeps.
        let mut bad = bytes.clone();
        bad[0] = b'X';
        assert_eq!(parse_font(&bad), None);

        // THE LENGTH LAW: a truncated record region the None keeps.
        assert_eq!(parse_font(&bytes[..bytes.len() - 9]), None);

        // THE EXACT-CONSUMPTION LAW: trailing garbage the None keeps.
        let mut extra = bytes.clone();
        extra.push(0xAB);
        assert_eq!(parse_font(&extra), None);

        // THE SORTED LAW: out-of-order codepoints the None keeps (the
        // binary_search's own precondition — the parse enforces it).
        let unsorted = encode(&[], 0, &[(66, 2, 0, 0, 5, 8, 7, 0), (65, 2, 8, 32, 5, 8, 7, 0)]);
        assert_eq!(parse_font(&unsorted), None);

        // THE BYTES LAW: broken bytes the None keeps, never a panic.
        assert_eq!(parse_font(b"not the font table"), None);
    }

    #[test]
    fn the_lookup_walks_the_sorted_table() {
        // The lookup law is about SEARCH, not the jar's values — the
        // synthetic table pins it cache-free (the CI no-cache boot runs
        // it too). parse_font's sorted Vec + binary_search_by_key: a hit
        // returns the exact record, a miss the None.
        let bytes = encode(
            &[],
            0,
            &[(65, 1, 0, 0, 5, 8, 7, 0), (90, 1, 40, 0, 5, 8, 7, 0)],
        );
        let t = parse_font(&bytes).unwrap();
        let idx = |ch: u32| t.glyphs.binary_search_by_key(&ch, |g| g.ch).ok();
        assert_eq!(idx(65), Some(0));
        assert_eq!(idx(90), Some(1));
        assert_eq!(idx(66), None);
        // i16 sign round-trips (the space provider's negative advances).
        let neg = encode(&[], 0, &[(32, 65535, 0, 0, 0, 0, 0, -1)]);
        assert_eq!(parse_font(&neg).unwrap().glyphs[0].space_advance, -1);
    }

    #[test]
    fn the_loaded_table_reports_the_boot_shape() {
        // The loader's own contract: a cache-present boot holds the full
        // measured table; an absent/broken cache the honest empty +
        // real=false keeps — the renderer's Option arms then walk the
        // fallback pen advance (the same degraded mode the absent font
        // pixels already drove).
        if real() {
            // the skipped count the loader reports (usize — the
            // u32 wire value the section header read)
            assert!(skipped_providers() <= u32::MAX as usize);
            assert!(!font_textures().is_empty());
            assert!(glyph('A').is_some());
        } else {
            assert!(glyph('A').is_none() && glyph(' ').is_none());
            assert!(font_textures().is_empty());
        }
    }

    #[test]
    fn the_measured_metrics_pin_the_jar_values() {
        // Cache-present boots only (the probe pins from the OLD compiled
        // table, the bridge's own printout): the space's sentinel tex +
        // advance, the ascii '!'/A/j cells, the nonlatin omega row.
        if !real() {
            return;
        }
        let space = glyph(' ').unwrap();
        assert_eq!((space.tex, space.space_advance), (65535, 4), "space");
        let bang = glyph('!').unwrap();
        assert_eq!(
            (bang.tex, bang.u, bang.v, bang.gw, bang.gh, bang.ascent),
            (2, 8, 16, 1, 8, 7),
            "'!'"
        );
        let a = glyph('A').unwrap();
        assert_eq!(
            (a.tex, a.u, a.v, a.gw, a.gh, a.ascent),
            (2, 8, 32, 5, 8, 7),
            "'A'"
        );
        let j = glyph('j').unwrap();
        assert_eq!(
            (j.tex, j.u, j.v, j.gw, j.gh, j.ascent),
            (2, 80, 48, 5, 8, 7),
            "'j' (the descender the plane test walks)"
        );
        let omega = glyph('ω').unwrap();
        assert_eq!(
            (omega.tex, omega.u, omega.v, omega.gw, omega.gh),
            (0, 112, 24, 7, 8),
            "'ω' (the nonlatin_european provider's row, codepoint 969)"
        );
        assert_eq!(skipped_providers(), 0, "ttf/legacy providers the 26.2 chain skips none of");
    }

    /// THE WIDTH-CLASS MAP'S OWN VERDICTS (javap `lambda$selectProviders$0`
    /// + `getRandomGlyph` — the membership the ceil-advance class keeps,
    /// the pick the uniform draw answers, the determinism the (class,
    /// roll) purity speaks). Cache-present boots only (the empty-table
    /// boot the None arm walks — the degraded mode the draw's fallback
    /// answers).
    #[test]
    fn the_width_classes_the_obfuscated_pool_keeps() {
        if !real() {
            return;
        }
        // THE ADVANCE LAW (the same walk the draw lane speaks — the
        // space the explicit 4 keeps, the bitmap the width+1 wears)
        assert_eq!(glyph_advance(glyph(' ').unwrap()), 4, "space");
        assert_eq!(glyph_advance(glyph('A').unwrap()), 6, "'A' (gw 5)");
        // THE MEMBERSHIP LAW (every codepoint the table carries its
        // class list holds, the list the codepoint order keeps)
        let map = glyphs_by_width();
        let six = map.get(&6).unwrap();
        assert!(!six.is_empty());
        assert!(six.contains(&(65)), "'A' the class-6 pool joins");
        assert!(map.get(&4).unwrap().contains(&(32)), "space the class-4 pool joins");
        // THE CLASS PINS THE ADVANCE (integer advances: every member of
        // class c the advance c wears — the ceil the identity walks)
        for d in &table().glyphs {
            assert_eq!(
                map.get(&glyph_advance(d)).unwrap().contains(&d.ch),
                true,
                "cp {} its own class {} joins",
                d.ch,
                glyph_advance(d)
            );
        }
        // THE MISSING-GLYPH CLASS (SpecialGlyphs.MISSING's 6 — the pool
        // the table's own class-6 glyphs answer, the pick never starves)
        assert!(
            map.contains_key(&MISSING_GLYPH_ADVANCE),
            "the missing glyph's class the real table keeps"
        );
        // THE PICK'S OWN LAWS (the determinism the same roll speaks, the
        // coverage the rolls' walk answers, the member the class keeps)
        for class in [4, 6, 7] {
            let Some(first) = obfuscated_pick(class, 0) else {
                continue; // the class the real table may honestly skip
            };
            assert_eq!(
                obfuscated_pick(class, 0).map(glyph_advance),
                Some(glyph_advance(first)),
                "the same roll the same pick wears"
            );
            assert_eq!(
                glyph_advance(first),
                class,
                "the pick the class's own advance wears"
            );
            let mut distinct = std::collections::HashSet::new();
            for roll in 0..256u64 {
                if let Some(d) = obfuscated_pick(class, roll) {
                    distinct.insert(d.ch);
                }
            }
            assert!(distinct.len() >= 2, "class {class} the rolls cover: {distinct:?}");
        }
        // THE EMPTY CLASS (the honest None the degraded boot keeps)
        assert_eq!(obfuscated_pick(9999, 0), None, "the starved class the None keeps");
    }
}

//! THE PROFILER PIE CHART'S OWN DATA (javap 26.2 `FilledProfileResults`
//! .getTimes / `ResultField` / `ProfilerPieChart.splitNodeName` — the
//! audited walk the F3+1 chord's own chart the client's real frame
//! stages feed).
//!
//! THE LAWS THE BYTECODE SPEAKS:
//! - `ResultField.getColor()` — the raw disasm's own chain:
//!   `(name.hashCode() & 11184810) + (-12303292)` — the 0xAAAAAA mask
//!   keeps two bits per byte channel, the 0xFF444444 add lays the
//!   opaque alpha + the 0x44 base the pastel palette wears.
//! - `ProfileResults.demanglePath` — `replace('\u{1e}', '.')`: the
//!   char-30 record separator the tree paths join with the display
//!   dot answers.
//! - `FilledProfileResults.getTimes(path)` — the verbatim walk: the
//!   prefix the non-empty path the `path + '\u{1e}'` suffix keeps,
//!   the direct-child gate `isDirectChild` (longer, prefixed, no
//!   further separator), the duration denominator the
//!   `max(sum-of-children, entry-duration)` cap the two lcmp pairs
//!   keep, the relative child names the `cand.substring(prefix)`
//!   slice speaks (the clean segment the leading separator never
//!   wears), the `unspecified` slice the cap bump alone adds (the
//!   time the listed children never cover, the f32 subtract the
//!   bytecode's own l2f/fsub/f2d chain keeps), the descending sort
//!   the `compareTo` law speaks (percentage desc, ties name desc),
//!   the self entry the `add(0, ResultField(path, 100, 100, count))`
//!   the chart's `removeFirst()` drops.
//! - `splitNodeName(name, firstWidth, restWidth)` — the verbatim
//!   wrap: the `\\.` split, the continuation pieces the leading dot
//!   carry, the accumulate-while-fits walk against the first-line
//!   width the empty list answers (the rest widths the later lines
//!   keep), the too-wide flush that adds the CURRENT line and
//!   resets (the piece the jar's own law drops when the current is
//!   non-empty — the bytecode's own shape the honest copy wears),
//!   the empty-current too-wide piece the line alone answers, the
//!   tail flush the non-empty current adds.
//!
//! THE DATA THE CLIENT'S OWN FRAME FEEDS: the window loop's real
//! stages (input / tick / render) the `root` children name — the
//! vanilla client's own profiler the same concept serves, the pie
//! the previous frame's completed walk draws.

/// THE TREE PATH'S OWN SEPARATOR (the char-30 record separator the
/// `getTimes` prefix and the drill-down concat both speak — the
/// `demanglePath` replace's own source the display never wears).
pub const PATH_SEP: char = '\u{1e}';

/// THE MASK THE HASH COLOR WALK KEEPS (`ldc 11184810` = 0xAAAAAA —
/// the two high bits of each byte channel the pastel palette owns).
const COLOR_MASK: i32 = 0x00AAAAAA;

/// THE BASE THE HASH COLOR WALK ADDS (`ldc -12303292` = 0xFF444444 —
/// the opaque alpha + the 0x44 floor each channel wears).
const COLOR_BASE: i32 = -12303292;

/// THE FULL-CIRCLE TURN THE SLICE ANGLES WALK (the
/// `ldc2_w 6.2831854820251465` the fan lambda's own constant keeps —
/// the f32-rounded 2π the trig folds wear).
pub const TWO_PI: f32 = 6.2831854820251465;

/// THE PIE'S OWN RADIUS PAIR (the fan lambda's `ldc 105.0` the x
/// radius keeps and the `ldc 0.5` y-squash the ellipse wears —
/// 105.0 x 52.5 the half-size pair the GUI-space pie answers).
pub const PIE_RX: f32 = 105.0;
pub const PIE_RY: f32 = 105.0 * 0.5;

/// THE SIDE EXTRUSION'S OWN DEPTH (the side lambda's `+10.0` the
/// lower-rim bands wear — the coin edge the darkened color paints).
pub const PIE_SIDE_DEPTH: f32 = 10.0;

/// ONE PROFILE RESULT ROW (the `ResultField(String,double,double,long)`
/// ctor the walk carries — the name the relative segment keeps, the
/// percentage the parent-share speaks, the global percentage the
/// whole-profile share, the count the entry's own invocation tally).
#[derive(Debug, Clone, PartialEq)]
pub struct ResultField {
    pub name: String,
    pub percentage: f64,
    pub global_percentage: f64,
    pub count: u64,
}

/// THE JAVA STRING HASH THE COLOR WALK READS (`String.hashCode`'s
/// own `s[0]*31^(n-1) + ...` chain — the i32 wrapping the JVM law
/// keeps, the Rust default hash the word never wears).
pub fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for ch in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(ch as i32);
    }
    h
}

/// THE ROW'S OWN COLOR (`ResultField.getColor` — the mask + base
/// chain the raw disasm keeps, the i32 the ARGB decoders speak).
pub fn result_field_color(name: &str) -> i32 {
    (java_string_hash(name) & COLOR_MASK).wrapping_add(COLOR_BASE)
}

/// THE DISPLAY NAME THE PATH WEARS (`ProfileResults.demanglePath` —
/// the char-30 separator the dot replaces, the rest verbatim).
pub fn demangle_path(s: &str) -> String {
    s.replace(PATH_SEP, ".")
}

/// THE DIRECT-CHILD GATE (`FilledProfileResults.isDirectChild` — the
/// three arms the bytecode keeps: the candidate the prefix longer
/// wears, the prefix the startsWith answers, the no-further
/// separator the `indexOf(char30, prefix.len()+1) < 0` law keeps).
pub fn is_direct_child(prefix: &str, candidate: &str) -> bool {
    if candidate.len() <= prefix.len() || !candidate.starts_with(prefix) {
        return false;
    }
    // THE FURTHER-SEPARATOR LAW (the `indexOf(char30, prefix.len()+1)
    // < 0` walk — the search region the byte offset opens, the ASCII
    // paths the profiler names the boundary keeps)
    let from = prefix.len() + 1;
    match candidate.get(from..) {
        Some(rest) => !rest.contains(PATH_SEP),
        None => true,
    }
}

/// THE FROZEN TICK'S OWN RESULTS (the `FilledProfileResults` entries
/// map the one completed walk keeps — the path the key, the
/// nanosecond duration and the invocation count the values wear).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProfilerResults {
    pub entries: Vec<(String, u64, u64)>,
}

impl ProfilerResults {
    pub fn new(entries: Vec<(String, u64, u64)>) -> Self {
        Self { entries }
    }

    /// THE ENTRY LOOKUP (`getEntry` — the missing path the zero pair
    /// answers, the duration + count the tuple carries).
    fn entry(&self, path: &str) -> (u64, u64) {
        self.entries
            .iter()
            .find(|(p, _, _)| p == path)
            .map(|(_, d, c)| (*d, *c))
            .unwrap_or((0, 0))
    }

    /// THE CHILDREN'S OWN ROWS (`getTimes` — the verbatim walk the
    /// module doc speaks; the self row the head insert keeps).
    pub fn get_times(&self, path: &str) -> Vec<ResultField> {
        let (base_dur, _) = self.entry(path);
        let (entry_dur, entry_count) = self.entry(path);
        // THE PREFIX (the non-empty path the separator suffix keeps —
        // the `invokedynamic '\u0001\u001e'` concat the bytecode 54-60
        // speaks; the empty path the suffix never gains)
        let prefix = if path.is_empty() {
            path.to_string()
        } else {
            format!("{}{}", path, PATH_SEP)
        };
        // THE CHILDREN'S SUM (the first keySet walk)
        let mut acc: u64 = 0;
        for (p, d, _) in &self.entries {
            if is_direct_child(&prefix, p) {
                acc += d;
            }
        }
        let acc_pre = acc;
        // THE TWO CAPS (the lcmp pairs — the sum the entry duration
        // answers when it under-covers, the base the same walk keeps)
        if (acc as i64) < entry_dur as i64 {
            acc = entry_dur;
        }
        let base = if base_dur < acc { acc } else { base_dur };
        let mut out: Vec<ResultField> = Vec::new();
        for (p, d, c) in &self.entries {
            if !is_direct_child(&prefix, p) {
                continue;
            }
            let d = *d;
            // THE TWO PERCENTAGES (the `* 100.0 / denom` ddiv pairs —
            // the relative and the global the same denominator speaks)
            let percentage = d as f64 * 100.0 / acc as f64;
            let global = d as f64 * 100.0 / base as f64;
            out.push(ResultField {
                // THE RELATIVE NAME (the `cand.substring(prefixWithSep)`
                // slice — the clean segment the leading separator never
                // wears; the suffixed prefix the length answers)
                name: p
                    .get(prefix.len()..)
                    .unwrap_or("")
                    .to_string(),
                percentage,
                global_percentage: global,
                count: *c,
            });
        }
        // THE UNSPECIFIED SLICE (the cap bump the `fcmpl` gate keeps —
        // the f32 subtract the bytecode's own l2f/fsub/f2d chain wears,
        // the uncovered time the listed children never name)
        if (acc as f32) > acc_pre as f32 {
            let diff = ((acc as f32) - (acc_pre as f32)) as f64;
            out.push(ResultField {
                name: "unspecified".to_string(),
                percentage: diff * 100.0 / acc as f64,
                global_percentage: diff * 100.0 / base as f64,
                count: entry_count,
            });
        }
        // THE SORT (the `compareTo` law — the percentage desc the dcmp
        // arms keep, the ties the name desc the String.compareTo's own
        // reversed receiver wears)
        out.sort_by(|a, b| {
            b.percentage
                .partial_cmp(&a.percentage)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.name.cmp(&a.name))
        });
        // THE SELF ROW (the `add(0, ResultField(path, 100, 100, count))`
        // the chart's own `removeFirst()` drops — the header's word)
        out.insert(
            0,
            ResultField {
                name: path.to_string(),
                percentage: 100.0,
                global_percentage: 100.0,
                count: entry_count,
            },
        );
        out
    }
}

/// THE HEADER/LEGEND'S OWN WRAP (`ProfilerPieChart.splitNodeName` —
/// the verbatim walk the module doc speaks; the `width` closure the
/// font's own measure answers).
pub fn split_node_name(
    width: impl Fn(&str) -> i32,
    name: &str,
    first_width: i32,
    rest_width: i32,
) -> Vec<String> {
    // THE `\\.` SPLIT (the regex the ldc string keeps — the literal
    // dot the demangled paths join with)
    let parts: Vec<&str> = name.split('.').collect();
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for (i, part) in parts.iter().enumerate() {
        // THE PIECE (the first index the bare part answers, the later
        // indices the leading dot carry)
        let piece = if i == 0 {
            (*part).to_string()
        } else {
            format!(".{}", part)
        };
        let candidate = format!("{}{}", current, piece);
        let w = width(&candidate);
        let limit = if lines.is_empty() {
            first_width
        } else {
            rest_width
        };
        if w <= limit {
            // THE ACCUMULATE (the candidate fits — the current the
            // merge keeps)
            current = candidate;
        } else if current.is_empty() {
            // THE EMPTY FLUSH (the piece itself the line answers —
            // the too-long single part the bytecode 110-120 keeps)
            lines.push(piece);
        } else {
            // THE NON-EMPTY FLUSH (the current line the list gains,
            // the reset the piece's own drop wears — the jar's own
            // law the honest copy preserves)
            lines.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_java_hash_the_color_walk_speaks() {
        // THE HASH (the String.hashCode chain — "" the zero word,
        // "hello" the 99162322 the JDK's own answer keeps)
        assert_eq!(java_string_hash(""), 0);
        assert_eq!(java_string_hash("hello"), 99162322);
        // THE COLOR (the mask + base chain — the hash & 0xAAAAAA the
        // two-bit-per-channel palette keeps, the 0xFF444444 base the
        // opaque floor wears)
        let c = result_field_color("unspecified");
        assert_eq!(
            c,
            ((java_string_hash("unspecified") & 0x00AAAAAA) as i32).wrapping_add(-12303292)
        );
        // the alpha the base carries (the opaque word every row wears)
        assert_eq!((c as u32 >> 24) & 0xFF, 0xFF);
    }

    #[test]
    fn the_demangle_the_separator_dots() {
        assert_eq!(demangle_path("root"), "root");
        assert_eq!(
            demangle_path("root\u{1e}gameRenderer"),
            "root.gameRenderer"
        );
    }

    #[test]
    fn the_get_times_the_audited_walk_speaks() {
        let res = ProfilerResults::new(vec![
            ("root".to_string(), 1000, 1),
            ("root\u{1e}a".to_string(), 600, 1),
            ("root\u{1e}b".to_string(), 300, 1),
            ("root\u{1e}a\u{1e}deep".to_string(), 100, 2),
        ]);
        let rows = res.get_times("root");
        // THE SELF HEAD (the path the name keeps, the 100/100 pair)
        assert_eq!(rows[0].name, "root");
        assert_eq!(rows[0].percentage, 100.0);
        assert_eq!(rows[0].global_percentage, 100.0);
        // THE CLEAN RELATIVE NAMES (the substring the separator never
        // carries; the grandchild the direct-child gate excludes)
        let names: Vec<&str> = rows[1..].iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["a", "b", "unspecified"]);
        // THE PERCENTAGES (the 600/1000 and 300/1000 words, the
        // unspecified the cap bump speaks — 100 of 1000)
        assert_eq!(rows[1].percentage, 60.0);
        assert_eq!(rows[2].percentage, 30.0);
        assert_eq!(rows[3].percentage, 10.0);
        // THE COUNTS (the rows' own entries the walk carries)
        assert_eq!(rows[3].count, 1);
        // THE SORT (the desc percentage the compareTo law keeps)
        assert!(rows[1].percentage >= rows[2].percentage);
        assert!(rows[2].percentage >= rows[3].percentage);
    }

    #[test]
    fn the_cap_the_children_over_cover() {
        let res = ProfilerResults::new(vec![
            ("root".to_string(), 100, 1),
            ("root\u{1e}a".to_string(), 80, 1),
            ("root\u{1e}b".to_string(), 80, 1),
        ]);
        let rows = res.get_times("root");
        // THE SUM CAP (the 160 the entry's 100 never under-covers —
        // the unspecified slice the no-bump walk never adds); THE
        // TIE (the equal percentages the name-desc answer keeps —
        // the compareTo's reversed receiver the walk banks)
        let names: Vec<&str> = rows[1..].iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["b", "a"]);
        assert_eq!(rows[1].percentage, 50.0);
        assert_eq!(rows[2].percentage, 50.0);
    }

    #[test]
    fn the_sort_the_tie_the_name_desc() {
        let res = ProfilerResults::new(vec![
            ("root".to_string(), 200, 1),
            ("root\u{1e}zz".to_string(), 100, 1),
            ("root\u{1e}aa".to_string(), 100, 1),
        ]);
        let rows = res.get_times("root");
        // THE TIE (the equal percentages the name-desc answer keeps —
        // the compareTo's reversed receiver the walk banks)
        assert_eq!(rows[1].name, "zz");
        assert_eq!(rows[2].name, "aa");
    }

    #[test]
    fn the_drill_the_path_the_walk_keeps() {
        let res = ProfilerResults::new(vec![
            ("root".to_string(), 1000, 1),
            ("root\u{1e}gameRenderer".to_string(), 700, 1),
        ]);
        let rows = res.get_times("root\u{1e}gameRenderer");
        // THE LEAF LEVEL (the prefixed path the grandchild lookups
        // answer — the self row the full path keeps, the childless
        // entry the cap-bump walk the 100% unspecified slice wears —
        // the verbatim law the bytecode's own fcmpl gate keeps)
        assert_eq!(rows[0].name, "root\u{1e}gameRenderer");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1].name, "unspecified");
        assert_eq!(rows[1].percentage, 100.0);
    }

    #[test]
    fn the_split_the_wrap_the_audited_law_speaks() {
        let w = |s: &str| s.len() as i32;
        // THE FIT (the single line the wide first width keeps)
        assert_eq!(split_node_name(w, "root.gameRenderer", 100, 90), vec![
            "root.gameRenderer"
        ]);
        // THE FLUSH (the current line the list gains, the reset the
        // piece's own drop wears — the jar's own law the copy keeps)
        assert_eq!(split_node_name(w, "root.gameRenderer", 5, 5), vec!["root"]);
        // THE EMPTY FLUSH (the piece itself the line answers — the
        // too-long first part), the tail the continuation accumulates
        assert_eq!(
            split_node_name(w, "abcdefgh.ij", 3, 3),
            vec!["abcdefgh", ".ij"]
        );
    }
}

//! The synth module — the 26.2 `PerlinSimplexNoise` + `SimplexNoise`
//! port, the noise trio the `Biome` temperature laws sample.
//!
//! THE AUDIT (the classfile_dump + raw_disasm + cp walks — the
//! no-javap container's tools keep, the module docs bank):
//!
//! - `SimplexNoise.<init>(RandomSource)`: `xo`, `yo`, `zo` are ONE
//!   `nextDouble() * 256.0` each (three draws, in that order), then the
//!   Fisher-Yates shuffle — `for j in 0..256: k = nextInt(256 - j)`,
//!   `l = p[j]; p[j] = p[j + k]; p[j + k] = l` — the bound SHRINKS each
//!   iteration (256-j), and THIS version's ctor keeps the plain swap:
//!   NO mirror write (`p[j + 128]` the older noise ctors wear) — the
//!   26.2 bytecode's four array stores are the whole body.
//! - `nextInt(bound)` (the `BitRandomSource` default the shuffle draws
//!   through): `bound <= 0` throws; a power of two rides the
//!   `(bound * next(31)) >>> 31` lane, everything else the rejection
//!   loop `do { r = next(31); val = r % bound; } while (r - val +
//!   (bound - 1) < 0)` — the java.util.Random law. `consumeCount(n)` =
//!   `n * nextInt()` = n * next(32) — the RandomSource default's own
//!   walk (NOT next(1) — the bytecode says nextInt()).
//! - `SimplexNoise.getValue(DD)`: the classic 2D simplex walk —
//!   `s = (x + y) * F2`, the skewed lattice floors, `t = (i + j) * G2`,
//!   the corner offsets, `d4 > d5` picks the diagonal (the dcmpl law),
//!   `x1/y1 = d4 - k + G2` pairs, `x2/y2 = d4 - 1 + 2*G2`; the gradient
//!   indices `p(i1 + p(j1)) % 12`, `p((i1 + k) + p(j1 + l)) % 12`,
//!   `p((i1 + 1) + p(j1 + 1)) % 12` — the INNER `p` reads the J side
//!   (the 26.2 bytecode's operand order, the word the older perm walks
//!   swap); each corner `getCornerNoise3D(gi, dx, dy, 0.0, 0.5)`; the
//!   result `70.0 * (n0 + n1 + n2)`.
//! - `getCornerNoise3D`: `t = maxDistSquared - x*x - y*y - z*z`; `t <
//!   0` (the dcmpg lane) returns 0.0, else `t*t` squared again — the
//!   `t^4 * dot(GRADIENT[gi], x, y, z)` falloff; `dot` the i2d-widened
//!   `g[0]*x + g[1]*y + g[2]*z`.
//! - `GRADIENT`: SIXTEEN int[3] rows the `<clinit>` banks — the classic
//!   twelve simplex gradients plus the four repeats (`1,1,0`, `0,-1,1`,
//!   `-1,1,0`, `0,-1,-1`); the `% 12` never reaches rows 12..15 but the
//!   table the jar allocates is the honest shape. `F2 = 0.5 * (sqrt(3)
//!   - 1)`, `G2 = (3 - sqrt(3)) / 6` — the Math.sqrt(3.0) word.
//! - `PerlinSimplexNoise.<init>(RandomSource, IntSortedSet)`: `first =
//!   -min`, `last = max`, `total = first + last + 1` (the `>= 1` guard
//!   the two IllegalArgumentExceptions guard); the FIRST SimplexNoise
//!   draws from the stream unconditionally; the slot walk: `last >= 0
//!   && last < total && octaves.contains(0)` seats the first noise at
//!   index `last` (the contains(0) the iconst_0 banks), then `for j in
//!   (last+1)..total`: `j >= 0 && octaves.contains(last - j)` seats a
//!   fresh SimplexNoise from the SAME stream, else
//!   `random.consumeCount(262)` (the sipush 262 word); the `last > 0`
//!   branch (the WorldgenRandom over a LegacyRandomSource seeded from
//!   the first noise's `getValue(xo, yo, zo) * 2^63` walk) NEVER RIDES
//!   for the three Biome statics — every one carries `lastInt == 0` —
//!   the port leaves the slots that branch would fill None and the
//!   main stream's discipline untouched (the documented skip, never a
//!   guess); `highestFreqInputFactor = 2^last`,
//!   `highestFreqValueFactor = 1 / (2^total - 1)` (the Math.pow pair —
//!   the value factor's denominator is the TOTAL, the iload loc5 the
//!   bytecode keeps).
//! - `PerlinSimplexNoise.getValue(DDZ)`: the octave walk — `acc +=
//!   level.getValue(x * freq + (flag ? level.xo : 0.0), y * freq +
//!   (flag ? level.yo : 0.0)) * amp` — and the freq halves / amp
//!   doubles for EVERY level INCLUDING the null ones (the ifnull skips
//!   only the accumulation, the dstore pair rides on); the flag is the
//!   `false` constant every Biome call site passes (the iconst_0) —
//!   the xo/yo offsets the port carries but the callers never ride.
//! - THE STATICS (the `Biome.<clinit>` walk, the cp constants the
//!   seeds ride): `TEMPERATURE_NOISE` = seed 1234, octaves `{0}`
//!   (cp #446); `FROZEN_TEMPERATURE_NOISE` = seed 3456, octaves
//!   `{-2, -1, 0}` (cp #467 — three SimplexNoise constructions from
//!   ONE stream, all three octave slots seat, no consumeCount); 
//!   `BIOME_INFO_NOISE` = seed 2345, octaves `{0}` (cp #475). The
//!   `WorldgenRandom(LegacyRandomSource(seed))` chain the clinit wraps
//!   draws NOTHING at construction — the stream starts at the seed
//!   scramble, the same LCG [`VanillaRandom`] carries. The octave list
//!   rides through an `IntRBTreeSet` (the List ctor wraps) — the port
//!   sorts + dedups.
//! - The reference vectors the tests pin come from the python twin
//!   (`scripts/psn_twin.py`) — the bit-exact f64 walk the jar's own
//!   arithmetic keeps (the f32 lanes round through struct pack).
use std::sync::OnceLock;

/// The `LegacyRandomSource` LCG twin — the java.util.Random law the
/// noise construction (and the weather columns) draw through. Moved
/// here from the weather module (the synth module the jar's own ctor
/// argument chain keeps); the fields stay private — `new(seed)` the
/// scramble word.
#[derive(Debug, Clone)]
pub struct VanillaRandom {
    state: u64,
    /// The polar method's cached second draw; `None` = none cached.
    gaussian_cache: Option<f64>,
}

impl VanillaRandom {
    /// The seeded ctor — the scramble `(seed ^ 0x5DEECE66D) &
    /// ((1<<48)-1)`, the gaussian cache empty.
    pub fn new(seed: i64) -> VanillaRandom {
        let mut r = VanillaRandom {
            state: 0,
            gaussian_cache: None,
        };
        r.set_seed(seed);
        r
    }

    /// `setSeed`: the scramble `(seed ^ 0x5DEECE66D) & ((1<<48)-1)` —
    /// the int hash sign-extends into the long first (the jar's `i2l`),
    /// the gaussian cache resets.
    pub fn set_seed(&mut self, seed: i64) {
        self.state = ((seed as u64) ^ 0x5DEECE66D) & ((1u64 << 48) - 1);
        self.gaussian_cache = None;
    }

    /// `next(bits)`: the LCG step `seed * 0x5DEECE66D + 0xB` masked to
    /// 48 bits, then the top `bits` of the new state.
    fn next(&mut self, bits: u32) -> i32 {
        self.state = self
            .state
            .wrapping_mul(0x5DEECE66D)
            .wrapping_add(0xB)
            & ((1u64 << 48) - 1);
        (self.state >> (48 - bits)) as i32
    }

    /// `nextInt(bound)` — the `BitRandomSource` default's law: a power
    /// of two rides `(bound * next(31)) >>> 31`, everything else the
    /// rejection loop `r - r % bound + (bound - 1) < 0` (the iand gate
    /// the bytecode keeps).
    pub fn next_int(&mut self, bound: i32) -> i32 {
        assert!(bound > 0, "Bound must be positive");
        if (bound & -bound) == bound {
            ((bound as i64 * self.next(31) as i64) >> 31) as i32
        } else {
            loop {
                let r = self.next(31);
                let val = r % bound;
                if r - val + (bound - 1) >= 0 {
                    return val;
                }
            }
        }
    }

    /// `consumeCount(n)` — the RandomSource default's walk: `n *
    /// nextInt()`, each a full `next(32)` (the invokeinterface nextInt
    /// the bytecode keeps).
    pub fn consume_count(&mut self, count: i32) {
        for _ in 0..count {
            self.next(32);
        }
    }

    /// `nextFloat = next(24) / 2^24` — the [0, 1) band.
    pub fn next_f32(&mut self) -> f32 {
        self.next(24) as f32 / (1u32 << 24) as f32
    }

    /// `nextDouble`: the two-draw law — `((next(26) << 27) + next(27))
    /// * 2^-53`. The 26-bit draw widens to i64 BEFORE the shift — the
    /// 53-bit sum the int overflow would truncate.
    pub fn next_f64(&mut self) -> f64 {
        let hi = self.next(26) as i64;
        let lo = self.next(27) as i64;
        (((hi << 27) + lo) as f64) / (1u64 << 53) as f64
    }

    /// `nextGaussian` — the `MarsagliaPolarGaussian` polar walk the
    /// jar's gaussianSource carries: two doubles mapped to [-1, 1), the
    /// `s >= 1 || s == 0` reject, the `sqrt(-2 * ln(s) / s)` fold, the
    /// second value cached for the next call.
    pub fn next_gaussian(&mut self) -> f64 {
        if let Some(cached) = self.gaussian_cache.take() {
            return cached;
        }
        let (mut v1, mut v2, mut s);
        loop {
            v1 = 2.0 * self.next_f64() - 1.0;
            v2 = 2.0 * self.next_f64() - 1.0;
            s = v1 * v1 + v2 * v2;
            if s < 1.0 && s != 0.0 {
                break;
            }
        }
        let mult = (-2.0 * s.ln() / s).sqrt();
        self.gaussian_cache = Some(v2 * mult);
        v1 * mult
    }
}

/// `Math.sqrt(3.0)` — the f64 nearest (the test pins the literal
/// against the runtime sqrt).
const SQRT_3: f64 = 1.7320508075688772;
/// `F2 = 0.5 * (sqrt(3) - 1)` — the `<clinit>` fold.
const F2: f64 = 0.5 * (SQRT_3 - 1.0);
/// `G2 = (3 - sqrt(3)) / 6` — the `<clinit>` fold.
const G2: f64 = (3.0 - SQRT_3) / 6.0;

/// The sixteen gradient rows the `<clinit>` banks — the classic twelve
/// simplex gradients, then the four repeats (`1,1,0`, `0,-1,1`,
/// `-1,1,0`, `0,-1,-1`) the jar's table keeps.
const GRADIENT: [[i32; 3]; 16] = [
    [1, 1, 0],
    [-1, 1, 0],
    [1, -1, 0],
    [-1, -1, 0],
    [1, 0, 1],
    [-1, 0, 1],
    [1, 0, -1],
    [-1, 0, -1],
    [0, 1, 1],
    [0, -1, 1],
    [0, 1, -1],
    [0, -1, -1],
    [1, 1, 0],
    [0, -1, 1],
    [-1, 1, 0],
    [0, -1, -1],
];

/// The 26.2 `SimplexNoise` — the permutation table, the three origin
/// offsets, the 2D walk. The `p` array is the jar's own 512 length;
/// only `[0..256)` is ever written and the `p(I)` mask (`& 255`) never
/// reads past 255 (the doc banks the honest shape).
pub struct SimplexNoise {
    /// The origin offsets — `nextDouble() * 256.0` each, the three
    /// draws the ctor order keeps. `zo` is drawn for the stream's
    /// discipline's sake (the jar's own ctor consumes the draw) and
    /// read only by the 3-arg getValue walk the three Biome statics
    /// never ride — the dead-code word the honest shape keeps.
    pub xo: f64,
    pub yo: f64,
    #[allow(dead_code)]
    pub zo: f64,
    /// The Fisher-Yates shuffled table — `[i32; 512]` the jar
    /// allocates, `[0..256)` the shuffle writes.
    p: Box<[i32; 512]>,
}

impl SimplexNoise {
    /// The ctor walk: three `nextDouble() * 256.0` draws, then the
    /// shuffle `for j in 0..256: k = nextInt(256 - j)` with the plain
    /// swap (NO mirror write — this version's word).
    pub fn new(random: &mut VanillaRandom) -> SimplexNoise {
        let xo = random.next_f64() * 256.0;
        let yo = random.next_f64() * 256.0;
        let zo = random.next_f64() * 256.0;
        let mut p = Box::new([0i32; 512]);
        for (i, slot) in p.iter_mut().enumerate().take(256) {
            *slot = i as i32;
        }
        for j in 0..256i32 {
            let k = random.next_int(256 - j);
            let l = p[j as usize];
            p[j as usize] = p[(j + k) as usize];
            p[(j + k) as usize] = l;
        }
        SimplexNoise { xo, yo, zo, p }
    }

    /// `p(I)I` — `self.p[i & 255]`, the mask inside the helper.
    fn p(&self, i: i32) -> i32 {
        self.p[(i & 255) as usize]
    }

    /// `dot([IDDD)D` — the i2d-widened `g[0]*x + g[1]*y + g[2]*z`.
    fn dot(g: &[i32; 3], x: f64, y: f64, z: f64) -> f64 {
        g[0] as f64 * x + g[1] as f64 * y + g[2] as f64 * z
    }

    /// `getCornerNoise3D(IDDDD)D` — `t = maxDistSquared - x*x - y*y -
    /// z*z`; `t < 0` (the dcmpg lane) returns 0.0, else the `t^4 *
    /// dot(GRADIENT[gi], x, y, z)` falloff (the `t*t` squared again the
    /// double dmul pair keeps).
    fn get_corner_noise_3d(
        &self,
        gradient_index: usize,
        x: f64,
        y: f64,
        z: f64,
        max_dist_squared: f64,
    ) -> f64 {
        let mut t = max_dist_squared - x * x - y * y - z * z;
        if t < 0.0 {
            return 0.0;
        }
        t *= t;
        t * t * Self::dot(&GRADIENT[gradient_index], x, y, z)
    }

    /// `getValue(DD)D` — the classic 2D simplex walk (see the module
    /// docs for the law's own words).
    pub fn get_value(&self, x: f64, y: f64) -> f64 {
        let s = (x + y) * F2;
        let i = (x + s).floor() as i32;
        let j = (y + s).floor() as i32;
        let t = (i + j) as f64 * G2;
        let x0 = i as f64 - t;
        let y0 = j as f64 - t;
        let d4 = x - x0;
        let d5 = y - y0;
        // the dcmpl law: `d4 > d5` picks the diagonal — else the other
        let (k, l): (i32, i32) = if d4 > d5 { (1, 0) } else { (0, 1) };
        let x1 = d4 - k as f64 + G2;
        let y1 = d5 - l as f64 + G2;
        let x2 = d4 - 1.0 + 2.0 * G2;
        let y2 = d5 - 1.0 + 2.0 * G2;
        let i1 = i & 255;
        let j1 = j & 255;
        // the INNER p reads the J side — the 26.2 bytecode's operand
        // order (each index masks inside the p helper)
        let gi0 = (self.p(i1 + self.p(j1)) % 12) as usize;
        let gi1 = (self.p((i1 + k) + self.p(j1 + l)) % 12) as usize;
        let gi2 = (self.p((i1 + 1) + self.p(j1 + 1)) % 12) as usize;
        let n0 = self.get_corner_noise_3d(gi0, d4, d5, 0.0, 0.5);
        let n1 = self.get_corner_noise_3d(gi1, x1, y1, 0.0, 0.5);
        let n2 = self.get_corner_noise_3d(gi2, x2, y2, 0.0, 0.5);
        70.0 * (n0 + n1 + n2)
    }
}

/// The 26.2 `PerlinSimplexNoise` — the octave stack the biome
/// temperature laws sample.
pub struct PerlinSimplexNoise {
    /// The octave slots — the None holes the contains-gates (and the
    /// never-ridden `last > 0` branch the module docs bank) keep.
    levels: Vec<Option<SimplexNoise>>,
    /// `2^last` — the highest octave's input scale.
    pub input_factor: f64,
    /// `1 / (2^total - 1)` — the accumulator's base amplitude.
    pub value_factor: f64,
}

impl PerlinSimplexNoise {
    /// The ctor walk — the module docs carry the law's own words. The
    /// octave list rides through the set semantics (sort + dedup — the
    /// `IntRBTreeSet` the List ctor wraps).
    pub fn new(random: &mut VanillaRandom, octaves: &[i32]) -> PerlinSimplexNoise {
        let mut oct = octaves.to_vec();
        oct.sort_unstable();
        oct.dedup();
        assert!(!oct.is_empty(), "Need some octaves!");
        let first = -oct[0];
        let last = *oct.last().expect("non-empty above");
        let total = first + last + 1;
        assert!(
            total >= 1,
            "Total number of octaves needs to be >= 1"
        );
        let simplex = SimplexNoise::new(random);
        let mut levels: Vec<Option<SimplexNoise>> = (0..total).map(|_| None).collect();
        // the seat walk: the FIRST noise rides only the contains(0)
        // gate at index `last`
        let l = last;
        if l >= 0 && l < total && oct.contains(&0) {
            levels[l as usize] = Some(simplex);
        }
        // the upper slots: a fresh SimplexNoise from the SAME stream
        // per present octave, else the consumeCount(262) walk
        for j in (l + 1)..total {
            if j >= 0 && oct.contains(&(l - j)) {
                levels[j as usize] = Some(SimplexNoise::new(random));
            } else {
                random.consume_count(262);
            }
        }
        // the `last > 0` branch (the WorldgenRandom over the
        // LegacyRandomSource seeded from the first noise's 3-arg walk)
        // never rides for the three Biome statics — every one carries
        // lastInt == 0; the slots it would fill stay None (the
        // documented skip, the main stream's discipline untouched)
        let input_factor = 2.0f64.powi(last);
        let value_factor = 1.0 / (2.0f64.powi(total) - 1.0);
        PerlinSimplexNoise {
            levels,
            input_factor,
            value_factor,
        }
    }

    /// `getValue(DDZ)D` — the octave walk; the freq halves / amp
    /// doubles for EVERY level INCLUDING the null ones (the ifnull
    /// skips only the accumulation).
    pub fn get_value(&self, x: f64, y: f64, use_offset: bool) -> f64 {
        let mut acc = 0.0f64;
        let mut freq = self.input_factor;
        let mut amp = self.value_factor;
        for level in &self.levels {
            if let Some(level) = level {
                let ox = if use_offset { level.xo } else { 0.0 };
                let oy = if use_offset { level.yo } else { 0.0 };
                acc += level.get_value(x * freq + ox, y * freq + oy) * amp;
            }
            freq /= 2.0;
            amp *= 2.0;
        }
        acc
    }
}

// ---- THE STATICS (the `Biome.<clinit>` walk, the cp seeds) ----

static TEMPERATURE_NOISE: OnceLock<PerlinSimplexNoise> = OnceLock::new();
static FROZEN_TEMPERATURE_NOISE: OnceLock<PerlinSimplexNoise> = OnceLock::new();
static BIOME_INFO_NOISE: OnceLock<PerlinSimplexNoise> = OnceLock::new();

/// `Biome.TEMPERATURE_NOISE` — seed 1234 (cp #446), octaves `{0}`.
pub fn temperature_noise() -> &'static PerlinSimplexNoise {
    TEMPERATURE_NOISE.get_or_init(|| PerlinSimplexNoise::new(&mut VanillaRandom::new(1234), &[0]))
}

/// `Biome.FROZEN_TEMPERATURE_NOISE` — seed 3456 (cp #467), octaves
/// `{-2, -1, 0}`: three SimplexNoise constructions from ONE stream,
/// all three octave slots seat, no consumeCount.
pub fn frozen_temperature_noise() -> &'static PerlinSimplexNoise {
    FROZEN_TEMPERATURE_NOISE
        .get_or_init(|| PerlinSimplexNoise::new(&mut VanillaRandom::new(3456), &[-2, -1, 0]))
}

/// `Biome.BIOME_INFO_NOISE` — seed 2345 (cp #475), octaves `{0}`.
pub fn biome_info_noise() -> &'static PerlinSimplexNoise {
    BIOME_INFO_NOISE.get_or_init(|| PerlinSimplexNoise::new(&mut VanillaRandom::new(2345), &[0]))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `<clinit>` folds: the literal `Math.sqrt(3.0)` word the
    /// constants ride on, the F2/G2 folds.
    #[test]
    fn the_sqrt3_literal_the_runtime_sqrt_keeps() {
        assert_eq!(SQRT_3, 3.0f64.sqrt());
        assert_eq!(F2, 0.5 * (3.0f64.sqrt() - 1.0));
        assert_eq!(G2, (3.0 - 3.0f64.sqrt()) / 6.0);
    }

    /// The GRADIENT table the `<clinit>` banks — the classic twelve,
    /// then the four repeats.
    #[test]
    fn the_gradient_table_the_sixteen_rows_keep() {
        assert_eq!(GRADIENT[0], [1, 1, 0]);
        assert_eq!(GRADIENT[3], [-1, -1, 0]);
        assert_eq!(GRADIENT[7], [-1, 0, -1]);
        assert_eq!(GRADIENT[11], [0, -1, -1]);
        // the repeats the rows 12..15 wear (the % 12 never reaches)
        assert_eq!(GRADIENT[12], GRADIENT[0]);
        assert_eq!(GRADIENT[13], GRADIENT[9]);
        assert_eq!(GRADIENT[14], GRADIENT[1]);
        assert_eq!(GRADIENT[15], GRADIENT[11]);
    }

    /// The shuffle: the seed-1234 stream the python twin's words pin —
    /// the three 256-scaled doubles, then the Fisher-Yates table (the
    /// plain swap, no mirror).
    #[test]
    fn the_shuffle_the_seed_words_speak() {
        let mut r = VanillaRandom::new(1234);
        let s = SimplexNoise::new(&mut r);
        assert_eq!(s.xo, 165.52503303447696);
        assert_eq!(s.yo, 243.54757399536433);
        assert_eq!(s.zo, 219.54264571054935);
        let want = [117, 50, 97, 189, 20, 161, 53, 254];
        for (i, w) in want.iter().enumerate() {
            assert_eq!(s.p(i as i32), *w, "p[{i}] drifts");
        }
        assert_eq!(s.p(254), 133);
        assert_eq!(s.p(255), 118);
    }

    /// `nextInt(bound)` — the BitRandomSource law the twin's successive
    /// draws pin: the power-of-two lane (256), the always-0 bound (1),
    /// the rejection lane (255, 7, 1000, 10).
    #[test]
    fn the_next_int_the_bitrandomsource_law_keeps() {
        let mut r = VanillaRandom::new(42);
        assert_eq!(r.next_int(256), 186);
        assert_eq!(r.next_int(1), 0, "bound 1 rides the shift lane to 0");
        assert_eq!(r.next_int(255), 153);
        assert_eq!(r.next_int(7), 3);
        assert_eq!(r.next_int(1000), 970);
        assert_eq!(r.next_int(10), 5);
    }

    #[test]
    #[should_panic(expected = "Bound must be positive")]
    fn the_next_int_the_zero_bound_throws() {
        let mut r = VanillaRandom::new(1);
        r.next_int(0);
    }

    /// `consumeCount(n)` = n * nextInt() — each a full next(32) (the
    /// RandomSource default the bytecode keeps). Seed 11, 262 consumes,
    /// then the next double — the twin's word.
    #[test]
    fn the_consume_count_the_full_next32_walk_keeps() {
        let mut r = VanillaRandom::new(11);
        r.consume_count(262);
        assert_eq!(r.next_f64(), 0.4696099267903897);
    }

    /// The 2D simplex walk the python twin's vectors pin — the three
    /// statics' seeds, the bit-exact f64 words.
    #[test]
    fn the_simplex_get_value_the_vectors_speak() {
        let tn = temperature_noise();
        assert_eq!(tn.get_value(0.0, 0.0, false), 0.0);
        assert_eq!(tn.get_value(0.125, 0.25, false), -0.8569647822162618);
        assert_eq!(tn.get_value(-0.375, 0.625, false), 0.47298002196987476);
        assert_eq!(tn.get_value(12.5, -7.25, false), 0.31056928987772464);
        assert_eq!(tn.get_value(0.125, 0.375, false), -0.48530470604666714);
        assert_eq!(tn.get_value(-100.125, 40.875, false), 0.39191912106674154);
        // the frozen trio's three-octave stack (seeds 3456, {-2,-1,0})
        let ftn = frozen_temperature_noise();
        assert_eq!(ftn.get_value(0.0, 0.0, false), 0.0);
        assert_eq!(ftn.get_value(0.05, 0.1, false), 0.11916988377626987);
        assert_eq!(ftn.get_value(-0.35, 0.7, false), 0.1103684569471595);
        assert_eq!(ftn.get_value(3.3, -1.4, false), 0.2960171278999197);
        // the biome-info static (seed 2345)
        let bin = biome_info_noise();
        assert_eq!(bin.get_value(0.0, 0.0, false), 0.0);
        assert_eq!(bin.get_value(0.2, 0.4, false), -0.0554800619850868);
        assert_eq!(bin.get_value(0.09, -0.27, false), 0.11476733972703412);
        assert_eq!(bin.get_value(1.7, 2.9, false), -0.06847348735148333);
    }

    /// The octave factors the Math.pow pair keeps: the single-octave
    /// statics ride 2^0 / 1/(2^1 - 1); the frozen stack 1/7.
    #[test]
    fn the_octave_factors_the_pow_pair_keeps() {
        assert_eq!(temperature_noise().input_factor, 1.0);
        assert_eq!(temperature_noise().value_factor, 1.0);
        assert_eq!(frozen_temperature_noise().input_factor, 1.0);
        assert_eq!(frozen_temperature_noise().value_factor, 1.0 / 7.0);
        assert_eq!(biome_info_noise().value_factor, 1.0);
    }

    /// The seat walk with holes: octaves `[0, 2]` — total 3, the first
    /// noise seats at index 2 (the contains(0) gate), the loop below it
    /// never runs, the hole slots 0/1 STILL halve the frequency and
    /// double the amplitude (the ifnull skips only the accumulation).
    /// The value the twin's word pins bit-exactly.
    #[test]
    fn the_octave_holes_the_freq_amp_walk_still_rides() {
        let mut r = VanillaRandom::new(31337);
        let p = PerlinSimplexNoise::new(&mut r, &[0, 2]);
        assert_eq!(p.input_factor, 4.0);
        assert_eq!(p.value_factor, 1.0 / 7.0);
        assert_eq!(p.get_value(0.7, -1.3, false), 0.15928111106650752);
        // the stream consumed exactly ONE SimplexNoise (259 draws —
        // no consumeCount, the seat loop never ran): the next draw
        // matches a manual replay
        let mut r2 = VanillaRandom::new(31337);
        let _s = SimplexNoise::new(&mut r2);
        assert_eq!(r.next_f64(), r2.next_f64());
    }

    /// The stream discipline the ctor keeps: the single-octave ctor
    /// consumes exactly the first SimplexNoise's draws (3 doubles +
    /// 256 shuffles) — the next draw matches the manual replay.
    #[test]
    fn the_ctor_stream_the_exact_draws_keep() {
        let mut r1 = VanillaRandom::new(7);
        let _p = PerlinSimplexNoise::new(&mut r1, &[0]);
        let draw_after = r1.next_f64();
        let mut r2 = VanillaRandom::new(7);
        let _s = SimplexNoise::new(&mut r2);
        let expected = r2.next_f64();
        assert_eq!(draw_after, expected);
    }
}

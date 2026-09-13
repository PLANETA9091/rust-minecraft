//! The 26.2 display-transform link — the models table's `display` stanzas
//! become the per-context view-space pose over the quads. Pure data, zero
//! dependencies, zero render-path contact (the client consumer is a later
//! round; goldens are untouched by construction).
//!
//! Every rule is javap evidence from the 26.2 client jar (39.2 MB,
//! `.mc2rs/classpath/client.jar`) and the bundled joml-1.10.8.jar:
//!
//! - `ItemTransform$Deserializer` (net.minecraft.client.resources.model.cuboid):
//!   rotation = exactly 3 floats, RAW (no clamp); translation = exactly 3
//!   floats, `mul(0.0625f)` FIRST then `Mth.clamp(-5, 5)` (MAX_TRANSLATION);
//!   scale = exactly 3 floats, `Mth.clamp(-4, 4)` (MAX_SCALE); missing keys
//!   default to (0,0,0)/(0,0,0)/(1,1,1). Gson numbers are parsed as doubles
//!   and cast, so the module parses f64 and casts to f32.
//! - `ItemTransforms$Deserializer`: reads the nine `ItemDisplayContext`
//!   serial names — thirdperson_righthand, thirdperson_lefthand,
//!   firstperson_righthand, firstperson_lefthand, head, gui, ground, fixed,
//!   ON_SHELF (the container's ninth field is Mojang-named
//!   `fixedFromBottom` but receives ON_SHELF; the enum has exactly NONE +
//!   those nine). NO legacy `thirdperson`/`firstperson` keys. Absent
//!   context = NO_TRANSFORM; an absent left-hand context is replaced by the
//!   right-hand transform AT PARSE TIME (`if (left == NO_TRANSFORM) left =
//!   right`), per stanza.
//! - `ResolvedModel.findTopTransform(s)`: 26.2 resolves PER CONTEXT — walk
//!   the parent chain; at each model with its own display stanza (which may
//!   be null), `getTransform(ctx)`; identity-compare against NO_TRANSFORM —
//!   a miss keeps walking; exhausted chain -> NO_TRANSFORM. (1.20's
//!   wholesale "child display replaces the parent's" is GONE: block/stairs
//!   owns gui [30,135,0] but inherits block/block's firstperson [0,45,0].)
//! - `ItemTransform.apply(boolean leftHand, PoseStack$Pose)`:
//!   NO_TRANSFORM (identity compare) -> bare `translate(-0.5,-0.5,-0.5)`;
//!   otherwise `translate(±tx, ty, tz)` -> ONE
//!   `Quaternionf().rotationXYZ(rx·DEG, ±ry·DEG, ±rz·DEG)` rotate ->
//!   `scale(sx, sy, sz)` with NO left-hand flip -> `translate(-0.5f×3)`.
//!   DEG = 0.017453292f, multiplied in f32 BEFORE rotationXYZ. leftHand
//!   negates ONLY translation.x, rotation.y and rotation.z (rotation.x and
//!   scale are raw — a 26.2 change vs the 1.20 lore).
//! - The -0.5 tail: 26.2 centers the [0,1] model space per application
//!   (both branches end with it) — element_quads' vertices are already in
//!   block units (0..1), so the pose composes directly.
//! - JOML 1.10.8 paths actually taken on a fresh-identity pose (properties
//!   26 after translate #1, 18 after rotate, per the oracle):
//!   `translate` on IDENTITY -> `translation(FFF)` builds T (properties
//!   26 = TRANSLATION|AFFINE|ORTHONORMAL); `rotate` on a TRANSLATION
//!   matrix -> `rotateTranslation` which ASSIGNS the rotation matrix
//!   wholesale (`_m00(f19)... _m22(f27)`, no 3x3 composition — sound
//!   because the TRANSLATION bit certifies a unit 3x3);
//!   `scale` -> `scaleGeneric` scales columns 0..2 by sx/sy/sz with single
//!   fmuls (so 0.0·(-4.0) = -0.0 propagates — the oracle pins those bits);
//!   the final `translate` -> `translateGeneric`: per row the nested
//!   `org.joml.Math.fma(m00, x, fma(m10, y, fma(m20, z, m30)))`.
//!   `org.joml.Math.fma` dispatches on `Runtime.HAS_Math_fma` =
//!   `Options.USE_MATH_FMA && hasMathFma()` — the vanilla client never
//!   sets `joml.useMathFMA`, so the flag is FALSE and the op is the plain
//!   two-op `a*b + c` (the oracle prints `HAS_Math_fma=false`); the module
//!   spells the nested adds without fusing.
//!   `Vector3f.mulPosition` on the pose: the same nested-fma shape
//!   (mulPositionTranslation/mulPositionGeneric are shape-identical).
//! - `Quaternionf.rotationXYZ`: half-angles in f32, `org.joml.Math.sin`
//!   = `(float) java.lang.Math.sin((double) x)` — F64 TRIG, F32 STORE
//!   (the banked fidelity lesson; it is why the 180-degree on_shelf
//!   rotation is EXACT ±1/0 in the oracle matrix), and `cosFromSin` =
//!   `sqrt(1-s*s)` with the wrap-angle sign fix, all f32.
//!
//! The bit-exact oracle: `scripts/display_ref/DisplayRef.java` runs the
//! REAL joml-1.10.8 op-for-op and prints `Float.floatToIntBits` hex;
//! `scripts/display_ref/display_ref.txt` is the transcript the tests pin.
//! The 26.2 census (scripts/display_census.py): 3928 models, 66 with their
//! own display stanza (zero legacy keys, zero scalar scales, zero clamp
//! hits — max |translation|/16 = 1.40625, max |scale| = 2.0), 483
//! 45-degree-multiple rotation entries; per-context 2622 models resolve a
//! display through the chain.

use crate::generated::models::MODELS;

/// The `0.017453292f` literal ItemTransform.apply multiplies the degrees by.
pub const DEG_TO_RAD: f32 = 0.017453292;

/// The context slot order = the ItemTransforms record's field order
/// (getTransform's switch maps ItemDisplayContext ordinals onto these).
pub const THIRDPERSON_LEFTHAND: usize = 0;
pub const THIRDPERSON_RIGHTHAND: usize = 1;
pub const FIRSTPERSON_LEFTHAND: usize = 2;
pub const FIRSTPERSON_RIGHTHAND: usize = 3;
pub const HEAD: usize = 4;
pub const GUI: usize = 5;
pub const GROUND: usize = 6;
pub const FIXED: usize = 7;
pub const ON_SHELF: usize = 8;

/// The nine serial names, slot order (the enum's getSerializedName values;
/// the JSON keys — no legacy forms exist in 26.2).
pub const CONTEXTS: [&str; 9] = [
    "thirdperson_lefthand",
    "thirdperson_righthand",
    "firstperson_lefthand",
    "firstperson_righthand",
    "head",
    "gui",
    "ground",
    "fixed",
    "on_shelf",
];

/// One parsed context transform: the three deserializer vectors, already
/// clamped and in block units (translation divided by 16).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DisplayTransform {
    pub rotation: [f32; 3],
    pub translation: [f32; 3],
    pub scale: [f32; 3],
}

/// The deserializer defaults — an all-default context is still a REAL
/// transform (it blocks parent inheritance for that context); only an
/// ABSENT context is `None`/NO_TRANSFORM.
pub const IDENTITY_TRANSFORM: DisplayTransform = DisplayTransform {
    rotation: [0.0; 3],
    translation: [0.0; 3],
    scale: [1.0; 3],
};

/// `Mth.clamp(v, lo, hi)` — `v < lo ? lo : Math.min(v, hi)`.
fn clamp(v: f32, lo: f32, hi: f32) -> f32 {
    if v < lo {
        lo
    } else {
        v.min(hi)
    }
}

/// The strict display-stanza parser: `{"context": {transform}, ...}` over
/// the canonical compact JSON the generator retained (serde_json, sorted
/// keys, no whitespace, no escapes — the 66 real stanzas are all plain).
/// Numbers are parsed as f64 and cast (Gson's double->float path).
/// Returns the nine slots with the parse-time left-hand fallback applied
/// (an absent left-hand context takes the stanza's right-hand transform).
pub fn parse_display(json: &str) -> Result<[Option<DisplayTransform>; 9], String> {
    let b = json.as_bytes();
    let mut i = 0usize;
    let mut slots = [None; 9];
    let expect = |i: &mut usize, c: u8| -> Result<(), String> {
        match b.get(*i) {
            Some(&c0) if c0 == c => {
                *i += 1;
                Ok(())
            }
            _ => Err(format!("display: expected {:?} at byte {}", c as char, i)),
        }
    };
    // a plain fn (not a closure): the key borrows from `json`
    fn scan_key<'a>(
        b: &[u8],
        json: &'a str,
        i: &mut usize,
        expect: &dyn Fn(&mut usize, u8) -> Result<(), String>,
    ) -> Result<&'a str, String> {
        expect(i, b'"')?;
        let start = *i;
        while matches!(b.get(*i), Some(c) if *c != b'"') {
            // the vocabulary is [a-z_]; any escape or quote dies here
            if !b[*i].is_ascii_lowercase() && b[*i] != b'_' {
                return Err(format!("display: bad key byte at {}", i));
            }
            *i += 1;
        }
        let s = json.get(start..*i).ok_or("display: key slice")?;
        expect(i, b'"')?;
        Ok(s)
    }
    let number = |i: &mut usize| -> Result<f32, String> {
        let start = *i;
        if matches!(b.get(*i), Some(b'-')) {
            *i += 1;
        }
        while matches!(b.get(*i), Some(c) if c.is_ascii_digit()) {
            *i += 1;
        }
        if matches!(b.get(*i), Some(b'.')) {
            *i += 1;
            while matches!(b.get(*i), Some(c) if c.is_ascii_digit()) {
                *i += 1;
            }
        }
        if matches!(b.get(*i), Some(b'e') | Some(b'E')) {
            *i += 1;
            if matches!(b.get(*i), Some(b'+') | Some(b'-')) {
                *i += 1;
            }
            while matches!(b.get(*i), Some(c) if c.is_ascii_digit()) {
                *i += 1;
            }
        }
        let s = json.get(start..*i).ok_or("display: number slice")?;
        s.parse::<f64>()
            .map(|v| v as f32)
            .map_err(|_| format!("display: bad number {s:?}"))
    };
    let vec3 = |i: &mut usize| -> Result<[f32; 3], String> {
        // the deserializer: getAsJsonArray + size == 3 or JsonParseException
        expect(i, b'[')?;
        let v0 = number(i)?;
        expect(i, b',')?;
        let v1 = number(i)?;
        expect(i, b',')?;
        let v2 = number(i)?;
        expect(i, b']')?;
        Ok([v0, v1, v2])
    };
    expect(&mut i, b'{')?;
    if matches!(b.get(i), Some(b'}')) {
        return Ok(slots); // an empty stanza is legal: every context defaults
    }
    loop {
        let ctx = scan_key(b, json, &mut i, &expect)?;
        let slot = CONTEXTS
            .iter()
            .position(|c| *c == ctx)
            .ok_or_else(|| format!("display: unknown context {ctx:?}"))?;
        if slots[slot].is_some() {
            return Err(format!("display: duplicate context {ctx:?}"));
        }
        expect(&mut i, b':')?;
        expect(&mut i, b'{')?;
        let mut t = IDENTITY_TRANSFORM;
        // an empty transform object keeps the deserializer defaults (and
        // still blocks the parent — a REAL transform, not NO_TRANSFORM)
        if matches!(b.get(i), Some(b'}')) {
            i += 1;
        } else {
            loop {
                let k = scan_key(b, json, &mut i, &expect)?;
                match k {
                    "rotation" => {
                        expect(&mut i, b':')?;
                        t.rotation = vec3(&mut i)?;
                    }
                    "translation" => {
                        expect(&mut i, b':')?;
                        // the deserializer: mul(0.0625f) FIRST, then clamp +-5
                        let v = vec3(&mut i)?;
                        t.translation = v.map(|x| clamp(x * 0.0625, -5.0, 5.0));
                    }
                    "scale" => {
                        expect(&mut i, b':')?;
                        let v = vec3(&mut i)?;
                        t.scale = v.map(|x| clamp(x, -4.0, 4.0));
                    }
                    other => return Err(format!("display: unknown transform key {other:?}")),
                }
                match b.get(i) {
                    Some(b',') => i += 1,
                    Some(b'}') => {
                        i += 1;
                        break;
                    }
                    _ => return Err(format!("display: bad transform tail at byte {i}")),
                }
            }
        }
        slots[slot] = Some(t);
        match b.get(i) {
            Some(b',') => i += 1,
            Some(b'}') => {
                i += 1;
                break;
            }
            _ => return Err(format!("display: bad stanza tail at byte {i}")),
        }
    }
    if i != b.len() {
        return Err(format!("display: trailing bytes at {i} of {}", b.len()));
    }
    // ItemTransforms$Deserializer: left == NO_TRANSFORM -> left = right
    for (right, left) in [
        (THIRDPERSON_RIGHTHAND, THIRDPERSON_LEFTHAND),
        (FIRSTPERSON_RIGHTHAND, FIRSTPERSON_LEFTHAND),
    ] {
        if slots[left].is_none() {
            slots[left] = slots[right];
        }
    }
    Ok(slots)
}

fn lookup(model_id: &str) -> Option<(Option<&'static str>, Option<&'static str>)> {
    MODELS
        .iter()
        .find(|(id, ..)| *id == model_id)
        .map(|(_, parent, _, display, ..)| (*parent, *display))
}

/// `ResolvedModel.findTopTransform` — the per-context chain walk: the
/// nearest ancestor whose own display stanza DECLARES `ctx` wins (an
/// explicit stanza context blocks the parent even when all-default);
/// exhausted chain -> None (NO_TRANSFORM).
pub fn resolve_context(
    model_id: &str,
    ctx: usize,
) -> Result<Option<DisplayTransform>, String> {
    let mut cur = model_id;
    for _ in 0..32 {
        let Some((parent, display)) = lookup(cur) else {
            return Ok(None); // exhausted (the builtins are anchors, not rows)
        };
        if let Some(json) = display {
            if let Some(t) = parse_display(json)?[ctx] {
                return Ok(Some(t));
            }
        }
        match parent {
            Some(p) if !p.starts_with("builtin/") => cur = p,
            _ => return Ok(None),
        }
    }
    Err(format!("display: parent chain of {model_id} exceeds 32"))
}

/// The nine resolved slots for one model (the ItemTransforms container the
/// client holds after discovery); `None` = NO_TRANSFORM for that context.
pub fn resolve_transforms(
    model_id: &str,
) -> Result<[Option<DisplayTransform>; 9], String> {
    let mut out = [None; 9];
    for ctx in 0..9 {
        out[ctx] = resolve_context(model_id, ctx)?;
    }
    Ok(out)
}

/// `ItemDisplayContext.leftHand()` — the two left slots get the mirrored
/// application.
pub fn context_is_left_hand(ctx: usize) -> bool {
    ctx == THIRDPERSON_LEFTHAND || ctx == FIRSTPERSON_LEFTHAND
}

/// `org.joml.Math.sin(float)` — the non-FASTMATH default:
/// `(float) java.lang.Math.sin((double) x)`. F64 trig, f32 store.
fn sin_joml(x: f32) -> f32 {
    (x as f64).sin() as f32
}

/// The JOML float literals of cosFromSinInternal (spelled as the ldc
/// constants; Rust's FRAC_PI_2/PI/TAU round to these exact values).
const HALF_PI_F32: f32 = 1.5707964;
const TAU_F32: f32 = 6.2831855;
const PI_F32: f32 = 3.1415927;

/// `org.joml.Math.cosFromSin(sin, angle)` — the default path:
/// sqrt(1-s*s), then the quadrant of `angle + pi/2` decides the sign.
/// The comparisons mirror the bytecode's dcmpg/fcmpl NaN shapes.
fn cos_from_sin_joml(s: f32, angle: f32) -> f32 {
    let c = (1.0f32 - s * s).sqrt();
    let mut w = angle + HALF_PI_F32;
    w = w - (w / TAU_F32) as i32 as f32 * TAU_F32;
    if w < 0.0 {
        w += TAU_F32;
    }
    if w >= PI_F32 { -c } else { c }
}

/// `Quaternionf.rotationXYZ` verbatim (f64-trig-f32-store half-angles,
/// f32 products, the XYZ intrinsic order R = Rx·Ry·Rz).
fn rotation_xyz(ax: f32, ay: f32, az: f32) -> [f32; 4] {
    let sx = sin_joml(ax * 0.5);
    let cx = cos_from_sin_joml(sx, ax * 0.5);
    let sy = sin_joml(ay * 0.5);
    let cy = cos_from_sin_joml(sy, ay * 0.5);
    let sz = sin_joml(az * 0.5);
    let cz = cos_from_sin_joml(sz, az * 0.5);
    let cycz = cy * cz;
    let sysz = sy * sz;
    let sycz = sy * cz;
    let cysz = cy * sz;
    [
        cx * cycz - sx * sysz, // w
        sx * cycz + cx * sysz, // x
        cx * sycz - sx * cysz, // y
        cx * cysz + sx * sycz, // z
    ]
}

/// The rotation matrix entries f19..f27 exactly as rotateAffine/
/// rotateTranslation compute them (ww/xx/yy/zz squares and the doubled
/// products; every add/sub in the bytecode's order).
fn rotation_matrix(q: [f32; 4]) -> [f32; 9] {
    let [_, x, y, z] = q;
    let w = q[0];
    let ww = w * w;
    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let zw = z * w;
    let zw2 = zw + zw;
    let xy = x * y;
    let xy2 = xy + xy;
    let xz = x * z;
    let xz2 = xz + xz;
    let yw = y * w;
    let yw2 = yw + yw;
    let yz = y * z;
    let yz2 = yz + yz;
    let xw = x * w;
    let xw2 = xw + xw;
    [
        (ww + xx - zz) - yy,   // f19 -> m00
        xy2 + zw2,             // f20 -> m01 (col0 row1)
        xz2 - yw2,             // f21 -> m02
        -zw2 + xy2,            // f22 -> m10
        ((yy - zz) + ww) - xx, // f23 -> m11
        yz2 + xw2,             // f24 -> m12
        yw2 + xz2,             // f25 -> m20
        yz2 - xw2,             // f26 -> m21
        ((zz - yy) - xx) + ww, // f27 -> m22
    ]
}

/// The pose: column-major 4x4 (`cols[c][r]`), the affine shape PoseStack
/// keeps throughout the sequence.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pose {
    cols: [[f32; 4]; 4],
}

/// `ItemTransform.apply` on a fresh-identity pose, op for op: the
/// translation() build, the wholesale rotation assignment, the per-column
/// scale, the nested-plain-fma tail translate. `left_hand` negates ONLY
/// translation.x / rotation.y / rotation.z (the 26.2 mirroring).
pub fn build_pose(t: &DisplayTransform, left_hand: bool) -> Pose {
    let tx = if left_hand { -t.translation[0] } else { t.translation[0] };
    let ry = if left_hand { -t.rotation[1] } else { t.rotation[1] };
    let rz = if left_hand { -t.rotation[2] } else { t.rotation[2] };
    // 1. translate(±tx, ty, tz) on IDENTITY -> translation(FFF) builds T
    let mut m = Pose {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [tx, t.translation[1], t.translation[2], 1.0],
        ],
    };
    // 2. rotate(rotationXYZ) on a TRANSLATION matrix -> rotateTranslation
    //    ASSIGNS R wholesale (no 3x3 composition)
    let f = rotation_matrix(rotation_xyz(
        t.rotation[0] * DEG_TO_RAD,
        ry * DEG_TO_RAD,
        rz * DEG_TO_RAD,
    ));
    m.cols[0] = [f[0], f[1], f[2], 0.0];
    m.cols[1] = [f[3], f[4], f[5], 0.0];
    m.cols[2] = [f[6], f[7], f[8], 0.0];
    // 3. scale(sx, sy, sz) -> scaleGeneric: the FULL columns 0..2 scaled
    //    in place (row 3 included — 0.0·(-4.0) = -0.0, the oracle pins it)
    for c in 0..3 {
        for r in 0..4 {
            m.cols[c][r] *= t.scale[c];
        }
    }
    // 4. translate(-0.5, -0.5, -0.5) -> translateGeneric: the nested
    //    PLAIN fma (org.joml.Math.fma fallback, HAS_Math_fma=false)
    for r in 0..3 {
        let a = m.cols[2][r] * -0.5 + m.cols[3][r];
        let b = m.cols[1][r] * -0.5 + a;
        m.cols[3][r] = m.cols[0][r] * -0.5 + b;
    }
    m
}

/// `Vector3f.mulPosition` — the nested plain-fma vertex multiply
/// (x·col0 + (y·col1 + (z·col2 + col3)), each product+add unfused).
pub fn apply_vertex(m: &Pose, v: [f32; 3]) -> [f32; 3] {
    [
        m.cols[0][0] * v[0] + (m.cols[1][0] * v[1] + (m.cols[2][0] * v[2] + m.cols[3][0])),
        m.cols[0][1] * v[0] + (m.cols[1][1] * v[1] + (m.cols[2][1] * v[2] + m.cols[3][1])),
        m.cols[0][2] * v[0] + (m.cols[1][2] * v[1] + (m.cols[2][2] * v[2] + m.cols[3][2])),
    ]
}

/// The full link for one vertex: `None` (NO_TRANSFORM — the context is
/// absent down the whole chain) applies the bare -0.5 centering; `Some`
/// runs the ItemTransform.apply sequence.
pub fn apply_transform(
    t: Option<&DisplayTransform>,
    left_hand: bool,
    v: [f32; 3],
) -> [f32; 3] {
    match t {
        None => {
            let m = Pose {
                cols: [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [-0.5, -0.5, -0.5, 1.0],
                ],
            };
            apply_vertex(&m, v)
        }
        Some(t) => apply_vertex(&build_pose(t, left_hand), v),
    }
}

/// The e2e composition the client consumer will call: the model's quads
/// (element_quads, block units) through the resolved display pose of
/// `context`. Unknown model -> None; a broken stanza -> Err.
pub fn posed_quads(
    model_id: &str,
    ctx: usize,
) -> Option<Result<Vec<[[f32; 3]; 4]>, String>> {
    let quads = crate::element_quads::derive_model(model_id)?;
    let transforms = match resolve_transforms(model_id) {
        Ok(t) => t,
        Err(e) => return Some(Err(e)),
    };
    let left_hand = context_is_left_hand(ctx);
    Some(Ok(quads
        .iter()
        .map(|q| {
            q.vertices
                .map(|v| apply_transform(transforms[ctx].as_ref(), left_hand, v))
        })
        .collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// item/handheld's display stanza, verbatim from the generated MODELS
    /// table (canonical compact JSON, sorted keys).
    const HANDHELD_DISPLAY: &str = concat!(
        r#"{"firstperson_lefthand":{"rotation":[0,90,-25],"scale":[0.68,0.68,0.68],"translation":[1.13,3.2,1.13]},"#,
        r#""firstperson_righthand":{"rotation":[0,-90,25],"scale":[0.68,0.68,0.68],"translation":[1.13,3.2,1.13]},"#,
        r#""thirdperson_lefthand":{"rotation":[0,90,-55],"scale":[0.85,0.85,0.85],"translation":[0,4.0,0.5]},"#,
        r#""thirdperson_righthand":{"rotation":[0,-90,55],"scale":[0.85,0.85,0.85],"translation":[0,4.0,0.5]}}"#,
    );

    #[test]
    fn parses_the_handheld_stanza_with_the_deserializer_semantics() {
        let slots = parse_display(HANDHELD_DISPLAY).expect("handheld parses");
        // all four hands declared -> no fallback involved
        for ctx in [
            THIRDPERSON_LEFTHAND,
            THIRDPERSON_RIGHTHAND,
            FIRSTPERSON_LEFTHAND,
            FIRSTPERSON_RIGHTHAND,
        ] {
            assert!(slots[ctx].is_some(), "{} declared", CONTEXTS[ctx]);
        }
        // the deserializer vectors, bit-exact vs the oracle
        // ("sword_fpr parse t 3d90a3d7 3e4ccccd 3d90a3d7 s 3f2e147b ...")
        let fpr = slots[FIRSTPERSON_RIGHTHAND].unwrap();
        assert_eq!(
            fpr.translation.map(f32::to_bits),
            [0x3d90a3d7, 0x3e4ccccd, 0x3d90a3d7]
        );
        assert_eq!(fpr.scale.map(f32::to_bits), [0x3f2e147b; 3]);
        assert_eq!(fpr.rotation, [0.0, -90.0, 25.0]);
        // thirdperson_righthand: translation [0, 4.0, 0.5] -> [0, 0.25, 0.03125]
        let tpr = slots[THIRDPERSON_RIGHTHAND].unwrap();
        assert_eq!(
            tpr.translation.map(f32::to_bits),
            [0x00000000, 0x3e800000, 0x3d000000]
        );
        // the non-hand contexts stay NO_TRANSFORM
        for ctx in [HEAD, GUI, GROUND, FIXED, ON_SHELF] {
            assert!(slots[ctx].is_none(), "{} absent", CONTEXTS[ctx]);
        }
    }

    #[test]
    fn the_parse_time_left_fallback_fills_from_the_right() {
        // block/block declares thirdperson_righthand but not the left —
        // the deserializer replaces the absent left with the right AT PARSE
        // TIME (ItemTransforms$Deserializer bytecode).
        let slots = parse_display(
            r#"{"firstperson_lefthand":{"rotation":[0,225,0],"scale":[0.4,0.4,0.4],"translation":[0,0,0]},"firstperson_righthand":{"rotation":[0,45,0],"scale":[0.4,0.4,0.4],"translation":[0,0,0]},"thirdperson_righthand":{"rotation":[75,45,0],"scale":[0.375,0.375,0.375],"translation":[0,2.5,0]}}"#,
        )
        .expect("block/block parses");
        assert_eq!(slots[THIRDPERSON_LEFTHAND], slots[THIRDPERSON_RIGHTHAND]);
        let tpr = slots[THIRDPERSON_RIGHTHAND].unwrap();
        assert_eq!(tpr.rotation, [75.0, 45.0, 0.0]);
        // the block-units translation: 2.5/16 = 0.15625
        assert_eq!(
            tpr.translation.map(f32::to_bits),
            [0x00000000, 0x3e200000, 0x00000000]
        );
    }

    #[test]
    fn the_clamps_fire_in_the_deserializer_order() {
        // the synthetic exercise: /16 FIRST then clamp +-5 (translation),
        // scale clamped +-4 (rotation raw). Oracle: "clamps parse t
        // 40a00000 c0a00000 40a00000 s 40800000 c0800000 40800000".
        let slots = parse_display(
            r#"{"gui":{"rotation":[0,0,0],"scale":[10,-5,4],"translation":[100,-90,80]}}"#,
        )
        .expect("clamps parse");
        let t = slots[GUI].unwrap();
        assert_eq!(t.translation.map(f32::to_bits), [0x40a00000, 0xc0a00000, 0x40a00000]);
        assert_eq!(t.scale.map(f32::to_bits), [0x40800000, 0xc0800000, 0x40800000]);
        // 80/16 = 5.0 exactly -> the clamp keeps it (>=, not >)
        assert_eq!(t.translation[2], 5.0);
        // scale 4 stays (the boundary is inclusive both ways)
        assert_eq!(t.scale[2], 4.0);
    }

    #[test]
    fn an_empty_context_is_a_real_transform_that_defaults() {
        // "{}" as a transform value is a PRESENT context (it blocks the
        // parent) with the deserializer defaults — not NO_TRANSFORM.
        let slots = parse_display(r#"{"gui":{}}"#).expect("empty gui parses");
        assert_eq!(slots[GUI], Some(IDENTITY_TRANSFORM));
    }

    #[test]
    fn the_parser_refuses_the_fail_closed_shapes() {
        assert!(parse_display(r#"{"nope":{}}"#).is_err(), "unknown context");
        assert!(parse_display(r#"{"gui":{"rotation":[0,45]}}"#).is_err(), "short array");
        assert!(parse_display(r#"{"gui":{"rotation":[0,45,0,15]}}"#).is_err(), "long array");
        assert!(parse_display(r#"{"gui":{"tilt":[0,0,0]}}"#).is_err(), "unknown key");
        assert!(parse_display(r#"{"gui":{"gui":{}}}"#).is_err(), "duplicate context");
        assert!(parse_display(r#" {"gui":{}}"#).is_err(), "not canonical");
        assert!(parse_display(r#"{"gui":{}} x"#).is_err(), "trailing");
    }

    // ---- the joml oracle (scripts/display_ref/display_ref.txt) ----

    /// One oracle fixture: (tag, transform, [right, left] matrices as 16
    /// column-major bit words).
    fn oracle_matrix_cases() -> Vec<(String, DisplayTransform, [[u32; 16]; 2])> {
        let block_block = resolve_transforms("block/block").expect("block/block");
        let sword = resolve_transforms("item/iron_sword").expect("iron_sword");
        let clamps =
            parse_display(r#"{"gui":{"rotation":[0,0,0],"scale":[10,-5,4],"translation":[100,-90,80]}}"#)
                .expect("clamps");
        vec![
            (
                "sword_fpr".into(),
                sword[FIRSTPERSON_RIGHTHAND].unwrap(),
                [
                    [
                        0x00000000, 0x3e932387, 0x3f1dc523, 0x00000000, 0x00000000, 0x3f1dc523,
                        0xbe932387, 0x00000000, 0xbf2e147b, 0x00000000, 0x00000000, 0x00000000,
                        0x3ed23d71, 0xbe80f080, 0xbdc029a8, 0x3f800000,
                    ],
                    [
                        0x00000000, 0xbe932387, 0xbf1dc523, 0x00000000, 0x00000000, 0x3f1dc523,
                        0xbe932387, 0x00000000, 0x3f2e147b, 0x00000000, 0x00000000, 0x00000000,
                        0xbed23d71, 0x3d119838, 0x3f05bfee, 0x3f800000,
                    ],
                ],
            ),
            (
                "block_fpr".into(),
                block_block[FIRSTPERSON_RIGHTHAND].unwrap(),
                [
                    [
                        0x3e90d0c2, 0x00000000, 0xbe90d0c3, 0x00000000, 0x00000000, 0x3ecccccd,
                        0x00000000, 0x00000000, 0x3e90d0c3, 0x00000000, 0x3e90d0c2, 0x00000000,
                        0xbe90d0c2, 0xbe4ccccd, 0x32800000, 0x3f800000,
                    ],
                    [
                        0x3e90d0c2, 0x80000000, 0x3e90d0c3, 0x00000000, 0x00000000, 0x3ecccccd,
                        0x00000000, 0x00000000, 0xbe90d0c3, 0x00000000, 0x3e90d0c2, 0x00000000,
                        0x32800000, 0xbe4ccccd, 0xbe90d0c2, 0x3f800000,
                    ],
                ],
            ),
            (
                "block_gui".into(),
                block_block[GUI].unwrap(),
                [
                    [
                        0xbee24634, 0xbe62462c, 0x3ec3f58b, 0x00000000, 0xb2200000, 0x3f0a9067,
                        0x3e9fffff, 0x00000000, 0xbee2462d, 0x3e624632, 0xbec3f58e, 0x00000000,
                        0x3ee24630, 0xbe8a9069, 0xbe1ffffc, 0x3f800000,
                    ],
                    [
                        0xbee24634, 0x3e62462c, 0xbec3f58b, 0x00000000, 0x32200000, 0x3f0a9067,
                        0x3e9fffff, 0x00000000, 0x3ee2462d, 0x3e624632, 0xbec3f58e, 0x00000000,
                        0x33e00000, 0xbefbb37f, 0x3e67eb1a, 0x3f800000,
                    ],
                ],
            ),
            (
                "block_shelf".into(),
                block_block[ON_SHELF].unwrap(),
                [
                    [
                        0xbf800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000,
                        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xbf800000, 0x00000000,
                        0x3f000000, 0xbf000000, 0x3f000000, 0x3f800000,
                    ],
                    [
                        0xbf800000, 0x80000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000,
                        0x00000000, 0x00000000, 0x80000000, 0x00000000, 0xbf800000, 0x00000000,
                        0x3f000000, 0xbf000000, 0x3f000000, 0x3f800000,
                    ],
                ],
            ),
            (
                "clamps".into(),
                clamps[GUI].unwrap(),
                [
                    [
                        0x40800000, 0x00000000, 0x00000000, 0x00000000, 0x80000000, 0xc0800000,
                        0x80000000, 0x80000000, 0x00000000, 0x00000000, 0x40800000, 0x00000000,
                        0x40400000, 0xc0400000, 0x40400000, 0x3f800000,
                    ],
                    [
                        0x40800000, 0x00000000, 0x80000000, 0x00000000, 0x80000000, 0xc0800000,
                        0x80000000, 0x80000000, 0x00000000, 0x80000000, 0x40800000, 0x00000000,
                        0xc0e00000, 0xc0400000, 0x40400000, 0x3f800000,
                    ],
                ],
            ),
        ]
    }

    #[test]
    fn the_oracle_matrices_are_bit_exact() {
        for (tag, t, matrices) in oracle_matrix_cases() {
            for (hand, bits) in matrices.iter().enumerate() {
                let pose = build_pose(&t, hand == 1);
                for (c, col) in pose.cols.iter().enumerate() {
                    for (r, v) in col.iter().enumerate() {
                        assert_eq!(
                            v.to_bits(),
                            bits[c * 4 + r],
                            "{tag} {} m{}{}",
                            if hand == 1 { "L" } else { "R" },
                            c,
                            r
                        );
                    }
                }
            }
        }
    }

    /// The oracle's vertex probes (scripts/display_ref/display_ref.txt):
    /// (tag, transform, left_hand, input, expected output bits).
    fn oracle_vertex_cases() -> Vec<(&'static str, DisplayTransform, bool, [f32; 3], [u32; 3])> {
        let block_block = resolve_transforms("block/block").expect("block/block");
        let sword = resolve_transforms("item/iron_sword").expect("iron_sword");
        let clamps = parse_display(
            r#"{"gui":{"rotation":[0,0,0],"scale":[10,-5,4],"translation":[100,-90,80]}}"#,
        )
        .expect("clamps");
        let sword_r = sword[FIRSTPERSON_RIGHTHAND].unwrap();
        let mut cases = Vec::new();
        // sword_fpr R — all six probes
        for (v, o) in [
            ([0.0, 0.0, 0.0], [0x3ed23d71, 0xbe80f080, 0xbdc029a8]),
            ([1.0, 1.0, 1.0], [0xbe89eb85, 0x3f26dea6, 0x3e70b8aa]),
            ([0.5, 0.5, 0.5], [0x3d90a3d8, 0x3e4ccccd, 0x3d90a3d6]),
            ([1.0, 0.0, 0.0], [0x3ed23d71, 0x3d119838, 0x3f05bfee]),
            (
                [0.0625, 0.8125, 0.4375],
                [0x3de7ae14, 0x3e88a210, 0xbe93dea4],
            ),
            (
                [0.9375, 0.1875, 0.6875],
                [0xbd68f5c0, 0x3e085578, 0x3edc308f],
            ),
        ] {
            cases.push(("sword_fpr R", sword_r, false, v, o));
        }
        // sword_fpr L — all six probes
        for (v, o) in [
            (
                [0.0, 0.0, 0.0],
                [0xbed23d71, 0x3d119838, 0x3f05bfee],
            ),
            (
                [1.0, 1.0, 1.0],
                [0x3e89eb85, 0x3eba99c5, 0xbec32df1],
            ),
            (
                [0.5, 0.5, 0.5],
                [0xbd90a3d8, 0x3e4ccccd, 0x3d90a3d4],
            ),
            (
                [1.0, 0.0, 0.0],
                [0xbed23d71, 0xbe80f080, 0xbdc029a8],
            ),
            (
                [0.0625, 0.8125, 0.4375],
                [0xbde7ae14, 0x3f04b094, 0x3e803a5a],
            ),
            (
                [0.9375, 0.1875, 0.6875],
                [0x3d68f5c0, 0xbdf2516c, 0xbddfa1bc],
            ),
        ] {
            cases.push(("sword_fpr L", sword_r, true, v, o));
        }
        // block_fpr R/L — three probes each
        let b_fpr = block_block[FIRSTPERSON_RIGHTHAND].unwrap();
        for (left, v, o) in [
            (false, [0.0, 0.0, 0.0], [0xbe90d0c2, 0xbe4ccccd, 0x32800000]),
            (false, [1.0, 1.0, 1.0], [0x3e90d0c3, 0x3e4ccccd, 0xb3000000]),
            (false, [0.5, 0.5, 0.5], [0x32800000, 0x00000000, 0x00000000]),
            (true, [0.0, 0.0, 0.0], [0x32800000, 0xbe4ccccd, 0xbe90d0c2]),
            (true, [1.0, 1.0, 1.0], [0x00000000, 0x3e4ccccd, 0x3e90d0c3]),
            (true, [0.5, 0.5, 0.5], [0x00000000, 0x00000000, 0x32800000]),
        ] {
            cases.push(("block_fpr", b_fpr, left, v, o));
        }
        // block_gui R/L
        let b_gui = block_block[GUI].unwrap();
        for (left, v, o) in [
            (false, [0.0, 0.0, 0.0], [0x3ee24630, 0xbe8a9069, 0xbe1ffffc]),
            (false, [1.0, 1.0, 1.0], [0xbee24631, 0x3e8a9068, 0x3e1ffffc]),
            (false, [0.5, 0.5, 0.5], [0xb2800000, 0xb2800000, 0x00000000]),
            (true, [0.0, 0.0, 0.0], [0x33e00000, 0xbefbb37f, 0x3e67eb1a]),
            (true, [1.0, 1.0, 1.0], [0xb4000000, 0x3efbb37e, 0xbe67eb1a]),
            (true, [0.5, 0.5, 0.5], [0x00000000, 0x00000000, 0x00000000]),
        ] {
            cases.push(("block_gui", b_gui, left, v, o));
        }
        // block_shelf R/L — the exact 180-degree rotation
        let b_shelf = block_block[ON_SHELF].unwrap();
        for (left, v, o) in [
            (false, [0.0, 0.0, 0.0], [0x3f000000, 0xbf000000, 0x3f000000]),
            (false, [1.0, 1.0, 1.0], [0xbf000000, 0x3f000000, 0xbf000000]),
            (false, [0.5, 0.5, 0.5], [0x00000000, 0x00000000, 0x00000000]),
            (true, [0.0, 0.0, 0.0], [0x3f000000, 0xbf000000, 0x3f000000]),
            (true, [1.0, 1.0, 1.0], [0xbf000000, 0x3f000000, 0xbf000000]),
            (true, [0.5, 0.5, 0.5], [0x00000000, 0x00000000, 0x00000000]),
        ] {
            cases.push(("block_shelf", b_shelf, left, v, o));
        }
        // clamps R/L
        let c_gui = clamps[GUI].unwrap();
        for (left, v, o) in [
            (false, [0.0, 0.0, 0.0], [0x40400000, 0xc0400000, 0x40400000]),
            (false, [1.0, 1.0, 1.0], [0x40e00000, 0xc0e00000, 0x40e00000]),
            (false, [0.5, 0.5, 0.5], [0x40a00000, 0xc0a00000, 0x40a00000]),
            (false, [1.0, 0.0, 0.0], [0x40e00000, 0xc0400000, 0x40400000]),
            (true, [0.0, 0.0, 0.0], [0xc0e00000, 0xc0400000, 0x40400000]),
            (true, [1.0, 1.0, 1.0], [0xc0400000, 0xc0e00000, 0x40e00000]),
            (true, [0.5, 0.5, 0.5], [0xc0a00000, 0xc0a00000, 0x40a00000]),
            (true, [1.0, 0.0, 0.0], [0xc0400000, 0xc0400000, 0x40400000]),
        ] {
            cases.push(("clamps", c_gui, left, v, o));
        }
        cases
    }

    #[test]
    fn the_oracle_vertices_are_bit_exact() {
        for (tag, t, left_hand, v, out) in oracle_vertex_cases() {
            assert_eq!(
                apply_transform(Some(&t), left_hand, v).map(f32::to_bits),
                out,
                "{tag} v{v:?} left={left_hand}"
            );
        }
    }

    #[test]
    fn no_transform_is_the_bare_centering() {
        // the NO_TRANSFORM branch: translate(-0.5) only, on a fresh identity.
        for (v, o) in [
            ([0.0, 0.0, 0.0], [0xbf000000, 0xbf000000, 0xbf000000]),
            ([1.0, 1.0, 1.0], [0x3f000000, 0x3f000000, 0x3f000000]),
            ([0.5, 0.5, 0.5], [0x00000000, 0x00000000, 0x00000000]),
            ([1.0, 0.0, 0.0], [0x3f000000, 0xbf000000, 0xbf000000]),
            (
                [0.0625, 0.8125, 0.4375],
                [0xbee00000, 0x3ea00000, 0xbd800000],
            ),
            (
                [0.9375, 0.1875, 0.6875],
                [0x3ee00000, 0xbea00000, 0x3e400000],
            ),
        ] {
            assert_eq!(
                apply_transform(None, false, v).map(f32::to_bits),
                o,
                "no_transform v{v:?}"
            );
            assert_eq!(
                apply_transform(None, true, v).map(f32::to_bits),
                o,
                "no_transform ignores the hand"
            );
        }
    }

    #[test]
    fn the_chain_walks_per_context_with_nearest_wins() {
        // block/stairs owns gui [30,135,0] — it must BLOCK block/block's
        // gui [30,225,0] (the 26.2 per-context walk; 1.20's wholesale
        // inheritance would have hidden block/block's firstperson from
        // stairs entirely).
        let stairs = resolve_transforms("block/stairs").expect("stairs");
        let gui = stairs[GUI].unwrap();
        assert_eq!(gui.rotation, [30.0, 135.0, 0.0]);
        // ...but stairs declares NO firstperson_righthand, so the walk
        // continues to block/block and takes [0,45,0] — the discriminating
        // case between per-context and wholesale semantics.
        let fpr = stairs[FIRSTPERSON_RIGHTHAND].unwrap();
        assert_eq!(fpr.rotation, [0.0, 45.0, 0.0]);
        assert_eq!(
            fpr.translation.map(f32::to_bits),
            [0x00000000, 0x00000000, 0x00000000]
        );
        assert_eq!(fpr.scale.map(f32::to_bits), [0x3ecccccd; 3]);
        // stairs' own thirdperson_lefthand [75,-135,0] beats the chain
        let tpl = stairs[THIRDPERSON_LEFTHAND].unwrap();
        assert_eq!(tpl.rotation, [75.0, -135.0, 0.0]);
        // block/custom_fence_inventory (own fixed/gui/on_shelf, parent
        // block/block): ground falls back to block/block's [0,3,0]/16;
        // head is declared NOWHERE on the chain -> NO_TRANSFORM.
        let fence = resolve_transforms("block/custom_fence_inventory").expect("fence");
        let ground = fence[GROUND].unwrap();
        assert_eq!(
            ground.translation.map(f32::to_bits),
            [0x00000000, 0x3e400000, 0x00000000]
        );
        assert!(fence[HEAD].is_none(), "head exhausted -> NO_TRANSFORM");
        // item/iron_sword: fpr through item/handheld; gui nowhere.
        let sword = resolve_transforms("item/iron_sword").expect("sword");
        assert_eq!(
            sword[FIRSTPERSON_RIGHTHAND].unwrap().rotation,
            [0.0, -90.0, 25.0]
        );
        assert!(sword[GUI].is_none());
        // the 26.2 on_shelf context resolves through the chain too
        assert_eq!(fence[ON_SHELF].unwrap().rotation, [0.0, 90.0, 0.0]);
        assert_eq!(
            resolve_transforms("block/block")
                .expect("block/block")[ON_SHELF]
                .unwrap()
                .rotation,
            [0.0, 180.0, 0.0]
        );
    }

    #[test]
    fn left_hand_contexts_are_the_two_left_slots() {
        assert!(context_is_left_hand(THIRDPERSON_LEFTHAND));
        assert!(context_is_left_hand(FIRSTPERSON_LEFTHAND));
        for ctx in [
            THIRDPERSON_RIGHTHAND,
            FIRSTPERSON_RIGHTHAND,
            HEAD,
            GUI,
            GROUND,
            FIXED,
            ON_SHELF,
        ] {
            assert!(!context_is_left_hand(ctx));
        }
    }

    #[test]
    fn the_registry_wide_display_census_holds() {
        // scripts/display_census.py over the real jar — the module must
        // reproduce every number.
        let mut with_display = 0usize;
        let mut per_ctx_own = [0usize; 9];
        let mut clamp_hits = 0usize;
        let mut max_t_bits = 0u32;
        let mut max_s_bits = 0u32;
        let mut rot_45 = 0usize;
        for (id, _, _, display, ..) in MODELS.iter() {
            let Some(json) = display else { continue };
            let _ = id;
            with_display += 1;
            for (ctx, t) in parse_display(json).expect("every stanza parses").iter().enumerate() {
                // the DECLARED contexts (pre-fallback: the fallback copies
                // right into left, so the raw JSON is the source of truth
                // for both the counts and the value scans)
                let declared = json.contains(&format!("\"{}\"", CONTEXTS[ctx]));
                if declared {
                    per_ctx_own[ctx] += 1;
                }
                let Some(t) = t else { continue };
                if declared {
                    for c in 0..3 {
                        let tb = t.translation[c].to_bits();
                        if (tb & 0x7fff_ffff) > max_t_bits {
                            max_t_bits = tb & 0x7fff_ffff;
                        }
                        let sb = t.scale[c].to_bits();
                        if (sb & 0x7fff_ffff) > max_s_bits {
                            max_s_bits = sb & 0x7fff_ffff;
                        }
                        // post-parse values cannot exceed the clamps
                        if t.translation[c].abs() > 5.0 || t.scale[c].abs() > 4.0 {
                            clamp_hits += 1;
                        }
                    }
                    for r in t.rotation {
                        // the f64 modulo (the JSON decimals are doubles;
                        // an f32 remainder could round to a false 0)
                        if (r as f64) % 45.0 == 0.0 {
                            rot_45 += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(with_display, 66, "models with their own display stanza");
        assert_eq!(
            per_ctx_own,
            [18, 39, 18, 34, 23, 31, 15, 26, 11],
            "per-context own counts (thirdperson_l/r, firstperson_l/r, head, gui, ground, fixed, on_shelf)"
        );
        assert_eq!(clamp_hits, 0, "the clamps never fire on real 26.2 data");
        // max |translation| post-/16 = 1.40625 (0x3fb40000), max |scale| = 2.0
        assert_eq!(max_t_bits, 0x3fb40000);
        assert_eq!(max_s_bits, 0x40000000);
        assert_eq!(rot_45, 483, "45-degree-multiple rotation entries");
    }

    #[test]
    fn the_registry_wide_chain_resolution_holds() {
        // the per-context walk over all 3928 models (findTopTransforms).
        // scripts/display_census.py, per-context semantics:
        let mut resolved = [0usize; 9];
        for (id, ..) in MODELS.iter() {
            let slots = resolve_transforms(id).expect("every stanza parses");
            for (ctx, t) in slots.iter().enumerate() {
                if t.is_some() {
                    resolved[ctx] += 1;
                }
            }
        }
        assert_eq!(
            resolved,
            [2622, 2622, 2618, 2618, 1386, 1431, 2621, 2621, 1357],
            "per-context resolved counts over the table"
        );
    }

    #[test]
    fn the_left_hand_wins_are_attribution_pinned() {
        // for each left-hand context: how many models resolve it from a
        // stanza that DECLARED the left context vs from the parse-time
        // right->left fallback (census: declared + fallback = resolved).
        for (left, right, declared, fallback) in [
            (
                THIRDPERSON_LEFTHAND,
                THIRDPERSON_RIGHTHAND,
                141usize,
                2481usize,
            ),
            (FIRSTPERSON_LEFTHAND, FIRSTPERSON_RIGHTHAND, 1344, 1274),
        ] {
            let left_key = format!("\"{}\"", CONTEXTS[left]);
            let right_key = format!("\"{}\"", CONTEXTS[right]);
            let mut d = 0usize;
            let mut f = 0usize;
            for (id, ..) in MODELS.iter() {
                // the winner: the first chain row whose RAW stanza declares
                // either the left or the right context (the fallback makes
                // them coincide); attribution reads the raw JSON.
                let mut cur = *id;
                let mut won = false;
                for _ in 0..32 {
                    let Some((parent, display)) = lookup(cur) else { break };
                    if let Some(json) = display {
                        if json.contains(&left_key) {
                            d += 1;
                            won = true;
                            break;
                        }
                        if json.contains(&right_key) {
                            f += 1;
                            won = true;
                            break;
                        }
                    }
                    match parent {
                        Some(p) if !p.starts_with("builtin/") => cur = p,
                        _ => break,
                    }
                }
                if !won {
                    // the ~1306 models whose chain carries no display at
                    // all resolve neither hand — not census members
                    continue;
                }
            }
            assert_eq!((d, f), (declared, fallback), "{} attribution", CONTEXTS[left]);
        }
    }

    #[test]
    fn posed_quads_compose_the_full_link() {
        // block/stairs is an element model WITH its own display: the quads
        // go through the resolved gui pose ([30,135,0]/0.625).
        let quads = crate::element_quads::derive_model("block/stairs")
            .expect("stairs has elements");
        let posed = posed_quads("block/stairs", GUI)
            .expect("known model")
            .expect("stanzas parse");
        assert_eq!(posed.len(), quads.len());
        // the first quad's first vertex through the whole chain, bit-pinned
        // against the joml oracle fed the same vertex
        // (scripts/display_ref DisplayRef stdin mode).
        let v0 = quads[0].vertices[0];
        let out = posed[0][0];
        let t = resolve_transforms("block/stairs")
            .expect("resolves")[GUI]
            .unwrap();
        assert_eq!(t.rotation, [30.0, 135.0, 0.0]);
        assert_eq!(out, apply_vertex(&build_pose(&t, false), v0));
        // and the same vertex against the ORACLE bits (filled from the
        // harness run below)
        let bits: [u32; 3] = ORACLE_QUAD0_V0.map(|w| w.parse().unwrap());
        assert_eq!(out.map(f32::to_bits), bits);
        // the first-person context falls back to block/block's [0,45,0]
        let posed_fpr = posed_quads("block/stairs", FIRSTPERSON_RIGHTHAND)
            .expect("known model")
            .expect("stanzas parse");
        assert_eq!(posed_fpr.len(), quads.len());
    }

    /// display_ref oracle, stdin mode, the stairs-gui transform
    /// ([30,135,0]/0.625), block/stairs quad[0] vertex 0 = (0,0,1):
    /// "STDIN 3ee24630 be8a9066 be1ffffd".
    const ORACLE_QUAD0_V0: [&str; 3] = ["3ee24630", "be8a9066", "be1ffffd"];
}


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

/// The e2e composition for GENERATED items (the ItemModelGenerator bake):
/// the bake quads through the resolved display pose of `context`. The
/// bake's pixel units are ALREADY converted to the vanilla BakedQuad
/// space inside the bake: element_quads::faceinfo_corners (which
/// item_bake's emit shares) divides the FaceInfo-selected corner by 16.0f
/// — the javap evidence (26.2 FaceBakery.bakeVertex: Vector3f.div(16.0f)
/// immediately after VertexInfo.select) — so the Quad vertices are block
/// units for element models AND baked items alike, and this composition
/// consumes them as-is, exactly like posed_quads. Unknown model -> None;
/// a broken stanza -> Err.
pub fn posed_item_quads(
    model_id: &str,
    ctx: usize,
) -> Option<Result<Vec<crate::element_quads::Quad>, String>> {
    posed_item_layer_quads(model_id, None, ctx)
}

/// The node transformation's composed pose — the
/// `com.mojang.math.Transformation.compose` bytecode verbatim:
/// `new Matrix4f().translation(t).rotate(left).scale(s).rotate(right)`,
/// each post-multiplied on identity (T·L·S·R). The JOML fast paths the
/// ItemTransform arc already pinned ride along: the first rotate() on a
/// pure translation ASSIGNS the rotation (the full multiply below is
/// bit-identical for a pure-translation left factor), the scale() scales
/// the FULL columns 0..2 in place (row 3 included — 0.0·s stays 0.0),
/// the second rotate() is the general affine multiply.
pub fn build_local_pose(t: ([f32; 3], [f32; 4], [f32; 3], [f32; 4])) -> Pose {
    // the JSON/tuple convention is [x, y, z, w]; the rotation_matrix
    // helper reads [w, x, y, z] (its q[0] is w) — reorder both
    let (translation, left_rotation, scale, right_rotation) = t;
    let wxzy = |q: [f32; 4]| [q[3], q[0], q[1], q[2]];
    let mut m = Pose {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    // translation()
    m.cols[3] = [translation[0], translation[1], translation[2], 1.0];
    // rotate(left)
    m = pose_mul(&m, &pose_from_rotation(rotation_matrix(wxzy(left_rotation))));
    // scale()
    for c in 0..3 {
        for r in 0..4 {
            m.cols[c][r] *= scale[c];
        }
    }
    // rotate(right)
    m = pose_mul(&m, &pose_from_rotation(rotation_matrix(wxzy(right_rotation))));
    m
}

/// The rotation matrix's column-major pose (the same assignment
/// build_pose makes after its own rotation_matrix call).
fn pose_from_rotation(f: [f32; 9]) -> Pose {
    Pose {
        cols: [
            [f[0], f[1], f[2], 0.0],
            [f[3], f[4], f[5], 0.0],
            [f[6], f[7], f[8], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

/// The 4x4 column-major multiply — `Matrix4f.mul`'s post-multiplication
/// (`this = this · other`); for affine inputs the bottom row stays
/// exactly [0, 0, 0, 1] (the zero products and the final 1·1 are exact).
fn pose_mul(a: &Pose, b: &Pose) -> Pose {
    let mut out = Pose { cols: [[0.0; 4]; 4] };
    for c in 0..4 {
        for r in 0..4 {
            let mut acc = 0.0f32;
            for k in 0..4 {
                acc += a.cols[k][r] * b.cols[c][k];
            }
            out.cols[c][r] = acc;
        }
    }
    out
}

/// The e2e composition for ONE resting-hand layer: the bake quads
/// through the node transformation FIRST, then the resolved display
/// pose of `context`. The order is the vanilla
/// `ItemStackRenderState$LayerRenderState.applyTransform` bytecode:
/// `itemTransform.apply(...)` THEN `pose.mulPose(localTransform)` — the
/// vertices render as handPose · displayPose · localTransform · v, so
/// the local transform acts on the raw bake vertices and the display
/// stanza on top. `None` (identity) feeds the raw vertices straight
/// through — byte-identical to [`posed_item_quads`]. Unknown model ->
/// None; a broken stanza -> Err.
pub fn posed_item_layer_quads(
    model_id: &str,
    transform: Option<([f32; 3], [f32; 4], [f32; 3], [f32; 4])>,
    ctx: usize,
) -> Option<Result<Vec<crate::element_quads::Quad>, String>> {
    // the vanilla bake's own two paths: the ItemModelGenerator bake for
    // the item/generated chains (the flat sprite extrusions), the
    // FaceBakery element bake for the element models (the beds'
    // template geometry, the block-model items) — the model's parent
    // chain picks the path, exactly like vanilla's ModelManager
    let quads = match crate::item_bake::derive_item_model(model_id) {
        Some(q) => q,
        None => crate::element_quads::derive_model_inherited(model_id)?,
    };
    let transforms = match resolve_transforms(model_id) {
        Ok(t) => t,
        Err(e) => return Some(Err(e)),
    };
    let local = transform.map(build_local_pose);
    let left_hand = context_is_left_hand(ctx);
    Some(Ok(quads
        .iter()
        .map(|q| {
            let mut posed = *q;
            posed.vertices = q
                .vertices
                .map(|v| {
                    let raw = match &local {
                        Some(m) => apply_vertex(m, v),
                        None => v,
                    };
                    apply_transform(transforms[ctx].as_ref(), left_hand, raw)
                });
            posed
        })
        .collect()))
}

/// The ModelPart node transform — `translateAndRotate`'s own order:
/// translate(pose / 16), rotX, rotY, rotZ, scale. The pose units are the
/// model's own 1/16-block units (the bytecode divides by 16 before the
/// translate); the rotations are radians (xRot, yRot, zRot, each the
/// mulPose(axis·angle) call); the scale is the direct factor.
fn part_pose(pose: [f32; 3], rot: [f32; 3], scale: [f32; 3]) -> Pose {
    let ident = Pose {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };
    let mut m = ident;
    m.cols[3] = [pose[0] / 16.0, pose[1] / 16.0, pose[2] / 16.0, 1.0];
    let axis_rot = |axis: usize, a: f32| -> Pose {
        let (s, c) = a.sin_cos();
        let f: [f32; 9] = match axis {
            0 => [1.0, 0.0, 0.0, 0.0, c, s, 0.0, -s, c],
            1 => [c, 0.0, -s, 0.0, 1.0, 0.0, s, 0.0, c],
            _ => [c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0],
        };
        pose_from_rotation(f)
    };
    for axis in 0..3 {
        if rot[axis] != 0.0 {
            m = pose_mul(&m, &axis_rot(axis, rot[axis]));
        }
    }
    if scale != [1.0, 1.0, 1.0] {
        for c in 0..3 {
            for r in 0..4 {
                m.cols[c][r] *= scale[c];
            }
        }
    }
    m
}

/// One special layer's model-space quads — the part tree's resolved
/// polygons with the accumulated parent transforms composed outward-in
/// (v' = P_root · ... · P_part · v, each P the part_pose above) and the
/// dump's normalized uv scaled into the consumer's 0..16 sample domain
/// (the exact f32 exponent shift — x16 then /16 round-trips). The face
/// order IS the vanilla cube order (down, up, west, north, east, south —
/// the dump's polygon array order), so the quad's slot/direction ride
/// the face index. `shade` = true (the entity render's diffuse), the
/// tints are generation-time (the banner's pre-tinted variants), no
/// cullface (the entities render double-sided-aware, never culled by
/// the block's neighbors).
fn special_layer_quads(layer: &crate::generated::special_models::SpecialLayer, texture: &'static str) -> Vec<crate::element_quads::Quad> {
    use crate::generated::elements::Cull;
    let faces: [Cull; 6] = [
        Cull::Down,
        Cull::Up,
        Cull::West,
        Cull::North,
        Cull::East,
        Cull::South,
    ];
    let mut mats: Vec<Pose> = Vec::with_capacity(layer.parts.len());
    let mut quads = Vec::new();
    for (i, part) in layer.parts.iter().enumerate() {
        let own = part_pose(part.pose, part.rot, part.scale);
        let m = if part.parent < 0 || part.parent >= i as i32 {
            own
        } else {
            pose_mul(&mats[part.parent as usize], &own)
        };
        mats.push(m);
        if !part.visible {
            continue;
        }
        for cube in part.cubes {
            for (fi, face) in cube.faces.iter().enumerate() {
                let mut q = crate::element_quads::Quad {
                    vertices: [[0.0; 3]; 4],
                    uvs: [[0.0; 2]; 4],
                    slot: faces[fi],
                    direction: faces[fi],
                    texture,
                    cullface: None,
                    tintindex: None,
                    uv_rotation: 0,
                    shade: true,
                    light_emission: None,
                };
                for (vi, v) in face.verts.iter().enumerate() {
                    q.vertices[vi] = apply_vertex(&m, [v[0], v[1], v[2]]);
                    q.uvs[vi] = [v[3] * 16.0, v[4] * 16.0];
                }
                quads.push(q);
            }
        }
    }
    quads
}

/// The SPECIAL item's posed first-person quads — the dump-backed arc:
/// the item's special renderer layers (the running client's OWN
/// LayerDefinitions polygons through their part-tree transforms) carry
/// the tree-resolution's LOCAL transform (the renderer's own pose —
/// the `build_local_pose` decode, the raw-vertices-first law), then
/// the BASE model's display stanza on top (the applyTransform order —
/// exactly [`posed_item_layer_quads`]'s composition, the hand anchor
/// outside). Unknown id -> None; a broken base stanza -> Err; the
/// layers concatenate in the law's order (the conduit's shell -> cage
/// -> eye, the banner's rod -> tinted flag).
pub fn special_item_quads(
    id: i32,
    ctx: usize,
) -> Option<Result<Vec<crate::element_quads::Quad>, String>> {
    let render = crate::special_item_renderer(id)?;
    special_render_quads(&render, ctx)
}

/// The USING-branch special's posed first-person quads — the same
/// dump-backed arc through the USING state's own payload (the
/// shield_blocking / trident_throwing base poses the geometry, the
/// mirror transform rides the local decode). Unknown id or a
/// non-special using lane -> None; a broken base stanza -> Err.
pub fn special_item_using_quads(
    id: i32,
    ctx: usize,
) -> Option<Result<Vec<crate::element_quads::Quad>, String>> {
    // the using specials are single-state defs: the state carrying the
    // payload IS the branch (index 0 — the item_using_def census)
    let def = crate::item_using_def(id)?;
    let state = def.states.first()?;
    let sp = state.special?;
    let render = crate::special_render_from_def(sp)?;
    special_render_quads(&render, ctx)
}

/// The shared posed-quad body over one resolved special payload — the
/// dump-backed arc [`special_item_quads`] documents (the local transform
/// first, the base model's display stanza on top, the hand anchor
/// outside).
fn special_render_quads(
    render: &crate::SpecialItemRender,
    ctx: usize,
) -> Option<Result<Vec<crate::element_quads::Quad>, String>> {
    let transforms = match resolve_transforms(render.base) {
        Ok(t) => t,
        Err(e) => return Some(Err(e)),
    };
    let local = render.transform.map(build_local_pose);
    let left_hand = context_is_left_hand(ctx);
    let mut out = Vec::new();
    for layer in &render.layers {
        for mut q in special_layer_quads(layer.layer, layer.texture) {
            q.vertices = q
                .vertices
                .map(|v| {
                    let v = match &local {
                        Some(m) => apply_vertex(m, v),
                        None => v,
                    };
                    apply_transform(transforms[ctx].as_ref(), left_hand, v)
                });
            out.push(q);
        }
    }
    Some(Ok(out))
}

/// The GUI icon's posed quads — one (quad, SHADED, tint jsons) per
/// face. The law is the vanilla atlas arc: the display transform
/// resolves PER MODEL through its parent chain at the GUI context
/// (block/block's [30,225,0] x0.625 for the blocks, identity for the
/// flat sprites) and its own ItemTransform.apply tail carries the
/// -0.5 centering (the FaceBakery cornerToCenter law, already folded
/// in build_pose's step 4 — the same bake domain the hand lane pins);
/// the special payloads render through their dump geometry (the
/// entity models are authored centered — the dump IS the truth) and
/// shade like the ITEM_3D rig's first order. The element-derived
/// quads carry the directional shade law, the sprite extrusions ride
/// flat (ITEMS_FLAT), the tint jsons ride per layer for the client's
/// ItemColors law.
pub fn item_gui_quads(
    id: i32,
) -> Option<Result<Vec<(crate::element_quads::Quad, bool, &'static [&'static str])>, String>> {
    let name = crate::item_name(id)?;
    let layers: Vec<(&str, &'static [&'static str])> =
        if let Some((_, icon)) = crate::generated::item_models::ITEM_GUI_OVERRIDES
            .iter()
            .find(|d| d.0 == name)
        {
            if icon.special {
                // the payload through the GUI context: the dump's
                // geometry, the base model's gui stanza, the payload's
                // own local transform — the special_item_quads law
                let quads = match special_item_quads(id, GUI) {
                    Some(Ok(q)) => q,
                    Some(Err(e)) => return Some(Err(e)),
                    None => return None,
                };
                return Some(Ok(quads
                    .into_iter()
                    .map(|q| (q, true, &[] as &'static [&'static str]))
                    .collect()));
            }
            icon.models
                .iter()
                .map(|m| {
                    (
                        m.strip_prefix("minecraft:").unwrap_or(m),
                        &[] as &'static [&'static str],
                    )
                })
                .collect()
        } else {
            // the plain definition: its model path + its tint jsons
            let def = crate::generated::item_models::ITEM_MODEL_DEFS
                .iter()
                .find(|d| d.0 == name)?;
            vec![(
                def.1.strip_prefix("minecraft:").unwrap_or(def.1),
                def.2,
            )]
        };

    // the bake per model: the element chain first (the 3D block
    // geometry), the generated extrusion second (the flat sprites).
    // The centering needs NO fold here: the display transform's own
    // ItemTransform.apply sequence ENDS with translate(-0.5, -0.5,
    // -0.5) (build_pose's step 4 — the vanilla apply bytecode's own
    // tail; apply_transform(None) is the bare centering), so the
    // uncentered [0..1] bake enters the same law the hand lane rides.
    // The GUI display transform resolves per model's own parent chain.
    let mut out = Vec::new();
    for (model, tints) in layers {
        let transforms = match resolve_transforms(model) {
            Ok(t) => t,
            Err(e) => return Some(Err(e)),
        };
        let gui_t = transforms[GUI].as_ref();
        if let Some(quads) = crate::element_quads::derive_model_inherited(model) {
            for mut q in quads {
                q.vertices = q.vertices.map(|v| apply_transform(gui_t, false, v));
                out.push((q, true, tints));
            }
            continue;
        }
        if let Some(quads) = crate::item_bake::derive_item_model(model) {
            for mut q in quads {
                q.vertices = q.vertices.map(|v| apply_transform(gui_t, false, v));
                out.push((q, false, tints));
            }
            continue;
        }
        return Some(Err(format!("{model}: neither an element chain nor a generated bake")));
    }
    Some(Ok(out))
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
        // 483 explicit 45-multiples + 14 contexts that declare NO rotation
        // key (the deserializer default [0,0,0] = three multiples each):
        // 483 + 42 = 525 effective (scripts/display_census.py prints both).
        assert_eq!(rot_45, 525, "45-degree-multiple rotation entries");
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
        let bits: [u32; 3] = ORACLE_QUAD0_V0.map(|w| u32::from_str_radix(w, 16).unwrap());
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

    #[test]
    fn posed_item_quads_compose_the_baked_item_link() {
        // the whole-sprite front face is the bake's first emit; its first
        // block-unit corner through the handheld firstperson_righthand
        // pose, bit-pinned against the joml oracle fed the same vertex
        // (scripts/display_ref DisplayRef stdin mode).
        let posed = posed_item_quads("item/iron_sword", FIRSTPERSON_RIGHTHAND)
            .expect("iron_sword is in the bake domain")
            .expect("the handheld stanza resolves");
        assert_eq!(posed.len(), 82, "the census quad count");
        // the bake's pixel quads and the posed quads ride the same order,
        // uv, texture and slot; ONLY the vertices move
        let raw = crate::item_bake::derive_item_model("item/iron_sword").expect("bake domain");
        assert_eq!(raw.len(), posed.len());
        for (r, p) in raw.iter().zip(&posed) {
            assert_eq!(r.uvs, p.uvs, "the 0..16 uv space is untouched by the pose");
            assert_eq!(r.texture, p.texture, "the texture id rides");
            assert_eq!(r.slot, p.slot);
            assert_eq!(r.tintindex, p.tintindex);
        }
        // the chain pin: quad[0] vertex[0] (already block units — the
        // bake's faceinfo_corners applied the bakeVertex /16), then the
        // resolved fpr transform (item/handheld's stanza) — the exact
        // composition the fn performs
        let v0 = raw[0].vertices[0];
        let t = resolve_transforms("item/iron_sword")
            .expect("resolves")[FIRSTPERSON_RIGHTHAND]
            .expect("handheld declares firstperson_righthand");
        let expected = apply_transform(Some(&t), false, v0);
        assert_eq!(posed[0].vertices[0], expected);
        // the ORACLE bits for the same input vertex (filled from the
        // harness run below)
        let in_bits: [u32; 3] = v0.map(f32::to_bits);
        let bits: [u32; 3] = ORACLE_ITEM0_V0.map(|w| u32::from_str_radix(w, 16).unwrap());
        assert_eq!(
            posed[0].vertices[0].map(f32::to_bits),
            bits,
            "oracle stdin rot 0 -90 25 tr 1.13 3.2 1.13 scl 0.68 in {in_bits:x?}"
        );
        // view-space geometry sanity: every corner stays finite through
        // the whole chain (the anchored-region invariants live on the
        // client side, where the hand anchor's -0.72 z applies)
        for q in &posed {
            for p in &q.vertices {
                assert!(p.iter().all(|v| v.is_finite()), "finite corners");
            }
        }
        // the element bake covers the block models — stone rides the
        // cube chain's element (the vanilla hand's own two bake paths)
        let stone = posed_item_quads("block/stone", FIRSTPERSON_RIGHTHAND)
            .expect("the element bake covers the cube")
            .expect("no stanza error");
        assert_eq!(stone.len(), 6, "the full cube's six faces");
        // unknown -> None
        assert!(posed_item_quads("item/definitely_not_a_thing", FIRSTPERSON_RIGHTHAND).is_none());
    }

    /// display_ref oracle, stdin mode, the handheld fpr transform
    /// (rot [0,-90,25]/tr [1.13,3.2,1.13]/scl 0.68), item/iron_sword
    /// bake quad[0] vertex 0 = (0, 1, 0.53125) (block units — the bake's
    /// faceinfo_corners already applied the bakeVertex /16):
    /// "STDIN 3d4a3d70 3eba99c6 bec32df1".
    const ORACLE_ITEM0_V0: [&str; 3] = ["3d4a3d70", "3eba99c6", "bec32df1"];

    #[test]
    fn the_local_pose_rides_the_compose_bytecode_order() {
        // identity components: the pure translation — bit-exact
        let m = build_local_pose(([0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0]));
        let v = apply_vertex(&m, [0.25, -0.5, 0.125]);
        assert_eq!(v.map(f32::to_bits), [0.25f32.to_bits(), (-0.5f32).to_bits(), 1.125f32.to_bits()]);

        // the scale's full columns (row 3 included, the scaleGeneric pin)
        let m = build_local_pose(([0.0; 3], [0.0, 0.0, 0.0, 1.0], [2.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0]));
        let v = apply_vertex(&m, [1.0, 3.0, 5.0]);
        assert_eq!(v.map(f32::to_bits), [2.0f32.to_bits(), 3.0f32.to_bits(), 5.0f32.to_bits()]);

        // the 90°-about-Z right rotation maps x -> y (the quaternion's
        // [0,0,sin(45°),cos(45°)] — the JOML rotation() convention)
        let s = 0.5f32.sqrt();
        let qz = [0.0, 0.0, s, s];
        let m = build_local_pose(([0.0; 3], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0], qz));
        let v = apply_vertex(&m, [1.0, 0.0, 0.0]);
        assert!((v[0] - 0.0).abs() < 1e-6, "{v:?}");
        assert!((v[1] - 1.0).abs() < 1e-6, "{v:?}");
        assert!((v[2] - 0.0).abs() < 1e-6, "{v:?}");

        // THE ORDER PIN: translation LAST in the chain (T·L·S·R) — the
        // right rotation acts on the vertex FIRST, the translation rides
        // after: (1,0,0) -> R -> (0,1,0) -> T(+1,0,0) -> (1,1,0). The
        // reversed order (R·T) would land (0,2,0).
        let m = build_local_pose(([1.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0], qz));
        let v = apply_vertex(&m, [1.0, 0.0, 0.0]);
        assert!((v[0] - 1.0).abs() < 1e-6 && (v[1] - 1.0).abs() < 1e-6, "T·L·S·R order, got {v:?}");
    }

    #[test]
    fn the_layer_posing_applies_the_local_transform_under_the_display_pose() {
        // the identity transform is byte-identical to posed_item_quads
        let plain = posed_item_quads("item/iron_sword", FIRSTPERSON_RIGHTHAND)
            .expect("sword poses")
            .expect("stanza ok");
        let layered = posed_item_layer_quads(
            "item/iron_sword",
            None,
            FIRSTPERSON_RIGHTHAND,
        )
        .expect("sword poses")
        .expect("stanza ok");
        assert_eq!(plain, layered, "None = the raw vertex feed");

        // a real translation rides the vertices: the sword shifted +1 on
        // z before the display pose — the delta goes through the display
        // pose's LINEAR part, so the asserted shape is the posed-vs-posed
        // difference (same linear map on every quad)
        let shifted = posed_item_layer_quads(
            "item/iron_sword",
            Some(([0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0])),
            FIRSTPERSON_RIGHTHAND,
        )
        .expect("sword poses")
        .expect("stanza ok");
        assert_eq!(shifted.len(), plain.len());
        for (a, b) in plain.iter().zip(shifted.iter()) {
            let d0 = [
                b.vertices[0][0] - a.vertices[0][0],
                b.vertices[0][1] - a.vertices[0][1],
                b.vertices[0][2] - a.vertices[0][2],
            ];
            // the display pose's linear part maps (0,0,1) identically for
            // every vertex: quad[0]'s delta holds for all of them
            for (a, b) in plain.iter().zip(shifted.iter()) {
                for vi in 0..4 {
                    let d = [
                        b.vertices[vi][0] - a.vertices[vi][0],
                        b.vertices[vi][1] - a.vertices[vi][1],
                        b.vertices[vi][2] - a.vertices[vi][2],
                    ];
                    assert_eq!(d.map(f32::to_bits), d0.map(f32::to_bits));
                }
            }
            // the delta is nonzero (the translation actually rode)
            assert!(d0.iter().any(|d| *d != 0.0));
        }

        // unknown model -> None; the element bake covers the block
        // models (stone rides the cube chain's element)
        assert!(posed_item_layer_quads("item/definitely_not_a_thing", None, FIRSTPERSON_RIGHTHAND).is_none());
        let stone = posed_item_layer_quads("block/stone", None, FIRSTPERSON_RIGHTHAND)
            .expect("the element bake covers the cube")
            .expect("no stanza error");
        assert_eq!(stone.len(), 6, "the full cube's six faces");
    }

    #[test]
    fn the_special_layer_bake_is_the_dump_polygon_times_sixteen() {
        // the shield's geometry, RAW model space: 2 parts x 1 cube x 6
        // faces = 12 quads; the plate's TOP face first vertex is the
        // dump's own xyz (the part pose is identity), and the uv rides
        // the dump's normalized value scaled into the 0..16 sample
        // domain (13/64 -> 13/4 = 3.25, an exact f32 shift)
        let layer = crate::special_item_render_by_name("shield")
            .expect("shield")
            .layers
            .remove(0);
        let quads = special_layer_quads(layer.layer, layer.texture);
        assert_eq!(quads.len(), 12);
        let plate = layer.layer.parts.iter().find(|p| p.name == "plate").unwrap();
        let up = &plate.cubes[0].faces[1]; // the vanilla face order: down, up, ...
        let v0 = quads[6].vertices[0]; // part 0 = root (6 faces), part 1 = plate -> quad 6 = its first face (down), quad 6+? -- pin via the raw polygon instead
        let _ = v0;
        // find the plate's UP-face quad by its first vertex matching the
        // dump's raw xyz (identity pose -> bit-exact)
        let q = quads
            .iter()
            .find(|q| q.vertices[0] == [6.0, -11.0, -1.0])
            .expect("the plate's top face, raw");
        assert_eq!(q.texture, "entity/shield/shield_base_nopattern");
        assert_eq!(q.uvs[0], [13.0 / 64.0 * 16.0, 0.0]);
        assert_eq!(q.uvs[1], [1.0 / 64.0 * 16.0, 0.0]);
        // the parts: the root + the plate + the handle (the dump's tree)
        assert_eq!(layer.layer.parts.len(), 3);
    }

    #[test]
    fn the_special_item_quads_carry_the_display_stanza() {
        // the shield's first-person pose: the base model item/shield's
        // display stanza applies ON TOP of the raw bake (the
        // applyTransform law) — every vertex MOVES (the stanza's
        // translation is nonzero), the texture ids ride through
        let shield_id = crate::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == "shield")
            .expect("shield rides the wire") as i32;
        let quads = special_item_quads(shield_id, FIRSTPERSON_RIGHTHAND)
            .expect("the shield lane")
            .expect("the stanza parses");
        assert_eq!(quads.len(), 12);
        assert!(quads
            .iter()
            .all(|q| q.texture == "entity/shield/shield_base_nopattern"));
        // the raw bake's plate top v0 was [6, -11, -1]; the posed vertex
        // differs (the stanza's 0.68 scale + the translation) — the
        // display arc actually RAN
        let raw = [6.0f32, -11.0, -1.0];
        assert!(quads.iter().all(|q| q.vertices.iter().all(|v| *v != raw)));
        // the GUI context resolves too (the same geometry, the stanza's
        // gui branch) — and the unknown id is the honest None
        assert!(special_item_quads(shield_id, 5).is_some());
        assert!(special_item_quads(-1, FIRSTPERSON_RIGHTHAND).is_none());
    }

    #[test]
    fn the_gui_icon_chain_bakes_centered_and_shaded() {
        // the block item: the element bake, the cornerToCenter fold, the
        // block/block gui stanza — every vertex lands INSIDE the
        // [-0.6, 0.6] block box (the centered cube + the 0.625 scale),
        // the quads carry the directional shade law
        let cobble = crate::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == "cobblestone")
            .expect("cobblestone rides the wire") as i32;
        let quads = item_gui_quads(cobble).expect("the gui lane").expect("the bake");
        assert_eq!(quads.len(), 6, "the full cube's six faces");
        for (q, shaded, tints) in &quads {
            assert!(*shaded, "element-derived quads shade");
            assert!(tints.is_empty(), "cobblestone carries no tint");
            for v in q.vertices.iter() {
                assert!(v[0] >= -0.6 && v[0] <= 0.6, "x {} off the slot", v[0]);
                assert!(v[1] >= -0.6 && v[1] <= 0.6, "y {} off the slot", v[1]);
            }
        }
        // the flat sprite item: the generated extrusion, UNshaded
        let diamond = crate::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == "diamond")
            .expect("diamond rides the wire") as i32;
        let quads = item_gui_quads(diamond).expect("the gui lane").expect("the bake");
        assert!(quads.iter().all(|(_, shaded, _)| !shaded), "flat = ITEMS_FLAT");
        assert!(quads.iter().all(|(q, _, _)| {
            q.vertices.iter().all(|v| v[0] >= -0.5 && v[0] <= 0.5)
        }), "the centered sprite spans the slot exactly");
        // the trident: the gui override selects the 2D sprite model
        let trident = crate::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == "trident")
            .expect("trident rides the wire") as i32;
        let quads = item_gui_quads(trident).expect("the gui lane").expect("the bake");
        assert!(quads.iter().all(|(_, shaded, _)| !shaded));
        assert!(quads.iter().all(|(q, _, _)| q.texture == "item/trident"));
        // the shield: the SPECIAL payload through the gui context
        let shield = crate::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == "shield")
            .expect("shield rides the wire") as i32;
        let quads = item_gui_quads(shield).expect("the gui lane").expect("the bake");
        assert_eq!(quads.len(), 12);
        assert!(quads.iter().all(|(q, _, _)| q.texture == "entity/shield/shield_base_nopattern"));
        // the unknown id is the honest None
        assert!(item_gui_quads(-1).is_none());
    }
}

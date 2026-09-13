//! Skin generation: the official player skin (wide Steve) out of the client
//! jar — the data half of the first-person hand arc. The generated table
//! carries the source's integrity sha1s plus the vanilla `PlayerModel`
//! arm-box geometry (the UV unwrap the first-person arm renders with); the
//! PIXELS stream at runtime from the extracted asset root (the font-texture
//! pattern), sha1-verified against the constants the generator recorded.

use anyhow::{Context, Result};

use crate::extract::{decode_rgba, read_entry};

/// The jar entry the wide (classic Steve) skin lives at.
pub const WIDE_STEVE_JAR_PATH: &str = "assets/minecraft/textures/entity/player/wide/steve.png";
/// The jar entry the slim (Alex) skin lives at.
pub const SLIM_ALEX_JAR_PATH: &str = "assets/minecraft/textures/entity/player/slim/alex.png";
/// Where the client reads the skin at runtime (under the runtime asset root).
pub const RUNTIME_PATH: &str = "entity/player/wide/steve.png";
pub const SLIM_RUNTIME_PATH: &str = "entity/player/slim/alex.png";

/// The vanilla `PlayerModel` arm box for the wide model: texOffs (40, 16),
/// box 4x12x4 texels (w, h, d).
pub const ARM_TEX_OFFS: (u16, u16) = (40, 16);
pub const ARM_BOX_PX: [u16; 3] = [4, 12, 4];

/// The vanilla `PlayerModel` arm box for the SLIM model, javap-evidenced
/// from the official 26.2 jar: `PlayerModel.createMesh(CubeDeformation,
/// boolean slim = true)` — right_arm `texOffs(40, 16)` (UNCHANGED from the
/// wide model), `addBox(-2, -2, -2, 3, 12, 4)`, pivot `offset(-5, 2, 0)`
/// (UNCHANGED — the transform chain in the client is kind-independent).
pub const SLIM_ARM_BOX_PX: [u16; 3] = [3, 12, 4];
pub const SLIM_ARM_BOX_ORIGIN_PX: [i16; 3] = [-2, -2, -2];

/// The extracted skin facts the generated table carries.
pub struct SkinData {
    /// sha1 of the source PNG bytes (jar-verbatim integrity).
    pub png_sha1: String,
    /// sha1 of the decoded RGBA8 pixels (row-major, side*side*4 bytes).
    pub rgba_sha1: String,
    /// texture side (the classic skin layout is 64x64).
    pub side: u32,
    /// sha1 of the slim (Alex) source PNG bytes (jar-verbatim integrity).
    pub slim_png_sha1: String,
    /// sha1 of the decoded slim RGBA8 pixels.
    pub slim_rgba_sha1: String,
}

/// One UV rect in the skin texture, in texels: (u, v, w, h).
type Rect = (u16, u16, u16, u16);

/// The arm box's six faces in vanilla `ModelPart` unwrap order and sizes:
/// top, bottom, then the side strip right -> front -> left -> back.
pub fn arm_faces() -> [Rect; 6] {
    arm_faces_for(ARM_TEX_OFFS, ARM_BOX_PX)
}

/// The unwrap for ANY box: the same `ModelPart$Cube` layout, parameterized
/// by texOffs + (w, h, d) — the wide and slim arms share it (the slim arm's
/// only geometry deltas are the box width 4 -> 3 and the origin x
/// -3 -> -2; texOffs and the pivot are unchanged, javap).
pub fn arm_faces_for(tex_offs: (u16, u16), box_px: [u16; 3]) -> [Rect; 6] {
    let (u, v) = tex_offs;
    let [w, h, d] = box_px;
    [
        (u + d, v, w, d),             // top    (w x d)
        (u + d + w, v, w, d),         // bottom (w x d)
        (u, v + d, d, h),             // right  (d x h) — outer
        (u + d, v + d, w, h),         // front  (w x h)
        (u + d + w, v + d, d, h),     // left   (d x h) — inner
        (u + d + w + d, v + d, w, h), // back   (w x h)
    ]
}

/// Read both official skins out of the jar, decode them and pin their
/// integrity — the wide (classic) and the slim (Alex) variants.
pub fn generate<R: std::io::Read + std::io::Seek>(archive: &mut zip::ZipArchive<R>) -> Result<SkinData> {
    let bytes = read_entry(archive, WIDE_STEVE_JAR_PATH).context("read the wide Steve skin")?;
    let png_sha1 = crate::util::sha1_hex(&bytes);
    let (w, h, rgba) = decode_rgba(&bytes).context("decode the wide Steve skin")?;
    let side = skin_side(w, h)?;
    let slim_bytes = read_entry(archive, SLIM_ALEX_JAR_PATH).context("read the slim Alex skin")?;
    let slim_png_sha1 = crate::util::sha1_hex(&slim_bytes);
    let (sw, sh, slim_rgba) = decode_rgba(&slim_bytes).context("decode the slim Alex skin")?;
    let slim_side = skin_side(sw, sh)?;
    anyhow::ensure!(
        slim_side == side,
        "the slim skin must share the classic layout ({}x{}), got {}x{}",
        side,
        side,
        slim_side,
        slim_side
    );
    Ok(SkinData {
        png_sha1,
        rgba_sha1: crate::util::sha1_hex(&rgba),
        side,
        slim_png_sha1,
        slim_rgba_sha1: crate::util::sha1_hex(&slim_rgba),
    })
}

/// The layout checks both skins must pass: square and the classic 64x64.
fn skin_side(w: u32, h: u32) -> Result<u32> {
    anyhow::ensure!(w == h, "skin texture must be square, got {w}x{h}");
    anyhow::ensure!(
        w == 64,
        "the classic skin layout is 64x64 texels, got {w}x{h}"
    );
    Ok(w)
}

/// Emit `gen_skin.rs` content: the integrity sha1s + the arm-box geometry.
pub fn emit(data: &SkinData) -> Result<String> {
    let mut out = String::new();
    out.push_str("//! GENERATED by mc2rs-gen from the official player skin (wide Steve).\n");
    out.push_str(&format!(
        "//! Source: {WIDE_STEVE_JAR_PATH} (sha1 below), decoded to RGBA8 and\n//! hashed by the generator. Do not edit: regenerate with\n//! `cargo run -p mc2rs-gen -- skin --version <ver> --jar <client.jar>`.\n\n"
    ));
    out.push_str("/// sha1 of the verbatim source PNG inside the client jar.\n");
    out.push_str(&format!(
        "pub const STEVE_WIDE_PNG_SHA1: &str = \"{}\";\n",
        data.png_sha1
    ));
    out.push_str("/// sha1 over the decoded RGBA8 pixels (row-major, side*side*4 bytes)\n/// exactly as the generator's decoder produced them.\n");
    out.push_str(&format!(
        "pub const STEVE_WIDE_RGBA_SHA1: &str = \"{}\";\n\n",
        data.rgba_sha1
    ));
    out.push_str("/// The classic skin texture is 64x64 texels.\n");
    out.push_str(&format!("pub const SKIN_SIDE: u32 = {};\n\n", data.side));
    out.push_str("/// One UV rect in the skin texture, in texels.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub struct FaceRect {\n    pub u: u16,\n    pub v: u16,\n    pub w: u16,\n    pub h: u16,\n}\n\n");
    out.push_str("/// The vanilla `PlayerModel` arm box for the wide model: texOffs (40, 16),\n/// box 4x12x4 texels (w, h, d).\n");
    out.push_str("pub const ARM_TEX_OFFS: (u16, u16) = (40, 16);\n");
    out.push_str("pub const ARM_BOX_PX: [u16; 3] = [4, 12, 4];\n\n");
    out.push_str("/// The arm box's six faces in vanilla `ModelPart` unwrap order:\n/// top, bottom, then the side strip right -> front -> left -> back.\n");
    out.push_str("pub static ARM_FACES: [FaceRect; 6] = [\n");
    let names = ["top", "bottom", "right (outer)", "front", "left (inner)", "back"];
    for (name, (u, v, w, h)) in names.iter().zip(arm_faces()) {
        out.push_str(&format!(
            "    FaceRect {{ u: {u}, v: {v}, w: {w}, h: {h} }}, // {name}\n"
        ));
    }
    out.push_str("];\n\n");
    out.push_str("/// Index into ARM_FACES (vanilla cube face order).\n");
    for (i, name) in ["TOP", "BOTTOM", "RIGHT", "FRONT", "LEFT", "BACK"].iter().enumerate() {
        out.push_str(&format!("pub const FACE_{name}: usize = {i};\n"));
    }
    out.push_str("\n/// The runtime asset file the client reads (under the runtime asset root),\n/// sha1-verified against STEVE_WIDE_PNG_SHA1.\n");
    out.push_str(&format!("pub const RUNTIME_PATH: &str = \"{RUNTIME_PATH}\";\n"));
    out.push_str("\n/// The SLIM (Alex) skin, generated from the same jar:\n/// sha1s + the slim arm box (javap: `PlayerModel.createMesh` with\n/// `slim = true` — right_arm `texOffs(40, 16)` unchanged,\n/// `addBox(-2, -2, -2, 3, 12, 4)`, pivot `(-5, 2, 0)` unchanged).\n");
    out.push_str(&format!(
        "pub const SLIM_ALEX_PNG_SHA1: &str = \"{}\";\n",
        data.slim_png_sha1
    ));
    out.push_str(&format!(
        "pub const SLIM_ALEX_RGBA_SHA1: &str = \"{}\";\n",
        data.slim_rgba_sha1
    ));
    out.push_str("pub const SLIM_ARM_BOX_PX: [u16; 3] = [3, 12, 4];\n");
    out.push_str("pub const SLIM_ARM_BOX_ORIGIN_PX: [i16; 3] = [-2, -2, -2];\n");
    out.push_str("pub static SLIM_ARM_FACES: [FaceRect; 6] = [\n");
    for (name, (u, v, w, h)) in names.iter().zip(arm_faces_for(ARM_TEX_OFFS, SLIM_ARM_BOX_PX)) {
        out.push_str(&format!(
            "    FaceRect {{ u: {u}, v: {v}, w: {w}, h: {h} }}, // {name}\n"
        ));
    }
    out.push_str("];\n");
    out.push_str(&format!(
        "pub const SLIM_RUNTIME_PATH: &str = \"{SLIM_RUNTIME_PATH}\";\n"
    ));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zip_of(entries: &[(&str, Vec<u8>)]) -> zip::ZipArchive<std::io::Cursor<Vec<u8>>> {
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut w = zip::ZipWriter::new(&mut buf);
            let opts = zip::write::SimpleFileOptions::default();
            for (name, data) in entries {
                w.start_file(*name, opts).unwrap();
                std::io::Write::write_all(&mut w, data).unwrap();
            }
            w.finish().unwrap();
        }
        zip::ZipArchive::new(buf).unwrap()
    }

    /// A 64x64 RGBA png with the arm's top + front faces painted.
    fn skin_png() -> Vec<u8> {
        let (w, h) = (64u32, 64u32);
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        let put = |rgba: &mut [u8], x: u32, y: u32, c: [u8; 4]| {
            let o = ((y * w + x) * 4) as usize;
            rgba[o..o + 4].copy_from_slice(&c);
        };
        for y in 16..20 {
            for x in 44..48 {
                put(&mut rgba, x, y, [0, 175, 175, 255]);
            }
        }
        for y in 20..32 {
            for x in 44..48 {
                put(&mut rgba, x, y, [155, 99, 73, 255]);
            }
        }
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, w, h);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&rgba).unwrap();
        }
        out
    }

    #[test]
    fn parses_the_skins_and_emits_the_arm_geometry() {
        let png = skin_png();
        let expected_png_sha1 = crate::util::sha1_hex(&png);
        let (_, _, decoded) = crate::extract::decode_rgba(&png).unwrap();
        let expected_rgba_sha1 = crate::util::sha1_hex(&decoded);

        let mut zip = zip_of(&[
            (WIDE_STEVE_JAR_PATH, png.clone()),
            (SLIM_ALEX_JAR_PATH, png),
        ]);
        let data = generate(&mut zip).unwrap();
        assert_eq!(data.side, 64);
        assert_eq!(data.png_sha1, expected_png_sha1, "source bytes verbatim");
        assert_eq!(data.rgba_sha1, expected_rgba_sha1, "decode pinned");
        assert_eq!(data.slim_png_sha1, expected_png_sha1, "the fixture doubles as the slim skin");

        let emitted = emit(&data).unwrap();
        assert!(emitted.contains(&format!("pub const SKIN_SIDE: u32 = 64;")));
        assert!(emitted.contains(&format!("pub const STEVE_WIDE_PNG_SHA1: &str = \"{expected_png_sha1}\";")));
        assert!(emitted.contains(&format!("pub const SLIM_ALEX_PNG_SHA1: &str = \"{expected_png_sha1}\";")));
        assert!(emitted.contains("entity/player/wide/steve.png"));
        assert!(emitted.contains("entity/player/slim/alex.png"));
        assert!(emitted.contains("pub const SLIM_ARM_BOX_PX: [u16; 3] = [3, 12, 4];"));
        assert!(emitted.contains("pub const SLIM_ARM_BOX_ORIGIN_PX: [i16; 3] = [-2, -2, -2];"));
        // the six faces unfold from texOffs (40,16) with the box 4x12x4
        for (u, v, w, h) in arm_faces() {
            assert!(
                emitted.contains(&format!("FaceRect {{ u: {u}, v: {v}, w: {w}, h: {h} }}")),
                "face rect ({u},{v},{w},{h}) must be emitted verbatim"
            );
        }
        // and they stay inside the texture
        for (u, v, w, h) in arm_faces() {
            assert!(u32::from(u) + u32::from(w) <= 64);
            assert!(u32::from(v) + u32::from(h) <= 64);
        }
        // the SLIM unwrap: the same layout, box 3x12x4 — every rect inside
        // the texture, and the strip widths shift with w (front 3 wide,
        // left/inner at u+d+w, back at u+d+w+d)
        for (u, v, w, h) in arm_faces_for(ARM_TEX_OFFS, SLIM_ARM_BOX_PX) {
            assert!(emitted.contains(&format!("FaceRect {{ u: {u}, v: {v}, w: {w}, h: {h} }}")));
            assert!(u32::from(u) + u32::from(w) <= 64);
            assert!(u32::from(v) + u32::from(h) <= 64);
        }
        assert_eq!(
            arm_faces_for(ARM_TEX_OFFS, SLIM_ARM_BOX_PX),
            [
                (44, 16, 3, 4), // top
                (47, 16, 3, 4), // bottom
                (40, 20, 4, 12), // right (outer)
                (44, 20, 3, 12), // front
                (47, 20, 4, 12), // left (inner)
                (51, 20, 3, 12), // back
            ]
        );
    }

    #[test]
    fn rejects_an_undersized_skin() {
        // a 32x64 (non-square) png must be refused: the classic layout is 64x64
        let mut rgba = vec![0u8; 32 * 64 * 4];
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, 32, 64);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&mut rgba).unwrap();
        }
        let mut zip = zip_of(&[
            (WIDE_STEVE_JAR_PATH, out),
            (SLIM_ALEX_JAR_PATH, skin_png()),
        ]);
        assert!(generate(&mut zip).is_err(), "non-square skin must be refused");
    }

    #[test]
    fn refuses_a_zip_without_the_slim_skin() {
        // the slim table must not be invented: a jar missing alex.png fails
        // the generation (the data-driven rule — no fabricated defaults)
        let mut zip = zip_of(&[(WIDE_STEVE_JAR_PATH, skin_png())]);
        assert!(generate(&mut zip).is_err(), "missing slim skin must be refused");
    }
}

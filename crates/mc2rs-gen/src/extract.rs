//! Extraction helpers shared by every generator module: zip entry access,
//! PNG IHDR header parsing, and sprite metadata (.mcmeta) parsing.
//! All data is derived from the official jar / asset index at generation
//! time — nothing Mojang-owned is committed to the repository.

use std::io::Read;

use anyhow::{Context, Result};
use serde_json::Value;
use zip::ZipArchive;

/// Read a single file from the jar into memory.
pub fn read_entry(archive: &mut ZipArchive<std::io::Cursor<&[u8]>>, name: &str) -> Result<Vec<u8>> {
    let mut f = archive
        .by_name(name)
        .with_context(|| format!("jar entry {name} not found"))?;
    let mut out = Vec::new();
    f.read_to_end(&mut out)?;
    Ok(out)
}

/// Optionally read a file from the jar.
pub fn try_read_entry(archive: &mut ZipArchive<std::io::Cursor<&[u8]>>, name: &str) -> Option<Vec<u8>> {
    read_entry(archive, name).ok()
}

/// Parse the PNG IHDR chunk: returns (width, height). Works on any valid PNG
/// (IHDR is always the first chunk after the signature).
pub fn png_size(bytes: &[u8]) -> Result<(u32, u32)> {
    const SIG: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    anyhow::ensure!(bytes.len() >= 24, "png too short");
    anyhow::ensure!(bytes[..8] == SIG, "not a PNG");
    let w = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let h = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    Ok((w, h))
}

/// Parsed sprite metadata from a `.png.mcmeta` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpriteMeta {
    /// nine_slice scaling: (target_width, target_height, border)
    pub nine_slice: Option<(u32, u32, u32)>,
    /// simple scaling factor (width/height omitted -> vanilla treats as int)
    pub scale: Option<u32>,
}

/// Parse a `.mcmeta` next to a sprite png (vanilla `gui.scaling` layout).
pub fn parse_sprite_mcmeta(bytes: &[u8]) -> Result<SpriteMeta> {
    let v: Value = serde_json::from_slice(bytes).context("mcmeta json parse")?;
    let mut out = SpriteMeta { nine_slice: None, scale: None };
    if let Some(scaling) = v.get("gui").and_then(|g| g.get("scaling")) {
        match scaling.get("type").and_then(|t| t.as_str()) {
            Some("nine_slice") => {
                let w = scaling.get("width").and_then(|x| x.as_u64()).unwrap_or(0) as u32;
                let h = scaling.get("height").and_then(|x| x.as_u64()).unwrap_or(0) as u32;
                let b = match scaling.get("border") {
                    Some(Value::Number(n)) => n.as_u64().unwrap_or(0) as u32,
                    Some(Value::Object(o)) => {
                        // border can be an object {left,right,top,bottom}
                        let g = |k: &str| o.get(k).and_then(|x| x.as_u64()).unwrap_or(0) as u32;
                        g("left").max(g("right")).max(g("top")).max(g("bottom"))
                    }
                    _ => 0,
                };
                out.nine_slice = Some((w, h, b));
            }
            Some(_) | None => {
                if let Some(s) = scaling.get("scale").and_then(|x| x.as_u64()) {
                    out.scale = Some(s as u32);
                }
            }
        }
    }
    Ok(out)
}

/// Decode a PNG into RGBA8 (expanding palette/grayscale/16-bit forms).
pub fn decode_rgba(bytes: &[u8]) -> Result<(u32, u32, Vec<u8>)> {
    let decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().context("png read_info")?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).context("png decode")?;
    let w = info.width as usize;
    let h = info.height as usize;
    let samples = info.color_type.samples();
    let mut rgba = vec![255u8; w * h * 4];
    for p in 0..w * h {
        let s = &buf[p * samples..p * samples + samples];
        let o = p * 4;
        match samples {
            4 => rgba[o..o + 4].copy_from_slice(s),
            3 => {
                rgba[o] = s[0];
                rgba[o + 1] = s[1];
                rgba[o + 2] = s[2];
            }
            2 => {
                // grayscale + alpha
                rgba[o] = s[0];
                rgba[o + 1] = s[0];
                rgba[o + 2] = s[0];
                rgba[o + 3] = s[1];
            }
            _ => {
                rgba[o] = s[0];
                rgba[o + 1] = s[0];
                rgba[o + 2] = s[0];
            }
        }
    }
    Ok((info.width, info.height, rgba))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn png_size_parses_ihdr() {
        // minimal 1x1 grayscale PNG
        let png: &[u8] = &[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // signature
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR len+type
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1
        ];
        let (w, h) = png_size(png).unwrap();
        assert_eq!((w, h), (1, 1));
    }

    #[test]
    fn mcmeta_nine_slice_border() {
        let m = parse_sprite_mcmeta(
            br#"{"gui":{"scaling":{"type":"nine_slice","width":200,"height":20,"border":3}}}"#,
        )
        .unwrap();
        assert_eq!(m.nine_slice, Some((200, 20, 3)));
        assert_eq!(m.scale, None);
    }

    #[test]
    fn mcmeta_object_border_and_scale() {
        let m = parse_sprite_mcmeta(
            br#"{"gui":{"scaling":{"type":"nine_slice","width":16,"height":16,"border":{"left":2,"right":3,"top":2,"bottom":3}}}}"#,
        )
        .unwrap();
        assert_eq!(m.nine_slice, Some((16, 16, 3)));
        let m2 = parse_sprite_mcmeta(br#"{"gui":{"scaling":{"scale":2}}}"#).unwrap();
        assert_eq!(m2.scale, Some(2));
    }
}

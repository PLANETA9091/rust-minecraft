//! Real-color LUT: averages the actual game textures (when the asset cache
//! is present) into per-block/per-face colors for the reference renderer.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use mc_registry::generated::atlas;

use crate::colors::{texture_stem, FACE_NAMES};
use crate::world::*;

/// Nearest-neighbor texel lookup in the generated atlas (vanilla block
/// filtering). Returns RGBA in 0..1. Out-of-range UVs are clamped.
pub fn sample_tile(idx: u16, u: f32, v: f32) -> [f32; 4] {
    let rgba = atlas::tile_rgba(idx);
    let s = atlas::TILE_SIZE;
    let xi = ((u * s as f32).floor() as isize).clamp(0, s as isize - 1) as usize;
    let yi = ((v * s as f32).floor() as isize).clamp(0, s as isize - 1) as usize;
    let off = (yi * s + xi) * 4;
    [
        rgba[off] as f32 / 255.0,
        rgba[off + 1] as f32 / 255.0,
        rgba[off + 2] as f32 / 255.0,
        rgba[off + 3] as f32 / 255.0,
    ]
}

pub struct TextureLut {
    /// (kind, face) -> average color 0..1
    map: HashMap<(u16, usize), [f32; 3]>,
}

impl TextureLut {
    /// Build from an official asset index json + downloaded objects root.
    pub fn load(index_json: &Path, objects_root: &Path) -> Result<Self> {
        let index = mc_assets::AssetIndex::from_file(index_json)?;
        let mut map = HashMap::new();
        // every non-air palette member (the generated atlas is the kind
        // count's source of truth — new members resolve automatically)
        for kind in 1..atlas::N_KINDS as u16 {
            for face in 0..6usize {
                let Some(stem) = texture_stem(kind, face) else {
                    continue;
                };
                let key = format!("minecraft/textures/block/{stem}.png");
                let Some(obj) = index.objects.get(&key) else {
                    continue;
                };
                let path = objects_root.join("objects").join(&obj.hash[..2]).join(&obj.hash);
                if let Ok(bytes) = fs::read(&path) {
                    if let Some(avg) = avg_color(&bytes) {
                        map.insert((kind, face), avg);
                    }
                }
            }
        }
        if map.is_empty() {
            anyhow::bail!("no textures resolved from {}", index_json.display());
        }
        Ok(Self { map })
    }

    pub fn color(&self, kind: u16, face: usize) -> Option<[f32; 3]> {
        self.map.get(&(kind, face)).copied()
    }

    /// Human-readable dump for debugging.
    pub fn describe(&self) -> String {
        let mut keys: Vec<_> = self.map.keys().collect();
        keys.sort();
        keys.iter()
            .map(|(k, f)| format!("{}:{}", NAME_OF[*k as usize], FACE_NAMES[*f]))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Average color of a PNG (alpha-weighted, transparent pixels skipped).
pub fn avg_color(png_bytes: &[u8]) -> Option<[f32; 3]> {
    let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
    let mut reader = decoder.read_info().ok()?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).ok()?;
    let w = info.width as usize;
    let h = info.height as usize;
    let samples = info.color_type.samples();
    if w == 0 || h == 0 || samples < 3 {
        return None;
    }
    let mut r = 0f64;
    let mut g = 0f64;
    let mut b = 0f64;
    let mut n = 0f64;
    for py in 0..h {
        for px in 0..w {
            let off = (py * w + px) * samples;
            let (pr, pg, pb) = (buf[off] as f64, buf[off + 1] as f64, buf[off + 2] as f64);
            if samples == 4 {
                let a = buf[off + 3];
                if a < 128 {
                    continue; // transparent hole (leaves etc.)
                }
            }
            r += pr;
            g += pg;
            b += pb;
            n += 1.0;
        }
    }
    if n < 1.0 {
        return None;
    }
    Some([(r / n / 255.0) as f32, (g / n / 255.0) as f32, (b / n / 255.0) as f32])
}

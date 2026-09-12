//! Real-color LUT: averages the actual game textures (when the asset cache
//! is present) into per-block/per-face colors for the reference renderer.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::colors::{texture_stem, FACE_NAMES};
use crate::world::*;

pub struct TextureLut {
    /// (kind, face) -> average color 0..1
    map: HashMap<(u16, usize), [f32; 3]>,
}

impl TextureLut {
    /// Build from an official asset index json + downloaded objects root.
    pub fn load(index_json: &Path, objects_root: &Path) -> Result<Self> {
        let index = mc_assets::AssetIndex::from_file(index_json)?;
        let mut map = HashMap::new();
        for kind in 1..=7u16 {
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

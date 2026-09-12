//! PNG IO helpers (RGB output).

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use anyhow::{Context, Result};

pub fn write_rgb_png(path: &Path, w: usize, h: usize, rgb: &[u8]) -> Result<()> {
    let file = File::create(path).with_context(|| format!("create {}", path.display()))?;
    let bw = BufWriter::new(file);
    let mut enc = png::Encoder::new(bw, w as u32, h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    let mut writer = enc.write_header().context("png header")?;
    writer.write_image_data(rgb).context("png data")?;
    writer.finish().context("png finish")?;
    Ok(())
}

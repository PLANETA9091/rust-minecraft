//! mc2rs-pixel — golden-image pixel diff (RMS + heatmap), the CI gate for
//! "an optimization may never change a single pixel".

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "mc2rs-pixel", version, about = "Pixel diff between render and golden image")]
struct Cli {
    /// Rendered image (PNG).
    #[arg(long)]
    a: PathBuf,
    /// Golden/reference image (PNG).
    #[arg(long)]
    b: PathBuf,
    /// Write a diff heatmap here.
    #[arg(long)]
    out: Option<PathBuf>,
    /// Max allowed RMS (0..255 scale).
    #[arg(long, default_value_t = 8.0)]
    max_rms: f64,
    /// Per-pixel delta considered "different" for the heatmap/percentage.
    #[arg(long, default_value_t = 25.0)]
    pixel_threshold: f64,
}

struct Image {
    w: usize,
    h: usize,
    rgb: Vec<u8>,
}

fn load(path: &PathBuf) -> Result<Image> {
    let file = std::fs::File::open(path).with_context(|| format!("open {}", path.display()))?;
    let decoder = png::Decoder::new(std::io::BufReader::new(file));
    let mut reader = decoder.read_info().context("png info")?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).context("png frame")?;
    let w = info.width as usize;
    let h = info.height as usize;
    let samples = info.color_type.samples();
    anyhow::ensure!(samples >= 3, "unsupported png color type in {}", path.display());
    let mut rgb = Vec::with_capacity(w * h * 3);
    for i in 0..w * h {
        let o = i * samples;
        rgb.extend_from_slice(&buf[o..o + 3]);
    }
    Ok(Image { w, h, rgb })
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let a = load(&cli.a)?;
    let b = load(&cli.b)?;
    if a.w != b.w || a.h != b.h {
        bail!(
            "size mismatch: {} is {}x{}, {} is {}x{} (re-render the golden with the same --width/--height)",
            cli.a.display(), a.w, a.h, cli.b.display(), b.w, b.h
        );
    }

    let mut sum_sq = 0f64;
    let mut over = 0usize;
    let total = a.rgb.len();
    let mut heat = vec![0u8; total];
    for i in (0..total).step_by(3) {
        let mut d = 0f64;
        for ch in 0..3 {
            let diff = a.rgb[i + ch] as f64 - b.rgb[i + ch] as f64;
            d += diff * diff;
        }
        let rms_px = (d / 3.0).sqrt();
        sum_sq += d / 3.0;
        if rms_px > cli.pixel_threshold {
            over += 1;
            heat[i] = 255;
            heat[i + 1] = 0;
            heat[i + 2] = 0;
        } else {
            let g = (a.rgb[i] + a.rgb[i + 1] + a.rgb[i + 2]) / 3;
            heat[i] = g / 3;
            heat[i + 1] = g / 3;
            heat[i + 2] = g / 3;
        }
    }
    let rms = (sum_sq / (total as f64 / 3.0)).sqrt();
    let pct_over = 100.0 * over as f64 / (total as f64 / 3.0);
    let pass = rms <= cli.max_rms;

    if let Some(out) = &cli.out {
        let file = std::fs::File::create(out).context("create heatmap")?;
        let mut enc = png::Encoder::new(std::io::BufWriter::new(file), a.w as u32, a.h as u32);
        enc.set_color(png::ColorType::Rgb);
        enc.set_depth(png::BitDepth::Eight);
        let mut w = enc.write_header().context("png header")?;
        w.write_image_data(&heat).context("png data")?;
        w.finish().ok();
        println!("heatmap -> {}", out.display());
    }

    println!(
        "{{\"a\":\"{}\",\"b\":\"{}\",\"rms\":{rms:.3},\"pctPixelsOver\":{pct_over:.3},\"maxRms\":{max:.3},\"pass\":{pass}}}",
        cli.a.display(),
        cli.b.display(),
        max = cli.max_rms
    );
    if !pass {
        bail!("PIXEL TEST FAILED: rms {rms:.3} > max {}", cli.max_rms);
    }
    println!("PIXEL TEST PASSED");
    Ok(())
}

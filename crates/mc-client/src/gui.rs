//! 2D GUI layer: sprite blitting (alpha), vanilla nine-slice widgets and
//! text rendering driven by the GENERATED font tables (real glyph metrics,
//! real texture cells) with `§` color codes and shadow — same rules as the
//! original bitmap font renderer.

use crate::raster::Framebuf;

use mc_registry::generated::font;
use mc_registry::generated::gui;

/// Decode a PNG into RGBA8 (expands palette/grayscale/16-bit forms).
fn decode_png_rgba(bytes: &[u8]) -> anyhow::Result<(u32, u32, Vec<u8>)> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
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

/// Vanilla text palette (§0-§f), RGB 0..1.
pub const COLOR_CODES: [[f32; 3]; 16] = [
    [0.0, 0.0, 0.0],       // 0 black
    [0.0, 0.0, 0.667],     // 1 dark_blue
    [0.0, 0.667, 0.0],     // 2 dark_green
    [0.0, 0.667, 0.667],   // 3 dark_aqua
    [0.667, 0.0, 0.0],     // 4 dark_red
    [0.667, 0.0, 0.667],   // 5 dark_purple
    [1.0, 0.667, 0.0],     // 6 gold
    [0.667, 0.667, 0.667], // 7 gray
    [0.333, 0.333, 0.333], // 8 dark_gray
    [0.333, 0.333, 1.0],   // 9 blue
    [0.333, 1.0, 0.333],   // a green
    [0.333, 1.0, 1.0],     // b aqua
    [1.0, 0.333, 0.333],   // c red
    [1.0, 0.333, 1.0],     // d light_purple
    [1.0, 1.0, 0.333],     // e yellow
    [1.0, 1.0, 1.0],       // f white
];

/// A decoded RGBA sprite.
#[derive(Debug, Clone)]
pub struct Sprite {
    pub w: u32,
    pub h: u32,
    pub rgba: Vec<u8>,
}

impl Sprite {
    pub fn decode(bytes: &[u8]) -> anyhow::Result<Self> {
        let (w, h, rgba) = decode_png_rgba(bytes)?;
        Ok(Sprite { w, h, rgba })
    }

    /// Nearest sample; returns [r,g,b,a] 0..1.
    pub fn sample(&self, x: u32, y: u32) -> [f32; 4] {
        let x = x.min(self.w.max(1) - 1);
        let y = y.min(self.h.max(1) - 1);
        let o = ((y * self.w + x) * 4) as usize;
        [
            self.rgba[o] as f32 / 255.0,
            self.rgba[o + 1] as f32 / 255.0,
            self.rgba[o + 2] as f32 / 255.0,
            self.rgba[o + 3] as f32 / 255.0,
        ]
    }
}

/// Loads and caches sprites + font textures from the extracted runtime
/// assets (assets-cache/jar, populated by `mc2rs-gen generate`).
pub struct GuiAssets {
    root: std::path::PathBuf,
    sprites: std::collections::HashMap<&'static str, Sprite>,
    font_textures: Vec<Option<Sprite>>,
}

impl GuiAssets {
    pub fn open(root: impl Into<std::path::PathBuf>) -> Self {
        Self {
            root: root.into(),
            sprites: std::collections::HashMap::new(),
            font_textures: Vec::new(),
        }
    }

    /// Load a sprite by registry name (panics not; returns None when the
    /// file is absent — screens degrade gracefully).
    pub fn sprite(&mut self, name: &'static str) -> Option<&Sprite> {
        if !self.sprites.contains_key(name) {
            let def = gui::sprite(name)?;
            let path = self.root.join(&def.file);
            let bytes = std::fs::read(path).ok()?;
            let s = Sprite::decode(&bytes).ok()?;
            self.sprites.insert(name, s);
        }
        self.sprites.get(name)
    }

    fn font_texture(&mut self, idx: u16) -> Option<&Sprite> {
        if self.font_textures.len() <= idx as usize {
            self.font_textures.resize(idx as usize + 1, None);
        }
        if self.font_textures[idx as usize].is_none() {
            let file = font::FONT_TEXTURES.get(idx as usize)?;
            let path = self.root.join(file);
            let bytes = std::fs::read(path).ok()?;
            let s = Sprite::decode(&bytes).ok()?;
            self.font_textures[idx as usize] = Some(s);
        }
        self.font_textures[idx as usize].as_ref()
    }
}

/// Drawing context bound to a framebuffer with a GUI scale
/// (1 GUI unit = `scale` framebuffer pixels; vanilla GUI pixel).
pub struct Gui<'a> {
    pub fb: &'a mut Framebuf,
    pub scale: u32,
}

impl<'a> Gui<'a> {
    pub fn new(fb: &'a mut Framebuf, scale: u32) -> Self {
        Self { fb, scale }
    }

    /// Vanilla auto GUI scale: the largest s (1..=4) where the framebuffer
    /// still fits the minimum 320x240 GUI workspace, else 1.
    pub fn auto_scale(fb_w: usize, fb_h: usize) -> u32 {
        let mut best = 1u32;
        for s in 1..=4u32 {
            if fb_w / s as usize >= 320 && fb_h / s as usize >= 240 {
                best = s;
            }
        }
        best
    }

    /// GUI-space width of the framebuffer.
    pub fn w(&self) -> i32 {
        (self.fb.w / self.scale as usize) as i32
    }

    /// GUI-space height of the framebuffer.
    pub fn h(&self) -> i32 {
        (self.fb.h / self.scale as usize) as i32
    }

    /// Alpha-blend one GUI pixel.
    fn px(&mut self, x: i32, y: i32, c: [f32; 4]) {
        if x < 0 || y < 0 {
            return;
        }
        let (ux, uy) = (x as usize, y as usize);
        for sy in 0..self.scale {
            for sx in 0..self.scale {
                let fx = ux * self.scale as usize + sx as usize;
                let fy = uy * self.scale as usize + sy as usize;
                if fx >= self.fb.w || fy >= self.fb.h {
                    continue;
                }
                let o = (fy * self.fb.w + fx) * 3;
                let a = c[3];
                for ch in 0..3 {
                    let dst = self.fb.color[o + ch] as f32 / 255.0;
                    let v = dst * (1.0 - a) + c[ch] * a;
                    self.fb.color[o + ch] = (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
                }
            }
        }
    }

    /// Solid fill in GUI space, color RGB + alpha.
    pub fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, c: [f32; 4]) {
        for yy in y..y + h {
            for xx in x..x + w {
                self.px(xx, yy, c);
            }
        }
    }

    /// Blit a sprite 1:1 in GUI space (alpha blended).
    pub fn sprite(&mut self, a: &mut GuiAssets, name: &'static str, x: i32, y: i32) {
        self.sprite_scaled(a, name, x, y, 1);
    }

    /// Blit a sprite with an integer scale factor.
    pub fn sprite_scaled(&mut self, a: &mut GuiAssets, name: &'static str, x: i32, y: i32, k: u32) {
        if let Some(s) = a.sprite(name).cloned() {
            for sy in 0..s.h {
                for sx in 0..s.w {
                    let c = s.sample(sx, sy);
                    if c[3] <= 0.0 {
                        continue;
                    }
                    for ksy in 0..k {
                        for ksx in 0..k {
                            self.px(
                                x + (sx * k + ksx) as i32,
                                y + (sy * k + ksy) as i32,
                                c,
                            );
                        }
                    }
                }
            }
        }
    }

    /// Blit a sprite stretched to an arbitrary destination size (nearest
    /// sampling, like the GL_NEAREST path vanilla uses for the title logo /
    /// panorama). Down- and up-scaling both allowed.
    pub fn sprite_at(
        &mut self,
        a: &mut GuiAssets,
        name: &'static str,
        x: i32,
        y: i32,
        dw: i32,
        dh: i32,
    ) {
        let Some(s) = a.sprite(name).cloned() else { return };
        if dw <= 0 || dh <= 0 || s.w == 0 || s.h == 0 {
            return;
        }
        for dy in 0..dh {
            let sy = (dy as u32 * s.h / dh as u32).min(s.h - 1);
            for dx in 0..dw {
                let sx = (dx as u32 * s.w / dw as u32).min(s.w - 1);
                let c = s.sample(sx, sy);
                if c[3] <= 0.0 {
                    continue;
                }
                self.px(x + dx, y + dy, c);
            }
        }
    }

    /// Vanilla nine-slice blit per the sprite's .mcmeta border.
    pub fn sprite_9slice(
        &mut self,
        a: &mut GuiAssets,
        name: &'static str,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) {
        let (sprite, border) = {
            let Some(def) = gui::sprite(name) else { return };
            let Some((_, _, b)) = def.nine_slice else {
                self.sprite(a, name, x, y);
                return;
            };
            (a.sprite(name).cloned(), b as i32)
        };
        let Some(s) = sprite else { return };
        let b = border.min(s.w as i32 / 2).min(s.h as i32 / 2);
        // 9 pieces: (sx, sy, sw, sh) -> dest rect
        let pieces = [
            (0, 0, b, b, x, y, b, b),
            (s.w as i32 - b, 0, b, b, x + w - b, y, b, b),
            (0, s.h as i32 - b, b, b, x, y + h - b, b, b),
            (s.w as i32 - b, s.h as i32 - b, b, b, x + w - b, y + h - b, b, b),
            (b, 0, s.w as i32 - 2 * b, b, x + b, y, w - 2 * b, b),
            (b, s.h as i32 - b, s.w as i32 - 2 * b, b, x + b, y + h - b, w - 2 * b, b),
            (0, b, b, s.h as i32 - 2 * b, x, y + b, b, h - 2 * b),
            (s.w as i32 - b, b, b, s.h as i32 - 2 * b, x + w - b, y + b, b, h - 2 * b),
            (b, b, s.w as i32 - 2 * b, s.h as i32 - 2 * b, x + b, y + b, w - 2 * b, h - 2 * b),
        ];
        for (ssx, ssy, sw, sh, dx, dy, dw, dh) in pieces {
            if sw <= 0 || sh <= 0 || dw <= 0 || dh <= 0 {
                continue;
            }
            for dy2 in 0..dh {
                let syy = ssy + (dy2 * sh / dh);
                for dx2 in 0..dw {
                    let sxx = ssx + (dx2 * sw / dw);
                    let c = s.sample(sxx as u32, syy as u32);
                    if c[3] <= 0.0 {
                        continue;
                    }
                    self.px(dx + dx2, dy + dy2, c);
                }
            }
        }
    }

    /// Width of a string in GUI pixels (vanilla advance rule: glyph + 1px).
    pub fn text_width(&self, s: &str) -> i32 {
        let mut w = 0i32;
        let mut in_code = false;
        for ch in s.chars() {
            if in_code {
                in_code = false;
                continue;
            }
            if ch == '§' {
                in_code = true;
                continue;
            }
            w += match font::glyph(ch) {
                Some(g) if g.space_advance != 0 => g.space_advance as i32,
                Some(g) => g.gw as i32 + 1,
                None => 4,
            };
        }
        w
    }

    /// Draw text with `§` color codes; when `shadow`, first draw a darkened
    /// copy at (+1,+1) exactly like the original renderer.
    pub fn text(
        &mut self,
        a: &mut GuiAssets,
        x: i32,
        y: i32,
        s: &str,
        color: [f32; 3],
        shadow: bool,
    ) -> i32 {
        let dark = [color[0] * 0.25, color[1] * 0.25, color[2] * 0.25];
        if shadow {
            self.draw_text_raw(a, x + 1, y + 1, s, dark, false);
        }
        self.draw_text_raw(a, x, y, s, color, shadow)
    }

    fn draw_text_raw(
        &mut self,
        a: &mut GuiAssets,
        x: i32,
        y: i32,
        s: &str,
        mut color: [f32; 3],
        _shadow_capable: bool,
    ) -> i32 {
        let mut cx = x;
        let mut in_code = false;
        for ch in s.chars() {
            if in_code {
                if let Some(i) = code_index(ch) {
                    color = COLOR_CODES[i];
                }
                in_code = false;
                continue;
            }
            if ch == '§' {
                in_code = true;
                continue;
            }
            let Some(g) = font::glyph(ch) else {
                cx += 4;
                continue;
            };
            if g.space_advance != 0 {
                cx += g.space_advance as i32;
                continue;
            }
            if g.gw > 0 {
                if let Some(tex) = a.font_texture(g.tex).cloned() {
                    // baseline: glyph cell top = y + ascent - cell height
                    let top = y + g.ascent as i32 - g.gh as i32;
                    for gy in 0..g.gh {
                        for gx in 0..g.gw {
                            let c = tex.sample(
                                g.u as u32 + gx as u32,
                                g.v as u32 + gy as u32,
                            );
                            if c[3] <= 0.1 {
                                continue;
                            }
                            let a = c[3];
                            self.px(
                                cx + gx as i32,
                                top + gy as i32,
                                [color[0], color[1], color[2], a],
                            );
                        }
                    }
                }
            }
            cx += g.gw as i32 + 1;
        }
        cx
    }

    /// Centered text; returns the x it started at.
    pub fn text_centered(
        &mut self,
        a: &mut GuiAssets,
        cx: i32,
        y: i32,
        s: &str,
        color: [f32; 3],
        shadow: bool,
    ) -> i32 {
        let w = self.text_width(s);
        self.text(a, cx - w / 2, y, s, color, shadow)
    }

    /// Scaled text (the vanilla Screen.drawCenteredString under a
    /// pose().scale(k) — the DeathScreen's 2x title): every glyph
    /// pixel blits as a k×k block, the advances and the shadow offset
    /// scale with it. Returns the end x in GUI units.
    pub fn text_scaled(
        &mut self,
        a: &mut GuiAssets,
        x: i32,
        y: i32,
        s: &str,
        color: [f32; 3],
        shadow: bool,
        k: i32,
    ) -> i32 {
        let dark = [color[0] * 0.25, color[1] * 0.25, color[2] * 0.25];
        if shadow {
            self.draw_text_scaled_raw(a, x + k, y + k, s, dark, k);
        }
        self.draw_text_scaled_raw(a, x, y, s, color, k)
    }

    fn draw_text_scaled_raw(
        &mut self,
        a: &mut GuiAssets,
        x: i32,
        y: i32,
        s: &str,
        mut color: [f32; 3],
        k: i32,
    ) -> i32 {
        let mut cx = x;
        let mut in_code = false;
        for ch in s.chars() {
            if in_code {
                if let Some(i) = code_index(ch) {
                    color = COLOR_CODES[i];
                }
                in_code = false;
                continue;
            }
            if ch == '§' {
                in_code = true;
                continue;
            }
            let Some(g) = font::glyph(ch) else {
                cx += 4 * k;
                continue;
            };
            if g.space_advance != 0 {
                cx += g.space_advance as i32 * k;
                continue;
            }
            if g.gw > 0 {
                if let Some(tex) = a.font_texture(g.tex).cloned() {
                    // baseline: glyph cell top = y + ascent*k - cell height*k
                    let top = y + g.ascent as i32 * k - g.gh as i32 * k;
                    for gy in 0..g.gh {
                        for gx in 0..g.gw {
                            let c = tex.sample(
                                g.u as u32 + gx as u32,
                                g.v as u32 + gy as u32,
                            );
                            if c[3] <= 0.1 {
                                continue;
                            }
                            let al = c[3];
                            for sy in 0..k {
                                for sx in 0..k {
                                    self.px(
                                        cx + gx as i32 * k + sx,
                                        top + gy as i32 * k + sy,
                                        [color[0], color[1], color[2], al],
                                    );
                                }
                            }
                        }
                    }
                }
            }
            cx += (g.gw as i32 + 1) * k;
        }
        cx
    }

    /// Vanilla mouse cursor (crosshair-style arrow fallback: simple arrow).
    #[allow(dead_code)] // wired in when the screens take pointer input
    pub fn cursor(&mut self, x: i32, y: i32) {
        let c = [1.0, 1.0, 1.0, 0.9];
        for i in 0..8 {
            for j in 0..=i.min(3) {
                self.px(x + i, y + j, c);
            }
        }
        for i in 0..4 {
            self.px(x + 1 + i, y + 8 - i + i / 2, c);
        }
    }
}

fn code_index(ch: char) -> Option<usize> {
    match ch {
        '0'..='9' => Some(ch as usize - '0' as usize),
        'a'..='f' | 'A'..='F' => {
            let lo = ch.to_ascii_lowercase();
            Some(lo as usize - 'a' as usize + 10)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_width_uses_real_metrics() {
        let g = Gui { fb: &mut Framebuf::new(64, 64), scale: 2 };
        // space advance is 4 per the official space provider
        assert_eq!(g.text_width(" "), 4);
        assert_eq!(g.text_width("  "), 8);
        // any missing glyph falls back to 4
        assert_eq!(g.text_width("\u{10FFFE}"), 4);
    }

    #[test]
    fn color_code_parsing() {
        assert_eq!(code_index('f'), Some(15));
        assert_eq!(code_index('0'), Some(0));
        assert_eq!(code_index('A'), Some(10));
        assert_eq!(code_index('g'), None);
        // white is white
        assert_eq!(COLOR_CODES[15], [1.0, 1.0, 1.0]);
    }

    #[test]
    fn gui_space_matches_scale() {
        let fb = Framebuf::new(320, 180);
        let g = Gui { fb: &mut Framebuf::new(320, 180), scale: 2 };
        assert_eq!(g.w(), 160);
        assert_eq!(g.h(), 90);
        drop(fb);
    }

    #[test]
    fn sprite_at_stretches_and_skips_transparent() {
        let dir = std::env::temp_dir().join(format!("mc2rs-gui-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("gui/sprites/widget")).unwrap();
        // 2x1 sprite: opaque red left half, transparent right half
        let mut rgba = vec![0u8; 8];
        rgba[0] = 255;
        rgba[3] = 255;
        let mut out = Vec::new();
        {
            let mut enc = png::Encoder::new(&mut out, 2, 1);
            enc.set_color(png::ColorType::Rgba);
            enc.set_depth(png::BitDepth::Eight);
            let mut wr = enc.write_header().unwrap();
            wr.write_image_data(&rgba).unwrap();
        }
        std::fs::write(dir.join("gui/sprites/widget/button.png"), &out).unwrap();

        let mut fb = Framebuf::new(16, 8);
        fb.clear([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        let mut g = Gui::new(&mut fb, 1);
        let mut a = GuiAssets::open(&dir);
        // stretch 2x1 -> 4x2; sx mapping: dx 0,1 -> src 0 (red), dx 2,3 -> src 1 (clear)
        g.sprite_at(&mut a, "widget/button", 0, 0, 4, 2);
        assert_eq!(g.fb.color[(0 * 16 + 0) * 3], 255, "left half red");
        assert_eq!(g.fb.color[(0 * 16 + 1) * 3], 255, "left half red");
        assert_eq!(g.fb.color[(0 * 16 + 3) * 3], 0, "transparent half skipped");
        assert_eq!(g.fb.color[(1 * 16 + 0) * 3], 255, "second dest row red");
        std::fs::remove_dir_all(&dir).ok();
    }
}

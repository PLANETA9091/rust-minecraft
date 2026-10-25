use super::*;

/// THE TEXT STYLES' OWN VERDICTS (the effect laws the 26.2
/// bytecode speaks: the bold the second-glyph advance +1 wears,
/// the color switch the flags clear, the obfuscated the consumed
/// skip keeps, the italic the per-row shear speaks, the bars the
/// strikethrough/underline rows wear, the wrap the bold re-opens).
#[test]
fn the_text_styles_the_flags_the_jar_laws_speak() {
    // THE WIDTH'S OWN LAW (the GlyphInfo.getAdvance(bold) the +1
    // boldOffset speaks; the color switch the flags clear — the
    // second glyph the unbolded advance wears; the obfuscated the
    // consumed skip keeps)
    assert_eq!(
        crate::gui::font_text_width("\u{a7}lA"),
        crate::gui::font_text_width("A") + 1
    );
    assert_eq!(
        crate::gui::font_text_width("\u{a7}lA\u{a7}cB"),
        crate::gui::font_text_width("A") + 1 + crate::gui::font_text_width("B")
    );
    assert_eq!(
        crate::gui::font_text_width("\u{a7}kA"),
        crate::gui::font_text_width("A")
    );
    // THE ITALIC'S OWN SHEAR (the per-row fold the linear walk
    // speaks: up = -1 the standard cell carries — the top rows the
    // +1 lean wears, the bottom rows the upright keeps)
    let expected = [1, 1, 1, 1, 0, 0, 0, 0];
    for (gy, shift) in expected.iter().enumerate() {
        assert_eq!(
            crate::gui::StyleState::italic_row_shift(-1, gy as i32),
            *shift,
            "the italic row {gy} the shear law keeps"
        );
    }
    // THE WRAP'S OWN RE-OPEN (the bold the wrapped line carries —
    // the style the line break never strips)
    let bolded = "\u{a7}l".to_string() + &"the quick brown fox jumps over the lazy dog".repeat(3);
    let wrapped = wrap_chat_text(&bolded, 320);
    assert!(wrapped.len() > 1, "the bolded line the wrap splits");
    for seg in &wrapped[1..] {
        assert!(
            seg.starts_with("\u{a7}l"),
            "the wrapped line the bold re-opens: {seg}"
        );
    }

    // THE PIXELS' OWN VERDICT (the harness the chat lane keeps —
    // the bold's doubled column, the bars' rows, the lean's
    // half-split the integer fold speaks)
    fn diff_count(a: &[u8], b: &[u8], x0: i32, y0: i32, w: i32, h: i32) -> usize {
        let mut n = 0usize;
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                let o = ((y as usize) * 640 + x as usize) * 3;
                if a[o..o + 3] != b[o..o + 3] {
                    n += 1;
                }
            }
        }
        n
    }
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/container/inventory.png").exists());
    let Some(root) = root else { return };
    let mut app = App::new(3);
    app.enter_world();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let y = chat_line_y(360, 0, 0.0);
    // THE BOLD (the second-glyph copy the +1 offset wears — the
    // columns the plain line never paints)
    app.push_chat_message("\u{a7}lIIII".to_string());
    app.render(&mut fb, &mut assets);
    let bold_frame = fb.color.clone();
    app.chat_messages[0].0 = "IIII".to_string();
    app.render(&mut fb, &mut assets);
    let plain_frame = fb.color.clone();
    let bold_diff = diff_count(&bold_frame, &plain_frame, 2, y - 1, 120, 10);
    assert!(
        bold_diff > 8,
        "the bold the second pass paints: {bold_diff}"
    );
    // THE UNDERLINE (the [y+8, y+9) row the effect quad wears —
    // the row the text's own baseline gap keeps bare)
    app.chat_messages[0].0 = "\u{a7}nAAAA".to_string();
    app.render(&mut fb, &mut assets);
    let underlined = fb.color.clone();
    app.chat_messages[0].0 = "AAAA".to_string();
    app.render(&mut fb, &mut assets);
    let plain2 = fb.color.clone();
    let under_diff = diff_count(&underlined, &plain2, 2, y + 8, 120, 1);
    assert!(
        under_diff > 20,
        "the underline the row paints: {under_diff}"
    );
    let gap_diff = diff_count(&underlined, &plain2, 2, y + 7, 120, 1);
    assert_eq!(gap_diff, 0, "the gap row the underline never touches");
    // THE STRIKETHROUGH (the [y+3.5, y+4.5) band's floor fold —
    // the row the glyph bodies and the bar both paint)
    app.chat_messages[0].0 = "\u{a7}mAAAA".to_string();
    app.render(&mut fb, &mut assets);
    let struck = fb.color.clone();
    let strike_diff = diff_count(&struck, &plain2, 2, y + 3, 120, 1);
    assert!(
        strike_diff > 10,
        "the strikethrough the row paints: {strike_diff}"
    );
    // THE ITALIC (the lean the per-row shear speaks — the top
    // rows the +1 shift wears, the bottom rows the upright keeps)
    app.chat_messages[0].0 = "\u{a7}oMMMMM".to_string();
    app.render(&mut fb, &mut assets);
    let italic = fb.color.clone();
    app.chat_messages[0].0 = "MMMMM".to_string();
    app.render(&mut fb, &mut assets);
    let plain3 = fb.color.clone();
    let top_diff = diff_count(&italic, &plain3, 2, y - 1, 200, 4);
    assert!(
        top_diff > 10,
        "the italic top rows the shear shifts: {top_diff}"
    );
    let bottom_diff = diff_count(&italic, &plain3, 2, y + 3, 200, 4);
    assert_eq!(bottom_diff, 0, "the italic bottom rows the upright keeps");
}

/// THE OBFUSCATED WALK'S OWN VERDICTS (javap `Font.getGlyph` +
/// `FontSet.getRandomGlyph` — the 26.2 random-glyph law the draw
/// answers: the pick the SAME-width class draws, the (seed,
/// position) fold the frame re-rolls, the shadow the SAME glyphs'
/// dimmed copy wears (the vanilla preparation's ONE pick), the
/// color switch the obfuscated flag kills, the wrap the §k
/// re-opens).
#[test]
fn the_obfuscated_walk_the_random_glyphs_speak() {
    // THE WIDTH'S OWN PIN (the class the advance never changes —
    // integer advances make the ceil the identity, the pick the
    // SAME advance class draws)
    assert_eq!(
        crate::gui::font_text_width("\u{a7}kAA"),
        crate::gui::font_text_width("AA")
    );
    // THE WRAP'S OWN RE-OPEN (the wrapped line the §k carries —
    // the style the line break never strips)
    let obfed = "\u{a7}k".to_string() + &"the quick brown fox jumps over the lazy dog".repeat(3);
    let wrapped = wrap_chat_text(&obfed, 320);
    assert!(wrapped.len() > 1, "the obfuscated line the wrap splits");
    for seg in &wrapped[1..] {
        assert!(
            seg.starts_with("\u{a7}k"),
            "the wrapped line the §k re-opens: {seg}"
        );
    }

    // THE PIXELS' OWN VERDICTS (the harness the style tests keep)
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/container/inventory.png").exists());
    let Some(root) = root else { return };
    let mut assets = GuiAssets::open(&root);
    fn draw(assets: &mut GuiAssets, s: &str, seed: u64, shadow: bool) -> crate::raster::Framebuf {
        let mut fb = crate::raster::Framebuf::new(200, 40);
        let mut g = Gui::new(&mut fb, 1).with_scramble(seed);
        g.text_faded(assets, 5, 5, s, [1.0, 1.0, 1.0], 1.0, shadow);
        fb
    }
    // THE DETERMINISM LAW (the same seed the same glyphs paint —
    // the pure (seed, position) fold, never a leaked stream)
    let a = draw(&mut assets, "\u{a7}kAAAAA", 7, false);
    let b = draw(&mut assets, "\u{a7}kAAAAA", 7, false);
    assert_eq!(a.color, b.color, "the same seed the same glyphs wear");
    // THE SCRAMBLE + ADVANCE LAWS (the seeds the picks re-roll —
    // the vanilla shared stream the frame counter answers; the
    // end-x the class pins — the integer advance the pick never
    // changes)
    let mut end_xs = std::collections::HashSet::new();
    let mut frames: Vec<Vec<u8>> = Vec::new();
    for seed in 0..4u64 {
        let mut fb = crate::raster::Framebuf::new(200, 40);
        let mut g = Gui::new(&mut fb, 1).with_scramble(seed);
        let end = g.text_faded(
            &mut assets,
            5,
            5,
            "\u{a7}kAAAAA",
            [1.0, 1.0, 1.0],
            1.0,
            false,
        );
        end_xs.insert(end);
        frames.push(fb.color.clone());
    }
    assert_eq!(end_xs.len(), 1, "the class the advance pins: {end_xs:?}");
    let mut differing_pairs = 0usize;
    for i in 0..frames.len() {
        for j in (i + 1)..frames.len() {
            let diff = (0..40).any(|y| {
                (0..200).any(|x| {
                    let o = (y * 200 + x) * 3;
                    frames[i][o..o + 3] != frames[j][o..o + 3]
                })
            });
            if diff {
                differing_pairs += 1;
            }
        }
    }
    assert!(
        differing_pairs >= 3,
        "the seeds the scramble re-rolls: {differing_pairs}"
    );
    // THE COLOR-CLEAR LAW (the §c the obfuscated flag kills — the
    // fresh style's five valueOf(0) — the frame the plain colored
    // line's own bytes keeps)
    let cleared = draw(&mut assets, "\u{a7}k\u{a7}cA", 3, false);
    let plain = draw(&mut assets, "\u{a7}cA", 3, false);
    assert_eq!(
        cleared.color, plain.color,
        "the color switch the obf clears"
    );
    // THE SHADOW'S SAME-GLYPH LAW (the shadow pass the SAME picks
    // walks — every shadow-only pixel the main glyph's (+1,+1) copy
    // answers; a mismatched pick would paint shadows the main line
    // never carries)
    let shadowed = draw(&mut assets, "\u{a7}kAAAAA", 7, true);
    let main = draw(&mut assets, "\u{a7}kAAAAA", 7, false);
    let px = |fb: &crate::raster::Framebuf, x: usize, y: usize| -> [f32; 3] {
        let o = (y * 200 + x) * 3;
        [
            fb.color[o] as f32 / 255.0,
            fb.color[o + 1] as f32 / 255.0,
            fb.color[o + 2] as f32 / 255.0,
        ]
    };
    let ink = |p: [f32; 3]| p[0] > 0.02 || p[1] > 0.02 || p[2] > 0.02;
    let mut shadow_only = 0usize;
    for y in 1..40usize {
        for x in 1..200usize {
            let sh = px(&shadowed, x, y);
            if ink(sh) && !ink(px(&main, x, y)) {
                shadow_only += 1;
                assert!(
                    ink(px(&main, x - 1, y - 1)),
                    "the shadow pixel ({x},{y}) the main glyph's copy answers"
                );
                assert!(sh[0] < 0.6, "the shadow the 0.25 dim keeps: {sh:?}");
            }
        }
    }
    assert!(
        shadow_only > 10,
        "the shadow pixels the same glyphs paint: {shadow_only}"
    );
}

/// THE TRUE HEX'S OWN WALK (the `§x§r§r§g§g§b§b` form the legacy
/// alphabet's RGB lane speaks — the scan the strict six pairs
/// walks, the exact value the draw paints, the width the fourteen
/// code chars never grows, the malformed form the unknown-code
/// law's own consumption answers, the wrap the §x re-open carries,
/// the scaled lane the SAME resolved rgb paints).
#[test]
fn the_true_hex_the_exact_rgb_walk_speaks() {
    const PINK: &str = "\u{a7}x\u{a7}f\u{a7}f\u{a7}0\u{a7}0\u{a7}8\u{a7}0";
    // THE WIDTH'S OWN LAW (the 14 code chars the draw never paints —
    // the hex walk the zero advance keeps)
    assert_eq!(
        crate::gui::font_text_width(&format!("{PINK}A")),
        crate::gui::font_text_width("A")
    );
    // THE MALFORMED (the pairs the strict form never completes —
    // the §x the unknown-code law consumes, the trailing pairs the
    // named codes the old walk answers, the width the glyph's own
    // advance keeps)
    assert_eq!(
        crate::gui::font_text_width("\u{a7}x\u{a7}f\u{a7}fA"),
        crate::gui::font_text_width("A")
    );
    // THE WRAP'S OWN RE-OPEN (the wrapped line the §x form re-opens
    // — the style the line break never strips)
    let hexed = format!("{PINK}{}", "the quick brown fox jumps ".repeat(12));
    let wrapped = wrap_chat_text(&hexed, 320);
    assert!(wrapped.len() > 1, "the hex line the wrap splits");
    for seg in &wrapped[1..] {
        assert!(
            seg.starts_with(PINK),
            "the wrapped line the §x re-opens: {seg}"
        );
    }

    // THE PIXELS' OWN VERDICTS (the harness the style tests keep)
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/container/inventory.png").exists());
    let Some(root) = root else { return };
    let mut assets = GuiAssets::open(&root);
    let px = |fb: &crate::raster::Framebuf, x: usize, y: usize| -> [f32; 3] {
        let o = (y * 200 + x) * 3;
        [
            fb.color[o] as f32 / 255.0,
            fb.color[o + 1] as f32 / 255.0,
            fb.color[o + 2] as f32 / 255.0,
        ]
    };
    // THE EXACT RGB (the 0xFF0080 the glyph's ink wears — the
    // blends over the black ground the alpha scale ONLY, the g the
    // zero the every ink pixel keeps, the full-alpha core the exact
    // triple speaks; the named palette the 16 codes never speak it)
    let draw = |assets: &mut GuiAssets, s: &str, shadow: bool| {
        let mut fb = crate::raster::Framebuf::new(200, 40);
        let mut g = Gui::new(&mut fb, 1);
        g.text_faded(assets, 5, 5, s, [1.0, 1.0, 1.0], 1.0, shadow);
        fb
    };
    let hexed = draw(&mut assets, &format!("{PINK}HHHH"), false);
    let mut ink = 0usize;
    let mut exact = 0usize;
    for y in 0..40usize {
        for x in 0..200usize {
            let p = px(&hexed, x, y);
            if p[0] > 0.02 || p[1] > 0.02 || p[2] > 0.02 {
                ink += 1;
                assert!(
                    p[1] <= 0.02,
                    "the hex the green channel never wears: ({x},{y}) {p:?}"
                );
                if (p[0] - 1.0).abs() < 1.0 / 255.0 && (p[2] - 128.0 / 255.0).abs() <= 1.0 / 255.0 {
                    exact += 1;
                }
            }
        }
    }
    assert!(ink > 20, "the HHHH the hex paints: {ink}");
    assert!(
        exact > 5,
        "the full-alpha core the exact 0xFF0080 paints: {exact}"
    );
    // THE FRESH-CLEAR (the bold BEFORE the hex the switch kills —
    // the frame the plain hex line's own bytes wears; the flag
    // AFTER the hex the accumulation keeps)
    let cleared = draw(&mut assets, &format!("\u{a7}l{PINK}H"), false);
    let plain = draw(&mut assets, &format!("{PINK}H"), false);
    assert_eq!(cleared.color, plain.color, "the hex switch the bold clears");
    let flagged = draw(&mut assets, &format!("{PINK}\u{a7}lH"), false);
    assert_ne!(flagged.color, plain.color, "the flag after the hex rides");
    // THE NAMED SWITCH (the §c after the hex the palette answers —
    // the frame the plain colored line's own bytes wears)
    let named = draw(&mut assets, &format!("{PINK}\u{a7}cH"), false);
    let red = draw(&mut assets, "\u{a7}cH", false);
    assert_eq!(named.color, red.color, "the named switch the rgb clears");
    // THE SHADOW (the dim the hex color also wears — the
    // shadow-only pixel the 0.25 fold darkens, the green the zero
    // the blend keeps)
    let shadowed = draw(&mut assets, &format!("{PINK}HHHH"), true);
    let mut shadow_only = 0usize;
    for y in 1..40usize {
        for x in 1..200usize {
            let p = px(&shadowed, x, y);
            let m = px(&hexed, x, y);
            let inked = p[0] > 0.02 || p[2] > 0.02;
            let main_inked = m[0] > 0.02 || m[2] > 0.02;
            if inked && !main_inked {
                shadow_only += 1;
                assert!(
                    p[0] < 0.35 && p[2] < 0.2,
                    "the shadow the 0.25 dim keeps: {p:?}"
                );
            }
        }
    }
    assert!(
        shadow_only > 5,
        "the shadow pixels the hex paints: {shadow_only}"
    );
    // THE SCALED LANE (the k-block the SAME resolved rgb paints)
    let mut fb = crate::raster::Framebuf::new(200, 40);
    let mut g = Gui::new(&mut fb, 1);
    g.text_scaled(
        &mut assets,
        5,
        5,
        &format!("{PINK}H"),
        [1.0, 1.0, 1.0],
        false,
        2,
    );
    let mut scaled_ink = 0usize;
    for y in 0..40usize {
        for x in 0..200usize {
            let p = px(&fb, x, y);
            if p[0] > 0.02 || p[2] > 0.02 {
                scaled_ink += 1;
                assert!(p[1] <= 0.02, "the scaled lane the green keeps: {p:?}");
            }
        }
    }
    assert!(scaled_ink > 20, "the scaled H the hex paints: {scaled_ink}");
}

#[test]
fn the_f2_key_grabs_the_frame() {
    // THE NAME'S OWN LAW (the Screenshot.getFile walk — the
    // yyyy-MM-dd_HH.mm.ss classic format the civil calendar
    // speaks, the _N dedup the concat recipe banks)
    assert_eq!(
        screenshot_file_name(0, &|_| false),
        "1970-01-01_00.00.00.png"
    );
    assert_eq!(
        screenshot_file_name(1_000_000_000, &|_| false),
        "2001-09-09_01.46.40.png"
    );
    assert_eq!(
        screenshot_file_name(951_782_400, &|_| false),
        "2000-02-29_00.00.00.png",
        "the leap day the civil calendar keeps"
    );
    assert_eq!(
        screenshot_file_name(1, &|n| n == "1970-01-01_00.00.01.png"),
        "1970-01-01_00.00.01_1.png",
        "the existing file the dedup suffix walks"
    );
    assert_eq!(
        screenshot_file_name(1, &|n| n == "1970-01-01_00.00.01.png"
            || n == "1970-01-01_00.00.01_1.png"),
        "1970-01-01_00.00.01_2.png",
        "the walked suffix the next free answers"
    );

    // THE STATE WALK (the keyScreenshot lane the flag raises, the
    // consumer tail the flag clears — the success the en_us
    // translation speaks, the failure the error carries; the
    // message the chat display the history answers)
    let mut app = App::new(3);
    app.enter_world();
    assert!(!app.screenshot_request);
    app.on_screenshot_key();
    assert!(app.screenshot_request);
    let frame_at = app.frame as u32;
    app.take_screenshot(Ok("2001-09-09_01.46.40.png".to_string()));
    assert!(!app.screenshot_request, "the grab the flag clears");
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("Saved screenshot as 2001-09-09_01.46.40.png"),
        "the success the screenshot.success law speaks"
    );
    assert_eq!(
        app.chat_messages.last().unwrap().1,
        frame_at,
        "the addedTime the push's own frame keeps"
    );
    app.take_screenshot(Err("disk full".to_string()));
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("Couldn't save screenshot: disk full"),
        "the failure the screenshot.failure law speaks"
    );
}

#[test]
fn the_f3_bottom_block_the_chart_keys_speak() {
    // THE KEYBIND WORD (formatKeybind's own pair — the bound
    // modifier the `[F3+1]` bracket word wears (the '\u0001+'
    // and '[\u0001\u0001]' recipes the concat bootstraps bank),
    // the unbound modifier the empty slot keeps (`[1]` the
    // ifeq-branch word wears))
    assert_eq!(debug_keybind_word(Some("F3"), "1"), "[F3+1]");
    assert_eq!(debug_keybind_word(Some("F3"), "F6"), "[F3+F6]");
    assert_eq!(debug_keybind_word(None, "1"), "[1]");

    // THE CHART ROW (formatChart's own fold — the keybind pair,
    // the chart name, the render flag's own visibility word the
    // '\u0001 \u0001 \u0001' recipe spaces)
    assert_eq!(
        debug_chart_row(Some("F3"), "1", "Profiler", false),
        "[F3+1] Profiler hidden"
    );
    assert_eq!(
        debug_chart_row(Some("F3"), "2", "FPS", true),
        "[F3+2] FPS visible"
    );

    // THE BOTTOM BLOCK (extractRenderState's isOverlayVisible
    // lane — the empty spacer the isNullOrEmpty law skips-but-
    // spends first, then the three rows the concat recipes bank;
    // the FPS row's word the REAL flag lane keeps — the all-false
    // boot the hidden word wears)
    let block = debug_bottom_lines(false, false, false, false);
    assert_eq!(block.len(), 4);
    assert_eq!(block[0], "", "the spacer row the cursor still walks");
    assert_eq!(
        block[1],
        "Debug charts: [F3+1] Profiler hidden; [F3+2] FPS hidden;"
    );
    assert_eq!(
        block[2],
        "[F3+3] Bandwidth + Ping hidden; [F3+4] Lightmap hidden"
    );
    assert_eq!(block[3], "To edit: press [F3+F6]");

    // THE INTEGRATED LANES (the FPS slot's name the integrated-
    // server probe keeps — `FPS + TPS` the some lane; the network
    // slot's name the isLocalServer probe keeps — `Ping` the
    // local lane, `Bandwidth + Ping` the remote the client's own
    // honest lane wears)
    let local = debug_bottom_lines(true, false, false, false);
    assert_eq!(
        local[1],
        "Debug charts: [F3+1] Profiler hidden; [F3+2] FPS + TPS hidden;"
    );
    assert_eq!(local[2], "[F3+3] Ping hidden; [F3+4] Lightmap hidden");

    // THE RAISED FLAG'S OWN WORD (the formatChart's render arg
    // the real renderFpsCharts state carries — the F3+2 toggle's
    // `visible` word the raised lane speaks, the row the jar's
    // own fold re-answers)
    let raised = debug_bottom_lines(false, true, false, false);
    assert_eq!(
        raised[1],
        "Debug charts: [F3+1] Profiler hidden; [F3+2] FPS visible;"
    );
}

#[test]
fn the_frame_ring_the_use_sample_walk_keeps() {
    // THE RING'S OWN LAWS (javap LocalSampleLogger — the append
    // at (start+size)%240 the useSample walk banks, the `size <
    // 240` grow else the start advance, the get(idx) the
    // wrapIndex(start+idx) row reads: idx 0 the oldest, size-1
    // the newest)
    let mut app = App::new(3);
    assert_eq!(app.frame_sample_size, 0);
    for i in 1..=240 {
        app.log_frame_duration(1_000_000 * i as i64);
    }
    assert_eq!(app.frame_sample_size, 240);
    assert_eq!(app.frame_sample(0), 1_000_000, "the oldest kept");
    assert_eq!(app.frame_sample(239), 240_000_000, "the newest kept");
    // THE 241ST SAMPLE (the full ring's own ride — the start
    // advances, the oldest column the write overwrites)
    app.log_frame_duration(999);
    assert_eq!(app.frame_sample_size, 240, "the cap holds");
    assert_eq!(app.frame_sample(0), 2_000_000, "the first dropped");
    assert_eq!(app.frame_sample(239), 999, "the newest lands");
}

#[test]
fn the_fps_chart_the_jar_math_keeps() {
    // THE BAR HEIGHTS (getSampleHeight's `Math.round(ms * 60.0 /
    // 33.333333333333336)` walk — 30fps the 60px full box, 60fps
    // the 30px half, 20fps the 90px poke-above the no-clamp lane
    // keeps)
    assert_eq!(fps_sample_height(33_333_333), 60);
    assert_eq!(fps_sample_height(16_666_667), 30);
    assert_eq!(fps_sample_height(50_000_000), 90);
    // THE COLORS (getSampleColor's clamp + two-arm lerp — the
    // exact f32 stops the half-deltas pin: 0ms the pure green,
    // 14ms the half green→yellow (red floor(127.5)=127), 28ms
    // the pure yellow (the arm boundary), 42ms the half
    // yellow→red (green floor(-127.5)=-128 → 127), 56ms+ the
    // pure red the clamp's own cap keeps)
    assert_eq!(fps_sample_color(0), -16711936, "0xFF00FF00 green");
    assert_eq!(fps_sample_color(14_000_000), -8388864, "0xFF7FFF00 half");
    assert_eq!(fps_sample_color(28_000_000), -256, "0xFFFFFF00 yellow");
    assert_eq!(fps_sample_color(42_000_000), -33024, "0xFFFF7F00 half");
    assert_eq!(fps_sample_color(56_000_000), -65536, "0xFFFF0000 red");
    assert_eq!(fps_sample_color(500_000_000), -65536, "the clamp caps");
    // THE WORDS (toDisplayString's `%d ms` — the Math.round the
    // floor(x+0.5) law keeps)
    assert_eq!(fps_display_word(16_666_667.0), "17 ms");
    assert_eq!(fps_display_word(0.0), "0 ms");
    // THE WIDTH + SKIP (getWidth's min(240, guiWidth/2) and the
    // bar loop's max(0, 240 - (width-2)) — the half-screen arg
    // the DSO call site keeps)
    assert_eq!(fps_chart_width(854), 240);
    assert_eq!(fps_chart_width(400), 200);
    assert_eq!(fps_chart_skip(240), 2);
    assert_eq!(fps_chart_skip(100), 142);
    assert_eq!(
        fps_chart_skip(2),
        240,
        "the two-column box the bars never fill"
    );
}

#[test]
fn the_fps_charts_key_the_toggle_and_release_laws_keep() {
    // THE TOGGLE'S OWN LAW (toggleFpsCharts's `!(visible &&
    // flag)` — the fresh app's closed panel the raise answers,
    // the raise the overlay's own forced-visible keeps)
    let mut app = App::new(3);
    assert!(!app.fps_charts && !app.debug_held);
    app.on_fps_charts_key();
    assert!(app.fps_charts, "the closed panel's chord raises");
    assert!(app.debug_held, "the raise forces the overlay visible");
    app.on_fps_charts_key();
    assert!(!app.fps_charts, "the visible raised chart lowers");
    assert!(app.debug_held, "the lower never closes the panel");
    // THE USED-AS-MODIFIER LANE (the chord's own ride — the F3
    // release the flag swallows, the next bare release the
    // toggleDebugOverlay flip answers)
    app.on_fps_charts_key();
    assert!(app.debug_modifier_used);
    app.on_debug_modifier_released();
    assert!(app.debug_held, "the consumed release keeps the panel");
    assert!(!app.debug_modifier_used, "the flag the release clears");
    app.on_debug_modifier_released();
    assert!(!app.debug_held, "the bare release toggles");
    assert!(
        app.fps_charts,
        "the chart flag the panel's toggle never touches"
    );
    // THE ROW'S OWN RE-SPEAK (the drawn panel's bottom block the
    // live flag carries — the visible word the raised lane wears)
    app.on_debug_modifier_released();
    assert!(app.debug_held);
    let block = debug_bottom_lines(false, app.fps_charts, app.network_charts, app.lightmap_view);
    assert_eq!(
        block[1],
        "Debug charts: [F3+1] Profiler hidden; [F3+2] FPS visible;"
    );
}

#[test]
fn the_network_charts_key_the_toggle_and_zero_laws_keep() {
    // THE TOGGLE'S OWN LAW (toggleNetworkCharts's `!(visible &&
    // flag)` — the bytecode the fps lane's walk mirrors verbatim:
    // the raise forces the overlay visible AND zeroes the fps
    // flag (the renderFpsCharts=false putfield the raise's tail
    // keeps), the lower keeps the panel)
    let mut app = App::new(3);
    assert!(!app.network_charts && !app.fps_charts && !app.debug_held);
    // THE FPS RAISE FIRST — the F3+3 chord's raise then ZEROES it
    // (the renderFpsCharts=false putfield the raise's tail keeps:
    // the two charts the toggle law never shows together)
    app.on_fps_charts_key();
    assert!(app.fps_charts && app.debug_held);
    app.on_network_charts_key();
    assert!(app.network_charts, "the closed flag's chord raises");
    assert!(app.debug_held, "the raise forces the overlay visible");
    assert!(
        !app.fps_charts,
        "the network raise zeroes the fps flag (the toggle law's own pair)"
    );
    // the raised flag + the visible panel the jar's own law
    // lowers (`!(visible && flag)` — the flip the second press
    // answers), the panel the lower never closes
    app.on_network_charts_key();
    assert!(!app.network_charts, "the visible raised chart lowers");
    assert!(app.debug_held, "the lower never closes the panel");
    // the down flag + the visible panel the law RAISES back (the
    // `!(true && false)` arm the closed flag's press answers)
    app.on_network_charts_key();
    assert!(app.network_charts, "the re-raise the law answers");
    app.on_network_charts_key();
    assert!(!app.network_charts, "the lower again");
    // THE WIRE HALF (the command the flag rides — the worker's
    // monitor the ping-request cadence arms, the lower disarms)
    let (tx, rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    app.on_network_charts_key();
    assert!(
        matches!(
            rx.try_recv().expect("the raise's command"),
            crate::world_view::ContainerCommand::NetworkCharts(true)
        ),
        "the raised flag rides the command channel"
    );
    // THE ROW'S OWN RE-SPEAK (the bottom block's network row the
    // live flag carries — the visible word the raised lane wears,
    // the fps row's hidden word the zeroed flag keeps)
    let block = debug_bottom_lines(false, app.fps_charts, app.network_charts, app.lightmap_view);
    assert_eq!(
        block[1],
        "Debug charts: [F3+1] Profiler hidden; [F3+2] FPS hidden;"
    );
    assert_eq!(
        block[2],
        "[F3+3] Bandwidth + Ping visible; [F3+4] Lightmap hidden"
    );
    app.on_network_charts_key();
    assert!(matches!(
        rx.try_recv().expect("the lower's command"),
        crate::world_view::ContainerCommand::NetworkCharts(false)
    ));
    assert_eq!(
        debug_bottom_lines(false, app.fps_charts, app.network_charts, app.lightmap_view)[2],
        "[F3+3] Bandwidth + Ping hidden; [F3+4] Lightmap hidden",
        "the lowered flag the hidden word wears"
    );
}

#[test]
fn the_ping_chart_the_jar_math_keeps() {
    // THE HEIGHT'S OWN LAW (PingDebugChart.getSampleHeight's
    // `round(ms * 60.0 / 500.0)` walk — 500ms the 60px full box,
    // 100ms the 12px one, the slow roundtrip the no-clamp top
    // pokes above, the sub-pixel the round answers)
    assert_eq!(ping_sample_height(500), 60);
    assert_eq!(ping_sample_height(250), 30);
    assert_eq!(ping_sample_height(100), 12);
    assert_eq!(ping_sample_height(0), 0);
    assert_eq!(ping_sample_height(600), 72, "the no-clamp poke-above");
    assert_eq!(ping_sample_height(5), 1, "0.6 the round lifts");
    // THE COLOR'S OWN STOPS (getSampleColor's (0.0, -16711936,
    // 250.0, -256, 500.0, -65536) walk — the clamp the 500 cap
    // keeps, the two arms the srgbLerp pair carries)
    assert_eq!(ping_sample_color(0), -16711936, "green the zero wears");
    assert_eq!(
        ping_sample_color(125),
        0xFF7FFF00u32 as i32,
        "the green→yellow half lerp"
    );
    assert_eq!(ping_sample_color(250), -256, "yellow the boundary wears");
    assert_eq!(
        ping_sample_color(375),
        0xFFFF7F00u32 as i32,
        "the yellow→red half lerp (the r channel the 255 pair keeps)"
    );
    assert_eq!(ping_sample_color(500), -65536, "red the cap wears");
    assert_eq!(
        ping_sample_color(900),
        -65536,
        "the clamp the over-cap keeps red"
    );
    // THE WORD'S OWN FOLD (toDisplayString's `'%d ms'` walk — the
    // sample IS the millis, no conversion the fps lane keeps)
    assert_eq!(ping_display_word(87.4), "87 ms");
    assert_eq!(ping_display_word(0.4), "0 ms");
    // THE RIGHT ANCHOR (the DSO's `guiWidth - width` walk — the
    // width the min(240, guiWidth/2) law keeps)
    assert_eq!(fps_chart_width(854), 240);
    assert_eq!(854 - fps_chart_width(854), 614, "the ping chart's own x");
    // THE SNAPSHOT LANDING (the shipped refresh's own ride — the
    // ring's chart order the App's belief keeps verbatim)
    let mut app = App::new(3);
    app.live_ping_updated(vec![10, 42, 250]);
    assert_eq!(app.ping_samples, vec![10, 42, 250]);
    app.live_ping_updated(Vec::new());
    assert!(
        app.ping_samples.is_empty(),
        "the empty ring the empty draw keeps"
    );
}

#[test]
fn the_bandwidth_chart_the_jar_math_keeps() {
    // THE LOG HEIGHT'S OWN LAW (getSampleHeightInternal's
    // `round(log(bps + 1.0) * 60.0 / log(1MiB))` walk — the
    // three grid lines the labeled pairs wear, the full box the
    // 1MiB word answers, the zero the zero bar)
    assert_eq!(bw_height_internal(64.0), 18);
    assert_eq!(bw_height_internal(1024.0), 30);
    assert_eq!(bw_height_internal(16384.0), 42);
    assert_eq!(bw_height_internal(1_048_576.0), 60);
    assert_eq!(bw_sample_height(0), 0, "ln(1) the zero bar keeps");
    // the sample's *20 conversion (toBytesPerSecond's own walk)
    // the log curve the full-box poke above answers
    assert_eq!(
        bw_sample_height(65536),
        61,
        "1.25 MiB/s the 61px poke wears"
    );
    // THE COLOR'S OWN STOPS (getSampleColor's (bps, 0.0,
    // -16711681, 8192.0, -6250241, 10485760.0, -65536) walk — the
    // clamp the 10MiB/s cap keeps, the cyan→lavender→red two-arm
    // lerp the srgbLerp pair carries)
    assert_eq!(bw_sample_color(0), -16711681, "cyan the zero wears");
    assert_eq!(
        bw_sample_color(1),
        0xFF00FEFFu32 as i32,
        "the tiny-t floor the g channel's -1 lifts"
    );
    assert_eq!(
        bw_sample_color(409),
        0xFF9FA0FFu32 as i32,
        "the near-boundary t the lavender approaches"
    );
    assert_eq!(
        bw_sample_color(410),
        0xFFA09FFEu32 as i32,
        "the second arm's tiny-t the red approaches"
    );
    assert_eq!(
        bw_sample_color(524_288),
        -65536,
        "the exact 10MiB/s cap the red wears"
    );
    assert_eq!(
        bw_sample_color(1_000_000),
        -65536,
        "the clamp the over-cap keeps red"
    );
    // THE WORD'S OWN THREE ARMS (toDisplayStringInternal's
    // dcmpl/iflt pairs — the '%.1f' HALF_UP rounding the java law
    // keeps: 1.25 the "1.3" word the Rust half-even wrap never
    // speaks)
    assert_eq!(bw_display_word(0.0), "0 B/s");
    assert_eq!(bw_display_word(63.9), "63 B/s");
    assert_eq!(bw_display_word(1024.0), "1.0 KiB/s");
    assert_eq!(bw_display_word(2047.0), "2.0 KiB/s");
    assert_eq!(bw_display_word(1_048_576.0), "1.0 MiB/s");
    assert_eq!(
        bw_display_word(1_500_000.0),
        "1.4 MiB/s",
        "the half-down fraction the floor law keeps"
    );
    assert_eq!(
        bw_display_word(1.25 * 1_048_576.0),
        "1.3 MiB/s",
        "the HALF_UP rounding the java format keeps"
    );
    // THE SAMPLE-WORD CONVERSION (toDisplayString's own walk —
    // the raw sample's *20 the bps word wears, the min/avg/max
    // labels the conversion call FIRST)
    assert_eq!(bw_display_word(65536.0 * 20.0), "1.3 MiB/s");
    // THE SNAPSHOT LANDING (the shipped refresh's own ride — the
    // ring's chart order the App's belief keeps verbatim)
    let mut app = App::new(3);
    app.live_bandwidth_updated(vec![0, 64, 524288]);
    assert_eq!(app.bw_samples, vec![0, 64, 524288]);
    app.live_bandwidth_updated(Vec::new());
    assert!(
        app.bw_samples.is_empty(),
        "the empty ring the empty draw keeps"
    );
}

#[test]
fn the_tps_chart_the_jar_math_keeps() {
    // THE HEIGHT'S OWN LAW (TpsDebugChart.getSampleHeight's
    // `round(ms * 60.0 / mspt)` walk — the mspt-RELATIVE law the
    // fps/ping lanes' fixed-ms walks never wear: 50ms the full
    // 60px box at the vanilla pace, the over-mspt tick the
    // no-clamp top pokes above)
    assert_eq!(tps_sample_height(50_000_000), 60);
    assert_eq!(tps_sample_height(25_000_000), 30);
    assert_eq!(tps_sample_height(0), 0);
    assert_eq!(
        tps_sample_height(62_500_000),
        75,
        "the no-clamp poke-above (the 62.5ms tick the 75px column wears)"
    );
    assert_eq!(tps_sample_height(500_000), 1, "0.6ms the round lifts");
    // THE COLOR'S OWN STOPS (getSampleColor's (ms, mspt, green,
    // mspt*1.125, yellow, mspt*1.25, red) walk — the clamp the
    // LOW end the mspt line itself bounds: a sub-mspt tick the
    // full green wears, the green→yellow lerp across
    // (mspt, mspt*1.125], the yellow→red across (mspt*1.125,
    // mspt*1.25], the over-cap red)
    assert_eq!(
        tps_sample_color(30_000_000),
        -16711936,
        "the below-mspt clamp the full green wears"
    );
    assert_eq!(
        tps_sample_color(50_000_000),
        -16711936,
        "the mspt line the green boundary wears"
    );
    assert_eq!(
        tps_sample_color(53_125_000),
        0xFF7FFF00u32 as i32,
        "the green→yellow half lerp (the 53.125ms midpoint)"
    );
    assert_eq!(
        tps_sample_color(56_250_000),
        -256,
        "yellow the ×1.125 boundary wears"
    );
    assert_eq!(
        tps_sample_color(59_375_000),
        0xFFFF7F00u32 as i32,
        "the yellow→red half lerp (the 59.375ms midpoint)"
    );
    assert_eq!(
        tps_sample_color(62_500_000),
        -65536,
        "red the ×1.25 cap wears"
    );
    assert_eq!(
        tps_sample_color(200_000_000),
        -65536,
        "the clamp the over-cap keeps red"
    );
    // THE AGGREGATION'S OWN VALUE (getValueForAggregation's
    // `get(i) - get(i, IDLE)` walk — the labels the
    // FULL-MINUS-IDLE value speak)
    assert_eq!(
        tps_aggregated_value(&[50_000_000, 41_000_000, 2_000_000, 1_000_000]),
        49_000_000
    );
    // THE LABEL'S OWN WORD (extractAdditionalLinesAndLabels's
    // `String.format(Locale.ROOT, '%.1f TPS', 1000.0f / mspt)`
    // walk — the f32 division the HALF_UP rounding speaks; 50ms
    // the vanilla pace the 20.0 TPS word wears)
    assert_eq!(tps_label_word(50.0), "20.0 TPS");
    assert_eq!(
        tps_label_word(25.0),
        "40.0 TPS",
        "the 25ms pace the 40.0 word wears"
    );
    // THE SNAPSHOT LANDING (the shipped refresh's own ride — the
    // ring's chart order the App's belief keeps verbatim)
    let mut app = App::new(3);
    app.live_tick_samples_updated(vec![[50_000_000, 41_000_000, 2_000_000, 1_000_000]]);
    assert_eq!(app.tick_samples.len(), 1);
    assert_eq!(app.tick_samples[0][0], 50_000_000);
    app.live_tick_samples_updated(Vec::new());
    assert!(
        app.tick_samples.is_empty(),
        "the empty ring the empty draw keeps"
    );
    // THE DRAW'S OWN GATE (the DSO extractRenderState's
    // showFpsCharts + tickTimeLogger.size() > 0 pair — the chart
    // the F3+2 chord's raised flag arms, the empty ring never
    // draws; the default App both lanes keep)
    assert!(!app.fps_charts && app.tick_samples.is_empty());
}

#[test]
fn the_fps_charts_key_the_subscription_word_rides() {
    // THE WIRE HALF (the command the F3+2 toggle's raise/lower
    // rides — the vanilla ClientDebugSubscriber's
    // requestedSubscriptions walk answers the showFpsCharts()
    // flag, the worker's own equality gate the request's send
    // keeps; the frame thread's half the command channel speaks)
    let mut app = App::new(3);
    let (tx, rx) = std::sync::mpsc::channel();
    app.live_clicks = Some(tx);
    app.on_fps_charts_key();
    assert!(
        matches!(
            rx.try_recv().expect("the raise's command"),
            crate::world_view::ContainerCommand::FpsCharts(true)
        ),
        "the raised flag rides the subscription's command"
    );
    app.on_fps_charts_key();
    assert!(
        matches!(
            rx.try_recv().expect("the lower's command"),
            crate::world_view::ContainerCommand::FpsCharts(false)
        ),
        "the lowered flag the empty-set word wears"
    );
}

#[test]
fn pause_buttons_route_back_to_game_options_and_title() {
    let mut app = App::new(3);
    app.enter_world();
    app.paused = true;
    app.render_title_buttons_for_test(); // builds pause buttons
    let back = app
        .buttons
        .iter()
        .find(|b| b.id == "backtogame")
        .expect("backtogame")
        .clone();
    app.on_click(back.x + back.w / 2, back.y + back.h / 2);
    assert!(!app.paused, "Back to Game resumes");
    assert_eq!(app.screen, ScreenKind::World);

    app.paused = true;
    app.render_title_buttons_for_test();
    let quit = app
        .buttons
        .iter()
        .find(|b| b.id == "saveandquit")
        .expect("saveandquit")
        .clone();
    app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::Title,
        "Save and Quit returns to title"
    );
    assert!(!app.paused);

    // options from pause returns to the paused world
    app.enter_world();
    app.paused = true;
    app.render_title_buttons_for_test();
    let opt = app
        .buttons
        .iter()
        .find(|b| b.id == "options")
        .expect("pauseoptions")
        .clone();
    app.on_click(opt.x + opt.w / 2, opt.y + opt.h / 2);
    assert_eq!(app.screen, ScreenKind::Options);
    app.render_title_buttons_for_test();
    let back = app
        .buttons
        .iter()
        .find(|b| b.id == "back")
        .expect("back")
        .clone();
    app.on_click(back.x + back.w / 2, back.y + back.h / 2);
    assert_eq!(app.screen, ScreenKind::World);
    assert!(app.paused, "returning from options stays paused");
}

/// THE WORLD OPTIONS' OWN WALK (javap the 26.2 jar the
/// OptionsScreen + WorldOptionsScreen + DifficultyButtons walks
/// speak): the in-world opener the `options.worldOptions.button`
/// word wears, the dial the `Difficulty: <name>` composition the
/// CycleButton's own displayString keeps, the click the NEXT
/// value the values() order answers (the OFFLINE world's own
/// field the integrated-server lane stores), the Done footer the
/// lastScreen chain wears back to the Options screen, the title
/// lane the opener the inWorld gate refuses.
#[test]
fn the_world_options_screen_wears_the_dial() {
    let mut app = App::new(3);
    app.enter_world();
    assert_eq!(
        app.offline_world.as_ref().unwrap().difficulty,
        crate::difficulty::Difficulty::Normal,
        "the new-world default the host's own dial speaks"
    );
    // THE TITLE LANE'S REFUSAL (the inWorld gate — the options
    // return the title lane keeps, the opener the same flag
    // never wears)
    app.options_return = ScreenKind::Title;
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    assert!(
        app.buttons.iter().all(|b| b.id != "worldoptions"),
        "the title lane the opener refuses"
    );
    // THE PAUSE LANE'S OPENER (the pause → Options walk — the
    // `options.worldOptions.button` the header row wears)
    app.enter_world();
    app.paused = true;
    app.render_title_buttons_for_test();
    let opt = app
        .buttons
        .iter()
        .find(|b| b.id == "options")
        .expect("pauseoptions")
        .clone();
    app.on_click(opt.x + opt.w / 2, opt.y + opt.h / 2);
    assert_eq!(app.screen, ScreenKind::Options);
    app.render_title_buttons_for_test();
    let opener = app
        .buttons
        .iter()
        .find(|b| b.id == "worldoptions")
        .expect("worldoptions opener")
        .clone();
    assert_eq!(
        opener.label, "World Options...",
        "the `options.worldOptions.button` word the lang table speaks"
    );
    app.on_click(opener.x + opener.w / 2, opener.y + opener.h / 2);
    assert_eq!(app.screen, ScreenKind::WorldOptions);
    // THE DIAL'S OWN WORD (the label + value the CycleButton's
    // displayString composition keeps — the 150-wide button the
    // create walk banks)
    app.render_title_buttons_for_test();
    let dial = app
        .buttons
        .iter()
        .find(|b| b.id == "difficulty")
        .expect("the dial")
        .clone();
    assert_eq!(dial.label, "Difficulty: Normal", "the new-world word");
    assert!(dial.active, "the OFFLINE world the permission arm wears");
    assert_eq!(
            dial.w,
            130,
            "the create walk's own setWidth — the dial the lock's 20 rides (150 - 20 = 130, the row's left child)"
        );
    // THE LOCK ROW'S OWN PAIR (the EqualSpacingLayout(150, 0,
    // HORIZONTAL) walk — the 20x20 lock the dial's right ride
    // wears, the flush row the zero remaining space keeps: the
    // dial's left edge + 130 the lock's own left edge answers)
    let lock = app
        .buttons
        .iter()
        .find(|b| b.id == "wodlock")
        .expect("the lock button the create walk banks")
        .clone();
    assert_eq!(lock.w, 20, "the LockIconButton's own 20x20 shape");
    assert_eq!(lock.h, 20, "the LockIconButton's own 20x20 shape");
    assert!(lock.active, "the unlocked lock the shared active arm wears");
    assert_eq!(
        lock.x,
        dial.x + 130,
        "the flush row — the lock the dial's right ride keeps"
    );
    assert_eq!(lock.y, dial.y, "the row the same band keeps");
    // THE CYCLE (the click the next value the values() order
    // answers — Normal → Hard, the wrap the four-turn walk rings
    // home: Hard → Peaceful → Easy → Normal)
    app.on_click(dial.x + dial.w / 2, dial.y + dial.h / 2);
    assert_eq!(
        app.offline_world.as_ref().unwrap().difficulty,
        crate::difficulty::Difficulty::Hard,
        "the first turn the values() order answers"
    );
    app.render_title_buttons_for_test();
    let dial = app
        .buttons
        .iter()
        .find(|b| b.id == "difficulty")
        .expect("the dial")
        .clone();
    assert_eq!(dial.label, "Difficulty: Hard", "the word the dial speaks");
    for expected in [
        crate::difficulty::Difficulty::Peaceful,
        crate::difficulty::Difficulty::Easy,
        crate::difficulty::Difficulty::Normal,
    ] {
        // the re-render the click walk's own clear answers — the
        // buttons the render pushes, the click the fresh vec hits
        app.render_title_buttons_for_test();
        let dial = app
            .buttons
            .iter()
            .find(|b| b.id == "difficulty")
            .expect("the dial")
            .clone();
        app.on_click(dial.x + dial.w / 2, dial.y + dial.h / 2);
        assert_eq!(
            app.offline_world.as_ref().unwrap().difficulty,
            expected,
            "the wrap walk rides the values() order"
        );
    }
    app.render_title_buttons_for_test();
    let dial = app
        .buttons
        .iter()
        .find(|b| b.id == "difficulty")
        .expect("the dial")
        .clone();
    assert_eq!(dial.label, "Difficulty: Normal", "the ring home's word");
    // THE DONE CHAIN (the lastScreen walk — the WorldOptions
    // screen the Options screen returns to, the Options screen
    // the paused world keeps)
    let done = app
        .buttons
        .iter()
        .find(|b| b.id == "wodone")
        .expect("wodone")
        .clone();
    app.on_click(done.x + done.w / 2, done.y + done.h / 2);
    assert_eq!(app.screen, ScreenKind::Options, "the chain's own hop");
    app.render_title_buttons_for_test();
    let back = app
        .buttons
        .iter()
        .find(|b| b.id == "back")
        .expect("back")
        .clone();
    app.on_click(back.x + back.w / 2, back.y + back.h / 2);
    assert_eq!(app.screen, ScreenKind::World);
    assert!(app.paused, "the chain's return the pause keeps");
}

/// THE GAMERULE'S OWN ROW THE WORLD OPTIONS WEARS (javap 26.2
/// GameRules.RULE_KEEPIVENTORY the row the death lanes' own gates
/// read): the label the `gamerule.keepInventory` lang row speaks,
/// the value the `options.on`/`options.off` pair the boolean
/// display keeps, the click the toggle the offline lane's own
/// gamerule store answers, the NETWORKED world the inactive button
/// the hovered() gate refuses.
#[test]
fn the_world_options_screen_wears_the_keep_inventory_row() {
    let mut app = App::new(3);
    app.enter_world();
    assert!(
        !app.keep_inventory,
        "the vanilla default the false boot wears"
    );
    app.open(ScreenKind::WorldOptions);
    app.render_title_buttons_for_test();
    let keep = app
        .buttons
        .iter()
        .find(|b| b.id == "wokeepinv")
        .expect("the keepInventory row the offline screen wears")
        .clone();
    assert_eq!(
        keep.label, "Keep inventory after death: OFF",
        "the lang row + the options.off word the fresh boot wears"
    );
    assert!(keep.active, "the OFFLINE world the gamerule arm wears");
    // THE TOGGLE (the click the BooleanValue's own flip answers —
    // the row's word the re-render speaks)
    app.on_click(keep.x + keep.w / 2, keep.y + keep.h / 2);
    assert!(app.keep_inventory, "the click the rule arms");
    app.render_title_buttons_for_test();
    let keep = app
        .buttons
        .iter()
        .find(|b| b.id == "wokeepinv")
        .expect("the row")
        .clone();
    assert_eq!(
        keep.label, "Keep inventory after death: ON",
        "the options.on word the armed row speaks"
    );
    // THE SECOND CLICK (the flip the boolean's own walk rings home)
    app.on_click(keep.x + keep.w / 2, keep.y + keep.h / 2);
    assert!(!app.keep_inventory, "the second click the row disarms");
    // THE NETWORKED LANE'S REFUSAL (the inactive button the
    // hovered() gate never clicks — the server's own /gamerule the
    // wire keeps)
    app.world_is_network = true;
    app.render_title_buttons_for_test();
    let keep = app
        .buttons
        .iter()
        .find(|b| b.id == "wokeepinv")
        .expect("the row")
        .clone();
    assert!(!keep.active, "the networked world the gamerule arm refuses");
    app.on_click(keep.x + keep.w / 2, keep.y + keep.h / 2);
    assert!(
        !app.keep_inventory,
        "the inactive button the click never arms"
    );
}

/// THE DIAL'S OWN GATES THE FOOD TICK WEARS (the FoodData tick's
/// ctx the level's own difficulty reads — the exhaustion fold the
/// PEACEFUL bar's drop refuses, the NORMAL arm the same fold the
/// bar drop answers): the OFFLINE world's own dial the stand-in
/// server's tick reads — the old hardcode the Normal word kept,
/// the WorldOptions dial's turn the same gates turn.
#[test]
fn the_food_tick_wears_the_worlds_own_dial() {
    let mut app = App::new(3);
    app.enter_world();
    // THE NORMAL ARM (the default dial — the fold the bar's drop
    // answers: the saturation spent, the food walks down)
    app.food_data.set_food_level(20);
    app.food_data.set_saturation(0.0);
    app.food_data.add_exhaustion(5.0);
    app.update();
    assert_eq!(
        app.food_data.food_level(),
        19,
        "the NORMAL fold the bar's own drop keeps"
    );
    // THE PEACEFUL ARM (the dial's turn — the same fold the bar
    // the peaceful refusal keeps)
    app.offline_world.as_mut().unwrap().difficulty = crate::difficulty::Difficulty::Peaceful;
    app.food_data.set_food_level(20);
    app.food_data.set_saturation(0.0);
    app.food_data.add_exhaustion(5.0);
    app.update();
    assert_eq!(
        app.food_data.food_level(),
        20,
        "the PEACEFUL arm the bar's own drop refuses"
    );
}

/// THE LOCK ICON'S OWN STATE ARMS (javap 26.2 LockIconButton$Icon
/// + `extractContents`' raw bytecode — the six states the clinit
/// banks, the DISABLED arm the `!active` walk wears FIRST, the
/// HIGHLIGHTED arm the `isHoveredOrFocused()` walk answers, the
/// plain pair the resting walk keeps): the full 2x2x2 arm table
/// the selection fold speaks.
#[test]
fn the_lock_icon_wears_the_extract_contents_arms() {
    // THE DISABLED PAIR (the `!active` arm — the ifeq -> 20 walk
    // the state speak answers before the hover the read rides)
    assert_eq!(
        lock_icon_sprite(true, false, false),
        "widget/locked_button_disabled",
        "the locked-disabled pair the Icon clinit banks"
    );
    assert_eq!(
        lock_icon_sprite(false, false, false),
        "widget/unlocked_button_disabled",
        "the unlocked-disabled pair the same walk keeps"
    );
    // the DISABLED arm the hover NEVER overrides (the arm order —
    // the disabled read the hover's ifeq precedes)
    assert_eq!(
        lock_icon_sprite(true, false, true),
        "widget/locked_button_disabled",
        "the disabled arm the hover never overrides"
    );
    assert_eq!(
        lock_icon_sprite(false, false, true),
        "widget/unlocked_button_disabled",
        "the same precedence the unlocked side keeps"
    );
    // THE HIGHLIGHTED PAIR (the active + hovered arm — the
    // isHoveredOrFocused() walk)
    assert_eq!(
        lock_icon_sprite(true, true, true),
        "widget/locked_button_highlighted",
        "the locked-hover pair the active lock wears"
    );
    assert_eq!(
        lock_icon_sprite(false, true, true),
        "widget/unlocked_button_highlighted",
        "the unlocked-hover pair the same walk answers"
    );
    // THE PLAIN PAIR (the active + resting walk)
    assert_eq!(
        lock_icon_sprite(true, true, false),
        "widget/locked_button",
        "the locked pair the resting active lock wears"
    );
    assert_eq!(
        lock_icon_sprite(false, true, false),
        "widget/unlocked_button",
        "the unlocked pair the fresh WorldOptions screen wears"
    );
}

/// THE LOCK QUESTION'S OWN SCREEN (javap 26.2 ConfirmScreen — the
/// 3-arg ctor the `gui.yes` / `gui.no` defaults keeps, the words
/// the en_us table banks, the layout the vertical LinearLayout's
/// centered cells speak, the ESCAPE the `shouldCloseOnEsc() =
/// false` walk refuses): the title + question the lock's onPress
/// opens, the 150-wide Yes/No pair the spacing(4) row banks, the
/// Escape the silence keeps.
#[test]
fn the_lock_confirm_screen_wears_the_vanilla_words() {
    let mut app = App::new(3);
    app.enter_world();
    // THE OPEN (the lock's onPress — the wodlock click the confirm
    // screen opens)
    app.open(ScreenKind::WorldOptions);
    app.render_title_buttons_for_test();
    let lock = app
        .buttons
        .iter()
        .find(|b| b.id == "wodlock")
        .expect("the lock")
        .clone();
    app.on_click(lock.x + lock.w / 2, lock.y + lock.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::DifficultyLockConfirm,
        "the confirm the lock's hand-off opens"
    );
    // THE TITLE + QUESTION (the ctor's translatable pair — the
    // `difficulty.lock.title` word the title lane keeps, the
    // `difficulty.lock.question` the display name fills)
    app.render_title_buttons_for_test();
    let yes = app
        .buttons
        .iter()
        .find(|b| b.id == "wodlockyes")
        .expect("the Yes button the addButtons walk banks")
        .clone();
    let no = app
        .buttons
        .iter()
        .find(|b| b.id == "wodlockno")
        .expect("the No button the same walk banks")
        .clone();
    assert_eq!(yes.label, "Yes", "the gui.yes word the 3-arg ctor banks");
    assert_eq!(no.label, "No", "the gui.no word the same ctor keeps");
    assert_eq!(yes.w, 150, "the default Button width the builder keeps");
    assert_eq!(no.w, 150, "the same width the No side wears");
    // THE SPACING(4) ROW (the horizontal LinearLayout walk — the
    // 304-wide pair the 4px gap keeps: yes.x + 150 + 4 the no's
    // own left edge answers)
    assert_eq!(
        no.x,
        yes.x + 150 + 4,
        "the spacing(4) row the addButtons walk banks"
    );
    // THE COLUMN'S OWN CENTER (the centerInRectangle walk — the
    // 304-wide row the screen's own center keeps: the 320-wide
    // test screen the yes left edge at 160 - 152 answers, the no
    // right edge at 160 + 152 the same center brackets)
    assert_eq!(yes.x, 320 / 2 - 152, "the row the screen's center keeps");
    assert_eq!(
        no.x + no.w,
        320 / 2 + 152,
        "the same center the row's right edge brackets"
    );
    // THE QUESTION'S OWN WORD (the `%1$s` the display name fills —
    // the render's own walk the wrapped rows answer: the full
    // word the lang table banks the screen's rows speak)
    assert!(
        tr("difficulty.lock.question")
            .expect("the question the en_us table banks")
            .contains("%1$s"),
        "the positional slot the display name fills"
    );
    assert_eq!(
        tr("difficulty.lock.title"),
        Some("Lock World Difficulty"),
        "the title word the lang table speaks"
    );
    // THE ESCAPE'S OWN REFUSAL (the shouldCloseOnEsc() = false walk
    // — the confirm screen the on_escape arms never wear: the
    // Escape the screen unchanged keeps)
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::DifficultyLockConfirm,
        "the Escape the modal discipline keeps — the buttons alone answer"
    );
}

/// THE ONLOCKCALLBACK'S OWN WALK (javap 26.2 DifficultyButtons —
/// the `setScreen(screen)` BOTH answers wear FIRST, the Yes the
/// `ServerboundLockDifficultyPacket(true)` send speaks, the lock +
/// dial pair the INACTIVE arm answers after): the OFFLINE world
/// the integrated-server lane IS — the sim's own flag the confirm's
/// Yes arms, the locked dial the next click refuses, the No the
/// flag never touched keeps.
#[test]
fn the_on_lock_callback_walks_both_answers() {
    let mut app = App::new(3);
    app.enter_world();
    app.open(ScreenKind::WorldOptions);
    app.render_title_buttons_for_test();
    let lock = app
        .buttons
        .iter()
        .find(|b| b.id == "wodlock")
        .expect("the lock")
        .clone();
    assert!(
        !app.offline_world.as_ref().unwrap().difficulty_locked,
        "the fresh world the flag never wears"
    );
    // THE NO ANSWER (the setScreen(screen) FIRST — the WorldOptions
    // screen the walk returns to, the flag the walk never touches)
    app.on_click(lock.x + lock.w / 2, lock.y + lock.h / 2);
    assert_eq!(app.screen, ScreenKind::DifficultyLockConfirm);
    app.render_title_buttons_for_test();
    let no = app
        .buttons
        .iter()
        .find(|b| b.id == "wodlockno")
        .expect("the No")
        .clone();
    app.on_click(no.x + no.w / 2, no.y + no.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::WorldOptions,
        "the NO the same return the vanilla's setScreen(screen) walk keeps"
    );
    assert!(
        !app.offline_world.as_ref().unwrap().difficulty_locked,
        "the NO the flag never touched keeps"
    );
    // THE YES ANSWER (the ServerboundLockDifficultyPacket(true) the
    // OFFLINE sim stores — the flag arms, the dial + lock pair the
    // INACTIVE arm answers on the return's own re-render)
    app.render_title_buttons_for_test();
    let lock = app
        .buttons
        .iter()
        .find(|b| b.id == "wodlock")
        .expect("the lock")
        .clone();
    app.on_click(lock.x + lock.w / 2, lock.y + lock.h / 2);
    app.render_title_buttons_for_test();
    let yes = app
        .buttons
        .iter()
        .find(|b| b.id == "wodlockyes")
        .expect("the Yes")
        .clone();
    app.on_click(yes.x + yes.w / 2, yes.y + yes.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::WorldOptions,
        "the YES the same return wears"
    );
    assert!(
        app.offline_world.as_ref().unwrap().difficulty_locked,
        "the YES the lock flag the sim's own lane arms"
    );
    // THE INACTIVE ARM (the refresh walk's writes — BOTH buttons
    // the locked world INACTIVE keeps: the lock's own isLocked()
    // read, the dial the same `!locked && permission` arm wears)
    app.render_title_buttons_for_test();
    let lock = app
        .buttons
        .iter()
        .find(|b| b.id == "wodlock")
        .expect("the lock")
        .clone();
    let dial = app
        .buttons
        .iter()
        .find(|b| b.id == "difficulty")
        .expect("the dial")
        .clone();
    assert!(
        !lock.active,
        "the locked lock the INACTIVE arm the refresh walk keeps"
    );
    assert!(
        !dial.active,
        "the locked dial the same arm answers (the create walk's own write)"
    );
    // THE LOCKED DIAL'S OWN REFUSAL (the inactive button the
    // hovered() gate refuses — the click the dial's center the
    // values() order NEVER rides)
    let before = app.offline_world.as_ref().unwrap().difficulty;
    app.on_click(dial.x + dial.w / 2, dial.y + dial.h / 2);
    assert_eq!(
        app.offline_world.as_ref().unwrap().difficulty,
        before,
        "the locked dial the click refuses — the lock the vanilla's own promise keeps"
    );
    // THE LOCK'S OWN REFUSAL (the inactive lock the hand-off never
    // wears — the confirm screen the second lock never opens)
    app.on_click(lock.x + lock.w / 2, lock.y + lock.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::WorldOptions,
        "the locked lock the hand-off refuses"
    );
}

/// THE NETWORKED DIAL'S OWN WIRE WORD (javap 26.2
/// `handleChangeDifficulty` — the ClientboundChangeDifficultyPacket
/// the levelData pair stores, the materialize walk the networked
/// World wears, the WorldOptions dial the CycleButton's
/// displayString the same field speaks): the wire's Peaceful +
/// locked word the dial's label answers, the PERMISSION arm both
/// buttons INACTIVE keeps (the `hasSingleplayerServer()` read the
/// wire world never answers — the locked bit the lock's ICON
/// still speaks), the clicks the hovered() gate refuses.
#[test]
fn the_networked_dial_wears_the_wires_own_word() {
    let mut app = App::new(3);
    app.world_is_network = true;
    // the wire's own pair the materialized World wears (the model's
    // bank the refresh ships — the Peaceful + locked word the
    // server's own dial keeps)
    app.net_world = Some(crate::world::World {
        darkness_scale: 0.0,
        chunks_per_side: 2,
        seed: 0,
        difficulty: crate::difficulty::Difficulty::Peaceful,
        difficulty_locked: true,
        inhabited: None,
        // THE FIRE'S OWN QUEUE + LADDER the test stage never arms —
        // the empty store the offline walk alone fills
        fire_ticks: Vec::new(),
        fire_age: std::collections::HashMap::new(),
        portal_axis: std::collections::HashMap::new(), // the portal's own axis store: empty
        container_fill: std::collections::HashMap::new(), // the comparator lane: no fill
        frame_analog: std::collections::HashMap::new(), // the frame lane: no analog
        has_skylight: true,                            // the portal's own axis store: empty
        campfire_ticks: Vec::new(),                    // the campfire's own queue: empty
        torch_ticks: Vec::new(),                       // the torch's own queue: empty
        detector_ticks: Vec::new(),                    // the detector's own queue: empty
        comparator_ticks: Vec::new(),
        dispenser_ticks: Vec::new(),
        repeater_ticks: Vec::new(),
        observer_ticks: Vec::new(),
        piston_ticks: Vec::new(),
        moving_states: Vec::new(),
        moving_ticks: Vec::new(),
        torch_toggles: Vec::new(), // the RECENT_TOGGLES window: empty
        bamboo_ticks: Vec::new(),  // the bamboo's own queue: empty
        chunks: vec![
            vec![
                crate::world::AIR;
                crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z
            ];
            4
        ],
        light: None,
        day_tick: None,           // test scenes stay day
        biomes: None,             // test scenes carry no biome layer
        variants: None,           // test scenes carry no state variants
        ambient_light: None,      // no dimension opinion — the black bytecode default
        block_light_tint: None,   // no dimension opinion — the bytecode default tint
        sky_light_factor: None,   // no dimension opinion — the bytecode default 1.0
        sky_light_color: None,    // no dimension opinion — the -1 default's white identity
        night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
        night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
        rain_level: 0.0,          // the dry test world — no weather word rides
        thunder_level: 0.0,       // the dry test world — no storm speaks
        // THE OFFLINE SAVE'S OWN LANES the staged world never wears —
        // the gate the gen-time default keeps off, the journal the
        // session's rows never names
        journaling: false,
        edit_journal: Vec::new(),
        variant_journal: Vec::new(),
        saved_edits_mark: 0,
        saved_variants_mark: 0,
    });
    app.open(ScreenKind::WorldOptions);
    app.render_title_buttons_for_test();
    // THE DIAL'S OWN WORD (the wire's Peaceful the displayString
    // composition speaks — the absent Normal default retired)
    let dial = app
        .buttons
        .iter()
        .find(|b| b.id == "difficulty")
        .expect("the dial")
        .clone();
    assert_eq!(
        dial.label, "Difficulty: Peaceful",
        "the wire's own word the materialized World wears"
    );
    // THE PERMISSION ARM (the create walk's `!locked && permission`
    // — the wire world the hasSingleplayerServer() read never
    // answers: BOTH buttons INACTIVE regardless of the lock)
    assert!(!dial.active, "the networked dial the permission arm keeps");
    let lock = app
        .buttons
        .iter()
        .find(|b| b.id == "wodlock")
        .expect("the lock")
        .clone();
    assert!(!lock.active, "the networked lock the same arm keeps");
    // THE LOCKED ICON THE DISABLED STATE WEARS (the extractContents
    // walk — the !active arm FIRST, the locked bit the pair speaks)
    assert_eq!(
        lock_icon_sprite(true, lock.active, false),
        "widget/locked_button_disabled",
        "the locked-disabled icon the networked world's bit wears"
    );
    // THE CLICKS THE HOVERED() GATE REFUSES (the inactive buttons
    // the on_click walk never passes — the dial's turn and the
    // confirm's hand-off the wire world both refuse)
    app.on_click(dial.x + dial.w / 2, dial.y + dial.h / 2);
    assert_eq!(
        app.net_world.as_ref().unwrap().difficulty,
        crate::difficulty::Difficulty::Peaceful,
        "the inactive dial the click refuses — the wire the truth owns"
    );
    app.on_click(lock.x + lock.w / 2, lock.y + lock.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::WorldOptions,
        "the networked lock the hand-off refuses — the confirm the offline lane alone opens"
    );
}

#[test]
fn hud_layout_matches_vanilla_geometry() {
    // vanilla: hotbar 182x22 centered, bottom edge 1px above screen;
    // hearts above left half at height - 39 (the Hud's own law, the
    // XP bar's slot between the hearts and the hotbar), hunger on
    // the same line, right edge flush with the hotbar's
    let (w, h) = (320, 180);
    assert_eq!(hotbar_rect(w, h), (320 / 2 - 91, 180 - 23));
    assert_eq!(hearts_origin(w, h), (320 / 2 - 91, 180 - 39));
    assert_eq!(food_origin(w, h), (320 / 2 + 91 - HEART, 180 - 39));
    // the bubbles' own line: the armor row the vehicle-less lift
    // speaks (the getAirBubbleYLine's -1 rowOffset folds +10 back
    // onto the height - 59 the caller arms)
    assert_eq!(bubbles_origin(w, h), 180 - 49);
    // THE EXPERIENCE BAR'S OWN SLOT (the ContextualBar defaults the
    // bytecode speaks): `left = (w-182)/2`, `top = h-24-5` — the
    // row exactly fills the gap the hearts' law leaves; the level
    // digits ride `h-24-9-2`, the centered walk
    assert_eq!((w / 2 - 91, h - 29), (69, 151));
    assert_eq!((w - w) / 2, 0, "the centered level digits' x walk");
    assert_eq!(h - 24 - 9 - 2, 145, "the level digits' own y");
}

#[test]
fn the_xp_fill_walks_the_region_law() {
    // javap ExperienceBar.extractBackground: the fill is the
    // TEN-INT region blit — `(int)(progress * 183.0f)` the f2i
    // TRUNCATION speaks, the region [0,fill) x [0,5) the crop
    // carries, the zero fill never draws (the `ifle 101` gate)
    let f = |progress: f32| (progress * 183.0) as i32;
    assert_eq!(f(0.0), 0, "the empty bar's fill never draws");
    assert_eq!(f(0.5), 91, "the half bar clips at 91");
    assert_eq!(f(0.9999999), 182, "the f2i truncation the bytecode speaks");
    assert_eq!(f(1.0), 183, "the full walk rides one past the sprite's 182 — the vanilla's own overflow the region read clamps");
    // the stat's own grant feeds the fraction the fill reads
    let mut xp = crate::xp::ExperienceStat::new();
    xp.give_points(5); // 5/7 at level 0
    let fill = (xp.progress * 183.0) as i32;
    assert!(
        fill > 0 && fill < 183,
        "the partial grant's fill walks between the gates: {fill}"
    );
}

#[test]
fn the_air_bubble_fold_speaks_the_extract_law() {
    use crate::body::TOTAL_AIR_SUPPLY as MAX;
    const AIR: &str = "hud/air";
    const BURST: &str = "hud/air_bursting";
    const EMPTY: &str = "hud/air_empty";
    let n = |r: &[Option<&'static str>; 10], s: &str| r.iter().filter(|x| **x == Some(s)).count();
    // the FULL store underwater: all ten speak the full bubble (the
    // gate alone would stay silent dry — the row's own law)
    let (row, popping) = air_bubble_row(MAX, MAX, true);
    assert_eq!(n(&row, AIR), 10);
    assert_eq!(popping, -1, "the delay's own +1 pushes past the row");
    // the HALF store underwater (150): five full, the one-tick
    // boundary blink (slot 5 = i=6 draws nothing), four empty
    let (row, popping) = air_bubble_row(150, MAX, true);
    assert_eq!(n(&row, AIR), 5);
    assert_eq!(n(&row, EMPTY), 4);
    assert_eq!(row[5], None, "the boundary blink the delay speaks");
    assert_eq!(popping, 4);
    // the BOUNDARY CROSSING underwater (121 — the changed window):
    // four full, the bursting at the boundary (i == full == 5),
    // five empty
    let (row, popping) = air_bubble_row(121, MAX, true);
    assert_eq!(n(&row, AIR), 4);
    assert_eq!(n(&row, BURST), 1, "the changed window's own sprite");
    assert_eq!(row[4], Some(BURST));
    assert_eq!(n(&row, EMPTY), 5);
    assert_eq!(popping, 5);
    // the DRAINED-DRY diver (0 underwater): the delay's own gate
    // needs air != 0 — the store's zero rides delay 0, the whole
    // row speaks empty (popping 10, the jitter arm's own frame)
    let (row, popping) = air_bubble_row(0, MAX, true);
    assert_eq!(n(&row, EMPTY), 10);
    assert_eq!(popping, 10, "the zero store's own delay-0 fold");
    // the LAST BUBBLE'S CROSSING (1 underwater): the zero boundary's
    // changed window speaks the bursting at slot 0, nine empties
    let (row, popping) = air_bubble_row(1, MAX, true);
    assert_eq!(n(&row, BURST), 1);
    assert_eq!(row[0], Some(BURST));
    assert_eq!(n(&row, EMPTY), 9);
    assert_eq!(popping, 9);
    // the DRAINED-DRY SURFACED body (0 dry): the same whole-empty
    // row (popping == 10, the jitter's own gate)
    let (row, popping) = air_bubble_row(0, MAX, false);
    assert_eq!(n(&row, EMPTY), 10);
    assert_eq!(popping, 10, "the jitter's own gate");
}

/// THE DROWN LANE'S OWN ROUND (the App update's own chain): the
/// submerged body's clock drains, the -20 floor folds the DROWNING
/// voice through the hurt pipeline (the cooldown's own window), the
/// 2.0 dose rides the health, and the store carries the reset the
/// fold speaks.
#[test]
fn the_drown_lane_hurts_the_submerged_body() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.world_size;
    let c = size / 2;
    let surface = app.spawn_y;
    // the 3-deep water column AT the body's own cells (the feet sit
    // on the solid below, the eye one cell under the surface)
    {
        let w = app.offline_world.as_mut().unwrap();
        for y in surface..surface + 3 {
            w.set(c, y as i32, c, crate::world::WATER);
        }
    }
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        surface as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    {
        let b = app.body.as_mut().unwrap();
        b.vy = 0.0;
        b.on_ground = true;
        b.air_supply = 1; // the seed the -20 fold's rhythm needs
    }
    let before = app.health;
    let mut voice = None;
    for _ in 0..25 {
        app.update();
        if voice.is_none() {
            voice = app
                .sounds
                .events
                .iter()
                .find(|e| e.starts_with("entity.player.hurt_drown @"))
                .cloned();
        }
    }
    assert!(
        !sounds_loaded() || voice.is_some(),
        "the -20 fold's dose spoke its DROWNING voice: {:?}",
        voice
    );
    if let Some(entry) = voice {
        assert!(entry.contains(" v1.000000 p"), "the voice fold: {entry}");
    }
    assert!(
        (app.health - (before - crate::body::DROWN_DAMAGE)).abs() < 1e-4,
        "the 2.0 dose rode the health ({} -> {})",
        before,
        app.health
    );
    assert!(
        app.body.as_ref().unwrap().air_supply < 0,
        "the clock walks under zero again (the reset the fold spoke, then the drain)"
    );
}

#[test]
fn spawn_scan_finds_top_solid_block() {
    let world = crate::world::World::new(1, 42);
    let y = top_solid_y(&world, 0, 0);
    assert!(y >= 1 && y <= crate::world::CHUNK_Y as i32);
    assert_ne!(
        world.get(0, y - 1, 0),
        crate::world::AIR,
        "block below eye is solid"
    );
}

#[test]
fn ping_icons_follow_vanilla_thresholds() {
    let mut e = ServerEntry {
        name: "x".into(),
        motd: "y".into(),
        ping_ms: 10,
        online: true,
        version: String::new(),
        host: None,
    };
    assert_eq!(e.ping_icon(), "server_list/ping_5");
    e.ping_ms = 90;
    assert_eq!(e.ping_icon(), "server_list/ping_3");
    e.ping_ms = 999;
    assert_eq!(e.ping_icon(), "server_list/ping_1");
    e.online = false;
    assert_eq!(e.ping_icon(), "server_list/unreachable");
}

#[test]
fn button_hover_and_hitbox() {
    let b = Button::new("test", "Hi".into(), 50, 40, 100);
    assert_eq!((b.x, b.y, b.w, b.h), (0, 40, 100, 20));
    assert!(b.hovered(50, 50));
    assert!(!b.hovered(150, 50));
    assert!(!b.hovered(50, 61));
}

#[test]
fn gui_cancel_and_title_keys_exist_in_official_lang() {
    // THE MIGRATION'S OWN LAW (the table the runtime cache keeps, the
    // loader the only parser): the assert computes the expected from
    // the ACTUAL loaded state — the cache-present boot the vanilla
    // values answer, the cache-absent boot the None the inline
    // fallbacks keep (the CI's pre-generate test job the same arm
    // walks). The keys the screens render, the loaded table must hold.
    let loaded = !load_lang().is_empty();
    let expect = |v: Option<&'static str>| if loaded { v } else { None };
    assert_eq!(tr("gui.cancel"), expect(Some("Cancel")));
    assert_eq!(tr("menu.singleplayer"), expect(Some("Singleplayer")));
    assert_eq!(tr("menu.multiplayer"), expect(Some("Multiplayer")));
    assert_eq!(tr("menu.options"), expect(Some("Options...")));
    assert_eq!(tr("menu.quit"), expect(Some("Quit Game")));
    assert_eq!(tr("multiplayer.title"), expect(Some("Play Multiplayer")));
    assert_eq!(
        tr("connect.failed"),
        expect(Some("Failed to connect to the server"))
    );
    assert_eq!(tr("selectServer.select"), expect(Some("Join Server")));
    assert_eq!(tr("options.title"), expect(Some("Options")));
    assert_eq!(tr("menu.online"), expect(Some("Minecraft Realms")));
}

#[test]
fn click_navigates_screens() {
    let mut app = App::new(3);
    for _ in 0..30 {
        app.update();
    }
    assert_eq!(app.screen, ScreenKind::Title);
    // render once to build the title buttons, then click Multiplayer
    app.render_title_buttons_for_test();
    let mp = app
        .buttons
        .iter()
        .find(|b| b.id == "multiplayer")
        .expect("multiplayer button")
        .clone();
    app.on_click(mp.x + mp.w / 2, mp.y + mp.h / 2);
    assert_eq!(app.screen, ScreenKind::Servers);
}

#[test]
fn button_click_records_the_vanilla_ui_sound() {
    let mut app = App::new(3);
    for _ in 0..30 {
        app.update();
    }
    app.render_title_buttons_for_test();
    let mp = app
        .buttons
        .iter()
        .find(|b| b.id == "multiplayer")
        .expect("multiplayer button")
        .clone();
    app.on_click(mp.x + mp.w / 2, mp.y + mp.h / 2);
    // vanilla AbstractButton.playDownSound: exactly one ui.button.click
    // per activated button, stamped with the pressing frame
    assert_eq!(
        app.ui_sounds,
        vec![format!("ui.button.click @{}", app.frame)]
    );
    // a click on empty space (no button) is silent
    app.on_click(1, 1);
    assert_eq!(app.ui_sounds.len(), 1, "no button -> no click sound");
    // the REALMS no-op still clicks, like vanilla — quit now wears
    // the stop chord (the flag the run loop reads), the click the
    // exit still speaks
    app.open(ScreenKind::Title);
    app.render_title_buttons_for_test();
    let quit = app
        .buttons
        .iter()
        .find(|b| b.id == "quit")
        .expect("quit button")
        .clone();
    app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
    assert_eq!(app.ui_sounds.len(), 2, "quit still clicks, like vanilla");
    assert_eq!(app.screen, ScreenKind::Title);
}

// ---- server-list refresh: the REAL mc-protocol status ping ----

#[test]
fn server_addresses_parse_with_the_vanilla_default_port() {
    assert_eq!(
        parse_address("hypixel.net"),
        ("hypixel.net".to_string(), 25565)
    );
    assert_eq!(
        parse_address("localhost:25500"),
        ("localhost".to_string(), 25500)
    );
    // non-numeric / empty ports leave the whole string as the host
    assert_eq!(
        parse_address("host:notaport"),
        ("host:notaport".to_string(), 25565)
    );
    assert_eq!(
        parse_address("mc.example.com:"),
        ("mc.example.com:".to_string(), 25565)
    );
}

#[test]
fn refresh_pings_only_rows_with_real_addresses() {
    let mut app = App::new(3);
    for _ in 0..30 {
        app.update();
    }
    app.open(ScreenKind::Servers);
    // render once to build the servers-screen buttons (the helper
    // renders whichever screen is active)
    app.render_title_buttons_for_test();
    let refresh = app
        .buttons
        .iter()
        .find(|b| b.id == "refresh")
        .expect("refresh button")
        .clone();
    app.on_click(refresh.x + refresh.w / 2, refresh.y + refresh.h / 2);
    // the sandbox row has no address and stays unpinged; hypixel.net
    // is requested with the vanilla default port
    assert_eq!(app.net_requests.len(), 1);
    let req = &app.net_requests[0];
    assert_eq!(req.entry, 1);
    assert_eq!(req.host, "hypixel.net");
    assert_eq!(req.port, 25565);
    assert_eq!(req.id, 0);
    // and the click still played the vanilla ui sound
    assert_eq!(app.ui_sounds.len(), 1);
}

#[test]
fn ping_result_updates_the_row_in_place() {
    let mut app = App::new(3);
    app.open(ScreenKind::Servers);
    app.refresh_servers();
    let id = app.net_requests[0].id;
    app.ping_finished(
        id,
        Ok(PingInfo {
            motd: "A Minecraft Server".to_string(),
            version: "26.2".to_string(),
            ping_ms: 42,
        }),
    );
    let s = &app.servers[1];
    assert!(s.online);
    assert_eq!(s.motd, "A Minecraft Server");
    assert_eq!(s.version, "26.2");
    assert_eq!(s.ping_ms, 42);
    // unknown/already-consumed ids are ignored, the log stays intact
    app.ping_finished(999, Err("nope".into()));
    assert!(app.servers[1].online);
    assert_eq!(app.net_requests.len(), 1);
}

#[test]
fn unreachable_ping_renders_the_vanilla_offline_row() {
    let mut app = App::new(3);
    app.open(ScreenKind::Servers);
    app.refresh_servers();
    let id = app.net_requests[0].id;
    app.servers[1].online = true; // cached list says online until pinged
    app.ping_finished(id, Err("tcp connect to 127.0.0.1:1".into()));
    let s = &app.servers[1];
    assert!(!s.online);
    assert_eq!(s.ping_ms, 0);
    assert_eq!(s.ping_icon(), "server_list/unreachable");
}

#[test]
fn the_login_handshake_speaks_the_generated_manifest() {
    // the wire protocol number the window worker passes to the login
    // is `mc_protocol::generated::pack_claim::PROTOCOL_VERSION` (the
    // manifest emitter REFUSES a jar that does not state the number —
    // it can never be the -1 "unknown" the build_info stamp may
    // legally carry). build_info.rs is a SECOND generated surface for
    // the same jar fact (the full `generate` flow's stamp, from the
    // v0.2.0 era), so the two are CROSS-PINNED here: if either
    // generator flow is ever re-run against a different jar or the
    // emissions drift, this test fails loudly instead of the wire
    // quietly speaking two truths (the timeline-snapshot pattern).
    assert_eq!(
        crate::build_info::PROTOCOL_VERSION,
        mc_protocol::generated::pack_claim::PROTOCOL_VERSION,
        "the two generated surfaces for the jar's protocol_version agree"
    );
    // the manifest's number is a REAL stated one, not a fallback
    assert!(
        mc_protocol::generated::pack_claim::PROTOCOL_VERSION > 0,
        "the manifest emitter refuses unstated numbers — this is 26.2's 776"
    );
}

#[test]
fn server_list_refresh_runs_a_real_status_ping() {
    // in-process vanilla-shaped status server (same dogfooding as the
    // mc-protocol login tests): handshake + request -> canned JSON
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = std::thread::spawn(move || {
        use mc_protocol::framing::PacketConn;
        use mc_protocol::varint::{write_string, write_varint};
        let (stream, _) = listener.accept().unwrap();
        let mut conn = PacketConn::new(stream);
        let hs = conn.read_packet().unwrap();
        assert_eq!(hs[0], 0x00, "handshake packet id");
        let rq = conn.read_packet().unwrap();
        assert_eq!(rq[0], 0x00, "status request packet id");
        let mut payload = Vec::new();
        write_varint(&mut payload, 0x00);
        write_string(
            &mut payload,
            r#"{"version":{"name":"26.2","protocol":776},"players":{"online":3,"max":42},"description":{"text":"mc2rs test server"}}"#,
        );
        conn.send_packet(&payload).unwrap();
    });

    let mut app = App::new(3);
    app.open(ScreenKind::Servers);
    // point the hosted row at the mock server
    app.servers[1].host = Some(format!("127.0.0.1:{port}"));
    app.refresh_servers();
    let req = app.net_requests[0].clone();
    assert_eq!(req.port, port, "explicit port parsed from the address");

    // the exact mapping the window worker performs
    let res = mc_protocol::ping(&req.host, req.port)
        .map(|r| PingInfo {
            motd: r.motd,
            version: r.version_name,
            ping_ms: r.latency_ms.min(i32::MAX as u128) as i32,
        })
        .map_err(|e| format!("{e:#}"));
    server.join().unwrap();
    app.ping_finished(req.id, res);

    let s = &app.servers[1];
    assert!(s.online, "the mock server answered the status request");
    assert_eq!(s.motd, "mc2rs test server");
    assert_eq!(s.version, "26.2");
    assert!(s.ping_ms >= 0);
    assert!(s.ping_icon().starts_with("server_list/ping_"));
}

#[test]
fn join_click_fires_the_real_login_for_hosted_rows() {
    let mut app = App::new(3);
    app.open(ScreenKind::Servers);
    // select the hosted row (second row, vanilla click-to-select), then
    // click through the REAL join button
    app.render_title_buttons_for_test();
    app.on_click(80, 34 + 28 + 12);
    assert_eq!(app.selected_server, 1);
    app.render_title_buttons_for_test();
    let join = app
        .buttons
        .iter()
        .find(|b| b.id == "join")
        .expect("join button")
        .clone();
    app.on_click(join.x + join.w / 2, join.y + join.h / 2);
    // the vanilla ConnectScreen flow: one login request against the
    // hosted row's address, screen switched while the login is in flight
    assert_eq!(app.login_requests.len(), 1);
    let req = &app.login_requests[0];
    assert_eq!(req.entry, 1);
    assert_eq!(req.host, "hypixel.net");
    assert_eq!(req.port, 25565);
    assert_eq!(req.id, 0);
    assert_eq!(app.screen, ScreenKind::Connect);
    // and the click still played the vanilla ui sound
    assert_eq!(app.ui_sounds.len(), 1);
}

#[test]
fn join_on_placeholder_rows_stays_a_noop_click() {
    let mut app = App::new(3);
    app.open(ScreenKind::Servers);
    app.render_title_buttons_for_test();
    // selected row 0 = the sandbox placeholder: nothing reachable to join
    let join = app
        .buttons
        .iter()
        .find(|b| b.id == "join")
        .expect("join button")
        .clone();
    app.on_click(join.x + join.w / 2, join.y + join.h / 2);
    assert!(app.login_requests.is_empty());
    assert_eq!(app.screen, ScreenKind::Servers);
    // but the button still activated, so the click sound still fired
    // (vanilla AbstractButton.playDownSound rule)
    assert_eq!(app.ui_sounds.len(), 1);
}

#[test]
fn login_outcomes_route_like_vanilla() {
    let mut app = App::new(3);
    app.open(ScreenKind::Servers);
    app.selected_server = 1;
    app.join_selected_server();
    let id = app.login_requests[0].id;
    // server-side Login Disconnect: the official disconnect.lost title
    // + the flattened server reason
    app.login_finished(
        id,
        Ok(LoginResult::Disconnected {
            reason: "invalid session".into(),
        }),
    );
    assert_eq!(app.screen, ScreenKind::Disconnect);
    assert_eq!(
        app.disconnect_title,
        tr("disconnect.lost").unwrap_or("Connection Lost")
    );
    assert_eq!(app.disconnect_reason, "invalid session");
    // transport errors surface the same vanilla way
    app.join_selected_server();
    let id2 = app.login_requests[1].id;
    app.login_finished(id2, Err("tcp connect to 127.0.0.1:1".into()));
    assert_eq!(app.screen, ScreenKind::Disconnect);
    assert_eq!(app.disconnect_reason, "tcp connect to 127.0.0.1:1");
    // Loaded means the client genuinely holds a network world: just
    // like the original, the completed join puts you IN it — the
    // world screen renders the materialized snapshot window
    app.join_selected_server();
    let id3 = app.login_requests[2].id;
    app.login_finished(
        id3,
        Ok(LoginResult::Loaded {
            username: "mc2rs".into(),
            entity_id: 17,
            view_distance: 8,
            chunks: 1,
            world: crate::world::World {
                darkness_scale: 0.0,
                chunks_per_side: 2,
                seed: 0,
                difficulty: crate::difficulty::Difficulty::Normal,
                difficulty_locked: false,
                inhabited: None,
                // THE FIRE'S OWN QUEUE + LADDER the test stage never arms —
                // the empty store the offline walk alone fills
                fire_ticks: Vec::new(),
                fire_age: std::collections::HashMap::new(),
                portal_axis: std::collections::HashMap::new(), // the portal's own axis store: empty
                container_fill: std::collections::HashMap::new(), // the comparator lane: no fill
                frame_analog: std::collections::HashMap::new(), // the frame lane: no analog
                has_skylight: true,                            // the portal's own axis store: empty
                campfire_ticks: Vec::new(),                    // the campfire's own queue: empty
                torch_ticks: Vec::new(),                       // the torch's own queue: empty
                detector_ticks: Vec::new(),                    // the detector's own queue: empty
                comparator_ticks: Vec::new(),
                dispenser_ticks: Vec::new(),
                repeater_ticks: Vec::new(),
                observer_ticks: Vec::new(),
                piston_ticks: Vec::new(),
                moving_states: Vec::new(),
                moving_ticks: Vec::new(),
                torch_toggles: Vec::new(), // the RECENT_TOGGLES window: empty
                bamboo_ticks: Vec::new(),  // the bamboo's own queue: empty
                chunks: vec![
                    vec![
                        crate::world::AIR;
                        crate::world::CHUNK_X
                            * crate::world::CHUNK_Y
                            * crate::world::CHUNK_Z
                    ];
                    4
                ],
                light: None,
                day_tick: None,           // test scenes stay day
                biomes: None,             // test scenes carry no biome layer
                variants: None,           // test scenes carry no state variants
                ambient_light: None,      // no dimension opinion — the black bytecode default
                block_light_tint: None,   // no dimension opinion — the bytecode default tint
                sky_light_factor: None,   // no dimension opinion — the bytecode default 1.0
                sky_light_color: None,    // no dimension opinion — the -1 default's white identity
                night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
                night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
                rain_level: 0.0,          // the dry test world — no weather word rides
                thunder_level: 0.0,       // the dry test world — no storm speaks
                // THE OFFLINE SAVE'S OWN LANES the staged world never wears —
                // the gate the gen-time default keeps off, the journal the
                // session's rows never names
                journaling: false,
                edit_journal: Vec::new(),
                variant_journal: Vec::new(),
                saved_edits_mark: 0,
                saved_variants_mark: 0,
            },
            camera: None,
            spawn_biome: Some("minecraft:plains".into()),
            slim_arm: false,
            skybox: Skybox::Overworld,
            fog: None,
            cloud_fog_end: None,
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
        }),
    );
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the join lands in the network world"
    );
    assert!(app.world_is_network);
    assert_eq!(app.world_size, 32);
    assert_eq!(
        app.net_biome.as_deref(),
        Some("minecraft:plains"),
        "the spawn biome rides the adoption"
    );
    // unknown/already-consumed ids are ignored, the log stays append-only
    app.login_finished(999, Err("nope".into()));
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "an unknown id changes nothing"
    );
    assert_eq!(app.login_requests.len(), 3);
}

#[test]
fn connect_and_outcome_screens_return_to_the_server_list() {
    let mut app = App::new(3);
    app.open(ScreenKind::Servers);
    app.selected_server = 1;
    app.join_selected_server();
    assert_eq!(app.screen, ScreenKind::Connect);
    // Escape leaves the in-flight join (late results carry known ids and
    // still land — vanilla shows outcome screens whenever they arrive)
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::Servers);
    // the DisconnectedScreen's Back button returns to the list
    app.login_finished(
        app.login_requests[0].id,
        Ok(LoginResult::Disconnected {
            reason: "bye".into(),
        }),
    );
    assert_eq!(app.screen, ScreenKind::Disconnect);
    app.render_title_buttons_for_test();
    let back = app
        .buttons
        .iter()
        .find(|b| b.id == "backtoservers")
        .expect("back button")
        .clone();
    app.on_click(back.x + back.w / 2, back.y + back.h / 2);
    assert_eq!(app.screen, ScreenKind::Servers);
}

#[test]
fn server_join_runs_the_real_join_into_the_world() {
    // in-process vanilla-shaped join server (same dogfooding as the
    // mc-protocol config/play tests): handshake + login start ->
    // compression switch -> Login Success -> introductions -> probes +
    // finish -> the play state: world info, center chunk, chunk batch,
    // a REAL chunk (stone+dirt), keep-alive, ping, position teleport —
    // the worker pumps all the way to the world-load milestone
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = std::thread::spawn(move || {
        use mc_protocol::framing::PacketConn;
        use mc_protocol::varint::{read_string, read_varint, write_string, write_varint};
        let (stream, _) = listener.accept().unwrap();
        let mut conn = PacketConn::new(stream);
        let hs = conn.read_packet().unwrap();
        let mut c: &[u8] = &hs;
        assert_eq!(read_varint(&mut c).unwrap(), 0x00, "handshake packet id");
        assert_eq!(read_varint(&mut c).unwrap(), 776, "REAL protocol number");
        let _host = read_string(&mut c).unwrap();
        let mut port_b = [0u8; 2];
        std::io::Read::read_exact(&mut c, &mut port_b).unwrap();
        assert_eq!(read_varint(&mut c).unwrap(), 2, "handshake targets login");
        // login start BEFORE the compression switch (it travels
        // uncompressed — the client has not seen Set Compression yet)
        let start = conn.read_packet().unwrap();
        let mut c: &[u8] = &start;
        assert_eq!(read_varint(&mut c).unwrap(), 0x00, "login start id");
        let name = read_string(&mut c).unwrap();
        assert_eq!(name, crate::screens::PLAYER_NAME);
        conn.send_packet(&mc_protocol::login::encode_set_compression(256))
            .unwrap();
        conn.enable_compression(256);
        conn.send_packet(&mc_protocol::login::encode_login_success(
            &mc_protocol::login::offline_uuid(&name),
            &name,
        ))
        .unwrap();
        // the introductions: login acknowledged, client information,
        // plugin brand — ids from the GENERATED tables (all three
        // kept their wiki-era numbers; now they're pinned to the
        // runtime truth instead of remembered constants)
        for expected in [
            mc_protocol::generated::packet_ids::LSB_LOGIN_ACKNOWLEDGED,
            mc_protocol::generated::packet_ids::CSB_CLIENT_INFORMATION,
            mc_protocol::generated::packet_ids::CSB_CUSTOM_PAYLOAD,
        ] {
            let pkt = conn.read_packet().unwrap();
            let mut c: &[u8] = &pkt;
            assert_eq!(read_varint(&mut c).unwrap(), expected, "introduction id");
        }
        // Registry Data (a real vanilla server ALWAYS streams it in
        // the configuration phase): the shared mock fixture's
        // dimension_type registry (overworld -64/384) — the same
        // bytes the mc-protocol arc mock sends. Without it the join's
        // snapshot carries no layout knowledge and the world window
        // honestly falls back to the column-local copy. The BIOME
        // registry rides alongside (one entry: swamp, id 0) — the
        // synced name table the chunk biome palettes resolve through.
        conn.send_packet(&mc_protocol::config::test_support::registry_packet(true))
            .unwrap();
        conn.send_packet(&mc_protocol::config::test_support::biome_registry_packet(
            "minecraft:swamp",
        ))
        .unwrap();
        // probes + finish — ids from the GENERATED configuration
        // table (4/5/3 on the real 26.2 wire; the wiki-era literals
        // these replaced were one low)
        let mut ka = Vec::new();
        write_varint(&mut ka, mc_protocol::config::clientbound::KEEP_ALIVE);
        ka.extend_from_slice(&0x1122334455667788u64.to_be_bytes());
        conn.send_packet(&ka).unwrap();
        let mut pg = Vec::new();
        write_varint(&mut pg, mc_protocol::config::clientbound::PING);
        pg.extend_from_slice(&77i32.to_be_bytes());
        conn.send_packet(&pg).unwrap();
        let mut fin = Vec::new();
        write_varint(&mut fin, mc_protocol::config::clientbound::FINISH);
        conn.send_packet(&fin).unwrap();
        // responses: keep-alive echo, pong, ack — ids from the same
        // generated truth
        let resp = conn.read_packet().unwrap();
        let mut c: &[u8] = &resp;
        assert_eq!(
            read_varint(&mut c).unwrap(),
            mc_protocol::config::serverbound::KEEP_ALIVE,
            "keep alive response id"
        );
        assert_eq!(
            i64::from_be_bytes(c[..8].try_into().unwrap()),
            0x1122334455667788
        );
        let resp = conn.read_packet().unwrap();
        let mut c: &[u8] = &resp;
        assert_eq!(
            read_varint(&mut c).unwrap(),
            mc_protocol::config::serverbound::PONG,
            "pong id"
        );
        assert_eq!(i32::from_be_bytes(c[..4].try_into().unwrap()), 77);
        let resp = conn.read_packet().unwrap();
        let mut c: &[u8] = &resp;
        assert_eq!(
            read_varint(&mut c).unwrap(),
            mc_protocol::config::serverbound::ACKNOWLEDGE_FINISH,
            "ack finish id"
        );
        // ---- the play state: the world burst ----
        use mc_protocol::play::{clientbound, serverbound};
        let mut login_pkt = Vec::new();
        write_varint(&mut login_pkt, clientbound::LOGIN);
        login_pkt.extend_from_slice(&17i32.to_be_bytes()); // entity id
        login_pkt.push(0); // not hardcore
        write_varint(&mut login_pkt, 1); // one dimension
        write_string(&mut login_pkt, "minecraft:overworld");
        write_varint(&mut login_pkt, 20); // max players (dropped)
        write_varint(&mut login_pkt, 8); // chunk radius (view distance)
        write_varint(&mut login_pkt, 8); // simulation distance
        login_pkt.push(0); // reduced debug (dropped)
        login_pkt.push(1); // show death screen (dropped)
        login_pkt.push(0); // do limited crafting (dropped)
                           // CommonPlayerSpawnInfo (vanilla 26.2): dimension_type id,
                           // dimension, seed, game types, debug/flat, no death location,
                           // portal cooldown, sea level
        write_varint(&mut login_pkt, 0);
        write_string(&mut login_pkt, "minecraft:overworld");
        login_pkt.extend_from_slice(&0i64.to_be_bytes()); // seed
        login_pkt.push(1); // game type
        login_pkt.push(1); // previous game type
        login_pkt.push(0); // is debug
        login_pkt.push(0); // is flat
        login_pkt.push(0); // no death location
        write_varint(&mut login_pkt, 300); // portal cooldown
        write_varint(&mut login_pkt, 63); // sea level
        login_pkt.push(1); // online mode (dropped)
        login_pkt.push(1); // enforces secure chat (dropped)
        conn.send_packet(&login_pkt).unwrap();
        let mut cc = Vec::new();
        write_varint(&mut cc, clientbound::CENTER_CHUNK);
        write_varint(&mut cc, 3);
        write_varint(&mut cc, -2);
        conn.send_packet(&cc).unwrap();
        // chunk batch start -> the pacing reply MUST arrive
        let mut bs = Vec::new();
        write_varint(&mut bs, clientbound::BATCH_START);
        conn.send_packet(&bs).unwrap();
        let resp = conn.read_packet().expect("chunk batch received");
        let mut c: &[u8] = &resp;
        assert_eq!(
            read_varint(&mut c).unwrap(),
            serverbound::CHUNK_BATCH_RECEIVED,
            "batch reply id"
        );
        let pace = f32::from_be_bytes(c[..4].try_into().unwrap());
        assert!(pace > 0.0, "server must be asked to keep streaming");
        // the real chunk (section 0): stone at (0,0,0), dirt at (1,2,3)
        // — the data array carries PALETTE INDICES, not global ids
        let palette = [0u32, 1, 10]; // air, stone, dirt
        let idx = |x: usize, y: usize, z: usize| (y << 8) | (z << 4) | x;
        let mut entries = vec![0u32; 4096];
        entries[idx(0, 0, 0)] = 1;
        entries[idx(3, 1, 2)] = 2;
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&2i16.to_be_bytes()); // block count
        buffer.push(5); // bits per entry -> indirect palette
        write_varint(&mut buffer, palette.len() as i32);
        for id in palette {
            write_varint(&mut buffer, id as i32);
        }
        buffer.extend_from_slice(&mc_protocol::pack_entries(&entries, 5));
        buffer.push(0); // biomes: single valued
        write_varint(&mut buffer, 0); // the synced biome: swamp (id 0)
                                      // a real vanilla join streams the FULL column (24 sections for
                                      // an overworld): 7 air sections below put the stone/dirt
                                      // section at index 7 — world y 48..63 with min_y -64, surface
                                      // terrain right under the teleport's feet y 64. The
                                      // player-anchored window (base 48 = 64 - 16) then shows the
                                      // stone at grid y 0 — a bottom-anchored copy would miss it.
                                      // An 8th air section (index 8, world y 64..79) sits UNDER THE
                                      // PLAYER: its biome cells are what the music situation reads.
        let mut air = Vec::new();
        air.extend_from_slice(&0i16.to_be_bytes()); // block count 0
        air.push(0); // bits per entry: single valued
        write_varint(&mut air, 0); // the single value: air
        air.push(0); // biomes: single valued
        write_varint(&mut air, 0); // the synced biome: swamp (id 0)
        let mut column = Vec::new();
        for _ in 0..7 {
            column.extend_from_slice(&air); // sections arrive bottom-first
        }
        column.extend_from_slice(&buffer);
        column.extend_from_slice(&air); // section 8: the player's own
        let buffer = column;
        let mut cd = Vec::new();
        write_varint(&mut cd, clientbound::CHUNK_DATA);
        cd.extend_from_slice(&3i32.to_be_bytes());
        cd.extend_from_slice(&(-2i32).to_be_bytes());
        cd.push(0x0A);
        cd.push(0x00); // empty heightmaps compound
        write_varint(&mut cd, buffer.len() as i32);
        cd.extend_from_slice(&buffer);
        write_varint(&mut cd, 0); // no block entities
        cd.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // light masks
        conn.send_packet(&cd).unwrap();
        // keep alive -> echo (serverbound play 0x14)
        let mut kap = Vec::new();
        write_varint(&mut kap, clientbound::KEEP_ALIVE);
        kap.extend_from_slice(&(-5_000_000_000i64).to_be_bytes());
        conn.send_packet(&kap).unwrap();
        let resp = conn.read_packet().expect("play keep alive echo");
        let mut c: &[u8] = &resp;
        assert_eq!(read_varint(&mut c).unwrap(), serverbound::KEEP_ALIVE);
        assert_eq!(
            i64::from_be_bytes(c[..8].try_into().unwrap()),
            -5_000_000_000
        );
        // ping -> pong (serverbound play 0x22)
        let mut pgp = Vec::new();
        write_varint(&mut pgp, clientbound::PING);
        pgp.extend_from_slice(&(-7i32).to_be_bytes());
        conn.send_packet(&pgp).unwrap();
        let resp = conn.read_packet().expect("play pong");
        let mut c: &[u8] = &resp;
        assert_eq!(read_varint(&mut c).unwrap(), serverbound::PONG);
        assert_eq!(i32::from_be_bytes(c[..4].try_into().unwrap()), -7);
        // position teleport id 42 -> confirm teleportation (0x00).
        // The spawn sits INSIDE the centered chunk (3,-2) — a real
        // vanilla join burst is exactly this: the player position
        // within the view-distance center. floor(56.5)>>4 = 3,
        // floor(-27.25)>>4 = -2. The javap-proven 26.2 shape:
        // teleport id FIRST, then the PositionMoveRotation (position
        // + delta movement Vec3s + yaw/pitch), then the int flags.
        let mut pos = Vec::new();
        write_varint(&mut pos, clientbound::SYNCHRONIZE_POSITION);
        write_varint(&mut pos, 42);
        pos.extend_from_slice(&56.5f64.to_be_bytes());
        pos.extend_from_slice(&64.0f64.to_be_bytes());
        pos.extend_from_slice(&(-27.25f64).to_be_bytes());
        pos.extend_from_slice(&0.0f64.to_be_bytes()); // delta movement
        pos.extend_from_slice(&0.0f64.to_be_bytes());
        pos.extend_from_slice(&0.0f64.to_be_bytes());
        pos.extend_from_slice(&90.0f32.to_be_bytes());
        pos.extend_from_slice(&0.0f32.to_be_bytes());
        pos.extend_from_slice(&0i32.to_be_bytes()); // all values absolute
        conn.send_packet(&pos).unwrap();
        let resp = conn.read_packet().expect("confirm teleportation");
        let mut c: &[u8] = &resp;
        assert_eq!(
            read_varint(&mut c).unwrap(),
            serverbound::CONFIRM_TELEPORTATION,
            "confirm id"
        );
        assert_eq!(read_varint(&mut c).unwrap(), 42, "teleport id verbatim");
    });

    let mut app = App::new(3);
    app.open(ScreenKind::Servers);
    // point the hosted row at the mock join server and join it
    app.servers[1].host = Some(format!("127.0.0.1:{port}"));
    app.selected_server = 1;
    app.join_selected_server();
    let req = app.login_requests[0].clone();
    assert_eq!(req.port, port, "explicit port parsed from the address");

    // the exact mapping the window worker performs: the milestone
    // adopts into the LIVE view and the first refresh IS the one-shot
    // snapshot view (the example mapper — the ids the mock streams;
    // the GUI resolves through the GENERATED registry table) — plus
    // the teleport's exact camera pose from the same decisions
    let res = mc_protocol::connect_and_join_world(
        &req.host,
        req.port,
        mc_protocol::generated::pack_claim::PROTOCOL_VERSION,
        crate::screens::PLAYER_NAME,
    )
    .map(|out| match out {
        mc_protocol::WorldOutcome::Loaded(snap) => {
            let (entity_id, view_distance, chunks) = (
                snap.info.entity_id,
                snap.info.view_distance,
                snap.chunks.len(),
            );
            // the worker's exact resolution order: the skybox resolves
            // through the model the snapshot adopts into (the synced
            // registry's own skybox is the server's truth — the wire
            // decode; the codec default when the element states
            // nothing; the built-in table on a skip-sync)
            let view = crate::world_view::LiveView::from_snapshot(snap, WORLD_CHUNKS);
            let skybox = view.model.skybox();
            let fog = view.model.fog_distances();
            let cloud_fog_end = view.model.cloud_fog_end_distance();
            let first = view.refresh(|id| match id {
                1 => crate::world::STONE,
                10 => crate::world::DIRT,
                _ => crate::world::AIR,
            });
            LoginResult::Loaded {
                username: crate::screens::PLAYER_NAME.to_string(),
                entity_id,
                view_distance,
                chunks,
                world: first.world,
                camera: first.camera,
                spawn_biome: first.biome,
                slim_arm: false,
                skybox,
                fog,
                cloud_fog_end,
                entity_types: Vec::new(),
                entity_type_tags: Vec::new(),
            }
        }
        mc_protocol::WorldOutcome::Disconnected { reason } => LoginResult::Disconnected { reason },
    })
    .map_err(|e| format!("{e:#}"));
    server.join().unwrap();
    app.login_finished(req.id, res);

    // the join landed IN the world (vanilla ConnectScreen → world):
    // the world screen renders the materialized network window, the
    // mock column (3,-2) sits at grid chunk (2,2) of the 5x5 window.
    // The stone at world y 48 landing at grid y 0 PINS the
    // player-anchored vertical placement (feet 64 - anchor 16 = base
    // 48): a bottom-anchored copy would show world -64..-16 — all air
    // this far above the one real section's world y 48.
    assert_eq!(app.screen, ScreenKind::World);
    assert!(
        app.world_is_network,
        "the world screen shows the network world"
    );
    assert_eq!(
        app.world_size,
        (WORLD_CHUNKS * crate::world::CHUNK_X) as i32
    );
    let w = app.net_world.as_ref().expect("network world retained");
    assert_eq!(
        w.get(32, 0, 32),
        crate::world::STONE,
        "stone at world 48 = feet 64 - 16"
    );
    assert_eq!(w.get(35, 1, 34), crate::world::DIRT, "dirt at world 49");
    assert_eq!(
        w.get(32, 16, 32),
        crate::world::AIR,
        "the feet row (world 64) is sky"
    );
    assert!(
        !app.world_quads
            .as_ref()
            .expect("meshed network world")
            .is_empty(),
        "the network terrain meshes"
    );
    // the camera column (grid center) is air in this one-column world:
    // the spawn scan honestly reports the grid top
    assert_eq!(app.spawn_y, crate::world::CHUNK_Y as i32);
    // the camera is the teleport's EXACT pose in the grid frame:
    // feet (56.5, 64, -27.25) with the window origin (1,-4) and base
    // 48 -> grid (40.5, 16, 36.75) + the standing eye height; the
    // server's yaw 90 (facing west) converts to camera yaw 90+180;
    // pitch 0 stays 0. The world screen renders FROM the player.
    let cam = app.net_camera.expect("the join shipped the teleport pose");
    assert_eq!(cam.pos.x, 40.5);
    assert_eq!(
        cam.pos.y,
        64.0 - 48.0 + crate::world_view::PLAYER_EYE_HEIGHT
    );
    assert_eq!(cam.pos.z, 36.75);
    assert_eq!(cam.yaw, 90.0f32.to_radians() + std::f32::consts::PI);
    assert_eq!(cam.pitch, 0.0);
    assert_eq!(
        app.world_camera(),
        cam,
        "the world screen renders FROM the teleport pose"
    );
    // the REAL spawn biome: the synced registry (one entry, swamp) +
    // the section under the player resolve to the situation's key,
    // and the music is that biome's official background-music event
    assert_eq!(
        app.net_biome.as_deref(),
        Some("minecraft:swamp"),
        "the biome at the player's feet, decoded from the sections"
    );
    use mc_registry::generated::worldgen as wm;
    let expect = wm::biome_music("minecraft:swamp", "default")
        .map(|m| m.event)
        .unwrap_or("music.game");
    assert_eq!(
        app.music.game_tracks(),
        [expect],
        "the join's music situation is the spawn biome's own event"
    );
}

#[test]
fn network_world_lands_in_the_world_screen_and_quits_cleanly() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    // an all-air network window: the state machine must not care
    app.net_world = Some(crate::world::World {
        darkness_scale: 0.0,
        chunks_per_side: 2,
        seed: 0,
        difficulty: crate::difficulty::Difficulty::Normal,
        difficulty_locked: false,
        inhabited: None,
        // THE FIRE'S OWN QUEUE + LADDER the test stage never arms —
        // the empty store the offline walk alone fills
        fire_ticks: Vec::new(),
        fire_age: std::collections::HashMap::new(),
        portal_axis: std::collections::HashMap::new(), // the portal's own axis store: empty
        container_fill: std::collections::HashMap::new(), // the comparator lane: no fill
        frame_analog: std::collections::HashMap::new(), // the frame lane: no analog
        has_skylight: true,                            // the portal's own axis store: empty
        campfire_ticks: Vec::new(),                    // the campfire's own queue: empty
        torch_ticks: Vec::new(),                       // the torch's own queue: empty
        detector_ticks: Vec::new(),                    // the detector's own queue: empty
        comparator_ticks: Vec::new(),
        dispenser_ticks: Vec::new(),
        repeater_ticks: Vec::new(),
        observer_ticks: Vec::new(),
        piston_ticks: Vec::new(),
        moving_states: Vec::new(),
        moving_ticks: Vec::new(),
        torch_toggles: Vec::new(), // the RECENT_TOGGLES window: empty
        bamboo_ticks: Vec::new(),  // the bamboo's own queue: empty
        chunks: vec![
            vec![
                crate::world::AIR;
                crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z
            ];
            4
        ],
        light: None,
        day_tick: None,           // test scenes stay day
        biomes: None,             // test scenes carry no biome layer
        variants: None,           // test scenes carry no state variants
        ambient_light: None,      // no dimension opinion — the black bytecode default
        block_light_tint: None,   // no dimension opinion — the bytecode default tint
        sky_light_factor: None,   // no dimension opinion — the bytecode default 1.0
        sky_light_color: None,    // no dimension opinion — the -1 default's white identity
        night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
        night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
        rain_level: 0.0,          // the dry test world — no weather word rides
        thunder_level: 0.0,       // the dry test world — no storm speaks
        // THE OFFLINE SAVE'S OWN LANES the staged world never wears —
        // the gate the gen-time default keeps off, the journal the
        // session's rows never names
        journaling: false,
        edit_journal: Vec::new(),
        variant_journal: Vec::new(),
        saved_edits_mark: 0,
        saved_variants_mark: 0,
    });
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World);
    assert!(app.world_is_network);
    assert_eq!(app.world_size, 32);
    assert!(
        app.net_biome.is_none(),
        "no biome shipped, no situation claimed"
    );
    assert_eq!(
        app.spawn_y,
        crate::world::CHUNK_Y as i32,
        "all-air window: the spawn scan reports the grid top"
    );
    assert!(
        app.world_quads.as_ref().unwrap().is_empty(),
        "an air world meshes to nothing"
    );

    // save and quit unloads the network level
    app.paused = true;
    app.render_title_buttons_for_test();
    let quit = app
        .buttons
        .iter()
        .find(|b| b.id == "saveandquit")
        .expect("saveandquit")
        .clone();
    app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
    assert_eq!(app.screen, ScreenKind::Title);
    assert!(!app.world_is_network);
    assert!(app.world_quads.is_none(), "the network mesh is unloaded");
    assert!(
        app.net_camera.is_none(),
        "the teleport pose unloads with the level"
    );
    assert!(
        app.net_biome.is_none(),
        "the biome situation unloads with the level"
    );

    // singleplayer afterwards rebuilds the PROCEDURAL world
    app.render_title_buttons_for_test();
    let sp = app
        .buttons
        .iter()
        .find(|b| b.id == "singleplayer")
        .expect("singleplayer")
        .clone();
    app.on_click(sp.x + sp.w / 2, sp.y + sp.h / 2);
    assert_eq!(app.screen, ScreenKind::World);
    assert!(
        !app.world_is_network,
        "singleplayer renders the procedural world"
    );
    assert!(
        !app.world_quads.unwrap().is_empty(),
        "procedural terrain meshes"
    );
}

#[test]
fn world_camera_follows_the_teleport_and_falls_back_to_the_flight_camera() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    // a network world installed WITHOUT a pose (no join carried it):
    // the deterministic flight camera stands — never a guess about
    // where the player is
    app.net_world = Some(crate::world::World {
        darkness_scale: 0.0,
        chunks_per_side: 2,
        seed: 0,
        difficulty: crate::difficulty::Difficulty::Normal,
        difficulty_locked: false,
        inhabited: None,
        // THE FIRE'S OWN QUEUE + LADDER the test stage never arms —
        // the empty store the offline walk alone fills
        fire_ticks: Vec::new(),
        fire_age: std::collections::HashMap::new(),
        portal_axis: std::collections::HashMap::new(), // the portal's own axis store: empty
        container_fill: std::collections::HashMap::new(), // the comparator lane: no fill
        frame_analog: std::collections::HashMap::new(), // the frame lane: no analog
        has_skylight: true,                            // the portal's own axis store: empty
        campfire_ticks: Vec::new(),                    // the campfire's own queue: empty
        torch_ticks: Vec::new(),                       // the torch's own queue: empty
        detector_ticks: Vec::new(),                    // the detector's own queue: empty
        comparator_ticks: Vec::new(),
        dispenser_ticks: Vec::new(),
        repeater_ticks: Vec::new(),
        observer_ticks: Vec::new(),
        piston_ticks: Vec::new(),
        moving_states: Vec::new(),
        moving_ticks: Vec::new(),
        torch_toggles: Vec::new(), // the RECENT_TOGGLES window: empty
        bamboo_ticks: Vec::new(),  // the bamboo's own queue: empty
        chunks: vec![
            vec![
                crate::world::AIR;
                crate::world::CHUNK_X * crate::world::CHUNK_Y * crate::world::CHUNK_Z
            ];
            4
        ],
        light: None,
        day_tick: None,           // test scenes stay day
        biomes: None,             // test scenes carry no biome layer
        variants: None,           // test scenes carry no state variants
        ambient_light: None,      // no dimension opinion — the black bytecode default
        block_light_tint: None,   // no dimension opinion — the bytecode default tint
        sky_light_factor: None,   // no dimension opinion — the bytecode default 1.0
        sky_light_color: None,    // no dimension opinion — the -1 default's white identity
        night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
        night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
        rain_level: 0.0,          // the dry test world — no weather word rides
        thunder_level: 0.0,       // the dry test world — no storm speaks
        // THE OFFLINE SAVE'S OWN LANES the staged world never wears —
        // the gate the gen-time default keeps off, the journal the
        // session's rows never names
        journaling: false,
        edit_journal: Vec::new(),
        variant_journal: Vec::new(),
        saved_edits_mark: 0,
        saved_variants_mark: 0,
    });
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let flight = app.world_camera();
    let n = app.world_size as f32;
    assert_eq!(
        flight.pos,
        Vec3::new(
            n / 2.0,
            app.spawn_y as f32 + 12.0 + crate::world_view::PLAYER_EYE_HEIGHT,
            n / 2.0
        ),
        "the flight camera over the spawn surface, eye 1.62 up"
    );
    assert_eq!(
        flight.yaw,
        std::f32::consts::FRAC_PI_2,
        "facing east at dawn"
    );
    assert_eq!(flight.pitch, -0.08);
    // a REAL pose (what the join worker ships) takes over: the world
    // screen renders from the player, not from the stand-in
    let pose = crate::camera::Camera {
        pos: Vec3::new(40.5, 17.62, 36.75),
        yaw: 5.5,
        pitch: 0.1,
    };
    app.net_camera = Some(pose);
    assert_eq!(
        app.world_camera(),
        pose,
        "the teleport pose drives the camera"
    );
}

#[test]
fn late_loaded_join_after_escape_is_dropped() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.on_escape(); // the user cancelled the join
    assert_eq!(app.screen, ScreenKind::Servers);
    app.login_requests.push(LoginRequest {
        id: 7,
        entry: 0,
        host: "127.0.0.1".to_string(),
        port: 1,
    });
    app.login_finished(
        7,
        Ok(LoginResult::Loaded {
            username: crate::screens::PLAYER_NAME.to_string(),
            entity_id: 1,
            view_distance: 8,
            chunks: 1,
            world: crate::world::World {
                darkness_scale: 0.0,
                chunks_per_side: 2,
                seed: 0,
                difficulty: crate::difficulty::Difficulty::Normal,
                difficulty_locked: false,
                inhabited: None,
                // THE FIRE'S OWN QUEUE + LADDER the test stage never arms —
                // the empty store the offline walk alone fills
                fire_ticks: Vec::new(),
                fire_age: std::collections::HashMap::new(),
                portal_axis: std::collections::HashMap::new(), // the portal's own axis store: empty
                container_fill: std::collections::HashMap::new(), // the comparator lane: no fill
                frame_analog: std::collections::HashMap::new(), // the frame lane: no analog
                has_skylight: true,                            // the portal's own axis store: empty
                campfire_ticks: Vec::new(),                    // the campfire's own queue: empty
                torch_ticks: Vec::new(),                       // the torch's own queue: empty
                detector_ticks: Vec::new(),                    // the detector's own queue: empty
                comparator_ticks: Vec::new(),
                dispenser_ticks: Vec::new(),
                repeater_ticks: Vec::new(),
                observer_ticks: Vec::new(),
                piston_ticks: Vec::new(),
                moving_states: Vec::new(),
                moving_ticks: Vec::new(),
                torch_toggles: Vec::new(), // the RECENT_TOGGLES window: empty
                bamboo_ticks: Vec::new(),  // the bamboo's own queue: empty
                chunks: vec![
                    vec![
                        crate::world::AIR;
                        crate::world::CHUNK_X
                            * crate::world::CHUNK_Y
                            * crate::world::CHUNK_Z
                    ];
                    4
                ],
                light: None,
                day_tick: None,           // test scenes stay day
                biomes: None,             // test scenes carry no biome layer
                variants: None,           // test scenes carry no state variants
                ambient_light: None,      // no dimension opinion — the black bytecode default
                block_light_tint: None,   // no dimension opinion — the bytecode default tint
                sky_light_factor: None,   // no dimension opinion — the bytecode default 1.0
                sky_light_color: None,    // no dimension opinion — the -1 default's white identity
                night_vision_color: None, // no dimension opinion — the #999999 default, factor 0.0
                night_vision_factor: 0.0, // no effect store rides these scenes — the golden floor
                rain_level: 0.0,          // the dry test world — no weather word rides
                thunder_level: 0.0,       // the dry test world — no storm speaks
                // THE OFFLINE SAVE'S OWN LANES the staged world never wears —
                // the gate the gen-time default keeps off, the journal the
                // session's rows never names
                journaling: false,
                edit_journal: Vec::new(),
                variant_journal: Vec::new(),
                saved_edits_mark: 0,
                saved_variants_mark: 0,
            },
            camera: None,
            spawn_biome: None,
            slim_arm: false,
            skybox: Skybox::Overworld,
            fog: None,
            cloud_fog_end: None,
            entity_types: Vec::new(),
            entity_type_tags: Vec::new(),
        }),
    );
    assert_eq!(
        app.screen,
        ScreenKind::Servers,
        "a cancelled join's late success is dropped"
    );
    assert!(app.net_world.is_none());
}

/// THE DAY CLOCK'S WIRE RE-SYNC (vanilla ClientLevel's own absolute
/// SetTime fold): the join snapshot's day opinion re-bases the
/// fixed clock BEFORE the first frame renders, the 250ms refresh
/// lane re-absorbs the drift, and a world with NO opinion (no Set
/// Time ever arrived) is the honest no-op — the clock keeps the
/// client's own base.
#[test]
fn the_wire_day_time_rebases_the_fixed_clock() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    // the join snapshot carries the server's Set Time opinion
    let mut joined = air_net_world(2, 0);
    joined.day_tick = Some(18_000);
    app.net_world = Some(joined);
    app.frame = 500;
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert!(app.world_is_network);
    // base + frame reads the server's day NOW (the first frame
    // renders the server's hour, never the client's dawn)
    assert_eq!(
        app.day_time_now(),
        18_000,
        "the join's own clock lands the server's day"
    );

    // the refresh lane's own re-sync: the fixed clock ADVANCED
    // (the local 20-tps law) and drifted off the server's truth —
    // the shipped window re-bases the pair to the wire's day
    app.frame = 700;
    let mut drifted = air_net_world(2, 0);
    drifted.day_tick = Some(18_100);
    app.live_view_updated(drifted, None, None, None);
    assert_eq!(app.day_time_now(), 18_100, "the refresh absorbs the drift");

    // the no-opinion arm: a world without Set Time never touches
    // the clock (the honest no-op — the client's own base stands)
    let silent = air_net_world(2, 0);
    app.live_view_updated(silent, None, None, None);
    assert_eq!(app.day_time_now(), 18_100, "no opinion keeps the clock");
}

/// THE WIRE MOBS' LERPTO LAW (javap LivingEntity's remote walk):
/// the refresh arms the 3-tick ease, the ticks land EXACTLY on the
/// server's word, the walk store speaks the stride, the head
/// offset reads the wrapped difference, the missing ids despawn.
#[test]
fn the_wire_mobs_walk_the_lerpto_law() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the spawn lands at the pose (the prev trio arms at it)
    app.live_mobs_updated(vec![wire_zombie(7, 10.0, 64.0, 10.0)]);
    assert_eq!(app.net_mobs.len(), 1, "the spawn rendered v1");
    assert_eq!(app.net_mobs[0].id, 7);
    assert_eq!(
        app.net_mobs[0].head_yaw_off, -30.0,
        "the head reads the wrap"
    );
    // the server's update: the target arms, the 3-tick walk eases
    app.live_mobs_updated(vec![wire_zombie(7, 16.0, 64.0, 10.0)]);
    app.tick_net_mobs();
    assert!(
        (app.net_mobs[0].x - 12.0).abs() < 1e-9,
        "one third of the way"
    );
    app.tick_net_mobs();
    assert!((app.net_mobs[0].x - 14.0).abs() < 1e-9, "two thirds");
    app.tick_net_mobs();
    assert_eq!(app.net_mobs[0].x, 16.0, "the exact landing");
    assert_eq!(app.net_mobs[0].lerp_steps, 0, "the walk spent");
    // the walk store's feed: the moved ticks spoke (the limbs swing)
    assert!(app.net_mobs[0].walk.position > 0.0, "the stride speaks");
    // the despawn: the wire stops naming id 7 — no ghost
    app.live_mobs_updated(vec![]);
    assert!(app.net_mobs.is_empty(), "the server's truth, no ghosts");
}

/// THE WIRE ARROWS' FLIGHT LANE (javap `AbstractArrow.
/// recreateFromPacket` the 26.2 walk the vanilla client speaks):
/// the Add Entity word becomes the CLIENT-SIDE projectile (the
/// pose AND the motion the packet's own fields keep), the flight
/// the client's own tick simulates (the move the velocity speaks,
/// the drag the 0.99 keeps, the gravity the 0.05 rides), the
/// still-named id rides its OWN sim untouched (the between-sync
/// truth the client's physics owns), the wire's silence drops the
/// shot (no ghosts), the offline lane never enters the lane.
#[test]
fn the_wire_arrows_fly_the_client_sim() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the spawn lands at the word (the prev trio arms at it — the
    // render's first lerp never moves)
    app.live_net_arrows_updated(vec![crate::world_view::WireArrow {
        id: 300,
        x: 10.0,
        y: 66.0,
        z: 10.0,
        vx: 2.4,
        vy: 0.0,
        vz: 0.0,
        yaw: -90.0,
        pitch: 0.0,
        crit: false,
    }]);
    assert_eq!(app.net_arrows.len(), 1, "the shot rendered v1");
    assert_eq!(app.net_arrows[0].id, 300);
    assert_eq!(
        (
            app.net_arrows[0].x,
            app.net_arrows[0].y,
            app.net_arrows[0].z
        ),
        (10.0, 66.0, 10.0),
        "the spawn IS the packet's word"
    );
    assert_eq!(
        (
            app.net_arrows[0].prev_x,
            app.net_arrows[0].prev_y,
            app.net_arrows[0].prev_z
        ),
        (10.0, 66.0, 10.0),
        "the prev trio arms at the spawn"
    );
    // THE FLIGHT (the client's own AbstractArrow.tick): the move
    // the velocity speaks (x += vx), then the drag and the
    // gravity ride the velocity's own tail (0.99, 0.05)
    app.tick_net_arrows();
    assert!(
        (app.net_arrows[0].x - 12.4).abs() < 1e-9,
        "the move the velocity speaks"
    );
    assert!(
        (app.net_arrows[0].vx - 2.376).abs() < 1e-9,
        "the drag the 0.99 keeps"
    );
    assert!(
        (app.net_arrows[0].vy + 0.05).abs() < 1e-9,
        "the gravity the 0.05 rides"
    );
    assert!(
        (app.net_arrows[0].prev_x - 10.0).abs() < 1e-9,
        "the prev trio armed"
    );
    // THE STILL-NAMED ID: the refresh ships the id again (a NEW
    // pose the server's own word would carry) — the client's own
    // sim rides untouched (the projectile the syncs never re-pose)
    app.live_net_arrows_updated(vec![crate::world_view::WireArrow {
        id: 300,
        x: 99.0,
        y: 99.0,
        z: 99.0,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        yaw: 0.0,
        pitch: 0.0,
        crit: false,
    }]);
    assert_eq!(app.net_arrows.len(), 1, "no duplicate spawn");
    assert!(
        (app.net_arrows[0].x - 12.4).abs() < 1e-9,
        "the sim's own truth stands"
    );
    // THE WIRE'S SILENCE: the Remove Entities truth drops the shot
    app.live_net_arrows_updated(vec![]);
    assert!(app.net_arrows.is_empty(), "the server's truth, no ghosts");
    // THE OFFLINE GATE: the lane the network arm never opened
    // stays the offline lane's own (the word lands nothing)
    let mut offline = App::new(3);
    offline.open(ScreenKind::World);
    offline.live_net_arrows_updated(vec![crate::world_view::WireArrow {
        id: 400,
        x: 1.0,
        y: 1.0,
        z: 1.0,
        vx: 1.0,
        vy: 0.0,
        vz: 0.0,
        yaw: 0.0,
        pitch: 0.0,
        crit: false,
    }]);
    assert!(
        offline.net_arrows.is_empty(),
        "the offline lane never mixes"
    );
}

/// THE WIRE THROWN SHOTS' FLIGHT LANE (javap `Projectile.
/// recreateFromPacket` the 26.2 walk the net-arrow lane's shape
/// wears): the Add Entity word becomes the CLIENT-side throwable
/// (the pose AND the motion the packet's own fields keep, the
/// kind the registry's own type name speaks), the arc the
/// client's own tick simulates (the move the velocity speaks, the
/// drag the 0.99 keeps, the gravity the 0.03 rides), the
/// still-named id rides its OWN sim untouched (the between-refresh
/// truth the client's physics owns), the wire's silence drops the
/// shot (no ghosts), the offline lane never enters the lane.
#[test]
fn the_wire_thrown_shots_fly_the_client_sim() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the spawn lands at the word (the kind the type name speaks,
    // the pose and the motion the packet's own fields carry)
    app.live_net_thrown_updated(vec![
        crate::world_view::WireThrown {
            id: 400,
            snowball: true,
            x: 10.0,
            y: 66.0,
            z: 10.0,
            vx: 2.4,
            vy: 0.0,
            vz: 0.0,
        },
        crate::world_view::WireThrown {
            id: 401,
            snowball: false,
            x: 30.0,
            y: 66.0,
            z: 30.0,
            vx: 1.0,
            vy: 0.0,
            vz: 0.0,
        },
    ]);
    assert_eq!(app.net_thrown.len(), 2, "the shots the wire spawned");
    assert_eq!(app.net_thrown[0].id, 400);
    assert_eq!(
        app.net_thrown[0].kind,
        crate::mob::ThrownProjectileKind::Snowball,
        "the kind the type name spoke"
    );
    assert_eq!(
        app.net_thrown[1].kind,
        crate::mob::ThrownProjectileKind::Egg,
        "the egg the type name spoke"
    );
    assert_eq!(
        (
            app.net_thrown[0].x,
            app.net_thrown[0].y,
            app.net_thrown[0].z
        ),
        (10.0, 66.0, 10.0),
        "the spawn IS the packet's word"
    );
    assert_eq!(app.net_thrown[0].yaw, -90.0, "the yaw the velocity speaks");
    // THE ARC (the client's own ThrowableProjectile.tick): the
    // move the velocity speaks (x += vx), the drag the 0.99 keeps,
    // the gravity the 0.03 rides
    app.tick_net_thrown();
    assert!(
        (app.net_thrown[0].x - 12.376).abs() < 1e-9,
        "the move the velocity speaks"
    );
    assert!(
        (app.net_thrown[0].vx - 2.376).abs() < 1e-9,
        "the drag the 0.99 keeps"
    );
    assert!(
        (app.net_thrown[0].vy + 0.03).abs() < 1e-9,
        "the gravity the 0.03 rides"
    );
    // THE STILL-NAMED IDS: the refresh ships the ids again — the
    // client's own sim rides untouched (the projectile the spawn
    // word never re-poses)
    app.live_net_thrown_updated(vec![
        crate::world_view::WireThrown {
            id: 400,
            snowball: true,
            x: 99.0,
            y: 99.0,
            z: 99.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
        },
        crate::world_view::WireThrown {
            id: 401,
            snowball: false,
            x: 99.0,
            y: 99.0,
            z: 99.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
        },
    ]);
    assert_eq!(
        app.net_thrown.len(),
        2,
        "the still-named shots the sim keeps"
    );
    assert!(
        (app.net_thrown[0].x - 12.376).abs() < 1e-9,
        "the sim's own flight rode"
    );
    assert!(
        (app.net_thrown[0].y - 65.97).abs() < 1e-9,
        "the arc's own fall rode"
    );
    // THE WIRE'S SILENCE: the id 401 the wire no longer names —
    // the body never re-enters (the despawn the Remove Entities
    // spoke, no ghosts)
    app.live_net_thrown_updated(vec![crate::world_view::WireThrown {
        id: 400,
        snowball: true,
        x: 99.0,
        y: 99.0,
        z: 99.0,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
    }]);
    assert_eq!(app.net_thrown.len(), 1, "the gone id the reconcile drops");
    assert_eq!(app.net_thrown[0].id, 400, "the still-named shot re-arms");
    // THE OFFLINE GATE: the local world's own eggs the wire lane
    // never touches (the world_is_network law the arrows wear)
    let mut offline = App::new(3);
    offline.enter_world();
    offline.live_net_thrown_updated(vec![crate::world_view::WireThrown {
        id: 7,
        snowball: true,
        x: 1.0,
        y: 1.0,
        z: 1.0,
        vx: 1.0,
        vy: 0.0,
        vz: 0.0,
    }]);
    assert!(
        offline.net_thrown.is_empty(),
        "the offline lane never mixes"
    );
}

/// THE WIRE THROWN SHOT'S OWN SHATTER LANE (javap `Snowball`/
/// `ThrownEgg.handleEntityEvent` case 3 the 26.2 walk): the
/// server's byte-3 word the sim'd shot DISCARDS at (the dead flag
/// the retain drops, no ghost flight the server's hit never rode),
/// the SIM pose the anchor lookup prefers (the store's stale word
/// the anchor never wears — the client copy's own position the
/// vanilla walk reads), the unknown id the store's pose the
/// fallback keeps (the model's gate the vanilla `level.getEntity`
/// answers), the kind the shard walk picks (the snowball's 8 plain
/// shards, the egg's 10-jitter loop), the ALREADY-BROKEN shot the
/// silence keeps (the sim's own ray the shards already spoke).
#[test]
fn the_wire_thrown_shatters_the_server_word_discards() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the snowball the spawn word minted: the sim the flight owns
    // (the store pose the server's spawn word keeps — the sim's
    // own tick the drift the store never re-poses)
    app.live_net_thrown_updated(vec![crate::world_view::WireThrown {
        id: 500,
        snowball: true,
        x: 10.0,
        y: 66.0,
        z: 10.0,
        vx: 2.4,
        vy: 0.0,
        vz: 0.0,
    }]);
    app.tick_net_thrown();
    assert!(
        (app.net_thrown[0].x - 12.376).abs() < 1e-9,
        "the sim's own flight rode"
    );
    // THE SERVER'S OWN DISCARD: the byte-3 arm the sim lookup
    // names — the shards anchor at the SIM pose (the store's stale
    // 99-trio the anchor never wears), the shot the lane drops
    app.live_thrown_shatter_events_updated(vec![crate::world_view::WireThrownShatterEvent {
        entity_id: 500,
        x: 99.0,
        y: 99.0,
        z: 99.0,
        snowball: true,
    }]);
    assert!(
        app.net_thrown.is_empty(),
        "the server's word the discard rides"
    );
    assert_eq!(app.particles.len(), 8, "the snowball's 8 plain shards");
    for p in &app.particles {
        assert_eq!(p.kind, crate::particles::ParticleKind::SnowballItem);
        assert!(
            (p.x - 12.376).abs() < 1e-9 && (p.y - 65.97).abs() < 1e-9,
            "the SIM pose the shards anchor at"
        );
    }
    // THE EGG'S OWN ARM: the kind the false keeps — the 10-jitter
    // loop the egg's own walk speaks (the store pose the no-sim
    // fallback keeps — the id the lane never minted)
    app.live_thrown_shatter_events_updated(vec![crate::world_view::WireThrownShatterEvent {
        entity_id: 501,
        x: 30.0,
        y: 66.0,
        z: 30.0,
        snowball: false,
    }]);
    assert_eq!(app.particles.len(), 18, "the egg's 10-jitter loop joins");
    for p in &app.particles[8..] {
        assert_eq!(p.kind, crate::particles::ParticleKind::EggItem);
        assert!(
            (p.x - 30.0).abs() <= 0.25 && (p.y - 66.0).abs() <= 0.25,
            "the jitter the 0.25 band keeps at the store pose"
        );
    }
    // THE ALREADY-BROKEN SHOT: the dead flag the double-burst
    // never arms — the second word the SAME batch carries finds
    // the dead flag (the sim's own ray the shards already spoke —
    // the silence the double burst never wears)
    app.live_net_thrown_updated(vec![crate::world_view::WireThrown {
        id: 502,
        snowball: true,
        x: 5.0,
        y: 66.0,
        z: 5.0,
        vx: 1.0,
        vy: 0.0,
        vz: 0.0,
    }]);
    let before = app.particles.len();
    app.live_thrown_shatter_events_updated(vec![
        crate::world_view::WireThrownShatterEvent {
            entity_id: 502,
            x: 99.0,
            y: 99.0,
            z: 99.0,
            snowball: true,
        },
        crate::world_view::WireThrownShatterEvent {
            entity_id: 502,
            x: 99.0,
            y: 99.0,
            z: 99.0,
            snowball: true,
        },
    ]);
    assert_eq!(
        app.particles.len(),
        before + 8,
        "the second word the silence keeps"
    );
    assert!(app.net_thrown.is_empty(), "the discard the retain swept");
}

/// THE WIRE THROWN SHOTS' OWN BREAK (the client-side
/// Projectile.tick's own onHit answer the wall speaks): the block
/// the clip's own ray arms — the shot dies AT the hit (the pose
/// the hit's own spot keeps), the SHATTER BURST the same case-3
/// walk rides (the snowball the 8 plain shards the entity's own
/// pos wears), the body the retain drops (no ghost flight
/// through the wall), the server's own hit words the mob lane
/// never rides (no chicks the wire egg's break never rolls).
#[test]
fn the_wire_thrown_shots_break_the_wall_the_burst_wears() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // THE WALL (the stone slab the shot's own flight line wears —
    // the vertical face the +x walk crosses; the y band the
    // world's own CHUNK_Y=48 ceiling the scene keeps — the
    // out-of-bounds cells the set never banks)
    {
        let w = app.net_world.as_mut().unwrap();
        for dy in 20..=30 {
            for dz in 8..=12 {
                w.set(20, dy, dz, crate::world::STONE);
            }
        }
    }
    // THE SHOT (the dispenser's own word — the flat arc the wall's
    // face crosses in two ticks)
    app.live_net_thrown_updated(vec![
        crate::world_view::WireThrown {
            id: 500,
            snowball: true,
            x: 10.5,
            y: 26.0,
            z: 10.5,
            vx: 5.0,
            vy: 0.0,
            vz: 0.0,
        },
        crate::world_view::WireThrown {
            id: 501,
            snowball: false,
            x: 10.5,
            y: 26.0,
            z: 11.5,
            vx: 5.0,
            vy: 0.0,
            vz: 0.0,
        },
    ]);
    assert_eq!(app.particles.len(), 0, "the fresh lane no burst carries");
    // TWO FLIGHT TICKS (the 5-cell stride the wall's face at x=20
    // the second tick's own segment crosses — the first the air
    // walk keeps)
    app.tick_net_thrown();
    app.tick_net_thrown();
    // THE BREAK (the wall face at x=20 the 5-cell stride crosses —
    // the clip the segment walks, the pose the hit's own spot
    // keeps, the bodies the retain drops)
    assert!(
        app.net_thrown.is_empty(),
        "the broken shots the retain drops"
    );
    // THE BURSTS (the snowball the 8 plain shards, the egg the
    // 10-jitter loop — the same ctors the offline lane rides)
    assert_eq!(
        app.particles.len(),
        18,
        "the two bursts the case-3 walk speaks"
    );
}

/// THE WIRE ARROWS' SYNC LANE (the handleEntityPositionSync walk
/// the raw disasm speaks): the near correction re-poses the sim at
/// the word with the prev trio keeping the OLD pose (the render's
/// own transit), the far correction snaps (the prev trio AND the
/// rotation ease-from arm AT the word — the teleport never
/// streaks), the velocity keeps its physics (the delta word the
/// handler never read), the unknown id drops (no phantom), and the
/// offline gate holds.
#[test]
fn the_arrow_sync_reposes_the_client_shot() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the spawn lands at the word, one tick flies the sim's own law
    app.live_net_arrows_updated(vec![crate::world_view::WireArrow {
        id: 300,
        x: 10.0,
        y: 66.0,
        z: 10.0,
        vx: 2.4,
        vy: 0.0,
        vz: 0.0,
        yaw: -90.0,
        pitch: 0.0,
        crit: false,
    }]);
    app.tick_net_arrows();
    assert!(
        (app.net_arrows[0].x - 12.4).abs() < 1e-9,
        "the flight's own spot"
    );
    // THE NEAR SYNC (0.5 out — inside the 64-cell band): the pose
    // and the exact angles land, the prev trio AND the rotation
    // ease-from keep the OLD pose (the render walks the transit)
    app.live_arrow_syncs_updated(vec![crate::world_view::WireArrowSync {
        id: 300,
        x: 12.9,
        y: 66.0,
        z: 10.0,
        yaw: -85.0,
        pitch: 0.0,
    }]);
    assert!((app.net_arrows[0].x - 12.9).abs() < 1e-9, "the word lands");
    assert!(
        (app.net_arrows[0].prev_x - 10.0).abs() < 1e-9,
        "the transit keeps"
    );
    assert!(
        (app.net_arrows[0].y_rot_o + 90.0).abs() < 1e-4,
        "the ease-from keeps"
    );
    assert!(
        (app.net_arrows[0].y_rot + 85.0).abs() < 1e-4,
        "the angles land"
    );
    // THE VELOCITY'S OWN SILENCE: the drag's word stands (the
    // delta the handler never read)
    assert!(
        (app.net_arrows[0].vx - 2.376).abs() < 1e-9,
        "the physics keeps"
    );
    // THE NEXT TICK re-arms the prev trio at the synced pose (the
    // tick's own head) and the flight resumes from the word
    app.tick_net_arrows();
    assert!(
        (app.net_arrows[0].prev_x - 12.9).abs() < 1e-9,
        "the tick's own head"
    );
    // THE FAR SYNC (100 cells out — beyond the 64 band): the snap
    // arms the prev trio AND the ease-from AT the word
    app.live_arrow_syncs_updated(vec![crate::world_view::WireArrowSync {
        id: 300,
        x: 112.9,
        y: 66.0,
        z: 10.0,
        yaw: -80.0,
        pitch: 0.0,
    }]);
    assert!(
        (app.net_arrows[0].x - 112.9).abs() < 1e-9,
        "the teleport lands"
    );
    assert!(
        (app.net_arrows[0].prev_x - 112.9).abs() < 1e-9,
        "the snap's own arm — no streak"
    );
    assert!(
        (app.net_arrows[0].y_rot_o + 80.0).abs() < 1e-4,
        "the ease-from armed at the word"
    );
    // THE UNKNOWN ID: the lookup's own gate — no phantom shot
    app.live_arrow_syncs_updated(vec![crate::world_view::WireArrowSync {
        id: 999,
        x: 1.0,
        y: 1.0,
        z: 1.0,
        yaw: 0.0,
        pitch: 0.0,
    }]);
    assert_eq!(app.net_arrows.len(), 1, "no phantom");
    assert!(
        (app.net_arrows[0].x - 112.9).abs() < 1e-9,
        "the neighbor untouched"
    );
    // THE OFFLINE GATE: the lane the network arm never opened
    // lands nothing
    let mut offline = App::new(3);
    offline.open(ScreenKind::World);
    offline.live_arrow_syncs_updated(vec![crate::world_view::WireArrowSync {
        id: 400,
        x: 1.0,
        y: 1.0,
        z: 1.0,
        yaw: 0.0,
        pitch: 0.0,
    }]);
    assert!(
        offline.net_arrows.is_empty(),
        "the offline lane never mixes"
    );
}

/// THE WIRE ROCKETS' FLIGHT LANE (the vanilla ClientLevel's own
/// recreateFromPacket law the net-arrow lane's shape wears): the
/// Add Entity word becomes the CLIENT-SIDE rocket (the pose, the
/// motion, the SHOT_AT_ANGLE word), the flight the client's own
/// tick simulates (the straight motion the shotAtAngle lane
/// keeps, the trail spark the every-tick walk spawns — NO launch
/// voice, NO self-explode the client walk never crosses), the
/// still-named id rides its OWN sim untouched, the wire's silence
/// drops the rocket (no ghosts), the offline lane never enters.
#[test]
fn the_wire_rockets_fly_the_client_sim() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the spawn lands at the word (the prev trio arms at it — the
    // render's first lerp never moves)
    app.live_net_fireworks_updated(vec![crate::world_view::WireFirework {
        id: 500,
        x: 10.0,
        y: 66.0,
        z: 10.0,
        vx: 0.0,
        vy: 1.2,
        vz: 0.0,
        shot_at_angle: true,
    }]);
    assert_eq!(app.net_fireworks.len(), 1, "the rocket rendered v1");
    assert_eq!(app.net_fireworks[0].id, 500);
    assert!(app.net_fireworks[0].shot_at_angle, "the axis word banks");
    assert_eq!(
        (
            app.net_fireworks[0].x,
            app.net_fireworks[0].y,
            app.net_fireworks[0].z
        ),
        (10.0, 66.0, 10.0),
        "the spawn IS the packet's word"
    );
    // THE FLIGHT (the client's own walk): the straight motion
    // adds (y += vy), the trail spark the every-tick walk spawns
    // (one particle the tick speaks), NO launch the client walk
    // never re-voices
    let before = app.particles.len();
    app.tick_net_fireworks();
    assert!(
        (app.net_fireworks[0].y - 67.2).abs() < 1e-9,
        "the move the velocity speaks"
    );
    assert_eq!(app.particles.len(), before + 1, "the trail the tick banks");
    assert!(
        matches!(
            app.particles[before].kind,
            crate::particles::ParticleKind::Spark { .. }
        ),
        "the trail the firework spark wears"
    );
    // THE NEVER-CROSSING WALK: many ticks the wire rocket flies
    // (the server's own ceiling the client never replays — the
    // byte-17 + the Remove the authority keeps)
    for _ in 0..200 {
        app.tick_net_fireworks();
        assert!(!app.net_fireworks[0].dead, "the sim never self-discards");
    }
    // THE STILL-NAMED ID: the refresh ships the id again — the
    // client's own sim rides untouched
    app.live_net_fireworks_updated(vec![crate::world_view::WireFirework {
        id: 500,
        x: 99.0,
        y: 99.0,
        z: 99.0,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        shot_at_angle: false,
    }]);
    assert_eq!(app.net_fireworks.len(), 1, "no duplicate spawn");
    assert!(
        app.net_fireworks[0].shot_at_angle,
        "the sim's own word stands"
    );
    assert!(
        (app.net_fireworks[0].y - 67.2 - 1.2 * 200.0).abs() < 1e-6,
        "the sim's own truth stands ({})",
        app.net_fireworks[0].y
    );
    // THE WIRE'S SILENCE: the Remove Entities truth drops the
    // rocket
    app.live_net_fireworks_updated(vec![]);
    assert!(
        app.net_fireworks.is_empty(),
        "the server's truth, no ghosts"
    );
    // THE OFFLINE GATE: the lane the network arm never opened
    // stays the offline lane's own (the word lands nothing)
    let mut offline = App::new(3);
    offline.open(ScreenKind::World);
    offline.live_net_fireworks_updated(vec![crate::world_view::WireFirework {
        id: 600,
        x: 1.0,
        y: 1.0,
        z: 1.0,
        vx: 1.0,
        vy: 0.0,
        vz: 0.0,
        shot_at_angle: false,
    }]);
    assert!(
        offline.net_fireworks.is_empty(),
        "the offline lane never mixes"
    );
}

/// THE WIRE ROCKETS' SYNC LANE (the handleEntityPositionSync walk
/// the arrow lane's shape wears) + THE SIM ANCHOR'S OWN BYTE-17:
/// the near correction re-poses the sim at the word with the prev
/// trio keeping the OLD pose, the far correction snaps, the
/// velocity keeps its physics, the unknown id drops — and the
/// explosion event the sim's CURRENT pose anchors (the
/// createFireworks anchor the client entity's own getX/getY/getZ
/// answers), the store pose the no-sim fallback keeps.
#[test]
fn the_firework_sync_reposes_the_rocket_and_the_burst_anchors_the_sim() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the spawn lands at the word, one tick flies the sim's own law
    app.live_net_fireworks_updated(vec![crate::world_view::WireFirework {
        id: 500,
        x: 10.0,
        y: 66.0,
        z: 10.0,
        vx: 0.0,
        vy: 1.2,
        vz: 0.0,
        shot_at_angle: true,
    }]);
    app.tick_net_fireworks();
    assert!(
        (app.net_fireworks[0].y - 67.2).abs() < 1e-9,
        "the flight's own spot"
    );
    // THE NEAR SYNC (0.5 out — inside the 64-cell band): the pose
    // lands, the prev trio keeps the OLD pose (the render walks
    // the transit)
    app.live_firework_syncs_updated(vec![crate::world_view::WireFireworkSync {
        id: 500,
        x: 10.0,
        y: 67.7,
        z: 10.0,
        yaw: 0.0,
        pitch: 90.0,
    }]);
    assert!(
        (app.net_fireworks[0].y - 67.7).abs() < 1e-9,
        "the word lands"
    );
    assert!(
        (app.net_fireworks[0].prev_y - 66.0).abs() < 1e-9,
        "the transit keeps"
    );
    assert!(
        (app.net_fireworks[0].x_rot - 90.0).abs() < 1e-4,
        "the angles land"
    );
    // THE VELOCITY'S OWN SILENCE: the straight flight's word
    // stands (the delta the handler never read)
    assert!(
        (app.net_fireworks[0].vy - 1.2).abs() < 1e-9,
        "the physics keeps"
    );
    // THE FAR SYNC (100 cells out): the snap arms the prev trio
    // AT the word — no streak
    app.live_firework_syncs_updated(vec![crate::world_view::WireFireworkSync {
        id: 500,
        x: 10.0,
        y: 167.7,
        z: 10.0,
        yaw: 0.0,
        pitch: 90.0,
    }]);
    assert!(
        (app.net_fireworks[0].y - 167.7).abs() < 1e-9,
        "the teleport lands"
    );
    assert!(
        (app.net_fireworks[0].prev_y - 167.7).abs() < 1e-9,
        "the snap's own arm — no streak"
    );
    // THE UNKNOWN ID: the lookup's own gate — no phantom rocket
    app.live_firework_syncs_updated(vec![crate::world_view::WireFireworkSync {
        id: 999,
        x: 1.0,
        y: 1.0,
        z: 1.0,
        yaw: 0.0,
        pitch: 0.0,
    }]);
    assert_eq!(app.net_fireworks.len(), 1, "no phantom");
    // THE BYTE-17'S OWN ANCHOR: the event the SIM pose answers —
    // the burst the poof branch speaks at the rocket's CURRENT
    // spot (167.7), NOT the arm's store word (3.5)
    app.net_fireworks[0].x = 10.0;
    app.net_fireworks[0].y = 167.7;
    app.net_fireworks[0].z = 10.0;
    let before = app.particles.len();
    app.live_firework_explode_events_updated(vec![crate::world_view::WireFireworkExplodeEvent {
        entity_id: 500,
        x: 3.5,
        y: 3.5,
        z: 3.5,
        mx: 0.0,
        my: 0.0,
        mz: 0.0,
        explosions: Vec::new(),
    }]);
    let burst: Vec<&crate::particles::Particle> = app.particles.iter().skip(before).collect();
    assert!(
        (2..=4).contains(&burst.len()),
        "the poof branch the burst banks ({})",
        burst.len()
    );
    for p in &burst {
        assert_eq!(
            (p.x, p.y, p.z),
            (10.0, 167.7, 10.0),
            "the sim's own pose the anchor answers"
        );
    }
    // THE NO-SIM FALLBACK: the id the refresh no longer names the
    // arm's own word anchors (the Remove Entities raced the event
    // — the store's last sync the crossing pose banks)
    app.live_net_fireworks_updated(vec![]);
    let before = app.particles.len();
    app.live_firework_explode_events_updated(vec![crate::world_view::WireFireworkExplodeEvent {
        entity_id: 500,
        x: 3.5,
        y: 3.5,
        z: 3.5,
        mx: 0.0,
        my: 0.0,
        mz: 0.0,
        explosions: Vec::new(),
    }]);
    let burst: Vec<&crate::particles::Particle> = app.particles.iter().skip(before).collect();
    for p in &burst {
        assert_eq!(
            (p.x, p.y, p.z),
            (3.5, 3.5, 3.5),
            "the store pose the fallback keeps"
        );
    }
    // THE OFFLINE GATE: the sync lane the network arm never
    // opened lands nothing
    let mut offline = App::new(3);
    offline.open(ScreenKind::World);
    offline.live_firework_syncs_updated(vec![crate::world_view::WireFireworkSync {
        id: 500,
        x: 1.0,
        y: 1.0,
        z: 1.0,
        yaw: 0.0,
        pitch: 0.0,
    }]);
    assert!(
        offline.net_fireworks.is_empty(),
        "the offline lane never mixes"
    );
}

/// THE WIRE ARROW'S CRIT TRAIL (the flags byte the projection
/// rides → the from_wire word the shot arms → the flight the
/// 4-star-per-tick the particle engine spawns): the crit star the
/// tick's span positions answer, the plain shot the trail never
/// arms, the stars the CritParticle law ticks.
#[test]
fn the_wire_arrow_the_crit_byte_the_trail_stars_speak() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // THE CRIT SPAWN: the metadata byte the projection read
    app.live_net_arrows_updated(vec![crate::world_view::WireArrow {
        id: 300,
        x: 10.0,
        y: 66.0,
        z: 10.0,
        vx: 2.4,
        vy: 0.0,
        vz: 0.0,
        yaw: -90.0,
        pitch: 0.0,
        crit: true,
    }]);
    assert!(app.net_arrows[0].crit, "the wire's byte the shot arms");
    // THE FLIGHT TICK: the 4 stars the trail words spawn (the
    // engine's own rng the life/size draws spend)
    app.tick_net_arrows();
    let crits: Vec<&crate::particles::Particle> = app
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Crit))
        .collect();
    assert_eq!(crits.len(), 4, "the 4-particle loop the tick speaks");
    for (j, p) in crits.iter().enumerate() {
        let jf = j as f64;
        assert!(
            (p.x - (10.0 + 2.4 * jf / 4.0)).abs() < 1e-9,
            "the span the j/4 keeps (x @ {j})"
        );
        assert!(
            (p.vx + 2.4).abs() < 1e-9 && (p.vy - 0.2).abs() < 1e-9,
            "the backward drift the star rides (@ {j})"
        );
    }
    // THE PLAIN SHOT: the silent byte the trail never arms
    let mut bare = App::new(3);
    bare.open(ScreenKind::Connect);
    bare.net_world = Some(air_net_world(2, 0));
    bare.enter_network_world(None, Skybox::Overworld, None, None);
    bare.live_net_arrows_updated(vec![crate::world_view::WireArrow {
        id: 301,
        x: 10.0,
        y: 66.0,
        z: 10.0,
        vx: 2.4,
        vy: 0.0,
        vz: 0.0,
        yaw: -90.0,
        pitch: 0.0,
        crit: false,
    }]);
    bare.tick_net_arrows();
    let bare_crits = bare
        .particles
        .iter()
        .filter(|p| matches!(p.kind, crate::particles::ParticleKind::Crit))
        .count();
    assert_eq!(bare_crits, 0, "the plain shot trails nothing");
}

/// THE WIRE MOBS' DEATH CLOCK (LivingEntity.tickDeath's own law
/// the AI lane speaks): the server's health word <= 0 — deathTime
/// walks (the flip the render's deathTime+partial read speaks);
/// the accessor the server never spoke is the honest no-opinion
/// the walk never mistakes for a kill.
#[test]
fn the_wire_mobs_walk_the_death_clock() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the kill's own word: the metadata's health 0.0 rides the
    // projection into the mob's own store
    let mut dying = wire_zombie(7, 10.0, 64.0, 10.0);
    dying.health = Some(0.0);
    app.live_mobs_updated(vec![dying]);
    assert_eq!(app.net_mobs[0].health, 0.0, "the server's word stands");
    // the flip walks: deathTime++ per tick (the 20-tick walk the
    // render's deathTime+partial read speaks)
    app.tick_net_mobs();
    assert_eq!(app.net_mobs[0].death_time, 1);
    app.tick_net_mobs();
    app.tick_net_mobs();
    assert_eq!(app.net_mobs[0].death_time, 3, "the flip walks");
    // the no-opinion arm: the accessor never spoken — the walk
    // never mistakes the silence for a kill (the refresh's own
    // despawn drops the dying id 7 — the server's truth)
    app.live_mobs_updated(vec![wire_zombie(9, 12.0, 64.0, 10.0)]);
    assert_eq!(app.net_mobs.len(), 1, "the dying id despawned");
    assert!(app.net_mobs[0].health > 0.0, "the ctor's full bar stands");
    app.tick_net_mobs();
    assert_eq!(app.net_mobs[0].death_time, 0, "the silence is not a kill");
}

/// THE WIRE MOBS' OWN VOICES (the offline server the wire lane
/// stands in for — the vanilla server's own playSound broadcasts
/// the client answers): the ambient the Mob.baseTick gate speaks
/// (the shared method, the id-seeded rng's own replay pins the
/// exact firing tick), the hurt the arm value's own moment (the
/// 10 the Hurt Animation packet ships — once per arm, the decayed
/// window never refires), the death the health word's own zero
/// crossing (once per die — the later death ticks silent, the
/// dying body's ambient clock frozen). The counts ride the ACTUAL
/// loaded table (the fixture answers the CI's cache-absent boot,
/// the real table the cache-present boot — both speak).
#[test]
fn the_wire_mobs_speak_the_vanilla_voices() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.live_mobs_updated(vec![wire_zombie(7, 10.0, 64.0, 10.0)]);
    let count = |app: &App, prefix: &str| {
        app.sounds
            .events
            .iter()
            .filter(|e| e.starts_with(prefix))
            .count()
    };
    // THE AMBIENT GATE (Mob.baseTick's own law through the shared
    // method): the replay walks the mob's own seed law — the clock
    // starts at -80, one draw per tick, the draw under the climbing
    // clock fires. The gate's first firing tick the replay names.
    let mut replay = crate::drops::Lcg::new(crate::mob::entity_voice_seed(7));
    let fire_tick = (1..=2000)
        .find(|t| {
            replay.next() % crate::mob::AMBIENT_DRAW_BOUND
                < (*t as i32 - crate::mob::AMBIENT_INTERVAL_TICKS).max(0) as u64
        })
        .expect("the gate fires within the walk");
    for t in 1..fire_tick {
        app.tick_net_mobs();
        assert_eq!(
            count(&app, "entity.zombie.ambient "),
            0,
            "tick {t} stays silent"
        );
    }
    app.tick_net_mobs();
    assert_eq!(
        count(&app, "entity.zombie.ambient "),
        1,
        "the groan at the law's own tick {fire_tick}"
    );
    // THE HURT ARM (the 10 the Hurt Animation packet ships): the
    // voice at the arm's own tick, the decayed window silent, the
    // fresh arm speaks again
    app.net_mobs[0].hurt_time = crate::mob::HURT_DURATION_TICKS;
    app.tick_net_mobs();
    assert_eq!(count(&app, "entity.zombie.hurt "), 1, "the arm spoke");
    for _ in 0..(crate::mob::HURT_DURATION_TICKS - 1) {
        app.tick_net_mobs();
    }
    assert_eq!(
        count(&app, "entity.zombie.hurt "),
        1,
        "the decay never refires"
    );
    app.net_mobs[0].hurt_time = crate::mob::HURT_DURATION_TICKS;
    app.tick_net_mobs();
    assert_eq!(
        count(&app, "entity.zombie.hurt "),
        2,
        "the fresh arm speaks"
    );
    // THE DEATH CROSSING (the health word's own zero): one voice at
    // the crossing, the later death ticks silent — and the dying
    // body's ambient clock frozen (the gate's alive arm)
    let groans_before = count(&app, "entity.zombie.ambient ");
    app.net_mobs[0].health = 0.0;
    app.tick_net_mobs();
    assert_eq!(count(&app, "entity.zombie.death "), 1, "the crossing spoke");
    for _ in 0..19 {
        app.tick_net_mobs();
    }
    assert_eq!(
        count(&app, "entity.zombie.death "),
        1,
        "the death spoke once"
    );
    assert_eq!(
        count(&app, "entity.zombie.ambient "),
        groans_before,
        "the dying body never groans"
    );
}

/// THE WIRE KINDS' OWN VOICES (the voice table through the wire's
/// own walk — the registry name the Add Entity spoke -> the kind ->
/// the class's own rows): the sheep's baa, the cow's moo, the
/// pig's oink answer the same arm/crossing moments the zombie's
/// groan does, each kind its own event, no cross-talk.
#[test]
fn the_wire_kinds_speak_their_own_voices() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.live_mobs_updated(vec![
        wire_mob_named(7, 10.0, 64.0, 10.0, "minecraft:sheep"),
        wire_mob_named(8, 12.0, 64.0, 10.0, "minecraft:cow"),
        wire_mob_named(9, 14.0, 64.0, 10.0, "minecraft:pig"),
    ]);
    let count = |app: &App, prefix: &str| {
        app.sounds
            .events
            .iter()
            .filter(|e| e.starts_with(prefix))
            .count()
    };
    // the arm rides all three (the staging's own law — the 10 the
    // Hurt Animation packet ships), each kind its own voice
    for m in app.net_mobs.iter_mut() {
        m.hurt_time = crate::mob::HURT_DURATION_TICKS;
    }
    app.tick_net_mobs();
    assert_eq!(count(&app, "entity.sheep.hurt "), 1, "the baa");
    assert_eq!(count(&app, "entity.cow.hurt "), 1, "the moo");
    assert_eq!(count(&app, "entity.pig.hurt "), 1, "the oink");
    assert_eq!(count(&app, "entity.zombie.hurt "), 0, "no cross-talk");
    // the sheep's death crossing — its own voice, once
    app.net_mobs[0].health = 0.0;
    app.tick_net_mobs();
    assert_eq!(count(&app, "entity.sheep.death "), 1, "the baa's last word");
    assert_eq!(count(&app, "entity.cow.death "), 0, "the cow stands");
    assert_eq!(count(&app, "entity.pig.death "), 0, "the pig stands");
}

/// THE WIRE VOICES' OWN DISTANCE FOLD (the SoundEngine's linear
/// channel the positioned voices ride — javap SoundEngine.play +
/// Channel.linearAttenuation): the ear the body's own eye speaks —
/// the at-ear voice full, the 4-out voice three quarters, the
/// 20-out voice the silence the zero crossing keeps — the log line
/// the folded gain records.
#[test]
fn the_wire_voices_fall_with_the_distance() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // THE LISTENER (the body's standing eye the camera speaks): the
    // ear at (0.0, 65.62, 0.0) — the mobs' own y the same eye line
    // keeps (the distance the z axis alone speaks)
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    app.live_mobs_updated(vec![
        wire_zombie(7, 0.0, 65.62, 0.0),
        wire_zombie(8, 0.0, 65.62, 4.0),
        wire_zombie(9, 0.0, 65.62, 20.0),
    ]);
    for m in app.net_mobs.iter_mut() {
        m.hurt_time = crate::mob::HURT_DURATION_TICKS;
    }
    app.tick_net_mobs();
    if !sounds_loaded() {
        assert!(
            app.sounds.events.is_empty(),
            "the empty table the silent voice lane keeps"
        );
        return;
    }
    let hurt_lines: Vec<&String> = app
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("entity.zombie.hurt "))
        .collect();
    assert_eq!(hurt_lines.len(), 3, "the three arms spoke");
    // the at-ear arm — the fold's 1.0
    assert!(
        hurt_lines.iter().any(|e| e.contains(" v1.000000 p")),
        "the at-ear voice full: {:?}",
        hurt_lines
    );
    // the 4-out arm — 1 - 4/16 = 0.75
    assert!(
        hurt_lines.iter().any(|e| e.contains(" v0.750000 p")),
        "the near voice folded: {:?}",
        hurt_lines
    );
    // the 20-out arm — past the 16 the zero crossing keeps
    assert!(
        hurt_lines.iter().any(|e| e.contains(" v0.000000 p")),
        "the far voice silent: {:?}",
        hurt_lines
    );
}

/// THE WIRE MAILBOX'S OWN FOLD (the handleEntitySound walk the
/// positioned voices speak): the packet's pitch verbatim, the
/// packet's volume the distance law's gain — the named entity's
/// own feet the ear measures, the loud packet's armed max the
/// stretch speaks, the cap the AL_MAX_GAIN keeps.
#[test]
fn the_wire_mailbox_speaks_the_distance_law() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let voice = |entity_id: i32, name: &str, volume: f32| crate::world_view::WireVoice {
        entity_id,
        name: name.to_string(),
        volume,
        pitch: 1.0,
    };
    // the quiet packet at 8 out: 1.0 x (1 - 8/16) = 0.5
    app.live_mobs_updated(vec![wire_zombie(7, 0.0, 65.62, 8.0)]);
    app.live_voices_updated(vec![voice(7, "minecraft:entity.zombie.ambient", 1.0)]);
    if !sounds_loaded() {
        assert!(
            app.sounds.events.is_empty(),
            "the empty table the silent voice lane keeps"
        );
        return;
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.zombie.ambient ") && e.contains(" v0.500000 p1.000000")),
        "the half-gain groan: {:?}",
        app.sounds.events
    );
    // the loud packet's stretch: the 4.0 arms the 64 — at 56 out
    // the fold lands 0.5 (the un-stretched 16 would be silent)
    app.live_mobs_updated(vec![wire_zombie(8, 0.0, 65.62, 56.0)]);
    app.live_voices_updated(vec![voice(8, "minecraft:entity.zombie.hurt", 4.0)]);
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.zombie.hurt ") && e.contains(" v0.500000 p1.000000")),
        "the stretched half: {:?}",
        app.sounds.events
    );
    // the cap: the 4.0 at the ear — the AL_MAX_GAIN 1.0 keeps
    app.live_mobs_updated(vec![wire_zombie(9, 0.0, 65.62, 0.0)]);
    app.live_voices_updated(vec![voice(9, "minecraft:entity.zombie.death", 4.0)]);
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.zombie.death ") && e.contains(" v1.000000 p1.000000")),
        "the capped ear shot: {:?}",
        app.sounds.events
    );
}

/// THE KINETIC HIT'S OWN VOICE (javap `LivingEntity.onKineticHit`
/// over the 26.2 jar — the Entity Event byte-2 word the server's
/// stab contact broadcasts): the 20-tick throttle the echo never
/// outruns, the bank the gate's own pass performs BEFORE the weapon
/// gate (the weaponless event the clock still spends — the spear's
/// later window the wasted bank eats), the seven spears' component
/// the KINETIC_WEAPON gate keeps, the local hit voice the
/// material's row names — the wooden tier's `item.spear_wood.hit`,
/// every other tier `item.spear.hit` — full volume, full pitch, the
/// attacker's own ear the playLocalSound channel carries.
#[test]
fn the_wire_kinetic_event_the_local_hit_voice_wears() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let spear = mc_registry::item_id_by_name("iron_spear").unwrap();
    let wooden = mc_registry::item_id_by_name("wooden_spear").unwrap();
    let arm = crate::world_view::WireKineticHitEvent { entity_id: 17 };
    // THE FIRST WORD (frame 30): the bank's 0 start the 30-gap
    // clears — the hit voice speaks
    app.live_held = Some(spear);
    app.frame = 30;
    app.clock_total = 30;
    app.live_kinetic_hit_events_updated(vec![arm.clone()]);
    if !sounds_loaded() {
        assert!(
            app.sounds.events.is_empty(),
            "the empty table the silent voice lane keeps"
        );
        return;
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear.hit ") && e.contains(" v1.000000 p1.000000")),
        "the iron spear's hit voice: {:?}",
        app.sounds.events
    );
    // THE ECHO (frame 40): the 10-tick gap under the 20 — silent
    app.frame = 40;
    app.clock_total = 40;
    let voiced = app.sounds.events.len();
    app.live_kinetic_hit_events_updated(vec![arm.clone()]);
    assert_eq!(app.sounds.events.len(), voiced, "the throttle's silence");
    // THE SECOND WINDOW (frame 51): the 21-tick gap clears — speaks
    app.frame = 51;
    app.clock_total = 51;
    app.live_kinetic_hit_events_updated(vec![arm.clone()]);
    assert!(app.sounds.events.len() > voiced, "the next window speaks");
    // THE WEAPONLESS BANK (frame 100): the fist the component never
    // carries — silent, AND the timestamp still banks (the vanilla
    // order the pcs 20-28 walk speaks: the bank rides BEFORE the
    // weapon gate)
    app.live_held = None;
    app.frame = 100;
    app.clock_total = 100;
    let voiced = app.sounds.events.len();
    app.live_kinetic_hit_events_updated(vec![arm.clone()]);
    assert_eq!(app.sounds.events.len(), voiced, "the fist's silence");
    // THE WASTED WINDOW (frame 110): the spear arrives — the fist's
    // own bank the 10-tick gap eats (the honest waste the vanilla
    // order keeps)
    app.live_held = Some(spear);
    app.frame = 110;
    app.clock_total = 110;
    app.live_kinetic_hit_events_updated(vec![arm.clone()]);
    assert_eq!(app.sounds.events.len(), voiced, "the eaten window");
    // THE WOOD TIER (frame 130): the factory's WOOD branch the row
    // names — `item.spear_wood.hit`
    app.live_held = Some(wooden);
    app.frame = 130;
    app.clock_total = 130;
    app.live_kinetic_hit_events_updated(vec![arm.clone()]);
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("item.spear_wood.hit ") && e.contains(" v1.000000 p1.000000")),
        "the wooden tier's own row: {:?}",
        app.sounds.events
    );
}

/// THE LOCAL ZOMBIE'S OWN FOLD (the distance law the wire lane
/// speaks, the offline lane the same answer): the fist's own hurt
/// voice folds by the mob's own feet-to-ear distance — the staged
/// geometry the exact numbers speak: the eye at the zombie's own
/// feet level (the dy 0 keeps), 2.5 east (inside the 3.0 arm the
/// crosshair keeps), the fold `1 - 2.5/16 = 0.84375` the log line
/// records.
#[test]
fn the_local_zombie_voice_folds_the_distance() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // the body ON the surface column (the standing eye 1.62 above)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        top as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    // the zombie's FEET on the eye's own level, 2.5 east: the dy
    // the zero keeps, the distance the 2.5 the fold reads exactly
    app.mobs.push(crate::mob::Zombie::new(
        7,
        c as f64 + 3.0,
        top as f64 + 1.62,
        c as f64 + 0.5,
        3,
    ));
    // the crosshair's own aim (the mid-height the ray walks — the
    // entry the box's interior speaks), then the fist's hit
    {
        let (tx, ty, tz) = {
            let m = &app.mobs[0];
            (m.x, m.y + crate::mob::ZOMBIE_HEIGHT * 0.5, m.z)
        };
        let b = app.body.as_mut().unwrap();
        let (dx, dy, dz) = (tx - b.x, ty - (b.y + b.eye_height()), tz - b.z);
        let horiz = (dx * dx + dz * dz).sqrt().max(1e-9);
        let len3 = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-9);
        b.yaw = ((dx / horiz).atan2(-dz / horiz) - std::f64::consts::PI).to_degrees() as f32;
        b.pitch = -(dy / len3).asin().to_degrees() as f32;
    }
    assert_eq!(
        app.pick_mob(&app.mobs),
        Some(0),
        "the 2.5 body inside the entity arm's reach"
    );
    app.attack_mob(0);
    if !sounds_loaded() {
        assert!(
            app.sounds.events.is_empty(),
            "the empty table the silent voice lane keeps"
        );
        return;
    }
    // the hurt voice — the fold's own 0.84375 (the un-folded 1.0
    // the pre-law lane spoke)
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.zombie.hurt ") && e.contains(" v0.843750 p")),
        "the fist's own fold: {:?}",
        app.sounds.events
    );
}

/// THE MOB FIZZ'S OWN DISTANCE LAW (the extinguish transition the
/// douse arm speaks — the playEntityOnFireExtinguishedSound the
/// entity's own spot carries): the log line's gain the LIVE
/// geometry answers — the ear-to-spot distance the caller's
/// post-tick read keeps, the fold `0.7 * (1 - d/16)` the
/// attenuate law speaks (the un-folded 0.7 the pre-law lane
/// spoke).
#[test]
fn the_mobs_fizz_folds_the_distance() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y;
    // the body ON the surface column (the standing eye 1.62 above)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        surface as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    // the water cell the douse arm's bbox scan reads + the burning
    // zombie 4.0 east, the FEET at the eye's own level — the dy
    // the zero keeps, the approach's own drift the live read folds
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c + 4, surface + 1, c, crate::world::WATER);
    app.mobs.push(crate::mob::Zombie::new(
        7,
        c as f64 + 4.5,
        surface as f64 + 1.62,
        c as f64 + 0.5,
        3,
    ));
    app.mobs[0].fire_ticks = 160;
    app.update();
    if !sounds_loaded() {
        assert!(
            app.sounds.events.is_empty(),
            "the empty table the silent voice lane keeps"
        );
        return;
    }
    // the fold's own law the live geometry answers: the distance
    // the caller's post-tick read keeps (the mob's own approach
    // walked the first tick's span), the gain the attenuate speaks
    let b = app.body.as_ref().unwrap();
    let ear_pos = (b.x, b.y + b.eye_height(), b.z);
    let m = &app.mobs[0];
    let d = ear_distance(Some(ear_pos), m.x, m.y, m.z);
    let expected = crate::sound::attenuate(
        crate::mob::EXTINGUISH_VOLUME,
        d,
        crate::sound::voice_max_distance(crate::mob::EXTINGUISH_VOLUME),
    );
    assert!(
        d > 3.0 && d < 4.0,
        "the staged approach the walk's own drift keeps ({d})"
    );
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.generic.extinguish_fire ")
                && e.contains(&format!(" v{expected:.6} p"))),
        "the douse's own fold (d = {d}, v = {expected:.6}): {:?}",
        app.sounds.events
    );
}

/// THE MOB VOICES' OWN STEREO (the lateral pan law the mob voices
/// now speak — the `ear_pan` walk the queues carry): the body
/// facing south (vanilla yaw 0 — the +Z forward the (-sin, cos)
/// fold wears), the burning zombie due EAST the left hand answers
/// (the right axis the (-cos, -sin) pair wears — facing south the
/// west the right), the hurt and the fizz lines the negative side
/// tail wear (the AI's own approach the pure -x line walks, the
/// SIGN the z's zero keeps); the FRONT zombie the center the
/// identity keeps (no ` s` field the no-op never needs — the same
/// 4-field shape the unmigrated lanes wear).
#[test]
fn the_mob_voices_split_the_right_axis() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y;
    // the body ON the surface column, facing SOUTH (vanilla yaw 0)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        surface as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    // the water cell the douse arm's bbox scan reads + the burning
    // zombie due EAST (the z the body's own line keeps — the
    // approach the AI walks is the pure -x line, the pan the pure
    // lateral ray speaks)
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c + 4, surface + 1, c, crate::world::WATER);
    app.mobs.push(crate::mob::Zombie::new(
        7,
        c as f64 + 4.5,
        surface as f64 + 1.62,
        c as f64 + 0.5,
        3,
    ));
    app.mobs[0].fire_ticks = 160;
    app.update();
    if !sounds_loaded() {
        assert!(
            app.sounds.events.is_empty(),
            "the empty table the silent voice lane keeps"
        );
        return;
    }
    assert!(
        app.sounds
            .events
            .iter()
            .any(|e| e.starts_with("entity.zombie.hurt ")),
        "the fire's own hurt spoke: {:?}",
        app.sounds.events
    );
    for e in app.sounds.events.iter() {
        if e.starts_with("entity.zombie.hurt ") || e.starts_with("entity.generic.extinguish_fire ")
        {
            assert!(
                e.contains(" s-"),
                "the east source the facing-south body's own left: {e}"
            );
            assert!(
                !e.contains(" s+"),
                "no right-ear voice the east source speaks: {e}"
            );
        }
    }
    // THE FRONT ZOMBIE'S OWN IDENTITY (the +z source the
    // facing-south body's own nose speaks — the right axis the
    // pure-front ray never answers, the pan the exact 0.0 the
    // identity lane keeps): no voice line the ` s` field wears
    let mut front = App::new(3);
    front.enter_world();
    let fsize = front.offline_world.as_ref().unwrap().size_blocks();
    let fc = fsize / 2;
    front.body = Some(crate::body::PlayerBody::new(
        fc as f64 + 0.5,
        surface as f64,
        fc as f64 + 0.5,
        0.0,
        0.0,
    ));
    front.mobs.push(crate::mob::Zombie::new(
        8,
        fc as f64 + 0.5,
        surface as f64 + 1.62,
        fc as f64 + 4.5,
        3,
    ));
    front.mobs[0].fire_ticks = 160;
    front.update();
    if sounds_loaded() {
        assert!(
            front
                .sounds
                .events
                .iter()
                .any(|e| e.starts_with("entity.zombie.hurt ")),
            "the fire's own hurt spoke: {:?}",
            front.sounds.events
        );
        assert!(
            front.sounds.events.iter().all(|e| !e.contains(" s")),
            "the front voice the identity keeps: {:?}",
            front.sounds.events
        );
    }
}

/// THE BLOCK LANES' OWN STEREO WALK (the pan law the positioned
/// block lanes' migration wears — the splash the free arm speaks,
/// the ear→source direction the listener's own facing answers):
/// the east source the facing-south body's own left ear answers
/// (the ` s-` tail the signed side wears), the west source the
/// right hand speaks (` s+`), the front source the identity the
/// plain 4-field shape keeps, the ear-less arm the center no-tail
/// shape keeps — the same arms the mob voices' walk banked.
#[test]
fn the_block_lanes_split_the_right_axis_too() {
    let splash = |ear: Option<(f64, f64, f64)>, yaw: f32, x: f64| {
        let mut sounds = crate::sound::BlockSounds::new(5);
        let mut entity_rng = crate::drops::Lcg::new(0x5EED_5A145);
        let mut level_rng = crate::drops::Lcg::new(0x5EED_1E7E1);
        let mut particles: Vec<crate::particles::Particle> = Vec::new();
        bubble_splash_arm(
            &mut entity_rng,
            &mut level_rng,
            &mut sounds,
            &mut particles,
            100,
            x,
            64.0,
            8.0,
            ear,
            yaw,
        );
        assert_eq!(particles.len(), 1, "the droplet the arm births");
        assert_eq!(sounds.events.len(), 1, "the one splash the arm speaks");
        sounds.events.pop().unwrap()
    };
    if !sounds_loaded() {
        // the empty table the silent voice lane keeps — the arm's
        // pick the None answer speaks
        let mut sounds = crate::sound::BlockSounds::new(5);
        let mut entity_rng = crate::drops::Lcg::new(0x5EED_5A145);
        let mut level_rng = crate::drops::Lcg::new(0x5EED_1E7E1);
        let mut particles: Vec<crate::particles::Particle> = Vec::new();
        bubble_splash_arm(
            &mut entity_rng,
            &mut level_rng,
            &mut sounds,
            &mut particles,
            100,
            12.0,
            64.0,
            8.0,
            Some((8.0, 65.0, 8.0)),
            0.0,
        );
        assert!(
            sounds.events.is_empty(),
            "the empty table the silent lane keeps: {:?}",
            sounds.events
        );
        return;
    }
    // the facing-south body's ear at (8, 65, 8): the east source the
    // LEFT ear answers (the right axis the (-cos, -sin) pair wears —
    // facing south the west the right)
    let east = splash(Some((8.0, 65.0, 8.0)), 0.0, 12.0);
    assert!(
        east.contains(" s-") && !east.contains(" s+"),
        "the east splash the facing-south body's own left: {east}"
    );
    // the west source the right hand speaks
    let west = splash(Some((8.0, 65.0, 8.0)), 0.0, 4.0);
    assert!(
        west.contains(" s+") && !west.contains(" s-"),
        "the west splash the facing-south body's own right: {west}"
    );
    // the front source (pure +z) the identity keeps — the right
    // axis the pure-front ray never answers
    let front = splash(Some((8.0, 65.0, 8.0)), 0.0, 8.0);
    assert!(
        !front.contains(" s"),
        "the front splash the identity keeps: {front}"
    );
    // the ear-less arm the center keeps — the same no-tail shape
    // the unmigrated lanes wear
    let deaf = splash(None, 0.0, 12.0);
    assert!(
        !deaf.contains(" s"),
        "the ear-less splash the center keeps: {deaf}"
    );
}

/// THE MOB STEP'S OWN DISTANCE LAW (the cadence's flat voice the
/// mob's own spot carries — the approach the fold speaks): every
/// step line's gain the base 0.15 never reaches (the d > 0 the
/// walking approach keeps — the fold's own ceiling), the walk the
/// zombie's own approach answers.
#[test]
fn the_mobs_step_folds_the_distance() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y;
    // the body ON the surface column — the zombie 8 east the
    // approach walks (the target scan the 35 ring keeps)
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        surface as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    app.mobs.push(crate::mob::Zombie::new(
        7,
        c as f64 + 8.5,
        surface as f64,
        c as f64 + 0.5,
        3,
    ));
    for _ in 0..80 {
        app.update();
    }
    if !sounds_loaded() {
        assert!(
            app.sounds.events.is_empty(),
            "the empty table the silent voice lane keeps"
        );
        return;
    }
    let steps: Vec<&String> = app
        .sounds
        .events
        .iter()
        .filter(|e| e.starts_with("entity.zombie.step "))
        .collect();
    assert!(
        !steps.is_empty(),
        "the approach's own steps the walk speaks: {:?}",
        app.sounds.events
    );
    for line in &steps {
        // the gain the log line carries — the parse the format's
        // own `#<int> v<gain> p<pitch>` shape answers
        let v: f32 = line
            .rsplit(" v")
            .next()
            .and_then(|s| s.split(' ').next())
            .and_then(|s| s.parse().ok())
            .expect("the gain the log line carries");
        assert!(
            v < 0.15,
            "the fold the step's gain speaks ({v} < 0.15): {line}"
        );
    }
}

/// THE ALERTOTHERS' OWN SIBLING PASS (javap `HurtByTargetGoal`
/// the 26.2 dump — the zombie's `.setAlertOthers(...)` the
/// alertSameType arms, the skeleton's bare ctor the alert NEVER
/// fires, the `getEntitiesOfClass(getClass())` the same-kind
/// filter speaks, the box the unit-cube inflate keeps, the
/// `getTarget() != null` skip the no-target gate answers): the
/// wall-hidden sibling the punched zombie's own event arms — the
/// pack the punch through the fence answers; the archer's
/// retaliation the lone walk keeps.
#[test]
fn the_zombie_alerts_the_box_siblings_the_skeleton_lone_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    // THE REAL SURFACE the top-solid scan speaks (the spawn_y the
    // body's own flight height keeps — the mobs the GROUND level
    // stand: the probe walk the floating wall taught)
    {
        let world = app.offline_world.as_mut().unwrap();
        // THE FLAT PAD the test's own law speaks (the offline
        // terrain's own undulation the probe walk caught — the
        // pad the columns' honest level keeps): stone 0..11, air
        // 11..20 over the whole band the lanes walk
        for x in (c as i32)..=(c as i32 + 24) {
            for z in ((c as i32) - 10)..=((c as i32) + 10) {
                for y in 0..11 {
                    world.set(x, y, z, crate::world::STONE);
                }
                for y in 11..20 {
                    world.set(x, y, z, crate::world::AIR);
                }
            }
        }
        // THE WALL between the body (west) and the hidden pair
        // (east): the zombie A BETWEEN the body and the wall (the
        // scan's own LOS clear), the zombie B + the skeleton S2
        // BEHIND it (the scan refuses both), the skeleton S1
        // beside A (the box's own reach)
        for y in 12..=15 {
            for z in ((c as i32) - 8)..=((c as i32) + 8) {
                world.set(c as i32 + 14, y, z, crate::world::STONE);
                world.set(c as i32 + 15, y, z, crate::world::STONE);
            }
        }
    }
    let surface: i32 = 11;
    app.body = Some(crate::body::PlayerBody::new(
        c as f64 + 0.5,
        surface as f64,
        c as f64 + 0.5,
        0.0,
        0.0,
    ));
    let za = crate::mob::Zombie::new(10, c as f64 + 8.5, surface as f64, c as f64 + 0.5, 7);
    let zb = crate::mob::Zombie::new(11, c as f64 + 20.5, surface as f64, c as f64 + 0.5, 8);
    let s1 = crate::mob::Zombie::new(12, c as f64 + 8.5, surface as f64, c as f64 + 1.5, 9);
    let s2 = crate::mob::Zombie::new(13, c as f64 + 20.5, surface as f64, c as f64 + 1.5, 10);
    app.mobs = vec![za, zb, s1, s2];
    // S1 and S2 the SKELETON kind (the lone-retaliation law's own
    // pair), A and B the zombies the default ctor keeps
    app.mobs[2].kind = crate::mob::MobKind::Skeleton;
    app.mobs[3].kind = crate::mob::MobKind::Skeleton;
    // the settle walk: A's scan targets (open line), B's and S2's
    // refuse (the wall), S1's scan the open line keeps (the same
    // clear band A rides — the 1-cell z offset)
    for _ in 0..3 {
        app.update();
    }
    // THE PUNCH on the zombie A (the direct hurt walk — the melee
    // lane's own word): the canUse's NEW timestamp the start arms,
    // the alertOthers the zombie's own setAlertOthers fires
    app.mobs[0].hurt(2.0, true);
    for _ in 0..12 {
        app.update();
    }
    // THE ZOMBIE PACK: B the wall behind — the scan still refuses,
    // the ALERT the box's own reach arms: the sibling the next
    // tick's own lanes answer (the 300-tick memory the blind walk
    // spends), the charge the melee approach rides
    assert!(
        app.mobs[1].has_target(),
        "the wall-hidden zombie the alert arms"
    );
    assert!(app.mobs[1].aggressive, "the hidden pack the charge rides");
    // THE ARCHER'S OWN LONE WALK: S1 punched? no — S1 stands beside
    // A UNPUNCHED: the alert the skeleton kind never fires (the
    // bare ctor), S1's own scan the open line arms — the two
    // skeletons the zombie punch never re-targets THROUGH the
    // alert lane: S2 the wall behind stays blind (no scan, no
    // alert)
    assert!(
        !app.mobs[3].has_target(),
        "the wall-hidden skeleton the alert never arms"
    );
    assert!(!app.mobs[3].aggressive);
    // THE PUNCHED ZOMBIE'S OWN RETALIATION (the solo arm — the
    // start the canUse's timestamp arms): A the open line the scan
    // already kept — the hurt word the same target re-speaks
    assert!(app.mobs[0].has_target() && app.mobs[0].aggressive);
}

/// THE ANIMALS' OWN STEP VOICE (Entity.playStepSound the base impl
/// the animals inherit — the state's SoundType speaks, NOT the
/// zombie's flat override): the hurt-stamped sheep the panic walk
/// flees, the footsteps the caller plays speak the blocks' own
/// events (the block.*.step rows the generated table keeps at the
/// SoundType's *0.15 fold) — the kind-blind zombie voice the old
/// stand-in kept for every kind, no longer.
#[test]
fn the_walking_sheep_speaks_the_blocks_own_voice() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let mut sheep =
        crate::mob::Zombie::new(7, c as f64 + 0.5, app.spawn_y as f64, c as f64 + 0.5, 5);
    sheep.kind = crate::mob::MobKind::Sheep;
    sheep.on_ground = true;
    app.mobs.push(sheep);
    // the hurt the panic's own stamp arms (the 40-tick window the
    // flee walk speaks; the re-hurt the window's life re-arms —
    // the damage the sheep's 20-health body survives)
    for _ in 0..5 {
        if let Some(m) = app.mobs.iter_mut().find(|m| m.id == 7) {
            m.hurt(1.0, true);
        }
        for _ in 0..30 {
            app.update();
        }
    }
    if !sounds_loaded() {
        assert!(
            app.sounds.events.is_empty(),
            "the empty table the silent voice lane keeps"
        );
        return;
    }
    assert!(
        app.mobs.iter().find(|m| m.id == 7).map(|m| m.move_dist) > Some(1.0),
        "the panic walk accumulated the vanilla law"
    );
    // the sheep's own steps the BLOCK law names (the player never
    // moved — no player block steps compete; the spawner's monsters
    // the zombie event speaks, the filter never reads)
    let block_steps: Vec<&String> = app
        .sounds
        .events
        .iter()
        .filter(|e| e.contains(".step @") && e.starts_with("block."))
        .collect();
    assert!(
        !block_steps.is_empty(),
        "the animals' block law the walk speaks: {:?}",
        app.sounds.events
    );
    for line in &block_steps {
        // the fold the SoundType's base speaks — the *0.15 volume
        // the block law walks (the distance the gain attenuates)
        let v: f32 = line
            .rsplit(" v")
            .next()
            .and_then(|s| s.split(' ').next())
            .and_then(|s| s.parse().ok())
            .expect("the gain the log line carries");
        assert!(v <= 0.15, "the animals' own fold ({v} <= 0.15): {line}");
    }
}

/// THE WOLF'S OWN BITE THE CALLER'S PASS SERVES (the mob-vs-mob
/// hurt the offline frame walks — the sheep the bite's own dose
/// drops, the knockback the attacker-minus-victim law pushes, the
/// kill the death voice + the loot the player-kill gate CLOSED
/// rolls)
#[test]
fn the_wolf_bite_the_caller_pass_serves() {
    let mut app = App::new(5);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y as f64;
    let mut sheep = crate::mob::Zombie::new(30, c as f64 + 0.5, surface, c as f64 + 0.5, 31);
    sheep.kind = crate::mob::MobKind::Sheep;
    sheep.on_ground = true;
    sheep.health = 2.0; // the bite the 4.0 dose the kill answers
    let mut wolf = crate::mob::Zombie::new(31, c as f64 + 1.5, surface, c as f64 + 0.5, 32);
    wolf.kind = crate::mob::MobKind::Wolf;
    wolf.on_ground = true;
    app.mobs = vec![sheep, wolf];
    let drops_before = app.drops.len();
    app.update();
    // THE BITE'S OWN DOSE (the kill the 2.0-health body answers —
    // the hurt pipeline's armor law the 4.0 fang walks)
    let sheep_after = app
        .mobs
        .iter()
        .find(|m| m.id == 30)
        .expect("the prey the slice keeps");
    assert!(
        sheep_after.health <= 0.0,
        "the bite the kill answers: {}",
        sheep_after.health
    );
    // THE KNOCKBACK (the attacker-minus-victim law pushes the prey
    // AWAY from the wolf — the wolf east, the corpse the west ride
    // wears)
    assert!(
        sheep_after.vx < 0.0,
        "the knockback the minus law pushes: {}",
        sheep_after.vx
    );
    // THE LOOT (the death_loot(false) roll the kill speaks — the
    // drops the feet spawn, the player-kill gate CLOSED)
    assert!(app.drops.len() > drops_before, "the loot the kill rolls");
    // THE DEATH CLOCK (the next tick's own walk the corpse speaks)
    app.update();
    let sheep_after = app
        .mobs
        .iter()
        .find(|m| m.id == 30)
        .expect("the prey the slice keeps");
    assert!(
        sheep_after.death_time > 0,
        "the death clock the corpse walks"
    );
}

/// THE COUNTER-FIRE THE APP FRAME SPEAKS (the mob-source dose the
/// caller's bite pass rides — the archer's own draw the world's
/// loop answers): the wolf's fangs the reference arms, the bow
/// the LIVE aim draws, the arrow the flight lane's owner word
/// carries, the pre-pass refresh the aim's live pos wears.
#[test]
fn the_counter_fire_the_app_frame_speaks() {
    let mut app = App::new(5);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y as f64;
    let mut sk = crate::mob::Zombie::new(40, c as f64 + 0.5, surface, c as f64 + 0.5, 41);
    sk.kind = crate::mob::MobKind::Skeleton;
    sk.held_weapon = Some("bow");
    sk.on_ground = true;
    sk.health = 1000.0; // the test's own cheat — the bites the draw survives
    let mut wolf = crate::mob::Zombie::new(41, c as f64 + 4.5, surface, c as f64 + 0.5, 42);
    wolf.kind = crate::mob::MobKind::Wolf;
    wolf.on_ground = true;
    app.mobs = vec![sk, wolf];
    // THE WARM-UP (the age word the dose's timestamp carries —
    // tick zero the goal's own refuse keeps)
    app.update();
    // THE MOB-SOURCE DOSE (the caller's bite pass the same word
    // rides — the direct call the wolf's own melee timing the
    // test keeps out)
    let wv = crate::mob::mob_prey_view(&app.mobs[1]);
    app.mobs[0].hurt_by_mob(4.0, wv);
    let mut fired = None;
    for _ in 0..160 {
        app.update();
        if let Some(a) = app.arrows.iter().find(|a| a.owner == Some(40)) {
            fired = Some(a.clone());
            break;
        }
    }
    let a = fired.expect("the counter-fire the app frame speaks");
    assert!(
        a.vx > 0.0,
        "the wolf sits east — the arrow the minus law never flies: {}",
        a.vx
    );
}

/// THE WIRE MOBS' HURT FLASH (javap LivingEntity's own hurt
/// clocks): the Hurt Animation's mailbox arm lands on the mob (the
/// 10-tick window the render's 0.7 shade speaks), the mob's OWN
/// tick counts it down — the refresh's silence never re-arms the
/// window the countdown already spent, a fresh packet re-arms.
#[test]
fn the_wire_mobs_flash_and_their_own_tick_counts_it_down() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the hit's own packet: the arm rides the FRESH spawn (the
    // window the projection shipped, the flash from tick one)
    let mut hit = wire_zombie(7, 10.0, 64.0, 10.0);
    hit.hurt_time = 10;
    app.live_mobs_updated(vec![hit]);
    assert_eq!(app.net_mobs[0].hurt_time, 10, "the shipped arm lands");
    // the mob's own tick counts the window down (the vanilla
    // client entity's own hurtTime, the store's not the boss)
    app.tick_net_mobs();
    assert_eq!(app.net_mobs[0].hurt_time, 9, "the own tick speaks");
    // the refresh WITHOUT an arm: the countdown STANDS (the
    // mailbox's drained silence never re-arms the spent window)
    app.live_mobs_updated(vec![wire_zombie(7, 10.0, 64.0, 10.0)]);
    assert_eq!(app.net_mobs[0].hurt_time, 9, "the silence never re-arms");
    // the window spends: 9 more ticks walk it to the honest 0
    for _ in 0..9 {
        app.tick_net_mobs();
    }
    assert_eq!(app.net_mobs[0].hurt_time, 0, "the window spent");
    app.tick_net_mobs();
    assert_eq!(app.net_mobs[0].hurt_time, 0, "the 0 floors, never wraps");
    // the RE-HIT: a fresh arm re-arms the flash (the packet's own
    // window, the mob's clock the only owner between packets)
    let mut rehit = wire_zombie(7, 10.0, 64.0, 10.0);
    rehit.hurt_time = 10;
    app.live_mobs_updated(vec![rehit.clone()]);
    assert_eq!(app.net_mobs[0].hurt_time, 10, "the re-hit re-arms");
    // the never-hit mob joins beside the hit one (both ids named):
    // the hit mob's countdown STANDS through the silent refresh,
    // the never-hit id stays the honest 0 the shade never darkens
    // — the arm rides its own id's lane only
    app.live_mobs_updated(vec![rehit, wire_zombie(11, 14.0, 64.0, 10.0)]);
    assert_eq!(app.net_mobs.len(), 2, "both ids ride");
    assert_eq!(app.net_mobs[0].hurt_time, 10, "the silence never re-arms");
    assert_eq!(app.net_mobs[1].hurt_time, 0, "the never-hit stays unhurt");
    app.tick_net_mobs();
    assert_eq!(app.net_mobs[0].hurt_time, 9, "the hit's own walk");
    assert_eq!(app.net_mobs[1].hurt_time, 0, "the silence's own walk");
}

/// THE WIRE MOBS' VOICE LANE (the vanilla handleEntitySound's own
/// gate): the Entity Sound mailbox's shipped voice lands at the
/// entity the level names — the registry name's namespace strip
/// the audio table speaks, the volume/pitch the packet carries
/// verbatim; the unknown id's voice drops (the entity lookup's
/// own gate), the unowned clip name is the honest no-op.
#[test]
fn the_wire_mobs_speak_the_servers_voices() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    app.live_mobs_updated(vec![wire_zombie(7, 10.0, 64.0, 10.0)]);
    let count = |app: &App, prefix: &str| {
        app.sounds
            .events
            .iter()
            .filter(|e| e.starts_with(prefix))
            .count()
    };
    // the ambient groan lands (the registry name's own strip —
    // `minecraft:entity.zombie.ambient` the audio table's
    // `entity.zombie.ambient` clip speaks), the volume/pitch ride
    app.live_voices_updated(vec![crate::world_view::WireVoice {
        entity_id: 7,
        name: "minecraft:entity.zombie.ambient".to_string(),
        volume: 1.0,
        pitch: 0.9,
    }]);
    // THE MIGRATION'S OWN LAW (the assert computes the expected from
    // the ACTUAL loaded state): the cache-absent boot the empty table
    // the silent voice lane keeps — the voice walks below speak where
    // the cache exists
    if !sounds_loaded() {
        assert!(
            app.sounds.events.is_empty(),
            "the empty table the silent voice lane keeps"
        );
        return;
    }
    assert_eq!(count(&app, "entity.zombie.ambient "), 1, "the groan spoke");
    // the hurt's own voice lands the same way
    app.live_voices_updated(vec![crate::world_view::WireVoice {
        entity_id: 7,
        name: "minecraft:entity.zombie.hurt".to_string(),
        volume: 1.0,
        pitch: 1.0,
    }]);
    assert_eq!(count(&app, "entity.zombie.hurt "), 1, "the hurt spoke");
    // the UNKNOWN id's voice drops (the entity lookup's own gate —
    // the vanilla client never plays a voice at a missing entity)
    app.live_voices_updated(vec![crate::world_view::WireVoice {
        entity_id: 404,
        name: "minecraft:entity.zombie.death".to_string(),
        volume: 1.0,
        pitch: 1.0,
    }]);
    assert_eq!(
        count(&app, "entity.zombie.death "),
        0,
        "the ghost is silent"
    );
    // the unowned clip name is the honest no-op (the play_voice's
    // own pick_file return — the sound registry's own verdict)
    app.live_voices_updated(vec![crate::world_view::WireVoice {
        entity_id: 7,
        name: "minecraft:this.sound.does.not.exist".to_string(),
        volume: 1.0,
        pitch: 1.0,
    }]);
    assert_eq!(
        count(&app, "this.sound.does.not.exist "),
        0,
        "the no-op spoke not"
    );
}

/// THE WIRE MOBS' BURNING (the vanilla isOnFire's own query): the
/// shared-flags accessor's bit 0 arms the flame gate through the
/// merge law's own floor (the flame never stutters between the
/// 250ms refreshes), the flag's drop is the douse, the accessor
/// never spoken is the honest unburnt.
#[test]
fn the_wire_mobs_burn_at_the_servers_word() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the burning word rides the FRESH spawn: the flag arms the
    // flame gate (the floor the refresh window's own pacing speaks)
    let mut burning = wire_zombie(7, 10.0, 64.0, 10.0);
    burning.on_fire = true;
    app.live_mobs_updated(vec![burning]);
    assert!(
        app.net_mobs[0].fire_ticks >= App::WIRE_FIRE_FLOOR,
        "the flag arms the flame gate"
    );
    // the refresh keeps the flame lit (the max fold — the floor
    // never resets the arm the query semantics speak): the server
    // STILL says burning, the word rides again
    let mut still_burning = wire_zombie(7, 10.0, 64.0, 10.0);
    still_burning.on_fire = true;
    app.live_mobs_updated(vec![still_burning]);
    assert!(
        app.net_mobs[0].fire_ticks >= App::WIRE_FIRE_FLOOR,
        "the refresh never stutters the flame"
    );
    // the ticks walk: the wire tick owns NO fire clock (the server
    // owns the countdown the flag summarizes) — the arm stands
    app.tick_net_mobs();
    assert!(
        app.net_mobs[0].fire_ticks > 0,
        "the arm stands between the ships"
    );
    // the flag's drop: the douse (the binary word the server speaks)
    app.live_mobs_updated(vec![wire_zombie(7, 10.0, 64.0, 10.0)]);
    assert_eq!(app.net_mobs[0].fire_ticks, 0, "the drop is the douse");
    // the never-hit accessor: the honest unburnt
    app.live_mobs_updated(vec![wire_zombie(11, 14.0, 64.0, 10.0)]);
    assert_eq!(app.net_mobs[0].fire_ticks, 0, "the silence never ignites");
}

/// THE WIRE MOBS' CROUCH (the vanilla isCrouching's own query):
/// the shared-flags accessor's bit 1 arms the avatar's crouch pose
/// the render bends through — the pose is the server's own STATE
/// (no clock, no floor: the bend and the stand both ride the same
/// binary assign), the never-spoken accessor is the honest stand.
#[test]
fn the_wire_mobs_bend_at_the_servers_pose() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the crouch word rides the FRESH spawn: the pose arms (the
    // render state's own isCrouching the model folds)
    let mut crouched = wire_zombie(7, 10.0, 64.0, 10.0);
    crouched.crouching = true;
    app.live_mobs_updated(vec![crouched]);
    assert!(app.net_mobs[0].crouching, "the word bends the fresh spawn");
    // the pose is STATE, not a countdown: the own tick owns NO
    // clock for it (the server owns the pose the flag summarizes)
    app.tick_net_mobs();
    assert!(
        app.net_mobs[0].crouching,
        "the bend stands between the ships"
    );
    // the stand rides the SAME lane: the word's drop is the pose's
    // own drop (the binary assign the merge law speaks)
    app.live_mobs_updated(vec![wire_zombie(7, 10.0, 64.0, 10.0)]);
    assert!(!app.net_mobs[0].crouching, "the stand rides the same lane");
    // the re-bend: the word's return re-poses the matched mob
    let mut re_bent = wire_zombie(7, 10.0, 64.0, 10.0);
    re_bent.crouching = true;
    app.live_mobs_updated(vec![re_bent, wire_zombie(11, 14.0, 64.0, 10.0)]);
    assert_eq!(app.net_mobs.len(), 2, "both ids ride");
    assert!(app.net_mobs[0].crouching, "the word re-bends");
    assert!(
        !app.net_mobs[1].crouching,
        "the never-spoken id stands (the arm rides its own lane only)"
    );
}

/// THE SPIDER'S OWN CLIMB WORD (the wire lane's fifth body — the
/// `minecraft:spider` the projection now names): the byte's bit
/// 0x1 the projection decoded, the merge the state carries (the
/// fresh spawn AND the lerpTo arm the same binary assign speaks —
/// the flag is STATE, the own tick owns no clock for it), the
/// never-spoken accessor the honest flat crawl keeps.
#[test]
fn the_wire_spider_climb_word_rides() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the climbing spider: the word rides the FRESH spawn (the
    // kind the name arms — the arachnid body the mesh bakes)
    let mut climber = wire_mob_named(30, 10.0, 64.0, 10.0, "minecraft:spider");
    climber.climbing = true;
    app.live_mobs_updated(vec![climber]);
    assert_eq!(
        app.net_mobs[0].kind,
        crate::mob::MobKind::Spider,
        "the spider's own kind"
    );
    assert!(
        app.net_mobs[0].climbing,
        "the wall's press the fresh spawn wears"
    );
    // the state stands between the ships (the server owns the
    // climb word the flag summarizes)
    app.tick_net_mobs();
    assert!(
        app.net_mobs[0].climbing,
        "the flag stands between the ships"
    );
    // the open air: the word's drop disarms (the same binary
    // assign the crouch lane speaks)
    app.live_mobs_updated(vec![wire_mob_named(
        30,
        12.0,
        64.0,
        10.0,
        "minecraft:spider",
    )]);
    assert!(!app.net_mobs[0].climbing, "the open air the flag dies");
    // the other kinds' own lane: the honest false the never-spoken
    // accessor keeps (the spider-gated decode the view wears)
    app.live_mobs_updated(vec![wire_zombie(31, 14.0, 64.0, 10.0)]);
    assert!(
        !app.net_mobs.iter().find(|m| m.id == 31).unwrap().climbing,
        "the zombie's honest false the gate keeps"
    );
}

/// THE WIRE BABY'S OWN VERDICT (the slot-16 Boolean the merge
/// translates — the vanilla CLIENT's own getAge split the -1/1
/// words wear): the small one's word the sim's `age < 0` law
/// answers (the scale lane's 0.5 the verdict arms), the grow-up's
/// own word the same binary assign keeps, the fresh spawn AND the
/// lerpTo arm the same lane speaks.
#[test]
fn the_wire_baby_word_the_age_verdict_wears() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the baby lamb: the word rides the FRESH spawn (the -1 the
    // client's own split wears, the verdict the age law answers)
    let mut lamb = wire_mob_named(40, 10.0, 64.0, 10.0, "minecraft:sheep");
    lamb.baby = true;
    app.live_mobs_updated(vec![lamb]);
    assert_eq!(
        app.net_mobs[0].age, -1,
        "the client's own -1 the split wears"
    );
    assert!(app.net_mobs[0].is_baby(), "the small one the verdict names");
    // the verdict stands between the ships (the own tick owns NO
    // growth clock for the wire body — the server owns the age)
    app.tick_net_mobs();
    assert!(
        app.net_mobs[0].is_baby(),
        "the verdict stands between the ships"
    );
    // the lerpTo arm: the grow-up's own word the same lane wears
    // (the 1 the adult word speaks)
    app.live_mobs_updated(vec![wire_mob_named(
        40,
        12.0,
        64.0,
        10.0,
        "minecraft:sheep",
    )]);
    assert_eq!(app.net_mobs[0].age, 1, "the client's own 1 the adult wears");
    assert!(!app.net_mobs[0].is_baby(), "the grown-up the verdict keeps");
    // the zombie's own accessor rides the SAME slot (the monster
    // branch's own DATA_BABY_ID the 16 the probe answered): the
    // baby zombie the same verdict wears
    let mut baby_z = wire_zombie(41, 14.0, 64.0, 10.0);
    baby_z.baby = true;
    app.live_mobs_updated(vec![baby_z]);
    let z = app.net_mobs.iter().find(|m| m.id == 41).unwrap();
    assert!(z.is_baby(), "the baby zombie the same verdict wears");
    assert_eq!(z.age, -1, "the same -1 the split keeps");
}

/// THE WIRE'S OWN HAND (the equipment word the live lane carries —
/// the armed zombie the record the render lane draws): the wire
/// mob's word the fresh spawn rides (the projection resolved the
/// Set Equipment store's MAINHAND), the lerpTo lane the STATE
/// replace speaks (the pickup the word arms, the empty-stack drop
/// the honest unarmed keeps), the offline lane the wire's word
/// never touches.
#[test]
fn the_wire_equipment_the_record_wears() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the armed zombie: the word rides the FRESH spawn (the render
    // lane's held chain reads the record from tick one)
    let mut armed = wire_zombie(20, 14.0, 64.0, 10.0);
    armed.held_weapon = Some("iron_sword");
    app.live_mobs_updated(vec![armed]);
    assert_eq!(
        app.net_mobs[0].held_weapon,
        Some("iron_sword"),
        "the armed spawn the hand wears"
    );
    // the lerpTo lane: the STATE assign — the pickup the word
    // replaces (the same binary assign the name/crouch lanes speak)
    let mut swapped = wire_zombie(20, 15.0, 64.0, 10.0);
    swapped.held_weapon = Some("iron_shovel");
    swapped.held_offhand = Some("shield");
    app.live_mobs_updated(vec![swapped]);
    assert_eq!(
        app.net_mobs[0].held_weapon,
        Some("iron_shovel"),
        "the pickup the server speaks replaces"
    );
    assert_eq!(
        app.net_mobs[0].held_offhand,
        Some("shield"),
        "the left hand the slot-1 word wears"
    );
    // the drop: the word's None the honest unarmed keeps (the
    // empty stack the packet's own clear the store merged) — the
    // offhand's own word the OTHER slot the drop never touches
    app.live_mobs_updated(vec![wire_zombie(20, 16.0, 64.0, 10.0)]);
    assert_eq!(
        app.net_mobs[0].held_weapon, None,
        "the drop the packet's empty stack speaks"
    );
    assert_eq!(
        app.net_mobs[0].held_offhand, None,
        "the never-spoken slot the projection the honest empty keeps"
    );
    // the never-armed id: the honest unarmed (the arm rides its
    // own lane only)
    app.live_mobs_updated(vec![wire_zombie(21, 18.0, 64.0, 12.0)]);
    assert_eq!(app.net_mobs[0].held_weapon, None, "the silence never arms");
    // THE ARMOR WORDS ride BOTH lanes (the same binary assign the
    // hands' lanes speak — the dress the fresh spawn arms, the
    // lerpTo the STATE assign the re-dress speaks, the silence the
    // honest bare keeps)
    let mut dressed = wire_zombie(22, 20.0, 64.0, 14.0);
    dressed.armor = [Some("diamond_boots"), None, Some("iron_chestplate"), None];
    app.live_mobs_updated(vec![dressed]);
    assert_eq!(
        app.net_mobs[0].armor,
        [Some("diamond_boots"), None, Some("iron_chestplate"), None],
        "the dressed spawn the rows wear"
    );
    // the re-dress: the rows the STATE assign replaces (the undress
    // the server speaks the rows bare keep)
    let mut redressed = wire_zombie(22, 21.0, 64.0, 14.0);
    redressed.armor = [None, None, None, Some("golden_helmet")];
    app.live_mobs_updated(vec![redressed]);
    assert_eq!(
        app.net_mobs[0].armor,
        [None, None, None, Some("golden_helmet")],
        "the re-dress the STATE assign speaks"
    );
    app.live_mobs_updated(vec![wire_zombie(22, 22.0, 64.0, 14.0)]);
    assert_eq!(
        app.net_mobs[0].armor, [None; 4],
        "the silence the bare keeps"
    );
}

/// THE WIRE SKELETON the record lane wears (the Add Entity type
/// name the from_wire_name walk arms, the held bow the Set
/// Equipment store's own MAINHAND word rides): the kind the merge
/// arms, the bow the record keeps both ways (the fresh spawn the
/// hand arms from tick one, the lerpTo the pickup/drop the server
/// speaks replaces — the same binary law the zombie's own lane
/// wears), the armor rows the same store walk keeps.
#[test]
fn the_wire_skeleton_the_bow_lane_wears() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the kind the type name arms (the from_wire_name walk the
    // merge speaks)
    let mut archer = wire_mob_named(30, 20.0, 64.0, 10.0, "minecraft:skeleton");
    archer.held_weapon = Some("bow");
    app.live_mobs_updated(vec![archer]);
    assert_eq!(
        app.net_mobs[0].kind,
        crate::mob::MobKind::Skeleton,
        "the type name the skeleton kind arms"
    );
    assert_eq!(
        app.net_mobs[0].held_weapon,
        Some("bow"),
        "the equipment store's MAINHAND the bow the hand wears"
    );
    // the lerpTo lane: the pickup the server speaks replaces (the
    // same STATE assign the zombie's own word wears)
    let mut disarmed = wire_mob_named(30, 21.0, 64.0, 10.0, "minecraft:skeleton");
    disarmed.held_weapon = Some("iron_sword");
    app.live_mobs_updated(vec![disarmed]);
    assert_eq!(
        app.net_mobs[0].held_weapon,
        Some("iron_sword"),
        "the pickup the STATE assign speaks"
    );
    // the drop: the None the honest unarmed keeps
    app.live_mobs_updated(vec![wire_mob_named(
        30,
        22.0,
        64.0,
        10.0,
        "minecraft:skeleton",
    )]);
    assert_eq!(
        app.net_mobs[0].held_weapon, None,
        "the empty stack the packet's own clear keeps"
    );
    // the armor rows the same store walk keeps (the
    // AbstractSkeletonRenderer the humanoid layers render)
    let mut dressed = wire_mob_named(31, 24.0, 64.0, 12.0, "minecraft:skeleton");
    dressed.armor = [Some("iron_boots"), None, None, Some("iron_helmet")];
    app.live_mobs_updated(vec![dressed]);
    assert_eq!(
        app.net_mobs[0].armor,
        [Some("iron_boots"), None, None, Some("iron_helmet")],
        "the dressed skeleton the rows wear"
    );
}

/// THE SECOND BODY'S OWN MERGE (the registry name the Add Entity
/// carries): the wire PIG lands through the same live_mobs_updated
/// entry the zombies ride — the kind the name arms draws the
/// quadruped mesh, the lerpTo walk speaks the SAME ease, the ids
/// the wire stops naming despawn the same law.
#[test]
fn the_wire_pig_walks_the_same_lerpto_lane() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the pig + the zombie share the ring (the ids apart, the kinds
    // the registry names armed)
    app.live_mobs_updated(vec![
        wire_zombie(7, 10.0, 64.0, 10.0),
        wire_mob_named(8, 12.0, 64.0, 10.0, "minecraft:pig"),
    ]);
    assert_eq!(app.net_mobs.len(), 2, "both bodies land");
    let pig = app.net_mobs.iter().find(|m| m.id == 8).unwrap();
    assert_eq!(pig.kind, crate::mob::MobKind::Pig, "the pig's own kind");
    let zombie = app.net_mobs.iter().find(|m| m.id == 7).unwrap();
    assert_eq!(
        zombie.kind,
        crate::mob::MobKind::Zombie,
        "the zombie's kind stands"
    );
    // the SECOND ship rides the MATCHED branch (the spawn's pose
    // was the first word, the ease the cadence's own next beat)
    app.live_mobs_updated(vec![
        wire_zombie(7, 10.5, 64.0, 10.0),
        wire_mob_named(8, 12.5, 64.0, 10.0, "minecraft:pig"),
    ]);
    let pig = app.net_mobs.iter().find(|m| m.id == 8).unwrap();
    assert!(pig.lerp_target.is_some(), "the ease armed");
    assert_eq!(pig.lerp_steps, 3, "the 3-tick walk the cadence arms");
    app.tick_net_mobs();
    let pig = app.net_mobs.iter().find(|m| m.id == 8).unwrap();
    assert_eq!(pig.lerp_steps, 2, "the own tick consumed one step");
    // the pig's head rides level (the pitch the wire spoke)
    assert_eq!(pig.head_pitch, 0.0);
    // the despawn law: the wire stops naming the pig, the ghost
    // never re-enters
    app.live_mobs_updated(vec![wire_zombie(7, 11.0, 64.0, 10.0)]);
    assert_eq!(
        app.net_mobs.len(),
        1,
        "the pig despawned at the wire's word"
    );
    assert_eq!(app.net_mobs[0].id, 7);
}

#[test]
fn the_wire_cow_walks_the_same_lerpto_lane() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the three bodies share the ring (the ids apart, the kinds
    // the registry names armed — the bovine the third word)
    app.live_mobs_updated(vec![
        wire_zombie(7, 10.0, 64.0, 10.0),
        wire_mob_named(8, 12.0, 64.0, 10.0, "minecraft:pig"),
        wire_mob_named(9, 14.0, 64.0, 10.0, "minecraft:cow"),
    ]);
    assert_eq!(app.net_mobs.len(), 3, "all three bodies land");
    let cow = app.net_mobs.iter().find(|m| m.id == 9).unwrap();
    assert_eq!(cow.kind, crate::mob::MobKind::Cow, "the cow's own kind");
    // the SECOND ship rides the MATCHED branch (the spawn's pose
    // was the first word, the ease the cadence's own next beat)
    app.live_mobs_updated(vec![
        wire_zombie(7, 10.5, 64.0, 10.0),
        wire_mob_named(8, 12.5, 64.0, 10.0, "minecraft:pig"),
        wire_mob_named(9, 14.5, 64.0, 10.0, "minecraft:cow"),
    ]);
    let cow = app.net_mobs.iter().find(|m| m.id == 9).unwrap();
    assert!(cow.lerp_target.is_some(), "the ease armed");
    assert_eq!(cow.lerp_steps, 3, "the 3-tick walk the cadence arms");
    app.tick_net_mobs();
    let cow = app.net_mobs.iter().find(|m| m.id == 9).unwrap();
    assert_eq!(cow.lerp_steps, 2, "the own tick consumed one step");
    // the cow's head rides level (the pitch the wire spoke)
    assert_eq!(cow.head_pitch, 0.0);
    // the despawn law: the wire stops naming the cow, the ghost
    // never re-enters
    app.live_mobs_updated(vec![
        wire_zombie(7, 11.0, 64.0, 10.0),
        wire_mob_named(8, 13.0, 64.0, 10.0, "minecraft:pig"),
    ]);
    assert_eq!(
        app.net_mobs.len(),
        2,
        "the cow despawned at the wire's word"
    );
    assert!(app.net_mobs.iter().all(|m| m.id != 9));
}

#[test]
fn the_wire_sheep_walks_the_same_lerpto_lane() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the four bodies share the ring (the ids apart, the kinds
    // the registry names armed — the ovine the fourth word)
    app.live_mobs_updated(vec![
        wire_zombie(7, 10.0, 64.0, 10.0),
        wire_mob_named(8, 12.0, 64.0, 10.0, "minecraft:pig"),
        wire_mob_named(9, 14.0, 64.0, 10.0, "minecraft:cow"),
        wire_mob_named(10, 16.0, 64.0, 10.0, "minecraft:sheep"),
    ]);
    assert_eq!(app.net_mobs.len(), 4, "all four bodies land");
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert_eq!(
        sheep.kind,
        crate::mob::MobKind::Sheep,
        "the sheep's own kind"
    );
    // the SECOND ship rides the MATCHED branch (the spawn's pose
    // was the first word, the ease the cadence's own next beat)
    app.live_mobs_updated(vec![
        wire_zombie(7, 10.5, 64.0, 10.0),
        wire_mob_named(8, 12.5, 64.0, 10.0, "minecraft:pig"),
        wire_mob_named(9, 14.5, 64.0, 10.0, "minecraft:cow"),
        wire_mob_named(10, 16.5, 64.0, 10.0, "minecraft:sheep"),
    ]);
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert!(sheep.lerp_target.is_some(), "the ease armed");
    assert_eq!(sheep.lerp_steps, 3, "the 3-tick walk the cadence arms");
    app.tick_net_mobs();
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert_eq!(sheep.lerp_steps, 2, "the own tick consumed one step");
    // the sheep's head rides level (the wire's pitch word is data
    // the ovine model's own eat-angle replacement never reads —
    // the bytecode's own putfield)
    assert_eq!(sheep.head_pitch, 0.0);
    // the despawn law: the wire stops naming the sheep, the ghost
    // never re-enters
    app.live_mobs_updated(vec![
        wire_zombie(7, 11.0, 64.0, 10.0),
        wire_mob_named(8, 13.0, 64.0, 10.0, "minecraft:pig"),
        wire_mob_named(9, 15.0, 64.0, 10.0, "minecraft:cow"),
    ]);
    assert_eq!(
        app.net_mobs.len(),
        3,
        "the sheep despawned at the wire's word"
    );
    assert!(app.net_mobs.iter().all(|m| m.id != 10));
}

#[test]
fn the_wire_sheep_sheds_and_dyes_at_the_servers_word() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the sheep lands with the vanilla default (byte 0: unsheared,
    // WHITE — the SynchedEntityData ctor's own define)
    app.live_mobs_updated(vec![wire_mob_named(
        10,
        16.0,
        64.0,
        10.0,
        "minecraft:sheep",
    )]);
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert!(!sheep.sheared, "the default's own unshorn");
    assert_eq!(sheep.wool_color, 0, "the default's own white");
    // THE SHEARED + DYED WORD (byte 0x1E = the sheared bit + the
    // RED nibble the isSheared/getColor decode speaks): the SECOND
    // ship rides the matched branch's own binary assign
    let mut w = wire_mob_named(10, 16.5, 64.0, 10.0, "minecraft:sheep");
    w.wool_byte = 0x1E;
    app.live_mobs_updated(vec![w]);
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert!(sheep.sheared, "the sheared bit's own word");
    assert_eq!(sheep.wool_color, 14, "the red nibble the mask speaks");
    // THE UNSHORN WORD (byte 14: the dye rides, the bit drops —
    // state not countdown, the same assign carries both)
    let mut w = wire_mob_named(10, 17.0, 64.0, 10.0, "minecraft:sheep");
    w.wool_byte = 14;
    app.live_mobs_updated(vec![w]);
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert!(!sheep.sheared, "the bit's drop the same assign speaks");
    assert_eq!(sheep.wool_color, 14, "the dye stands");
}

/// THE JEB_ SHEEP'S OWN NAME (MobRenderer.checkMagicName's law —
/// the custom name the accessor 2 carries, the "jeb_" equality the
/// rainbow gate reads): the name rides the merge's own binary
/// assign, the rename replaces, the unnamed silence drops the word.
#[test]
fn the_wire_sheep_answers_to_its_name() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    // the named sheep lands (the jeb_ word the wire carries)
    let mut w = wire_mob_named(10, 16.0, 64.0, 10.0, "minecraft:sheep");
    w.custom_name = Some("jeb_".to_string());
    app.live_mobs_updated(vec![w]);
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert_eq!(
        sheep.custom_name.as_deref(),
        Some("jeb_"),
        "the name the merge carries"
    );
    // THE VISIBILITY BIT rides the same carry (the accessor 3's
    // Boolean — the shouldShowName gate's own first arm): the
    // raised bit arms the tag, the silent byte disarms the arm
    // the spawn spoke
    let mut w = wire_mob_named(10, 16.0, 64.0, 10.0, "minecraft:sheep");
    w.custom_name = Some("Named".to_string());
    w.name_visible = true;
    app.live_mobs_updated(vec![w]);
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert!(sheep.name_visible, "the raised bit the merge carries");
    let w = wire_mob_named(10, 16.2, 64.0, 10.0, "minecraft:sheep");
    app.live_mobs_updated(vec![w]);
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert!(!sheep.name_visible, "the silent byte the disarm speaks");
    // the RENAME rides the same assign (state, not countdown —
    // the server's word replaces the old one)
    let mut w = wire_mob_named(10, 16.5, 64.0, 10.0, "minecraft:sheep");
    w.custom_name = Some("Dinnerbone".to_string());
    app.live_mobs_updated(vec![w]);
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert_eq!(
        sheep.custom_name.as_deref(),
        Some("Dinnerbone"),
        "the rename the same assign speaks"
    );
    // the SILENCE unnamed (the accessor never spoken — the honest
    // None the projection's own default rides)
    let w = wire_mob_named(10, 17.0, 64.0, 10.0, "minecraft:sheep");
    app.live_mobs_updated(vec![w]);
    let sheep = app.net_mobs.iter().find(|m| m.id == 10).unwrap();
    assert_eq!(sheep.custom_name, None, "the silence the unnamed speaks");
}

/// THE NAMEPLATE'S OWN PLANES — the geometry the 26.2 raw walk
/// speaks: the +0.5 lift over the AT_HEIGHT attachment, the 0.025
/// font-px scale (the negative-y the flip speaks), the centered
/// x the ineg/fdiv computes, the markBackground box the min/max
/// folds bound, the glyph cells the font table's own law rides.
#[test]
fn the_name_tag_planes_walk_the_billboard_law() {
    let feet = crate::math::Vec3::new(10.5, 64.0, 10.5);
    // the sheep's own total lift (the attachment's bbHeight + the
    // dadd's 0.5): the SHEEP_HEIGHT constant the bbox law speaks
    let top_y = 64.0 + crate::mob::SHEEP_HEIGHT + 0.5;
    let name = "jeb_";
    let w = crate::gui::font_text_width(name);
    let x0 = -(w as f32) / 2.0;
    let (bg, glyphs) = name_tag_planes(feet, top_y, name, 0.0, 0.0);
    // yaw 0 pitch 0: right = +x, up = +y — the corner law the
    // basis fold speaks: world = feet + (fx*k, -fy*k)
    let bg = bg.expect("the box the see-through pass draws");
    // the [BL, BR, TR, TL] order the raster's edge fold accepts:
    // the bottom edge at fy = 9 (the 8px cell + the pad below),
    // the top edge at fy = -1 (the markBackground top pad — one
    // font px ABOVE the anchor)
    assert!(
        (bg[0][1] - (top_y as f32 - 0.225)).abs() < 1e-4,
        "the box's bottom the y+9 law drops"
    );
    assert!(
        (bg[2][1] - (top_y as f32 + 0.025)).abs() < 1e-4,
        "the box's top the -1 pad lifts"
    );
    // the left/right edges the centered x ± the pads speak
    assert!(
        (bg[0][0] - (10.5 + (x0 - 1.0) * 0.025)).abs() < 1e-4,
        "the box's left the x-1 pad speaks"
    );
    assert!(
        (bg[1][0] - (10.5 + (x0 + w as f32) * 0.025)).abs() < 1e-4,
        "the box's right the x+w law speaks"
    );
    // THE GLYPHS: one plane per non-space char, the cells the
    // font table bakes (top = ascent - gh, the page texel window).
    // Cache-present boots only — the metric pins the measured table
    // owns (the no-cache CI boot the honest empty table keeps, the
    // glyph walk then renders nothing to pin).
    if !crate::font_table::real() {
        return;
    }
    assert_eq!(glyphs.len(), 4, "j/e/b/_ the four planes the walk speaks");
    let mut pen = x0;
    for (gl, ch) in glyphs.iter().zip(name.chars()) {
        let g = crate::font_table::glyph(ch).unwrap();
        let top = (g.ascent as i32 - g.gh as i32) as f32;
        assert!(
            (gl.pts[0][0] - (10.5 + pen * 0.025)).abs() < 1e-4,
            "the cell's left the pen walk speaks"
        );
        assert!(
            (gl.pts[0][1] - (top_y as f32 - (top + g.gh as f32) * 0.025)).abs() < 1e-4,
            "the cell's bottom the ascent-gh + gh walk speaks"
        );
        assert!(
            (gl.pts[3][1] - (top_y as f32 - top * 0.025)).abs() < 1e-4,
            "the cell's top the ascent-gh law speaks"
        );
        assert_eq!(
            gl.uv[0],
            [g.u as f32, (g.v + g.gh) as f32],
            "the cell's bottom-left the page window speaks"
        );
        assert_eq!(
            gl.uv[2],
            [(g.u + g.gw) as f32, g.v as f32],
            "the cell's top-right the page window speaks"
        );
        pen += g.gw as f32 + 1.0;
    }
    // THE BILLBOARD'S OWN TILT (the yaw the camera basis rides):
    // yaw 90 makes right = +z (the [cy, 0, sy] fold speaks) — the
    // box's font-left edge walks the tag's own plane in Z while X
    // stays the anchor's (the right axis carries no x)
    let (bg_t, _) = name_tag_planes(feet, top_y, "a", 90f32.to_radians(), 0.0);
    let bg_t = bg_t.unwrap();
    let w_a = crate::gui::font_text_width("a");
    let x0_a = -(w_a as f32) / 2.0;
    assert!(
        (bg_t[0][0] - 10.5).abs() < 1e-4,
        "the yaw's right axis carries no x"
    );
    assert!(
        (bg_t[0][2] - (10.5 + (x0_a - 1.0) * 0.025)).abs() < 1e-4,
        "the font-left edge walks z the yaw-90 basis speaks"
    );
    let (bg_p, _) = name_tag_planes(feet, top_y, "a", 0.0, 90f32.to_radians());
    let bg_p = bg_p.unwrap();
    assert!(
        (bg_p[0][0] - (10.5 + (x0_a - 1.0) * 0.025)).abs() < 1e-4,
        "the right axis stays the yaw's +x"
    );
    assert!(
        (bg_p[0][2] - (10.5 - 0.225)).abs() < 1e-4,
        "the pitch re-aims the UP axis: at 90 the bottom pad walks -z"
    );
}

#[test]
fn live_view_updates_swap_the_network_world_in_place() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    // the join landed (an all-air window: the state machine must not care)
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert!(app.world_is_network);
    assert!(
        app.world_quads.as_ref().unwrap().is_empty(),
        "air meshes to nothing"
    );
    // no biome known: the vanilla in-game default situation
    assert_eq!(app.music.game_tracks(), ["music.game"]);

    // a live update ships a world WITH content + a pose + the biome
    // at the player: the swap is IN PLACE — net_world/net_camera
    // refresh, the showing screen re-meshes (the terrain changes
    // under the player's eyes, the vanilla chunk-update behavior)
    // and the music situation resolves to the biome's own event
    let mut streamed = air_net_world(2, 7);
    streamed.chunks[0][0] = crate::world::STONE;
    let pose = crate::camera::Camera {
        pos: Vec3::new(1.0, 2.0, 3.0),
        yaw: 0.5,
        pitch: -0.1,
    };
    app.live_view_updated(streamed, Some(pose), Some("minecraft:swamp".into()), None);
    let w = app
        .net_world
        .as_ref()
        .expect("the streamed world swapped in");
    assert_eq!(
        w.seed, 7,
        "the update REPLACED the join world (newest wins)"
    );
    assert_eq!(
        w.get(0, 0, 0),
        crate::world::STONE,
        "the streamed block shows"
    );
    assert_eq!(
        app.net_camera,
        Some(pose),
        "the refreshed pose drives the camera"
    );
    assert_eq!(
        app.world_camera(),
        pose,
        "world_is_network renders FROM the pose"
    );
    assert!(
        !app.world_quads
            .as_ref()
            .expect("re-meshed on the live update")
            .is_empty(),
        "the streamed terrain re-meshed under the player"
    );
    // the biome change re-resolved the music situation — the swamp's
    // official background-music event, from the GENERATED table
    assert_eq!(app.net_biome.as_deref(), Some("minecraft:swamp"));
    use mc_registry::generated::worldgen as wm;
    let swamp_event = wm::biome_music("minecraft:swamp", "default")
        .map(|m| m.event)
        .unwrap_or("music.game");
    assert_eq!(app.music.game_tracks(), [swamp_event]);
    // a SAME-biome refresh never restarts the situation (the vanilla
    // behavior — the scheduler is not touched, its event log frozen)
    let events_before = app.music.events.len();
    app.live_view_updated(
        air_net_world(2, 8),
        Some(pose),
        Some("minecraft:swamp".into()),
        None,
    );
    assert_eq!(app.net_biome.as_deref(), Some("minecraft:swamp"));
    assert_eq!(
        app.music.events.len(),
        events_before,
        "no situation restart"
    );
    // an UNRESOLVABLE biome (None) keeps what is playing — no
    // knowledge is not a change
    app.live_view_updated(air_net_world(2, 9), Some(pose), None, None);
    assert_eq!(app.net_biome.as_deref(), Some("minecraft:swamp"));
    assert_eq!(app.music.events.len(), events_before);
    // and a real change re-fires once
    app.live_view_updated(
        air_net_world(2, 10),
        Some(pose),
        Some("minecraft:desert".into()),
        None,
    );
    assert_eq!(app.net_biome.as_deref(), Some("minecraft:desert"));
    let desert_event = wm::biome_music("minecraft:desert", "default")
        .map(|m| m.event)
        .unwrap_or("music.game");
    assert_eq!(app.music.game_tracks(), [desert_event]);
}

#[test]
fn live_updates_drop_after_save_and_quit() {
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(
        Some("minecraft:swamp".into()),
        Skybox::Overworld,
        None,
        None,
    );
    assert_eq!(app.net_biome.as_deref(), Some("minecraft:swamp"));
    // Save and Quit unloads the level AND closes the live session
    let run = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    app.attach_live_run(1, std::sync::Arc::clone(&run));
    app.paused = true;
    app.render_title_buttons_for_test();
    let quit = app
        .buttons
        .iter()
        .find(|b| b.id == "saveandquit")
        .expect("saveandquit")
        .clone();
    app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
    assert_eq!(app.screen, ScreenKind::Title);
    assert!(
        !run.load(std::sync::atomic::Ordering::Relaxed),
        "the session's flag flipped"
    );
    assert!(
        app.live_runs.is_empty(),
        "the closed session's entry is gone"
    );

    // a LATE live update (the worker raced its final refresh): the
    // world the UI no longer shows — dropped honestly, nothing
    // re-meshes, the title screen stays
    let mut late = air_net_world(2, 9);
    late.chunks[0][0] = crate::world::STONE;
    app.live_view_updated(
        late,
        Some(crate::camera::Camera {
            pos: Vec3::new(1.0, 2.0, 3.0),
            yaw: 0.0,
            pitch: 0.0,
        }),
        Some("minecraft:plains".into()),
        None,
    );
    assert_eq!(
        app.screen,
        ScreenKind::Title,
        "the late update never opens a screen"
    );
    assert_eq!(
        app.net_world
            .as_ref()
            .expect("net_world dormant, not cleared")
            .seed,
        0,
        "the stale join world is untouched by the dropped update"
    );
    assert!(app.world_quads.is_none(), "no mesh reappeared");
    assert!(app.net_camera.is_none(), "no pose reappeared");
    assert!(
        app.net_biome.is_none(),
        "no situation reappeared for a dead world"
    );
    // a late session death drops equally silently
    app.live_session_lost("late kick".to_string());
    assert_eq!(
        app.screen,
        ScreenKind::Title,
        "no world to lose — no disconnect screen"
    );
}

/// THE CONTAINER TITLE'S OWN DECODE (the OpenScreen Component's
/// rows the walk reads): the `text` literal the anvil rename
/// speaks, the `translate` row the lang table answers — the
/// container.dropper default the live dropper window wears, the
/// missing row the key ITSELF answers (the getOrDefault law) —
/// the `extra` siblings the concatenation rides, the empty or
/// truncated bytes the honest None keeps (the default lang row
/// the canvas banks).
#[test]
fn the_container_title_decode_walks_the_component_rows() {
    use mc_protocol::nbt::{write_network_nbt, Nbt};
    // the wire's own encoder: the anonymous-root compound the
    // parse retains (the write side the reader's mirror is)
    let enc = |v: &Nbt| -> Vec<u8> {
        let mut b = Vec::new();
        write_network_nbt(v, &mut b).unwrap();
        b
    };
    // the anvil rename's own literal
    assert_eq!(
        decode_container_title(&enc(&Nbt::Compound(vec![(
            "text".to_string(),
            Nbt::String("Rename Machine".to_string()),
        )]))),
        Some("Rename Machine".to_string()),
        "the text row the literal speaks"
    );
    // the live dropper window's default name (the translatable
    // the lang table answers — the Dropper row the canvas draws)
    assert_eq!(
        decode_container_title(&enc(&Nbt::Compound(vec![(
            "translate".to_string(),
            Nbt::String("container.dropper".to_string()),
        )]))),
        Some("Dropper".to_string()),
        "the translate row the lang table answers"
    );
    // the missing row the key ITSELF answers (getOrDefault)
    assert_eq!(
        decode_container_title(&enc(&Nbt::Compound(vec![(
            "translate".to_string(),
            Nbt::String("container.server.custom".to_string()),
        )]))),
        Some("container.server.custom".to_string()),
        "the untranslatable key displays as its key"
    );
    // the extra siblings the concatenation rides
    assert_eq!(
        decode_container_title(&enc(&Nbt::Compound(vec![(
            "extra".to_string(),
            Nbt::List(vec![
                Nbt::Compound(vec![("text".to_string(), Nbt::String("a".to_string()),)]),
                Nbt::Compound(vec![(
                    "translate".to_string(),
                    Nbt::String("container.dropper".to_string()),
                )]),
            ])
        ),]))),
        Some("aDropper".to_string()),
        "the extras ride after the base"
    );
    // the honest Nones: the empty root, the truncated bytes, the
    // styled compound with neither row (the scalar shapes the
    // walk never understands)
    assert_eq!(
        decode_container_title(&[0x0A, 0x00]),
        None,
        "the empty root"
    );
    assert_eq!(decode_container_title(&[0x0A]), None, "the truncated root");
    assert_eq!(decode_container_title(&[]), None, "no bytes at all");
    assert_eq!(
        decode_container_title(&enc(&Nbt::Compound(vec![(
            "color".to_string(),
            Nbt::String("red".to_string()),
        )]))),
        None,
        "the styled compound with neither row"
    );
}

/// THE LIVE WINDOW'S OWN TITLE LANE (the 3x3 canvas the wire's
/// own name rides): the OpenScreen Component the projection
/// feeds, the decode the dropper window's real default answers
/// (the menu type 6 the BlockEntity pair shares — the title the
/// ONLY distinguishing row the canvas reads), the fresh window's
/// re-speak the rename ride, the store's drop the title kills.
#[test]
fn the_live_dispenser_window_wears_the_wires_title() {
    use crate::world_view::LiveContainer;
    use mc_protocol::nbt::{write_network_nbt, Nbt};
    let enc = |v: &Nbt| -> Vec<u8> {
        let mut b = Vec::new();
        write_network_nbt(v, &mut b).unwrap();
        b
    };
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World);

    // the server opens a DROPPER (the same GENERIC_3x3 menu the
    // dispenser window shares): the wire's own translatable the
    // decode answers — the Dropper row the live canvas wears
    // (the offline kind-0 default the honest gap's old law kept)
    let dropper = LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_GENERIC_3X3,
        slots: vec![(0i32, 0i32); 45],
        carried: (0, 0),
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: enc(&Nbt::Compound(vec![(
            "translate".to_string(),
            Nbt::String("container.dropper".to_string()),
        )])),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(dropper.clone()));
    assert_eq!(
        app.screen,
        ScreenKind::Dispenser,
        "the window shows over the world"
    );
    assert!(app.dispenser_is_live, "the live lane owns the screen now");
    assert_eq!(
        app.live_dispenser_title.as_deref(),
        Some("Dropper"),
        "the wire's own translatable the canvas banks"
    );

    // a NEW window speaks a CUSTOM name (the anvil rename the
    // text row carries): the fresh decode replaces the default
    let mut custom = dropper.clone();
    custom.state_id = 9;
    custom.title_nbt = enc(&Nbt::Compound(vec![(
        "text".to_string(),
        Nbt::String("Rename Machine".to_string()),
    )]));
    app.live_view_updated(air_net_world(2, 2), None, None, Some(custom));
    assert_eq!(app.screen, ScreenKind::Dispenser, "the re-open is in place");
    assert_eq!(
        app.live_dispenser_title.as_deref(),
        Some("Rename Machine"),
        "the fresh window's own name re-speaks"
    );

    // the wire never spoke a decodable name (the empty bytes the
    // empty-root compound carries): the None the default lang row
    // banks — the live canvas's honest fallback
    let mut silent = dropper.clone();
    silent.state_id = 11;
    silent.title_nbt = vec![0x0A, 0x00];
    app.live_view_updated(air_net_world(2, 3), None, None, Some(silent));
    assert_eq!(
        app.live_dispenser_title, None,
        "the undecodable bytes the fallback banks"
    );

    // the store DROPPED (the server's Container Close): the title
    // dies with the window — no stale name rides a later screen
    app.live_view_updated(air_net_world(2, 4), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the close returns to the world"
    );
    assert!(
        !app.dispenser_is_live,
        "the live flag dropped with the window"
    );
    assert_eq!(
        app.live_dispenser_title, None,
        "the title died with the window"
    );

    // and the lane re-arms: the next window's own name rides in
    app.live_view_updated(air_net_world(2, 5), None, None, Some(dropper));
    assert!(app.dispenser_is_live, "the live lane re-took the screen");
    assert_eq!(
        app.live_dispenser_title.as_deref(),
        Some("Dropper"),
        "the fresh window's title rode in again"
    );
}

#[test]
fn the_live_chest_furnace_and_table_windows_wear_the_wires_title() {
    use crate::world_view::LiveContainer;
    use mc_protocol::nbt::{write_network_nbt, Nbt};
    // the wire's own encoder (the decode walk's own mirror)
    let enc = |v: &Nbt| -> Vec<u8> {
        let mut b = Vec::new();
        write_network_nbt(v, &mut b).unwrap();
        b
    };
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(3, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World, "the world owns the entry");

    // the CHEST window (the generic_9x3 menu the 63-slot store
    // speaks): the wire's own text row the decode answers — the
    // anvil rename the live canvas wears (the container.chest
    // default the None banks)
    let chest = LiveContainer {
        id: 3,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: vec![(0i32, 0i32); 63],
        carried: (0, 0),
        state_id: 4,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: enc(&Nbt::Compound(vec![(
            "text".to_string(),
            Nbt::String("Storage Room".to_string()),
        )])),
    };
    app.live_view_updated(air_net_world(3, 1), None, None, Some(chest.clone()));
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the window shows over the world"
    );
    assert!(app.chest_is_live, "the live lane owns the screen now");
    assert_eq!(
        app.live_chest_title.as_deref(),
        Some("Storage Room"),
        "the wire's own text row the canvas banks"
    );
    // the server CLOSES the chest before the next window (the one
    // window at a time the wire's own flow speaks — the drop the
    // world returns)
    app.live_view_updated(air_net_world(3, 2), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the drop returns to the world"
    );

    // the FURNACE window (the menu type 14 the same lane routes):
    // the translatable the lang row answers
    let furnace = LiveContainer {
        id: 4,
        menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
        slots: vec![(0i32, 0i32); 39],
        carried: (0, 0),
        state_id: 6,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: enc(&Nbt::Compound(vec![(
            "translate".to_string(),
            Nbt::String("container.furnace".to_string()),
        )])),
    };
    app.live_view_updated(air_net_world(3, 3), None, None, Some(furnace.clone()));
    assert_eq!(
        app.screen,
        ScreenKind::Furnace,
        "the window shows over the world"
    );
    assert!(app.furnace_is_live, "the live lane owns the screen now");
    assert_eq!(
        app.live_furnace_title.as_deref(),
        Some("Furnace"),
        "the kind's own lang row the decode answers"
    );
    app.live_view_updated(air_net_world(3, 4), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the drop returns to the world"
    );

    // the TABLE window (the crafting menu the 46-slot store
    // speaks): the translatable the Crafting row answers
    let table = LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
        slots: vec![(0i32, 0i32); 46],
        carried: (0, 0),
        state_id: 8,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: enc(&Nbt::Compound(vec![(
            "translate".to_string(),
            Nbt::String("container.crafting".to_string()),
        )])),
    };
    app.live_view_updated(air_net_world(3, 5), None, None, Some(table));
    assert_eq!(
        app.screen,
        ScreenKind::Crafting,
        "the window shows over the world"
    );
    assert!(app.table_is_live, "the live lane owns the screen now");
    assert_eq!(
        app.live_table_title.as_deref(),
        Some("Crafting"),
        "the table's own lang row the decode answers"
    );
    app.live_view_updated(air_net_world(3, 6), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the drop returns to the world"
    );

    // a NEW window speaks a CUSTOM name (the anvil rename the
    // text row carries): the fresh decode replaces the default —
    // and the silent bytes the None fallback banks
    let mut custom = chest.clone();
    custom.state_id = 10;
    custom.title_nbt = enc(&Nbt::Compound(vec![(
        "text".to_string(),
        Nbt::String("Loot Cellar".to_string()),
    )]));
    app.live_view_updated(air_net_world(3, 7), None, None, Some(custom));
    assert_eq!(app.screen, ScreenKind::Chest, "the re-open is in place");
    assert_eq!(
        app.live_chest_title.as_deref(),
        Some("Loot Cellar"),
        "the fresh window's own name re-speaks"
    );
    let mut silent = chest;
    silent.state_id = 12;
    silent.title_nbt = vec![0x0A, 0x00];
    app.live_view_updated(air_net_world(3, 8), None, None, Some(silent));
    assert_eq!(
        app.live_chest_title, None,
        "the undecodable bytes the fallback banks"
    );
    app.live_view_updated(air_net_world(3, 9), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the drop returns to the world"
    );

    // the store DROPPED (the server's Container Close): every
    // title dies with its window — no stale name rides a later
    // screen (the chest's drop the furnace's and the table's own
    // resets the same arm serves)
    app.live_view_updated(air_net_world(3, 9), None, None, Some(furnace));
    assert_eq!(
        app.screen,
        ScreenKind::Furnace,
        "the furnace window re-armed"
    );
    assert_eq!(app.live_furnace_title.as_deref(), Some("Furnace"));
    app.live_view_updated(air_net_world(3, 10), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the close returns to the world"
    );
    assert!(
        !app.furnace_is_live,
        "the live flag dropped with the window"
    );
    assert_eq!(
        app.live_chest_title, None,
        "the chest's title died with its window"
    );
    assert_eq!(
        app.live_furnace_title, None,
        "the furnace's title died with its window"
    );
    assert_eq!(
        app.live_table_title, None,
        "the table's title died with its window"
    );

    // and the lane re-arms: the next window's own name rides in
    let rearm = LiveContainer {
        id: 3,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: vec![(0i32, 0i32); 63],
        carried: (0, 0),
        state_id: 14,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: enc(&Nbt::Compound(vec![(
            "translate".to_string(),
            Nbt::String("container.chest".to_string()),
        )])),
    };
    app.live_view_updated(air_net_world(3, 11), None, None, Some(rearm));
    assert!(app.chest_is_live, "the live lane re-took the screen");
    assert_eq!(
        app.live_chest_title.as_deref(),
        Some("Chest"),
        "the fresh window's title rode in again"
    );
}

#[test]
fn the_player_s_own_close_kills_the_live_windows_title() {
    use crate::world_view::LiveContainer;
    use mc_protocol::nbt::{write_network_nbt, Nbt};
    // the wire's own encoder (the decode walk's own mirror)
    let enc = |v: &Nbt| -> Vec<u8> {
        let mut b = Vec::new();
        write_network_nbt(v, &mut b).unwrap();
        b
    };
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(4, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World, "the world owns the entry");

    // the CHEST window the player opens: the OWN close (Escape/E
    // the close walk speaks) kills the title with the window —
    // the offline stand-in's canvas the default row banks
    let chest = LiveContainer {
        id: 2,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: vec![(0i32, 0i32); 63],
        carried: (0, 0),
        state_id: 3,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: enc(&Nbt::Compound(vec![(
            "text".to_string(),
            Nbt::String("Named Store".to_string()),
        )])),
    };
    app.live_view_updated(air_net_world(4, 1), None, None, Some(chest.clone()));
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the window shows over the world"
    );
    assert_eq!(app.live_chest_title.as_deref(), Some("Named Store"));
    app.close_chest_container(ScreenKind::World);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the own close returns to the world"
    );
    assert!(!app.chest_is_live, "the live flag dropped");
    assert_eq!(app.live_chest_title, None, "the title died with the window");

    // the FURNACE window the same law wears (the kind's own close
    // walk the chest's shape mirrors)
    let furnace = LiveContainer {
        id: 3,
        menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
        slots: vec![(0i32, 0i32); 39],
        carried: (0, 0),
        state_id: 5,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: enc(&Nbt::Compound(vec![(
            "text".to_string(),
            Nbt::String("Smeltery".to_string()),
        )])),
    };
    app.live_view_updated(air_net_world(4, 2), None, None, Some(furnace));
    assert_eq!(
        app.screen,
        ScreenKind::Furnace,
        "the window shows over the world"
    );
    assert_eq!(app.live_furnace_title.as_deref(), Some("Smeltery"));
    app.close_furnace(ScreenKind::World);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the own close returns to the world"
    );
    assert!(!app.furnace_is_live, "the live flag dropped");
    assert_eq!(
        app.live_furnace_title, None,
        "the title died with the window"
    );

    // the TABLE window the same law wears (the crafting menu's
    // own close the chest's shape mirrors)
    let table = LiveContainer {
        id: 4,
        menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
        slots: vec![(0i32, 0i32); 46],
        carried: (0, 0),
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: enc(&Nbt::Compound(vec![(
            "text".to_string(),
            Nbt::String("Workbench".to_string()),
        )])),
    };
    app.live_view_updated(air_net_world(4, 3), None, None, Some(table));
    assert_eq!(
        app.screen,
        ScreenKind::Crafting,
        "the window shows over the world"
    );
    assert_eq!(app.live_table_title.as_deref(), Some("Workbench"));
    app.close_crafting_table(ScreenKind::World);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the own close returns to the world"
    );
    assert!(!app.table_is_live, "the live flag dropped");
    assert_eq!(app.live_table_title, None, "the title died with the window");

    // the offline stand-in the dead field never reads: the fresh
    // offline chest the default row banks (the golden's own law)
    app.open_chest();
    assert!(
        !app.chest_is_live,
        "the offline stand-in the live lane never owns"
    );
    assert_eq!(
        app.live_chest_title, None,
        "no stale name rides the offline canvas"
    );
}

#[test]
fn the_live_container_drives_the_chest_screen() {
    use crate::world_view::LiveContainer;
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World);

    // the server opens a chest (Open Screen → the store, the Set
    // Content fills it — here the store's PROJECTION arrives): the
    // screen SHOWS over the world, the slots ride the menu's own
    // numbering into the grid and the mirror
    let chest = LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: {
            let mut s = vec![(0i32, 0i32); 63];
            s[4] = (926, 12); // the container's slot 4: diamond x12
            s[27] = (921, 5); // the mirror's first main row slot: apple x5
            s[54] = (932, 64); // the mirror's hotbar head: iron_ingot x64
            s
        },
        carried: (0, 0),
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest.clone()));
    assert_eq!(
        app.screen,
        ScreenKind::Chest,
        "the window shows over the world"
    );
    assert!(app.chest_is_live, "the live lane owns the screen now");
    assert_eq!(app.chest_slots.len(), 27);
    assert_eq!(app.chest_slots[4], (926, 12), "the container grid rode in");
    assert_eq!(app.chest_mirror.len(), 36);
    assert_eq!(
        app.chest_mirror[0],
        (921, 5),
        "the mirror's main row slot 0 = menu slot 27"
    );
    assert_eq!(
        app.chest_mirror[27],
        (932, 64),
        "the mirror's hotbar head = menu slot 54"
    );

    // a NEWER snapshot of the same window updates IN PLACE (the
    // Set Content packet's replacement — no re-open flicker)
    let mut chest2 = chest.clone();
    chest2.slots[4] = (0, 0); // the diamond was taken out
    chest2.slots[9] = (981, 3);
    app.live_view_updated(air_net_world(2, 2), None, None, Some(chest2));
    assert_eq!(app.screen, ScreenKind::Chest, "the update is in place");
    assert_eq!(app.chest_slots[4], (0, 0), "the taken stack emptied");
    assert_eq!(app.chest_slots[9], (981, 3), "the new stack landed");

    // a NON-chest menu is the honest drop: the store keeps it, the
    // screen machinery does not move (its screen is its own round)
    let hopper = LiveContainer {
        id: 6,
        menu_type_id: 15,
        slots: vec![],
        carried: (0, 0),
        state_id: 0,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 3), None, None, Some(hopper));
    assert_eq!(app.screen, ScreenKind::Chest, "the wired screen stays");
    assert_eq!(app.chest_slots[9], (981, 3), "the hopper touched nothing");

    // the store DROPPED (the server's Container Close): the live
    // chest returns to the world — the vanilla closeContainer's own
    app.live_view_updated(air_net_world(2, 4), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the close returns to the world"
    );
    assert!(!app.chest_is_live, "the live flag dropped with the window");

    // a container-less update with NO chest up: nothing happens
    app.live_view_updated(air_net_world(2, 5), None, None, None);
    assert_eq!(app.screen, ScreenKind::World);

    // the OFFLINE stand-in is never the live lane's to close: the
    // deterministic chest stays up (the app went PROCEDURAL with
    // the offline scene — the world guard drops the live updates)
    app.open_chest();
    assert_eq!(app.screen, ScreenKind::Chest);
    assert!(!app.chest_is_live);
    assert!(
        app.chest_mirror.is_empty(),
        "the offline mirror is chrome-only"
    );
    app.live_view_updated(air_net_world(2, 6), None, None, None);
    assert_eq!(app.screen, ScreenKind::Chest, "the offline chest survives");
    // and the lane re-arms on a NEW join: the network world back,
    // the live window opens over it (the fresh menu's slots — the
    // vanilla open is a fresh screen)
    app.net_world = Some(air_net_world(2, 8));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World);
    app.live_view_updated(air_net_world(2, 9), None, None, Some(chest));
    assert_eq!(app.screen, ScreenKind::Chest);
    assert!(app.chest_is_live, "the live lane took the screen over");
    assert_eq!(
        app.chest_slots[4],
        (926, 12),
        "the live slots replaced the loadout"
    );
    assert_eq!(app.chest_mirror[0], (921, 5), "the live mirror filled in");
}

/// THE WIRE DISPENSER'S OWN WINDOW (the menu type id 6 the
/// OpenScreen word carries — the GENERIC_3x3 the clinit order
/// pins): the store's projection opens the 3x3 canvas over the
/// world (the window-1 belief the 45-slot shape feeds — the nine
/// + the standard rows the menu's own numbering keeps), a NEWER
/// snapshot updates IN PLACE, the foreign menu stays the honest
/// drop, the store's drop returns to the world, and the offline
/// stand-in is never the live lane's to close.
#[test]
fn the_wire_dispenser_menu_drives_the_3x3_canvas() {
    use crate::world_view::LiveContainer;
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World);

    // the server opens a dispenser (Open Screen → the store, the
    // Set Content fills it — here the store's PROJECTION arrives):
    // the screen SHOWS over the world, the slots ride the menu's
    // own numbering into the nine and the mirror
    let disp = LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_GENERIC_3X3,
        slots: {
            let mut s = vec![(0i32, 0i32); 45];
            s[4] = (926, 12); // the grid's center: diamond x12
            s[9] = (921, 5); // the mirror's first main row slot: apple x5
            s[36] = (932, 64); // the mirror's hotbar head: iron_ingot x64
            s
        },
        carried: (0, 0),
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(disp.clone()));
    assert_eq!(
        app.screen,
        ScreenKind::Dispenser,
        "the window shows over the world"
    );
    assert!(app.dispenser_is_live, "the live lane owns the screen now");
    assert_eq!(
        app.dispenser_live_window,
        Some((5, 7)),
        "the window identity rides"
    );
    assert_eq!(
        app.dispenser_open_cell, None,
        "the server's window no cell key banks"
    );
    assert_eq!(app.dispenser_slots.len(), 9);
    assert_eq!(app.dispenser_slots[4], (926, 12), "the grid slot rode in");
    assert_eq!(app.dispenser_mirror.len(), 36);
    assert_eq!(
        app.dispenser_mirror[0],
        (921, 5),
        "the mirror's main row slot 0 = menu slot 9"
    );
    assert_eq!(
        app.dispenser_mirror[27],
        (932, 64),
        "the mirror's hotbar head = menu slot 36"
    );

    // a NEWER snapshot of the same window updates IN PLACE (the
    // Set Content packet's replacement — no re-open flicker)
    let mut disp2 = disp.clone();
    disp2.slots[4] = (0, 0); // the diamond was taken out
    disp2.slots[5] = (981, 3);
    app.live_view_updated(air_net_world(2, 2), None, None, Some(disp2));
    assert_eq!(app.screen, ScreenKind::Dispenser, "the update is in place");
    assert_eq!(app.dispenser_slots[4], (0, 0), "the taken stack emptied");
    assert_eq!(app.dispenser_slots[5], (981, 3), "the new stack landed");

    // a NON-3x3 menu is the honest drop: the store keeps it, the
    // screen machinery does not move
    let hopper = LiveContainer {
        id: 6,
        menu_type_id: 15,
        slots: vec![],
        carried: (0, 0),
        state_id: 0,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 3), None, None, Some(hopper));
    assert_eq!(app.screen, ScreenKind::Dispenser, "the wired screen stays");
    assert_eq!(
        app.dispenser_slots[5],
        (981, 3),
        "the hopper touched nothing"
    );

    // the store DROPPED (the server's Container Close): the live
    // dispenser returns to the world — the vanilla closeContainer's own
    app.live_view_updated(air_net_world(2, 4), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the close returns to the world"
    );
    assert!(
        !app.dispenser_is_live,
        "the live flag dropped with the window"
    );

    // a container-less update with NO dispenser up: nothing happens
    app.live_view_updated(air_net_world(2, 5), None, None, None);
    assert_eq!(app.screen, ScreenKind::World);

    // the OFFLINE stand-in is never the live lane's to close: the
    // deterministic canvas stays up (the app went PROCEDURAL with
    // the offline scene — the world guard drops the live updates)
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::DISPENSER);
    app.open_dispenser_for(cell);
    assert_eq!(app.screen, ScreenKind::Dispenser);
    assert!(!app.dispenser_is_live);
    app.live_view_updated(air_net_world(2, 6), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::Dispenser,
        "the offline canvas survives"
    );
    // and the lane re-arms on a NEW join: the network world back,
    // the live window opens over it (the fresh menu's slots — the
    // vanilla open is a fresh screen)
    app.net_world = Some(air_net_world(2, 8));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World);
    app.live_view_updated(air_net_world(2, 9), None, None, Some(disp));
    assert_eq!(app.screen, ScreenKind::Dispenser);
    assert!(app.dispenser_is_live, "the live lane took the screen over");
    assert_eq!(
        app.dispenser_slots[4],
        (926, 12),
        "the live slots replaced the loadout"
    );
    assert_eq!(
        app.dispenser_mirror[0],
        (921, 5),
        "the live mirror filled in"
    );
}

/// THE WIRE DISPENSER'S OWN CLICK ARC (the window-5 identity the
/// ServerboundContainerClickPacket speaks — the chest's send lane
/// the GENERIC_3x3 menu's own numbering rides): the grid seat's
/// pickup carries the window's id + revision, the place-back the
/// same PICKUP input speaks, the shift quick-move the QuickMove
/// input's changed map wears (the moveItemStackTo(9, 45, true)
/// reverse scan the DispenserMenu's own quickMoveStack banks),
/// and the close echoes the Container Close packet.
#[test]
fn the_wire_dispenser_clicks_speak_the_window() {
    use crate::world_view::{ContainerCommand, ContainerInput, LiveContainer};
    let diamond = mc_registry::item_id_by_name("diamond").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let disp = LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_GENERIC_3X3,
        slots: {
            let mut s = vec![(0i32, 0i32); 45];
            s[4] = (diamond, 12); // the grid's center: diamond x12
            s
        },
        carried: (0, 0),
        state_id: 3,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(disp));
    // THE RENDER'S OWN LAYOUT (the hit boxes the render pre-lays —
    // the test the helper's own shape wears)
    let panel = inventory_panel_origin(640, 360);
    app.dispenser_panel = panel;
    app.dispenser_slot_rects = dispenser_rects_for(panel);
    // THE GRID SEAT'S PICKUP (menu slot 4 = the 3x3's center): the
    // whole stack rides the cursor, the packet the window's own
    // identity names
    let (gx, gy) = dispenser_grid_slot(panel, 4);
    app.on_press(gx + 1 + 8, gy + 1 + 8);
    match click_rx.try_recv().expect("the grid click rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5, "the window's own id");
            assert_eq!(c.state_id, 3, "the revision rides");
            assert_eq!(c.slot, 4, "the menu's own numbering");
            assert_eq!(c.input, ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(4, (0, 0))], "the taken slot emptied");
            assert_eq!(c.carried, (diamond, 12), "the cursor carries the stack");
        }
        _ => panic!("the click command's own shape"),
    }
    assert_eq!(
        app.dispenser_carried,
        (diamond, 12),
        "the belief's cursor follows"
    );
    // THE PLACE-BACK (the carried press the DRAG ARMS — the
    // mouseClicked pcs 316-378 law the chest's own live lane
    // wears: NO slot click, NO packet — the empty-painted release
    // the DEFERRED PLACE speaks: the release position the PICKUP
    // table serves, the vanilla doClick's symmetric answer)
    app.on_mouse_move(gx + 1 + 8, gy + 1 + 8);
    app.on_press(gx + 1 + 8, gy + 1 + 8);
    app.on_left_release();
    match click_rx.try_recv().expect("the place click rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.changed, vec![(4, (diamond, 12))], "the stack landed back");
            assert_eq!(c.carried, (0, 0), "the cursor emptied");
        }
        _ => panic!("the click command's own shape"),
    }
    // THE SHIFT QUICK-MOVE (the whole stack the OTHER section
    // takes — the 45-slot belief the predictor wears, the
    // changed map the deltas name, the cursor untouched)
    app.shift_down = true;
    app.on_press(gx + 1 + 8, gy + 1 + 8);
    app.shift_down = false;
    match click_rx.try_recv().expect("the quick-move rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.slot, 4);
            assert_eq!(c.input, ContainerInput::QuickMove);
            assert_eq!(c.changed[0], (4, (0, 0)), "the source emptied");
            assert_eq!(
                c.changed.last().copied().unwrap().0,
                44,
                "the hotbar tail the rev scan takes"
            );
            assert_eq!(c.carried, (0, 0), "the cursor the quick-move never rides");
        }
        _ => panic!("the click command's own shape"),
    }
    assert_eq!(app.dispenser_slots[4], (0, 0), "the belief's grid emptied");
    // THE CLOSE (the E/Escape lane the live window's own echo
    // wears): the Container Close packet the window's id names,
    // the belief resets with the window
    app.on_escape();
    match click_rx.try_recv().expect("the close rode the lane") {
        ContainerCommand::Close { window_id } => {
            assert_eq!(window_id, 5, "the window's own id");
        }
        other => panic!("expected a container close, got {other:?}"),
    }
    assert!(!app.dispenser_is_live, "the live flag the close drops");
    assert_eq!(app.screen, ScreenKind::World, "the close the world wears");
}

/// THE DISPENSER'S OWN RIGHT PRESS (the dev-150 candidate the
/// record "already serves" the pin banks — the carried-EMPTY
/// press the split-pickup dispatch keeps: the ceiling half the
/// PICKUP's own button-1 walk serves, the held press the type-1
/// drag arms — NO place, NO packet —, the one-each the release
/// serves, the masked START/ADD/END bytes the wire wears, the
/// release the arm clears).
#[test]
fn the_dispenser_right_press_the_split_and_the_one_each_wear() {
    use crate::world_view::{ContainerCommand, ContainerInput, LiveContainer};
    let diamond = mc_registry::item_id_by_name("diamond").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let disp = LiveContainer {
        id: 6,
        menu_type_id: crate::world_model::MENU_TYPE_GENERIC_3X3,
        slots: {
            let mut s = vec![(0i32, 0i32); 45];
            s[4] = (diamond, 5); // the odd split's own number
            s
        },
        carried: (0, 0),
        state_id: 4,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(disp));
    assert_eq!(app.screen, ScreenKind::Dispenser);
    let panel = inventory_panel_origin(640, 360);
    app.dispenser_panel = panel;
    app.dispenser_slot_rects = dispenser_rects_for(panel);
    // THE CARRIED-EMPTY PRESS THE SPLIT-PICKUP KEEPS (the ceiling
    // half rides — 5 -> 3 taken, 2 left — the PICKUP button-1 byte
    // the wire wears)
    let (gx, gy) = dispenser_grid_slot(panel, 4);
    app.on_right_press(gx + 9, gy + 9);
    assert_eq!(
        app.dispenser_slots[4],
        (diamond, 2),
        "the odd split left two"
    );
    assert_eq!(
        app.dispenser_carried,
        (diamond, 3),
        "the ceiling half rides"
    );
    match click_rx.try_recv().expect("the split's click") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 6, "the window's own id");
            assert_eq!(c.state_id, 4, "the revision rides");
            assert_eq!(c.slot, 4, "the menu's own numbering");
            assert_eq!(c.button, 1, "the right press's own byte");
            assert_eq!(c.input, ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(4, (diamond, 2))]);
            assert_eq!(c.carried, (diamond, 3));
        }
        other => panic!("expected a click, got {other:?}"),
    }
    // THE HELD PRESS THE TYPE-1 DRAG ARMS (no place, no packet —
    // the arm the silent start keeps, the one-each the release
    // serves)
    app.dispenser_carried = (dirt, 5);
    let (ex, ey) = dispenser_grid_slot(panel, 5);
    app.on_right_press(ex + 9, ey + 9);
    assert!(app.qc_armed, "the right press the drag arms");
    assert_eq!(app.qc_type, 1, "the right button's own type");
    assert_eq!(app.qc_button, 1, "the right button remembered");
    assert!(app.qc_slots.is_empty(), "the press paints nothing");
    assert_eq!(app.dispenser_slots[5], (0, 0), "the press NO place speaks");
    assert!(click_rx.try_recv().is_err(), "the arm NO packet speaks");
    app.on_mouse_move(ex + 9, ey + 9);
    let (fx, fy) = dispenser_grid_slot(panel, 6);
    app.on_mouse_move(fx + 9, fy + 9);
    assert_eq!(app.qc_slots, vec![5, 6], "the two seats the motion paints");
    app.on_right_release();
    assert_eq!(app.dispenser_slots[5], (dirt, 1), "the one-each's own one");
    assert_eq!(app.dispenser_slots[6], (dirt, 1), "the one-each's own one");
    assert_eq!(app.dispenser_carried, (dirt, 3), "the two the cursor paid");
    assert!(!app.qc_armed, "the release the arm clears");
    // THE MASKED BYTES THE WIRE WEARS (the START mask(0,1) byte 4,
    // the ADDs mask(1,1) byte 5, the END mask(2,1) byte 6 — the
    // changed map the one-each deltas names)
    match click_rx.try_recv().expect("the START rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 6);
            assert_eq!(c.state_id, 4);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 4, "the START's own masked byte");
            assert_eq!(c.input, ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    match click_rx.try_recv().expect("the first ADD rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 6);
            assert_eq!(c.slot, 5);
            assert_eq!(c.button, 5, "the ADD's own masked byte");
            assert_eq!(c.input, ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    match click_rx.try_recv().expect("the second ADD rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 6);
            assert_eq!(c.slot, 6);
            assert_eq!(c.button, 5, "the ADD's own masked byte");
            assert_eq!(c.input, ContainerInput::QuickCraft);
            assert_eq!(c.changed, Vec::new());
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    match click_rx.try_recv().expect("the END rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 6);
            assert_eq!(c.slot, -999);
            assert_eq!(c.button, 6, "the END's own masked byte");
            assert_eq!(c.input, ContainerInput::QuickCraft);
            assert_eq!(
                c.changed,
                vec![(5, (dirt, 1)), (6, (dirt, 1))],
                "the one-each deltas"
            );
            assert_eq!(c.carried, (dirt, 3), "the remainder the packet keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE WIRE TABLE'S OWN WINDOW (the menu type id 12 the OpenScreen
/// word carries — the autopush-329 round): the store's projection
/// opens the Crafting screen over the world (the window-1 belief
/// the 46-slot shape feeds), a NEWER snapshot updates IN PLACE,
/// the foreign menu stays the honest drop, the store's drop
/// returns to the world, and the offline stand-in is never the
/// live lane's to close.
#[test]
fn the_wire_crafting_menu_drives_the_table_screen() {
    use crate::world_view::LiveContainer;
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World);

    // the server opens a crafting table (Open Screen menu 12 →
    // the store, the Set Content fills it — here the store's
    // PROJECTION arrives): the screen SHOWS over the world, the
    // slots ride the CraftingMenu's own numbering into the belief
    let table = LiveContainer {
        id: 9,
        menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
        slots: {
            let mut s = vec![(0i32, 0i32); 46];
            s[0] = (925, 4); // the result slot: stick x4
            s[1] = (926, 7); // the grid's cell 1: diamond x7 (demo cargo)
            s[4] = (926, 7); // the grid's cell 4 (row 1)
            s[10] = (921, 5); // the main grid's head: apple x5
            s[37] = (932, 64); // the hotbar row's head: iron_ingot x64
            s
        },
        carried: (0, 0),
        state_id: 3,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(table.clone()));
    assert_eq!(
        app.screen,
        ScreenKind::Crafting,
        "the window shows over the world"
    );
    assert!(app.table_is_live, "the live lane owns the screen now");
    assert_eq!(app.live_table_slots[0], (925, 4), "the result slot rode in");
    assert_eq!(
        app.live_table_slots[1],
        (926, 7),
        "the grid's cell 1 rode in"
    );
    assert_eq!(
        app.live_table_slots[4],
        (926, 7),
        "the grid's cell 4 rode in"
    );
    assert_eq!(
        app.live_table_slots[10],
        (921, 5),
        "the main grid's head = menu slot 10"
    );
    assert_eq!(
        app.live_table_slots[37],
        (932, 64),
        "the hotbar's head = menu slot 37"
    );
    assert_eq!(
        app.table_live_window,
        Some((9, 3)),
        "the window's identity rode in"
    );

    // a NEWER snapshot of the same window updates IN PLACE (the
    // Set Content packet's replacement — no re-open flicker)
    let mut table2 = table.clone();
    table2.slots[0] = (0, 0); // the result was taken
    table2.slots[11] = (981, 3);
    table2.state_id = 4;
    app.live_view_updated(air_net_world(2, 2), None, None, Some(table2));
    assert_eq!(app.screen, ScreenKind::Crafting, "the update is in place");
    assert_eq!(app.live_table_slots[0], (0, 0), "the taken result emptied");
    assert_eq!(app.live_table_slots[11], (981, 3), "the new stack landed");
    assert_eq!(app.table_live_window, Some((9, 4)), "the revision followed");

    // a NON-wired menu is the honest drop: the store keeps it, the
    // screen machinery does not move
    let hopper = LiveContainer {
        id: 6,
        menu_type_id: 15,
        slots: vec![],
        carried: (0, 0),
        state_id: 0,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 3), None, None, Some(hopper));
    assert_eq!(app.screen, ScreenKind::Crafting, "the wired screen stays");
    assert_eq!(
        app.live_table_slots[11],
        (981, 3),
        "the hopper touched nothing"
    );

    // the store DROPPED (the server's Container Close): the live
    // table returns to the world — the vanilla closeContainer's own
    app.live_view_updated(air_net_world(2, 4), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the close returns to the world"
    );
    assert!(!app.table_is_live, "the live flag dropped with the window");
    assert_eq!(app.table_live_window, None, "the window's identity died");

    // the OFFLINE stand-in is never the live lane's to close: the
    // deterministic table stays up (the is_live flag the close
    // walk's wire arm reads)
    app.open_crafting_table();
    assert_eq!(app.screen, ScreenKind::Crafting);
    assert!(!app.table_is_live);
    app.live_view_updated(air_net_world(2, 6), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::Crafting,
        "the offline table survives"
    );
    // and the lane re-arms on a NEW window: the live menu opens
    // over the world again (the fresh menu's slots — the vanilla
    // open is a fresh screen)
    app.net_world = Some(air_net_world(2, 8));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World);
    app.live_view_updated(air_net_world(2, 9), None, None, Some(table));
    assert_eq!(app.screen, ScreenKind::Crafting);
    assert!(app.table_is_live, "the live lane took the screen over");
    assert_eq!(
        app.live_table_slots[0],
        (925, 4),
        "the live slots replaced the fresh grid"
    );
    assert_eq!(
        app.live_table_slots[10],
        (921, 5),
        "the store rows filled in"
    );
}

/// THE WIRE FURNACE'S OWN ARC (the menu type 14 the OpenScreen
/// word speaks — the chest's own projection shape, the Set Data
/// packet's progress words riding the store): the screen SHOWS
/// over the world, the menu's own numbering lands (input 0, fuel
/// 1, result 2, the standard rows 3..39), the progress words bank,
/// a data-word update re-banks IN PLACE (the content update never
/// wipes the words), the unknown menu drops, the close returns to
/// the world, the E-key close walks the live arm, and the pinned
/// formulas speak the jar's own pairs.
#[test]
fn the_wire_furnace_opens_the_progress_words_bank() {
    use crate::world_view::LiveContainer;
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
    let iron_ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World);

    // the server opens a furnace (Open Screen menu 14 → the store,
    // the Set Content fills it, the Set Data words bank — here the
    // store's PROJECTION arrives): the screen SHOWS over the
    // world, the menu's own numbering rides the belief
    let furnace = LiveContainer {
        id: 7,
        menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
        slots: {
            let mut s = vec![(0i32, 0i32); 39];
            s[0] = (raw_iron, 5); // the input slot
            s[1] = (coal, 3); // the fuel slot
            s[2] = (iron_ingot, 2); // the result slot
            s[3] = (921, 5); // the standard rows' head (menu 3)
            s[30] = (932, 64); // the hotbar row's head (menu 30)
            s
        },
        carried: (0, 0),
        state_id: 2,
        bundle_rows: Vec::new(),
        data_slots: vec![180, 200, 90, 200], // lit remaining/total, cook progress/total
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(furnace.clone()));
    assert_eq!(
        app.screen,
        ScreenKind::Furnace,
        "the window shows over the world"
    );
    assert!(app.furnace_is_live, "the live lane owns the screen now");
    assert_eq!(app.furnace_slots[0], (raw_iron, 5), "the input rode in");
    assert_eq!(app.furnace_slots[1], (coal, 3), "the fuel rode in");
    assert_eq!(app.furnace_slots[2], (iron_ingot, 2), "the result rode in");
    assert_eq!(
        app.furnace_slots[3],
        (921, 5),
        "the standard rows' head = menu slot 3"
    );
    assert_eq!(
        app.furnace_slots[30],
        (932, 64),
        "the hotbar's head = menu slot 30"
    );
    assert_eq!(
        app.furnace_live_window,
        Some((7, 2)),
        "the window's identity rode in"
    );
    assert_eq!(
        app.furnace_data,
        [180, 200, 90, 200],
        "the progress words rode in"
    );
    // THE PINNED FORMULAS (javap AbstractFurnaceScreen): the flame
    // ceil(180/200 * 13.0) + 1 = ceil(11.7) + 1 = 13, the arrow
    // ceil(90/200 * 24.0) = ceil(10.8) = 11
    assert_eq!(furnace_flame_height(&app.furnace_data), 13);
    assert_eq!(furnace_arrow_width(&app.furnace_data), 11);

    // a Set Data word re-banks IN PLACE through the store's
    // projection (the cooking progresses; the slots never wipe)
    let mut cooked = furnace.clone();
    cooked.data_slots = vec![170, 200, 91, 200];
    cooked.state_id = 3;
    app.live_view_updated(air_net_world(2, 2), None, None, Some(cooked));
    assert_eq!(app.screen, ScreenKind::Furnace, "the update is in place");
    assert_eq!(app.furnace_data, [170, 200, 91, 200], "the words re-banked");
    assert_eq!(app.furnace_slots[0], (raw_iron, 5), "the slots never wiped");
    assert_eq!(
        app.furnace_live_window,
        Some((7, 3)),
        "the revision followed"
    );

    // a NON-wired menu is the honest drop: the store keeps it, the
    // screen machinery does not move
    let hopper = LiveContainer {
        id: 6,
        menu_type_id: 15,
        slots: vec![],
        carried: (0, 0),
        state_id: 0,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 3), None, None, Some(hopper));
    assert_eq!(app.screen, ScreenKind::Furnace, "the wired screen stays");
    assert_eq!(
        app.furnace_data,
        [170, 200, 91, 200],
        "the hopper touched nothing"
    );

    // the store DROPPED (the server's Container Close): the live
    // furnace returns to the world — the vanilla closeContainer's
    // own
    app.live_view_updated(air_net_world(2, 4), None, None, None);
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the close returns to the world"
    );
    assert!(
        !app.furnace_is_live,
        "the live flag dropped with the window"
    );
    assert_eq!(app.furnace_live_window, None, "the window's identity died");

    // the lane re-arms on a NEW window and the E-KEY close walks
    // the live arm (the inventory-key lane the table's arm shares)
    app.net_world = Some(air_net_world(2, 6));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    assert_eq!(app.screen, ScreenKind::World);
    app.live_view_updated(air_net_world(2, 7), None, None, Some(furnace));
    assert_eq!(app.screen, ScreenKind::Furnace);
    assert!(app.furnace_is_live, "the live lane took the screen over");
    app.on_inventory_key();
    assert_eq!(app.screen, ScreenKind::World, "the E close returns");
    assert!(!app.furnace_is_live);
    assert_eq!(
        app.live_furnace_carried,
        (0, 0),
        "the belief reset with the window"
    );
}

/// THE WIRE FURNACE'S OWN SHIFT-CLICK (the quick-move lane the
/// dev-24 round deferred now the chest's own convention wears:
/// the predictor's multi-slot deltas the command rides, the local
/// belief the same walk mutates — javap 26.2
/// AbstractFurnaceMenu.quickMoveStack).
#[test]
fn the_wire_furnace_the_shift_click_the_deltas_wear() {
    use crate::world_view::LiveContainer;
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
    let iron_ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let furnace = LiveContainer {
        id: 7,
        menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
        slots: {
            let mut s = vec![(0i32, 0i32); 39];
            s[0] = (raw_iron, 5); // the input slot
            s[1] = (coal, 3); // the fuel slot
            s[2] = (iron_ingot, 2); // the result slot
            s[30] = (raw_iron, 5); // the hotbar's head carries the ore
            s
        },
        carried: (0, 0),
        state_id: 2,
        bundle_rows: Vec::new(),
        data_slots: vec![180, 200, 90, 200],
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(furnace));
    assert_eq!(app.screen, ScreenKind::Furnace);
    // THE SHIFT-CLICK ON THE HOTBAR'S ORE (menu 30 — the smeltable
    // arm the routing speaks: the INPUT slot the merge takes)
    app.furnace_panel = (0, 0);
    app.furnace_slot_rects = vec![(100, 100, 30)];
    app.on_shift_down(true);
    app.on_press(108, 108);
    assert_eq!(
        app.furnace_slots[0],
        (raw_iron, 10),
        "the merge the input took"
    );
    assert_eq!(app.furnace_slots[30], (0, 0), "the source emptied");
    assert_eq!(app.live_furnace_carried, (0, 0), "the cursor not the lane");
    match click_rx.try_recv().expect("the shift click rode the lane") {
        crate::world_view::ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 7, "the window's own id");
            assert_eq!(c.state_id, 2, "the revision rides");
            assert_eq!(c.slot, 30, "the hotbar's own numbering");
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickMove);
            assert_eq!(
                c.changed,
                vec![(30, (0, 0)), (0, (raw_iron, 10))],
                "the multi-slot belief rides the command"
            );
            assert_eq!(c.carried, (0, 0), "the carried lane untouched");
        }
        other => panic!("expected a click, got {other:?}"),
    }
    // THE SHIFT-CLICK ON THE RESULT (menu 2 — the reverse walk the
    // inventory takes: the main rows' first empty the merge
    // misses, the forward range the reverse flag never names...
    // the hotbar-first order the reverse walk speaks — the empty
    // slot 38 the ONE-empty pass takes)
    app.furnace_slot_rects = vec![(100, 100, 2)];
    app.on_press(108, 108);
    assert_eq!(app.furnace_slots[2], (0, 0), "the result emptied");
    assert_eq!(
        app.furnace_slots[38],
        (iron_ingot, 2),
        "the hotbar's tail took it"
    );
    match click_rx.try_recv().expect("the result shift rode") {
        crate::world_view::ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 2);
            assert_eq!(c.input, crate::world_view::ContainerInput::QuickMove);
            assert_eq!(
                c.changed,
                vec![(2, (0, 0)), (38, (iron_ingot, 2))],
                "the reverse walk the deltas speak"
            );
        }
        other => panic!("expected a click, got {other:?}"),
    }
    app.on_shift_down(false);
}

/// THE FURNACE'S OWN Q, THE WIRE (the doClick THROW arm over the
/// 39-slot belief — the hotbar rows the menu 30..39 range speaks):
/// the PLAIN Q the fuel seat's ONE off wears, the Ctrl the WHOLE
/// count (the HUD row's belief the every-mutation law syncs), the
/// RESULT seat the PLAIN TAKE answers (the mayPickup the base
/// Slot's unconditional TRUE keeps — the input's decrement the
/// SMELTING TICK the server keeps, no recompute the furnace walk
/// ever fires), the input's feed the Q never disturbs.
#[test]
fn the_wire_furnace_drop_key_the_fuel_the_result_and_the_hud_wear() {
    use crate::world_view::{ContainerCommand, ContainerInput, LiveContainer};
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
    let iron_ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let furnace = LiveContainer {
        id: 7,
        menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
        slots: {
            let mut s = vec![(0i32, 0i32); 39];
            s[0] = (raw_iron, 5); // the input seat — the smelting's own feed
            s[1] = (coal, 7); // the fuel seat
            s[2] = (iron_ingot, 2); // the result seat — the smelting's own yield
            s[31] = (dirt, 32); // the hotbar row 1 — menu 31
            s
        },
        carried: (0, 0),
        state_id: 2,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(furnace));
    assert_eq!(app.screen, ScreenKind::Furnace, "the window shows");
    // THE PLAIN Q ON THE FUEL SEAT (menu 1): the ONE off
    app.furnace_slot_rects = vec![(100, 100, 1)];
    app.mouse_pos = Some((108, 108));
    app.on_furnace_drop_key(false);
    assert_eq!(app.furnace_slots[1], (coal, 6), "the one off rode");
    assert_eq!(
        app.furnace_slots[0],
        (raw_iron, 5),
        "the input's feed the Q never disturbs"
    );
    match click_rx.try_recv().expect("the packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 7, "the window's own id");
            assert_eq!(c.state_id, 2, "the revision rides");
            assert_eq!(c.slot, 1, "the fuel seat's own menu id");
            assert_eq!(c.button, 0, "the plain Q the byte keys");
            assert_eq!(c.input, ContainerInput::Throw);
            assert_eq!(c.changed, vec![(1, (coal, 6))], "the mutation named");
            assert_eq!(c.carried, (0, 0), "the cursor pair rides unchanged");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE CTRL+Q ON THE HOTBAR ROW 1 (menu 31): the whole stack
    // gone, the HUD row's belief synced
    app.furnace_slot_rects = vec![(100, 100, 31)];
    app.mouse_pos = Some((108, 108));
    app.on_furnace_drop_key(true);
    assert_eq!(app.furnace_slots[31], (0, 0), "the whole stack gone");
    assert_eq!(app.hotbar_slots[1], (0, 0), "the HUD belief synced");
    match click_rx.try_recv().expect("the packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 31, "the hotbar row's own menu id");
            assert_eq!(c.button, 1, "the Ctrl the byte keys");
            assert_eq!(c.changed, vec![(31, (0, 0))]);
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE PLAIN Q ON THE RESULT SEAT (menu 2): the plain take the
    // answer wears — the yield the Q throws, the input's decrement
    // the SMELTING TICK the server keeps (no recompute the furnace
    // walk ever fires)
    app.furnace_slot_rects = vec![(100, 100, 2)];
    app.mouse_pos = Some((108, 108));
    app.on_furnace_drop_key(false);
    assert_eq!(
        app.furnace_slots[2],
        (iron_ingot, 1),
        "the one ingot the yield keeps"
    );
    assert_eq!(app.furnace_slots[0], (raw_iron, 5), "the input whole");
    match click_rx.try_recv().expect("the packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 2, "the result seat's own menu id");
            assert_eq!(
                c.changed,
                vec![(2, (iron_ingot, 1))],
                "the take's mutation named"
            );
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE FURNACE'S OWN Q, THE WIRE GATES (the keyPressed chain's own
/// shape over the 39-slot belief): the EMPTY hovered seat the
/// silence keeps (NO packet — the hasItem gate lives in the
/// keyPressed walk), the carried cursor the ALWAYS-PACKET law
/// names (the EMPTY changed map, the belief whole), the
/// between-slots press the null hover the silence speaks, and the
/// gates' pass the take walk restores.
#[test]
fn the_wire_furnace_drop_key_the_gates_and_the_silences_wear() {
    use crate::world_view::{ContainerCommand, LiveContainer};
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let furnace = LiveContainer {
        id: 7,
        menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
        slots: {
            let mut s = vec![(0i32, 0i32); 39];
            s[1] = (coal, 7); // the fuel seat — the Q's own target
            s
        },
        carried: (0, 0),
        state_id: 2,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(furnace));
    app.furnace_slot_rects = vec![(100, 100, 1)];
    // THE EMPTY HOVERED SEAT (the hasItem gate — the input seat
    // empty): silence, NO packet
    app.furnace_slot_rects = vec![(100, 100, 0)];
    app.mouse_pos = Some((108, 108));
    app.on_furnace_drop_key(false);
    assert!(
        click_rx.try_recv().is_err(),
        "the hasItem gate the packet lane never passes"
    );
    // THE CARRIED CURSOR (the always-packet law): the packet rides,
    // the EMPTY changed map the untouched state keeps, the belief
    // whole
    app.furnace_slot_rects = vec![(100, 100, 1)];
    app.mouse_pos = Some((108, 108));
    app.live_furnace_carried = (dirt, 5);
    app.on_furnace_drop_key(true);
    assert_eq!(app.furnace_slots[1], (coal, 7), "the belief whole");
    match click_rx.try_recv().expect("the always packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 1, "the fuel seat's own menu id");
            assert_eq!(c.button, 1, "the Ctrl the byte keys");
            assert!(c.changed.is_empty(), "the untouched state keeps");
            assert_eq!(c.carried, (dirt, 5), "the cursor pair rides unchanged");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    app.live_furnace_carried = (0, 0);
    // THE BETWEEN-SLOTS PRESS (the null hoveredSlot): the refusal
    app.mouse_pos = Some((160, 108));
    app.on_furnace_drop_key(false);
    assert!(click_rx.try_recv().is_err(), "the null hover is silence");
    // THE GATES' PASS — the take walk restores (the one off)
    app.mouse_pos = Some((108, 108));
    app.on_furnace_drop_key(false);
    assert_eq!(app.furnace_slots[1], (coal, 6), "the one off rode");
    match click_rx.try_recv().expect("the packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.changed, vec![(1, (coal, 6))]);
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE FURNACE'S OWN Q, THE OFFLINE SIM (the vanilla doClick THROW
/// the local lane answers — the offline world the integrated walk
/// IS): the PLAIN Q the fuel seat's ONE off wears, the drop entity
/// the AIMED look spawns, the CTRL+Q on the hotbar seat the
/// WRITE-THROUGH projection syncs (the menu slot the inventory
/// cell answers), the RESULT seat the plain take answers, the
/// carried cursor the doClick arm's own refusal keeps.
#[test]
fn the_offline_furnace_drop_key_the_write_through_and_the_drop_wear() {
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let iron_ingot = mc_registry::item_id_by_name("iron_ingot").unwrap();
    let furnace_id = mc_registry::item_id_by_name("furnace").unwrap();
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.inventory.items[0] = (furnace_id, 1);
    app.inventory.items[1] = (dirt, 32); // the hotbar row 1 — menu 31
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: std::f32::consts::PI,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    assert!(app.on_right_press(0, 0), "the furnace placed");
    assert!(app.on_right_press(0, 0), "the door opened");
    assert_eq!(app.screen, ScreenKind::Furnace);
    app.furnace_panel = (100, 100);
    app.furnace_slots[1] = (coal, 7);
    app.furnace_slots[2] = (iron_ingot, 2);
    app.furnace_slot_rects = vec![(100, 100, 1), (140, 100, 31), (180, 100, 2)];
    let drops_before = app.drops.len();
    // THE PLAIN Q ON THE FUEL (menu 1): the ONE off, the drop
    // entity the AIMED look spawns
    app.mouse_pos = Some((108, 108));
    app.on_furnace_drop_key(false);
    assert_eq!(app.furnace_slots[1], (coal, 6), "the one off rode");
    assert_eq!(
        app.drops.len(),
        drops_before + 1,
        "the drop entity the aimed look spawned"
    );
    // THE CTRL+Q ON THE HOTBAR SEAT (menu 31 — the write-through
    // the projection is: the menu slot the inventory cell answers)
    app.mouse_pos = Some((148, 108));
    app.on_furnace_drop_key(true);
    assert_eq!(app.furnace_slots[31], (0, 0), "the whole stack gone");
    assert_eq!(
        app.inventory.items[1],
        (0, 0),
        "the write-through the projection synced"
    );
    assert_eq!(app.hotbar_slots[1], (0, 0), "the HUD row synced");
    // THE PLAIN Q ON THE RESULT SEAT (menu 2): the plain take
    app.mouse_pos = Some((188, 108));
    app.on_furnace_drop_key(false);
    assert_eq!(
        app.furnace_slots[2],
        (iron_ingot, 1),
        "the one ingot the yield keeps"
    );
    // THE CARRIED CURSOR (the getCarried().isEmpty() gate): the
    // silence, the belief whole, no further drop
    app.mouse_pos = Some((108, 108));
    app.furnace_carried = (dirt, 5);
    app.on_furnace_drop_key(false);
    assert_eq!(app.furnace_slots[1], (coal, 6), "the belief whole");
    assert_eq!(
        app.drops.len(),
        drops_before + 3,
        "the carried the silence keeps (the walk's own three drops the arms spent)"
    );
    app.furnace_carried = (0, 0);
}

/// THE FURNACE'S OWN SWAP, THE TABLE THE HUD WEARS (the doClick
/// SWAP table over the 39-slot belief): CASE B the fuel seat's
/// stack rides the button row AS-IS (the cell EMPTIES, the HUD row
/// the every-mutation law syncs); the CASE D straight trade the
/// main store row over the hotbar row wears (the row the
/// hovered's takes, the cell the row's stack wears).
#[test]
fn the_furnace_swap_key_the_table_the_hud_wear() {
    use crate::world_view::{ContainerCommand, LiveContainer};
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let furnace = LiveContainer {
        id: 7,
        menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
        slots: {
            let mut s = vec![(0i32, 0i32); 39];
            s[1] = (coal, 7); // the fuel seat
            s[7] = (stone, 5); // the main store 4 — menu 7
            s[30] = (planks, 7); // the hotbar row 0 — menu 30
            s
        },
        carried: (0, 0),
        state_id: 2,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(furnace));
    assert_eq!(app.screen, ScreenKind::Furnace, "the window shows");
    // CASE B: THE FUEL SEAT'S STACK OVER THE EMPTY HOTBAR ROW 2
    // (menu 32)
    app.furnace_slot_rects = vec![(100, 100, 1)];
    app.mouse_pos = Some((108, 108));
    assert!(app.on_furnace_swap_key(2), "the digit consumes");
    assert_eq!(app.furnace_slots[1], (0, 0), "the cell emptied");
    assert_eq!(app.furnace_slots[32], (coal, 7), "the row rides AS-IS");
    assert_eq!(app.hotbar_slots[2], (coal, 7), "the HUD synced");
    match click_rx.try_recv().expect("the packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 7, "the window's own id");
            assert_eq!(c.state_id, 2, "the revision rides");
            assert_eq!(c.slot, 1, "the fuel seat's own menu id");
            assert_eq!(c.button, 2, "the digit the byte keys");
            assert_eq!(c.input, crate::world_view::ContainerInput::Swap);
            assert_eq!(
                c.changed,
                vec![(1, (0, 0)), (32, (coal, 7))],
                "the mutations named"
            );
            assert_eq!(c.carried, (0, 0), "the cursor pair rides unchanged");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE CASE D STRAIGHT TRADE: THE MAIN STORE 4 (menu 7) OVER
    // THE HOTBAR ROW 0 (menu 30)
    app.furnace_slot_rects = vec![(100, 100, 7)];
    app.mouse_pos = Some((108, 108));
    assert!(app.on_furnace_swap_key(0));
    assert_eq!(
        app.furnace_slots[7],
        (planks, 7),
        "the cell wears the row's stack"
    );
    assert_eq!(
        app.furnace_slots[30],
        (stone, 5),
        "the row takes the hovered's"
    );
    assert_eq!(app.hotbar_slots[0], (stone, 5), "the HUD synced");
    match click_rx.try_recv().expect("the packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 7, "the main store's own menu id");
            assert_eq!(
                c.changed,
                vec![(30, (stone, 5)), (7, (planks, 7))],
                "the trade's mutations named"
            );
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE FURNACE'S OWN SWAP, THE SPLIT LAW AND THE SEAT VERDICTS
/// (the mayPickup/mayPlace pair the SWAP table keys): CASE C the
/// row's 20 eggs over the empty main store row — the cell takes
/// the max 16, the row the remainder 4 keeps; the FUEL seat the
/// isFuel verdict answers (the coal places, the row empties), the
/// INPUT seat the SMELTABLE verdict answers (the raw_iron places,
/// the dirt the refusal the always-packet keeps), the RESULT seat
/// the mayPlace iconst_0 the place arms refuse (the packet rides,
/// the belief whole).
#[test]
fn the_furnace_swap_key_the_split_law_and_the_seat_verdicts_wear() {
    use crate::world_view::{ContainerCommand, LiveContainer};
    let egg = crate::mob::wire_id("egg").expect("the egg's own id");
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let raw_iron = mc_registry::item_id_by_name("raw_iron").unwrap();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let furnace = LiveContainer {
        id: 7,
        menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
        slots: {
            let mut s = vec![(0i32, 0i32); 39];
            s[31] = (coal, 5); // the hotbar row 1 — menu 31 (the fuel place arm's own row)
            s[33] = (egg, 20); // the hotbar row 3 — menu 33
            s[34] = (raw_iron, 3); // the hotbar row 4 — menu 34
            s[35] = (dirt, 3); // the hotbar row 5 — menu 35
            s[36] = (stone, 3); // the hotbar row 6 — menu 36
            s
        },
        carried: (0, 0),
        state_id: 2,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(furnace));
    app.furnace_slot_rects = vec![(100, 100, 12), (140, 100, 1), (180, 100, 0), (220, 100, 2)];
    // CASE C: THE EMPTY MAIN STORE ROW TAKES THE MAX 16
    app.mouse_pos = Some((108, 108));
    assert!(app.on_furnace_swap_key(3));
    assert_eq!(app.furnace_slots[12], (egg, 16), "the cell takes the max");
    assert_eq!(
        app.furnace_slots[33],
        (egg, 4),
        "the row the remainder keeps"
    );
    assert_eq!(app.hotbar_slots[3], (egg, 4), "the HUD synced");
    match click_rx.try_recv().expect("the packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 12, "the main store's own menu id");
            assert_eq!(
                c.changed,
                vec![(12, (egg, 16)), (33, (egg, 4))],
                "the split's mutations named"
            );
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE FUEL SEAT'S ISFUEL VERDICT (the coal places, the row
    // empties)
    app.mouse_pos = Some((148, 108));
    assert!(app.on_furnace_swap_key(1));
    assert_eq!(
        app.furnace_slots[1],
        (coal, 5),
        "the fuel seat the coal takes"
    );
    assert_eq!(app.furnace_slots[31], (0, 0), "the row emptied");
    match click_rx.try_recv().expect("the packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(
                c.changed,
                vec![(31, (0, 0)), (1, (coal, 5))],
                "the place's mutations named"
            );
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE INPUT SEAT'S SMELTABLE VERDICT (the raw_iron places)
    app.mouse_pos = Some((188, 108));
    assert!(app.on_furnace_swap_key(4));
    assert_eq!(
        app.furnace_slots[0],
        (raw_iron, 3),
        "the input seat the smeltable takes"
    );
    assert_eq!(app.furnace_slots[34], (0, 0), "the row emptied");
    match click_rx.try_recv().expect("the packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(
                c.changed,
                vec![(34, (0, 0)), (0, (raw_iron, 3))],
                "the place's mutations named"
            );
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE INPUT SEAT'S REFUSAL (the dirt the smeltable verdict
    // refuses — the always-packet law the empty changed map names)
    // — the row 5 re-fed first (the prior swap spent it)
    app.furnace_slots[35] = (dirt, 3);
    app.mouse_pos = Some((188, 108));
    assert!(app.on_furnace_swap_key(5), "the refusal still consumes");
    assert_eq!(app.furnace_slots[0], (raw_iron, 3), "the input whole");
    assert_eq!(app.furnace_slots[35], (dirt, 3), "the row whole");
    match click_rx.try_recv().expect("the always packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 0, "the input seat's own menu id");
            assert_eq!(c.button, 5, "the digit the byte keys");
            assert!(c.changed.is_empty(), "the refusal no mutation names");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE RESULT SEAT'S MAYPLACE REFUSAL (the iconst_0 the place
    // arms refuse): the packet rides, the belief whole
    app.mouse_pos = Some((228, 108));
    assert!(app.on_furnace_swap_key(6), "the refusal still consumes");
    assert_eq!(app.furnace_slots[2], (0, 0), "the result whole");
    assert_eq!(app.furnace_slots[36], (stone, 3), "the row whole");
    match click_rx.try_recv().expect("the always packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 2, "the result seat's own menu id");
            assert_eq!(c.button, 6, "the digit the byte keys");
            assert!(c.changed.is_empty(), "the refusal no mutation names");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
}

/// THE FURNACE'S OWN SWAP, THE GATES (the checkHotbarKeyPressed
/// chain's own shape over the 39-slot belief): the BOTH-EMPTY
/// press the consumed silence keeps AND the packet rides (the
/// empty changed map the untouched state names), the SAME-SEAT
/// press (the hovered hotbar row IS the button's own row) the
/// identical pair sets back — no mutation the diff ever names,
/// the carried cursor the false BEFORE the packet lane keeps, the
/// closed window and the between-slots press the refusals keep.
#[test]
fn the_furnace_swap_key_the_gates_and_the_always_packet_wear() {
    use crate::world_view::{ContainerCommand, LiveContainer};
    let coal = mc_registry::item_id_by_name("coal").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let furnace = LiveContainer {
        id: 7,
        menu_type_id: crate::world_model::MENU_TYPE_FURNACE,
        slots: {
            let mut s = vec![(0i32, 0i32); 39];
            s[31] = (stone, 7); // the hotbar row 1 — menu 31
            s
        },
        carried: (0, 0),
        state_id: 2,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(furnace));
    app.furnace_slot_rects = vec![(100, 100, 12), (140, 100, 31)];
    // THE CLOSED WINDOW (the identity the sync's close lane died):
    // the silence
    app.furnace_live_window = None;
    app.mouse_pos = Some((108, 108));
    assert!(!app.on_furnace_swap_key(0), "no window — the false wears");
    assert!(click_rx.try_recv().is_err(), "the closed window is silence");
    // THE WINDOW ARMS (the sync's own identity the test restores)
    app.furnace_live_window = Some((7, 2));
    // THE BOTH-EMPTY PRESS: the digit CONSUMES and the packet
    // rides with the EMPTY changed map
    app.mouse_pos = Some((108, 108));
    assert!(app.on_furnace_swap_key(0), "the silence still consumes");
    match click_rx.try_recv().expect("the always packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 12, "the hovered seat's own menu id");
            assert_eq!(c.button, 0, "the digit the byte keys");
            assert_eq!(c.input, crate::world_view::ContainerInput::Swap);
            assert!(c.changed.is_empty(), "the untouched state keeps");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE SAME-SEAT PRESS (the hotbar row 1 IS menu 31): the
    // identical pair sets back — no mutation, the belief whole
    app.mouse_pos = Some((148, 108));
    assert!(app.on_furnace_swap_key(1), "the same seat still consumes");
    assert_eq!(app.furnace_slots[31], (stone, 7), "the belief whole");
    assert_eq!(app.hotbar_slots[1], (stone, 7), "the HUD whole");
    match click_rx.try_recv().expect("the packet rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 31, "the same seat's own menu id");
            assert!(c.changed.is_empty(), "no mutation the diff names");
        }
        other => panic!("expected a container click, got {other:?}"),
    }
    // THE CARRIED CURSOR (the getCarried().isEmpty() gate): the
    // false BEFORE the packet lane — NO packet
    app.mouse_pos = Some((108, 108));
    app.live_furnace_carried = (coal, 5);
    assert!(!app.on_furnace_swap_key(0), "the carried gate refuses");
    assert!(click_rx.try_recv().is_err(), "no packet the gate passes");
    app.live_furnace_carried = (0, 0);
    // THE BETWEEN-SLOTS PRESS (the null hoveredSlot): the refusal
    app.mouse_pos = Some((160, 108));
    assert!(!app.on_furnace_swap_key(0), "the null hover refuses");
    assert!(click_rx.try_recv().is_err(), "still silence");
}

/// THE FURNACE'S OWN FORMULA WALK (javap AbstractFurnaceMenu +
/// AbstractFurnaceScreen — the pinned pairs the pure fns bank):
/// the lit pair's clamp, the zero-total's 200 fallback, the +1
/// minimum, the cooking pair's zero guards, the ceil rounding the
/// Mth.ceil law speaks.
#[test]
fn the_furnace_formulas_speak_the_pinned_pairs() {
    // the fresh menu's zeros: the arrow 0 (the zero guards), the
    // flame UNLIT (the data[0] > 0 gate the caller keeps)
    let fresh = [0, 0, 0, 0];
    assert_eq!(furnace_arrow_width(&fresh), 0);
    // the zero-total fallback: lit remaining 100 of UNKNOWN total
    // → the 200 fallback speaks, ceil(0.5 * 13.0) + 1 = 8
    assert_eq!(furnace_flame_height(&[100, 0, 0, 0]), 8);
    // the clamp pair: over-full lit (remaining > total) pins at
    // 1.0 — ceil(13.0) + 1 = 14
    assert_eq!(furnace_flame_height(&[500, 200, 0, 0]), 14);
    // full lit: ceil(1.0 * 13.0) + 1 = 14
    assert_eq!(furnace_flame_height(&[200, 200, 0, 0]), 14);
    // the +1 minimum: the tiniest lit word speaks —
    // ceil(1/200 * 13.0) + 1 = ceil(0.065) + 1 = 2
    assert_eq!(furnace_flame_height(&[1, 200, 0, 0]), 2);
    // the arrow's zero guards: progress 0 OR total 0 → 0
    assert_eq!(furnace_arrow_width(&[180, 200, 0, 200]), 0);
    assert_eq!(furnace_arrow_width(&[180, 200, 90, 0]), 0);
    // the clamp: over-cooked progress pins at 1.0 → ceil(24.0) = 24
    assert_eq!(furnace_arrow_width(&[180, 200, 300, 200]), 24);
    // full arrow: ceil(1.0 * 24.0) = 24
    assert_eq!(furnace_arrow_width(&[180, 200, 200, 200]), 24);
}

/// THE WIRE TABLE'S OWN CLICK ARC (the window-1 identity the
/// ServerboundContainerClickPacket speaks — the chest's send lane
/// the crafting menu's own numbering rides): the store row's
/// pickup carries the window's id + revision, the result slot's
/// take the onTake's changed entries name (the cells' decrement +
/// the recompute), the outside press the THROW input speaks, and
/// the close echoes the Container Close packet.
#[test]
fn the_wire_table_clicks_speak_the_window() {
    use crate::world_view::{ContainerCommand, ContainerInput, LiveContainer};
    // the REAL recipe ids the preview's recompute needs (the sticks
    // column the cells keep — the belief's own verdict the onTake
    // re-speaks; a non-recipe cargo the honest EMPTY verdict gives)
    let planks = mc_registry::item_id_by_name("oak_planks").unwrap();
    let stick = mc_registry::item_id_by_name("stick").unwrap();
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let table = LiveContainer {
        id: 9,
        menu_type_id: crate::world_model::MENU_TYPE_CRAFTING,
        slots: {
            let mut s = vec![(0i32, 0i32); 46];
            s[0] = (stick, 4); // the result: stick x4
            s[1] = (planks, 7); // the grid's cell 1 — the sticks column
            s[4] = (planks, 7); // the grid's cell 4 keeps
            s[10] = (921, 5); // the main grid's head: apple x5
            s
        },
        carried: (0, 0),
        state_id: 3,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    let table2 = table.clone(); // the re-arm's own fresh window
    app.live_view_updated(air_net_world(2, 1), None, None, Some(table));
    app.table_panel = inventory_panel_origin(640, 360);
    let (px, py) = inventory_panel_origin(640, 360);
    // THE STORE ROW'S PICKUP (menu slot 10 = the main grid's
    // head): the whole stack rides the cursor, the packet the
    // window's own identity names
    app.on_press(px + 8 + 1 + 8, py + 84 + 1 + 8);
    match click_rx.try_recv().expect("the store click rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9, "the window's own id");
            assert_eq!(c.state_id, 3, "the revision rides");
            assert_eq!(c.slot, 10, "the menu's own numbering");
            assert_eq!(c.input, ContainerInput::Pickup);
            assert_eq!(c.changed, vec![(10, (0, 0))], "the taken slot emptied");
            assert_eq!(c.carried, (921, 5), "the cursor carries the stack");
        }
        _ => panic!("the click command's own shape"),
    }
    assert_eq!(
        app.live_table_carried,
        (921, 5),
        "the belief's cursor follows"
    );
    // the SECOND press places the stack back (the pickup walk's
    // own place arm the uniform table keeps). THE QUICKCRAFT
    // KEYING (the 26.2 mouseClicked's own law): the carried press
    // ARMS the drag (no place on press), and the empty-paint
    // release the deferred place speaks — the same PICKUP walk
    // the press used to ride, the release's own lane answers.
    app.on_mouse_move(px + 8 + 1 + 8, py + 84 + 1 + 8);
    app.on_press(px + 8 + 1 + 8, py + 84 + 1 + 8);
    app.on_left_release();
    match click_rx.try_recv().expect("the place click rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.changed, vec![(10, (921, 5))], "the stack landed back");
            assert_eq!(c.carried, (0, 0), "the cursor emptied");
        }
        _ => panic!("the click command's own shape"),
    }
    // THE RESULT SLOT'S TAKE (menu slot 0 — the mayPlace-false
    // walk): the carried receives, the cells one off, the
    // recompute the preview speaks (the sticks recipe the column
    // keeps — the result's own verdict no changed entry names)
    app.on_press(px + 124 + 1 + 8, py + 35 + 1 + 8);
    match click_rx.try_recv().expect("the result click rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 0, "the result slot's own id");
            assert_eq!(c.input, ContainerInput::Pickup);
            assert_eq!(c.carried, (stick, 4), "the whole result taken");
            assert_eq!(
                c.changed,
                vec![(1, (planks, 6)), (4, (planks, 6))],
                "the cells' decrement the onTake names"
            );
        }
        _ => panic!("the click command's own shape"),
    }
    assert_eq!(
        app.live_table_slots[1],
        (planks, 6),
        "the belief's cell spent"
    );
    assert_eq!(app.live_table_slots[0], (stick, 4), "the recipe re-spoke");
    // THE THROW (the outside press — the carried press the drag
    // ARMS, the release the -999 PICKUP the whole carried drops:
    // the vanilla law the RELEASE lane rides, the THROW-on-press
    // the doClick's own -999 arm retires) — the carried stack the
    // packet names, the belief clears (the drop entity the SERVER
    // spawns)
    app.on_press(10, 10);
    app.on_mouse_move(10, 10);
    app.on_left_release();
    match click_rx.try_recv().expect("the throw rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 9, "the window's own id");
            assert_eq!(c.slot, -999, "the outside release's own -999");
            assert_eq!(c.button, 0, "the whole stack the PRIMARY drops");
            assert_eq!(c.input, ContainerInput::Pickup);
            assert_eq!(c.changed, Vec::new(), "the drop the map never names");
            assert_eq!(c.carried, (0, 0), "the whole stack thrown");
        }
        _ => panic!("the click command's own shape"),
    }
    // THE CLOSE (the Escape's own arm): the Container Close packet
    // echoes, the belief resets with the window, the world stands
    app.on_escape();
    match click_rx.try_recv().expect("the close echo rode the lane") {
        ContainerCommand::Close { window_id } => {
            assert_eq!(window_id, 9, "the window's own close");
        }
        _ => panic!("the close command's own shape"),
    }
    assert_eq!(app.screen, ScreenKind::World, "the menu closed");
    assert!(!app.table_is_live, "the live flag dropped");
    assert_eq!(app.table_live_window, None, "the window died");
    // the lane RE-ARMS (the server's window back) and the
    // BETWEEN-SLOTS press is the honest no-op (inside the panel,
    // no slot — the wire lane's own gate)
    app.live_view_updated(air_net_world(2, 10), None, None, Some(table2));
    assert_eq!(app.screen, ScreenKind::Crafting, "the window re-opened");
    app.table_panel = inventory_panel_origin(640, 360);
    app.on_press(px + 2, py + 70);
    assert!(click_rx.try_recv().is_err(), "no slot, no click");
}

/// The slot click's whole arc: the live chest's press rides the
/// send lane with the window's own identity (id + revision), the
/// between-slots press is the honest no-op, and the offline
/// stand-in never speaks (no window, no session).
#[test]
fn the_slot_click_rides_the_send_lane() {
    use crate::world_view::{ContainerCommand, ContainerInput};
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let chest = crate::world_view::LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: {
            let mut s = vec![(0i32, 0i32); 63];
            s[4] = (926, 12); // the diamond x12 in the container's slot 4
            s
        },
        carried: (0, 0),
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert_eq!(app.screen, ScreenKind::Chest);
    assert!(app.chest_is_live);
    assert_eq!(
        app.chest_live_window,
        Some((5, 7)),
        "the window's identity rode in"
    );

    // the render lays out the hit boxes (the buttons' own pattern);
    // the press at the box's center clicks the MENU's slot number
    app.render_title_buttons_for_test();
    assert_eq!(app.chest_slot_rects.len(), 63, "27 grid + 36 mirror boxes");
    let &(sx, sy, _slot) = app
        .chest_slot_rects
        .iter()
        .find(|&&(_, _, s)| s == 4)
        .expect("the container's slot 4 box");
    app.on_press(sx + 8, sy + 8);
    // THE PREDICTION: the pickup ran locally — the slot emptied and
    // the believed cursor holds the stack
    assert_eq!(app.chest_slots[4], (0, 0), "the slot's belief emptied");
    assert_eq!(
        app.chest_carried,
        (926, 12),
        "the cursor's belief holds the stack"
    );
    match click_rx.try_recv().expect("the click rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5, "the live window's id");
            assert_eq!(c.state_id, 7, "the revision the client last saw");
            assert_eq!(c.slot, 4, "the MENU's own slot number");
            assert_eq!(c.button, 0, "the left press");
            assert_eq!(c.input, ContainerInput::Pickup);
            assert_eq!(
                c.changed,
                vec![(4, (0, 0))],
                "the post-click delta rides the command"
            );
            assert_eq!(c.carried, (926, 12), "the believed cursor rides");
        }
        other => panic!("expected a click, got {other:?}"),
    }

    // the MIRROR's box speaks the menu's tail numbering (the main
    // row's first slot is the menu's 27) — the held stack PLACES
    // into the empty mirror slot (the pickup table's own arc). THE
    // QUICKCRAFT KEYING (the 26.2 mouseClicked's own law): the
    // carried press ARMS the drag (no place on press), and the
    // empty-paint release the deferred place speaks — the same
    // PICKUP walk the press used to ride, the release's own lane
    // answers (the cursor the frame stream feeds first — the
    // release POSITION the deferred place reads).
    let &(mx, my, _) = app
        .chest_slot_rects
        .iter()
        .find(|&&(_, _, s)| s == 27)
        .expect("the mirror's first box");
    app.on_mouse_move(mx + 8, my + 8);
    app.on_press(mx + 8, my + 8);
    app.on_left_release();
    assert_eq!(app.chest_mirror[0], (926, 12), "the held stack placed");
    assert_eq!(app.chest_carried, (0, 0), "the cursor emptied");
    match click_rx.try_recv().expect("the mirror click rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 27, "the menu's tail numbering");
            assert_eq!(c.changed, vec![(27, (926, 12))]);
            assert_eq!(c.carried, (0, 0));
        }
        other => panic!("expected a click, got {other:?}"),
    }

    // a press BETWEEN the boxes dies on the frame thread (no click
    // rides the lane)
    app.on_press(0, 0);
    assert!(click_rx.try_recv().is_err(), "no slot, no click");

    // the OFFLINE stand-in (the dev-66 round): the same screen, no
    // window, no session — the press the LOCAL walk answers (the
    // grid's diamond the cursor takes), the wire never speaks
    app.live_view_updated(air_net_world(2, 2), None, None, None);
    assert_eq!(app.screen, ScreenKind::World, "the live chest closed");
    app.open_chest();
    assert_eq!(app.screen, ScreenKind::Chest);
    assert!(!app.chest_is_live, "the offline flag stays down");
    assert_eq!(app.chest_live_window, None, "no window identity");
    app.render_title_buttons_for_test();
    let &(ox, oy, _) = app
        .chest_slot_rects
        .iter()
        .find(|&&(_, _, s)| s == 4)
        .expect("the offline grid's own slot 4 box");
    app.on_press(ox + 8, oy + 8);
    assert!(
        click_rx.try_recv().is_err(),
        "the offline chest never speaks"
    );
    assert_eq!(
        app.chest_carried,
        (926, 12),
        "the LOCAL take the walk answers"
    );
}

/// The player's OWN close echoes the serverbound Container Close
/// packet through the send lane (the live chest only — the offline
/// stand-in's close stays a local act).
#[test]
fn escape_echoes_the_close_packet() {
    use crate::world_view::ContainerCommand;
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let chest = crate::world_view::LiveContainer {
        id: 2,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: vec![(926, 1)],
        carried: (0, 0),
        state_id: 3,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert_eq!(app.screen, ScreenKind::Chest);
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World, "the container closed");
    assert!(!app.chest_is_live);
    assert_eq!(app.chest_live_window, None, "the window's identity dropped");
    match click_rx.try_recv().expect("the close echo rode the lane") {
        ContainerCommand::Close { window_id } => assert_eq!(window_id, 2),
        other => panic!("expected a close, got {other:?}"),
    }

    // the OFFLINE chest's Escape: the same close, no echo (no
    // window to close on the wire)
    app.open_chest();
    assert_eq!(app.screen, ScreenKind::Chest);
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World);
    assert!(
        click_rx.try_recv().is_err(),
        "the offline close never speaks"
    );
}

/// The prediction's own arcs over the believed state: the same-item
/// MERGE (the remainder rides), the different-item SWAP, the
/// authoritative correction's wholesale replacement (the server's
/// projection overrules every local belief), and the Escape reset.
#[test]
fn the_prediction_believes_and_the_correction_overrules() {
    use crate::world_view::{ContainerCommand, ContainerInput};
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let chest = crate::world_view::LiveContainer {
        id: 9,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: {
            let mut s = vec![(0i32, 0i32); 63];
            s[8] = (926, 60); // diamond x60 in the grid
            s[9] = (926, 10); // diamond x10 beside it
            s[10] = (921, 3); // apple x3
            s
        },
        carried: (0, 0),
        state_id: 11,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert_eq!(app.screen, ScreenKind::Chest);
    app.render_title_buttons_for_test();
    let box_of = |app: &App, want: i16| {
        app.chest_slot_rects
            .iter()
            .find(|&&(_, _, s)| s == want)
            .copied()
            .expect("the box")
    };

    // MERGE: pick up the x60, drop it on the x10 — the slot takes
    // what fits (to the ceiling), the remainder rides the cursor.
    // THE QUICKCRAFT KEYING: the carried press the arm speaks, the
    // empty-paint release the deferred place's merge answers (the
    // same PICKUP walk, the release's own lane)
    let (bx, by, _) = box_of(&app, 8);
    app.on_press(bx + 8, by + 8);
    assert_eq!(app.chest_carried, (926, 60));
    let (bx, by, _) = box_of(&app, 9);
    app.on_mouse_move(bx + 8, by + 8);
    app.on_press(bx + 8, by + 8);
    app.on_left_release();
    assert_eq!(app.chest_slots[9], (926, 64), "the merge filled the slot");
    assert_eq!(app.chest_carried, (926, 6), "the remainder rides");
    let _ = click_rx.try_recv(); // the first click
    match click_rx.try_recv().expect("the merge's click") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.changed, vec![(9, (926, 64))]);
            assert_eq!(c.carried, (926, 6));
        }
        other => panic!("expected a click, got {other:?}"),
    }

    // SWAP: the held diamonds over the apple — the stacks trade
    // (the same quickcraft keying: the press arms, the release's
    // deferred place speaks)
    let (bx, by, _) = box_of(&app, 10);
    app.on_mouse_move(bx + 8, by + 8);
    app.on_press(bx + 8, by + 8);
    app.on_left_release();
    assert_eq!(app.chest_slots[10], (926, 6), "the swap landed");
    assert_eq!(app.chest_carried, (921, 3), "the apple rides");
    match click_rx.try_recv().expect("the swap's click") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.changed, vec![(10, (926, 6))]);
            assert_eq!(c.carried, (921, 3));
        }
        other => panic!("expected a click, got {other:?}"),
    }

    // THE CORRECTION: the server's authoritative projection replaces
    // the whole belief — slots, cursor, window identity alike
    let truth = crate::world_view::LiveContainer {
        id: 9,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        data_slots: Vec::new(),
        slots: {
            let mut s = vec![(0i32, 0i32); 63];
            s[8] = (926, 60); // the server never processed the clicks
            s
        },
        carried: (0, 0),
        state_id: 12,
        bundle_rows: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 2), None, None, Some(truth));
    assert_eq!(app.chest_slots[9], (0, 0), "the prediction's merge is gone");
    assert_eq!(app.chest_slots[10], (0, 0), "the prediction's swap is gone");
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the server's cursor is the truth"
    );
    assert_eq!(
        app.chest_live_window,
        Some((9, 12)),
        "the revision advanced"
    );
    // the next click echoes the NEW revision
    let (bx, by, _) = box_of(&app, 8);
    app.on_press(bx + 8, by + 8);
    match click_rx.try_recv().expect("the post-correction click") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.state_id, 12, "the correction's revision rides");
            assert_eq!(c.input, ContainerInput::Pickup);
        }
        other => panic!("expected a click, got {other:?}"),
    }

    // ESCAPE resets the belief with the window (the held stack
    // returns to the inventory server-side)
    assert_eq!(app.chest_carried, (926, 60), "the held stack rides out");
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World);
    assert_eq!(app.chest_carried, (0, 0), "no window, no held stack");
    let _ = click_rx.try_recv(); // the close echo
}

/// The RIGHT press's own lane: the split-stack pickup (the ceiling
/// half), the place-one, and the button byte riding the command —
/// the same PICKUP input mode, the wire's other button value. The
/// offline stand-in stays silent under the right press too.
#[test]
fn the_right_press_splits_and_places_one() {
    use crate::world_view::{ContainerCommand, ContainerInput};
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let chest = crate::world_view::LiveContainer {
        id: 4,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: {
            let mut s = vec![(0i32, 0i32); 63];
            s[6] = (926, 5); // diamond x5 — the odd split's own number
            s
        },
        carried: (0, 0),
        state_id: 8,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert_eq!(app.screen, ScreenKind::Chest);
    app.render_title_buttons_for_test();
    let (bx, by, _) = app
        .chest_slot_rects
        .iter()
        .find(|&&(_, _, s)| s == 6)
        .copied()
        .expect("the box");

    // the right press takes the CEILING half (5 -> 3 taken, 2 left)
    app.on_right_press(bx + 8, by + 8);
    assert_eq!(app.chest_slots[6], (926, 2), "the odd split left two");
    assert_eq!(app.chest_carried, (926, 3), "the ceiling half rides");
    match click_rx.try_recv().expect("the right press's click") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.button, 1, "the right press's own button byte");
            assert_eq!(c.input, ContainerInput::Pickup, "the same PICKUP mode");
            assert_eq!(c.changed, vec![(6, (926, 2))]);
            assert_eq!(c.carried, (926, 3));
        }
        other => panic!("expected a click, got {other:?}"),
    }

    // the held half places ONE back per right press (THE VANILLA
    // RE-KEY: the carried press the type-1 drag arms, the release's
    // deferred place the place-ONE serves — the same PICKUP button
    // 1 the packet wears; the un-armed motion the position speaks,
    // the paint never runs)
    let (ex, ey, _) = app
        .chest_slot_rects
        .iter()
        .find(|&&(_, _, s)| s == 7)
        .copied()
        .expect("the empty box");
    app.on_mouse_move(ex + 8, ey + 8);
    app.on_right_press(ex + 8, ey + 8);
    assert!(app.qc_armed, "the carried press the type-1 drag arms");
    assert_eq!(app.chest_slots[7], (0, 0), "the press NO place speaks");
    app.on_right_release();
    assert!(!app.qc_armed, "the release the arm clears");
    assert_eq!(app.chest_slots[7], (926, 1), "one placed");
    assert_eq!(app.chest_carried, (926, 2), "the cursor paid one");
    match click_rx.try_recv().expect("the place-one's click") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.button, 1);
            assert_eq!(c.input, ContainerInput::Pickup, "the same PICKUP mode");
            assert_eq!(c.changed, vec![(7, (926, 1))]);
            assert_eq!(c.carried, (926, 2));
        }
        other => panic!("expected a click, got {other:?}"),
    }

    // the offline stand-in (the dev-66 round): the right press the
    // LOCAL walk answers, the wire stays silent
    app.on_escape();
    let _ = click_rx.try_recv(); // the close echo
    app.open_chest();
    app.render_title_buttons_for_test();
    let (ox, oy, _) = app
        .chest_slot_rects
        .iter()
        .find(|&&(_, _, s)| s == 6)
        .copied()
        .expect("the offline box");
    app.on_right_press(ox + 8, oy + 8);
    assert!(
        click_rx.try_recv().is_err(),
        "the offline right press is silent"
    );
}

/// The shift-click's whole arc: the keyboard lane's shift state
/// routes the press through the QUICK_MOVE table — the whole stack
/// moves to the OTHER section (the merge, the remainder's hotbar
/// fill), the multi-slot belief rides the command, the cursor is
/// NOT the lane, and the offline stand-in stays silent.
#[test]
fn the_shift_click_moves_the_whole_stack() {
    use crate::world_view::{ContainerCommand, ContainerInput};
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);
    let chest = crate::world_view::LiveContainer {
        id: 5,
        menu_type_id: crate::world_model::MENU_TYPE_CHEST,
        slots: {
            let mut s = vec![(0i32, 0i32); 63];
            s[4] = (926, 30); // the chest grid's diamond stack
            s[50] = (926, 40); // the mirror's same-item merge target
            s
        },
        carried: (921, 3), // a held stack: QUICK_MOVE never touches it
        state_id: 7,
        bundle_rows: Vec::new(),
        data_slots: Vec::new(),
        title_nbt: Vec::new(),
    };
    app.live_view_updated(air_net_world(2, 1), None, None, Some(chest));
    assert_eq!(app.screen, ScreenKind::Chest);
    app.render_title_buttons_for_test();
    let &(sx, sy, _slot) = app
        .chest_slot_rects
        .iter()
        .find(|&&(_, _, s)| s == 4)
        .expect("the container's slot 4 box");

    // the SHIFT+left press: the merge fills the mirror's stack to
    // the ceiling, the remainder rides the hotbar's last slot. THE
    // QUICKCRAFT KEYING: the carried press the arm speaks (the
    // shift the QUICK_MOVE type the click walk's own precedence
    // keeps — the ride the RELEASE's deferred place answers), the
    // same QUICK_MOVE packet the release lane speaks
    app.on_shift_down(true);
    app.on_mouse_move(sx + 8, sy + 8);
    app.on_press(sx + 8, sy + 8);
    app.on_left_release();
    assert_eq!(app.chest_slots[4], (0, 0), "the source emptied");
    assert_eq!(
        app.chest_mirror[23],
        (926, 64),
        "the merge filled (menu 50)"
    );
    assert_eq!(
        app.chest_mirror[35],
        (926, 6),
        "the remainder rides (menu 62)"
    );
    assert_eq!(
        app.chest_carried,
        (921, 3),
        "the cursor is not the QUICK_MOVE's lane"
    );
    match click_rx.try_recv().expect("the shift click rode the lane") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.window_id, 5);
            assert_eq!(c.state_id, 7);
            assert_eq!(c.slot, 4);
            assert_eq!(c.button, 0, "the left press's own button byte");
            assert_eq!(c.input, ContainerInput::QuickMove);
            assert_eq!(
                c.changed,
                vec![(4, (0, 0)), (50, (926, 64)), (62, (926, 6))],
                "the multi-slot belief rides the command"
            );
            assert_eq!(c.carried, (921, 3), "the carried lane untouched");
        }
        other => panic!("expected a click, got {other:?}"),
    }

    // the SHIFT+right press on the MIRROR moves the full stack back
    // to the grid (slot 0 forward) — the vanilla hasShiftDown
    // overrides the right button too (THE VANILLA RE-KEY: the
    // carried press the type-1 drag arms — the arm block's own no-
    // shift law keeps — the release's deferred place the QUICK_MOVE
    // serves, the button byte the press's own value keeps)
    let &(mx, my, mslot) = app
        .chest_slot_rects
        .iter()
        .find(|&&(_, _, s)| s == 50)
        .expect("the mirror's menu-50 box");
    assert_eq!(mslot, 50);
    app.on_mouse_move(mx + 8, my + 8);
    app.on_right_press(mx + 8, my + 8);
    assert!(app.qc_armed, "the carried press the type-1 drag arms");
    app.on_right_release();
    assert!(!app.qc_armed, "the release the arm clears");
    assert_eq!(app.chest_mirror[23], (0, 0), "the mirror's stack moved out");
    assert_eq!(app.chest_slots[0], (926, 64), "the grid's slot 0 took it");
    match click_rx.try_recv().expect("the mirror's shift click rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.slot, 50);
            assert_eq!(c.button, 1, "the right press's own button byte");
            assert_eq!(c.input, ContainerInput::QuickMove);
            assert_eq!(c.changed, vec![(50, (0, 0)), (0, (926, 64))]);
        }
        other => panic!("expected a click, got {other:?}"),
    }

    // the shift RELEASE restores the pickup table: the same spot's
    // plain left press now PLACES the held stack into the empty
    // slot (the pickup table's own place law — the shift's
    // whole-move routing is gone). THE QUICKCRAFT KEYING: the
    // carried press the arm speaks, the release's deferred place
    // the same PICKUP walk answers
    app.on_shift_down(false);
    app.on_mouse_move(sx + 8, sy + 8);
    app.on_press(sx + 8, sy + 8);
    app.on_left_release();
    assert_eq!(app.chest_slots[4], (921, 3), "the held stack placed");
    assert_eq!(app.chest_carried, (0, 0), "the cursor emptied");
    match click_rx.try_recv().expect("the plain click rode") {
        ContainerCommand::Click(c) => {
            assert_eq!(c.input, ContainerInput::Pickup, "the plain press");
            assert_eq!(c.changed, vec![(4, (921, 3))]);
            assert_eq!(c.carried, (0, 0));
        }
        other => panic!("expected a click, got {other:?}"),
    }

    // the offline stand-in: the shift press never speaks
    app.on_escape();
    let _ = click_rx.try_recv(); // the close echo
    app.open_chest();
    app.render_title_buttons_for_test();
    let (ox, oy, _) = app
        .chest_slot_rects
        .iter()
        .find(|&&(_, _, s)| s == 6)
        .copied()
        .expect("the offline box");
    app.on_shift_down(true);
    app.on_press(ox + 8, oy + 8);
    assert!(
        click_rx.try_recv().is_err(),
        "the offline shift press is silent"
    );
}

/// The live held-item lane's laws: the store's selected stack
/// loads its first-person model on change, the count is not the
/// hand's business, the emptiness and the out-of-bake ids drop to
/// the bare arm, and the wire id is the reload key.
#[test]
fn the_live_held_lane_loads_the_selected_model() {
    let mut app = App::new(3);
    assert!(app.held_item.is_none(), "the bare arm boots");
    assert_eq!(app.live_held, None);
    // the diamond's stack rides: the model loads
    app.live_held_updated(Some((926, 12)));
    assert_eq!(app.live_held, Some(926));
    assert!(app.held_item.is_some(), "the diamond's model loaded");
    // the same id, another count: the loaded model stands (the
    // count is the GUI's business, the hand renders the model)
    app.live_held_updated(Some((926, 5)));
    assert_eq!(app.live_held, Some(926));
    assert!(app.held_item.is_some());
    // the slot empties: the bare arm returns
    app.live_held_updated(Some((926, 0)));
    assert_eq!(app.live_held, None);
    assert!(app.held_item.is_none(), "the emptiness drops to the arm");
    // an id outside the bake domain (past the wire table): the
    // honest arm — no model, no lie; the id stays the store's truth
    app.live_held_updated(Some((5000, 3)));
    assert_eq!(app.live_held, Some(5000));
    assert!(app.held_item.is_none(), "no model, no lie");
    // back to the diamond: the model reloads on the id's change
    app.live_held_updated(Some((926, 1)));
    assert_eq!(app.live_held, Some(926));
    assert!(app.held_item.is_some(), "the model reloads");
}

/// The GUI icons' 3D law: the block items render their isometric
/// geometry (non-blank, the tint law rides), the flat sprites fill
/// the slot exactly, the specials' payloads land — the fallback
/// blit covers only what the quad path refuses.
#[test]
fn the_item_icons_render_the_gui_quads() {
    let lit_in_slot = |g: &Gui| -> usize {
        let mut lit = 0;
        for py in 16..48 {
            for px in 16..48 {
                let o = (py * 64 + px) * 3;
                if g.fb.color[o] != 0 || g.fb.color[o + 1] != 0 || g.fb.color[o + 2] != 0 {
                    lit += 1;
                }
            }
        }
        lit
    };
    let wire = |name: &str| -> i32 {
        mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == name)
            .expect(name) as i32
    };
    // the cobblestone icon: the element cube's isometric render —
    // the slot area carries pixels now (it was blank before this
    // round), the shaded faces riding the FACE_SHADE multipliers
    let mut fb = crate::raster::Framebuf::new(64, 64);
    let mut g = Gui::new(&mut fb, 2);
    draw_item_icon(&mut g, wire("cobblestone"), 8, 8);
    let lit = lit_in_slot(&g);
    assert!(lit > 100, "the cobblestone icon renders ({lit} px)");
    // the diamond: the flat sprite item — non-blank, centered
    let mut fb = crate::raster::Framebuf::new(64, 64);
    let mut g = Gui::new(&mut fb, 2);
    draw_item_icon(&mut g, wire("diamond"), 8, 8);
    let lit = lit_in_slot(&g);
    assert!(lit > 100, "the diamond icon renders ({lit} px)");
    // the grass block: the tint law rides — the icon's pixels lean
    // GREEN (the vanilla no-level climate's grass tint through the
    // audited colormap)
    let mut fb = crate::raster::Framebuf::new(64, 64);
    let mut g = Gui::new(&mut fb, 2);
    draw_item_icon(&mut g, wire("grass_block"), 8, 8);
    let mut green_lean = 0;
    for py in 16..48 {
        for px in 16..48 {
            let o = (py * 64 + px) * 3;
            let (r, gr, b) = (g.fb.color[o], g.fb.color[o + 1], g.fb.color[o + 2]);
            let painted = r != 0 || gr != 0 || b != 0;
            if painted && gr > r && gr >= b {
                green_lean += 1;
            }
        }
    }
    assert!(
        green_lean > 20,
        "the grass icon tints green ({green_lean} px)"
    );
    // the shield: the SPECIAL payload in the slot (the dump's
    // geometry, the base model's gui stanza)
    let mut fb = crate::raster::Framebuf::new(64, 64);
    let mut g = Gui::new(&mut fb, 2);
    draw_item_icon(&mut g, wire("shield"), 8, 8);
    let lit = lit_in_slot(&g);
    assert!(lit > 50, "the shield icon renders ({lit} px)");
}

/// The live held lane's SPECIAL payloads: the 63 block-entity /
/// entity-model items route through the dump-backed geometry lane —
/// the shield's 12 quads (2 parts x 1 cube x 6 faces) with the
/// nopattern base texture, the white banner's tinted flag variant —
/// and the emptiness law is the same honest arm as the model lane.
#[test]
fn the_live_held_lane_routes_the_special_payloads() {
    let mut app = App::new(3);
    let shield = mc_registry::generated::item_order::ITEM_WIRE_ORDER
        .iter()
        .position(|&n| n == "shield")
        .expect("shield rides the wire") as i32;
    let banner = mc_registry::generated::item_order::ITEM_WIRE_ORDER
        .iter()
        .position(|&n| n == "white_banner")
        .expect("white_banner rides the wire") as i32;
    // the shield's stack rides: the SPECIAL geometry loads
    app.live_held_updated(Some((shield, 1)));
    assert_eq!(app.live_held, Some(shield));
    let held = app.held_item.as_ref().expect("the shield loads");
    assert_eq!(held.quad_len(), 12);
    assert!(held.binds_texture("entity/shield/shield_base_nopattern"));
    // the banner: two layers (the rod + the tinted flag), the
    // flag's pixels riding the PER-COLOR pre-tinted variant
    app.live_held_updated(Some((banner, 1)));
    let held = app.held_item.as_ref().expect("the banner loads");
    assert!(held.binds_texture("entity/banner/banner_base_white"));
    // the emptiness law holds on the special lane too
    app.live_held_updated(Some((banner, 0)));
    assert!(app.held_item.is_none(), "the arm returns");
}

/// THE USE ARC: the right press raises the shield into its blocking
/// pose (the using branch's own payload — the shield_blocking base
/// poses the geometry, the mirror transform moves the centroid), the
/// bow's hold pulls through the charge states (the javap-pinned
/// charge law: 13 ticks -> pulling_1, 18 -> pulling_2), the release
/// returns the resting payload, and the wire echo rides the command
/// channel (the offline stand-in without a sink stays cosmetic).
#[test]
fn the_use_arc_raises_the_shield_and_pulls_the_bow() {
    use crate::world_view::ContainerCommand;
    let wire = |name: &str| {
        mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == name)
            .expect("the item rides the wire") as i32
    };
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);

    // THE SHIELD: the resting special loads first
    app.live_held_updated(Some((wire("shield"), 1)));
    let resting = app.held_item.as_ref().expect("the resting shield loads");
    assert_eq!(resting.quad_len(), 12);
    let resting_centroid = resting.centroid();
    assert!(app.on_right_press(0, 0), "the shield starts a use");
    assert!(app.is_using(), "the use is active");
    let blocking = app.held_item.as_ref().expect("the blocking payload loads");
    assert_eq!(
        blocking.quad_len(),
        12,
        "the same geometry — the same 12 quads"
    );
    assert!(blocking.binds_texture("entity/shield/shield_base_nopattern"));
    let blocking_centroid = blocking.centroid();
    assert!(
        (blocking_centroid[0] - resting_centroid[0]).abs() > 0.01
            || (blocking_centroid[1] - resting_centroid[1]).abs() > 0.01,
        "the blocking stanza MOVES the payload ({resting_centroid:?} -> {blocking_centroid:?})"
    );
    // the use echo rode the command channel: the camera's angles in
    // the vanilla degrees (the flight camera faces east — yaw −90)
    match click_rx.try_recv().expect("the use echo rode the lane") {
        ContainerCommand::UseItem { yaw, pitch, hand } => {
            assert_eq!(yaw, -90.0, "the vanilla yRot");
            assert!((pitch - 4.583).abs() < 0.01, "the vanilla xRot");
            assert_eq!(hand, 0, "the MAIN_HAND ordinal the wire speaks");
        }
        other => panic!("expected a UseItem echo, got {other:?}"),
    }
    // the shield has no charge lane: the ticks advance, the state
    // holds (no reload churn)
    for _ in 0..30 {
        app.update();
    }
    assert_eq!(
        app.using_item.as_ref().unwrap().state,
        0,
        "the pure gate holds"
    );
    // THE RELEASE: the falling edge returns the resting payload
    app.on_right_release();
    assert!(!app.is_using());
    let back = app.held_item.as_ref().expect("the resting payload returns");
    let back_centroid = back.centroid();
    assert!(
        (back_centroid[0] - resting_centroid[0]).abs() < 1e-4
            && (back_centroid[1] - resting_centroid[1]).abs() < 1e-4,
        "the rest pose is the one the arc started from"
    );
    assert!(matches!(
        click_rx.try_recv(),
        Ok(ContainerCommand::ReleaseUse)
    ));

    // THE BOW: the hold pulls through the charge states — 13 ticks
    // cross 0.65, 18 cross 0.9 (the UseDuration x scale law)
    app.live_held_updated(Some((wire("bow"), 1)));
    assert!(!app.is_using(), "the item switch ended the old use");
    assert!(app.on_right_press(0, 0), "the bow starts a use");
    match click_rx
        .try_recv()
        .expect("the bow's use echo rode the lane")
    {
        ContainerCommand::UseItem { .. } => {}
        other => panic!("expected a UseItem echo, got {other:?}"),
    }
    let pulling0 = app.held_item.as_ref().expect("pulling_0 loads");
    assert!(
        pulling0.binds_texture("item/bow_pulling_0"),
        "the bottom lane"
    );
    for _ in 0..13 {
        app.update();
    }
    let pulling1 = app.held_item.as_ref().expect("pulling_1 loads");
    assert!(
        pulling1.binds_texture("item/bow_pulling_1"),
        "13 * 0.05 = 0.65"
    );
    for _ in 0..5 {
        app.update();
    }
    let pulling2 = app.held_item.as_ref().expect("pulling_2 loads");
    assert!(
        pulling2.binds_texture("item/bow_pulling_2"),
        "18 * 0.05 = 0.90"
    );
    app.on_right_release();
    let resting_bow = app.held_item.as_ref().expect("the resting bow returns");
    assert!(
        resting_bow.binds_texture("item/bow"),
        "the condition's on_false"
    );
    assert!(matches!(
        click_rx.try_recv(),
        Ok(ContainerCommand::ReleaseUse)
    ));

    // A PLAIN ITEM: no using lane — the press is the honest no-op
    app.live_held_updated(Some((wire("diamond"), 1)));
    assert!(!app.on_right_press(0, 0), "the diamond cannot start a use");
    assert!(!app.is_using());
    assert!(click_rx.try_recv().is_err(), "no echo either");
}

/// THE CROSSBOW'S COMPONENT ARC — the charge_type select's consumer:
/// the full-charge release LOADS the projectile (javap
/// releaseUsing: power >= 1.0 -> tryLoadProjectiles sets the
/// CHARGED_PROJECTILES component — the pull loaded, it did not
/// fire), the resting render switches to the charged bolt (the
/// select's arrow case), the next right press fires INSTANTLY
/// (javap use: the component non-empty -> performShooting + CONSUME
/// — no pull), and the component belongs to the STACK (the item
/// switch drops it).
#[test]
fn the_crossbow_loads_on_the_full_charge_and_fires_instantly() {
    use crate::world_view::ContainerCommand;
    let wire = |name: &str| {
        mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == name)
            .expect("the item rides the wire") as i32
    };
    let mut app = App::new(3);
    app.open(ScreenKind::Connect);
    app.net_world = Some(air_net_world(2, 0));
    app.enter_network_world(None, Skybox::Overworld, None, None);
    let (click_tx, click_rx) = std::sync::mpsc::channel::<ContainerCommand>();
    app.attach_live_clicks(click_tx);

    // THE UNCHARGED PULL: the fresh stack carries no component, the
    // press starts the pull like the bow's
    app.live_held_updated(Some((wire("crossbow"), 1)));
    assert!(
        app.held_charge.is_none(),
        "the fresh stack carries no component"
    );
    assert!(
        app.on_right_press(0, 0),
        "the uncharged crossbow starts the pull"
    );
    match click_rx
        .try_recv()
        .expect("the pull's use echo rode the lane")
    {
        ContainerCommand::UseItem { .. } => {}
        other => panic!("expected a UseItem echo, got {other:?}"),
    }
    let pulling = app.held_item.as_ref().expect("pulling_0 loads");
    assert!(pulling.binds_texture("item/crossbow_pulling_0"));
    // 24 ticks then the release: under the charge duration — the
    // power < 1.0 loads nothing (the javap releaseUsing gate)
    for _ in 0..24 {
        app.update();
    }
    app.on_right_release();
    assert!(app.held_charge.is_none(), "24 < 25 ticks — no load");
    let resting = app
        .held_item
        .as_ref()
        .expect("the resting crossbow returns");
    assert!(
        resting.binds_texture("item/crossbow_standby"),
        "the model's own layer0"
    );
    assert!(matches!(
        click_rx.try_recv(),
        Ok(ContainerCommand::ReleaseUse)
    ));

    // THE FULL CHARGE: 25 ticks then the release — the LOAD law and
    // the resting render's switch to the loaded bolt (the select's
    // arrow case rides the crossbow_arrow texture)
    assert!(app.on_right_press(0, 0), "the pull starts again");
    match click_rx
        .try_recv()
        .expect("the second pull's echo rode the lane")
    {
        ContainerCommand::UseItem { .. } => {}
        other => panic!("expected a UseItem echo, got {other:?}"),
    }
    for _ in 0..25 {
        app.update();
    }
    app.on_right_release();
    assert_eq!(
        app.held_charge,
        Some(mc_registry::ChargeKind::Arrow),
        "the full charge LOADED the arrow (the component set)"
    );
    let loaded = app.held_item.as_ref().expect("the charged payload loads");
    assert!(
        loaded.binds_texture("item/crossbow_arrow"),
        "the charge_type select's arrow case"
    );
    assert!(matches!(
        click_rx.try_recv(),
        Ok(ContainerCommand::ReleaseUse)
    ));

    // THE INSTANT SHOT: the right press on the charged crossbow
    // consumes the charge — no pull starts (the vanilla CONSUME
    // branch), the plain crossbow renders again, and the shot's
    // UseItem echo rides the wire lane
    assert!(
        app.on_right_press(0, 0),
        "the charged crossbow answers the press"
    );
    assert!(app.held_charge.is_none(), "the shot consumed the charge");
    assert!(!app.is_using(), "the shot is instant — no pull started");
    let plain = app.held_item.as_ref().expect("the plain crossbow returns");
    assert!(
        plain.binds_texture("item/crossbow_standby"),
        "the standby texture returns"
    );
    match click_rx
        .try_recv()
        .expect("the shot's use echo rode the lane")
    {
        ContainerCommand::UseItem { .. } => {}
        other => panic!("expected a UseItem echo, got {other:?}"),
    }
    // THE LIVE LANE'S OWN SILENCE (javap `performShooting` the
    // `instanceof ServerLevel` gate keeps — the server's own walk
    // the arrow spawns, the wire's Add Entity the client's
    // projectile lane serves): the client predicts NO arrow the
    // echo the shot rides
    assert!(
        app.arrows.is_empty(),
        "the LIVE lane the server's own walk the wire serves — no client arrow"
    );

    // THE ROCKET SEED: the component lane's other case (the CLI
    // seed's own domain) renders the firework bolt
    app.held_charge = Some(mc_registry::ChargeKind::Rocket);
    app.load_held_payload(wire("crossbow"), None);
    let rocket = app.held_item.as_ref().expect("the rocket payload loads");
    assert!(rocket.binds_texture("item/crossbow_firework"));

    // THE STACK SWITCH: the component belongs to the stack, never
    // the slot — the item switch drops it
    app.live_held_updated(Some((wire("diamond"), 1)));
    assert!(app.held_charge.is_none(), "the switch dropped the charge");
}

/// THE CROSSBOW'S OWN OFFLINE SHOT — the arrow the walk never
/// spawned the round banks (javap `CrossbowItem.use` →
/// `performShooting` → `ProjectileWeaponItem.shoot` →
/// `CrossbowItem.shootProjectile` the 26.2 bytecode speaks): the
/// charged crossbow's press the instant CONSUME branch keeps (no
/// pull), the arrow the `getShootingPower` verdict's flat 3.15
/// velocity rides, the eye-minus-0.1 spawn the AbstractArrow ctor
/// keeps, the CRIT the `setCritArrow(isPlayer)` the player's own
/// shot ALWAYS arms, the voice the `getShotPitch(random, 0)`'s
/// own early return keeps (1.0 — no random the index-0 walk
/// spends), and the resting render the standby texture returns.
#[test]
fn the_crossbow_the_offline_shot_the_arrow_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let wire = |name: &str| {
        mc_registry::generated::item_order::ITEM_WIRE_ORDER
            .iter()
            .position(|&n| n == name)
            .expect("the item rides the wire") as i32
    };
    let crossbow_id = wire("crossbow");
    let arrow_id = wire("arrow");
    let slot = app.selected_hotbar;
    app.inventory.items[slot] = (crossbow_id, 1);
    // THE QUIVER'S OWN ARROW (the container scan the load's own
    // scan law serves — the ARROW_ONLY predicate the
    // getAllSupportedProjectiles carries, the first slot the
    // container order picks)
    let quiver = (slot + 1) % app.inventory.items.len();
    app.inventory.items[quiver] = (arrow_id, 1);
    app.hotbar_slots = app.inventory.hotbar();
    // the spawn band's air (the terrain's own tops never reach),
    // the level -z look the camera's own forward keeps
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let eye_y = app.spawn_y + 7;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, eye_y as f32, c as f32 + 0.5),
        yaw: 0.0,
        pitch: 0.0,
    });
    // THE LOAD: the full-charge crossing the USE TICK banks (javap
    // `CrossbowItem.onUseTick` the 26.2 walk — the release the
    // boolean the update walk reads, the component the crossing's
    // own scan+consume arms; the load the shot never speaks)
    assert!(app.on_right_press(0, 0), "the pull starts");
    for _ in 0..25 {
        app.update();
    }
    assert_eq!(
        app.held_charge,
        Some(mc_registry::ChargeKind::Arrow),
        "the crossing LOADED the arrow mid-hold"
    );
    assert_eq!(
        app.inventory.items[quiver].1, 0,
        "the survival draw the useAmmo's own split(1) costs"
    );
    let midhold = app
        .held_item
        .as_ref()
        .expect("the charged payload mid-hold");
    assert!(
        midhold.binds_texture("item/crossbow_arrow"),
        "the charged bolt the mid-hold render wears"
    );
    app.on_right_release();
    assert_eq!(
        app.held_charge,
        Some(mc_registry::ChargeKind::Arrow),
        "the release the component keeps"
    );
    assert!(app.arrows.is_empty(), "the load the shot never speaks");
    // THE SHOT: the charged press the instant walk answers — the
    // arrow the offline lane banks (the server's stand-in the
    // client wears)
    assert!(app.on_right_press(0, 0), "the charged press answers");
    assert!(!app.is_using(), "the shot is instant — no pull");
    assert!(app.held_charge.is_none(), "the shot consumed the charge");
    assert_eq!(app.arrows.len(), 1, "the offline shot the arrow speaks");
    let a = &app.arrows[0];
    assert!(a.player_shot, "the owner lane the player speaks");
    assert!(
        a.crit,
        "the setCritArrow(isPlayer) the player's shot ALWAYS arms"
    );
    assert!(
        (a.x - (c as f64 + 0.5)).abs() < 1e-6
            && (a.y - (eye_y as f64 - 0.1)).abs() < 1e-6
            && (a.z - (c as f64 + 0.5)).abs() < 1e-6,
        "the spawn the eye walk keeps ({},{},{})",
        a.x,
        a.y,
        a.z
    );
    assert!(
        a.vz < -3.09 && a.vz > -3.21,
        "the -z the 3.15 flat power speaks ({})",
        a.vz
    );
    // THE VOICE: the index-0 pitch the early return keeps (1.0 —
    // the fconst_1 the getShotPitch walk answers, NO random spent)
    if sounds_loaded() {
        assert!(
            app.sounds
                .events
                .iter()
                .any(|e| e.starts_with("item.crossbow.shoot @")),
            "the CROSSBOW_SHOOT voice the shot rode"
        );
    }
    // THE RESTORE: the plain crossbow renders again
    let plain = app.held_item.as_ref().expect("the plain crossbow returns");
    assert!(
        plain.binds_texture("item/crossbow_standby"),
        "the standby returns"
    );
    // THE RE-CHARGE: the fresh pull the next arrow arms (the lane
    // the one-shot never breaks)
    assert!(app.on_right_press(0, 0), "the fresh pull starts");
    assert!(app.is_using(), "the pull the fresh stack arms");
}

/// THE GAMERULE'S OWN SECOND ROW THE WORLD OPTIONS WEARS (javap 26.2
/// GameRules.RULE_DAYLIGHT the row the day clock's advance gate
/// reads): the label the `gamerule.doDaylightCycle` lang row speaks,
/// the value the `options.on`/`options.off` pair the boolean display
/// keeps, the click the toggle the offline lane's own gamerule store
/// answers, the NETWORKED world the inactive button the hovered()
/// gate refuses.
#[test]
fn the_world_options_screen_wears_the_do_daylight_cycle_row() {
    let mut app = App::new(3);
    app.enter_world();
    assert!(
        app.do_daylight_cycle,
        "the vanilla default the true boot wears"
    );
    app.open(ScreenKind::WorldOptions);
    app.render_title_buttons_for_test();
    let daylight = app
        .buttons
        .iter()
        .find(|b| b.id == "wodaylight")
        .expect("the doDaylightCycle row the offline screen wears")
        .clone();
    assert_eq!(
        daylight.label, "Advance time of day: ON",
        "the lang row + the options.on word the fresh boot wears"
    );
    assert!(daylight.active, "the OFFLINE world the gamerule arm wears");
    // THE TOGGLE (the click the BooleanValue's own flip answers —
    // the row's word the re-render speaks)
    app.on_click(daylight.x + daylight.w / 2, daylight.y + daylight.h / 2);
    assert!(!app.do_daylight_cycle, "the click the rule disarms");
    app.render_title_buttons_for_test();
    let daylight = app
        .buttons
        .iter()
        .find(|b| b.id == "wodaylight")
        .expect("the row")
        .clone();
    assert_eq!(
        daylight.label, "Advance time of day: OFF",
        "the options.off word the disarmed row speaks"
    );
    // THE SECOND CLICK (the flip the boolean's own walk rings home)
    app.on_click(daylight.x + daylight.w / 2, daylight.y + daylight.h / 2);
    assert!(app.do_daylight_cycle, "the second click the row re-arms");
    // THE NETWORKED LANE'S REFUSAL (the inactive button the
    // hovered() gate never clicks — the server's own /gamerule the
    // wire keeps)
    app.world_is_network = true;
    app.render_title_buttons_for_test();
    let daylight = app
        .buttons
        .iter()
        .find(|b| b.id == "wodaylight")
        .expect("the row")
        .clone();
    assert!(
        !daylight.active,
        "the networked world the gamerule arm refuses"
    );
    app.on_click(daylight.x + daylight.w / 2, daylight.y + daylight.h / 2);
    assert!(
        app.do_daylight_cycle,
        "the inactive button the click never disarms"
    );
}

/// THE GAMERULE'S OWN THIRD ROW THE WORLD OPTIONS WEARS (javap 26.2
/// GameRules.RULE_DOMOBSPAWNING the row the natural spawner's cycle
/// reads): the label the `gamerule.doMobSpawning` lang row speaks,
/// the value the `options.on`/`options.off` pair the boolean display
/// keeps, the click the toggle the offline lane's own gamerule store
/// answers, the NETWORKED world the inactive button the hovered()
/// gate refuses.
#[test]
fn the_world_options_screen_wears_the_do_mob_spawning_row() {
    let mut app = App::new(3);
    app.enter_world();
    assert!(
        app.do_mob_spawning,
        "the vanilla default the true boot wears"
    );
    app.open(ScreenKind::WorldOptions);
    app.render_title_buttons_for_test();
    let mobspawn = app
        .buttons
        .iter()
        .find(|b| b.id == "womobspawn")
        .expect("the doMobSpawning row the offline screen wears")
        .clone();
    assert_eq!(
        mobspawn.label, "Spawn mobs: ON",
        "the lang row + the options.on word the fresh boot wears"
    );
    assert!(mobspawn.active, "the OFFLINE world the gamerule arm wears");
    // THE TOGGLE (the click the BooleanValue's own flip answers —
    // the row's word the re-render speaks)
    app.on_click(mobspawn.x + mobspawn.w / 2, mobspawn.y + mobspawn.h / 2);
    assert!(!app.do_mob_spawning, "the click the rule disarms");
    app.render_title_buttons_for_test();
    let mobspawn = app
        .buttons
        .iter()
        .find(|b| b.id == "womobspawn")
        .expect("the row")
        .clone();
    assert_eq!(
        mobspawn.label, "Spawn mobs: OFF",
        "the options.off word the disarmed row speaks"
    );
    // THE SECOND CLICK (the flip the boolean's own walk rings home)
    app.on_click(mobspawn.x + mobspawn.w / 2, mobspawn.y + mobspawn.h / 2);
    assert!(app.do_mob_spawning, "the second click the row re-arms");
    // THE NETWORKED LANE'S REFUSAL (the inactive button the
    // hovered() gate never clicks — the server's own /gamerule the
    // wire keeps)
    app.world_is_network = true;
    app.render_title_buttons_for_test();
    let mobspawn = app
        .buttons
        .iter()
        .find(|b| b.id == "womobspawn")
        .expect("the row")
        .clone();
    assert!(
        !mobspawn.active,
        "the networked world the gamerule arm refuses"
    );
    app.on_click(mobspawn.x + mobspawn.w / 2, mobspawn.y + mobspawn.h / 2);
    assert!(
        app.do_mob_spawning,
        "the inactive button the click never disarms"
    );
}

/// THE SENSITIVITY SLIDER'S OWN LAW (the OptionInstance.SENSITIVITY
/// seat the 26.2 Options clinit banks — the UnitDouble 0..1 the
/// default 0.5 wears, the caption the value*2 the percent answers
/// (the 0.0 the *yawn* end wears, the 2.0 the HYPERSPEED!!! end
/// wears), the press the setValueFromMouseX walk speaks, the armed
/// drag the track's outside rides, the release the disarm speaks).
#[test]
fn the_sensitivity_slider_wears_the_value_and_caption_law() {
    let mut app = App::new(3);
    assert_eq!(
        app.look_sensitivity, 0.5,
        "the default the UnitDouble's own 0.5 wears"
    );
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let b = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_sensitivity")
        .expect("the slider row")
        .clone();
    assert_eq!(
        b.label, "Sensitivity: 100%",
        "the default's own caption the 0.5*2 percent speaks"
    );
    assert_eq!(b.value, Some(0.5), "the slider render path the value wears");
    // THE CENTER PRESS (the setValueFromMouseX walk — the 4px inset
    // the thumb travel wears, the mid-track the 0.5 the unit keeps)
    app.on_press(b.x + b.w / 2, b.y + b.h / 2);
    assert_eq!(app.look_sensitivity, 0.5, "the center the 0.5 keeps");
    assert!(app.sensitivity_drag, "the press inside the track arms");
    // THE RIGHT EDGE (the clamp the 1.0 keeps) + THE ARMED DRAG the
    // rect never re-checks (the track's outside the same seat rides)
    app.on_press(b.x + b.w - 1, b.y + b.h / 2);
    assert_eq!(app.look_sensitivity, 1.0, "the right edge clamps");
    app.on_sensitivity_slider_drag(b.x + b.w + 60);
    assert_eq!(app.look_sensitivity, 1.0, "the beyond-right drag clamps");
    // THE RELEASE'S OWN DISARM (the mouseReleased walk — the flag
    // the unconditional false speaks, the disarmed drag the
    // silence keeps)
    app.on_left_release();
    assert!(!app.sensitivity_drag, "the release disarms");
    app.on_sensitivity_slider_drag(b.x + 4);
    assert_eq!(app.look_sensitivity, 1.0, "the disarmed drag never moves");
    // THE MAX END'S CAPTION (the d0 == 2.0 the HYPERSPEED!!! wears)
    app.render_title_buttons_for_test();
    let b = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_sensitivity")
        .expect("the slider row")
        .clone();
    assert_eq!(b.label, "Sensitivity: HYPERSPEED!!!");
    assert_eq!(b.value, Some(1.0));
    // THE LEFT EDGE (the clamp the 0.0 keeps — the *yawn* end wears)
    app.on_press(b.x + 4, b.y + b.h / 2);
    assert_eq!(app.look_sensitivity, 0.0, "the left edge clamps");
    app.on_left_release();
    app.render_title_buttons_for_test();
    let b = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_sensitivity")
        .expect("the slider row")
        .clone();
    assert_eq!(b.label, "Sensitivity: *yawn*");
}

/// THE LOOK LANE'S OWN STORE (the on_look_delta walk — the slider
/// the `options.sensitivity` the curve's own gain writes, the same
/// drag the sharper turn speaks; THE KEYBOARD LANE'S OWN
/// INDEPENDENCE the verdict keeps — the turn key's rate the
/// slider never scales, the lane's own law the store never reads).
#[test]
fn the_look_lane_wears_the_stored_sensitivity() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.look_sensitivity = 0.5;
    let y0 = app.body.as_ref().unwrap().yaw;
    app.on_look_delta(100.0, 0.0);
    let y1 = app.body.as_ref().unwrap().yaw;
    assert!(y1 > y0, "the positive drag the positive yaw wears");
    // THE SHARPER GAIN (the 1.0 the same drag the wider turn wears
    // — the store the curve's own sensitivity reads)
    app.look_sensitivity = 1.0;
    app.on_look_delta(-100.0, 0.0);
    let y2 = app.body.as_ref().unwrap().yaw;
    assert!(
        y2 < y0,
        "the 1.0 gain the same drag the sharper turn-back wears"
    );
    // THE KEYBOARD LANE'S OWN INDEPENDENCE (the rate the slider
    // never scales — 180 deg/s the dt alone scales, the 0.5s the
    // quarter revolution speaks at ANY sensitivity)
    let y3 = app.body.as_ref().unwrap().yaw;
    app.on_turn_key(1.0, 0.0, 0.5);
    let y4 = app.body.as_ref().unwrap().yaw;
    assert_eq!(
        y4 - y3,
        90.0,
        "the keyboard lane's own rate the slider never scales"
    );
}

/// THE FOV SLIDER'S OWN LAW (the `options.fov` OptionInstance the
/// 26.2 clinit banks — the IntRange(30, 110) the default 70 wears,
/// the caption the ends law speaks (the 30 the options.fov.min
/// 'Normal' row, the 110 the options.fov.max 'Quake Pro' row, the
/// between the raw int), the press the IntRange's own step lands,
/// the armed drag the track's outside rides, the release the
/// disarm speaks).
#[test]
fn the_fov_slider_wears_the_range_and_caption_law() {
    let mut app = App::new(3);
    assert_eq!(app.fov, 70.0, "the default the IntRange's own 70 wears");
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let b = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov")
        .expect("the slider row")
        .clone();
    assert_eq!(b.label, "FOV: 70", "the default the raw-int caption speaks");
    assert_eq!(b.value, Some(0.5), "the 70 the range's own mid wears");
    // THE CENTER PRESS (the same track math — the 1-degree step the
    // round keeps)
    app.on_press(b.x + b.w / 2, b.y + b.h / 2);
    assert_eq!(app.fov, 70.0, "the center the 70 keeps");
    assert!(app.fov_drag, "the press inside the track arms");
    // THE RIGHT EDGE (the 110 the clamp keeps — the Quake Pro end)
    app.on_press(b.x + b.w - 1, b.y + b.h / 2);
    assert_eq!(app.fov, 110.0, "the right edge the 110 keeps");
    app.on_sensitivity_slider_drag(0);
    app.on_fov_slider_drag(b.x + b.w + 60);
    assert_eq!(app.fov, 110.0, "the beyond-right drag clamps");
    app.on_left_release();
    assert!(!app.fov_drag, "the release disarms");
    app.on_fov_slider_drag(b.x + 4);
    assert_eq!(app.fov, 110.0, "the disarmed drag never moves");
    app.render_title_buttons_for_test();
    let b = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov")
        .expect("the slider row")
        .clone();
    assert_eq!(b.label, "FOV: Quake Pro");
    assert_eq!(b.value, Some(1.0));
    // THE LEFT EDGE (the 30 the clamp keeps — the Normal end)
    app.on_press(b.x + 4, b.y + b.h / 2);
    assert_eq!(app.fov, 30.0, "the left edge the 30 keeps");
    app.on_left_release();
    app.render_title_buttons_for_test();
    let b = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov")
        .expect("the slider row")
        .clone();
    assert_eq!(b.label, "FOV: Normal");
    assert_eq!(b.value, Some(0.0));
}

/// THE CAMERA FOLD'S OWN LAW (the CAMERA.calculateFov walk — the
/// store's degrees the body's modifier scales, the default 70 the
/// bit-exact product keeps, the sprint's 1.15 the wider arc the
/// frame speaks, the sneak's 0.85 the narrower walk keeps; the
/// IntRange round-trip the slider's own math banks).
#[test]
fn the_camera_fold_wears_the_stored_fov() {
    // THE DEFAULT'S BIT-EXACT PRODUCT (the ×1.0 the legacy literal
    // the store wears — the golden seat the same degrees keeps)
    assert_eq!(calculate_fov_degrees(70.0, 1.0), 70.0);
    assert_eq!(calculate_fov_degrees(110.0, 1.0), 110.0);
    assert_eq!(calculate_fov_degrees(30.0, 1.0), 30.0);
    // THE MODIFIER'S OWN RIDE (the sprint/sneak lerp the fold
    // scales — the f32 product the approx law asserts)
    let sprint = calculate_fov_degrees(70.0, 1.15);
    assert!(
        (sprint - 80.5).abs() < 1e-2,
        "the sprint's 1.15 the wider arc speaks"
    );
    let sneak = calculate_fov_degrees(70.0, 0.85);
    assert!(
        (sneak - 59.5).abs() < 1e-2,
        "the sneak's 0.85 the narrower walk keeps"
    );
    // THE INT RANGE'S OWN ROUND-TRIP (the slider's own math — the
    // 30 the value's 0.0 wears, the 110 the 1.0 wears, the step
    // the round keeps)
    assert_eq!(fov_value(30.0), 0.0);
    assert_eq!(fov_value(110.0), 1.0);
    assert_eq!(fov_value(70.0), 0.5);
    assert_eq!(fov_from_value(0.0), 30.0);
    assert_eq!(fov_from_value(1.0), 110.0);
    assert_eq!(fov_from_value(0.5), 70.0);
    assert_eq!(fov_from_value(0.51), 71.0, "the 1-degree step rounds");
}

/// THE VOLUME SLIDER'S OWN LAW (the "Music & Sounds" master the
/// Options row speaks — the 0..1 the value IS, the caption the
/// percent answers (the 0.0 the options.off 'OFF' end wears, the
/// default 1.0 the 'Music & Sounds: 100%' word keeps), the press
/// the track math lands, the armed drag the track's outside rides,
/// the release the disarm speaks; the OUTPUT'S OWN GAIN the
/// deterministic stand-in records — the master the every lane
/// scales).
#[test]
fn the_volume_slider_wears_the_caption_and_gain_law() {
    let mut app = App::new(3);
    assert_eq!(app.volume, 1.0, "the default the master's 1.0 wears");
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let b = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_music")
        .expect("the slider row")
        .clone();
    assert_eq!(
        b.label, "Music & Sounds: 100%",
        "the default the master's own percent speaks"
    );
    assert_eq!(b.value, Some(1.0));
    // THE CENTER PRESS (the 0..1 the value IS — the mid-track the
    // 0.5 keeps)
    app.on_press(b.x + b.w / 2, b.y + b.h / 2);
    assert_eq!(app.volume, 0.5, "the center the 0.5 keeps");
    assert!(app.volume_drag, "the press inside the track arms");
    // THE RIGHT EDGE (the 1.0 the clamp keeps)
    app.on_press(b.x + b.w - 1, b.y + b.h / 2);
    assert_eq!(app.volume, 1.0, "the right edge clamps");
    app.on_volume_slider_drag(b.x + b.w + 60);
    assert_eq!(app.volume, 1.0, "the beyond-right drag clamps");
    // THE RELEASE'S OWN DISARM (the mouseReleased walk — the
    // disarmed drag the silence keeps)
    app.on_left_release();
    assert!(!app.volume_drag, "the release disarms");
    app.on_volume_slider_drag(b.x + 4);
    assert_eq!(app.volume, 1.0, "the disarmed drag never moves");
    // THE MID CAPTION (the percent the 0.5 speaks)
    app.on_press(b.x + b.w / 2, b.y + b.h / 2);
    app.on_left_release();
    app.render_title_buttons_for_test();
    let b = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_music")
        .expect("the slider row")
        .clone();
    assert_eq!(b.label, "Music & Sounds: 50%");
    assert_eq!(b.value, Some(0.5));
    // THE ZERO END (the options.off 'OFF' the 0.0 wears)
    app.on_press(b.x + 4, b.y + b.h / 2);
    assert_eq!(app.volume, 0.0, "the left edge clamps");
    app.on_left_release();
    app.render_title_buttons_for_test();
    let b = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_music")
        .expect("the slider row")
        .clone();
    assert_eq!(b.label, "Music & Sounds: OFF");
    // THE OUTPUT'S OWN RECORD (the deterministic stand-in the
    // audible path's sink gain mirrors — the master the set lane
    // stores)
    use crate::audio::AudioOutput;
    let mut out = crate::audio::SilentOutput::default();
    out.set_volume(0.25);
    assert_eq!(
        out.volume(),
        0.25,
        "the master the output's own record wears"
    );
}
// THE OPTIONS FILE'S OWN ROUND TRIP (the persistence law — the vanilla
// `Options.save()`/`load()` walks the stores' restart outlives: the Done
// chord the file's rows become, the fresh boot the file's truth wears,
// the missing file the defaults keep)
#[test]
fn the_options_file_the_stores_restart_outlives() {
    let dir = std::env::temp_dir().join(format!("mc2rs_options_stores_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE MISSING FILE the defaults keep (the exists() gate the load
    // head wears — no file, no guess)
    app.options_file_load();
    assert_eq!(app.look_sensitivity, 0.5, "the default the load keeps");
    assert_eq!(app.fov, 70.0, "the default the load keeps");
    assert_eq!(app.volume, 1.0, "the default the load keeps");
    // THE DRAGS THE STORES WEAR (the sensitivity row's right-edge press
    // the 1.0 the track law clamps; the fov/volume stores the same
    // slider rows' own drags wear — the caption/drag laws the sibling
    // tests keep)
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let sens = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_sensitivity")
        .expect("the slider row")
        .clone();
    app.on_press(sens.x + sens.w - 1, sens.y + sens.h / 2);
    assert_eq!(app.look_sensitivity, 1.0, "the right edge clamps");
    app.on_left_release();
    app.fov = 110.0;
    app.volume = 0.0;
    // THE DONE CHORD'S OWN SAVE (the vanilla OptionsScreen.onClose
    // moment — the "back" button the sliders' drags the file's rows
    // become, the return the options_return speaks)
    let back = app
        .buttons
        .iter()
        .find(|b| b.id == "back")
        .expect("the Done row")
        .clone();
    app.on_click(back.x + back.w / 2, back.y + back.h / 2);
    assert_eq!(app.screen, ScreenKind::Title, "the Done the return wears");
    let text = std::fs::read_to_string(&path).expect("the chord the file writes");
    assert_eq!(
        text, "version:4903\nshowSubtitles:false\nmouseSensitivity:1.0\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:110\nsoundCategory_master:0.0\nadvancedItemTooltips:false\npauseOnLostFocus:true\nautoJump:false\noperatorItemsTab:false\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\nnotificationDisplayTime:1.0\nkey_key.forward:key.keyboard.w\nkey_key.left:key.keyboard.a\nkey_key.back:key.keyboard.s\nkey_key.right:key.keyboard.d\nkey_key.jump:key.keyboard.space\nkey_key.sneak:key.keyboard.left.shift\nkey_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.e\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\nkey_key.debug.copyRecreateCommand:key.keyboard.i\nkey_key.debug.spectate:key.keyboard.n\nkey_key.debug.switchGameMode:key.keyboard.f4\nkey_key.debug.debugOptions:key.keyboard.f6\nkey_key.debug.focusPause:key.keyboard.p\nkey_key.debug.dumpDynamicTextures:key.keyboard.s\nkey_key.debug.reloadResourcePacks:key.keyboard.t\nkey_key.debug.profiling:key.keyboard.l\nkey_key.debug.copyLocation:key.keyboard.c\nkey_key.debug.dumpVersion:key.keyboard.v\nkey_key.debug.profilingChart:key.keyboard.1\nkey_key.debug.fpsCharts:key.keyboard.2\nkey_key.debug.networkCharts:key.keyboard.3\nkey_key.debug.lightmapTexture:key.keyboard.4\n",
        "the file the vanilla rows wear"
    );
    // THE RESTART (the fresh boot the file's truth wears — the session's
    // drags the file the restart outlives)
    let mut app2 = App::new(262);
    app2.options_file = Some(path.clone());
    app2.options_file_load();
    assert_eq!(app2.look_sensitivity, 1.0, "the drag the reload wears");
    assert_eq!(app2.fov, 110.0, "the dial the reload wears");
    assert_eq!(app2.volume, 0.0, "the mute the reload wears");
    let _ = std::fs::remove_dir_all(&dir);
}

// THE OPTIONS ESCAPE'S OWN CHORD (the vanilla Escape lane — the
// Screen.shouldCloseOnEsc default the keyPressed walk answers, the
// SAME onClose moment the Done chord wears: the options.save() the
// file's rows keep, the lastScreen the return speaks)
#[test]
fn the_options_escape_the_save_and_return_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_options_escape_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE TITLE LANE (the title screen's own Options — the lastScreen
    // the Title the return speaks)
    app.options_return = ScreenKind::Title;
    app.open(ScreenKind::Options);
    app.look_sensitivity = 0.75;
    app.fov = 90.0;
    app.volume = 0.25;
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::Title, "the escape the return wears");
    let text = std::fs::read_to_string(&path).expect("the escape the file writes");
    assert_eq!(
        text, "version:4903\nshowSubtitles:false\nmouseSensitivity:0.75\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:90\nsoundCategory_master:0.25\nadvancedItemTooltips:false\npauseOnLostFocus:true\nautoJump:false\noperatorItemsTab:false\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\nnotificationDisplayTime:1.0\nkey_key.forward:key.keyboard.w\nkey_key.left:key.keyboard.a\nkey_key.back:key.keyboard.s\nkey_key.right:key.keyboard.d\nkey_key.jump:key.keyboard.space\nkey_key.sneak:key.keyboard.left.shift\nkey_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.e\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\nkey_key.debug.copyRecreateCommand:key.keyboard.i\nkey_key.debug.spectate:key.keyboard.n\nkey_key.debug.switchGameMode:key.keyboard.f4\nkey_key.debug.debugOptions:key.keyboard.f6\nkey_key.debug.focusPause:key.keyboard.p\nkey_key.debug.dumpDynamicTextures:key.keyboard.s\nkey_key.debug.reloadResourcePacks:key.keyboard.t\nkey_key.debug.profiling:key.keyboard.l\nkey_key.debug.copyLocation:key.keyboard.c\nkey_key.debug.dumpVersion:key.keyboard.v\nkey_key.debug.profilingChart:key.keyboard.1\nkey_key.debug.fpsCharts:key.keyboard.2\nkey_key.debug.networkCharts:key.keyboard.3\nkey_key.debug.lightmapTexture:key.keyboard.4\n",
        "the SAME save chord the Done wears the escape speaks"
    );
    // THE PAUSED LANE (the pause screen's own Options — the
    // options_return the World the pause overlay's shape keeps, the
    // world the escape returns the pause still wears)
    app.paused = true;
    app.options_return = ScreenKind::World;
    app.open(ScreenKind::Options);
    app.look_sensitivity = 0.9;
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the paused the world return wears"
    );
    assert!(app.paused, "the pause the world still wears");
    let text = std::fs::read_to_string(&path).expect("the second chord the file writes");
    assert!(
        text.contains("mouseSensitivity:0.9"),
        "the second drag the file's rows keep"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// THE SCREEN TAKEOVER'S OWN DRAG DEATH (the removed() walk the open()
// banks — the mid-drag Escape the armed flag dies with the screen, the
// re-open the pressless drag the stale arm never wears)
#[test]
fn the_screen_takeover_the_slider_drag_kills() {
    let mut app = App::new(262);
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let b = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_sensitivity")
        .expect("the slider row")
        .clone();
    app.on_press(b.x + b.w / 2, b.y + b.h / 2);
    assert!(app.sensitivity_drag, "the press the drag arms");
    assert_eq!(
        app.look_sensitivity, 0.5,
        "the center press the value wears"
    );
    // THE MID-DRAG ESCAPE (the takeover the death walk answers — the
    // armed flag the screen instance carried dies with the open())
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::Title, "the escape the return wears");
    assert!(!app.sensitivity_drag, "the takeover the drag kills");
    assert!(
        !app.fov_drag && !app.volume_drag,
        "the siblings the same death wear"
    );
    // THE RE-OPEN (the fresh screen the pressless drag refuses — the
    // stale arm the rebuild never inherits)
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    app.on_sensitivity_slider_drag(12345);
    assert_eq!(
        app.look_sensitivity, 0.5,
        "the pressless drag the value never moves"
    );
}

// THE QUIT LANE'S OWN STOP CHORD (the vanilla Title screen's quit
// button — the lambda$init$5 the `Minecraft.stop()` call answers, the
// running flag the run loop reads, the window's close the exit speaks.
// THE TRUE-ABSENCE THE RECON BANKS: the 26.2 Minecraft.close() walk the
// Options.save NEVER wears — the save's own sites the ctor boot, the
// onGameLoadFinished, the clearResourcePacksOnError, the
// onFullscreenError, the tick's joinedFirstServer, the toggleFullscreen
// speak; the shutdown walk the second write refuses — the options.txt
// the OptionsScreen.removed() moment ALONE keeps, the quit lane the
// file never touches. THE autopush-171 WALK THE MOMENT ITSELF WEARS —
// the open()'s own head the save speaks, the takeover's exit the rows
// bank, the quit the SECOND write still refuses)
#[test]
fn the_quit_lane_the_stop_flag_and_the_files_absence_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_quit_stop_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    app.open(ScreenKind::Title);
    app.render_title_buttons_for_test();
    let quit = app
        .buttons
        .iter()
        .find(|b| b.id == "quit")
        .expect("the Quit row")
        .clone();
    app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
    assert!(app.stopping, "the stop the quit arm speaks");
    assert_eq!(app.screen, ScreenKind::Title, "the exit the screen keeps");
    assert!(
        !path.exists(),
        "the quit the file NEVER writes — the OptionsScreen's own removed() moment the rows keep"
    );
    // THE REMOVED MOMENT'S OWN BANK (the autopush-171 walk — the
    // Options screen's exit the open() itself answers: the takeover's
    // screen-replacement lane the file's rows keep, the quit lane the
    // SECOND write the law still refuses)
    app.stopping = false;
    app.open(ScreenKind::Options);
    app.look_sensitivity = 0.8;
    app.open(ScreenKind::Title);
    assert_eq!(
        std::fs::read_to_string(&path).expect("the removed() moment the file writes"),
        "version:4903\nshowSubtitles:false\nmouseSensitivity:0.8\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:true\nautoJump:false\noperatorItemsTab:false\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\nnotificationDisplayTime:1.0\nkey_key.forward:key.keyboard.w\nkey_key.left:key.keyboard.a\nkey_key.back:key.keyboard.s\nkey_key.right:key.keyboard.d\nkey_key.jump:key.keyboard.space\nkey_key.sneak:key.keyboard.left.shift\nkey_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.e\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\nkey_key.debug.copyRecreateCommand:key.keyboard.i\nkey_key.debug.spectate:key.keyboard.n\nkey_key.debug.switchGameMode:key.keyboard.f4\nkey_key.debug.debugOptions:key.keyboard.f6\nkey_key.debug.focusPause:key.keyboard.p\nkey_key.debug.dumpDynamicTextures:key.keyboard.s\nkey_key.debug.reloadResourcePacks:key.keyboard.t\nkey_key.debug.profiling:key.keyboard.l\nkey_key.debug.copyLocation:key.keyboard.c\nkey_key.debug.dumpVersion:key.keyboard.v\nkey_key.debug.profilingChart:key.keyboard.1\nkey_key.debug.fpsCharts:key.keyboard.2\nkey_key.debug.networkCharts:key.keyboard.3\nkey_key.debug.lightmapTexture:key.keyboard.4\n",
        "the takeover's own exit the rows bank"
    );
    app.render_title_buttons_for_test();
    let quit2 = app
        .buttons
        .iter()
        .find(|b| b.id == "quit")
        .expect("the Quit row")
        .clone();
    app.on_click(quit2.x + quit2.w / 2, quit2.y + quit2.h / 2);
    assert!(app.stopping, "the second stop the same chord wears");
    assert_eq!(
        std::fs::read_to_string(&path).expect("the file the exit keeps"),
        "version:4903\nshowSubtitles:false\nmouseSensitivity:0.8\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:true\nautoJump:false\noperatorItemsTab:false\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\nnotificationDisplayTime:1.0\nkey_key.forward:key.keyboard.w\nkey_key.left:key.keyboard.a\nkey_key.back:key.keyboard.s\nkey_key.right:key.keyboard.d\nkey_key.jump:key.keyboard.space\nkey_key.sneak:key.keyboard.left.shift\nkey_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.e\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\nkey_key.debug.copyRecreateCommand:key.keyboard.i\nkey_key.debug.spectate:key.keyboard.n\nkey_key.debug.switchGameMode:key.keyboard.f4\nkey_key.debug.debugOptions:key.keyboard.f6\nkey_key.debug.focusPause:key.keyboard.p\nkey_key.debug.dumpDynamicTextures:key.keyboard.s\nkey_key.debug.reloadResourcePacks:key.keyboard.t\nkey_key.debug.profiling:key.keyboard.l\nkey_key.debug.copyLocation:key.keyboard.c\nkey_key.debug.dumpVersion:key.keyboard.v\nkey_key.debug.profilingChart:key.keyboard.1\nkey_key.debug.fpsCharts:key.keyboard.2\nkey_key.debug.networkCharts:key.keyboard.3\nkey_key.debug.lightmapTexture:key.keyboard.4\n",
        "the quit the file NEVER re-writes — the removed() moment ALONE the rows keep"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// THE NONE BOOT'S OWN QUIT (the headless goldens' own shape — the
// options_file None the save walk the silent keep answers, the stop
// flag the lane still arms, the no-panic the absence speaks)
#[test]
fn the_none_boot_the_quit_still_stops() {
    let mut app = App::new(262);
    assert!(
        app.options_file.is_none(),
        "the headless boot the file keeps"
    );
    app.open(ScreenKind::Title);
    app.render_title_buttons_for_test();
    let quit = app
        .buttons
        .iter()
        .find(|b| b.id == "quit")
        .expect("the Quit row")
        .clone();
    app.on_click(quit.x + quit.w / 2, quit.y + quit.h / 2);
    assert!(app.stopping, "the None boot the stop still wears");
    assert_eq!(app.screen, ScreenKind::Title, "the exit the screen keeps");
}

// THE REMOVED MOMENT'S OWN WALK (the autopush-171 law — the vanilla
// OptionsScreen.removed() pc 4 the Options.save call the open()'s own
// head now wears: the Done AND the Escape AND the takeover's own
// screen-replacement lane the SAME moment keep — the save's seat the
// open() walk itself is, the explicit arms' chords the walk retires)
#[test]
fn the_options_exit_the_removed_moment_itself_saves() {
    let dir = std::env::temp_dir().join(format!("mc2rs_removed_save_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE TAKEOVER (the screen-replacement lane the Done/Escape arms
    // never speak — the open() walk the old Options screen's
    // discarded() moment answers: the rows the replacement banks)
    app.open(ScreenKind::Title);
    app.open(ScreenKind::Options);
    app.look_sensitivity = 0.65;
    app.fov = 85.0;
    app.open(ScreenKind::Servers);
    assert_eq!(
        std::fs::read_to_string(&path).expect("the takeover the file writes"),
        "version:4903\nshowSubtitles:false\nmouseSensitivity:0.65\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:85\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:true\nautoJump:false\noperatorItemsTab:false\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\nnotificationDisplayTime:1.0\nkey_key.forward:key.keyboard.w\nkey_key.left:key.keyboard.a\nkey_key.back:key.keyboard.s\nkey_key.right:key.keyboard.d\nkey_key.jump:key.keyboard.space\nkey_key.sneak:key.keyboard.left.shift\nkey_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.e\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\nkey_key.debug.copyRecreateCommand:key.keyboard.i\nkey_key.debug.spectate:key.keyboard.n\nkey_key.debug.switchGameMode:key.keyboard.f4\nkey_key.debug.debugOptions:key.keyboard.f6\nkey_key.debug.focusPause:key.keyboard.p\nkey_key.debug.dumpDynamicTextures:key.keyboard.s\nkey_key.debug.reloadResourcePacks:key.keyboard.t\nkey_key.debug.profiling:key.keyboard.l\nkey_key.debug.copyLocation:key.keyboard.c\nkey_key.debug.dumpVersion:key.keyboard.v\nkey_key.debug.profilingChart:key.keyboard.1\nkey_key.debug.fpsCharts:key.keyboard.2\nkey_key.debug.networkCharts:key.keyboard.3\nkey_key.debug.lightmapTexture:key.keyboard.4\n",
        "the replacement lane the removed() moment speaks"
    );
    // THE SAME-KIND RE-ENTRY (the open(Options) from Options the
    // vanilla removed() walk the old instance's discard answers — the
    // re-init the save still wears, the vanilla setScreen's own law)
    app.open(ScreenKind::Options);
    app.volume = 0.5;
    app.open(ScreenKind::Options);
    assert_eq!(
        std::fs::read_to_string(&path).expect("the re-entry the file writes"),
        "version:4903\nshowSubtitles:false\nmouseSensitivity:0.65\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:85\nsoundCategory_master:0.5\nadvancedItemTooltips:false\npauseOnLostFocus:true\nautoJump:false\noperatorItemsTab:false\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\nnotificationDisplayTime:1.0\nkey_key.forward:key.keyboard.w\nkey_key.left:key.keyboard.a\nkey_key.back:key.keyboard.s\nkey_key.right:key.keyboard.d\nkey_key.jump:key.keyboard.space\nkey_key.sneak:key.keyboard.left.shift\nkey_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.e\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\nkey_key.debug.copyRecreateCommand:key.keyboard.i\nkey_key.debug.spectate:key.keyboard.n\nkey_key.debug.switchGameMode:key.keyboard.f4\nkey_key.debug.debugOptions:key.keyboard.f6\nkey_key.debug.focusPause:key.keyboard.p\nkey_key.debug.dumpDynamicTextures:key.keyboard.s\nkey_key.debug.reloadResourcePacks:key.keyboard.t\nkey_key.debug.profiling:key.keyboard.l\nkey_key.debug.copyLocation:key.keyboard.c\nkey_key.debug.dumpVersion:key.keyboard.v\nkey_key.debug.profilingChart:key.keyboard.1\nkey_key.debug.fpsCharts:key.keyboard.2\nkey_key.debug.networkCharts:key.keyboard.3\nkey_key.debug.lightmapTexture:key.keyboard.4\n",
        "the same-kind walk the old screen's discard keeps"
    );
    // THE NONE BOOT'S SILENT KEEP (the options_file None the save walk
    // the silent keep answers — the takeover no-panic the absence
    // speaks, the in-memory stores the truth stays)
    let mut app2 = App::new(262);
    app2.open(ScreenKind::Options);
    app2.look_sensitivity = 0.9;
    app2.open(ScreenKind::Title);
    assert_eq!(app2.look_sensitivity, 0.9, "the None boot the memory keeps");
    let _ = std::fs::remove_dir_all(&dir);
}

// THE FOCUS-LOSS'S OWN PAUSE (the vanilla pauseIfInactive law — the
// renderFrame's per-frame call the 26.2 bytecode banks: the focused
// walk the timestamp refresh speaks, the unfocused walk the 500ms
// grace the elapsed answers, the pauseGame(false) the FULL pause menu
// the PauseScreen's !arg ctor wears — the ESC walk's own pause the
// same shape keeps, the death the modal law refuses)
#[test]
fn the_focus_loss_the_grace_and_the_pause_wears() {
    let mut app = App::new(262);
    app.enter_world();
    app.spawn_offline_body();
    assert_eq!(app.screen, ScreenKind::World);
    assert!(!app.paused, "the live world the pause never wears");
    // THE ACTIVE WALK (the focus the timestamp refresh speaks — the
    // grace's own seat the active frames keep)
    app.on_window_focus(true, 0);
    // THE UNFOCUSED GRACE (the 500ms the vanilla law wears — the
    // flick the pause never speaks)
    app.on_window_focus(false, 300);
    assert!(!app.paused, "the grace the flick never pauses");
    app.on_window_focus(false, 500);
    assert!(
        !app.paused,
        "the grace the boundary keeps — > 500 the law alone fires"
    );
    // THE EXPIRY (the elapsed the 500ms gate answers — the pause the
    // full menu's own walk wears)
    app.on_window_focus(false, 501);
    assert!(app.paused, "the expiry the pause speaks");
    assert_eq!(app.screen, ScreenKind::World, "the pause the world keeps");
    // THE RE-FIRE'S OWN IDEMPOTENCE (the vanilla setPauseScreen's own
    // first check the screen-up walk answers — the paused gate the
    // re-fire eats, the buttons the clear never re-wears)
    app.on_window_focus(false, 2000);
    assert!(app.paused, "the re-fire the idempotence keeps");
    // THE REFOCUS (the timestamp the active walk refreshes — the pause
    // the user's own ESC/backtogame lane alone lifts, the refocus the
    // pause never touches)
    app.on_window_focus(true, 2100);
    assert!(app.paused, "the refocus the pause never lifts");
    // THE OPTION'S OWN FALSE ARM (the pauseOnLostFocus false the walk
    // the refresh alone speaks — the unfocused stretch the pause never
    // wears)
    app.paused = false;
    app.pause_on_lost_focus = false;
    app.on_window_focus(false, 100_000);
    assert!(!app.paused, "the option's false the pause refuses");
    // THE REFRESH THE FALSE ARM SPEAKS (the re-arm the next true
    // walk's grace the fresh timestamp carries — the option's flip
    // the mid-unfocus the grace re-arms)
    app.pause_on_lost_focus = true;
    app.on_window_focus(false, 100_100);
    assert!(!app.paused, "the fresh timestamp the grace re-arms");
    app.on_window_focus(false, 100_601);
    assert!(app.paused, "the re-armed grace the expiry fires");
}

// THE GATES' OWN SEATS (the pause the walk's own gates wear — the
// title screen no world the pause never wears, the dead the ESC arm's
// own modal law the focus walk keeps)
#[test]
fn the_focus_pause_the_gates_the_screen_and_death_wear() {
    let mut app = App::new(262);
    // THE TITLE SEAT (no world — the gate the screen keeps)
    app.open(ScreenKind::Title);
    app.on_window_focus(false, 10_000);
    assert!(!app.paused, "the title seat the pause never wears");
    // THE DEATH GATE (the world the dead modal law keeps — the
    // Respawn/Title buttons the only exits the focus walk respects)
    let mut app2 = App::new(262);
    app2.enter_world();
    app2.spawn_offline_body();
    app2.dead = true;
    app2.on_window_focus(false, 10_000);
    assert!(!app2.paused, "the dead the pause never wears");
    // THE PAUSED GATE (the already-paused walk the idempotence keeps —
    // the buttons the clear never re-wears)
    let mut app3 = App::new(262);
    app3.enter_world();
    app3.spawn_offline_body();
    app3.on_escape();
    assert!(app3.paused, "the ESC the pause wears");
    app3.buttons.push(crate::screens::Button::new(
        "backtogame",
        "Back to Game".to_string(),
        10,
        10,
        100,
    ));
    app3.on_window_focus(false, 10_000);
    assert!(app3.paused, "the paused the walk never touches");
    assert_eq!(
        app3.buttons.len(),
        1,
        "the buttons the re-fire never clears"
    );
}

// THE FOCUS-PAUSE TOGGLE'S OWN CHORD (the 26.2 debug entry the
// key.debug.focusPause binding wears — the F3+P chord the flip
// serves: the option the memory flips, the file's row the removed()
// moment's own save rides, the boot load the row's truth wears)
#[test]
fn the_focus_pause_chord_the_flip_and_the_rows_wear() {
    let dir = std::env::temp_dir().join(format!("mc2rs_focus_pause_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    app.enter_world();
    app.spawn_offline_body();
    // THE DEFAULT (the ctor's own true the option wears)
    assert!(app.pause_on_lost_focus, "the default the ctor speaks");
    // THE F3+P FLIP (the debug entry the chord toggles — the modifier
    // flag the family's own shape wears)
    app.on_focus_pause_key();
    assert!(!app.pause_on_lost_focus, "the chord the flip speaks");
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    // THE FILE'S ROW (the Options exit the removed() moment the save
    // rides — the false the round-trip's own row wears)
    app.open(ScreenKind::Options);
    app.open(ScreenKind::Title);
    assert_eq!(
        std::fs::read_to_string(&path).expect("the exit the file writes"),
        "version:4903\nshowSubtitles:false\nmouseSensitivity:0.5\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:false\nautoJump:false\noperatorItemsTab:false\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\nnotificationDisplayTime:1.0\nkey_key.forward:key.keyboard.w\nkey_key.left:key.keyboard.a\nkey_key.back:key.keyboard.s\nkey_key.right:key.keyboard.d\nkey_key.jump:key.keyboard.space\nkey_key.sneak:key.keyboard.left.shift\nkey_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.e\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\nkey_key.debug.copyRecreateCommand:key.keyboard.i\nkey_key.debug.spectate:key.keyboard.n\nkey_key.debug.switchGameMode:key.keyboard.f4\nkey_key.debug.debugOptions:key.keyboard.f6\nkey_key.debug.focusPause:key.keyboard.p\nkey_key.debug.dumpDynamicTextures:key.keyboard.s\nkey_key.debug.reloadResourcePacks:key.keyboard.t\nkey_key.debug.profiling:key.keyboard.l\nkey_key.debug.copyLocation:key.keyboard.c\nkey_key.debug.dumpVersion:key.keyboard.v\nkey_key.debug.profilingChart:key.keyboard.1\nkey_key.debug.fpsCharts:key.keyboard.2\nkey_key.debug.networkCharts:key.keyboard.3\nkey_key.debug.lightmapTexture:key.keyboard.4\n",
        "the flipped row the exit banks"
    );
    // THE SECOND FLIP (the chord the option re-arms — the row the next
    // exit the true re-wears)
    app.on_focus_pause_key();
    assert!(app.pause_on_lost_focus, "the second flip the true re-arms");
    app.open(ScreenKind::Options);
    app.open(ScreenKind::Title);
    assert_eq!(
        std::fs::read_to_string(&path).expect("the second exit the file writes"),
        "version:4903\nshowSubtitles:false\nmouseSensitivity:0.5\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:true\nautoJump:false\noperatorItemsTab:false\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\nnotificationDisplayTime:1.0\nkey_key.forward:key.keyboard.w\nkey_key.left:key.keyboard.a\nkey_key.back:key.keyboard.s\nkey_key.right:key.keyboard.d\nkey_key.jump:key.keyboard.space\nkey_key.sneak:key.keyboard.left.shift\nkey_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.e\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\nkey_key.debug.copyRecreateCommand:key.keyboard.i\nkey_key.debug.spectate:key.keyboard.n\nkey_key.debug.switchGameMode:key.keyboard.f4\nkey_key.debug.debugOptions:key.keyboard.f6\nkey_key.debug.focusPause:key.keyboard.p\nkey_key.debug.dumpDynamicTextures:key.keyboard.s\nkey_key.debug.reloadResourcePacks:key.keyboard.t\nkey_key.debug.profiling:key.keyboard.l\nkey_key.debug.copyLocation:key.keyboard.c\nkey_key.debug.dumpVersion:key.keyboard.v\nkey_key.debug.profilingChart:key.keyboard.1\nkey_key.debug.fpsCharts:key.keyboard.2\nkey_key.debug.networkCharts:key.keyboard.3\nkey_key.debug.lightmapTexture:key.keyboard.4\n",
        "the re-armed row the exit banks"
    );
    // THE THIRD FLIP (the option the false re-wears — the restart the
    // flip outlives)
    app.on_focus_pause_key();
    assert!(!app.pause_on_lost_focus, "the third flip the false wears");
    app.open(ScreenKind::Options);
    app.open(ScreenKind::Title);
    assert_eq!(
        std::fs::read_to_string(&path).expect("the third exit the file writes"),
        "version:4903\nshowSubtitles:false\nmouseSensitivity:0.5\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:false\nautoJump:false\noperatorItemsTab:false\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\nnotificationDisplayTime:1.0\nkey_key.forward:key.keyboard.w\nkey_key.left:key.keyboard.a\nkey_key.back:key.keyboard.s\nkey_key.right:key.keyboard.d\nkey_key.jump:key.keyboard.space\nkey_key.sneak:key.keyboard.left.shift\nkey_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.e\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\nkey_key.debug.copyRecreateCommand:key.keyboard.i\nkey_key.debug.spectate:key.keyboard.n\nkey_key.debug.switchGameMode:key.keyboard.f4\nkey_key.debug.debugOptions:key.keyboard.f6\nkey_key.debug.focusPause:key.keyboard.p\nkey_key.debug.dumpDynamicTextures:key.keyboard.s\nkey_key.debug.reloadResourcePacks:key.keyboard.t\nkey_key.debug.profiling:key.keyboard.l\nkey_key.debug.copyLocation:key.keyboard.c\nkey_key.debug.dumpVersion:key.keyboard.v\nkey_key.debug.profilingChart:key.keyboard.1\nkey_key.debug.fpsCharts:key.keyboard.2\nkey_key.debug.networkCharts:key.keyboard.3\nkey_key.debug.lightmapTexture:key.keyboard.4\n",
        "the flipped row the exit banks"
    );
    // THE BOOT LOAD (the file's truth the boot walk wears — the false
    // row the restart outlives, the vanilla Options.load law the same
    // walk the three stores' restart keep)
    let mut app2 = App::new(262);
    app2.options_file = Some(path.clone());
    app2.options_file_load();
    assert!(
        !app2.pause_on_lost_focus,
        "the row's false the boot load wears"
    );
    // THE MISSING KEY (the older file's shape the default keeps — the
    // three-row file the vanilla's own default true answers)
    std::fs::write(
        &path,
        "version:4903\nmouseSensitivity:0.9\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:100\nsoundCategory_master:0.1\n",
    )
    .expect("the legacy file writes");
    let mut app3 = App::new(262);
    app3.options_file = Some(path.clone());
    app3.options_file_load();
    assert!(
        app3.pause_on_lost_focus,
        "the missing row the default true keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// THE CLEAR CHAT'S OWN CHORD (the 26.2 handleDebugKeys walk the
// keyDebugClearChat lane wears — the clearchat_lane_205.py probe the
// clearMessages(Z) body banks: the display rows the three
// unconditional clears speak, the recentChat arm the iload_1 ifeq
// gate TRUE alone serves — the false arg the F3+D lane passes the
// input history the up-arrow recall keeps)
#[test]
fn the_clear_chat_chord_the_rows_and_the_history_keep() {
    let mut app = App::new(262);
    app.enter_world();
    app.spawn_offline_body();
    // THE SEED (the display rows + the input history the pair fills —
    // the push walk the display queue serves, the history the
    // typed-rows store keeps)
    app.push_chat_message("hello world".to_string());
    app.push_chat_message("second row".to_string());
    app.add_recent_chat("/time set day".to_string());
    app.add_recent_chat("/weather clear".to_string());
    assert_eq!(
        app.chat_messages.len(),
        2,
        "the seeded rows the queue wears"
    );
    assert_eq!(
        app.recent_chat.len(),
        2,
        "the seeded history the store wears"
    );
    // THE CHORD (the F3+D lane the clear serves — the display rows
    // the walk never keeps, the modifier flag the family spends)
    app.on_clear_chat_key();
    assert!(
        app.chat_messages.is_empty(),
        "the chord the display rows clear"
    );
    assert_eq!(
        app.recent_chat.len(),
        2,
        "the false arg the input history keeps"
    );
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    // THE RELEASE (the bare F3 the flag re-arms — the overlay toggle
    // the spent chord never speaks)
    app.on_debug_modifier_released();
    assert!(!app.debug_modifier_used, "the release the flag consumes");
    // THE DRAW WALK (the empty queue the zero rows renders — the
    // chat draw the cleared queue the silent band serves)
    assert!(
        app.chat_messages.iter().rev().count() == 0,
        "the cleared queue the draw never feeds"
    );
}

// THE ADVANCED TOOLTIPS' OWN CHORD (the 26.2 handleDebugKeys walk the
// keyDebugShowAdvancedTooltips lane wears — the advanced_tooltips_168.py
// probe the pc 297-349 window banks: the options flag the flip serves,
// the debugFeedbackTranslated the yellow chat row speaks, the
// options.save() the debug family's own save-wearing lane calls — the
// immediate write the flip's own moment keeps)
#[test]
fn the_advanced_tooltips_chord_the_flip_the_row_and_the_word_wear() {
    let dir = std::env::temp_dir().join(format!("mc2rs_adv_tooltips_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    app.enter_world();
    app.spawn_offline_body();
    // THE DEFAULT (the Java field's own zero the decode's default arm
    // keeps — the ctor the putfield never wears)
    assert!(
        !app.advanced_item_tooltips,
        "the default the field's zero speaks"
    );
    // THE CHORD (the flip the lane serves — the save the chord's own
    // write wears, the yellow word the chat row speaks, the modifier
    // flag the family spends)
    app.on_advanced_tooltips_key();
    assert!(app.advanced_item_tooltips, "the chord the flip speaks");
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str())
            == Some("\u{a7}eAdvanced tooltips: shown"),
        "the yellow word the on row wears"
    );
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    let text = std::fs::read_to_string(&path).expect("the chord the file writes");
    assert!(
        text.contains("\nadvancedItemTooltips:true\n"),
        "the flip's own save the row wears"
    );
    assert!(
        text.contains(
            "soundCategory_master:1.0\nadvancedItemTooltips:true\npauseOnLostFocus:true\n"
        ),
        "the row the visit-order seat keeps"
    );
    // THE SECOND CHORD (the flip the false re-wears — the hidden word
    // the off row speaks, the file's row the false re-writes)
    app.on_debug_modifier_released();
    app.on_advanced_tooltips_key();
    assert!(
        !app.advanced_item_tooltips,
        "the second chord the false re-wears"
    );
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str())
            == Some("\u{a7}eAdvanced tooltips: hidden"),
        "the yellow word the off row wears"
    );
    let text = std::fs::read_to_string(&path).expect("the second chord the file writes");
    assert!(
        text.contains("\nadvancedItemTooltips:false\n"),
        "the false row the second save banks"
    );
    // THE BOOT LOAD (the file's truth the boot walk wears — the true
    // row the restart outlives, the vanilla processOptions decode the
    // same walk the stores' restart keeps)
    std::fs::write(
        &path,
        "version:4903\nadvancedItemTooltips:true\npauseOnLostFocus:true\n",
    )
    .expect("the file writes");
    let mut app2 = App::new(262);
    app2.options_file = Some(path.clone());
    app2.options_file_load();
    assert!(
        app2.advanced_item_tooltips,
        "the row's true the boot load wears"
    );
    // THE NONE FILE (the unbound path the chord the flip + the word
    // still serves — the save the None the silent no-op keeps)
    let mut app3 = App::new(262);
    app3.enter_world();
    app3.on_advanced_tooltips_key();
    assert!(
        app3.advanced_item_tooltips,
        "the None file the flip still speaks"
    );
    assert_eq!(app3.chat_messages.len(), 1, "the word the None file wears");
    // THE MISSING KEY (the older file's shape the default keeps)
    std::fs::write(
        &path,
        "version:4903\nmouseSensitivity:0.9\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:100\nsoundCategory_master:0.1\n",
    )
    .expect("the legacy file writes");
    let mut app4 = App::new(262);
    app4.options_file = Some(path.clone());
    app4.options_file_load();
    assert!(
        !app4.advanced_item_tooltips,
        "the missing row the default false keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// THE DEBUG FEEDBACK'S OWN WORDS (the 26.2 handleDebugKeys walk the
// debugFeedbackTranslated pairs wear — the feedback_words_206.py probe
// the cp strings + the lang rows banks: the hitboxes@193 the
// show_hitboxes.on/off pair, the chunkBorders@292 the
// chunk_boundaries.on/off pair, the focusPause@796 the
// pause_focus.on/off pair — the ChatFormatting.YELLOW the §e prefix
// the showDebugChat walk the dev-168 lane established wears)
#[test]
fn the_debug_feedback_words_the_family_wears() {
    let mut app = App::new(262);
    app.enter_world();
    app.spawn_offline_body();
    // THE BORDERS' PAIR (the F3+G flip the on word speaks, the
    // overlay-open second flip the off word answers — the
    // toggleStatus cycle the isOverlayVisible frame the NEVER arm
    // wears)
    app.on_chunk_borders_key();
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str()) == Some("\u{a7}eChunk borders: shown"),
        "the on word the borders flip speaks"
    );
    app.on_debug_key();
    app.on_chunk_borders_key();
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str()) == Some("\u{a7}eChunk borders: hidden"),
        "the off word the overlay-open flip answers"
    );
    assert_eq!(
        app.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY],
        crate::screens::DebugEntryStatus::Never,
        "the pair the flip restores"
    );
    app.on_debug_key();
    // THE HITBOXES' PAIR (the F3+B flip the same walk wears)
    app.on_hitboxes_key();
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str()) == Some("\u{a7}eHitboxes: shown"),
        "the on word the hitboxes flip speaks"
    );
    app.on_debug_key();
    app.on_hitboxes_key();
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str()) == Some("\u{a7}eHitboxes: hidden"),
        "the off word the overlay-open flip answers"
    );
    assert_eq!(
        app.debug_entry_states[crate::screens::ENTITY_HITBOXES_ENTRY],
        crate::screens::DebugEntryStatus::Never,
        "the pair the flip restores"
    );
    app.on_debug_key();
    // THE FOCUS-PAUSE'S PAIR (the F3+P flip the enabled word speaks —
    // the default true the flip the disabled word answers)
    app.on_focus_pause_key();
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str())
            == Some("\u{a7}ePause on lost focus: disabled"),
        "the disabled word the flip speaks"
    );
    app.on_focus_pause_key();
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str())
            == Some("\u{a7}ePause on lost focus: enabled"),
        "the enabled word the second flip answers"
    );
    assert!(app.pause_on_lost_focus, "the pair the flip restores");
    // THE QUEUE (the six words the chat rows wear — the count the
    // family's own feedback bank keeps)
    assert_eq!(app.chat_messages.len(), 6, "the six words the queue wears");
    assert!(
        app.debug_modifier_used,
        "the flag the last lane's spend wears"
    );
}

// THE COPY RECREATE COMMAND'S OWN CHORD (the 26.2 handleDebugKeys
// keyDebugCopyRecreateCommand lane the khcopy_169.py walk banks — the
// BLOCK arm the copyCreateBlockCommand builder serves: the
// `/setblock %d %d %d %s` format the Locale.ROOT walk speaks, the
// state the BlockStateParser.serialize the registry name the
// minecraft: prefix wears, the client word the debug.inspect.client
// .block key the en_us row `Copied client-side block data to
// clipboard` keeps)
#[test]
fn the_copy_recreate_chord_the_block_command_the_word_wear() {
    let mut app = App::new(262);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE COLUMN'S OWN PLANTING (the deterministic hit the surface's
    // own carve never promises — the stone the top-1 cell wears, the
    // air the walk-down the ray reads, the part_8 law the same
    // planting keeps)
    {
        let world = app.offline_world.as_mut().unwrap();
        world.set(c, top - 1, c, crate::world::STONE);
        world.set(c, top, c, crate::world::AIR);
    }
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    // THE CHORD (the block hit the lane's own walk serves — the
    // clipboard the command stores, the yellow word the chat speaks)
    app.on_copy_recreate_command_key();
    // THE COMMAND (the world's own hit the expected string builds —
    // the solid cell the serialize's own state reads)
    let hit = app.clip_crosshair().expect("the ground the down look hits");
    assert_eq!(
        hit.pos,
        (c, top - 1, c),
        "the planted stone the ray's own cell keeps"
    );
    assert_eq!(
        app.clipboard,
        format!(
            "/setblock {} {} {} minecraft:stone",
            hit.pos.0, hit.pos.1, hit.pos.2
        ),
        "the block command the format speaks"
    );
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str())
            == Some("\u{a7}eCopied client-side block data to clipboard"),
        "the client word the no-shift walk wears"
    );
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    let _ = app;
}

// THE COPY RECREATE'S OWN ENTITY ARM + SHIFT WORD (the ENTITY case
// the copyCreateEntityCommand builder serves — the `/summon %s %.2f
// %.2f %.2f` format the wire_name walk speaks, the shift the
// hasShiftDown arg the SERVER word the debug.inspect.server key
// answers, the UUID/Pos strips the null tag never runs)
#[test]
fn the_copy_recreate_chord_the_entity_summon_the_shift_word_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: -std::f32::consts::FRAC_PI_2,
    });
    let mut mob = crate::mob::Zombie::new(7, c as f64 + 0.5, top as f64, c as f64 + 0.5, 3);
    mob.kind = crate::mob::MobKind::Skeleton;
    app.mobs.push(mob);
    // THE SHIFT (the hasShiftDown the server arg wears — the word the
    // server-query arm speaks)
    app.on_shift_down(true);
    app.on_copy_recreate_command_key();
    assert_eq!(
        app.clipboard,
        format!(
            "/summon minecraft:skeleton {:.2} {:.2} {:.2}",
            c as f64 + 0.5,
            top as f64,
            c as f64 + 0.5
        ),
        "the summon command the entity builder speaks"
    );
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str())
            == Some("\u{a7}eCopied server-side entity data to clipboard"),
        "the server word the shift walk wears"
    );
    // THE UN-SHIFT (the second chord the client word answers — the
    // same entity the two words the arms split)
    app.on_shift_down(false);
    app.on_copy_recreate_command_key();
    assert!(
        app.chat_messages.last().map(|(t, _)| t.as_str())
            == Some("\u{a7}eCopied client-side entity data to clipboard"),
        "the client word the un-shift walk answers"
    );
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    let _ = app;
}

// THE COPY RECREATE'S OWN MISS (the hitResult the null return the
// body's first walk keeps — the silence the copy never serves, the
// istore_3 the used flag the call site banks REGARDLESS [the pc
// 418-419 the gated-out walk wears too])
#[test]
fn the_copy_recreate_chord_the_miss_silence_keeps() {
    let mut app = App::new(262);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    app.offline_camera_updated(crate::camera::Camera {
        pos: crate::math::Vec3::new(c as f32 + 0.5, top as f32 + 4.0, c as f32 + 0.5),
        yaw: 0.0,
        pitch: std::f32::consts::FRAC_PI_2,
    });
    app.on_copy_recreate_command_key();
    assert!(app.clipboard.is_empty(), "the miss the store never wears");
    assert!(
        app.chat_messages.is_empty(),
        "the miss the feedback row never speaks"
    );
    assert!(
        app.debug_modifier_used,
        "the used flag the chord matched banks"
    );
    let _ = app;
}

// THE RELOAD CHUNKS' OWN CHORD (the 26.2 handleDebugKeys walk the
// keyDebugReloadChunk lane wears — the reloadchunks_170.py probe the
// pc 101-128 window banks: the `levelExtractor.allChanged()` walk
// pc 111-118 the FULL re-extract speaks, the debugFeedbackTranslated
// pc 121-124 the yellow `debug.reload_chunks.message` row serves,
// the istore_3 the used flag the arm banks REGARDLESS — the
// handleDebugKeys' FIRST lane the ladder's own head wears)
#[test]
fn the_reload_chunks_chord_the_mesh_repair_the_word_wear() {
    let mut app = App::new(262);
    app.enter_world();
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let top = app.spawn_y;
    // THE STALE MESH (the chat-open refresh's screen gate the repair
    // exists to serve — live_view_updated's `screen == World` check
    // the shipped world the mesh walk never re-speaks: the world the
    // set walk edits, the store the skip keeps)
    let stale = app.world_quads.as_ref().map(|q| q.len()).unwrap_or(0);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(c, top + 1, c, crate::world::STONE);
    assert_eq!(
        app.world_quads.as_ref().map(|q| q.len()).unwrap_or(0),
        stale,
        "the edit the stale mesh never re-speaks (no remesh lane ran)"
    );
    // THE CHORD (the all-changed walk the mesh store rebuilds — the
    // planted stone the reload re-speaks, the yellow word the
    // feedback channel serves)
    app.on_reload_chunks_key();
    let fresh = {
        let world = app.offline_world.as_ref().unwrap();
        crate::mesher::mesh_world(world, None).len()
    };
    let reloaded = app.world_quads.as_ref().map(|q| q.len()).unwrap_or(0);
    assert_eq!(reloaded, fresh, "the reload the mesh store rebuilds");
    assert_ne!(
        reloaded, stale,
        "the planted stone the mesh count the reload re-speaks"
    );
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("\u{a7}eReloading all chunks"),
        "the word the lang row keeps"
    );
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    app.on_debug_modifier_released();
    assert!(!app.debug_modifier_used, "the release the flag consumes");
    let _ = app;
}

// THE RELOAD CHUNKS' OWN NET ARM (the wire world the same lane
// serves — the live refresh's screen gate the chat-open walk skips,
// the shipped window the mesh walk never re-speaks until the World
// screen returns — the chord the repair NOW wears: the net store
// the re-mesh re-speaks, the word the same yellow row keeps, the
// used flag the same unconditional bank wears)
#[test]
fn the_reload_chunks_chord_the_net_window_repair_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.world_is_network = true;
    // THE WIRE WINDOW'S OWN PLANTING (the all-air net world the stone
    // block wears — the mesh the store's own content speaks)
    let mut net = air_net_world(2, 0);
    net.set(16, 20, 16, crate::world::STONE);
    app.net_world = Some(net);
    // THE STALE MESH (the chat-open refresh the empty store ships —
    // the gate's skip the chord exists to repair)
    app.world_quads = Some(Vec::new());
    // THE CHORD (the net arm the same all-changed walk serves)
    app.on_reload_chunks_key();
    let fresh = {
        let net = app.net_world.as_ref().unwrap();
        crate::mesher::mesh_world(net, None).len()
    };
    let reloaded = app.world_quads.as_ref().map(|q| q.len()).unwrap_or(0);
    assert_eq!(reloaded, fresh, "the net store the reload re-speaks");
    assert!(reloaded > 0, "the planted stone the net mesh wears");
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("\u{a7}eReloading all chunks"),
        "the word the same lane wears"
    );
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    let _ = app;
}

// THE COPY LOCATION'S OWN CHORD (the keyDebugCopyLocation lane the
// copyloc_207.py walk banks — the `/execute in %s run tp @s %.2f
// %.2f %.2f %.2f %.2f` format the Locale.ROOT walk speaks, the word
// row the format walk PRECEDES the debugFeedbackTranslated yellow
// wears, the used flag the gate-passed bank keeps, the release the
// bare F3 consumes)
#[test]
fn the_copy_location_chord_the_tp_format_the_word_wear() {
    let mut app = App::new(262);
    app.enter_world();
    // THE BODY'S OWN FEET (the dyadic values the %.2f format never
    // rounds — the exact two-decimal strings the walk wears)
    app.body = Some(crate::body::PlayerBody::new(
        12.25, 70.0, -8.25, 180.5, 45.25,
    ));
    // THE CHORD (the gate-passed walk the word + the clipboard + the
    // flag the same breath serves)
    app.on_copy_location_key();
    // THE COMMAND (the overworld identifier the default dimension
    // keeps, the five %.2f the feet + angles speak)
    assert_eq!(
        app.clipboard, "/execute in minecraft:overworld run tp @s 12.25 70.00 -8.25 180.50 45.25",
        "the /tp command the Locale.ROOT format speaks"
    );
    // THE WORD (the debug.copy_location.message row the lang table
    // keeps, the §e the family's yellow channel wears)
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("\u{a7}eCopied location to clipboard"),
        "the word the lang row keeps"
    );
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    app.on_debug_modifier_released();
    assert!(!app.debug_modifier_used, "the release the flag consumes");
    let _ = app;
}

// THE COPY LOCATION'S OWN GATE + NETHER IDENTIFIER (the player-null
// arm pc 1014-1021 the `ifnull -> 1163` walk keeps — the word the
// clipboard the flag the gated walk never serves —, the swap's arm
// the the_nether identifier the ResourceKey speaks)
#[test]
fn the_copy_location_chord_the_null_body_silence_the_nether_wears() {
    let mut app = App::new(262);
    app.enter_world();
    // THE NULL-BODY ARM (the body the Option never carries — the
    // gate the walk never passes)
    app.clipboard = "the previous copy keeps".to_string();
    let rows_before = app.chat_messages.len();
    app.on_copy_location_key();
    assert_eq!(
        app.clipboard, "the previous copy keeps",
        "the gate the write refuses"
    );
    assert_eq!(
        app.chat_messages.len(),
        rows_before,
        "the silence the gate keeps"
    );
    assert!(
        !app.debug_modifier_used,
        "the flag the gated walk never banks"
    );
    // THE NETHER'S OWN IDENTIFIER (the swap the dimension wears — the
    // ResourceKey's the_nether word the /execute arg speaks)
    app.world_dimension = Dimension::Nether;
    app.body = Some(crate::body::PlayerBody::new(0.5, 64.0, 0.5, -90.0, 0.0));
    app.on_copy_location_key();
    assert_eq!(
        app.clipboard, "/execute in minecraft:the_nether run tp @s 0.50 64.00 0.50 -90.00 0.00",
        "the nether identifier the swap's own world wears"
    );
    assert!(
        app.debug_modifier_used,
        "the gate-passed walk the bank wears"
    );
    let _ = app;
}

// THE RELOAD RESOURCE PACKS' OWN CHORD (the keyDebugReloadResourcePacks
// lane the copyloc_207.py lane-map scan banks — the no-gate walk the
// unconditional body keeps, the word row the reload walk PRECEDES the
// debugFeedbackTranslated yellow wears, the splash store the file
// re-read re-serves, the flag the no-gate bank keeps)
#[test]
fn the_reload_resource_packs_chord_the_word_the_splash_reload_wear() {
    let mut app = App::new(262);
    app.enter_world();
    // THE SENTINEL (the stale list the reload replaces — the file's
    // own parse the fresh list keeps)
    app.splashes = vec!["the stale splash!".to_string()];
    // THE CHORD (the no-gate walk the word + the reload + the flag
    // the same breath serves)
    app.on_reload_resource_packs_key();
    // THE RELOAD (the load walk re-ran — the sentinel the fresh list
    // replaced, the list the loader's own parse answers)
    assert_ne!(
        app.splashes,
        vec!["the stale splash!".to_string()],
        "the reload the stale list replaces"
    );
    assert_eq!(
        app.splashes,
        load_splashes(),
        "the reload the file's own parse re-serves"
    );
    // THE WORD (the debug.reload_resourcepacks.message row the lang
    // table keeps, the §e the family's yellow channel wears)
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("\u{a7}eReloaded resource packs"),
        "the word the lang row keeps"
    );
    // THE FLAG (the no-gate walk the bank wears regardless — the
    // release the bare F3 consumes)
    assert!(app.debug_modifier_used, "the no-gate walk the bank wears");
    app.on_debug_modifier_released();
    assert!(!app.debug_modifier_used, "the release the flag consumes");
    let _ = app;
}

// THE DEBUG CRASH'S OWN HOLD (the keyPress debugCrashKeyTime arm +
// the tick() expiry the merged walk serves — the start the clock
// arms, the first second the held row the yellow speaks, the rest
// the countdown the red ceil speaks, the release the clock clears,
// the remaining < 2s the stopping flag the game's own end banks)
#[test]
fn the_debug_crash_hold_the_clock_the_warnings_the_crash_wear() {
    let mut app = App::new(262);
    app.enter_world();
    let t0 = 1_000_000i64;
    // THE START (the F3+C down the clock arms — the reported clock +
    // the count the same breath banks, the modifier the hold spends)
    app.tick_debug_crash_hold(true, true, t0);
    assert_eq!(app.debug_crash_key_time, t0, "the clock the start banks");
    assert_eq!(
        app.debug_crash_reported_count, 0,
        "the count the start zeroes"
    );
    assert!(app.debug_modifier_used, "the modifier the hold spends");
    // THE IDLE CLOCKS (the no-hold walk the clocks never arms — the
    // lone F3 the gate refuses, the lone C the same walk keeps)
    let mut idle = App::new(262);
    idle.tick_debug_crash_hold(true, false, t0);
    idle.tick_debug_crash_hold(false, true, t0 + 500);
    assert_eq!(idle.debug_crash_key_time, 0, "the gates the start refuse");
    assert!(!idle.stopping, "the idle walk the game keeps");
    let _ = idle;
    // THE FIRST WARNING (the 1-second cadence the HELD row speaks —
    // the §e the feedback channel wears, the count the +1 grows)
    app.tick_debug_crash_hold(true, true, t0 + 1100);
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("\u{a7}eF3 + C is held down. This will crash the game unless released."),
        "the held row the feedback walk wears"
    );
    assert_eq!(
        app.debug_crash_reported_count, 1,
        "the count the warning grows"
    );
    assert!(!app.stopping, "the game the hold never ends early");
    // THE COUNTDOWN (the second warning the ceil serves — the §c the
    // warning channel wears; remaining 7500 the ceil-8 row speaks)
    app.tick_debug_crash_hold(true, true, t0 + 2500);
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("\u{a7}cCrashing in 8..."),
        "the countdown row the warning walk wears"
    );
    // THE RELEASE (the clock clears — the game lives, the walk stops)
    app.tick_debug_crash_hold(false, true, t0 + 3000);
    assert_eq!(app.debug_crash_key_time, -1, "the release the clock clears");
    assert!(!app.stopping, "the release the game keeps");
    // THE RESTART + THE CRASH (the remaining < 2s the
    // ReportedException serves — the stopping flag the run loop's own
    // exit speaks, the clock the crash walk never clears)
    app.tick_debug_crash_hold(true, true, t0 + 4000);
    assert_eq!(
        app.debug_crash_key_time,
        t0 + 4000,
        "the restart the clock arms"
    );
    app.tick_debug_crash_hold(true, true, t0 + 12500);
    assert!(app.stopping, "the crash the stopping flag speaks");
    assert_eq!(
        app.debug_crash_key_time,
        t0 + 4000,
        "the clock the crash walk never clears"
    );
    let _ = app;
}

// THE FOV EFFECT SCALE'S OWN ROW (the 26.2 AccessibilityOptionsScreen
// row the file's law wears — the fov_scale_tail_262.py probe the
// Camera.tickFov read the body's tick passes: the boot load the body's
// own store wears, the strict decode the out-of-range/bool rows the
// skip keeps, the restart the row outlives)
#[test]
fn the_fov_effect_scale_row_the_boot_the_body_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_fov_scale_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE DEFAULT (the dconst_1 the ctor speaks)
    assert_eq!(app.fov_effect_scale, 1.0, "the default the ctor banks");
    // THE FILE'S TRUTH (the 0.5 row the boot load wears — the body's
    // own store the tick's read passes)
    std::fs::write(
        &path,
        "version:4903\nmouseSensitivity:0.5\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:true\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:0.5\nchatOpacity:1.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\n",
    )
    .expect("the scale file writes");
    app.enter_world();
    app.spawn_offline_body();
    app.options_file_load();
    assert_eq!(app.fov_effect_scale, 0.5, "the row the load wears");
    let b = app.body.as_ref().expect("the spawned body");
    assert!(
        (b.fov_effect_scale - 0.5).abs() < 1e-6,
        "the body the read wears"
    );
    // THE OFF (the 0.0 row the boot the body's silent store wears —
    // the effects the OFF caption keeps, the zoom the 1.0 answers)
    std::fs::write(
        &path,
        "version:4903\nmouseSensitivity:0.5\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:true\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:0.0\nchatOpacity:1.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\n",
    )
    .expect("the off file writes");
    let mut app2 = App::new(262);
    app2.options_file = Some(path.clone());
    app2.enter_world();
    app2.spawn_offline_body();
    app2.options_file_load();
    let b2 = app2.body.as_ref().expect("the body");
    assert!(
        (b2.fov_effect_scale - 0.0).abs() < 1e-6,
        "the OFF row the body wears"
    );
    // THE STRICT DECODE (the out-of-range 1.5 the default keeps — the
    // clamp never wears, the doubleRange codec the skip answers)
    std::fs::write(
        &path,
        "version:4903\nmouseSensitivity:0.5\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:true\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.5\nchatOpacity:1.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\n",
    )
    .expect("the oor file writes");
    let mut app4 = App::new(262);
    app4.options_file = Some(path.clone());
    app4.options_file_load();
    assert_eq!(
        app4.fov_effect_scale, 1.0,
        "the out-of-range the default keeps"
    );
    // THE ROUND TRIP (the in-memory scale the removed() exit banks —
    // the restart the row outlives)
    let mut app5 = App::new(262);
    app5.options_file = Some(path.clone());
    app5.fov_effect_scale = 0.25;
    app5.open(ScreenKind::Options);
    app5.open(ScreenKind::Title);
    let text = std::fs::read_to_string(&path).expect("the exit the file writes");
    assert!(
        text.contains("fovEffectScale:0.25\n"),
        "the scale row the exit banks"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// THE ACCESSIBILITY SCREEN'S OWN SEAT (the 26.2
// AccessibilityOptionsScreen the access_screen_262.py probe banks —
// the `options.accessibility` button the lambda$init$8 supplier
// opens, the row array's index-12 fovEffectScale the sub-screen's
// slider wears, the Escape the shouldCloseOnEsc default the parent
// ride returns, the save law the OptionsSubScreen.removed() the
// class INHERITS the exit's own moment keeps — the base class's
// Options.save the 185 recon the whole family banks)
#[test]
fn the_accessibility_screen_the_row_the_save_seat_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_access_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE OPENER'S OWN ROW (the title lane the row wears — the
    // inWorld flag the accessibility button never reads, the
    // `options.accessibility` word the lang table speaks)
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let acc = app
        .buttons
        .iter()
        .find(|b| b.id == "accessibility")
        .expect("the accessibility opener")
        .clone();
    assert_eq!(
        acc.label, "Accessibility Settings...",
        "the `options.accessibility` word the lang table speaks"
    );
    // THE PAUSE LANE THE SAME ROW WEARS (the worldoptions test's own
    // walk — the pause → Options screen the BOTH openers keep)
    app.enter_world();
    app.paused = true;
    app.render_title_buttons_for_test();
    let opt = app
        .buttons
        .iter()
        .find(|b| b.id == "options")
        .expect("pauseoptions")
        .clone();
    app.on_click(opt.x + opt.w / 2, opt.y + opt.h / 2);
    assert_eq!(app.screen, ScreenKind::Options);
    app.render_title_buttons_for_test();
    assert!(
        app.buttons.iter().any(|b| b.id == "accessibility"),
        "the pause lane the same row wears"
    );
    assert!(
        app.buttons.iter().any(|b| b.id == "worldoptions"),
        "the in-world opener the same lane keeps"
    );
    // THE SUB-SCREEN'S OWN WALK (the lambda$init$8 supplier the click
    // answers — the open() head the old==Options removed() moment the
    // file's rows keep: the DEFAULT 1.0 the save the entry banks)
    app.on_click(acc.x + acc.w / 2, acc.y + acc.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::Accessibility,
        "the sub-screen the opener wears"
    );
    let pre_exit = std::fs::read_to_string(&path).expect("the entry save wrote");
    assert!(
        pre_exit.contains("fovEffectScale:1.0\n"),
        "the removed() moment the entry save keeps ({pre_exit})"
    );
    // THE ROW'S OWN CAPTION (the UnitDouble percent the
    // generic_value composition speaks — the default 1.0 the 100%
    // word keeps)
    app.render_title_buttons_for_test();
    let fes = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov_effect")
        .expect("the fov effects row")
        .clone();
    assert_eq!(
        fes.label, "FOV Effects: 100%",
        "the default's own percent word"
    );
    assert_eq!(fes.value, Some(1.0), "the slider the store wears");
    // THE SLIDER'S OWN LAW (the setValueFromMouseX walk the Options
    // screen's own seats wear — the left edge the OFF caption speaks,
    // the armed drag the store's own seat follows, the release the
    // disarm speaks)
    app.on_press(fes.x + 4, fes.y + fes.h / 2);
    assert_eq!(app.fov_effect_scale, 0.0, "the left edge clamps");
    assert!(app.fov_effect_drag, "the press inside the track arms");
    app.render_title_buttons_for_test();
    let fes = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov_effect")
        .expect("the row")
        .clone();
    assert_eq!(fes.label, "FOV Effects: OFF", "the 0.0 the OFF end wears");
    app.on_fov_effect_slider_drag(fes.x + fes.w / 2);
    assert_eq!(
        app.fov_effect_scale, 0.5,
        "the mid-track drag the half wears"
    );
    app.on_fov_effect_slider_drag(fes.x + fes.w + 60);
    assert_eq!(app.fov_effect_scale, 1.0, "the beyond-right drag clamps");
    app.on_left_release();
    assert!(!app.fov_effect_drag, "the release disarms");
    app.on_fov_effect_slider_drag(fes.x + 4);
    assert_eq!(app.fov_effect_scale, 1.0, "the disarmed drag never moves");
    // THE ESCAPE'S OWN LANE (the shouldCloseOnEsc default the
    // keyPressed walk answers — the parent the lastScreen ride
    // returns; THE REMOVED MOMENT THE BASE CLASS THE 185 RECON BANKS:
    // the `OptionsSubScreen.removed()` pc 7 the Options.save call the
    // class inherits — the sub-screen the own-removed-less class the
    // super chain answers, the in-screen change the exit's save the
    // file keeps — the autopush-176 "the save never speaks" verdict
    // the base-class walk the probe REFUTES, the class-file probe the
    // own-methods table only read)
    app.fov_effect_scale = 0.75;
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::Options,
        "the parent the Escape ride returns"
    );
    let mid = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        mid.contains("fovEffectScale:0.75\n"),
        "the inherited removed() the exit save keeps ({mid})"
    );
    // THE PARENT'S OWN EXIT (the removed() moment the save the
    // sub-screen's change banks — the restart the row outlives)
    app.open(ScreenKind::Title);
    let post = std::fs::read_to_string(&path).expect("the parent exit wrote");
    assert!(
        post.contains("fovEffectScale:0.75\n"),
        "the sub-screen's own 0.75 the exit saves keep"
    );
    // THE DONE FOOTER'S OWN RETURN (the onClose walk the accdone id
    // answers — the same inherited removed() law the Escape lane
    // keeps)
    let mut app2 = App::new(262);
    app2.options_file = Some(path.clone());
    app2.open(ScreenKind::Options);
    app2.open(ScreenKind::Accessibility);
    app2.fov_effect_scale = 0.25;
    app2.render_title_buttons_for_test();
    let done = app2
        .buttons
        .iter()
        .find(|b| b.id == "accdone")
        .expect("the done footer")
        .clone();
    app2.on_click(done.x + done.w / 2, done.y + done.h / 2);
    assert_eq!(
        app2.screen,
        ScreenKind::Options,
        "the parent the Done wears"
    );
    let mid2 = std::fs::read_to_string(&path).expect("the rows");
    assert!(
        mid2.contains("fovEffectScale:0.25\n"),
        "the done exit the inherited removed() banks"
    );
    app2.open(ScreenKind::Title);
    let post2 = std::fs::read_to_string(&path).expect("the parent exit wrote");
    assert!(
        post2.contains("fovEffectScale:0.25\n"),
        "the sub-screen's change the parent exit banks"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// THE CHAT BACKGROUNDS' OWN ROW (the autopush-177 round — the
// `textBackgroundOpacity` option the accessibility array's index-4
// seat wears: the Options ctor's `ldc2_w double 4602678819172646912`
// bits the 0.5 default speak, the noTooltip lane the registration
// wears, the UnitDouble slider the row's own walk keeps; the
// ChatComponent's per-line fill the lambda$extractRenderState$1 walk
// the `ARGB.black(fade * textbg)` composition keeps — the
// textbg_262v2.py + textbg_lambda_262.py probes the whole law bank).
#[test]
fn the_textbg_row_the_chat_background_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_textbg_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE SUB-SCREEN'S OWN SEAT (the accessibility row the opener
    // wears — the entry save the removed() moment the file's rows
    // keep, the DEFAULT 0.5 the chat background's row speaks)
    app.open(ScreenKind::Options);
    app.open(ScreenKind::Accessibility);
    let pre_exit = std::fs::read_to_string(&path).expect("the entry save wrote");
    assert!(
        pre_exit.contains("textBackgroundOpacity:0.5\n"),
        "the removed() moment the entry save keeps ({pre_exit})"
    );
    // THE ROW'S OWN SEAT (the array's index 4 the row ABOVE the fov
    // seat wears — index 4 before index 12 the order keeps)
    app.render_title_buttons_for_test();
    let tb = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_textbg")
        .expect("the chat background row")
        .clone();
    let fes = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov_effect")
        .expect("the fov row")
        .clone();
    assert!(
        tb.y < fes.y,
        "the index-4 row the seat above the fov row wears"
    );
    assert_eq!(
        tb.label, "Text Background Opacity: 50%",
        "the default's own percent word"
    );
    assert_eq!(tb.value, Some(0.5), "the slider the store wears");
    // THE SLIDER'S OWN LAW (the setValueFromMouseX walk the fov row's
    // own shape wears — the left edge the OFF caption speaks, the
    // armed drag the store's own seat follows, the beyond-right the
    // clamp answers, the release the disarm speaks)
    app.on_press(tb.x + 4, tb.y + tb.h / 2);
    assert_eq!(app.text_background_opacity, 0.0, "the left edge clamps");
    assert!(app.textbg_drag, "the press inside the track arms");
    app.render_title_buttons_for_test();
    let tb = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_textbg")
        .expect("the row")
        .clone();
    assert_eq!(
        tb.label, "Text Background Opacity: OFF",
        "the 0.0 the OFF end wears"
    );
    app.on_textbg_slider_drag(tb.x + tb.w / 2);
    assert_eq!(
        app.text_background_opacity, 0.5,
        "the mid-track drag the half wears"
    );
    app.on_textbg_slider_drag(tb.x + tb.w + 60);
    assert_eq!(
        app.text_background_opacity, 1.0,
        "the beyond-right drag clamps"
    );
    app.on_left_release();
    assert!(!app.textbg_drag, "the release disarms");
    app.on_textbg_slider_drag(tb.x + 4);
    assert_eq!(
        app.text_background_opacity, 1.0,
        "the disarmed drag never moves"
    );
    // THE ESCAPE'S OWN LANE (the shouldCloseOnEsc default the
    // keyPressed walk answers — the parent the lastScreen ride
    // returns; THE REMOVED MOMENT THE BASE CLASS THE 185 RECON BANKS:
    // the `OptionsSubScreen.removed()` the class inherits — the
    // in-screen 1.0 the exit's save the file keeps)
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::Options,
        "the parent the Escape ride returns"
    );
    let mid = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        mid.contains("textBackgroundOpacity:1.0\n"),
        "the inherited removed() the exit save keeps"
    );
    // THE PARENT'S OWN EXIT (the removed() moment the save the
    // sub-screen's change banks — the restart the row outlives)
    app.open(ScreenKind::Title);
    let post = std::fs::read_to_string(&path).expect("the parent exit wrote");
    assert!(
        post.contains("textBackgroundOpacity:1.0\n"),
        "the drag's own 1.0 the parent exit banks"
    );
    // THE CHAT ROW'S OWN COMPOSITION (the
    // lambda$extractRenderState$1 law the `ARGB.black(fade * textbg)`
    // pair speaks — the default 0.5 the client's own 0.5 fold
    // byte-identical keeps, the FULLY_VISIBLE foreground the 1.0 fade
    // the focused history wears, the OFF the zero the fill keeps)
    assert_eq!(
        chat_row_bg_alpha(0.5, 1.0),
        0.5,
        "the default the focused fill byte-identical keeps"
    );
    assert_eq!(
        chat_row_bg_alpha(1.0, chat_line_alpha(0)),
        1.0,
        "the full scale the fresh row the full alpha wears"
    );
    assert_eq!(
        chat_row_bg_alpha(0.0, 1.0),
        0.0,
        "the OFF row the fill the zero keeps"
    );
    assert!(
        (chat_row_bg_alpha(0.75, 0.25) - 0.1875).abs() < 1e-6,
        "the fade the scale's own multiplier wears"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE CHAT TEXT'S OWN ROW (the 26.2 chatopacity_178.py probe the
/// walk banks — the `chatOpacity` option the Options ctor's dconst_1
/// pcs 741-818 speaks, the accessibility array the index-6 seat the
/// row BETWEEN the textbg and the fov seats wears, the
/// ChatComponent$1.accept the `fload_3 * val$textOpacity` pair the
/// per-line fade the glyph alpha multiplies keeps, the file row the
/// visit-order adjacency the chatOpacity@480 < textbg@506 walk banks).
#[test]
fn the_chat_opacity_row_the_chat_text_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_chatop_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE SUB-SCREEN'S OWN SEAT (the accessibility row the opener
    // wears — the entry save the removed() moment the file's rows
    // keep, the DEFAULT 1.0 the chat text's row speaks)
    app.open(ScreenKind::Options);
    app.open(ScreenKind::Accessibility);
    let pre_exit = std::fs::read_to_string(&path).expect("the entry save wrote");
    assert!(
        pre_exit.contains("chatOpacity:1.0\n"),
        "the removed() moment the entry save keeps ({pre_exit})"
    );
    // THE FILE ROW'S OWN ADJACENCY (the visit-order seat the
    // processOptions walk banks — the chatOpacity row the
    // textBackgroundOpacity row's BEFORE wears)
    let co_pos = pre_exit.find("chatOpacity:").expect("the row");
    let tb_pos = pre_exit.find("textBackgroundOpacity:").expect("the row");
    assert!(
        co_pos < tb_pos,
        "the visit-order adjacency the file rows keep"
    );
    // THE ROW'S OWN SEAT (the array's index 6 the row BETWEEN the
    // textbg and the fov seats wears — index 4 < index 6 < index 12
    // the order keeps)
    app.render_title_buttons_for_test();
    let tb = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_textbg")
        .expect("the chat background row")
        .clone();
    let co = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_opacity")
        .expect("the chat text row")
        .clone();
    let fes = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov_effect")
        .expect("the fov row")
        .clone();
    assert!(
        tb.y < co.y && co.y < fes.y,
        "the index-6 row the seat between wears"
    );
    assert_eq!(
        co.label, "Chat Text Opacity: 100%",
        "the default's own percent word"
    );
    assert_eq!(co.value, Some(1.0), "the slider the store wears");
    // THE SLIDER'S OWN LAW (the setValueFromMouseX walk the textbg
    // row's own shape wears — the left edge the OFF caption speaks,
    // the armed drag the store's own seat follows, the beyond-right
    // the clamp answers, the release the disarm speaks)
    app.on_press(co.x + 4, co.y + co.h / 2);
    assert_eq!(app.chat_opacity, 0.0, "the left edge clamps");
    assert!(app.chat_opacity_drag, "the press inside the track arms");
    app.render_title_buttons_for_test();
    let co = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_opacity")
        .expect("the row")
        .clone();
    assert_eq!(
        co.label, "Chat Text Opacity: OFF",
        "the 0.0 the OFF end wears"
    );
    app.on_chat_opacity_slider_drag(co.x + co.w / 2);
    assert_eq!(app.chat_opacity, 0.5, "the mid-track drag the half wears");
    app.on_chat_opacity_slider_drag(co.x + co.w + 60);
    assert_eq!(app.chat_opacity, 1.0, "the beyond-right drag clamps");
    app.on_left_release();
    assert!(!app.chat_opacity_drag, "the release disarms");
    app.on_chat_opacity_slider_drag(co.x + 4);
    assert_eq!(app.chat_opacity, 1.0, "the disarmed drag never moves");
    // THE RE-ARM'S OWN LANE (the left edge the press arms again — the
    // 0.0 the off-end store keeps, the Escape the armed flag the
    // open()'s own drag death clears, the file's rows the sub-screen
    // never wears)
    app.on_press(co.x + 4, co.y + co.h / 2);
    assert_eq!(app.chat_opacity, 0.0, "the re-arm the edge keeps");
    // THE ESCAPE'S OWN LANE (the shouldCloseOnEsc default the
    // keyPressed walk answers — the parent the lastScreen ride
    // returns; THE REMOVED MOMENT THE BASE CLASS THE 185 RECON BANKS:
    // the `OptionsSubScreen.removed()` the class inherits — the
    // in-screen 0.0 the exit's save the file keeps)
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::Options,
        "the parent the Escape ride returns"
    );
    let mid = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        mid.contains("chatOpacity:0.0\n"),
        "the inherited removed() the exit save keeps"
    );
    // THE PARENT'S OWN EXIT (the removed() moment the save the
    // sub-screen's change banks — the restart the row outlives)
    app.open(ScreenKind::Title);
    let post = std::fs::read_to_string(&path).expect("the parent exit wrote");
    assert!(
        post.contains("chatOpacity:0.0\n"),
        "the drag's own 0.0 the parent exit banks"
    );
    // THE TEXT'S OWN COMPOSITION (the ChatComponent$1.accept law the
    // `fload_3 * val$textOpacity` pair speaks — the default 1.0 the
    // 1.0 fold the byte-identical keeps, the 0.5 the 0.55 base the
    // `* 0.9f + 0.1f` rows answer, the OFF the 0.1 the floor the
    // +0.1 keeps, the per-line fade the glyph alpha multiplies)
    assert_eq!(
        chat_text_alpha_base(1.0),
        1.0,
        "the default the focused fold byte-identical keeps"
    );
    assert!(
        (chat_text_alpha_base(0.5) - 0.55).abs() < 1e-6,
        "the half scale the 0.55 base wears"
    );
    assert_eq!(
        chat_text_alpha_base(0.0),
        0.1,
        "the OFF row the 0.1 floor the +0.1 keeps"
    );
    assert!(
        (chat_text_alpha_base(0.5) * chat_line_alpha(0) - 0.55).abs() < 1e-6,
        "the fresh row's full fade the base multiplies"
    );
    assert!(
        (chat_text_alpha_base(0.25) - 0.325).abs() < 1e-6,
        "the quarter scale the 0.325 base the same rows answer"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE BAND SCOPE'S OWN ROW (the 26.2 chatbgonly_179 probes the walk
/// banks — the `backgroundForChatOnly` option the Options ctor's
/// `iconst_1` pcs 2092-2120 speaks, the accessibility array the
/// index-5 seat the row BETWEEN the textbg and the chat-text seats
/// wears, the CycleButton's DisplayState.VALUE the label law the
/// lambda$new$76 pair keeps — the caption the walk ignores, the
/// `getBackgroundColor(I)I` the input band's raw/Everywhere pair the
/// ChatScreen.extractRenderState call answers, the file row the
/// visit-order adjacency the textbg@506 < bgOnly@523 walk banks).
#[test]
fn the_chat_bg_only_row_the_band_scope_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_chatbgonly_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE SUB-SCREEN'S OWN SEAT (the accessibility row the opener
    // wears — the entry save the removed() moment the file's rows
    // keep, the DEFAULT TRUE the band scope's row speaks)
    app.open(ScreenKind::Options);
    app.open(ScreenKind::Accessibility);
    let pre_exit = std::fs::read_to_string(&path).expect("the entry save wrote");
    assert!(
        pre_exit.contains("backgroundForChatOnly:true\n"),
        "the removed() moment the entry save keeps ({pre_exit})"
    );
    // THE FILE ROW'S OWN ADJACENCY (the visit-order seat the
    // processOptions walk banks — the bgOnly row the
    // textBackgroundOpacity row's AFTER wears)
    let bo_pos = pre_exit.find("backgroundForChatOnly:").expect("the row");
    let tb_pos = pre_exit.find("textBackgroundOpacity:").expect("the row");
    assert!(
        tb_pos < bo_pos,
        "the visit-order adjacency the file rows keep"
    );
    // THE ROW'S OWN SEAT (the array's index 5 the row BETWEEN the
    // textbg and the chat-text seats wears — index 4 < index 5 <
    // index 6 < index 12 the order keeps)
    app.render_title_buttons_for_test();
    let tb = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_textbg")
        .expect("the chat background row")
        .clone();
    let bo = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_bg_only")
        .expect("the band scope row")
        .clone();
    let co = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_opacity")
        .expect("the chat text row")
        .clone();
    let fes = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov_effect")
        .expect("the fov row")
        .clone();
    assert!(
        tb.y < bo.y && bo.y < co.y && co.y < fes.y,
        "the index-5 row the seat between wears"
    );
    assert_eq!(bo.label, "Chat", "the TRUE's own VALUE word");
    // THE TOGGLE'S OWN LAW (the CycleButton's own press walk the bool
    // row wears — the press the TRUE/FALSE pair flips, the caption
    // the Everywhere word the FALSE row speaks, the second press the
    // TRUE seat restores)
    app.on_press(bo.x + bo.w / 2, bo.y + bo.h / 2);
    assert!(
        !app.background_for_chat_only,
        "the press the FALSE seat flips"
    );
    app.render_title_buttons_for_test();
    let bo = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_bg_only")
        .expect("the row")
        .clone();
    assert_eq!(
        bo.label, "Everywhere",
        "the FALSE's own VALUE word the caption speaks"
    );
    app.on_press(bo.x + bo.w / 2, bo.y + bo.h / 2);
    assert!(
        app.background_for_chat_only,
        "the second press the TRUE seat restores"
    );
    // THE BAND'S OWN FOLD (the getBackgroundColor(I)I law the pure fn
    // speaks — the TRUE lane the raw 0.5 the textbg never moves, the
    // Everywhere lane the textbg-scaled alpha the pair answers, the
    // 0.5 default the byte-identical fold the two lanes share)
    assert_eq!(
        chat_band_alpha(true, 0.25),
        0.5,
        "the TRUE lane the raw alpha keeps — the textbg the lane never wears"
    );
    assert_eq!(
        chat_band_alpha(false, 0.5),
        0.5,
        "the 0.5 default the byte-identical fold the two lanes share"
    );
    assert_eq!(
        chat_band_alpha(false, 0.25),
        0.25,
        "the Everywhere lane the textbg-scaled alpha wears"
    );
    assert_eq!(
        chat_band_alpha(false, 1.0),
        1.0,
        "the Everywhere lane the full-opacity end answers"
    );
    // THE PARENT'S OWN EXIT (the removed() moment the save the
    // sub-screen's change banks — the Escape ride the sub-screen's
    // own walk never wears, the Options exit the file's rows speaks —
    // the Everywhere row the restart outlives)
    app.on_press(bo.x + bo.w / 2, bo.y + bo.h / 2);
    assert!(
        !app.background_for_chat_only,
        "the third press the FALSE seat the exit banks"
    );
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::Options,
        "the parent the Escape ride returns"
    );
    // THE REMOVED MOMENT THE BASE CLASS THE 185 RECON BANKS (the
    // `OptionsSubScreen.removed()` the class inherits — the FALSE
    // seat the exit's save the file keeps)
    let mid = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        mid.contains("backgroundForChatOnly:false\n"),
        "the inherited removed() the exit save keeps"
    );
    app.open(ScreenKind::Title);
    let post = std::fs::read_to_string(&path).expect("the parent exit wrote");
    let bo_pos = post.find("backgroundForChatOnly:").expect("the row");
    let tb_pos = post.find("textBackgroundOpacity:").expect("the row");
    assert!(
        post.contains("backgroundForChatOnly:false\n") && tb_pos < bo_pos,
        "the Everywhere row the parent exit banks ({post})"
    );
    // THE RESTART'S OWN DECODE (the fresh boot the file's truth wears
    // — the FALSE row the store the reload keeps)
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.background_for_chat_only, false,
        "the restart the Everywhere row the decode keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE CHAT LINES' OWN ROW (the 26.2 chatlinespacing_181/181b probes
/// the walk banks — the `chatLineSpacing` option the Options ctor's
/// dconst_0 pcs 813-842 speaks, the accessibility array the index-7
/// seat the row BETWEEN the chat-text and the fov seats wears, the
/// chatOpacity/textbg rows the SAME indy@622 CaptionBasedToString the
/// percent/OFF words keep, the processOptions the ldc@493 the
/// chatOpacity@480 the row AFTER answers, the textBackgroundOpacity
/// @506 the row BEFORE the same ladder speaks; the ChatComponent's
/// own pitch the extractRenderState istore-18/istore-19 laws the
/// `trunc(9.0 * (spacing + 1.0))` stride and the `round(8.0 *
/// (spacing + 1.0) - 4.0 * spacing)` glyph offset speak — the `9 +
/// round(4.0 * spacing)` and `round(8 - 4 * spacing)` guesses the old
/// notes wore the bytecode refuses, the zero spacing the 9/8 pair the
/// byte-identical answer keeps).
#[test]
fn the_chat_line_spacing_row_the_chat_pitch_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_chatls_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE SUB-SCREEN'S OWN SEAT (the accessibility row the opener
    // wears — the entry save the removed() moment the file's rows
    // keep, the DEFAULT 0.0 the chat lines' row speaks)
    app.open(ScreenKind::Options);
    app.open(ScreenKind::Accessibility);
    let pre_exit = std::fs::read_to_string(&path).expect("the entry save wrote");
    assert!(
        pre_exit.contains("chatLineSpacing:0.0\n"),
        "the removed() moment the entry save keeps ({pre_exit})"
    );
    // THE FILE ROW'S OWN ADJACENCY (the visit-order seat the
    // processOptions walk banks — the chatLineSpacing row the
    // chatOpacity row's AFTER the textBackgroundOpacity row's BEFORE
    // wears)
    let ls_pos = pre_exit.find("chatLineSpacing:").expect("the row");
    let co_pos = pre_exit.find("chatOpacity:").expect("the row");
    let tb_pos = pre_exit.find("textBackgroundOpacity:").expect("the row");
    assert!(
        co_pos < ls_pos && ls_pos < tb_pos,
        "the visit-order adjacency the file rows keep"
    );
    // THE ROW'S OWN SEAT (the array's index 7 the row BETWEEN the
    // chat-text and the fov seats wears — index 4 < index 5 < index 6
    // < index 7 < index 12 the order keeps)
    app.render_title_buttons_for_test();
    let tb = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_textbg")
        .expect("the chat background row")
        .clone();
    let bo = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_bg_only")
        .expect("the band scope row")
        .clone();
    let co = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_opacity")
        .expect("the chat text row")
        .clone();
    let ls = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_line_spacing")
        .expect("the line spacing row")
        .clone();
    let fes = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov_effect")
        .expect("the fov row")
        .clone();
    assert!(
        tb.y < bo.y && bo.y < co.y && co.y < ls.y && ls.y < fes.y,
        "the index-7 row the seat between wears"
    );
    assert_eq!(
        ls.label, "Line Spacing: OFF",
        "the lang row's own word the OFF end wears"
    );
    assert_eq!(ls.value, Some(0.0), "the slider the store wears");
    // THE SLIDER'S OWN LAW (the setValueFromMouseX walk the chatOpacity
    // row's own shape wears — the left edge the OFF caption speaks,
    // the armed drag the store's own seat follows, the beyond-right
    // the clamp answers, the release the disarm speaks)
    app.on_press(ls.x + 4, ls.y + ls.h / 2);
    assert_eq!(app.chat_line_spacing, 0.0, "the left edge clamps");
    assert!(
        app.chat_line_spacing_drag,
        "the press inside the track arms"
    );
    app.render_title_buttons_for_test();
    let ls = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_line_spacing")
        .expect("the row")
        .clone();
    assert_eq!(ls.label, "Line Spacing: OFF", "the 0.0 the OFF end wears");
    app.on_chat_line_spacing_slider_drag(ls.x + ls.w / 2);
    assert_eq!(
        app.chat_line_spacing, 0.5,
        "the mid-track drag the half wears"
    );
    app.render_title_buttons_for_test();
    let ls = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_line_spacing")
        .expect("the row")
        .clone();
    assert_eq!(
        ls.label, "Line Spacing: 50%",
        "the half the percent word wears"
    );
    app.on_chat_line_spacing_slider_drag(ls.x + ls.w + 60);
    assert_eq!(app.chat_line_spacing, 1.0, "the beyond-right drag clamps");
    app.on_left_release();
    assert!(!app.chat_line_spacing_drag, "the release disarms");
    app.on_chat_line_spacing_slider_drag(ls.x + 4);
    assert_eq!(app.chat_line_spacing, 1.0, "the disarmed drag never moves");
    // THE RE-ARM'S OWN LANE (the left edge the press arms again — the
    // 0.0 the off-end store keeps, the Escape the armed flag the
    // open()'s own drag death clears, the file's rows the sub-screen
    // never wears)
    app.on_press(ls.x + 4, ls.y + ls.h / 2);
    assert_eq!(app.chat_line_spacing, 0.0, "the re-arm the edge keeps");
    // THE ESCAPE'S OWN LANE (the shouldCloseOnEsc default the
    // keyPressed walk answers — the parent the lastScreen ride
    // returns; THE REMOVED MOMENT THE BASE CLASS THE 185 RECON BANKS:
    // the `OptionsSubScreen.removed()` the class inherits — the
    // in-screen 0.0 the exit's save the file keeps)
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::Options,
        "the parent the Escape ride returns"
    );
    let mid = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        mid.contains("chatLineSpacing:0.0\n"),
        "the inherited removed() the exit save keeps"
    );
    // THE PARENT'S OWN EXIT (the removed() moment the save the
    // sub-screen's change banks — the restart the row outlives)
    app.open(ScreenKind::Title);
    let post = std::fs::read_to_string(&path).expect("the parent exit wrote");
    assert!(
        post.contains("chatLineSpacing:0.0\n"),
        "the drag's own 0.0 the parent exit banks"
    );
    // THE PITCH'S OWN FOLDS (the 26.2 extractRenderState walk the
    // istore-18/istore-19 laws speak — the stride the `trunc(9.0 *
    // (spacing + 1.0))` d2i keeps, the glyph offset the `round(8.0 *
    // (spacing + 1.0) - 4.0 * spacing)` Math.round answers; the zero
    // spacing the 9/8 pair the client's own constants wear — the
    // `chat_line_y` the byte-identical answer keeps, the 0.5 the
    // 13/10 pair speaks, the 1.0 the 18/12, the 0.75 the stride's
    // d2i truncation the 15.75's floor answers)
    assert_eq!(chat_line_stride(0.0), 9, "the default the 9 keeps");
    assert_eq!(chat_line_stride(0.5), 13, "the half the 13.5's d2i wears");
    assert_eq!(
        chat_line_stride(0.75),
        15,
        "the three-quarter the 15.75's trunc keeps"
    );
    assert_eq!(chat_line_stride(1.0), 18, "the full the 18.0 speaks");
    assert_eq!(chat_line_glyph_offset(0.0), 8, "the default the 8 keeps");
    assert_eq!(
        chat_line_glyph_offset(0.5),
        10,
        "the half the 10.0's round wears"
    );
    assert_eq!(chat_line_glyph_offset(1.0), 12, "the full the 12.0 speaks");
    assert_eq!(
        chat_line_glyph_offset(0.25),
        9,
        "the quarter the 9.0's round keeps"
    );
    assert_eq!(
        chat_line_y(360, 0, 0.0),
        360 - 41,
        "the zero spacing the newest line's own top keeps"
    );
    assert_eq!(
        chat_line_y(360, 1, 0.0),
        360 - 50,
        "the zero spacing the second line's pitch keeps"
    );
    assert_eq!(
        chat_line_y(360, 0, 0.5),
        320 - 13 + 10,
        "the half spacing the stride/offset pair wears"
    );
    assert_eq!(
        chat_line_y(360, 1, 1.0),
        320 - 36 + 12,
        "the full spacing the doubled pitch speaks"
    );
    // THE RESTART'S OWN DECODE (the boot the file's truth wears — the
    // 0.5 row the load keeps, the out-of-range the default answers,
    // the missing row the 0.0 keeps)
    std::fs::write(
        &path,
        "version:4903\nmouseSensitivity:0.5\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:true\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.5\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\n",
    )
    .expect("the spacing file writes");
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.chat_line_spacing, 0.5,
        "the restart the half row the decode keeps"
    );
    std::fs::write(
        &path,
        "version:4903\nmouseSensitivity:0.5\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\nadvancedItemTooltips:false\npauseOnLostFocus:true\ntoggleCrouch:false\ntoggleSprint:false\ntoggleAttack:false\ntoggleUse:false\nsprintWindow:7\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:1.5\nchatLineSpacing:0.25\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\n",
    )
    .expect("the ladder file writes");
    let mut re2 = App::new(262);
    re2.options_file = Some(path.clone());
    re2.options_file_load();
    assert_eq!(
        re2.chat_line_spacing, 0.25,
        "the last valid row the decode banks — the out-of-range the skip keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE CLOSED CAPTIONS' OWN ROW (the 26.2 subtitles_182 probe the walk
/// banks — the `showSubtitles` option the Options ctor's `iconst_0`
/// pcs 2053-2067 speaks, the createBoolean law the CLOSED_CAPTIONS
/// tooltip wears, the accessibility array the index-1 seat the row
/// ABOVE the textbg seat hosts, the caption the `options.showSubtitles`
/// lang row the "Closed Captions" word speaks, the ON/OFF words the
/// CommonComponents pair keeps, the processOptions the ldc@149 the
/// visit seat keeps — the row the version line's AFTER the
/// mouseSensitivity@292 row's BEFORE wears; the SubtitleOverlay's own
/// walk the render law serves — the fade the `clampedLerp(ratio, 255,
/// 75)` floor keeps, the rows the purge/audible/closest laws the
/// Subtitle methods speak, the band the 0.8F fold the 204/255 byte
/// wears, the arrows the lateral/forward dot pair the listener's
/// right axis answers).
#[test]
fn the_show_subtitles_row_the_overlay_wears() {
    fn diff_count(a: &[u8], b: &[u8], x0: i32, y0: i32, w: i32, h: i32) -> usize {
        let mut n = 0usize;
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                let o = ((y as usize) * 640 + x as usize) * 3;
                if a[o..o + 3] != b[o..o + 3] {
                    n += 1;
                }
            }
        }
        n
    }
    let dir = std::env::temp_dir().join(format!("mc2rs_showsubs_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE SUB-SCREEN'S OWN SEAT (the accessibility row the opener
    // wears — the entry save the removed() moment the file's rows
    // keep, the DEFAULT FALSE the captions' row speaks)
    app.open(ScreenKind::Options);
    app.open(ScreenKind::Accessibility);
    let pre_exit = std::fs::read_to_string(&path).expect("the entry save wrote");
    assert!(
        pre_exit.contains("showSubtitles:false\n"),
        "the removed() moment the entry save keeps ({pre_exit})"
    );
    // THE FILE ROW'S OWN ADJACENCY (the visit-order seat the
    // processOptions walk banks — the showSubtitles row the version
    // line's AFTER the mouseSensitivity row's BEFORE wears)
    let ss_pos = pre_exit.find("showSubtitles:").expect("the row");
    let v_pos = pre_exit.find("version:").expect("the row");
    let ms_pos = pre_exit.find("mouseSensitivity:").expect("the row");
    assert!(
        v_pos < ss_pos && ss_pos < ms_pos,
        "the visit-order adjacency the file rows keep"
    );
    // THE ROW'S OWN SEAT (the array's index 1 the row ABOVE the textbg
    // seat wears — index 1 < index 4 < index 5 < index 6 < index 7 <
    // index 12 the order keeps)
    app.render_title_buttons_for_test();
    let ss = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_show_subtitles")
        .expect("the captions row")
        .clone();
    let tb = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_textbg")
        .expect("the chat background row")
        .clone();
    let ls = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_line_spacing")
        .expect("the line spacing row")
        .clone();
    let fes = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov_effect")
        .expect("the fov row")
        .clone();
    assert!(
        ss.y < tb.y && tb.y < ls.y && ls.y < fes.y,
        "the index-1 row the first seat wears"
    );
    assert_eq!(
        ss.label, "Closed Captions: OFF",
        "the lang row's own word the OFF end wears"
    );
    // THE TOGGLE'S OWN LANE (the CycleButton press walk the bool row
    // wears — the press INSIDE the row the TRUE/FALSE pair the
    // store's own seat flips, the caption the ON word speaks)
    app.on_press(ss.x + 4, ss.y + ss.h / 2);
    assert!(app.show_subtitles, "the press the toggle flips");
    app.render_title_buttons_for_test();
    let ss = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_show_subtitles")
        .expect("the row")
        .clone();
    assert_eq!(
        ss.label, "Closed Captions: ON",
        "the TRUE the ON word wears"
    );
    // THE ESCAPE'S OWN LANE (the shouldCloseOnEsc default the
    // keyPressed walk answers — the parent the lastScreen ride
    // returns; THE REMOVED MOMENT THE BASE CLASS THE 185 RECON BANKS:
    // the `OptionsSubScreen.removed()` the class inherits — the
    // in-screen TRUE the exit's save the file keeps)
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::Options,
        "the parent the Escape ride returns"
    );
    let mid = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        mid.contains("showSubtitles:true\n"),
        "the inherited removed() the exit save keeps"
    );
    // THE PARENT'S OWN EXIT (the removed() moment the save the
    // sub-screen's change banks — the restart the row outlives)
    app.open(ScreenKind::Title);
    let post = std::fs::read_to_string(&path).expect("the parent exit wrote");
    assert!(
        post.contains("showSubtitles:true\n"),
        "the toggle's own TRUE the parent exit banks"
    );
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert!(
        re.show_subtitles,
        "the restart the TRUE row the decode keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
    // THE FADE'S OWN FOLD (the clampedLerp(ratio, 255, 75) floor the
    // istore-27 law keeps — the fresh play the 255 speaks, the
    // window's half the 165 wears, the end the 75 the floor answers,
    // the beyond-window the 75 clamp keeps, the longer display time
    // the window's own scale speaks)
    assert_eq!(
        crate::sound::subtitle_fade(0, 1.0),
        255,
        "the fresh play the 255 wears"
    );
    assert_eq!(
        crate::sound::subtitle_fade(30, 1.0),
        165,
        "the half-window the 165 keeps"
    );
    assert_eq!(
        crate::sound::subtitle_fade(60, 1.0),
        75,
        "the window's end the 75 wears"
    );
    assert_eq!(
        crate::sound::subtitle_fade(61, 1.0),
        75,
        "the beyond-window the 75 clamp keeps"
    );
    assert_eq!(
        crate::sound::subtitle_fade(120, 2.0),
        75,
        "the doubled display time the doubled window wears"
    );
    // THE BAND'S OWN ALPHA (the (F)I overload the raw 0.8 verbatim the
    // colorFromFloat law keeps — the floor fold the 204/255 byte
    // answers)
    assert!(
        (crate::screens::subtitle_band_alpha() - 204.0 / 255.0).abs() < 1e-6,
        "the 0.8 gate the floor fold wears"
    );
    // THE ROWS' OWN FOLD (the Subtitle methods the 182 probe banks —
    // the fresh play the row the closest speaks, the same-location
    // replay the refresh the newest stamp answers, the new location
    // the play the list adds, the closest the min-distance walk
    // answers, the stale the purge removes, the beyond-range the
    // inaudible the row never wears)
    let entry = crate::sound::SubtitleEntry {
        text_key: "subtitles.block.generic.break".to_string(),
        range: 16.0,
        plays: vec![
            crate::sound::SubtitleLocation {
                x: 10.5,
                y: 2.5,
                z: 10.5,
                tick: 100,
            },
            crate::sound::SubtitleLocation {
                x: 12.5,
                y: 2.5,
                z: 10.5,
                tick: 110,
            },
        ],
    };
    let rows = crate::sound::subtitle_rows(&[entry.clone()], 110, (10.0, 2.0, 10.0), 1.0);
    assert_eq!(rows.len(), 1, "the entry the row the merge keeps");
    assert_eq!(
        (rows[0].x, rows[0].z),
        (10.5, 10.5),
        "the closest play the min-distance walk wears"
    );
    assert_eq!(
        rows[0].age, 10,
        "the closest's own tick the age keeps — the 100-stamp the 110-now reads"
    );
    // THE SAME LOCATION'S OWN REFRESH (the removeIf+add pair the
    // newest stamp the seat keeps — the render fold the max speaks)
    let refreshed = crate::sound::SubtitleEntry {
        plays: vec![
            crate::sound::SubtitleLocation {
                x: 10.5,
                y: 2.5,
                z: 10.5,
                tick: 100,
            },
            crate::sound::SubtitleLocation {
                x: 10.5,
                y: 2.5,
                z: 10.5,
                tick: 105,
            },
        ],
        ..entry.clone()
    };
    let rows = crate::sound::subtitle_rows(&[refreshed], 110, (10.0, 2.0, 10.0), 1.0);
    assert_eq!(rows[0].age, 5, "the refresh the newest stamp keeps");
    // THE PURGE'S OWN WALK (the strictly-greater lambda the window's
    // own edge keeps — the 60-tick window the 20tps the 3000ms pair
    // answers: the age-60 play the edge keeps, the age-61 the purge
    // removes)
    let rows = crate::sound::subtitle_rows(&[entry.clone()], 171, (10.0, 2.0, 10.0), 1.0);
    assert_eq!(rows.len(), 0, "the stale plays the purge removes");
    let rows = crate::sound::subtitle_rows(&[entry.clone()], 170, (10.0, 2.0, 10.0), 1.0);
    assert_eq!(rows.len(), 1, "the window's edge the strict law keeps");
    // THE AUDIBLE'S OWN RADIUS (the isAudibleFrom law the closerThan
    // pair wears — the beyond-range the inaudible the row never wears)
    let far = crate::sound::SubtitleEntry {
        plays: vec![crate::sound::SubtitleLocation {
            x: 100.5,
            y: 2.5,
            z: 10.5,
            tick: 110,
        }],
        ..entry.clone()
    };
    let rows = crate::sound::subtitle_rows(&[far], 110, (10.0, 2.0, 10.0), 1.0);
    assert_eq!(rows.len(), 0, "the beyond-range the inaudible keeps");
    // THE RECORD'S OWN SEAT (the onPlaySound walk the merge lanes
    // serve — the fresh text the entry the sound table's subtitle key
    // banks, the same-location replay the refresh answers, the new
    // location the second play adds)
    let mut app2 = App::new(262);
    app2.sounds
        .record_subtitle(5, "stone", crate::sound::Lane::Break, 1.5, 2.5, 3.5);
    let ev = crate::sound_table::event("block.stone.break").expect("the stone row");
    let key = ev.subtitle.as_ref().expect("the subtitle row").clone();
    assert_eq!(
        app2.sounds.subtitle_plays.len(),
        1,
        "the fresh text the entry adds"
    );
    assert_eq!(
        app2.sounds.subtitle_plays[0].text_key, key,
        "the sound table's own subtitle key the row wears"
    );
    app2.sounds
        .record_subtitle(9, "stone", crate::sound::Lane::Break, 1.5, 2.5, 3.5);
    assert_eq!(
        app2.sounds.subtitle_plays.len(),
        1,
        "the same text the merge keeps"
    );
    assert_eq!(
        app2.sounds.subtitle_plays[0].plays.len(),
        1,
        "the same location the refresh keeps"
    );
    assert_eq!(
        app2.sounds.subtitle_plays[0].plays[0].tick, 9,
        "the refresh the newest stamp wears"
    );
    app2.sounds
        .record_subtitle(11, "stone", crate::sound::Lane::Break, 5.5, 2.5, 3.5);
    assert_eq!(
        app2.sounds.subtitle_plays[0].plays.len(),
        2,
        "the new location the play adds"
    );
    // THE RENDER'S OWN LAYER (the default FALSE the silence the
    // goldens' own law keeps — the TRUE the band + the text the
    // bottom-right anchor draws, the first row's band the h-40 top
    // wears)
    let root = crate::screens::tests::assets_root();
    let mut app3 = App::new(262);
    app3.enter_world();
    app3.spawn_offline_body();
    let (bx, by, bz) = {
        let b = app3.body.as_ref().expect("the offline body");
        (b.x, b.y, b.z)
    };
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    app3.show_subtitles = false;
    // THE SAME-CELL PLAY (the body's own feet the closest lane wears —
    // the within-range the audible filter keeps; the tick 0 the boot's
    // own clock wears — the frame the update's walk advances, the
    // render's read the same tick keeps)
    app3.sounds.record_subtitle(
        0,
        "stone",
        crate::sound::Lane::Break,
        bx + 0.5,
        by + 0.5,
        bz + 0.5,
    );
    app3.render(&mut fb, &mut assets);
    let bare = fb.color.clone();
    app3.show_subtitles = true;
    app3.render(&mut fb, &mut assets);
    let diff = diff_count(&bare, &fb.color, 500, 314, 140, 24);
    assert!(diff > 20, "the captions the band + text paint: {diff}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE BG-OPACITY GATE'S OWN LANES (the 26.2 bgopacity_lanes_180 probe
/// the walk banks — the `Options.getBackgroundOpacity(F)F` law the
/// bytecode's `ifeq -> fload_1` / `textBackgroundOpacity` pair speaks,
/// the `ARGB.as8BitChannel` the `Mth.floor(f * 255)` fold the byte the
/// band paints keeps, the `getBackgroundColor(F)I` overload the Hud
/// sidebar's 0.3F/0.4F and the SubtitleOverlay's 0.8F walks wear, the
/// `getBackgroundColor(I)I` the tab row's 0x20FFFFFF operand carries —
/// the raw/Everywhere pair the three lanes the gates keep).
#[test]
fn the_bg_opacity_gate_the_lanes_fold_wears() {
    // THE (F)F LAW'S OWN PAIR (the TRUE lane the raw arg verbatim, the
    // Everywhere lane the textbg ALONE — the arg the lane discards,
    // the multiply the bytecode refuses)
    assert_eq!(bg_opacity_gate(true, 0.5, 0.3), 0.3, "the TRUE's own raw");
    assert_eq!(
        bg_opacity_gate(false, 0.5, 0.3),
        0.5,
        "the Everywhere lane the textbg alone the arg discards"
    );
    assert_eq!(
        bg_opacity_gate(false, 0.25, 0.8),
        0.25,
        "the raw the fold never multiplies"
    );
    // THE BAND FOLD'S OWN BYTES (the floor law the sidebar's raw lane
    // keeps — the 0.3 the floor(76.5) = 76, the 0.4 the 102, the 0.8
    // the 204 the subtitle's band would wear, the Everywhere lane the
    // floor(0.5 × 255) = 127 the textbg fold speaks — the raw
    // `f * 255 + 0.5` round the old guess wears the bytecode refuses)
    assert_eq!(
        bg_band_alpha(true, 0.5, 0.3),
        76.0 / 255.0,
        "the header band's own byte"
    );
    assert_eq!(
        bg_band_alpha(true, 0.5, 0.4),
        102.0 / 255.0,
        "the body band's own byte"
    );
    assert_eq!(
        bg_band_alpha(true, 0.5, 0.8),
        204.0 / 255.0,
        "the subtitle band's own byte"
    );
    assert_eq!(
        bg_band_alpha(false, 0.5, 0.3),
        127.0 / 255.0,
        "the Everywhere fold the floor law keeps"
    );
    // THE ROW GATE'S OWN PAIR (the TRUE lane the raw ARGB verbatim —
    // the 0x20FFFFFF the tab row keeps — the Everywhere lane the
    // textbg-folded black the colorFromFloat rows speak)
    assert_eq!(
        bg_row_gate(true, 0.5, TAB_ROW_BG),
        TAB_ROW_BG,
        "the TRUE lane the raw verbatim"
    );
    assert_eq!(
        bg_row_gate(false, 0.5, TAB_ROW_BG),
        [0.0, 0.0, 0.0, 127.0 / 255.0],
        "the Everywhere lane the folded black"
    );
    // THE DEFAULT'S OWN NO-CHANGE (the boot defaults the gates the
    // byte-identical fold keep — the bgOnly TRUE the raw lane the
    // constants' own bytes answer, the render the goldens never see
    // the flip speak)
    let app = App::new(262);
    assert!(
        app.background_for_chat_only,
        "the boot's own TRUE the raw lane arms"
    );
    assert_eq!(
        bg_band_alpha(
            app.background_for_chat_only,
            app.text_background_opacity,
            0.3
        ),
        SIDEBAR_HEADER_BG[3],
        "the default fold the header's own byte wears"
    );
    assert_eq!(
        bg_band_alpha(
            app.background_for_chat_only,
            app.text_background_opacity,
            0.4
        ),
        SIDEBAR_BODY_BG[3],
        "the default fold the body's own byte wears"
    );
}

/// THE NOTIFICATIONS' OWN ROW (the 26.2 notifications_183 probes the
/// walk banks — the `notificationDisplayTime` option the Options
/// ctor's dconst_1 pc 1352 speaks, the default 1.0 the dconst_1 keeps,
/// the `Codec.doubleRange(0.5, 10.0)` codec the strict double wears,
/// the value set the `IntRange(5, 100).xmap(i -> i / 10.0, v ->
/// (int)(v * 10.0), true)` slider walk the 96 seats keeps, the caption
/// the lambda$new$49 law the `genericValueLabel(label,
/// translatable("options.multiplier", value))` composition speaks —
/// the lang rows the "Notification Time" + "%sx" pair the jar's en_us
/// table keeps, the accessibility array the index-9 seat the row
/// BETWEEN the chat-lines and the fov seats hosts (7 < 9 < 12 the
/// order speaks); the processOptions the ldc@682 the visit seat keeps
/// — the walk's LAST carried seat the file's own END row answers; the
/// consumer the SubtitleOverlay's extractRenderState dstore-9 walk
/// speaks — the `3000.0 * displayTime` window multiplier the
/// subtitle rows' own age fold wears, the doubled value the doubled
/// window keeps).
#[test]
fn the_notification_row_the_multiplier_wears() {
    fn diff_count(a: &[u8], b: &[u8], x0: i32, y0: i32, w: i32, h: i32) -> usize {
        let mut n = 0usize;
        for y in y0..y0 + h {
            for x in x0..x0 + w {
                let o = ((y as usize) * 640 + x as usize) * 3;
                if a[o..o + 3] != b[o..o + 3] {
                    n += 1;
                }
            }
        }
        n
    }
    let dir = std::env::temp_dir().join(format!("mc2rs_notif_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE SUB-SCREEN'S OWN SEAT (the accessibility row the opener
    // wears — the entry save the removed() moment the file's rows
    // keep, the DEFAULT 1.0 the multiplier's row speaks)
    app.open(ScreenKind::Options);
    app.open(ScreenKind::Accessibility);
    let pre_exit = std::fs::read_to_string(&path).expect("the entry save wrote");
    assert!(
        pre_exit.contains("notificationDisplayTime:1.0\n"),
        "the removed() moment the entry save keeps ({pre_exit})"
    );
    // THE FILE ROW'S OWN ADJACENCY (the visit-order seat the
    // processOptions walk banks — the notificationDisplayTime the
    // ldc@682 the walk's LAST CARRIED OPTION seat wears, the
    // backgroundForChatOnly row's AFTER answers; THE KEY BLOCK THE
    // 186 ROUND BANKS: the key_ rows the option rows' AFTER the
    // processOptions key loop the vanilla ladder wears — the option
    // rows the block the row's END never speaks)
    let nd_pos = pre_exit.find("notificationDisplayTime:").expect("the row");
    let bg_pos = pre_exit.find("backgroundForChatOnly:").expect("the row");
    let key_pos = pre_exit.find("key_key.forward:").expect("the key block");
    assert!(
        bg_pos < nd_pos && nd_pos < key_pos,
        "the visit-order adjacency the file rows keep"
    );
    assert!(
        pre_exit
            .trim_end()
            .ends_with("key_key.debug.lightmapTexture:key.keyboard.4"),
        "the key block the file's own END row wears"
    );
    // THE ROW'S OWN SEAT (the array's index 9 the row BETWEEN the
    // chat-lines and the fov seats wears — index 7 < index 9 < index
    // 12 the order keeps) + THE DEFAULT CAPTION (the lambda$new$49
    // composition the "1.0x" multiplier word wears)
    app.render_title_buttons_for_test();
    let ls = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_chat_line_spacing")
        .expect("the line spacing row")
        .clone();
    let nd = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_notification")
        .expect("the notification row")
        .clone();
    let fes = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_fov_effect")
        .expect("the fov row")
        .clone();
    assert!(
        ls.y < nd.y && nd.y < fes.y,
        "the index-9 row the between-seat wears"
    );
    assert_eq!(
        nd.label, "Notification Time: 1.0x",
        "the lang rows' own words the multiplier caption keeps"
    );
    // THE SLIDER'S OWN SEATS (the IntRange(5, 100).xmap walk the 96
    // seats keep — the track's left handle seat the 0.5 the range's own
    // floor answers, the right the 10.0 the ceiling speaks, the middle
    // the 5.3 the seat grid's own 0.1 step wears, the caption the
    // java_double decimal shape wears; the 4px handle inset the
    // AbstractSliderButton's own 8px margin keeps)
    app.on_press(nd.x + 4, nd.y + nd.h / 2);
    assert_eq!(
        app.notification_display_time, 0.5,
        "the track's left handle the floor seat wears"
    );
    app.render_title_buttons_for_test();
    let nd = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_notification")
        .expect("the row")
        .clone();
    assert_eq!(
        nd.label, "Notification Time: 0.5x",
        "the floor seat the caption speaks"
    );
    app.on_press(nd.x + 70, nd.y + nd.h / 2);
    assert_eq!(
        app.notification_display_time, 5.3,
        "the track's middle the seat grid wears"
    );
    app.render_title_buttons_for_test();
    let nd = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_notification")
        .expect("the row")
        .clone();
    assert_eq!(
        nd.label, "Notification Time: 5.3x",
        "the middle seat the caption speaks"
    );
    app.on_press(nd.x + 136, nd.y + nd.h / 2);
    assert_eq!(
        app.notification_display_time, 10.0,
        "the track's right handle the ceiling seat wears"
    );
    app.render_title_buttons_for_test();
    let nd = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_notification")
        .expect("the row")
        .clone();
    assert_eq!(
        nd.label, "Notification Time: 10.0x",
        "the ceiling seat the caption speaks"
    );
    // THE ESCAPE'S OWN LANE (the shouldCloseOnEsc default the parent
    // ride returns — the base class's own `OptionsSubScreen.removed()`
    // the exit's save the file keeps, the 185 recon the family banks)
    // + THE PARENT'S OWN EXIT (the removed() moment the save the
    // slider's seats bank — the restart the row outlives)
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::Options,
        "the parent the Escape ride returns"
    );
    app.open(ScreenKind::Title);
    let post = std::fs::read_to_string(&path).expect("the parent exit wrote");
    assert!(
        post.contains("notificationDisplayTime:10.0\n"),
        "the slider's own ceiling the parent exit banks"
    );
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.notification_display_time, 10.0,
        "the restart the ceiling row the decode keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
    // THE WINDOW'S OWN FEED (the extractRenderState dstore-9 walk the
    // store's own value wears — the doubled multiplier the doubled
    // window keeps: the age-90 play the 2.0x window's INSIDE the row
    // paints, the default 1.0x window the same play the purge
    // removes — the feed the goldens' default silence keeps)
    let root = crate::screens::tests::assets_root();
    let mut app3 = App::new(262);
    app3.enter_world();
    app3.spawn_offline_body();
    let (bx, by, bz) = {
        let b = app3.body.as_ref().expect("the offline body");
        (b.x, b.y, b.z)
    };
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    app3.show_subtitles = true;
    app3.sounds.record_subtitle(
        0,
        "stone",
        crate::sound::Lane::Break,
        bx + 0.5,
        by + 0.5,
        bz + 0.5,
    );
    app3.frame = 90;
    app3.render(&mut fb, &mut assets);
    let bare = fb.color.clone();
    // THE DEFAULT'S OWN WINDOW (the 1.0x the 60-tick window the age-90
    // play the purge answers — the store's own default the row's
    // silence keeps; the second render the same silence the fold's
    // idempotence speaks)
    app3.notification_display_time = 1.0;
    app3.render(&mut fb, &mut assets);
    assert_eq!(
        diff_count(&bare, &fb.color, 0, 300, 640, 60),
        0,
        "the default window the stale play the purge keeps"
    );
    // THE DOUBLED'S OWN WINDOW (the 2.0x the 120-tick window the
    // age-90 play the INSIDE answers — the store's own value the
    // overlay's rows paint)
    app3.notification_display_time = 2.0;
    app3.render(&mut fb, &mut assets);
    let diff = diff_count(&bare, &fb.color, 0, 300, 640, 60);
    assert!(diff > 20, "the doubled window the row paints: {diff}");
}

// THE CONTROLS SCREEN'S OWN PAIR (the autopush-185 round — the
// OptionsScreen's `options.controls` button the first addSmall row
// the openScreenButton walk builds, the ControlsScreen the
// OptionsSubScreen family extends: the addOptions walk the addSmall
// PAIR builds (the LEFT `options.mouse_settings` the
// `lambda$addOptions$0` walk the MouseSettingsScreen opens, the RIGHT
// `controls.keybinds` the `lambda$addOptions$1` walk the
// KeyBindsScreen opens), the MouseSettings the sensitivity index-0
// seat the store's own slider wears, the KeyBinds the EMPTY list the
// client's keyMappings the store never carried keeps + the Reset
// Keys/Done footer pair, and the SAVE LAW the base class's own
// `OptionsSubScreen.removed()` the whole family inherits — every
// sub-screen exit the file's rows keep)
#[test]
fn the_controls_pair_the_sub_screens_the_family_save_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_controls_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE OPENER'S OWN ROW (the title lane the pair wears — the
    // `options.controls` word the lang table speaks, the LEFT seat
    // the first row's own precedence keeps)
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let ctl = app
        .buttons
        .iter()
        .find(|b| b.id == "controls")
        .expect("the controls opener")
        .clone();
    assert_eq!(
        ctl.label, "Controls...",
        "the `options.controls` word the lang table speaks"
    );
    let acc = app
        .buttons
        .iter()
        .find(|b| b.id == "accessibility")
        .expect("the pair's right seat")
        .clone();
    assert!(
        ctl.x + ctl.w / 2 < acc.x + acc.w / 2,
        "the pair the LEFT/RIGHT seats keep"
    );
    // THE ENTRY SAVE THE OPTIONS EXIT BANKS (the old==Options head
    // the removed() moment the file's rows keep — the DEFAULT 0.5
    // the mouseSensitivity row speaks)
    app.on_click(ctl.x + ctl.w / 2, ctl.y + ctl.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::Controls,
        "the sub-screen the opener wears"
    );
    let entry = std::fs::read_to_string(&path).expect("the entry save wrote");
    assert!(
        entry.contains("mouseSensitivity:0.5\n"),
        "the options exit the removed() moment banks ({entry})"
    );
    // THE PAIR'S OWN SEATS (the addOptions walk the addSmall pair
    // keeps — the LEFT the mouse settings caption the cp#44 utf8's
    // own row speaks, the RIGHT the keybinds caption the cp#68 utf8
    // wears; the 7-toggle rows the store the client never carried —
    // the pair row the ONLY seat the list hosts)
    app.render_title_buttons_for_test();
    let ms = app
        .buttons
        .iter()
        .find(|b| b.id == "mousesettings")
        .expect("the pair's left seat")
        .clone();
    let kb = app
        .buttons
        .iter()
        .find(|b| b.id == "keybinds")
        .expect("the pair's right seat")
        .clone();
    assert_eq!(
        ms.label, "Mouse Settings...",
        "the `options.mouse_settings` word the pair speaks"
    );
    assert_eq!(
        kb.label, "Key Binds...",
        "the `controls.keybinds` word the pair speaks"
    );
    assert!(
        app.buttons.iter().any(|b| b.id == "controlsdone"),
        "the Done footer the base class's own walk keeps"
    );
    // THE LEFT SEAT'S OWN WALK (the lambda$addOptions$0 supplier the
    // click answers — the MouseSettingsScreen the new
    // ControlsScreen(this, options) parent wears; the ENTRY SAVE the
    // family gate's Controls arm banks)
    app.on_click(ms.x + ms.w / 2, ms.y + ms.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::MouseSettings,
        "the mouse settings the left seat opens"
    );
    // THE SENSITIVITY ROW'S OWN SEAT (the Options.sensitivity()
    // index-0 the array's own walk keeps — the caption the `*2.0`
    // percent law the default's 100% word speaks, the value the
    // slider's own unit wears)
    app.render_title_buttons_for_test();
    let sens = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_ms_sensitivity")
        .expect("the sensitivity row")
        .clone();
    assert_eq!(
        sens.label, "Sensitivity: 100%",
        "the default's own percent word"
    );
    assert_eq!(sens.value, Some(0.5), "the slider the store wears");
    // THE SLIDER'S OWN LAW (the same walk the Options screen's row
    // wears — the press inside the track arms, the drag the store's
    // own seat follows, the release the disarm speaks, the disarmed
    // drag never moves)
    app.on_press(sens.x + 4, sens.y + sens.h / 2);
    assert_eq!(app.look_sensitivity, 0.0, "the left edge clamps");
    assert!(app.sensitivity_drag, "the press inside the track arms");
    app.render_title_buttons_for_test();
    let sens = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_ms_sensitivity")
        .expect("the row")
        .clone();
    assert_eq!(
        sens.label, "Sensitivity: *yawn*",
        "the 0.0 the min end wears"
    );
    app.on_sensitivity_slider_drag(sens.x + sens.w / 2);
    assert_eq!(
        app.look_sensitivity, 0.5,
        "the mid-track drag the half wears"
    );
    app.on_left_release();
    assert!(!app.sensitivity_drag, "the release disarms");
    app.on_sensitivity_slider_drag(sens.x + 4);
    assert_eq!(app.look_sensitivity, 0.5, "the disarmed drag never moves");
    // THE DONE FOOTER'S OWN RETURN (the onClose walk the msdone id
    // answers — the lastScreen the CONTROLS screen wears; THE FAMILY
    // SAVE THE BASE CLASS BANKS: the MouseSettings exit the inherited
    // `OptionsSubScreen.removed()` the file's rows keep)
    app.look_sensitivity = 0.75;
    app.render_title_buttons_for_test();
    let done = app
        .buttons
        .iter()
        .find(|b| b.id == "msdone")
        .expect("the done footer")
        .clone();
    app.on_click(done.x + done.w / 2, done.y + done.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::Controls,
        "the parent the Done wears"
    );
    let mid = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        mid.contains("mouseSensitivity:0.75\n"),
        "the inherited removed() the MouseSettings exit banks ({mid})"
    );
    // THE RIGHT SEAT'S OWN WALK (the lambda$addOptions$1 supplier the
    // click answers — the KeyBindsScreen the empty list the client's
    // keyMappings the store never carried keeps, the footer pair the
    // addFooter override wears)
    app.render_title_buttons_for_test();
    let kb = app
        .buttons
        .iter()
        .find(|b| b.id == "keybinds")
        .expect("the right seat")
        .clone();
    app.on_click(kb.x + kb.w / 2, kb.y + kb.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::KeyBinds,
        "the key binds the right seat opens"
    );
    app.render_title_buttons_for_test();
    let reset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_reset")
        .expect("the reset footer")
        .clone();
    let kbd = app
        .buttons
        .iter()
        .find(|b| b.id == "kbdone")
        .expect("the done footer")
        .clone();
    assert_eq!(
        reset.label, "Reset Keys",
        "the `controls.resetAll` word the footer speaks"
    );
    assert_eq!(kbd.label, "Done", "the GUI_DONE word the pair wears");
    assert!(
        reset.x + reset.w / 2 < kbd.x + kbd.w / 2,
        "the footer pair the LEFT/RIGHT seats keep"
    );
    // THE RESET'S OWN WALK (the lambda$addFooter$0 the array over —
    // setKey(getDefaultKey()) the each-row answer; the EMPTY array
    // the loop the zero rows the honest no-op keeps)
    app.on_click(reset.x + reset.w / 2, reset.y + reset.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::KeyBinds,
        "the empty array the reset the same screen keeps"
    );
    // THE ESCAPE'S OWN LANE (the shouldCloseOnEsc default the
    // keyPressed walk answers — the lastScreen the CONTROLS ride
    // returns; the family save the inherited removed() banks)
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::Controls,
        "the parent the Escape ride returns"
    );
    // THE CONTROLS SCREEN'S OWN ESCAPE (the same default — the
    // lastScreen the parent Options wears; the family save the same
    // law the exit keeps)
    app.on_escape();
    assert_eq!(
        app.screen,
        ScreenKind::Options,
        "the Options screen the Controls escape wears"
    );
    let post = std::fs::read_to_string(&path).expect("the exits wrote");
    assert!(
        post.contains("mouseSensitivity:0.75\n"),
        "the family's own exits the rows keep ({post})"
    );
    // THE RESTART'S OWN ROW (the load the file's parse rides — the
    // 0.75 the decode keeps)
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.look_sensitivity, 0.75,
        "the restart the sub-screen's row the decode keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// THE KEYMAPPINGS' OWN STORE (the autopush-186 round — the Options
// ctor's registrations pc 2402-2545 the movement rows the array order
// keeps: key.forward/W, key.left/A, key.back/S, key.right/D,
// key.jump/Space, the ToggleKeyMapping pair the sneak/sprint rows
// wear; the KeyBindsScreen the rows the KeyEntry shape renders (the
// name the left label + the 75x20 change + the 50x20 reset the
// !isDefault active law keeps), the rebind walk the selectedKey the
// keyPressed law serves (the pressed key the bind wears, the ESCAPE
// the UNKNOWN clear the modern law speaks — NOT a cancel), the
// reset-all the lambda$addFooter$0 array walk answers, the key_ rows
// the processOptions key loop the file ladder wears, the restart the
// decode keeps)
#[test]
fn the_key_binds_the_rows_the_rebind_the_file_wear() {
    let dir = std::env::temp_dir().join(format!("mc2rs_keybinds_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE OPENER'S OWN WALK (the pair's RIGHT seat the Controls screen
    // keeps — the KeyBindsScreen the lambda$addOptions$1 walk opens)
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let ctl = app
        .buttons
        .iter()
        .find(|b| b.id == "controls")
        .expect("the controls opener")
        .clone();
    app.on_click(ctl.x + ctl.w / 2, ctl.y + ctl.h / 2);
    assert_eq!(app.screen, ScreenKind::Controls, "the pair's parent wears");
    app.render_title_buttons_for_test();
    let kb = app
        .buttons
        .iter()
        .find(|b| b.id == "keybinds")
        .expect("the pair's right seat")
        .clone();
    app.on_click(kb.x + kb.w / 2, kb.y + kb.h / 2);
    assert_eq!(
        app.screen,
        ScreenKind::KeyBinds,
        "the list the right seat opens"
    );
    // THE ROWS' OWN RENDER (the taller viewport the 7 rows the band
    // keeps — the list the clip the smaller windows the honest partial
    // renders)
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 260);
    app.render(&mut fb, &mut assets);
    // THE DEFAULTS' OWN CAPTIONS (the getTranslatedKeyMessage walk —
    // the ctor's codes the lang rows the displays answer)
    let fwd = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_forward")
        .expect("the forward row")
        .clone();
    assert_eq!(fwd.label, "W", "the default's own display speaks");
    assert_eq!(
        app.buttons
            .iter()
            .find(|b| b.id == "kb_change_left")
            .expect("the left row")
            .label,
        "A",
        "the left the default wears"
    );
    assert_eq!(
        app.buttons
            .iter()
            .find(|b| b.id == "kb_change_jump")
            .expect("the jump row")
            .label,
        "Space",
        "the jump the default wears"
    );
    assert_eq!(
        app.buttons
            .iter()
            .find(|b| b.id == "kb_change_sprint")
            .expect("the sprint row")
            .label,
        "Left Control",
        "the sprint the default wears"
    );
    assert_eq!(
        app.buttons
            .iter()
            .find(|b| b.id == "kb_rowreset_sprint")
            .expect("the sprint reset")
            .label,
        "Reset",
        "the `controls.reset` word the row speaks"
    );
    // THE RESET'S OWN ACTIVE SEAT (the refreshEntry law — the button
    // ALWAYS renders, the active seat the !isDefault arms: the default
    // binding the grey keeps)
    assert!(
        !app.buttons
            .iter()
            .find(|b| b.id == "kb_rowreset_forward")
            .expect("the forward reset")
            .active,
        "the default the inactive seat keeps"
    );
    // THE FOOTER'S OWN PAIR (the 185 test's law keeps)
    assert_eq!(
        app.buttons
            .iter()
            .find(|b| b.id == "kb_reset")
            .expect("the reset-all footer")
            .label,
        "Reset Keys",
        "the `controls.resetAll` word the footer speaks"
    );
    assert_eq!(
        app.buttons
            .iter()
            .find(|b| b.id == "kbdone")
            .expect("the done footer")
            .label,
        "Done",
        "the GUI_DONE word the pair wears"
    );
    // THE REBIND'S OWN WALK (the selectedKey the change click arms —
    // the wrap the selected row's caption wears)
    app.on_click(fwd.x + fwd.w / 2, fwd.y + fwd.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(crate::options_file::key_binds::FORWARD),
        "the selectedKey the change click arms"
    );
    app.render(&mut fb, &mut assets);
    let fwd = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_forward")
        .expect("the row")
        .clone();
    assert_eq!(fwd.label, "> W <", "the wrap the selected row wears");
    // THE CAPTURE'S OWN LAW (the keyPressed walk the method serves —
    // the pressed key the bind wears, the selected the null the walk
    // clears)
    assert!(
        app.kb_capture(Some(minifb::Key::L)),
        "the pressed key the capture consumes"
    );
    assert_eq!(
        app.key_binds[crate::options_file::key_binds::FORWARD],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::L
        )),
        "the bind the capture writes"
    );
    assert_eq!(app.kb_selected, None, "the selected the walk clears");
    app.render(&mut fb, &mut assets);
    let fwd = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_forward")
        .expect("the row")
        .clone();
    assert_eq!(fwd.label, "L", "the new bind the display speaks");
    // THE RESET'S OWN ACTIVE SEAT (the changed binding the arms — the
    // press the default restores, the lambda$new$2 walk answers)
    let rfwd = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_forward")
        .expect("the reset")
        .clone();
    assert!(rfwd.active, "the changed binding the active seat wears");
    app.on_click(rfwd.x + rfwd.w / 2, rfwd.y + rfwd.h / 2);
    assert_eq!(
        app.key_binds[crate::options_file::key_binds::FORWARD],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::W
        )),
        "the default the reset restores"
    );
    app.render(&mut fb, &mut assets);
    // THE ESCAPE'S OWN CLEAR (the modern law — the isEscape arm the
    // UNKNOWN seat answers, NOT a cancel)
    let fwd = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_forward")
        .expect("the row")
        .clone();
    app.on_click(fwd.x + fwd.w / 2, fwd.y + fwd.h / 2);
    assert!(
        app.kb_capture(Some(minifb::Key::Escape)),
        "the escape the capture consumes"
    );
    assert_eq!(
        app.key_binds[crate::options_file::key_binds::FORWARD],
        None,
        "the UNKNOWN seat the clear wears"
    );
    // THE NO-SELECTION'S OWN SILENCE (the null selectedKey the walk
    // never claims — the false the frame keeps)
    assert!(
        !app.kb_capture(Some(minifb::Key::L)),
        "the no-selection the capture refuses"
    );
    app.render(&mut fb, &mut assets);
    let fwd = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_forward")
        .expect("the row")
        .clone();
    assert_eq!(
        fwd.label, "Not Bound",
        "the `key.keyboard.unknown` row the unbound display speaks"
    );
    // THE RESET-ALL'S OWN WALK (the lambda$addFooter$0 the array over
    // — the DEFAULTS the each-row answer; the unbound sneak the same
    // restore wears)
    app.key_binds[crate::options_file::key_binds::SNEAK] = Some(
        crate::options_file::key_binds::Bind::Keyboard(minifb::Key::K),
    );
    let ra = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_reset")
        .expect("the footer")
        .clone();
    app.on_click(ra.x + ra.w / 2, ra.y + ra.h / 2);
    assert_eq!(
        app.key_binds,
        crate::options_file::key_binds::DEFAULTS,
        "the array walk the defaults restore"
    );
    assert_eq!(app.kb_selected, None, "the reset-all the selection clears");
    // THE FILE'S OWN ROWS (the family save the exit banks — the
    // rebinds the `key_` rows the block wears)
    app.key_binds[crate::options_file::key_binds::FORWARD] = Some(
        crate::options_file::key_binds::Bind::Keyboard(minifb::Key::L),
    );
    app.key_binds[crate::options_file::key_binds::SNEAK] = None;
    app.open(ScreenKind::Controls);
    let text = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        text.contains("key_key.forward:key.keyboard.l\n"),
        "the rebind the file row keeps ({text})"
    );
    assert!(
        text.contains("key_key.sneak:key.keyboard.unknown\n"),
        "the unbound the UNKNOWN row wears"
    );
    // THE RESTART'S OWN DECODE (the fresh boot the file's truth wears)
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.key_binds[crate::options_file::key_binds::FORWARD],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::L
        )),
        "the restart the rebind the decode keeps"
    );
    assert_eq!(
        re.key_binds[crate::options_file::key_binds::SNEAK],
        None,
        "the restart the unbound row the decode keeps"
    );
    assert_eq!(
        re.key_binds[crate::options_file::key_binds::SPRINT],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::LeftCtrl
        )),
        "the restart the missing row the default keeps"
    );
    // THE ESCAPE'S OWN SCREEN LANE (the no-selected walk — the parent
    // the lastScreen ride returns, the 185 test's law keeps)
    re.open(ScreenKind::Options);
    re.open(ScreenKind::Controls);
    re.open(ScreenKind::KeyBinds);
    re.on_escape();
    assert_eq!(
        re.screen,
        ScreenKind::Controls,
        "the parent the Escape ride returns"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE COMPARETO'S OWN RE-SEAT (the autopush-226 walk): the
/// KeyBindsList ctor's `Arrays.sort` the vanilla law wears — the
/// `KeyMapping.compareTo` the three-key ladder the rows' seats
/// answer (the category the SORT_ORDER ordinal the group boundaries
/// the render's own seats keep, the orderI the WITHIN-group primary
/// the ORDER_I bank wears, the orderI=0 ties the `I18n.get(name)`
/// alphabetical secondary the re-seat answers): the movement group
/// the Jump-first re-seat wears (the forward row the LAST slot), the
/// gameplay trio the Attack/Pick/Use walk, the inventory trio the
/// Drop-first seat, the creative pair the Load/Save swap the
/// alphabetical law speaks, the debug group the overlay(-2)/
/// modifier(-1) the BEFORE seats keep, the orderI=0 rows the
/// Clear Chat..Toggle Lost Focus Pause the alphabetical walk wears,
/// the charts the orderI 1..4 the AFTER slots bank — the re-seat
/// the SEAT order alone re-arms (the ids the rows' own words keep,
/// the rebind walk the same ids the capture serves, the scroll the
/// content height the same answer keeps).
#[test]
fn the_key_binds_the_compareto_the_orderI_the_name_reseat_wears() {
    use crate::options_file::key_binds as kb;
    let mut app = App::new(262);
    app.open(ScreenKind::KeyBinds);
    // THE TALL BAND'S OWN WALK (the 1300 viewport the whole list the
    // visible band keeps — the 48 rows the clip the full render
    // wears, the seats the button y the honest answer speaks)
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 1300);
    app.render(&mut fb, &mut assets);
    let seat = |app: &App, id: &str| -> i32 {
        app.buttons
            .iter()
            .find(|b| b.id == id)
            .unwrap_or_else(|| panic!("the {id} row the list wears"))
            .y
    };
    // THE MOVEMENT GROUP'S OWN RE-SEAT (the orderI=0 ties the
    // alphabetical law answers — Jump < Sneak < Sprint < Strafe Left
    // < Strafe Right < Walk Backward < Walk Forward; the forward row
    // the registration head the LAST slot the re-seat wears)
    assert_eq!(
        seat(&app, "kb_change_jump"),
        kb::MOVEMENT_ROWS_START,
        "the Jump the first slot the alphabetical law wears"
    );
    assert_eq!(
        seat(&app, "kb_change_sneak"),
        kb::MOVEMENT_ROWS_START + kb::ROW_STRIDE,
        "the Sneak the second slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_sprint"),
        kb::MOVEMENT_ROWS_START + 2 * kb::ROW_STRIDE,
        "the Sprint the third slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_left"),
        kb::MOVEMENT_ROWS_START + 3 * kb::ROW_STRIDE,
        "the Strafe Left the fourth slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_right"),
        kb::MOVEMENT_ROWS_START + 4 * kb::ROW_STRIDE,
        "the Strafe Right the fifth slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_back"),
        kb::MOVEMENT_ROWS_START + 5 * kb::ROW_STRIDE,
        "the Walk Backward the sixth slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_forward"),
        kb::MOVEMENT_ROWS_START + 6 * kb::ROW_STRIDE,
        "the Walk Forward the LAST slot the re-seat wears"
    );
    // THE GAMEPLAY TRIO'S OWN RE-SEAT (the Attack/Destroy the
    // alphabetical head wears, the Pick Block the middle, the Use
    // Item/Place Block the tail the registration head never keeps)
    assert_eq!(
        seat(&app, "kb_change_attack"),
        kb::GAMEPLAY_ROWS_START,
        "the Attack the group's head wears"
    );
    assert_eq!(
        seat(&app, "kb_change_pickitem"),
        kb::GAMEPLAY_ROWS_START + kb::ROW_STRIDE,
        "the Pick Block the middle slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_use"),
        kb::GAMEPLAY_ROWS_START + 2 * kb::ROW_STRIDE,
        "the Use the group's tail wears"
    );
    // THE INVENTORY TRIO'S OWN RE-SEAT (the Drop Selected Item the
    // alphabetical head wears, the Open/Close Inventory the middle,
    // the Swap Item With Off Hand the tail)
    assert_eq!(
        seat(&app, "kb_change_drop"),
        kb::INVENTORY_ROWS_START,
        "the Drop the trio's head wears"
    );
    assert_eq!(
        seat(&app, "kb_change_inventory"),
        kb::INVENTORY_ROWS_START + kb::ROW_STRIDE,
        "the Inventory the middle slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_swapoffhand"),
        kb::INVENTORY_ROWS_START + 2 * kb::ROW_STRIDE,
        "the Swap the trio's tail wears"
    );
    // THE CREATIVE PAIR'S OWN SWAP (the Load Hotbar Activator the
    // alphabetical head wears — the registration order the pair the
    // Save-first walk never keeps)
    assert_eq!(
        seat(&app, "kb_change_loadactivator"),
        kb::CREATIVE_ROWS_START,
        "the Load the pair's head the swap wears"
    );
    assert_eq!(
        seat(&app, "kb_change_saveactivator"),
        kb::CREATIVE_ROWS_START + kb::ROW_STRIDE,
        "the Save the pair's tail keeps"
    );
    // THE SPECTATOR PAIR'S OWN SEAT (the Highlight Players the head
    // the alphabetical law keeps — the registration order the same
    // walk wears, the pair the honest answer keeps)
    assert_eq!(
        seat(&app, "kb_change_spectatoroutlines"),
        kb::SPECTATOR_ROWS_START,
        "the Highlight Players the head keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_spectatorhotbar"),
        kb::SPECTATOR_ROWS_START + kb::ROW_STRIDE,
        "the Select On Hotbar the tail keeps"
    );
    // THE DEBUG GROUP'S OWN LADDER (the orderI the -2/-1 the BEFORE
    // seats keep — the overlay the 726, the modifier the 750; the
    // orderI=0 ties the alphabetical walk the 774..1134 strides wear
    // (the Clear Chat the head, the Copy Data the Copy Location the
    // 'D'<'L' seat answers, the Show family the A/C/H the inner walk
    // keeps), the charts the orderI 1..4 the AFTER slots bank)
    assert_eq!(
        seat(&app, "kb_change_debugoverlay"),
        kb::DEBUG_ROWS_START,
        "the overlay the orderI -2 the head wears"
    );
    assert_eq!(
        seat(&app, "kb_change_debugmodifier"),
        kb::DEBUG_ROWS_START + kb::ROW_STRIDE,
        "the modifier the orderI -1 the second seat keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_debugclearchat"),
        kb::DEBUG_ROWS_START + 2 * kb::ROW_STRIDE,
        "the Clear Chat the orderI=0 head wears"
    );
    assert_eq!(
        seat(&app, "kb_change_debugcopyrecreate"),
        kb::DEBUG_ROWS_START + 3 * kb::ROW_STRIDE,
        "the Copy Data the second slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_debugcopylocation"),
        kb::DEBUG_ROWS_START + 4 * kb::ROW_STRIDE,
        "the Copy Location the third slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_debugspectate"),
        kb::DEBUG_ROWS_START + 5 * kb::ROW_STRIDE,
        "the Cycle Spectator the fourth slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_debugcrash"),
        kb::DEBUG_ROWS_START + 6 * kb::ROW_STRIDE,
        "the Debug Crash the fifth slot keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_debugshowhitboxes"),
        kb::DEBUG_ROWS_START + 15 * kb::ROW_STRIDE,
        "the Show Hitboxes the Show-family tail wears"
    );
    assert_eq!(
        seat(&app, "kb_change_debugfocuspause"),
        kb::DEBUG_ROWS_START + 17 * kb::ROW_STRIDE,
        "the Toggle Lost Focus Pause the orderI=0 tail wears"
    );
    assert_eq!(
        seat(&app, "kb_change_debugprofilingchart"),
        kb::DEBUG_ROWS_START + 18 * kb::ROW_STRIDE,
        "the orderI 1 the charts' head wears"
    );
    assert_eq!(
        seat(&app, "kb_change_debugfpscharts"),
        kb::DEBUG_ROWS_START + 19 * kb::ROW_STRIDE,
        "the orderI 2 the second chart keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_debugnetworkcharts"),
        kb::DEBUG_ROWS_START + 20 * kb::ROW_STRIDE,
        "the orderI 3 the third chart keeps"
    );
    assert_eq!(
        seat(&app, "kb_change_debuglightmaptexture"),
        kb::DEBUG_ROWS_START + 21 * kb::ROW_STRIDE,
        "the orderI 4 the LAST slot wears"
    );
}

/// THE KEY BINDS' OWN SORT-KEY CACHE (the autopush-239 walk): the
/// `kb_seated` the per-frame sort the OnceLock static table answers —
/// the vanilla KeyBindsList ctor's own once-ness the shape serves
/// (the Arrays.sort the <init> speaks once, the list the rows the
/// sorted array carries, the rebind the bound key alone re-wears),
/// the seven group renders the static's own filter the seats wear.
#[test]
fn the_key_binds_the_seated_static_table_the_per_frame_sort_answers() {
    use crate::options_file::key_binds as kb;
    // THE STATIC'S OWN ONCE (two reads the SAME order wear — the
    // get_or_init the second read the sort never re-answers)
    let full = App::kb_seated_full();
    let again = App::kb_seated_full();
    assert_eq!(full, again, "the static table the same order serves");
    // THE PERMUTATION'S OWN LAW (the table 0..COUNT the each slot the
    // exact once wears — the sort the slot never loses, never twins)
    let mut sorted = full.clone();
    sorted.sort_unstable();
    let identity: Vec<usize> = (0..kb::COUNT).collect();
    assert_eq!(sorted, identity, "the table the full permutation keeps");
    // THE MONOTONE LAW (the adjacent pairs the comparator's own
    // verdict never flips — the ORDER_I the primary, the tr-name the
    // ties the secondary answers, the equal pair the stable law
    // never wears)
    for pair in full.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let ka = (
            kb::ORDER_I[a],
            tr(kb::NAMES[a]).unwrap_or(kb::NAME_FALLBACKS[a]),
        );
        let kb_ = (
            kb::ORDER_I[b],
            tr(kb::NAMES[b]).unwrap_or(kb::NAME_FALLBACKS[b]),
        );
        assert!(
            ka <= kb_,
            "the {}..{} adjacency the comparator's own order keeps",
            a,
            b
        );
    }
    // THE STABLE-SORT THEOREM'S OWN VERDICT (the seven render groups
    // the static's own filter the DIRECT per-group sort's own order
    // wears — the same comparator + the same ascending-index input
    // the within-group relative order both walks keep, the filter
    // the order carries verbatim; the direct walk the test's own
    // independent recomputation wears, the cache the theorem the
    // honest answer serves)
    let direct = |lo: usize, hi: usize| -> Vec<usize> {
        let mut rows: Vec<usize> = (lo..hi).collect();
        rows.sort_by(|&a, &b| {
            kb::ORDER_I[a].cmp(&kb::ORDER_I[b]).then_with(|| {
                let na = tr(kb::NAMES[a]).unwrap_or(kb::NAME_FALLBACKS[a]);
                let nb = tr(kb::NAMES[b]).unwrap_or(kb::NAME_FALLBACKS[b]);
                na.cmp(nb)
            })
        });
        rows
    };
    const GROUPS: [(usize, usize); 7] = [
        (0, kb::INVENTORY),
        (kb::INVENTORY, kb::USE),
        (kb::USE, kb::HOTBAR_1),
        (kb::HOTBAR_1, kb::SAVE_ACTIVATOR),
        (kb::SAVE_ACTIVATOR, kb::SPECTATOR_OUTLINES),
        (kb::SPECTATOR_OUTLINES, kb::DEBUG_OVERLAY),
        (kb::DEBUG_OVERLAY, kb::COUNT),
    ];
    for (lo, hi) in GROUPS {
        assert_eq!(
            App::kb_seated(lo, hi),
            direct(lo, hi),
            "the group [{lo},{hi}) the static's own filter the exact order wears"
        );
    }
    // THE MOVEMENT GROUP'S OWN HEAD/TAIL (the cached path the
    // alphabetical law keeps — the Jump the head, the Walk Forward
    // the registration head the LAST slot wears)
    let movement = App::kb_seated(0, kb::INVENTORY);
    assert_eq!(
        movement[0],
        kb::JUMP,
        "the Jump the head the cached path wears"
    );
    assert_eq!(
        movement[movement.len() - 1],
        kb::FORWARD,
        "the Walk Forward the tail keeps"
    );
    // THE DEBUG GROUP'S OWN LADDER (the orderI seats the static
    // keeps — the -2 overlay the head, the -1 modifier the second,
    // the orderI=0 rows the alphabetical middle, the 1..4 charts the
    // AFTER tail wears)
    let debug = App::kb_seated(kb::DEBUG_OVERLAY, kb::COUNT);
    assert_eq!(
        debug[0],
        kb::DEBUG_OVERLAY,
        "the overlay the orderI -2 the head wears"
    );
    assert_eq!(
        debug[1],
        kb::DEBUG_MODIFIER,
        "the modifier the orderI -1 the second seat keeps"
    );
    let tail = debug.len() - 4;
    assert_eq!(
        debug[tail],
        kb::DEBUG_PROFILING_CHART,
        "the orderI 1 the charts' head wears"
    );
    assert_eq!(
        debug[tail + 1],
        kb::DEBUG_FPS_CHARTS,
        "the orderI 2 the second chart keeps"
    );
    assert_eq!(
        debug[tail + 2],
        kb::DEBUG_NETWORK_CHARTS,
        "the orderI 3 the third chart keeps"
    );
    assert_eq!(
        debug[tail + 3],
        kb::DEBUG_LIGHTMAP_TEXTURE,
        "the orderI 4 the LAST slot wears"
    );
    // THE EMPTY BAND'S OWN SILENCE (the zero-width range the empty
    // order wears — the filter the nothing carries)
    assert!(
        App::kb_seated(5, 5).is_empty(),
        "the empty band the empty order wears"
    );
}

/// THE INVENTORY TRIO'S OWN ROUND (the autopush-187 walk): the trio
/// the Options ctor's registrations the movement pair's AFTER wear
/// (the keyInventory/keySwapOffhand/keyDrop rows the INVENTORY
/// category answers), the list the category boundary the SECOND
/// header wears, the SCROLL the wheel law the entries' shift reads
/// (the mouseScrolled walk the AbstractScrollArea verbatim serves —
/// the `yScroll * 4.0 * 6.0` step one 24px row answers, the clamp
/// the maxScrollAmount law keeps), the clipped rows the scroll the
/// band REACHES (the 320x240 render the trio the clip refuses, the
/// wheel the rows the band restores), the LANES the store's own keys
/// re-wear (the E/F/Q hardcoded reads retire — the rebind the lanes
/// honor), the file the trio's rows the exit saves (the array order
/// the sprint row's AFTER keeps), the restart the trio the decode
/// keeps.
#[test]
fn the_inventory_trio_the_scroll_the_lanes_the_file_wear() {
    let dir = std::env::temp_dir().join(format!("mc2rs_keybinds_trio_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE TRIO'S OWN DEFAULTS (the ctor's codes the store keeps)
    assert_eq!(
        app.key_binds[crate::options_file::key_binds::INVENTORY],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::E
        )),
        "the inventory the E default wears"
    );
    assert_eq!(
        app.key_binds[crate::options_file::key_binds::SWAP_OFFHAND],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::F
        )),
        "the swap the F default wears"
    );
    assert_eq!(
        app.key_binds[crate::options_file::key_binds::DROP],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::Q
        )),
        "the drop the Q default wears"
    );
    // THE SCROLL'S OWN LAW (the mouseScrolled walk the method serves —
    // the wheel UP at the top the clamp keeps, the wheel DOWN the row
    // the shift wears, the 4.0*6.0 step the 24px answer speaks)
    app.on_kb_scroll(1.0, 240);
    assert_eq!(
        app.kb_scroll, 0.0,
        "the wheel up at the top the clamp keeps"
    );
    app.on_kb_scroll(-1.0, 240);
    assert_eq!(app.kb_scroll, 24.0, "the one notch the one row wears");
    app.on_kb_scroll(-1000.0, 240);
    assert_eq!(
        app.kb_scroll, 1038.0,
        "the clamp the maxScroll law keeps (content 1220 - band 182)"
    );
    app.on_kb_scroll(1000.0, 240);
    assert_eq!(
        app.kb_scroll, 0.0,
        "the wheel up the top the clamp restores"
    );
    // THE TALL WINDOW'S OWN CLAMP (the maxScroll the max(0, ..) law
    // keeps; the 1220 content the 780-tall band the max 440 speaks
    // — the fpsCharts row the band the walk answers; the
    // THREE notches the 72 the walk wears — the
    // 320-under the max the clamp never speaks)
    app.on_kb_scroll(-1.0, 780);
    app.on_kb_scroll(-1.0, 780);
    app.on_kb_scroll(-1.0, 780);
    assert_eq!(app.kb_scroll, 72.0, "the tall band the max 200 wears");
    app.kb_scroll = 24.0;
    // THE OPENER'S OWN WALK (the fresh screen the list rebuilds — the
    // scrollAmount the zero the fresh seat wears)
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let ctl = app
        .buttons
        .iter()
        .find(|b| b.id == "controls")
        .expect("the controls opener")
        .clone();
    app.on_click(ctl.x + ctl.w / 2, ctl.y + ctl.h / 2);
    app.render_title_buttons_for_test();
    let kb = app
        .buttons
        .iter()
        .find(|b| b.id == "keybinds")
        .expect("the pair's right seat")
        .clone();
    app.on_click(kb.x + kb.w / 2, kb.y + kb.h / 2);
    assert_eq!(app.screen, ScreenKind::KeyBinds, "the list the seat opens");
    assert_eq!(app.kb_scroll, 0.0, "the fresh list the zero keeps");
    assert_eq!(app.kb_selected, None, "the fresh seat the null keeps");
    // THE SMALL BAND'S OWN CLIP (the real-client h=240 the trio the
    // footer band refuses — the rows the render's own walk never
    // pushes, the reach the scroll alone serves)
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 240);
    app.render(&mut fb, &mut assets);
    assert!(
        app.buttons
            .iter()
            .all(|b| !b.id.starts_with("kb_change_inventory")),
        "the clipped trio the band render refuses"
    );
    // THE SCROLL'S OWN REACH (the two notches the gameplay's first row
    // the band restores — the 222 y the 48 shift the 174 seat wears,
    // the header the boundary the shift rides; the re-seat the
    // Attack/Destroy row the group's own head the alphabetical law
    // wears)
    app.on_kb_scroll(-1.0, 240);
    app.on_kb_scroll(-1.0, 240);
    assert_eq!(app.kb_scroll, 48.0, "the two notches the two rows wear");
    app.render(&mut fb, &mut assets);
    let use_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_attack")
        .expect("the scrolled row the band restores")
        .clone();
    assert_eq!(
        use_row.label, "Left Button",
        "the default the display speaks"
    );
    assert_eq!(use_row.y, 174, "the shifted row the scroll seat wears");
    assert!(
        app.buttons.iter().any(|b| b.id == "kb_rowreset_attack"),
        "the reset the row's pair keeps"
    );
    // THE CLAMP'S OWN BOTTOM (the wheel down the max the walk keeps —
    // the content's own bottom the band's own bottom aligns, the last
    // row the fully-visible seat wears)
    app.on_kb_scroll(-1000.0, 240);
    assert_eq!(app.kb_scroll, 1038.0, "the wheel down the clamp keeps");
    app.render(&mut fb, &mut assets);
    assert!(
        app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debuglightmaptexture" && b.y + b.h <= 240 - 28),
        "the last row the max scroll the fully-visible seat wears"
    );
    // THE SCROLL'S OWN RETURN (the six notches the inventory row the
    // 186 seat the band restores — the re-seat the trio's own row the
    // middle slot the +24 shift answers, the rebind the row's own
    // walk needs)
    app.on_kb_scroll(1000.0, 240);
    for _ in 0..6 {
        app.on_kb_scroll(-1.0, 240);
    }
    assert_eq!(app.kb_scroll, 144.0, "the six notches the seat wears");
    app.render(&mut fb, &mut assets);
    // THE REBIND'S OWN WALK (the trio's change click the selectedKey
    // arms, the capture the bind wears — the lanes the store honor;
    // the re-render the row's own seat re-wears, the stale clone the
    // scroll's own shift retires, the fresh find the 186 law keeps)
    let inv = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_inventory")
        .expect("the re-rendered row")
        .clone();
    app.on_click(inv.x + inv.w / 2, inv.y + inv.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(crate::options_file::key_binds::INVENTORY),
        "the trio's change the selectedKey arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::L)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[crate::options_file::key_binds::INVENTORY],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::L
        )),
        "the rebind the store writes"
    );
    // THE FILE'S OWN ROWS (the family save the exit banks — the trio
    // the array order the sprint row's AFTER appends, the gameplay
    // trio the drop row's AFTER the ctor's own registrations order
    // wears, the rebind the row's own value wears)
    app.open(ScreenKind::Controls);
    let text = std::fs::read_to_string(&path).expect("the exit save wrote");
    let tail = text
        .find("key_key.sprint:")
        .map(|i| &text[i..])
        .unwrap_or_default();
    assert!(
        tail.starts_with(
            "key_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.l\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\n"
        ),
        "the trio the array order the file rows wear ({tail})"
    );
    // THE RESTART'S OWN DECODE (the fresh boot the trio's truth wears)
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.key_binds[crate::options_file::key_binds::INVENTORY],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::L
        )),
        "the restart the rebind the decode keeps"
    );
    assert_eq!(
        re.key_binds[crate::options_file::key_binds::SWAP_OFFHAND],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::F
        )),
        "the restart the default the decode keeps"
    );
    assert_eq!(
        re.key_binds[crate::options_file::key_binds::DROP],
        Some(crate::options_file::key_binds::Bind::Keyboard(
            minifb::Key::Q
        )),
        "the restart the default the decode keeps"
    );
    assert_eq!(
        re.key_binds[crate::options_file::key_binds::USE],
        Some(crate::options_file::key_binds::Bind::Mouse(
            minifb::MouseButton::Right
        )),
        "the restart the mouse default the decode keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE MOUSE SETTINGS' OWN ROUND (the autopush-227 walk): the
/// MouseSettingsScreen.options the 6-instance array the bytecode
/// banks (the sensitivity the index-0 seat, the
/// mouseWheelSensitivity the index-1 seat the IntRange(-200, 100)
/// xmap(logMouse, unlogMouse) the log-scale slider wears, the
/// discrete/invertX/invertY/allowCursor the createBoolean rows) —
/// the addOptions walk's OWN TAIL the rawInput row the supported arm
/// appends, the addSmall grid the two-per-row pairs the array order
/// keeps, the wheel caption the `options.multiplier` "%sx" law the
/// lambda$new$49 speaks, the invert rows the look lane's own axis
/// flips the MouseHandler law serves, the wheel the log-scale edges
/// the fromSlider round answers (the left edge the 0.01x, the right
/// the 10.0x), the file the six rows the visit-order keys wear.
#[test]
fn the_mouse_settings_the_rows_the_wheel_the_invert_the_file_wear() {
    let dir = std::env::temp_dir().join(format!("mc2rs_mouse_rows_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE DEFAULTS' OWN SEATS (the ctor's own ints the six
    // registrations bank — the wheel the 1.0, the booleans the
    // discrete/invert trio the FALSE, the allowCursor/rawInput pair
    // the TRUE)
    assert_eq!(app.mouse_wheel_sensitivity, 1.0, "the wheel the 1.0 wears");
    assert!(!app.discrete_mouse_scroll, "the discrete the FALSE keeps");
    assert!(!app.invert_mouse_x, "the invertX the FALSE keeps");
    assert!(!app.invert_mouse_y, "the invertY the FALSE keeps");
    assert!(app.allow_cursor_changes, "the allowCursor the TRUE keeps");
    assert!(app.raw_mouse_input, "the rawInput the TRUE keeps");
    // THE SCREEN'S OWN RENDER (the direct open the 7-row grid wears —
    // the 6-instance array + the rawInput tail the supported arm
    // appends)
    app.open(ScreenKind::MouseSettings);
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 260);
    app.render(&mut fb, &mut assets);
    for id in [
        "opt_ms_sensitivity",
        "opt_ms_wheel",
        "opt_ms_discrete",
        "opt_ms_invertx",
        "opt_ms_inverty",
        "opt_ms_allowcursor",
        "opt_ms_rawinput",
        "msdone",
    ] {
        assert!(
            app.buttons.iter().any(|b| b.id == id),
            "the {id} row the grid wears"
        );
    }
    // THE WHEEL'S OWN CAPTION + UNIT (the `options.multiplier` "%sx"
    // law the default's 1.0 word speaks — the slider's own unit the
    // unlogMouse fold answers: the 1.0 the (0 + 200)/300 = 2/3 seat)
    let wheel = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_ms_wheel")
        .expect("the wheel row")
        .clone();
    assert_eq!(
        wheel.label, "Scroll Sensitivity: 1.0x",
        "the default's own multiplier word"
    );
    let unit = wheel.value.expect("the wheel the slider wears");
    assert!(
        (unit - 2.0 / 3.0).abs() < 1e-9,
        "the default the center-log seat wears ({unit})"
    );
    // THE LOG-SCALE'S OWN EDGES (the fromSlider round the xmap's
    // logMouse fold answers — the left edge the -200 int the 0.01
    // wears, the right the +100 int the 10.0 speaks)
    app.on_press(wheel.x + 4, wheel.y + wheel.h / 2);
    assert_eq!(
        app.mouse_wheel_sensitivity, 0.01,
        "the left edge the logMouse(-200) wears"
    );
    assert!(app.wheel_drag, "the press inside the track arms");
    app.on_press(wheel.x + wheel.w - 4, wheel.y + wheel.h / 2);
    assert_eq!(
        app.mouse_wheel_sensitivity, 10.0,
        "the right edge the logMouse(100) wears"
    );
    // THE DRAG'S OWN LAW (the same armed lane the sensitivity seat
    // wears — the held frames the value's own seat follows, the
    // release the disarm speaks, the disarmed drag never moves)
    app.on_wheel_slider_drag(wheel.x + 4);
    assert_eq!(
        app.mouse_wheel_sensitivity, 0.01,
        "the left-edge drag the 0.01 wears"
    );
    app.on_left_release();
    assert!(!app.wheel_drag, "the release disarms");
    app.on_wheel_slider_drag(wheel.x + wheel.w - 4);
    assert_eq!(
        app.mouse_wheel_sensitivity, 0.01,
        "the disarmed drag never moves"
    );
    // THE INVERT'S OWN FLIP (the boolean row's own click walk — the
    // press the store's own seat flips, the row's caption the
    // re-render's own read speaks)
    let inv = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_ms_invertx")
        .expect("the invertX row")
        .clone();
    assert_eq!(
        inv.label, "Invert Mouse X: OFF",
        "the default's own OFF word"
    );
    app.on_click(inv.x + inv.w / 2, inv.y + inv.h / 2);
    assert!(app.invert_mouse_x, "the click the store flips");
    app.render(&mut fb, &mut assets);
    let inv = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_ms_invertx")
        .expect("the row")
        .clone();
    assert_eq!(
        inv.label, "Invert Mouse X: ON",
        "the ON word the flip wears"
    );
    // THE LOOK LANE'S OWN FLIP (the MouseHandler's own walk — the
    // raw delta the positive yaw wears, the TRUE the axis the
    // negation speaks: the SAME drag the negative answers. The row's
    // flip the walk RESETS — the raw lane the store's own FALSE seat
    // probes, the re-arm the negation answers)
    app.open(ScreenKind::World);
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    app.invert_mouse_x = false;
    app.on_look_delta(10.0, 0.0);
    let yaw_right = app.body.as_ref().expect("the body").yaw;
    assert!(yaw_right > 0.0, "the raw delta the positive yaw wears");
    app.invert_mouse_x = true;
    app.on_look_delta(10.0, 0.0);
    let yaw_flipped = app.body.as_ref().expect("the body").yaw;
    assert!(
        yaw_flipped < yaw_right,
        "the invert the axis the negation speaks"
    );
    // THE FILE'S OWN ROWS (the save walk the six rows the visit-order
    // keys wear — the wheel the log-scale double, the booleans the
    // "true"/"false" vocabulary speaks, the restart the decode keeps)
    app.mouse_wheel_sensitivity = 0.5;
    app.discrete_mouse_scroll = true;
    app.invert_mouse_y = true;
    app.options_file_save();
    let text = std::fs::read_to_string(&path).expect("the save wrote");
    assert!(
        text.contains("mouseWheelSensitivity:0.5\n"),
        "the wheel the log-scale double wears ({text})"
    );
    assert!(
        text.contains("discrete_mouse_scroll:true\n"),
        "the discrete the TRUE row wears"
    );
    assert!(
        text.contains("invertXMouse:true\n"),
        "the invertX the TRUE row wears"
    );
    assert!(
        text.contains("invertYMouse:true\n"),
        "the invertY the TRUE row wears"
    );
    assert!(
        text.contains("allowCursorChanges:true\n"),
        "the allowCursor the default row wears"
    );
    assert!(
        text.contains("rawMouseInput:true\n"),
        "the rawInput the default row wears"
    );
    // THE RESTART'S OWN DECODE (the fresh boot the rows' truth wears —
    // the out-of-range the default keeps, the clamp never wears)
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.mouse_wheel_sensitivity, 0.5,
        "the restart the wheel decodes"
    );
    assert!(re.discrete_mouse_scroll, "the restart the discrete keeps");
    assert!(re.invert_mouse_x, "the restart the invertX keeps");
    assert!(re.invert_mouse_y, "the restart the invertY keeps");
    assert!(re.allow_cursor_changes, "the restart the allowCursor keeps");
    assert!(re.raw_mouse_input, "the restart the rawInput keeps");
    // THE OUT-OF-RANGE'S OWN ROW (the codec doubleRange(0.01, 10.0)
    // the decode's own bounds keep — the bogus row the default wears)
    let mut text2 = std::fs::read_to_string(&path).expect("the read");
    text2 = text2.replace("mouseWheelSensitivity:0.5", "mouseWheelSensitivity:99.0");
    std::fs::write(&path, &text2).expect("the bogus write");
    let mut re2 = App::new(262);
    re2.options_file = Some(path.clone());
    re2.options_file_load();
    assert_eq!(
        re2.mouse_wheel_sensitivity, 1.0,
        "the out-of-range the default keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE WHEEL'S OWN FOLD (javap `MouseHandler.onScroll` pcs 67-103 —
/// the 262 dump banks: the discrete flag the signum the raw delta
/// answers, the sensitivity the `dmul` the BOTH branches multiplies —
/// the ternary the operand selects, the multiply the selection
/// follows): the stores the 227 round wore the fold's own operands
/// become, the folded pair the whole dispatch rides, the hotbar's
/// own accumulator the folded doubles the whole steps emit (the
/// notch the ONE slot the signum law speaks, the sub-notch the
/// remainder the crossing's next notch answers).
#[test]
fn the_wheel_fold_the_sensitivity_the_discrete_the_lanes_wear() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE DEFAULT the identity keeps (the sensitivity 1.0 the raw
    // double the fold never bends, the discrete false the signum
    // never rides)
    let (fx, fy) = app.fold_scroll(0.3, -1.0);
    assert!((fx - 0.3).abs() < 1e-9, "the X the identity keeps: {fx}");
    assert!((fy + 1.0).abs() < 1e-9, "the Y the identity keeps: {fy}");
    // THE SENSITIVITY the dmul serves (the wheel slider's own
    // `10^(i/100)` double the 227 round stores — the 2.0 the raw
    // BOTH axes doubles)
    app.mouse_wheel_sensitivity = 2.0;
    let (fx, fy) = app.fold_scroll(0.3, -1.0);
    assert!((fx - 0.6).abs() < 1e-9, "the X the scale wears: {fx}");
    assert!((fy + 2.0).abs() < 1e-9, "the Y the scale wears: {fy}");
    // THE DISCRETE the signum answers (the sub-notch trackpad delta
    // the whole sign the fold speaks — the signum the sensitivity's
    // own dmul carries)
    app.discrete_mouse_scroll = true;
    let (fx, fy) = app.fold_scroll(0.3, -1.0);
    assert!((fx - 2.0).abs() < 1e-9, "the X the signum wears: {fx}");
    assert!((fy + 2.0).abs() < 1e-9, "the Y the signum wears: {fy}");
    // THE NOTCH the ONE slot the hotbar cycle speaks (the fold the
    // defaults restored — the folded -1.0 the whole step the d2i
    // emits, the selection the signum moves, the store untouched)
    app.discrete_mouse_scroll = false;
    app.mouse_wheel_sensitivity = 1.0;
    app.inventory.items[1] = (62, 5);
    app.hotbar_slots = app.inventory.hotbar(); // the staging's own sync
    app.on_hotbar_scroll(0.0, -1.0);
    assert_eq!(app.selected_hotbar, 1, "the notch the next slot");
    assert_eq!(app.held_id(), Some(62), "the held model follows");
    // THE SUB-NOTCH the remainder the accumulator keeps (the 0.6 the
    // crossing's next notch the 1.2 the ONE step wears — the
    // direction flip the stale remainder discards, the up-scroll the
    // back slot answers)
    app.on_hotbar_scroll(0.0, 0.6);
    assert_eq!(app.selected_hotbar, 1, "the sub-notch stays silent");
    assert!(
        (app.scroll_acc_y - 0.6).abs() < 1e-9,
        "the remainder held: {}",
        app.scroll_acc_y
    );
    app.on_hotbar_scroll(0.0, 0.6);
    assert_eq!(app.selected_hotbar, 0, "the crossing the back slot");
}

/// THE GAMEPLAY TRIO'S OWN LAWS (the autopush-188 walk — the
/// use/attack/pickItem MOUSE binds the ctor's registrations the store
/// keeps, the mouse capture the mouseClicked gate the armed rebind
/// answers, the display the seeded `key.mouse.*` rows speak, and the
/// decode the `mouse.` prefix walk the computed rows answer).
#[test]
fn the_gameplay_trio_the_mouse_binds_the_capture_the_lanes_wear() {
    use crate::options_file::key_binds as kb;
    let dir = std::env::temp_dir().join(format!("mc2rs_gameplay_trio_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE MOUSE DEFAULTS' OWN SEATS (the ctor's registrations the
    // store keeps — the ToggleKeyMapping pair the use/attack rows
    // wear, the plain KeyMapping the pickItem row speaks)
    assert_eq!(
        app.key_binds[kb::USE],
        Some(kb::Bind::Mouse(minifb::MouseButton::Right)),
        "the use the RIGHT default wears"
    );
    assert_eq!(
        app.key_binds[kb::ATTACK],
        Some(kb::Bind::Mouse(minifb::MouseButton::Left)),
        "the attack the LEFT default wears"
    );
    assert_eq!(
        app.key_binds[kb::PICK_ITEM],
        Some(kb::Bind::Mouse(minifb::MouseButton::Middle)),
        "the pickItem the MIDDLE default wears"
    );
    // THE DISPLAY'S OWN ROWS (the seeded names the getDisplayName law
    // speaks — the lang rows the "Left Button" words wear)
    assert_eq!(
        key_bind_display(app.key_binds[kb::ATTACK]),
        "Left Button",
        "the attack the Left Button row speaks"
    );
    assert_eq!(
        key_bind_display(app.key_binds[kb::USE]),
        "Right Button",
        "the use the Right Button row speaks"
    );
    assert_eq!(
        key_bind_display(app.key_binds[kb::PICK_ITEM]),
        "Middle Button",
        "the pickItem the Middle Button row speaks"
    );
    // THE MOUSE CAPTURE'S OWN LANE (the mouseClicked walk pc 0-37 —
    // the armed rebind the ANY click the MOUSE bind answers FIRST:
    // the capture the TRUE returns, the selected the null clears,
    // the click the button hit-test never reaches)
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let ctl = app
        .buttons
        .iter()
        .find(|b| b.id == "controls")
        .expect("the controls opener")
        .clone();
    app.on_click(ctl.x + ctl.w / 2, ctl.y + ctl.h / 2);
    app.render_title_buttons_for_test();
    let kbbtn = app
        .buttons
        .iter()
        .find(|b| b.id == "keybinds")
        .expect("the pair's right seat")
        .clone();
    app.on_click(kbbtn.x + kbbtn.w / 2, kbbtn.y + kbbtn.h / 2);
    // THE SCROLL'S OWN REACH (the small band the gameplay rows the
    // footer refuses at the fresh seat — the three notches the attack
    // row the 174 seat the band restores)
    app.on_kb_scroll(-1.0, 240);
    app.on_kb_scroll(-1.0, 240);
    app.on_kb_scroll(-1.0, 240);
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 240);
    app.render(&mut fb, &mut assets);
    let attack_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_attack")
        .expect("the attack row the gameplay group renders")
        .clone();
    app.on_click(
        attack_row.x + attack_row.w / 2,
        attack_row.y + attack_row.h / 2,
    );
    assert_eq!(
        app.kb_selected,
        Some(kb::ATTACK),
        "the change the click the selectedKey arms"
    );
    // THE GATE'S OWN CONSUME (the SECOND click the capture serves —
    // the selectedKey the gate answers FIRST, the change buttons the
    // walk's return never re-arms)
    app.render(&mut fb, &mut assets);
    let any = app.buttons[0].clone();
    app.on_click(any.x + any.w / 2, any.y + any.h / 2);
    assert_eq!(
        app.key_binds[kb::ATTACK],
        Some(kb::Bind::Mouse(minifb::MouseButton::Left)),
        "the left click the LEFT bind wears"
    );
    assert_eq!(app.kb_selected, None, "the capture the seat clears");
    // THE RIGHT SEAT'S OWN CAPTURE (the use row the armed seat the
    // right press the MOUSE 1 bind wears — the re-seat the Use row
    // the group's own tail the fourth notch the band restores, the
    // capture the FALSE verdict the use-start probe keeps, the press
    // consumed)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let use_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_use")
        .expect("the use row")
        .clone();
    app.on_click(use_row.x + use_row.w / 2, use_row.y + use_row.h / 2);
    assert_eq!(app.kb_selected, Some(kb::USE), "the use the seat arms");
    assert!(
        !app.on_right_press(use_row.x + 1, use_row.y + 1),
        "the right press the capture consumed"
    );
    assert_eq!(
        app.key_binds[kb::USE],
        Some(kb::Bind::Mouse(minifb::MouseButton::Right)),
        "the right press the RIGHT bind wears"
    );
    assert_eq!(app.kb_selected, None, "the capture the seat clears");
    // THE FILE'S OWN ROWS (the family exit the save wears — the
    // gameplay trio the drop row's AFTER the ctor's registrations
    // order keeps, the seeded names the save rows wear)
    app.open(ScreenKind::Controls);
    let text = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        text.contains(
            "key_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\n"
        ),
        "the gameplay trio the array order the file rows wear"
    );
    // THE DECODE'S OWN LAWS (the NAME_MAP the seeded rows answer, the
    // `mouse.` prefix walk the minus-one the computed rows keep, the
    // minifb boundary the unknown the None wears)
    assert_eq!(
        kb::bind_from_save("key.mouse.left"),
        Some(kb::Bind::Mouse(minifb::MouseButton::Left)),
        "the NAME_MAP hit the decode keeps"
    );
    assert_eq!(
        kb::bind_from_save("mouse.2"),
        Some(kb::Bind::Mouse(minifb::MouseButton::Right)),
        "the prefix walk the minus-one law keeps"
    );
    assert_eq!(
        kb::bind_from_save("mouse.9"),
        None,
        "the computed button the minifb boundary never carries"
    );
    assert_eq!(
        kb::bind_from_save("garbage"),
        None,
        "the miss the None keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE GAMEPLAY LANES' OWN GATES (the in-world walks the store's own
/// binds serve — the physical left press the DEFAULT wiring the
/// attack arms with, the keyboard rebind the store the gate refuses,
/// the release the hold's own tail clears).
#[test]
fn the_gameplay_lanes_the_store_gates_the_attack_wear() {
    use crate::options_file::key_binds as kb;
    // THE REBOUND ATTACK'S OWN REFUSAL (the physical left press the
    // keyboard-rebound attack never serves — the on_press fallthrough
    // the gate speaks, the dig the press never arms)
    let mut app = App::new(3);
    app.enter_world();
    app.key_binds[kb::ATTACK] = Some(kb::Bind::Keyboard(minifb::Key::R));
    app.on_press(0, 0);
    assert!(
        !app.dig_held,
        "the rebound attack the physical left press never arms"
    );
    assert!(
        !app.dig.is_destroying,
        "the rebound attack the dig the press never starts"
    );
    // THE DEFAULT WIRING'S OWN LANE (the MOUSE left bind the store
    // keeps — the same press the attack arms with, the dig the hold
    // speaks)
    app.key_binds[kb::ATTACK] = Some(kb::Bind::Mouse(minifb::MouseButton::Left));
    app.on_press(0, 0);
    assert!(
        app.dig_held,
        "the default wiring the physical left press the attack serves"
    );
    // THE RELEASE'S OWN GATE (the same bind law the hold's clear
    // wears — the physical left release the dig abort speaks)
    app.on_left_release();
    assert!(!app.dig_held, "the release the hold the walk clears");
    assert!(
        !app.dig.is_destroying,
        "the release the abort the tail keeps"
    );
}

/// THE HOTBAR NINE'S OWN LAWS (the dev-159 walk — the key.hotbar.1..9
/// bindings the ctor's `keyHotbarSlots` array the store keeps (the
/// codes 49..57 the GLFW keys answer, the INVENTORY category the all
/// nine wear), the list the same group's rows the stride keeps, the
/// file the nine rows the ladder appends, and the decode the restart
/// the truth wears).
#[test]
fn the_hotbar_nine_the_rows_the_group_the_file_wear() {
    use crate::options_file::key_binds as kb;
    let dir = std::env::temp_dir().join(format!("mc2rs_hotbar_nine_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE NINE'S OWN DEFAULTS (the ctor's codes the store keeps — the
    // GLFW 1..9 the minifb enum's own discriminants answer)
    assert_eq!(
        app.key_binds[kb::HOTBAR_1],
        Some(kb::Bind::Keyboard(minifb::Key::Key1)),
        "the first slot the 1 key wears"
    );
    assert_eq!(
        app.key_binds[kb::HOTBAR_9],
        Some(kb::Bind::Keyboard(minifb::Key::Key9)),
        "the ninth slot the 9 key wears"
    );
    // THE DISPLAY'S OWN ROW (the lang rows the captions speak)
    assert_eq!(
        key_bind_display(app.key_binds[kb::HOTBAR_5]),
        "5",
        "the fifth slot the digit row speaks"
    );
    // THE LIST'S OWN SEAT (the inventory group the hotbar rows the
    // SAME header keeps — the trio's bottom the next stride slot the
    // rows take: the hotbar.1 row the 378 y the walk answers, the
    // CREATIVE pair the 594-header the 606-start rows wear, the
    // SPECTATOR pair the 654-header the 666-start rows the walk's
    // own bottom keeps)
    assert_eq!(
        kb::content_height(),
        1220,
        "the content the walk's own bottom keeps"
    );
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let ctl = app
        .buttons
        .iter()
        .find(|b| b.id == "controls")
        .expect("the controls opener")
        .clone();
    app.on_click(ctl.x + ctl.w / 2, ctl.y + ctl.h / 2);
    app.render_title_buttons_for_test();
    let kbbtn = app
        .buttons
        .iter()
        .find(|b| b.id == "keybinds")
        .expect("the pair's right seat")
        .clone();
    app.on_click(kbbtn.x + kbbtn.w / 2, kbbtn.y + kbbtn.h / 2);
    // THE SMALL BAND'S OWN CLIP (the fresh seat the hotbar rows the
    // footer refuses — the reach the scroll alone serves)
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 240);
    app.render(&mut fb, &mut assets);
    assert!(
        !app.buttons.iter().any(|b| b.id == "kb_change_hotbar1"),
        "the clipped hotbar rows the band render refuses"
    );
    // THE SCROLL'S OWN REACH (the max the 1038 clamp keeps — the
    // hotbar.1/2/9 rows the max scroll the clip refuses: the nine
    // the 570 y the -420 seat the list-top 30 the band never seats,
    // the debug twenty-two-row's own reach the nine's rows the walk
    // refuses — the lightmapTexture row the END the content 1220
    // the walk banks)
    app.on_kb_scroll(-1000.0, 240);
    assert_eq!(app.kb_scroll, 1038.0, "the clamp the content 1220 keeps");
    app.render(&mut fb, &mut assets);
    assert!(
        !app.buttons.iter().any(|b| b.id == "kb_change_hotbar1"),
        "the hotbar.1 row the max scroll the clip refuses"
    );
    assert!(
        !app.buttons.iter().any(|b| b.id == "kb_change_hotbar2"),
        "the hotbar.2 row the max scroll the clip refuses"
    );
    assert!(
        !app.buttons.iter().any(|b| b.id == "kb_change_hotbar9"),
        "the hotbar.9 row the max scroll the list-top clip refuses"
    );
    // THE REBIND'S OWN REACH (the TWENTY-ONE notches up the 534 scroll
    // the nine's own seat restores — the 570 y the 36 seat the band
    // 30..212 the walk seats, the max 1038 the extra notch the walk
    // wears)
    app.on_kb_scroll(21.0, 240);
    assert_eq!(
        app.kb_scroll, 534.0,
        "the twenty-one notches up the walk wears"
    );
    app.render(&mut fb, &mut assets);
    let h9 = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_hotbar9")
        .expect("the scrolled hotbar row the band restores")
        .clone();
    assert_eq!(h9.label, "9", "the default the display speaks");
    assert_eq!(h9.y, 36, "the scrolled row the seat wears");
    assert!(
        app.buttons
            .iter()
            .any(|b| b.id == "kb_rowreset_hotbar9" && !b.active),
        "the default row the reset the INACTIVE seat wears"
    );
    // THE REBIND'S OWN WALK (the change the click the selectedKey arms,
    // the capture the bind wears — the lanes the store honor)
    app.on_click(h9.x + h9.w / 2, h9.y + h9.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::HOTBAR_9),
        "the change the seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::L)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::HOTBAR_9],
        Some(kb::Bind::Keyboard(minifb::Key::L)),
        "the rebind the store writes"
    );
    // THE FILE'S OWN ROWS (the family save the exit banks — the nine
    // the pickItem row's AFTER the ladder appends, the rebind the row's
    // own value wears)
    app.open(ScreenKind::Controls);
    let text = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        text.contains(
            "key_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\n"
        ),
        "the hotbar rows the array order the file wears"
    );
    assert!(
        text.trim_end()
            .ends_with("key_key.debug.lightmapTexture:key.keyboard.4"),
        "the lightmapTexture row the file's own END wears"
    );
    // THE RESTART'S OWN DECODE (the fresh boot the nine's truth wears)
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.key_binds[kb::HOTBAR_9],
        Some(kb::Bind::Keyboard(minifb::Key::L)),
        "the restart the rebind the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::HOTBAR_5],
        Some(kb::Bind::Keyboard(minifb::Key::Key5)),
        "the restart the default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::SAVE_ACTIVATOR],
        Some(kb::Bind::Keyboard(minifb::Key::C)),
        "the restart the pair's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::LOAD_ACTIVATOR],
        Some(kb::Bind::Keyboard(minifb::Key::X)),
        "the restart the pair's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::SPECTATOR_OUTLINES],
        None,
        "the restart the unbound default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::SPECTATOR_HOTBAR],
        Some(kb::Bind::Mouse(minifb::MouseButton::Middle)),
        "the restart the middle default the decode keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE ACTIVATOR PAIR'S OWN LAWS (the autopush-189 walk — the
/// save/load hotbar activators the ctor's putfields the hotbar
/// array's AFTER the store keeps, the CREATIVE category the pair's
/// own group the sorted walk answers, the chords the store's own
/// binds read — the rebind the C/X law honors, the file the pair
/// rows the END appends, the restart the pair decodes).
#[test]
fn the_activator_pair_the_creative_group_the_chords_the_store_wear() {
    use crate::options_file::key_binds as kb;
    let dir = std::env::temp_dir().join(format!("mc2rs_activator_pair_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE PAIR'S OWN DEFAULTS (the ctor's `bipush 67` the GLFW C
    // answers, the `bipush 88` the GLFW X — the CREATIVE category
    // the pair's rows wear)
    assert_eq!(
        app.key_binds[kb::SAVE_ACTIVATOR],
        Some(kb::Bind::Keyboard(minifb::Key::C)),
        "the save chord the C default wears"
    );
    assert_eq!(
        app.key_binds[kb::LOAD_ACTIVATOR],
        Some(kb::Bind::Keyboard(minifb::Key::X)),
        "the load chord the X default wears"
    );
    // THE CREATIVE GROUP'S OWN SEATS (the sorted walk's own y — the
    // hotbar nine the 378..570 stride wear, the header the 594 seat
    // the nine's own bottom the next stride slot answers, the pair
    // rows the 606 start the +12 the groups' own gap keeps)
    assert_eq!(
        kb::CREATIVE_HEADER_Y,
        594,
        "the header the nine's bottom wears"
    );
    assert_eq!(kb::CREATIVE_ROWS_START, 606, "the rows the +12 gap keeps");
    assert_eq!(
        kb::content_height(),
        1220,
        "the content the fpsCharts row's own bottom keeps"
    );
    // THE FRESH LIST'S OWN WALK (the opener the zero scroll keeps)
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let ctl = app
        .buttons
        .iter()
        .find(|b| b.id == "controls")
        .expect("the controls opener")
        .clone();
    app.on_click(ctl.x + ctl.w / 2, ctl.y + ctl.h / 2);
    app.render_title_buttons_for_test();
    let kbbtn = app
        .buttons
        .iter()
        .find(|b| b.id == "keybinds")
        .expect("the pair's right seat")
        .clone();
    app.on_click(kbbtn.x + kbbtn.w / 2, kbbtn.y + kbbtn.h / 2);
    assert_eq!(app.screen, ScreenKind::KeyBinds, "the list the seat opens");
    // THE SMALL BAND'S OWN CLIP (the h=240 the creative group the
    // 594/606 seats the footer band never reaches — the fresh render
    // the pair's rows the band refuses)
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 240);
    app.render(&mut fb, &mut assets);
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_saveactivator"),
        "the clipped creative rows the band render refuses"
    );
    // THE SCROLL'S OWN REACH (the max the 1038 clamp keeps — the
    // creative header the -396 seat the shift wears, the
    // save/load pair the -384/-360 seats the list-top the clip
    // refuses; the SPECTATOR pair the -324/-300 seats the list-top
    // clip refuses)
    app.on_kb_scroll(-1000.0, 240);
    assert_eq!(app.kb_scroll, 1038.0, "the clamp the content 1220 keeps");
    app.render(&mut fb, &mut assets);
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_saveactivator"),
        "the save row the max scroll the list-top clip refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_loadactivator"),
        "the load row the max scroll the list-top clip refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_spectatoroutlines"),
        "the outlines row the max scroll the list-top clip refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_spectatorhotbar"),
        "the hotbar row the max scroll the list-top clip refuses"
    );
    // THE TWENTY NOTCHES' OWN RETURN (the wheel up the 558 scroll the
    // load row the 48 seat the band restores — the re-seat the pair's
    // own head the alphabetical law wears, the save row the 72 seat
    // the pair keeps, the max 1038 the extra notch the walk wears)
    app.on_kb_scroll(20.0, 240);
    assert_eq!(app.kb_scroll, 558.0, "the twenty notches the walk wears");
    app.render(&mut fb, &mut assets);
    let save_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_saveactivator")
        .expect("the notch-up save row the band restores")
        .clone();
    assert_eq!(save_row.label, "C", "the default the display speaks");
    assert_eq!(save_row.y, 72, "the save row the notch seat wears");
    let load_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_loadactivator")
        .expect("the notch-up load row the band restores")
        .clone();
    assert_eq!(load_row.label, "X", "the default the display speaks");
    assert_eq!(load_row.y, 48, "the load row the notch seat wears");
    assert!(
        load_row.y + load_row.h <= 240 - 28,
        "the load row the notch walk the fully-visible seat wears"
    );
    // THE REBIND'S OWN WALK (the change the click the selectedKey
    // arms, the capture the bind wears — the chords the store's own
    // binds read, the rebind the C/X law honors)
    app.on_click(save_row.x + save_row.w / 2, save_row.y + save_row.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::SAVE_ACTIVATOR),
        "the change the seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::K)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::SAVE_ACTIVATOR],
        Some(kb::Bind::Keyboard(minifb::Key::K)),
        "the rebind the store writes"
    );
    // THE ROW-RESET'S OWN SEAT (the reset the press the default
    // writes — the pair's own C the DEFAULTS arm answers)
    app.render(&mut fb, &mut assets);
    let reset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_saveactivator")
        .expect("the reset the row's pair keeps")
        .clone();
    app.on_click(reset.x + reset.w / 2, reset.y + reset.h / 2);
    assert_eq!(
        app.key_binds[kb::SAVE_ACTIVATOR],
        Some(kb::Bind::Keyboard(minifb::Key::C)),
        "the reset the C default restores"
    );
    // THE FILE'S OWN ROWS (the family save the exit banks — the pair
    // the hotbar.9 row's AFTER the ladder appends, the rebind the
    // row's own value wears)
    app.key_binds[kb::SAVE_ACTIVATOR] = Some(kb::Bind::Keyboard(minifb::Key::K));
    app.open(ScreenKind::Controls);
    let text = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        text.contains(
            "key_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.k\nkey_key.loadToolbarActivator:key.keyboard.x\n"
        ),
        "the pair rows the array order the file wears"
    );
    assert!(
        text.trim_end()
            .ends_with("key_key.debug.lightmapTexture:key.keyboard.4"),
        "the lightmapTexture row the file's own END wears"
    );
    // THE RESTART'S OWN DECODE (the fresh boot the pair's truth wears)
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.key_binds[kb::SAVE_ACTIVATOR],
        Some(kb::Bind::Keyboard(minifb::Key::K)),
        "the restart the rebind the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::LOAD_ACTIVATOR],
        Some(kb::Bind::Keyboard(minifb::Key::X)),
        "the restart the default the decode keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE SPECTATOR PAIR'S OWN LAWS (the autopush-190 walk — the
/// keySpectatorOutlines/keySpectatorHotbar the ctor's putfields the
/// activator pair's AFTER the store keeps, the SPECTATOR category
/// the pair's own group the sorted walk answers: the outlines the
/// `InputConstants.UNKNOWN` the UNBOUND default wears, the hotbar
/// the `Type.MOUSE, iconst_2` the MIDDLE button the seeded name
/// speaks — the file the pair rows the END appends, the restart the
/// pair decodes).
#[test]
fn the_spectator_pair_the_group_the_unbound_the_file_wear() {
    use crate::options_file::key_binds as kb;
    let dir = std::env::temp_dir().join(format!("mc2rs_spectator_pair_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE PAIR'S OWN DEFAULTS (the ctor's pc 3193 the UNKNOWN the
    // unbound law keeps — the None the `key.keyboard.unknown` row
    // wears, the pc 3216 the `Type.MOUSE, iconst_2` the MIDDLE the
    // seeded name answers)
    assert_eq!(
        app.key_binds[kb::SPECTATOR_OUTLINES],
        None,
        "the outlines the unbound default wears"
    );
    assert_eq!(
        app.key_binds[kb::SPECTATOR_HOTBAR],
        Some(kb::Bind::Mouse(minifb::MouseButton::Middle)),
        "the hotbar the middle default wears"
    );
    // THE DISPLAY'S OWN ROWS (the seeded names the getDisplayName law
    // speaks — the UNKNOWN the Not Bound row wears, the middle the
    // lang rows answer)
    assert_eq!(
        key_bind_display(app.key_binds[kb::SPECTATOR_OUTLINES]),
        "Not Bound",
        "the unbound the Not Bound row speaks"
    );
    assert_eq!(
        key_bind_display(app.key_binds[kb::SPECTATOR_HOTBAR]),
        "Middle Button",
        "the middle the seeded name speaks"
    );
    // THE SPECTATOR GROUP'S OWN SEATS (the sorted walk's own y — the
    // creative pair the 606-start stride wear, the header the 654
    // seat the pair's own bottom the next stride slot answers, the
    // rows the 666 start the +12 the groups' own gap keeps, the
    // content the walk's own bottom speaks)
    assert_eq!(
        kb::SPECTATOR_HEADER_Y,
        654,
        "the header the creative pair's bottom wears"
    );
    assert_eq!(kb::SPECTATOR_ROWS_START, 666, "the rows the +12 gap keeps");
    assert_eq!(
        kb::content_height(),
        1220,
        "the content the fpsCharts row's own bottom keeps"
    );
    // THE FRESH LIST'S OWN WALK (the opener the zero scroll keeps)
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let ctl = app
        .buttons
        .iter()
        .find(|b| b.id == "controls")
        .expect("the controls opener")
        .clone();
    app.on_click(ctl.x + ctl.w / 2, ctl.y + ctl.h / 2);
    app.render_title_buttons_for_test();
    let kbbtn = app
        .buttons
        .iter()
        .find(|b| b.id == "keybinds")
        .expect("the pair's right seat")
        .clone();
    app.on_click(kbbtn.x + kbbtn.w / 2, kbbtn.y + kbbtn.h / 2);
    assert_eq!(app.screen, ScreenKind::KeyBinds, "the list the seat opens");
    // THE SMALL BAND'S OWN CLIP (the h=240 the spectator group the
    // 654/666 seats the footer band never reaches — the fresh render
    // the pair's rows the band refuses)
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 240);
    app.render(&mut fb, &mut assets);
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_spectatoroutlines"),
        "the clipped spectator rows the band render refuses"
    );
    // THE SCROLL'S OWN REACH (the max the 1038 clamp keeps — the
    // outlines row the 666 y the -324 seat the max scroll wears, the
    // hotbar row the 690 y the -300 seat the list-top clip refuses,
    // the dumpDynamicTextures row the 1038 y the 0 seat the
    // list-top clip refuses, the reloadResourcePacks row the 1062
    // y the 24 seat the list-top clip refuses, the profiling row
    // the 1086 y the 48 seat the END-6 the walk keeps, the
    // copyLocation row the 1110 y the 72 seat the END-5 the walk
    // keeps, the dumpVersion row the 1134 y the 96 seat the
    // END-4 wears, the profilingChart row the 1158 y the 120 seat
    // the END-3 wears, the fpsCharts row the 1182 y the 144 seat
    // the END-2 wears, the networkCharts row the 1206 y the 168 seat
    // the END-1 wears, the lightmapTexture row the 1230 y the 192
    // seat the fully-visible law keeps — the row's own bottom the
    // band's own bottom the walk aligns)
    app.on_kb_scroll(-1000.0, 240);
    assert_eq!(app.kb_scroll, 1038.0, "the clamp the content 1220 keeps");
    app.render(&mut fb, &mut assets);
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_spectatoroutlines"),
        "the outlines row the max scroll the list-top clip refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_spectatorhotbar"),
        "the hotbar row the max scroll the list-top clip refuses"
    );
    // THE REBIND'S OWN WALK (the SEVENTEEN notches up the outlines row
    // the band restores — the 630 scroll the 36 seat the rebind's own
    // walk needs, the hotbar row the 60 seat the pair keeps, the
    // change the click the selectedKey arms, the capture the bind
    // wears — the unbound row the store honors)
    app.on_kb_scroll(17.0, 240);
    app.render(&mut fb, &mut assets);
    let out_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_spectatoroutlines")
        .expect("the notch-up outlines row the band restores")
        .clone();
    assert_eq!(out_row.y, 36, "the outlines row the notch seat wears");
    let hot_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_spectatorhotbar")
        .expect("the notch-up hotbar row the band restores")
        .clone();
    assert_eq!(hot_row.y, 60, "the hotbar row the notch seat wears");
    assert!(
        hot_row.y + hot_row.h <= 240 - 28,
        "the hotbar row the notch walk the fully-visible seat wears"
    );
    app.on_click(out_row.x + out_row.w / 2, out_row.y + out_row.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::SPECTATOR_OUTLINES),
        "the change the seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::O)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::SPECTATOR_OUTLINES],
        Some(kb::Bind::Keyboard(minifb::Key::O)),
        "the rebind the store writes"
    );
    // THE ROW-RESET'S OWN SEAT (the reset the press the default
    // writes — the outlines the UNBOUND None the UNKNOWN law
    // restores, the reset row the INACTIVE seat the default wears)
    app.render(&mut fb, &mut assets);
    let reset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_spectatoroutlines")
        .expect("the reset the row's pair keeps")
        .clone();
    app.on_click(reset.x + reset.w / 2, reset.y + reset.h / 2);
    assert_eq!(
        app.key_binds[kb::SPECTATOR_OUTLINES],
        None,
        "the reset the unbound default restores"
    );
    app.render(&mut fb, &mut assets);
    assert!(
        app.buttons
            .iter()
            .find(|b| b.id == "kb_rowreset_spectatoroutlines")
            .map(|b| !b.active)
            .unwrap_or(false),
        "the default row the reset the INACTIVE seat wears"
    );
    // THE FILE'S OWN ROWS (the family save the exit banks — the pair
    // the load activator row's AFTER the ladder appends, the rebind
    // the row's own value wears, the UNKNOWN the unbound row speaks)
    app.key_binds[kb::SPECTATOR_OUTLINES] = Some(kb::Bind::Keyboard(minifb::Key::O));
    app.open(ScreenKind::Controls);
    let text = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        text.contains(
            "key_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.o\nkey_key.spectatorHotbar:key.mouse.middle\n"
        ),
        "the pair rows the array order the file wears"
    );
    assert!(
        text.trim_end()
            .ends_with("key_key.debug.lightmapTexture:key.keyboard.4"),
        "the lightmapTexture row the file's own END wears"
    );
    // THE RESTART'S OWN DECODE (the fresh boot the pair's truth wears
    // — the walk the UNKNOWN's own row the SAME file the overwrides
    // never races, the decode the exit save's own rows reads)
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.key_binds[kb::SPECTATOR_OUTLINES],
        Some(kb::Bind::Keyboard(minifb::Key::O)),
        "the restart the rebind the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::SPECTATOR_HOTBAR],
        Some(kb::Bind::Mouse(minifb::MouseButton::Middle)),
        "the restart the middle default the decode keeps"
    );
    // THE UNKNOWN'S OWN ROW (the reset None the save walk the
    // `key.keyboard.unknown` row wears — the bind_save the unbound
    // law keeps)
    let mut fresh = App::new(262);
    fresh.options_file = Some(path.clone());
    fresh.key_binds[kb::SPECTATOR_OUTLINES] = None;
    fresh.options_file_save();
    let text2 = std::fs::read_to_string(&path).expect("the fresh save wrote");
    assert!(
        text2.contains("key_key.spectatorOutlines:key.keyboard.unknown\n"),
        "the unbound row the UNKNOWN string wears"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE DEBUG OVERLAY'S OWN LAWS (the dev-160 walk — the keyDebugOverlay
/// the ctor's pc 3229-3251 putfields the spectator pair's AFTER the
/// store keeps: the `ldc_w cp#1407` the `key.debug.overlay` string
/// answers, the `KEYSYM, sipush 292` the F3 default wears, the
/// Category.DEBUG the group's own ordinal 7 the sorted walk answers,
/// the FIFTH ctor arg the `bipush -2` the `orderI` field the sort
/// walk banks — the 5-arg overload the modifier-aware shape the
/// modifier row's -1 the next walk wears —, the header the NEW
/// `toLanguageKey` format the label() law speaks, the file the row
/// the END appends, the restart the row decodes).
#[test]
fn the_debug_overlay_the_group_the_file_the_restart_wear() {
    use crate::options_file::key_binds as kb;
    let dir = std::env::temp_dir().join(format!("mc2rs_debug_overlay_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    // THE ROW'S OWN DEFAULT (the ctor's pc 3240 the `sipush 292` the
    // GLFW F3 answers — the KEYSYM type the keyboard bind wears)
    assert_eq!(
        app.key_binds[kb::DEBUG_OVERLAY],
        Some(kb::Bind::Keyboard(minifb::Key::F3)),
        "the overlay the F3 default wears"
    );
    // THE DISPLAY'S OWN ROW (the getDisplayName law — the F3 the lang
    // row's own word speaks)
    assert_eq!(
        key_bind_display(app.key_binds[kb::DEBUG_OVERLAY]),
        "F3",
        "the F3 the display row speaks"
    );
    // THE DEBUG GROUP'S OWN SEATS (the sorted walk's own y — the
    // spectator pair the 666-start stride wears, the DEBUG header
    // the pair's own bottom the next stride slot answers (690 + 24),
    // the row the +12 the groups' own gap keeps, the content the
    // row's own bottom speaks)
    assert_eq!(
        kb::DEBUG_HEADER_Y,
        714,
        "the header the spectator pair's bottom wears"
    );
    assert_eq!(kb::DEBUG_ROWS_START, 726, "the row the +12 gap keeps");
    assert_eq!(
        kb::content_height(),
        1220,
        "the content the fpsCharts row's own bottom keeps"
    );
    // THE FRESH LIST'S OWN WALK (the opener the zero scroll keeps)
    app.open(ScreenKind::Options);
    app.render_title_buttons_for_test();
    let ctl = app
        .buttons
        .iter()
        .find(|b| b.id == "controls")
        .expect("the controls opener")
        .clone();
    app.on_click(ctl.x + ctl.w / 2, ctl.y + ctl.h / 2);
    app.render_title_buttons_for_test();
    let kbbtn = app
        .buttons
        .iter()
        .find(|b| b.id == "keybinds")
        .expect("the pair's right seat")
        .clone();
    app.on_click(kbbtn.x + kbbtn.w / 2, kbbtn.y + kbbtn.h / 2);
    assert_eq!(app.screen, ScreenKind::KeyBinds, "the list the seat opens");
    // THE MODIFIER'S OWN DEFAULT + DISPLAY (the ctor's pc 3259 the
    // `ldc_w cp#1421` the `key.debug.modifier` answers, the pc 3265
    // the same `sipush 292` the F3 the pair shares, the `iconst_m1`
    // the orderI banks, the lang row the 'Debug Modifier Key' words
    // wear — the display the F3 row the overlay's own word speaks)
    assert_eq!(
        app.key_binds[kb::DEBUG_MODIFIER],
        Some(kb::Bind::Keyboard(minifb::Key::F3)),
        "the modifier the F3 default wears"
    );
    assert_eq!(
        key_bind_display(app.key_binds[kb::DEBUG_MODIFIER]),
        "F3",
        "the F3 the display row speaks"
    );
    // THE CRASH'S OWN DEFAULT + DISPLAY (the ctor's pc 3283 the
    // `ldc_w cp#1426` the `key.debug.crash` answers, the pc 3289 the
    // `bipush 67` the C the plain-shape default wears, the 4-arg
    // ctor the `iconst_0` the orderI default banks, the lang row the
    // 'Debug Crash' words wear)
    assert_eq!(
        app.key_binds[kb::DEBUG_CRASH],
        Some(kb::Bind::Keyboard(minifb::Key::C)),
        "the crash the C default wears"
    );
    assert_eq!(
        key_bind_display(app.key_binds[kb::DEBUG_CRASH]),
        "C",
        "the C the display row speaks"
    );
    // THE RELOAD'S OWN DEFAULT + DISPLAY (the ctor's pc 3305 the
    // `ldc_w cp#1431` the `key.debug.reloadChunk` answers, the
    // pc 3311 the `bipush 65` the A the plain-shape default wears,
    // the 4-arg ctor the `iconst_0` the orderI default banks, the
    // lang row the 'Reload Chunks' words wear)
    assert_eq!(
        app.key_binds[kb::DEBUG_RELOAD_CHUNK],
        Some(kb::Bind::Keyboard(minifb::Key::A)),
        "the reload the A default wears"
    );
    assert_eq!(
        key_bind_display(app.key_binds[kb::DEBUG_RELOAD_CHUNK]),
        "A",
        "the A the display row speaks"
    );
    // THE SMALL BAND'S OWN CLIP (the h=240 the debug group the
    // 714/726 seats the footer band never reaches — the fresh render
    // the pair's rows the band refuses)
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 240);
    app.render(&mut fb, &mut assets);
    assert!(
        !app.buttons.iter().any(|b| b.id == "kb_change_debugoverlay"),
        "the clipped debug row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugmodifier"),
        "the clipped modifier row the band render refuses"
    );
    assert!(
        !app.buttons.iter().any(|b| b.id == "kb_change_debugcrash"),
        "the clipped crash row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugreloadchunk"),
        "the clipped reload row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugshowhitboxes"),
        "the clipped hitboxes row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugclearchat"),
        "the clipped clearChat row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugchunkborders"),
        "the clipped chunkBorders row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugadvtooltips"),
        "the clipped advTooltips row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugcopyrecreate"),
        "the clipped copyRecreate row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugspectate"),
        "the clipped spectate row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugswitchgamemode"),
        "the clipped switchGameMode row the band render refuses"
    );
    assert!(
        !app.buttons.iter().any(|b| b.id == "kb_change_debugoptions"),
        "the clipped debugOptions row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugfocuspause"),
        "the clipped focusPause row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugdumptextures"),
        "the clipped dumpDynamicTextures row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugreloadpacks"),
        "the clipped reloadResourcePacks row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugprofiling"),
        "the clipped profiling row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugcopylocation"),
        "the clipped copyLocation row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugdumpversion"),
        "the clipped dumpVersion row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugprofilingchart"),
        "the clipped profilingChart row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugfpscharts"),
        "the clipped fpsCharts row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugnetworkcharts"),
        "the clipped networkCharts row the band render refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debuglightmaptexture"),
        "the clipped lightmapTexture row the band render refuses"
    );
    // THE SCROLL'S OWN REACH (the max the 1038 clamp keeps — the
    // lightmapTexture row the 1230 y the 192 seat the LAST row
    // the walk keeps, the row's own bottom the band's own bottom
    // aligns — the fully-visible law the END seat wears; the overlay
    // row the -312 seat the list-top clip refuses, the FIFTEEN
    // notches up the 678 scroll the seated walk's own head the band
    // restores)
    app.on_kb_scroll(-1000.0, 240);
    assert_eq!(app.kb_scroll, 1038.0, "the clamp the content 1220 keeps");
    app.render(&mut fb, &mut assets);
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugswitchgamemode"),
        "the switchGameMode row the max scroll the list-top clip refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugspectate"),
        "the spectate row the max scroll the list-top clip refuses"
    );
    assert!(
        !app.buttons.iter().any(|b| b.id == "kb_change_debugoverlay"),
        "the overlay row the max scroll the list-top clip refuses"
    );
    assert!(
        !app.buttons.iter().any(|b| b.id == "kb_change_debugoptions"),
        "the debugOptions row the max scroll the list-top clip refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugchunkborders"),
        "the chunkBorders row the max scroll the list-top clip refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugdumptextures"),
        "the dumpDynamicTextures row the max scroll the list-top clip refuses"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugreloadpacks"),
        "the reloadResourcePacks row the max scroll the list-top clip refuses"
    );
    // THE COMPARETO'S OWN MAX-SCROLL SEATS (the re-seat the band's
    // own bottom the seated order wears — the Show Hitboxes the
    // 1086 slot the 48 seat the alphabetical tail answers, the
    // profiling the 72, the focusPause the 96, the charts the
    // 120..192 the orderI seats speak)
    let hb_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugshowhitboxes")
        .expect("the scrolled hitboxes row the band restores")
        .clone();
    assert_eq!(
        hb_row.label, "B",
        "the hitboxes's own default the display speaks"
    );
    assert_eq!(hb_row.y, 48, "the hitboxes row the max-scroll seat wears");
    assert!(
        hb_row.y + hb_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let pf_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugprofiling")
        .expect("the scrolled profiling row the band restores")
        .clone();
    assert_eq!(
        pf_row.label, "L",
        "the profiling's own default the display speaks"
    );
    assert_eq!(pf_row.y, 72, "the profiling row the max-scroll seat wears");
    assert!(
        pf_row.y + pf_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let fp_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugfocuspause")
        .expect("the scrolled focusPause row the band restores")
        .clone();
    assert_eq!(
        fp_row.label, "P",
        "the focusPause's own default the display speaks"
    );
    assert_eq!(fp_row.y, 96, "the focusPause row the max-scroll seat wears");
    assert!(
        fp_row.y + fp_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let pc_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugprofilingchart")
        .expect("the scrolled profilingChart row the band restores")
        .clone();
    assert_eq!(
        pc_row.label, "1",
        "the profilingChart's own default the display speaks"
    );
    assert_eq!(
        pc_row.y, 120,
        "the profilingChart row the max-scroll seat wears"
    );
    assert!(
        pc_row.y + pc_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let fc_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugfpscharts")
        .expect("the scrolled fpsCharts row the band restores")
        .clone();
    assert_eq!(
        fc_row.label, "2",
        "the fpsCharts's own default the display speaks"
    );
    assert_eq!(fc_row.y, 144, "the fpsCharts row the max-scroll seat wears");
    assert!(
        fc_row.y + fc_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let nc_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugnetworkcharts")
        .expect("the scrolled networkCharts row the band restores")
        .clone();
    assert_eq!(
        nc_row.label, "3",
        "the networkCharts's own default the display speaks"
    );
    assert_eq!(
        nc_row.y, 168,
        "the networkCharts row the max-scroll seat wears"
    );
    assert!(
        nc_row.y + nc_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let lt_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debuglightmaptexture")
        .expect("the scrolled lightmapTexture row the band restores")
        .clone();
    assert_eq!(
        lt_row.label, "4",
        "the lightmapTexture's own default the display speaks"
    );
    assert_eq!(
        lt_row.y, 192,
        "the lightmapTexture row the max-scroll seat wears"
    );
    assert!(
        lt_row.y + lt_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    // THE FIFTEEN NOTCHES' OWN RETURN (the wheel up the 678 scroll
    // the seated walk's own head the band restores — the overlay the
    // 48, the modifier the 72, the orderI=0 ties the alphabetical
    // seats the clearChat the 96 the copyRecreate the 120 the
    // copyLocation the 144 the spectate the 168 the crash the 192
    // wear)
    app.on_kb_scroll(15.0, 240);
    app.render(&mut fb, &mut assets);
    let dbg_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugoverlay")
        .expect("the scrolled overlay row the band restores")
        .clone();
    assert_eq!(
        dbg_row.label, "F3",
        "the overlay's own default the display speaks"
    );
    assert_eq!(dbg_row.y, 48, "the overlay row the max-scroll seat wears");
    assert!(
        dbg_row.y + dbg_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let mod_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugmodifier")
        .expect("the scrolled modifier row the band restores")
        .clone();
    assert_eq!(
        mod_row.label, "F3",
        "the modifier's own default the display speaks"
    );
    assert_eq!(mod_row.y, 72, "the modifier row the max-scroll seat wears");
    assert!(
        mod_row.y + mod_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let cc_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugclearchat")
        .expect("the scrolled clearChat row the band restores")
        .clone();
    assert_eq!(
        cc_row.label, "D",
        "the clearChat's own default the display speaks"
    );
    assert_eq!(cc_row.y, 96, "the clearChat row the max-scroll seat wears");
    assert!(
        cc_row.y + cc_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let crr_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugcopyrecreate")
        .expect("the scrolled copyRecreate row the band restores")
        .clone();
    assert_eq!(
        crr_row.label, "I",
        "the copyRecreate's own default the display speaks"
    );
    assert_eq!(
        crr_row.y, 120,
        "the copyRecreate row the max-scroll seat wears"
    );
    assert!(
        crr_row.y + crr_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let cl_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugcopylocation")
        .expect("the scrolled copyLocation row the band restores")
        .clone();
    assert_eq!(
        cl_row.label, "C",
        "the copyLocation's own default the display speaks"
    );
    assert_eq!(
        cl_row.y, 144,
        "the copyLocation row the max-scroll seat wears"
    );
    assert!(
        cl_row.y + cl_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let sp_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugspectate")
        .expect("the scrolled spectate row the band restores")
        .clone();
    assert_eq!(
        sp_row.label, "N",
        "the spectate's own default the display speaks"
    );
    assert_eq!(sp_row.y, 168, "the spectate row the max-scroll seat wears");
    assert!(
        sp_row.y + sp_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    let cr_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugcrash")
        .expect("the scrolled crash row the band restores")
        .clone();
    assert_eq!(
        cr_row.label, "C",
        "the crash's own default the display speaks"
    );
    assert_eq!(cr_row.y, 192, "the crash row the max-scroll seat wears");
    assert!(
        cr_row.y + cr_row.h <= 240 - 28,
        "the row the max scroll the fully-visible seat wears"
    );
    // THE SEATED WALK'S OWN REBINDS (the list the seated order the
    // capture lanes serve — the row's own slot the scroll follows,
    // the notch the 24px stride the next seated row the 192 END seat
    // restores; the reset the pair's own default the walks wear)
    // THE OVERLAY'S OWN WALK (the seated head the 48 seat the walk
    // serves — the F3 the store honors)
    app.render(&mut fb, &mut assets);
    let orow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugoverlay")
        .expect("the overlay row the seated walk keeps")
        .clone();
    app.on_click(orow.x + orow.w / 2, orow.y + orow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_OVERLAY),
        "the change the overlay's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::J)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_OVERLAY],
        Some(kb::Bind::Keyboard(minifb::Key::J)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let oreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugoverlay")
        .expect("the reset the overlay's pair keeps")
        .clone();
    app.on_click(oreset.x + oreset.w / 2, oreset.y + oreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_OVERLAY],
        Some(kb::Bind::Keyboard(minifb::Key::F3)),
        "the reset the F3 default restores"
    );
    app.render(&mut fb, &mut assets);
    assert!(
        app.buttons
            .iter()
            .find(|b| b.id == "kb_rowreset_debugoverlay")
            .map(|b| !b.active)
            .unwrap_or(false),
        "the default row the reset the INACTIVE seat wears"
    );
    // THE MODIFIER'S OWN WALK (the seated -1 the 72 seat the walk
    // serves — the F3 the pair's shared default restores)
    app.render(&mut fb, &mut assets);
    let mrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugmodifier")
        .expect("the debugmodifier row the seated walk keeps")
        .clone();
    assert_eq!(mrow.y, 72, "the debugmodifier row the seated 72 seat wears");
    app.on_click(mrow.x + mrow.w / 2, mrow.y + mrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_MODIFIER),
        "the change the modifier's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::K)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_MODIFIER],
        Some(kb::Bind::Keyboard(minifb::Key::K)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let mreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugmodifier")
        .expect("the reset the modifier's pair keeps")
        .clone();
    app.on_click(mreset.x + mreset.w / 2, mreset.y + mreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_MODIFIER],
        Some(kb::Bind::Keyboard(minifb::Key::F3)),
        "the reset the F3 default restores"
    );
    // THE CLEAR CHAT'S OWN WALK (the seated orderI=0 head the
    // 96 seat the alphabetical law wears — the D the plain-shape
    // default restores)
    app.render(&mut fb, &mut assets);
    let ccrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugclearchat")
        .expect("the debugclearchat row the seated walk keeps")
        .clone();
    assert_eq!(
        ccrow.y, 96,
        "the debugclearchat row the seated 96 seat wears"
    );
    app.on_click(ccrow.x + ccrow.w / 2, ccrow.y + ccrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_CLEAR_CHAT),
        "the change the clearChat's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::P)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_CLEAR_CHAT],
        Some(kb::Bind::Keyboard(minifb::Key::P)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let ccreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugclearchat")
        .expect("the reset the clearChat's pair keeps")
        .clone();
    app.on_click(ccreset.x + ccreset.w / 2, ccreset.y + ccreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_CLEAR_CHAT],
        Some(kb::Bind::Keyboard(minifb::Key::D)),
        "the reset the D default restores"
    );
    // THE COPY RECREATE COMMAND'S OWN WALK (the Copy Data word the
    // alphabetical law speaks — the I the plain-shape default
    // restores)
    app.render(&mut fb, &mut assets);
    let crrrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugcopyrecreate")
        .expect("the debugcopyrecreate row the seated walk keeps")
        .clone();
    assert_eq!(
        crrrow.y, 120,
        "the debugcopyrecreate row the seated 120 seat wears"
    );
    app.on_click(crrrow.x + crrrow.w / 2, crrrow.y + crrrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_COPY_RECREATE_COMMAND),
        "the change the copyRecreate's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::V)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_COPY_RECREATE_COMMAND],
        Some(kb::Bind::Keyboard(minifb::Key::V)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let crrreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugcopyrecreate")
        .expect("the reset the copyRecreate's pair keeps")
        .clone();
    app.on_click(crrreset.x + crrreset.w / 2, crrreset.y + crrreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_COPY_RECREATE_COMMAND],
        Some(kb::Bind::Keyboard(minifb::Key::I)),
        "the reset the I default restores"
    );
    // THE COPY LOCATION'S OWN WALK (the Copy Location the Copy
    // Data's AFTER seat the 'D'<'L' compare answers — the C the
    // plain-shape default restores)
    app.render(&mut fb, &mut assets);
    let clrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugcopylocation")
        .expect("the debugcopylocation row the seated walk keeps")
        .clone();
    assert_eq!(
        clrow.y, 144,
        "the debugcopylocation row the seated 144 seat wears"
    );
    app.on_click(clrow.x + clrow.w / 2, clrow.y + clrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_COPY_LOCATION),
        "the change the copyLocation's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::B)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_COPY_LOCATION],
        Some(kb::Bind::Keyboard(minifb::Key::B)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let clreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugcopylocation")
        .expect("the reset the copyLocation's pair keeps")
        .clone();
    app.on_click(clreset.x + clreset.w / 2, clreset.y + clreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_COPY_LOCATION],
        Some(kb::Bind::Keyboard(minifb::Key::C)),
        "the reset the C default restores"
    );
    // THE SPECTATE'S OWN WALK (the Cycle Spectator the C-family's
    // AFTER seat wears — the N the plain-shape default restores)
    app.render(&mut fb, &mut assets);
    let sprow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugspectate")
        .expect("the debugspectate row the seated walk keeps")
        .clone();
    assert_eq!(
        sprow.y, 168,
        "the debugspectate row the seated 168 seat wears"
    );
    app.on_click(sprow.x + sprow.w / 2, sprow.y + sprow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_SPECTATE),
        "the change the spectate's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::W)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_SPECTATE],
        Some(kb::Bind::Keyboard(minifb::Key::W)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let spreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugspectate")
        .expect("the reset the spectate's pair keeps")
        .clone();
    app.on_click(spreset.x + spreset.w / 2, spreset.y + spreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_SPECTATE],
        Some(kb::Bind::Keyboard(minifb::Key::N)),
        "the reset the N default restores"
    );
    // THE CRASH'S OWN WALK (the Debug Crash the D-family's AFTER
    // seat wears — the C the plain-shape default restores)
    app.render(&mut fb, &mut assets);
    let crrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugcrash")
        .expect("the debugcrash row the seated walk keeps")
        .clone();
    assert_eq!(crrow.y, 192, "the debugcrash row the seated 192 seat wears");
    app.on_click(crrow.x + crrow.w / 2, crrow.y + crrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_CRASH),
        "the change the crash's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::M)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_CRASH],
        Some(kb::Bind::Keyboard(minifb::Key::M)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let crreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugcrash")
        .expect("the reset the crash's pair keeps")
        .clone();
    app.on_click(crreset.x + crreset.w / 2, crreset.y + crreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_CRASH],
        Some(kb::Bind::Keyboard(minifb::Key::C)),
        "the reset the C default restores"
    );
    // THE DEBUG OPTIONS' OWN WALK (the notch the next seated row
    // the 192 END seat restores — the F6 the plain-shape default
    // restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let do_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugoptions")
        .expect("the debugoptions row the seated walk keeps")
        .clone();
    assert_eq!(
        do_row.y, 192,
        "the debugoptions row the seated 192 seat wears"
    );
    app.on_click(do_row.x + do_row.w / 2, do_row.y + do_row.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_DEBUG_OPTIONS),
        "the change the debugOptions's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::Y)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_DEBUG_OPTIONS],
        Some(kb::Bind::Keyboard(minifb::Key::Y)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let do_reset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugoptions")
        .expect("the reset the debugOptions's pair keeps")
        .clone();
    app.on_click(do_reset.x + do_reset.w / 2, do_reset.y + do_reset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_DEBUG_OPTIONS],
        Some(kb::Bind::Keyboard(minifb::Key::F6)),
        "the reset the F6 default restores"
    );
    // THE DUMP DYNAMIC TEXTURES' OWN WALK (the next notch the
    // seated tail wears — the S the plain-shape default restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let dtrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugdumptextures")
        .expect("the debugdumptextures row the seated walk keeps")
        .clone();
    assert_eq!(
        dtrow.y, 192,
        "the debugdumptextures row the seated 192 seat wears"
    );
    app.on_click(dtrow.x + dtrow.w / 2, dtrow.y + dtrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_DUMP_DYNAMIC_TEXTURES),
        "the change the dumpDynamicTextures's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::G)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_DUMP_DYNAMIC_TEXTURES],
        Some(kb::Bind::Keyboard(minifb::Key::G)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let dtreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugdumptextures")
        .expect("the reset the dumpDynamicTextures's pair keeps")
        .clone();
    app.on_click(dtreset.x + dtreset.w / 2, dtreset.y + dtreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_DUMP_DYNAMIC_TEXTURES],
        Some(kb::Bind::Keyboard(minifb::Key::S)),
        "the reset the S default restores"
    );
    // THE DUMP VERSION'S OWN WALK (the next notch the seated tail
    // wears — the V the plain-shape default restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let dvrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugdumpversion")
        .expect("the debugdumpversion row the seated walk keeps")
        .clone();
    assert_eq!(
        dvrow.y, 192,
        "the debugdumpversion row the seated 192 seat wears"
    );
    app.on_click(dvrow.x + dvrow.w / 2, dvrow.y + dvrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_DUMP_VERSION),
        "the change the dumpVersion's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::Q)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_DUMP_VERSION],
        Some(kb::Bind::Keyboard(minifb::Key::Q)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let dvreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugdumpversion")
        .expect("the reset the dumpVersion's pair keeps")
        .clone();
    app.on_click(dvreset.x + dvreset.w / 2, dvreset.y + dvreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_DUMP_VERSION],
        Some(kb::Bind::Keyboard(minifb::Key::V)),
        "the reset the V default restores"
    );
    // THE SWITCH GAME MODE'S OWN WALK (the next notch the seated
    // tail wears — the F4 the plain-shape default restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let sgrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugswitchgamemode")
        .expect("the debugswitchgamemode row the seated walk keeps")
        .clone();
    assert_eq!(
        sgrow.y, 192,
        "the debugswitchgamemode row the seated 192 seat wears"
    );
    app.on_click(sgrow.x + sgrow.w / 2, sgrow.y + sgrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_SWITCH_GAME_MODE),
        "the change the switchGameMode's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::X)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_SWITCH_GAME_MODE],
        Some(kb::Bind::Keyboard(minifb::Key::X)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let sgreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugswitchgamemode")
        .expect("the reset the switchGameMode's pair keeps")
        .clone();
    app.on_click(sgreset.x + sgreset.w / 2, sgreset.y + sgreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_SWITCH_GAME_MODE],
        Some(kb::Bind::Keyboard(minifb::Key::F4)),
        "the reset the F4 default restores"
    );
    // THE RELOAD CHUNK'S OWN WALK (the next notch the seated tail
    // wears — the A the plain-shape default restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let rlrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugreloadchunk")
        .expect("the debugreloadchunk row the seated walk keeps")
        .clone();
    assert_eq!(
        rlrow.y, 192,
        "the debugreloadchunk row the seated 192 seat wears"
    );
    app.on_click(rlrow.x + rlrow.w / 2, rlrow.y + rlrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_RELOAD_CHUNK),
        "the change the reloadChunk's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::N)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_RELOAD_CHUNK],
        Some(kb::Bind::Keyboard(minifb::Key::N)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let rlreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugreloadchunk")
        .expect("the reset the reloadChunk's pair keeps")
        .clone();
    app.on_click(rlreset.x + rlreset.w / 2, rlreset.y + rlreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_RELOAD_CHUNK],
        Some(kb::Bind::Keyboard(minifb::Key::A)),
        "the reset the A default restores"
    );
    // THE RELOAD RESOURCE PACKS' OWN WALK (the next notch the
    // seated tail wears — the T the plain-shape default restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let rprow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugreloadpacks")
        .expect("the debugreloadpacks row the seated walk keeps")
        .clone();
    assert_eq!(
        rprow.y, 192,
        "the debugreloadpacks row the seated 192 seat wears"
    );
    app.on_click(rprow.x + rprow.w / 2, rprow.y + rprow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_RELOAD_RESOURCE_PACKS),
        "the change the reloadResourcePacks's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::E)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_RELOAD_RESOURCE_PACKS],
        Some(kb::Bind::Keyboard(minifb::Key::E)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let rpreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugreloadpacks")
        .expect("the reset the reloadResourcePacks's pair keeps")
        .clone();
    app.on_click(rpreset.x + rpreset.w / 2, rpreset.y + rpreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_RELOAD_RESOURCE_PACKS],
        Some(kb::Bind::Keyboard(minifb::Key::T)),
        "the reset the T default restores"
    );
    // THE SHOW ADVANCED TOOLTIPS' OWN WALK (the next notch the
    // seated tail wears — the H the plain-shape default restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let atrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugadvtooltips")
        .expect("the debugadvtooltips row the seated walk keeps")
        .clone();
    assert_eq!(
        atrow.y, 192,
        "the debugadvtooltips row the seated 192 seat wears"
    );
    app.on_click(atrow.x + atrow.w / 2, atrow.y + atrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_SHOW_ADV_TOOLTIPS),
        "the change the advTooltips's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::U)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_SHOW_ADV_TOOLTIPS],
        Some(kb::Bind::Keyboard(minifb::Key::U)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let atreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugadvtooltips")
        .expect("the reset the advTooltips's pair keeps")
        .clone();
    app.on_click(atreset.x + atreset.w / 2, atreset.y + atreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_SHOW_ADV_TOOLTIPS],
        Some(kb::Bind::Keyboard(minifb::Key::H)),
        "the reset the H default restores"
    );
    // THE SHOW CHUNK BORDERS' OWN WALK (the next notch the seated
    // tail wears — the G the plain-shape default restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let cbrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugchunkborders")
        .expect("the debugchunkborders row the seated walk keeps")
        .clone();
    assert_eq!(
        cbrow.y, 192,
        "the debugchunkborders row the seated 192 seat wears"
    );
    app.on_click(cbrow.x + cbrow.w / 2, cbrow.y + cbrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_SHOW_CHUNK_BORDERS),
        "the change the chunkBorders's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::R)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_SHOW_CHUNK_BORDERS],
        Some(kb::Bind::Keyboard(minifb::Key::R)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let cbreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugchunkborders")
        .expect("the reset the chunkBorders's pair keeps")
        .clone();
    app.on_click(cbreset.x + cbreset.w / 2, cbreset.y + cbreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_SHOW_CHUNK_BORDERS],
        Some(kb::Bind::Keyboard(minifb::Key::G)),
        "the reset the G default restores"
    );
    // THE SHOW HITBOXES' OWN WALK (the next notch the seated tail
    // wears — the B the plain-shape default restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let htrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugshowhitboxes")
        .expect("the debugshowhitboxes row the seated walk keeps")
        .clone();
    assert_eq!(
        htrow.y, 192,
        "the debugshowhitboxes row the seated 192 seat wears"
    );
    app.on_click(htrow.x + htrow.w / 2, htrow.y + htrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_SHOW_HITBOXES),
        "the change the hitboxes's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::O)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_SHOW_HITBOXES],
        Some(kb::Bind::Keyboard(minifb::Key::O)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let htreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugshowhitboxes")
        .expect("the reset the hitboxes's pair keeps")
        .clone();
    app.on_click(htreset.x + htreset.w / 2, htreset.y + htreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_SHOW_HITBOXES],
        Some(kb::Bind::Keyboard(minifb::Key::B)),
        "the reset the B default restores"
    );
    // THE PROFILING'S OWN WALK (the next notch the seated tail
    // wears — the L the plain-shape default restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let pfrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugprofiling")
        .expect("the debugprofiling row the seated walk keeps")
        .clone();
    assert_eq!(
        pfrow.y, 192,
        "the debugprofiling row the seated 192 seat wears"
    );
    app.on_click(pfrow.x + pfrow.w / 2, pfrow.y + pfrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_PROFILING),
        "the change the profiling's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::R)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_PROFILING],
        Some(kb::Bind::Keyboard(minifb::Key::R)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let pfreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugprofiling")
        .expect("the reset the profiling's pair keeps")
        .clone();
    app.on_click(pfreset.x + pfreset.w / 2, pfreset.y + pfreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_PROFILING],
        Some(kb::Bind::Keyboard(minifb::Key::L)),
        "the reset the L default restores"
    );
    // THE FOCUS PAUSE'S OWN WALK (the next notch the seated tail
    // wears — the P the plain-shape default restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let fprow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugfocuspause")
        .expect("the debugfocuspause row the seated walk keeps")
        .clone();
    assert_eq!(
        fprow.y, 192,
        "the debugfocuspause row the seated 192 seat wears"
    );
    app.on_click(fprow.x + fprow.w / 2, fprow.y + fprow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_FOCUS_PAUSE),
        "the change the focusPause's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::Z)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_FOCUS_PAUSE],
        Some(kb::Bind::Keyboard(minifb::Key::Z)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let fpreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugfocuspause")
        .expect("the reset the focusPause's pair keeps")
        .clone();
    app.on_click(fpreset.x + fpreset.w / 2, fpreset.y + fpreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_FOCUS_PAUSE],
        Some(kb::Bind::Keyboard(minifb::Key::P)),
        "the reset the P default restores"
    );
    // THE PROFILING CHART'S OWN WALK (the next notch the orderI=1
    // row the 192 END seat restores — the '1' the FIVE-arg default
    // restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let pcrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugprofilingchart")
        .expect("the debugprofilingchart row the seated walk keeps")
        .clone();
    assert_eq!(
        pcrow.y, 192,
        "the debugprofilingchart row the seated 192 seat wears"
    );
    app.on_click(pcrow.x + pcrow.w / 2, pcrow.y + pcrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_PROFILING_CHART),
        "the change the profilingChart's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::W)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_PROFILING_CHART],
        Some(kb::Bind::Keyboard(minifb::Key::W)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let pcreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugprofilingchart")
        .expect("the reset the profilingChart's pair keeps")
        .clone();
    app.on_click(pcreset.x + pcreset.w / 2, pcreset.y + pcreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_PROFILING_CHART],
        Some(kb::Bind::Keyboard(minifb::Key::Key1)),
        "the reset the '1' default restores"
    );
    // THE FPS CHARTS'S OWN WALK (the next notch the orderI=2 row
    // the 192 END seat restores — the '2' the FIVE-arg default
    // restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let fcrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugfpscharts")
        .expect("the debugfpscharts row the seated walk keeps")
        .clone();
    assert_eq!(
        fcrow.y, 192,
        "the debugfpscharts row the seated 192 seat wears"
    );
    app.on_click(fcrow.x + fcrow.w / 2, fcrow.y + fcrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_FPS_CHARTS),
        "the change the fpsCharts's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::A)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_FPS_CHARTS],
        Some(kb::Bind::Keyboard(minifb::Key::A)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let fcreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugfpscharts")
        .expect("the reset the fpsCharts's pair keeps")
        .clone();
    app.on_click(fcreset.x + fcreset.w / 2, fcreset.y + fcreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_FPS_CHARTS],
        Some(kb::Bind::Keyboard(minifb::Key::Key2)),
        "the reset the '2' default restores"
    );
    // THE NETWORK CHARTS'S OWN WALK (the next notch the orderI=3
    // row the 192 END seat restores — the '3' the FIVE-arg default
    // restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let ncrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugnetworkcharts")
        .expect("the debugnetworkcharts row the seated walk keeps")
        .clone();
    assert_eq!(
        ncrow.y, 192,
        "the debugnetworkcharts row the seated 192 seat wears"
    );
    app.on_click(ncrow.x + ncrow.w / 2, ncrow.y + ncrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_NETWORK_CHARTS),
        "the change the networkCharts's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::S)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_NETWORK_CHARTS],
        Some(kb::Bind::Keyboard(minifb::Key::S)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let ncreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debugnetworkcharts")
        .expect("the reset the networkCharts's pair keeps")
        .clone();
    app.on_click(ncreset.x + ncreset.w / 2, ncreset.y + ncreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_NETWORK_CHARTS],
        Some(kb::Bind::Keyboard(minifb::Key::Key3)),
        "the reset the '3' default restores"
    );
    // THE LIGHTMAP TEXTURE'S OWN WALK (the THREE notches the max
    // clamp the orderI=4 row the 192 END seat restores — the '4' the
    // FIVE-arg default restores)
    app.on_kb_scroll(-1.0, 240);
    app.on_kb_scroll(-1.0, 240);
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let ltrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debuglightmaptexture")
        .expect("the debuglightmaptexture row the seated walk keeps")
        .clone();
    assert_eq!(
        ltrow.y, 192,
        "the debuglightmaptexture row the seated 192 seat wears"
    );
    app.on_click(ltrow.x + ltrow.w / 2, ltrow.y + ltrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_LIGHTMAP_TEXTURE),
        "the change the lightmapTexture's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::U)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_LIGHTMAP_TEXTURE],
        Some(kb::Bind::Keyboard(minifb::Key::U)),
        "the rebind the store writes"
    );
    app.render(&mut fb, &mut assets);
    let ltreset = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_rowreset_debuglightmaptexture")
        .expect("the reset the lightmapTexture's pair keeps")
        .clone();
    app.on_click(ltreset.x + ltreset.w / 2, ltreset.y + ltreset.h / 2);
    assert_eq!(
        app.key_binds[kb::DEBUG_LIGHTMAP_TEXTURE],
        Some(kb::Bind::Keyboard(minifb::Key::Key4)),
        "the reset the '4' default restores"
    );
    app.on_kb_scroll(1.0, 240);
    app.on_kb_scroll(1.0, 240);
    app.on_kb_scroll(1.0, 240);
    // THE FILE'S OWN ROWS (the family save the exit banks — the
    // twenty-two-row walk the spectatorHotbar row's AFTER the ladder
    // appends, the networkCharts row the file's own END the
    // last row wears)
    app.key_binds[kb::DEBUG_OVERLAY] = Some(kb::Bind::Keyboard(minifb::Key::J));
    app.open(ScreenKind::Controls);
    let text = std::fs::read_to_string(&path).expect("the exit save wrote");
    assert!(
        text.contains(
            "key_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.j\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\nkey_key.debug.copyRecreateCommand:key.keyboard.i\nkey_key.debug.spectate:key.keyboard.n\nkey_key.debug.switchGameMode:key.keyboard.f4\nkey_key.debug.debugOptions:key.keyboard.f6\nkey_key.debug.focusPause:key.keyboard.p\nkey_key.debug.dumpDynamicTextures:key.keyboard.s\nkey_key.debug.reloadResourcePacks:key.keyboard.t\nkey_key.debug.profiling:key.keyboard.l\nkey_key.debug.copyLocation:key.keyboard.c\nkey_key.debug.dumpVersion:key.keyboard.v\nkey_key.debug.profilingChart:key.keyboard.1\nkey_key.debug.fpsCharts:key.keyboard.2\nkey_key.debug.networkCharts:key.keyboard.3\nkey_key.debug.lightmapTexture:key.keyboard.4\n"
        ),
        "the debug twenty-two-row walk the ladder's own tail the file wears"
    );
    assert!(
        text.trim_end()
            .ends_with("key_key.debug.lightmapTexture:key.keyboard.4"),
        "the lightmapTexture row the file's own END wears"
    );
    // THE RESTART'S OWN DECODE (the fresh boot the pair's truth wears)
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert_eq!(
        re.key_binds[kb::DEBUG_OVERLAY],
        Some(kb::Bind::Keyboard(minifb::Key::J)),
        "the restart the rebind the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_MODIFIER],
        Some(kb::Bind::Keyboard(minifb::Key::F3)),
        "the restart the modifier's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_CRASH],
        Some(kb::Bind::Keyboard(minifb::Key::C)),
        "the restart the crash's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_RELOAD_CHUNK],
        Some(kb::Bind::Keyboard(minifb::Key::A)),
        "the restart the reload's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_SHOW_HITBOXES],
        Some(kb::Bind::Keyboard(minifb::Key::B)),
        "the restart the hitboxes' default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_CLEAR_CHAT],
        Some(kb::Bind::Keyboard(minifb::Key::D)),
        "the restart the clearChat's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_SHOW_CHUNK_BORDERS],
        Some(kb::Bind::Keyboard(minifb::Key::G)),
        "the restart the chunkBorders' default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_SHOW_ADV_TOOLTIPS],
        Some(kb::Bind::Keyboard(minifb::Key::H)),
        "the restart the advTooltips' default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_COPY_RECREATE_COMMAND],
        Some(kb::Bind::Keyboard(minifb::Key::I)),
        "the restart the copyRecreate's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_SPECTATE],
        Some(kb::Bind::Keyboard(minifb::Key::N)),
        "the restart the spectate's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_SWITCH_GAME_MODE],
        Some(kb::Bind::Keyboard(minifb::Key::F4)),
        "the restart the switchGameMode's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_DEBUG_OPTIONS],
        Some(kb::Bind::Keyboard(minifb::Key::F6)),
        "the restart the debugOptions' default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_FOCUS_PAUSE],
        Some(kb::Bind::Keyboard(minifb::Key::P)),
        "the restart the focusPause's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_DUMP_DYNAMIC_TEXTURES],
        Some(kb::Bind::Keyboard(minifb::Key::S)),
        "the restart the dumpDynamicTextures' default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_RELOAD_RESOURCE_PACKS],
        Some(kb::Bind::Keyboard(minifb::Key::T)),
        "the restart the reloadResourcePacks' default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_PROFILING],
        Some(kb::Bind::Keyboard(minifb::Key::L)),
        "the restart the profiling' default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_COPY_LOCATION],
        Some(kb::Bind::Keyboard(minifb::Key::C)),
        "the restart the copyLocation's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_DUMP_VERSION],
        Some(kb::Bind::Keyboard(minifb::Key::V)),
        "the restart the dumpVersion's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_PROFILING_CHART],
        Some(kb::Bind::Keyboard(minifb::Key::Key1)),
        "the restart the profilingChart's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_FPS_CHARTS],
        Some(kb::Bind::Keyboard(minifb::Key::Key2)),
        "the restart the fpsCharts's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_NETWORK_CHARTS],
        Some(kb::Bind::Keyboard(minifb::Key::Key3)),
        "the restart the networkCharts's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::DEBUG_LIGHTMAP_TEXTURE],
        Some(kb::Bind::Keyboard(minifb::Key::Key4)),
        "the restart the lightmapTexture's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::SPECTATOR_HOTBAR],
        Some(kb::Bind::Mouse(minifb::MouseButton::Middle)),
        "the restart the pair's default the decode keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// THE CYCLE SPECTATOR'S OWN CHORD (the keyDebugSpectate lane the
// spectate_209.py walk banks — the survival default the spectator
// arm wears, the offline seam the previous bank + the abilities
// re-grant the apply_gamemode_hovered's own walk keeps, the restore
// the firstNonNull previous the wire-less bank returns, the used
// flag the walk banks)
#[test]
fn the_cycle_spectator_chord_the_spectator_arm_the_restore_wears() {
    let mut app = App::new(262);
    app.enter_world();
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let rows_before = app.chat_messages.len();
    // THE SPECTATOR ARM (the survival default the isSpectator false
    // verdict wears — the offline apply the SPECTATOR mode banks)
    app.on_cycle_spectator_key();
    assert_eq!(
        app.game_mode,
        GameMode::Spectator,
        "the spectator arm the mode banks"
    );
    assert_eq!(
        app.previous_game_mode,
        Some(GameMode::Survival),
        "the previous bank the changed mode earns"
    );
    {
        let b = app.body.as_ref().expect("the body rides");
        assert!(b.flying, "the spectator regrant the flight banks");
        assert!(b.invulnerable, "the invulnerable word the mode keeps");
    }
    assert_eq!(
        app.chat_messages.len(),
        rows_before,
        "the permission pass the word never speaks"
    );
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    app.on_debug_modifier_released();
    // THE RESTORE ARM (the spectator verdict the previous wears —
    // the Survival the bank carries the wire-less apply returns)
    app.on_cycle_spectator_key();
    assert_eq!(
        app.game_mode,
        GameMode::Survival,
        "the restore the previous returns"
    );
    assert_eq!(
        app.previous_game_mode,
        Some(GameMode::Spectator),
        "the restore's own previous the bank keeps"
    );
    {
        let b = app.body.as_ref().expect("the body rides");
        assert!(!b.flying, "the survival regrant the flight revokes");
        assert!(!b.invulnerable, "the survival the invulnerable revokes");
    }
    assert!(app.debug_modifier_used, "the second chord the flag banks");
    let _ = app;
}

// THE CYCLE SPECTATOR'S OWN FALLBACK + ERROR WORD (the restore's
// None-previous arm the CREATIVE default keeps, the player-null arm
// the `ifnull -> 462` the error word wears — the copyLocation's
// silent-null the neighbor differs, the mode the word never moves,
// the flag the error arm's own goto banks)
#[test]
fn the_cycle_spectator_chord_the_creative_fallback_the_null_body_error() {
    let mut app = App::new(262);
    app.enter_world();
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    // THE SPECTATOR BANK (the direct mode the previous None keeps —
    // the restore the CREATIVE fallback the null default wears)
    app.game_mode = GameMode::Spectator;
    app.on_cycle_spectator_key();
    assert_eq!(
        app.game_mode,
        GameMode::Creative,
        "the None previous the CREATIVE fallback restores"
    );
    assert_eq!(
        app.previous_game_mode,
        Some(GameMode::Spectator),
        "the fallback's own previous the bank keeps"
    );
    app.on_debug_modifier_released();
    // THE NULL-BODY ARM (the gate chain the error word speaks — the
    // mode the word never moves, the bank the word never moves)
    app.body = None;
    let rows_before = app.chat_messages.len();
    app.on_cycle_spectator_key();
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("\u{a7}eUnable to switch game mode; no permission"),
        "the error word the lang row keeps"
    );
    assert_eq!(
        app.chat_messages.len(),
        rows_before + 1,
        "the single word the arm speaks"
    );
    assert_eq!(
        app.game_mode,
        GameMode::Creative,
        "the word the mode never moves"
    );
    assert_eq!(
        app.previous_game_mode,
        Some(GameMode::Spectator),
        "the word the bank never moves"
    );
    assert!(app.debug_modifier_used, "the error arm the flag TOO banks");
    let _ = app;
}

// THE CYCLE SPECTATOR'S OWN WIRE PACKET (the ServerboundChangeGameModePacket
// the wire arm sends ALONE — the local prediction the echo lane
// fills, the previous bank the wire never touches, the restore the
// CREATIVE fallback the same packet lane speaks)
#[test]
fn the_cycle_spectator_chord_the_wire_packet_the_no_prediction() {
    let mut app = App::new(262);
    app.enter_world();
    app.world_is_network = true;
    app.body = Some(crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0));
    let (tx, rx) = std::sync::mpsc::channel::<crate::world_view::ContainerCommand>();
    app.attach_live_clicks(tx);
    // THE SPECTATOR ARM (the packet ALONE the wire sends — the
    // SPECTATOR wire id 3 the GameType clinit banks)
    app.on_cycle_spectator_key();
    match rx.try_recv().expect("the spectator arm the packet carried") {
        crate::world_view::ContainerCommand::ChangeGameMode { mode_id } => {
            assert_eq!(mode_id, 3, "the SPECTATOR wire id the clinit banks");
        }
        _ => panic!("the game-mode packet's own shape"),
    }
    assert_eq!(
        app.game_mode,
        GameMode::Survival,
        "the mode the server owns"
    );
    assert!(
        app.previous_game_mode.is_none(),
        "the bank the wire never fills"
    );
    // THE RESTORE ARM (the previous None the CREATIVE fallback keeps
    // — the wire the CREATIVE id 1 the same packet lane speaks)
    app.game_mode = GameMode::Spectator;
    app.on_cycle_spectator_key();
    match rx.try_recv().expect("the restore arm the packet carried") {
        crate::world_view::ContainerCommand::ChangeGameMode { mode_id } => {
            assert_eq!(mode_id, 1, "the CREATIVE fallback the wire speaks");
        }
        _ => panic!("the game-mode packet's own shape"),
    }
    assert_eq!(
        app.game_mode,
        GameMode::Spectator,
        "the wire world the local bank never moves"
    );
    assert!(app.debug_modifier_used, "the modifier the chord spends");
    let _ = app;
}

// === autopush-210: THE ITEM TOOLTIP'S OWN RENDER ===

/// THE ITEM TOOLTIP'S OWN ROWS (the 26.2 Screen.getTooltipFromItem ->
/// addDetailsToTooltip walk — the name row the honest stand-in keeps,
/// the ADVANCED ID row the pc 602-632 arm serves, the durability +
/// components rows the model's own walls keep silent).
#[test]
fn the_item_tooltip_rows_the_name_row_the_advanced_id_row_wear() {
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    // THE NORMAL FLAG (the options.advancedItemTooltips false the
    // pc 16-19 ifeq arm serves — the name row ALONE the list keeps)
    assert_eq!(
        App::item_tooltip_rows(dirt, false),
        vec![("Dirt".to_string(), [1.0, 1.0, 1.0])],
        "the name row the white the stand-in wears"
    );
    // THE ADVANCED FLAG (the TooltipFlag.Default.ADVANCED the pc 22
    // arm serves — the ID row the pc 602-632 arm banks, the
    // `minecraft:<name>` word the Item.toString's getRegisteredName
    // answers, the 0x555555 the DARK_GRAY keeps)
    let rows = App::item_tooltip_rows(dirt, true);
    assert_eq!(
        rows,
        vec![
            ("Dirt".to_string(), [1.0, 1.0, 1.0]),
            ("minecraft:dirt".to_string(), [0.3333, 0.3333, 0.3333]),
        ],
        "the advanced walk the ID row the dark gray wears"
    );
    // THE MODEL'S OWN WALLS (the isDamaged gate the pc 543-547 arm
    // keeps + the components.size() gate the pc 633-644 arm keeps —
    // the wire's stacks the damage/component data never carry, the
    // rows the walls never let speak)
    assert_eq!(rows.len(), 2, "the walls the advanced list's tail holds");
    let _ = rows;
}

/// THE ITEM TOOLTIP'S OWN ORIGIN (the DefaultTooltipPositioner walk —
/// the base the +12/-12 add keeps, the right overflow the -24 flip
/// the max-4 clamp holds, the bottom the +3 gap the screen edge
/// answers).
#[test]
fn the_item_tooltip_origin_the_flip_the_bottom_the_max4_wear() {
    // THE BASE (the `new Vector2i(mx, my).add(12, -12)` pc 4-14 walk)
    assert_eq!(
        App::item_tooltip_origin(100, 200, 50, 33, 640, 360),
        (112, 188),
        "the base the +12/-12 offsets keep"
    );
    // THE RIGHT OVERFLOW (the `max(x - 24 - tooltipWidth, 4)` flip the
    // pc 11-26 arm serves: 112 + 50 = 162 > 140 -> max(112 - 24 - 50, 4) = 38)
    assert_eq!(
        App::item_tooltip_origin(100, 200, 50, 33, 140, 360),
        (38, 188),
        "the flip the room the left seat keeps"
    );
    // THE MAX-4 WALL (the flip's own floor the pc 22-26 arm keeps:
    // 32 + 50 = 82 > 80 -> max(32 - 24 - 50, 4) = 4)
    assert_eq!(
        App::item_tooltip_origin(20, 200, 50, 33, 80, 360),
        (4, 188),
        "the deep flip the max-4 clamp answers"
    );
    // THE FLIP'S OWN ROOM (the mx+12-24-tw the positive lane wears:
    // 600+12 = 612, 612 + 50 = 662 > 640 -> max(612 - 24 - 50, 4) = 538)
    assert_eq!(
        App::item_tooltip_origin(600, 200, 50, 33, 640, 360),
        (538, 188),
        "the mid-screen flip the room the cursor keeps"
    );
    // THE BOTTOM CLAMP (the `y = screenHeight - (tooltipHeight + 3)`
    // pc 29-51 arm: 348 + 33 + 3 = 384 > 360 -> 360 - 36)
    assert_eq!(
        App::item_tooltip_origin(100, 360, 50, 33, 640, 360),
        (112, 324),
        "the bottom the +3 gap the clamp answers"
    );
    let _ = App::item_tooltip_origin(0, 0, 0, 0, 0, 0);
}

/// THE ITEM TOOLTIP'S OWN GATE (the extractTooltip walk — the
/// hoveredSlot.hasItem gate the seat walk keeps, the carried wall the
/// showTooltipWithItemInHand false answer serves, the advanced flag
/// the F3+H chord's own field wears).
#[test]
fn the_item_tooltip_gate_the_hasItem_and_carried_walls_speak() {
    let mut app = App::new(262);
    app.enter_world();
    app.open_inventory();
    app.inv_panel = inventory_panel_origin(640, 360);
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    // THE SEAT'S OWN STACK (the offline store the local truth keeps —
    // slot 11 the dirt stack wears)
    app.inventory.items[11] = (dirt, 7);
    let (sx, sy) = inventory_slot_origin(app.inv_panel, 11);
    let (id, rows) = app
        .item_tooltip_hover(sx + 8, sy + 8)
        .expect("the filled seat the gate answers");
    assert_eq!(id, dirt, "the hovered seat's own id the gate banks");
    assert_eq!(
        rows,
        vec![("Dirt".to_string(), [1.0, 1.0, 1.0])],
        "the normal flag the name row alone wears"
    );
    // THE EMPTY SEAT (the hasItem() false the pc 11-14 return keeps)
    let (ex, ey) = inventory_slot_origin(app.inv_panel, 12);
    assert!(
        app.item_tooltip_hover(ex + 8, ey + 8).is_none(),
        "the empty seat the hasItem wall refuses"
    );
    // THE CARRIED WALL (the getCarried().isEmpty() false the
    // showTooltipWithItemInHand false answer keeps — the image-less
    // stacks the Optional.empty().orElse(false) walk the FALSE serves)
    app.inv_carried = (dirt, 1);
    assert!(
        app.item_tooltip_hover(sx + 8, sy + 8).is_none(),
        "the carried stack the wall refuses"
    );
    app.inv_carried = (0, 0);
    // THE ADVANCED FLAG (the F3+H chord's own field the flag IS — the
    // ID row the advanced walk banks)
    app.on_advanced_tooltips_key();
    assert!(app.advanced_item_tooltips, "the chord the flip speaks");
    let (_, rows) = app
        .item_tooltip_hover(sx + 8, sy + 8)
        .expect("the advanced flag the gate keeps the seat");
    assert_eq!(
        rows[1],
        ("minecraft:dirt".to_string(), [0.3333, 0.3333, 0.3333]),
        "the ID row the advanced flag wears"
    );
    let _ = app;
}

/// THE CREATIVE PICKER'S OWN SEAT (the CreativeModeInventoryScreen's
/// extractTooltip ride — the palette/store/wrapped seats the same
/// gates serve, the INVENTORY tab's wrapped-first order the palette
/// grid's overlap answers, the BIN the hasItem false keeps, the
/// carried wall the same field the picker's own render reads).
#[test]
fn the_creative_item_tooltip_the_palette_the_wrap_the_walls_wear() {
    let mut app = App::new(262);
    app.enter_world();
    app.game_mode = GameMode::Creative;
    app.open_inventory();
    assert_eq!(app.screen, ScreenKind::Creative, "the creative arm opens");
    app.creative_panel = (100, 50);
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    let stone = mc_registry::item_id_by_name("stone").unwrap();
    // THE PALETTE SEAT (the picker tab's own 45 — cell 0 at (9, 18))
    app.creative_palette[0] = (dirt, 1);
    let (id, rows) = app
        .creative_item_tooltip_hover(100 + 9 + 8, 50 + 18 + 8)
        .expect("the palette seat the gate answers");
    assert_eq!(id, dirt, "the hovered palette cell's own id the gate banks");
    assert_eq!(
        rows,
        vec![("Dirt".to_string(), [1.0, 1.0, 1.0])],
        "the normal flag the name row alone wears"
    );
    // THE STORE SEAT (the believed hotbar row at y 112 — the
    // click_creative_store walk's own reads)
    app.inventory.items[3] = (dirt, 5);
    let (id, _) = app
        .creative_item_tooltip_hover(100 + 9 + 3 * 18 + 8, 50 + 112 + 8)
        .expect("the store seat the gate answers");
    assert_eq!(id, dirt, "the store row's own stack the gate banks");
    // THE WRAPPED TAB (the INVENTORY tab the palette hides — the
    // wrapped-first order the overlapping region answers)
    let inv_tab = (0..CREATIVE_TABS.len())
        .find(|&i| CREATIVE_TABS[i].kind == CreativeTabKind::Inventory)
        .expect("the inventory tab the strip keeps");
    app.select_creative_tab(inv_tab);
    // THE OVERLAP DISCRIMINATOR (the palette cell 9's region the
    // wrapped main-9 seat covers — the STORE's stack the gate speaks,
    // the palette's own item the wrapped tab never serves)
    app.creative_palette[9] = (stone, 1);
    app.inventory.items[9] = (dirt, 2);
    let (id, _) = app
        .creative_item_tooltip_hover(100 + 9 + 8, 50 + 54 + 8)
        .expect("the wrapped main seat the gate answers");
    assert_eq!(
        id, dirt,
        "the wrapped-first order the store's stack speaks — the palette's own item the overlap never serves"
    );
    // THE ARMOR + OFFHAND SEATS (the wrap walk's own rows)
    app.inventory.armor[0] = (dirt, 1);
    app.inventory.offhand[0] = (dirt, 1);
    let (id, _) = app
        .creative_item_tooltip_hover(100 + 54 + 8, 50 + 6 + 8)
        .expect("the armor seat the gate answers");
    assert_eq!(id, dirt, "the armor row's own stack the gate banks");
    let (id, _) = app
        .creative_item_tooltip_hover(100 + 35 + 8, 50 + 20 + 8)
        .expect("the offhand seat the gate answers");
    assert_eq!(id, dirt, "the offhand's own stack the gate banks");
    // THE BIN (the destroyed-slot's hasItem false the walk keeps)
    assert!(
        app.creative_item_tooltip_hover(100 + 173 + 8, 50 + 112 + 8)
            .is_none(),
        "the bin the hasItem wall refuses"
    );
    // THE CARRIED WALL (the picker's own carried field the walk reads)
    app.select_creative_tab(0);
    app.creative_palette[0] = (dirt, 1); // the tab select's own refill the set re-wears
    app.inv_carried = (dirt, 1);
    assert!(
        app.creative_item_tooltip_hover(100 + 9 + 8, 50 + 18 + 8)
            .is_none(),
        "the carried stack the wall refuses"
    );
    app.inv_carried = (0, 0);
    // THE ADVANCED FLAG (the F3+H chord's own field the rows wear)
    app.on_advanced_tooltips_key();
    let (_, rows) = app
        .creative_item_tooltip_hover(100 + 9 + 8, 50 + 18 + 8)
        .expect("the advanced flag the gate keeps the seat");
    assert_eq!(
        rows[1],
        ("minecraft:dirt".to_string(), [0.3333, 0.3333, 0.3333]),
        "the ID row the advanced flag wears"
    );
    let _ = app;
}

/// THE DUMP DYNAMIC TEXTURES' OWN LANE (the 26.2 debug entry the
/// F3+S chord wears — the pc 802 lane the silentlanes_212.py walk
/// banks: the debug path the two resolves keep, the sheet the
/// writeAsPNG law writes, the sprite list the dumpSpriteNames law
/// keeps, the yellow word the feedback channel speaks, the flag
/// the no-gate walk banks).
#[test]
fn the_dump_dynamic_textures_key_the_sheets_the_word_the_flag_wear() {
    // THE PRE-CLEAN (the createDirectories the overwrite keeps —
    // the stale dumps a crashed prior walk never leaves)
    let _ = std::fs::remove_dir_all("screenshots/debug");
    let mut app = App::new(262);
    // THE WORD + THE FLAG (pc 883-903 — the no-gate walk the
    // bank wears regardless)
    app.on_dump_dynamic_textures_key();
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("\u{a7}eSaved dynamic textures to \u{a7}nscreenshots/debug"),
        "the saved-word row the yellow + the UNDERLINE arg wears"
    );
    assert!(
        app.debug_modifier_used,
        "the used flag the no-gate walk banks"
    );
    app.on_debug_modifier_released();
    assert!(!app.debug_modifier_used, "the release the flag consumes");
    // THE SHEET (the writeAsPNG law — the mip-0 file the
    // `\x01_\x01.png` concat keeps, the dims the 32-col grid the
    // 929 tiles wear)
    let sheet = std::path::Path::new("screenshots/debug/minecraft_blocks_0.png");
    let bytes = std::fs::read(sheet).expect("the sheet the dump writes");
    assert_eq!(
        &bytes[12..16],
        b"IHDR",
        "the PNG the sheet's own signature keeps"
    );
    let dw = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let dh = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    assert_eq!(dw, 512, "the 32-col grid the sheet width wears");
    assert_eq!(dh, 480, "the 30-row grid the sheet height wears");
    // THE SPRITE LIST (the dumpSpriteNames law — the key sort the
    // comparingByKey stream wears, the Locale.ROOT format the tab
    // rows keep, the position the tile's OWN index speaks)
    let list = std::fs::read_to_string("screenshots/debug/minecraft_blocks.txt")
        .expect("the list the dump writes");
    let rows: Vec<&str> = list.lines().collect();
    let names = mc_registry::generated::atlas::TILE_NAMES;
    assert_eq!(
        rows.len(),
        names.len(),
        "the atlas's own members the list banks"
    );
    let mut sorted: Vec<&str> = names.to_vec();
    sorted.sort();
    let first = names.iter().position(|&n| n == sorted[0]).unwrap();
    assert_eq!(
        rows[0],
        format!(
            "{}\tx={}\ty={}\tw=16\th=16",
            sorted[0],
            (first % 32) * 16,
            (first / 32) * 16
        ),
        "the first row the key sort + the tile's own seat wear"
    );
    let last = names.iter().position(|&n| n == sorted[928]).unwrap();
    assert_eq!(
        rows[928],
        format!(
            "{}\tx={}\ty={}\tw=16\th=16",
            sorted[928],
            (last % 32) * 16,
            (last / 32) * 16
        ),
        "the tail row the key sort + the tile's own seat wear"
    );
    // THE CLEANUP (the test's own dir the walk removes)
    let _ = std::fs::remove_dir_all("screenshots/debug");
}

/// THE F3+F6 DEBUG OPTIONS' OWN WALK (javap 26.2
/// KeyboardHandler.handleDebugKeys's keyDebugDebugOptions lane — the
/// `matches` gate pc 650, the instanceof/interrupt arms pc 671-743,
/// the used flag pc 746-747 the ALL-arms law banks).
#[test]
fn the_debug_options_chord_the_open_the_second_tap_the_flag_wear() {
    let mut app = App::new(3);
    app.enter_world();
    // THE OPEN WALK (the else arm — the screen-null world the
    // canInterrupt TRUE verdict keeps, the DebugOptionsScreen the
    // setScreen opens, the flag the matched gate banks)
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the interrupt gate the debug options opens"
    );
    assert!(
        app.debug_modifier_used,
        "the used flag the matched gate banks"
    );
    app.on_debug_modifier_released();
    assert!(!app.debug_modifier_used, "the release the flag consumes");
    // THE SECOND TAP'S OWN CLOSE (the instanceof arm pc 674-690 —
    // the open screen the onClose ride answers, the world the
    // setScreen(null) law keeps)
    app.on_debug_options_key();
    assert_eq!(app.screen, ScreenKind::World, "the instanceof arm closes");
    assert!(
        app.debug_modifier_used,
        "the close walk the same istore banks"
    );
    app.on_debug_modifier_released();
    // THE ESCAPE'S OWN LANE (the base Screen's shouldCloseOnEsc
    // default the class never overrides — the reopen the Escape
    // the same close wears)
    app.on_debug_options_key();
    assert_eq!(app.screen, ScreenKind::DebugOptions, "the reopen serves");
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World, "the Escape lane closes");
    // THE PAUSE ARM (the else-branch law — the paused world the
    // interrupt allows, the close ride the pause flag releases)
    app.paused = true;
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the paused world the interrupt allows"
    );
    assert!(!app.paused, "the close ride the pause releases");
}

#[test]
fn the_debug_options_chord_the_dead_world_refuses_the_flag_banks() {
    let mut app = App::new(3);
    app.enter_world();
    // THE DEATH SCREEN'S OWN REFUSAL (the DeathScreen
    // shouldCloseOnEsc iconst_0 override the canInterrupt FALSE
    // verdict speaks — the dead world the open walk never serves,
    // the used flag the ifeq->746 jump STILL banks)
    app.dead = true;
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the death screen the interrupt refuses"
    );
    assert!(
        app.debug_modifier_used,
        "the refusal the same istore banks (the ifeq->746 jump)"
    );
    app.on_debug_modifier_released();
    // THE LOCK CONFIRM'S OWN REFUSAL (the shouldCloseOnEsc=false
    // walk the Escape lane already banks — the modal question the
    // interrupt never steals)
    app.dead = false;
    app.screen = ScreenKind::DifficultyLockConfirm;
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DifficultyLockConfirm,
        "the lock confirm the interrupt refuses"
    );
    assert!(app.debug_modifier_used, "the refusal the same istore banks");
}

#[test]
fn the_debug_options_screen_the_rows_the_profiles_the_chrome_wear() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_debug_options_key();
    // THE FRESH PROFILE'S OWN DEFAULT (the profile field the
    // isUsingProfile identity banks, the renderer flags the fresh
    // client keeps)
    assert_eq!(
        app.debug_profile,
        Some(false),
        "the fresh client the DEFAULT profile keeps"
    );
    assert_eq!(
        app.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY],
        crate::screens::DebugEntryStatus::Never,
        "the fresh entries the NEVER arm keeps"
    );
    // THE ENTRY PILLS' OWN WALKS (the CycleButton setValue law —
    // the coral Always pill the ALWAYS_ON banks, the yellow In
    // Overlay pill the IN_OVERLAY arm speaks, the white Off pill the
    // NEVER keeps, the manual touch the profile identity BREAKS —
    // the setStatus putfield null the both buttons' active law
    // re-arms)
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let mut assets = GuiAssets::open(assets_root());
    // THE RENDERER BLOCK'S OWN SEAT (the rows the visible window the
    // scroll carries — the chunk_borders/entity_hitboxes seats the
    // 700px offset brings into the band)
    app.debug_scroll = 700.0;
    app.render(&mut fb, &mut assets);
    let always_border = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgalways" && b.label == "31")
        .expect("the always pill the row wears")
        .clone();
    app.on_click(always_border.x + always_border.w / 2, always_border.y + 10);
    assert_eq!(
        app.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY],
        crate::screens::DebugEntryStatus::Always,
        "the coral pill the ALWAYS_ON banks"
    );
    assert_eq!(
        app.debug_profile, None,
        "the manual touch the profile identity breaks"
    );
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the pill click the screen never leaves"
    );
    // THE SECOND RENDER'S OWN SEAT (the pills the fresh render
    // rebuilds — the buttons the dispatch's clear keeps, the
    // hovered() the row's rect the second render arms)
    app.render(&mut fb, &mut assets);
    let overlay_hitboxes = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgoverlay" && b.label == "35")
        .expect("the overlay pill the rebuild wears")
        .clone();
    app.on_click(
        overlay_hitboxes.x + overlay_hitboxes.w / 2,
        overlay_hitboxes.y + 10,
    );
    assert_eq!(
        app.debug_entry_states[crate::screens::ENTITY_HITBOXES_ENTRY],
        crate::screens::DebugEntryStatus::Overlay,
        "the yellow pill the IN_OVERLAY arm speaks"
    );
    // THE THIRD RENDER'S OWN SEAT (the Off pill the NEVER arm wears)
    app.render(&mut fb, &mut assets);
    let off_hitboxes = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgoff" && b.label == "35")
        .expect("the off pill the rebuild wears")
        .clone();
    app.on_click(off_hitboxes.x + off_hitboxes.w / 2, off_hitboxes.y + 10);
    assert_eq!(
        app.debug_entry_states[crate::screens::ENTITY_HITBOXES_ENTRY],
        crate::screens::DebugEntryStatus::Never,
        "the white pill the NEVER keeps"
    );
    // THE PERFORMANCE PROFILE'S OWN WALK (the lambda$addProfileButton$0
    // — the loadProfile + save + refreshEntries the click serves, the
    // FPS ALWAYS_ON word the fps_charts flag wears, the renderer flags
    // the reset keeps OFF, the identity the putfield restores)
    app.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY] =
        crate::screens::DebugEntryStatus::Always;
    app.render(&mut fb, &mut assets);
    let perf = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgprofperf")
        .expect("the performance button the row wears")
        .clone();
    app.on_click(perf.x + perf.w / 2, perf.y + 10);
    assert_eq!(
        app.debug_profile,
        Some(true),
        "the profile field the resetToProfile putfield speaks"
    );
    assert!(
        app.fps_charts,
        "the FPS ALWAYS_ON word the flag's raise wears"
    );
    // THE PRESET MAP'S OWN ROWS (the PROFILES map's PERFORMANCE arm
    // the resetStatuses walk serves — the fps ALWAYS_ON + the
    // tps/gpu_utilization/memory/simple_performance_impactors rows
    // the IN_OVERLAY keep, the rest the sparse-map NEVER zeroes)
    assert_eq!(
        app.debug_entry_states[8],
        crate::screens::DebugEntryStatus::Always,
        "the performance preset the fps ALWAYS_ON wears"
    );
    assert_eq!(
        app.debug_entry_states[29],
        crate::screens::DebugEntryStatus::Overlay,
        "the performance preset the tps IN_OVERLAY wears"
    );
    assert_eq!(
        app.debug_entry_states[10],
        crate::screens::DebugEntryStatus::Overlay,
        "the performance preset the gpu row wears"
    );
    assert_eq!(
        app.debug_entry_states[20],
        crate::screens::DebugEntryStatus::Overlay,
        "the performance preset the memory row wears"
    );
    assert_eq!(
        app.debug_entry_states[25],
        crate::screens::DebugEntryStatus::Overlay,
        "the performance preset the impactors row wears"
    );
    assert_eq!(
        app.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY],
        crate::screens::DebugEntryStatus::Never,
        "the unpreset row the sparse-map reset zeroes"
    );
    assert_eq!(
        app.debug_entry_states[0],
        crate::screens::DebugEntryStatus::Never,
        "the crosshair row the performance preset retires"
    );
    // THE DEFAULT PROFILE'S OWN RETURN (the same walk the fresh
    // preset answers — the fps flag the reset keeps OFF, the field
    // the identity flips)
    app.render(&mut fb, &mut assets);
    let dflt = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgprofdflt")
        .expect("the default button the row wears")
        .clone();
    app.on_click(dflt.x + dflt.w / 2, dflt.y + 10);
    assert_eq!(
        app.debug_profile,
        Some(false),
        "the DEFAULT profile the identity flips"
    );
    assert!(!app.fps_charts, "the DEFAULT map none of the toggles wear");
    // THE INACTIVE BUTTON'S OWN WALL (the `!isUsingProfile` putfield —
    // the ACTIVE profile's button the disabled sprite keeps, the
    // hovered() the click never earns)
    app.render(&mut fb, &mut assets);
    let dflt = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgprofdflt")
        .expect("the default button the row wears")
        .clone();
    assert!(
        !dflt.active,
        "the active profile's button the inactive law wears"
    );
    let perf = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgprofperf")
        .expect("the performance button the row wears")
        .clone();
    assert!(
        perf.active,
        "the OTHER profile's button the clickable law wears"
    );
}

/// THE F3+V DUMP VERSION'S OWN WALK (javap 26.2
/// KeyboardHandler.handleDebugKeys's keyDebugDumpVersion lane — the
/// `matches` gate pc 1163, the header word pc 1174-1178, the nine
/// rows the VersionCommand.dumpVersion consumer speaks, the used flag
/// pc 1198-1199).
#[test]
fn the_dump_version_key_the_header_the_nine_rows_the_flag_wear() {
    let mut app = App::new(3);
    app.enter_world();
    // THE HEADER FIRST (pc 1174-1178 — the debugFeedbackTranslated
    // walk the cp[606] `debug.version.header` row sends, the yellow
    // channel the family's own law wears)
    app.on_dump_version_key();
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("\u{a7}estable = yes"),
        "the tail row the stable constant keeps"
    );
    assert!(
        app.debug_modifier_used,
        "the used flag the gate-passed walk banks"
    );
    app.on_debug_modifier_released();
    assert!(!app.debug_modifier_used, "the release the flag consumes");
    // THE FULL QUEUE (the dumpVersion pc 0-264 walk — the header the
    // first row keeps, the nine translatables the version.json's own
    // values wear: the id/name the client's build bank speaks, the
    // data/series the WorldVersion keeps, the protocol the
    // toHexString fold answers, the build_time the Date.toString
    // literal wears, the pack pair the '%d.%d' law banks, the stable
    // the STABLE word closes)
    let rows: Vec<&str> = app.chat_messages.iter().map(|(t, _)| t.as_str()).collect();
    let expected = [
        "\u{a7}eClient version info:",
        "\u{a7}eid = 26.2",
        "\u{a7}ename = 26.2",
        "\u{a7}edata = 4903",
        "\u{a7}eseries = main",
        "\u{a7}eprotocol = 776 (308)",
        "\u{a7}ebuild_time = Tue Jun 16 12:01:27 UTC 2026",
        "\u{a7}epack_resource = 88.0",
        "\u{a7}epack_data = 107.1",
        "\u{a7}estable = yes",
    ];
    let tail = &rows[rows.len() - expected.len()..];
    assert_eq!(
        tail,
        &expected[..],
        "the nine rows the dumpVersion walk keeps, the header the first wears"
    );
    // THE WIRE BANK'S OWN IDENTITY (the pack_claim's
    // PROTOCOL_VERSION the row's own arg speaks — the build_info
    // bank the same 776 keeps, the test's own equality the
    // part_4.rs:2048 walk already banks)
    assert_eq!(
        mc_protocol::generated::pack_claim::PROTOCOL_VERSION,
        crate::build_info::PROTOCOL_VERSION,
        "the wire bank the build bank's own id keeps"
    );
}

/// THE /VERSION COMMAND'S OWN SERVER ARM (javap VersionCommand.register
/// — the `literal("version")` node the Commands.<init> call-site walk
/// serves, the INTEGRATED selection the `false` flag wears, the
/// LEVEL_ALL permission the offline walk the no-gate law keeps; the
/// executes lambda the HEADER FIRST the plain sendSystemMessage channel
/// sends, the dumpVersion walk the nine translatables the shared jar's
/// WorldVersion values answer).
#[test]
fn the_version_command_the_server_header_the_plain_rows_wear() {
    let mut app = App::new(3);
    app.enter_world();
    let before = app.chat_messages.len();
    app.run_local_command("version");
    assert_eq!(
        app.chat_messages.len(),
        before + 10,
        "the header the first row keeps, the nine rows the dumpVersion walk adds"
    );
    // THE FULL QUEUE (the lambda$register$0 walk — the server header
    // the plain channel speaks, the §e the system-chat law never
    // wears, the nine rows the SAME values the F3+V's client walk
    // banks: the id/name the build bank speaks, the data/series the
    // WorldVersion keeps, the protocol the toHexString fold answers,
    // the build_time the Date.toString literal wears, the pack pair
    // the '%d.%d' law banks, the stable the STABLE constant closes)
    let rows: Vec<&str> = app.chat_messages.iter().map(|(t, _)| t.as_str()).collect();
    let expected = [
        "Server version info:",
        "id = 26.2",
        "name = 26.2",
        "data = 4903",
        "series = main",
        "protocol = 776 (308)",
        "build_time = Tue Jun 16 12:01:27 UTC 2026",
        "pack_resource = 88.0",
        "pack_data = 107.1",
        "stable = yes",
    ];
    let tail = &rows[rows.len() - expected.len()..];
    assert_eq!(
        tail,
        &expected[..],
        "the server header the first row wears, the plain rows the law keeps"
    );
    // THE FLAG THE CHORD NEVER BANKS (the command the debug
    // modifier's own store never touches — the F3 release gate the
    // lane never reads)
    assert!(
        !app.debug_modifier_used,
        "the command the debug flag never banks"
    );
}

/// THE ARG WORD'S OWN PARSE FAILURE (the brigadier literal walk —
/// the `version` node NO children the parser keeps, the trailing word
/// the unknown-command verdict answers, the RED the error channel
/// wears).
#[test]
fn the_version_command_the_arg_word_the_parse_failure_wears() {
    let mut app = App::new(3);
    app.enter_world();
    let before = app.chat_messages.len();
    app.run_local_command("version extra");
    assert_eq!(
        app.chat_messages.len(),
        before + 1,
        "the parse failure the single error row keeps"
    );
    assert_eq!(
        app.chat_messages.last().map(|(t, _)| t.as_str()),
        Some("\u{a7}cUnknown or incomplete command. See below for error"),
        "the unknown-command verdict the red channel wears"
    );
}

/// THE TOGGLE KEYMAPPING'S OWN FLIP LAW (javap 26.2 ToggleKeyMapping —
/// the `setDown(Z)` walk the four keyShift/keySprint/keyUse/keyAttack
/// instances wear: the option's FALSE the plain KeyMapping the
/// pass-through answers, the TRUE + down the `super.setDown(!isDown())`
/// the flip serves, the release the silence keeps, the latch the
/// physical state the lanes read).
#[test]
fn the_toggle_key_state_the_flip_law_the_latch_wears() {
    use crate::screens::{TOGGLE_ATTACK, TOGGLE_SPRINT};
    let mut app = App::new(3);
    // THE OPTION'S OFF (the plain KeyMapping the pass-through keeps —
    // the physical word the lanes verbatim read)
    assert!(
        app.toggle_key_state(TOGGLE_SPRINT, true, true),
        "the off option the press verbatim wears"
    );
    assert!(
        !app.toggle_key_state(TOGGLE_SPRINT, false, true),
        "the off option the release verbatim wears"
    );
    // THE OPTION'S ON (the toggleSprint:true row the store wears):
    // the press the world consumes the latch flips — the hold keeps,
    // the physical release the silence keeps
    app.toggle_sprint = true;
    assert!(
        app.toggle_key_state(TOGGLE_SPRINT, true, true),
        "the press the latch wears"
    );
    assert!(
        app.toggle_key_state(TOGGLE_SPRINT, false, true),
        "the release the latch keeps"
    );
    assert!(
        app.toggle_key_state(TOGGLE_SPRINT, false, true),
        "the held silence the same word answers"
    );
    // THE SECOND PRESS (the unlatch — the flip the fresh edge speaks)
    assert!(
        !app.toggle_key_state(TOGGLE_SPRINT, true, true),
        "the unlatch press the latch drops"
    );
    assert!(
        !app.toggle_key_state(TOGGLE_SPRINT, false, true),
        "the unlatched hold the false wears"
    );
    // THE THIRD PRESS (the re-latch — the cycle the same law speaks)
    assert!(
        app.toggle_key_state(TOGGLE_SPRINT, true, true),
        "the re-latch press the cycle wears"
    );
    // THE ATTACK'S OWN SEAT (the independent law — the sprint's latch
    // the attack's bank never rides)
    assert!(
        !app.toggle_key_state(TOGGLE_ATTACK, false, true),
        "the attack's own seat the fresh bank keeps"
    );
    app.toggle_attack = true;
    assert!(
        app.toggle_key_state(TOGGLE_ATTACK, true, true),
        "the attack's own press its own latch wears"
    );
    assert!(
        app.toggle_key_state(TOGGLE_SPRINT, false, true),
        "the sprint's latched seat the attack's flip never rides"
    );
}

/// THE SCREEN'S OWN GATE (the vanilla KeyboardHandler/MouseHandler
/// walks — the open screen the key events the screen dispatch answers,
/// the `KeyMapping.setDown` the world walk never reaches: the
/// `world_reads` FALSE the press the bank alone tracks, the flip the
/// law never speaks — the close-while-held the fresh edge never
/// re-wears, the release + the re-press the world the flip answers).
#[test]
fn the_toggle_key_state_the_screen_gate_the_press_refuses() {
    use crate::screens::TOGGLE_CROUCH;
    let mut app = App::new(3);
    app.toggle_crouch = true;
    // THE SCREEN-OPEN PRESS (the world_reads false — the bank tracks,
    // the flip refuses)
    assert!(
        !app.toggle_key_state(TOGGLE_CROUCH, true, false),
        "the screen-eaten press the latch never wears"
    );
    assert!(
        !app.toggle_key_state(TOGGLE_CROUCH, true, false),
        "the held frame the bank the same word keeps"
    );
    // THE CLOSE WHILE HELD (the world reads again — the stale edge the
    // bank's freshness kills: the held key the fresh press never re-speaks)
    assert!(
        !app.toggle_key_state(TOGGLE_CROUCH, true, true),
        "the close-while-held the fresh edge never re-wears"
    );
    // THE RELEASE + THE RE-PRESS (the world-consumed edge the flip serves)
    assert!(!app.toggle_key_state(TOGGLE_CROUCH, false, true));
    assert!(
        app.toggle_key_state(TOGGLE_CROUCH, true, true),
        "the re-press the world the flip wears"
    );
}

/// THE FIVE ROWS' OWN ROUND TRIP (the processOptions walk's
/// toggleCrouch@192/toggleSprint@205/toggleAttack@218/toggleUse@231/
/// sprintWindow@244 visit seats — the file rows the save walk keeps,
/// the restart the decode arms the store answers, the IntRange(0, 10)
/// the out-of-range the default keeps).
#[test]
fn the_toggle_options_the_file_round_trip_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_toggle_options_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(262);
    app.options_file = Some(path.clone());
    app.toggle_crouch = true;
    app.toggle_sprint = true;
    app.toggle_use = true;
    app.sprint_window = 3;
    app.options_file_save();
    let text = std::fs::read_to_string(&path).expect("the save wrote");
    assert!(
        text.contains("toggleCrouch:true\n"),
        "the crouch row the true wears"
    );
    assert!(
        text.contains("toggleSprint:true\n"),
        "the sprint row the true wears"
    );
    assert!(
        text.contains("toggleAttack:false\n"),
        "the attack's default row the false wears"
    );
    assert!(
        text.contains("toggleUse:true\n"),
        "the use row the true wears"
    );
    assert!(
        text.contains("sprintWindow:3\n"),
        "the window row the option's own value wears"
    );
    // THE RESTART'S OWN DECODE (the fresh boot the file's truth wears)
    let mut re = App::new(262);
    re.options_file = Some(path.clone());
    re.options_file_load();
    assert!(re.toggle_crouch, "the crouch row the decode keeps");
    assert!(re.toggle_sprint, "the sprint row the decode keeps");
    assert!(!re.toggle_attack, "the attack row the default keeps");
    assert!(re.toggle_use, "the use row the decode keeps");
    assert_eq!(re.sprint_window, 3, "the window row the decode keeps");
    // THE OUT-OF-RANGE'S OWN DEFAULT (the IntRange(0, 10) the decode
    // refuses — the malformed row the default keeps)
    std::fs::write(&path, "sprintWindow:11\n").expect("the range row wrote");
    let mut bad = App::new(262);
    bad.options_file = Some(path.clone());
    bad.options_file_load();
    assert_eq!(
        bad.sprint_window,
        crate::options_file::DEFAULT_SPRINT_WINDOW,
        "the out-of-range row the default keeps"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE SPRINT WINDOW'S OWN OPTION SEAT (the LocalPlayer.aiStep pcs
/// 405-425 walk — the W-press edge the expired window the option's
/// own value the re-arm writes: the law's `window_ticks` parameter
/// the store's own seat reads, the 3 the slider's own value speaks).
#[test]
fn the_sprint_window_option_the_rearm_wears() {
    let mut b = crate::body::PlayerBody::new(0.0, 64.0, 0.0, 0.0, 0.0);
    let mut trigger = 0;
    let mut last = false;
    // THE SLIDER'S OWN 3 (the sprintWindow:3 row the store wears —
    // the W-press edge the option's value the arm writes)
    b.forward = 1.0;
    App::sprint_law_tick(&mut b, &mut trigger, &mut last, false, true, false, 3);
    assert_eq!(trigger, 3, "the option's own value the arm writes");
    // THE DEFAULT'S OWN 7 (the missing row the DEFAULT constant keeps —
    // the fresh press the edge re-speaks)
    trigger = 0;
    last = false;
    App::sprint_law_tick(&mut b, &mut trigger, &mut last, false, true, false, 7);
    assert_eq!(
        trigger,
        crate::options_file::DEFAULT_SPRINT_WINDOW,
        "the default seat the 7 speaks"
    );
}

/// THE CONTROLS SCREEN'S OWN FOUR ROWS (the 26.2 ControlsScreen
/// `options(Options)` array's indices 0-3 — the toggle rows the pair
/// layout hosts, the captions the lambda$new$79..82 words wear: the
/// FALSE the `options.key.hold` "Hold" speaks, the TRUE the
/// `options.key.toggle` "Toggle" answers; the click the value's own
/// flip serves, the store the next render's caption reads).
#[test]
fn the_controls_screen_the_four_rows_the_toggle_captions_wear() {
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/container/inventory.png").exists());
    let Some(root) = root else { return };
    let mut app = App::new(262);
    app.open(ScreenKind::Controls);
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 240);
    // THE FIRST RENDER'S OWN ROWS (the defaults FALSE the four rows
    // keep — the label the KeyMapping name's translatable speaks, the
    // Hold word the value's own lambda wears)
    app.render(&mut fb, &mut assets);
    let seat = |app: &App, id: &str| -> (String, i32, i32) {
        app.buttons
            .iter()
            .find(|b| b.id == id)
            .map(|b| (b.label.clone(), b.x, b.y))
            .expect(id)
    };
    let (crouch_label, cx, cy) = seat(&app, "opt_ctl_crouch");
    assert_eq!(
        crouch_label, "Sneak: Hold",
        "the crouch row the Hold word wears"
    );
    let (sprint_label, sx, sy) = seat(&app, "opt_ctl_sprint");
    assert_eq!(
        sprint_label, "Sprint: Hold",
        "the sprint row the Hold word wears"
    );
    assert_eq!(cy, sy, "the first row the pair's own y keeps");
    assert!(sx > cx, "the sprint row the pair's right seat wears");
    assert_eq!(
        seat(&app, "opt_ctl_attack").0,
        "Attack/Destroy: Hold",
        "the attack row the label's own word wears"
    );
    assert_eq!(
        seat(&app, "opt_ctl_use").0,
        "Use Item/Place Block: Hold",
        "the use row the label's own word wears"
    );
    // THE CLICK'S OWN CYCLE (the CycleButton's press walk — the flip
    // the store wears, the caption the Toggle word the next render
    // speaks)
    app.on_click(sx + 2, sy + 2);
    assert!(app.toggle_sprint, "the click the sprint option flips");
    app.render(&mut fb, &mut assets);
    let (label, _, _) = seat(&app, "opt_ctl_sprint");
    assert_eq!(
        label, "Sprint: Toggle",
        "the flipped row the Toggle word wears"
    );
    // THE HOLD WORD'S OWN RETURN (the second click the cycle answers)
    app.on_click(sx + 2, sy + 2);
    assert!(!app.toggle_sprint, "the second click the cycle returns");
}

/// THE CONTROLS SCREEN'S OWN AUTOJUMP ROW (the 26.2 ControlsScreen
/// `options(Options)` array's index-4 seat — the (4,5) pair's LEFT the
/// addSmall two-at-a-time law keeps, the generic boolean's own caption
/// the `options.on`/`options.off` pair the `options.autoJump` label
/// wears; the click the value's own flip serves, the caption the next
/// render's rebuild reads, the ms/keybinds pair the AFTER seat the
/// pitch keeps).
#[test]
fn the_controls_screen_the_autojump_row_the_generic_caption_wears() {
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/container/inventory.png").exists());
    let Some(root) = root else { return };
    let mut app = App::new(262);
    app.open(ScreenKind::Controls);
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 240);
    // THE FIRST RENDER'S OWN ROW (the default FALSE the OFF word the
    // generic pair keeps)
    app.render(&mut fb, &mut assets);
    let seat = |app: &App, id: &str| -> (String, i32, i32) {
        app.buttons
            .iter()
            .find(|b| b.id == id)
            .map(|b| (b.label.clone(), b.x, b.y))
            .expect(id)
    };
    let (label, ax, ay) = seat(&app, "opt_ctl_autojump");
    assert_eq!(label, "Auto-Jump: OFF", "the OFF word the default wears");
    // THE SEAT'S OWN GRID (the (4,5) pair's LEFT the toggle rows'
    // AFTER the 24-pitch keeps, the ms pair the array tail's AFTER
    // seat wears)
    let (_, cx, cy) = seat(&app, "opt_ctl_crouch");
    assert_eq!(ax, cx, "the left seat the pair's own x keeps");
    assert_eq!(ay, cy + 48, "the third row the 24-pitch grid keeps");
    let (_, mx, my) = seat(&app, "mousesettings");
    assert_eq!(mx, ax, "the ms pair the left seat's own x keeps");
    assert_eq!(my, ay + 48, "the ms pair the array tail's AFTER seat wears");
    // THE CLICK'S OWN FLIP (the boolean row's own press walk — the
    // store the flip wears, the caption the ON word the next render
    // speaks)
    app.on_click(ax + 2, ay + 2);
    assert!(app.auto_jump, "the click the autoJump option flips");
    app.render(&mut fb, &mut assets);
    let (label, _, _) = seat(&app, "opt_ctl_autojump");
    assert_eq!(label, "Auto-Jump: ON", "the flipped row the ON word wears");
    // THE OFF WORD'S OWN RETURN (the second click the cycle answers)
    app.on_click(ax + 2, ay + 2);
    assert!(!app.auto_jump, "the second click the cycle returns");
}

/// THE AUTOJUMP ROW'S OWN FILE WALK (the processOptions pc 5-13 the
/// FIRST row the visit ladder keeps — the `autoJump:true` row the boot
/// load wears, the save the row's own writer serves, the missing row
/// the default FALSE the ctor's iconst_0 keeps, the malformed the
/// default answers).
#[test]
fn the_autojump_row_the_file_round_trip_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_autojump_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    // THE DEFAULT (the ctor's iconst_0 the FALSE keeps)
    let mut app = App::new(262);
    assert!(!app.auto_jump, "the default the ctor banks");
    // THE FILE'S TRUTH (the autoJump:true row the boot load wears —
    // the store the walk answers, the malformed the default keeps)
    std::fs::write(
        &path,
        "version:4903\nautoJump:true\nmouseSensitivity:0.5\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\n",
    )
    .expect("the autojump file writes");
    app.options_file = Some(path.clone());
    app.options_file_load();
    assert!(app.auto_jump, "the true row the boot load wears");
    // THE SAVE'S OWN ROW (the visit-order seat the writer keeps — the
    // pauseOnLostFocus/toggleCrouch seam the row wears)
    app.options_file_save();
    let text = std::fs::read_to_string(&path).expect("the save the file writes");
    assert!(
        text.contains(
            "pauseOnLostFocus:true\nautoJump:true\noperatorItemsTab:false\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:"
        ),
        "the save the visit-order seat wears: {}",
        text
    );
    // THE MISSING ROW (the default FALSE the decode's skip law keeps —
    // the boot latch the fresh load the reset wears)
    std::fs::write(&path, "version:4903\nmouseSensitivity:0.5\n").expect("the bare file writes");
    app.options_file_loaded = false;
    app.options_file_load();
    assert!(!app.auto_jump, "the missing row the default keeps");
    // THE MALFORMED ROW (the non-boolean the default answers)
    std::fs::write(&path, "version:4903\nautoJump:yes\nmouseSensitivity:0.5\n")
        .expect("the malformed file writes");
    app.options_file_loaded = false;
    app.options_file_load();
    assert!(!app.auto_jump, "the malformed row the default answers");
    let _ = std::fs::remove_file(&path);
}

/// THE CONTROLS SCREEN'S OWN ARRAY TAIL (the 26.2 ControlsScreen
/// `options(Options)` array's indices 5-6 — the (4,5) pair's RIGHT the
/// sprintWindow slider the IntRange(0, 10) track wears (the default 7
/// the ctor's bipush pc 2325 speaks, the caption the
/// `options.generic_value` "%s: %s" fold the label wears, the
/// press/drag the IntRange's own 1-tick seat the cursor's x answers,
/// the release the disarm speaks), the operatorItemsTab the index-6
/// the tail's own LEFT seat hosts (the generic OFF/ON caption the
/// click-cycle law wears), the ms/keybinds pair the array's AFTER
/// seat the pitch keeps).
#[test]
fn the_controls_screen_the_array_tail_the_slider_and_operator_row_wear() {
    let root = [
        "assets-cache/jar",
        "../assets-cache/jar",
        "../../assets-cache/jar",
    ]
    .iter()
    .map(std::path::Path::new)
    .find(|p| p.join("gui/container/inventory.png").exists());
    let Some(root) = root else { return };
    let mut app = App::new(262);
    app.open(ScreenKind::Controls);
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(320, 240);
    // THE FIRST RENDER'S OWN ROWS (the default 7 the caption wears,
    // the OFF word the operator row keeps — the (4,5) pair the RIGHT
    // seat the slider's own x answers, the tail row the pair's AFTER
    // seat the pitch keeps)
    app.render(&mut fb, &mut assets);
    let seat = |app: &App, id: &str| -> (String, i32, i32) {
        app.buttons
            .iter()
            .find(|b| b.id == id)
            .map(|b| (b.label.clone(), b.x, b.y))
            .expect(id)
    };
    let (sw_label, swx, swy) = seat(&app, "opt_ctl_sprint_window");
    assert_eq!(
        sw_label, "Sprint Window: 7",
        "the default the caption wears"
    );
    let (_, ax, ay) = seat(&app, "opt_ctl_autojump");
    assert_eq!(swx, ax + 148, "the right seat the pair's own x keeps");
    assert_eq!(swy, ay, "the slider the pair's own row wears");
    let sw_val = app
        .buttons
        .iter()
        .find(|b| b.id == "opt_ctl_sprint_window")
        .and_then(|b| b.value);
    assert_eq!(sw_val, Some(0.7), "the default 7 the unit's own seat wears");
    let (op_label, ox, oy) = seat(&app, "opt_ctl_operator_tab");
    assert_eq!(
        op_label, "Operator Items Tab: OFF",
        "the OFF word the default wears"
    );
    assert_eq!(ox, ax, "the tail row the left seat's own x keeps");
    assert_eq!(oy, ay + 24, "the tail row the pair's AFTER seat wears");
    // THE PAIR'S OWN AFTER SEAT (the ms/keybinds the tail row's AFTER
    // the 24-pitch keeps)
    let (_, mx, my) = seat(&app, "mousesettings");
    assert_eq!(mx, ox, "the ms pair the left seat's own x keeps");
    assert_eq!(my, oy + 24, "the ms pair the tail row's AFTER seat wears");
    // THE SLIDER'S OWN PRESS (the IntRange's own seat the cursor's x
    // answers — the track's midpoint the 5 the round speaks, the flag
    // the held drag arms)
    app.on_press(swx + 70, swy + 2);
    assert_eq!(app.sprint_window, 5, "the midpoint press the 5 speaks");
    assert!(app.sprint_window_drag, "the press the drag arms");
    // THE HELD DRAG'S OWN SEAT (the cursor's x the value re-writes —
    // the left end the 0 the floor speaks, the right end the 10 the
    // ceiling answers)
    app.on_sprint_window_slider_drag(swx + 4);
    assert_eq!(app.sprint_window, 0, "the left end the floor speaks");
    app.on_sprint_window_slider_drag(swx + 136);
    assert_eq!(app.sprint_window, 10, "the right end the ceiling answers");
    // THE RELEASE'S OWN DISARM (the flag the unconditional false
    // clears, the caption the next render's rebuild reads)
    app.on_left_release();
    assert!(!app.sprint_window_drag, "the release the disarm speaks");
    app.render(&mut fb, &mut assets);
    let (sw_label, _, _) = seat(&app, "opt_ctl_sprint_window");
    assert_eq!(
        sw_label, "Sprint Window: 10",
        "the caption the value's own word wears"
    );
    // THE OPERATOR ROW'S OWN FLIP (the boolean row's own press walk —
    // the click the value's own flip answers, the caption the ON word
    // the next render speaks)
    app.on_click(ox + 2, oy + 2);
    assert!(
        app.operator_items_tab,
        "the click the operator option flips"
    );
    app.render(&mut fb, &mut assets);
    let (op_label, _, _) = seat(&app, "opt_ctl_operator_tab");
    assert_eq!(
        op_label, "Operator Items Tab: ON",
        "the flipped row the ON word wears"
    );
    // THE OFF WORD'S OWN RETURN (the second click the cycle answers)
    app.on_click(ox + 2, oy + 2);
    assert!(
        !app.operator_items_tab,
        "the second click the cycle returns"
    );
}

/// THE OPERATOR TAB ROW'S OWN FILE WALK (the processOptions pc 31-39
/// the THIRD visit row the ladder keeps — the `operatorItemsTab:true`
/// row the boot load wears, the save the row's own writer serves [the
/// autoJump/toggleCrouch seam the visit-order seat wears], the missing
/// row the default FALSE the ctor's iconst_0 keeps, the malformed the
/// default answers).
#[test]
fn the_operator_tab_row_the_file_round_trip_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_optab_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    // THE DEFAULT (the ctor's iconst_0 the FALSE keeps)
    let mut app = App::new(262);
    assert!(!app.operator_items_tab, "the default the ctor banks");
    // THE FILE'S TRUTH (the operatorItemsTab:true row the boot load
    // wears — the store the walk answers)
    std::fs::write(
        &path,
        "version:4903\noperatorItemsTab:true\nmouseSensitivity:0.5\nmouseWheelSensitivity:1.0\nrawMouseInput:true\nallowCursorChanges:true\nfov:70\nsoundCategory_master:1.0\n",
    )
    .expect("the operator tab file writes");
    app.options_file = Some(path.clone());
    app.options_file_load();
    assert!(app.operator_items_tab, "the true row the boot load wears");
    // THE SAVE'S OWN ROW (the visit-order seat the writer keeps — the
    // autoJump/toggleCrouch seam the row's own word wears)
    app.options_file_save();
    let text = std::fs::read_to_string(&path).expect("the save the file writes");
    assert!(
        text.contains("autoJump:false\noperatorItemsTab:true\ndiscrete_mouse_scroll:false\ninvertXMouse:false\ninvertYMouse:false\ntoggleCrouch:"),
        "the save the visit-order seat wears: {}",
        text
    );
    // THE MISSING ROW (the default FALSE the decode's skip law keeps —
    // the boot latch the fresh load the reset wears)
    std::fs::write(&path, "version:4903\nmouseSensitivity:0.5\n").expect("the bare file writes");
    app.options_file_loaded = false;
    app.options_file_load();
    assert!(!app.operator_items_tab, "the missing row the default keeps");
    // THE MALFORMED ROW (the non-boolean the default answers)
    std::fs::write(
        &path,
        "version:4903\noperatorItemsTab:yes\nmouseSensitivity:0.5\n",
    )
    .expect("the malformed file writes");
    app.options_file_loaded = false;
    app.options_file_load();
    assert!(
        !app.operator_items_tab,
        "the malformed row the default answers"
    );
    let _ = std::fs::remove_file(&path);
}

/// THE SCREEN DANCE'S OWN RELEASE WALK (the vanilla Gui.setScreen's
/// pc 215 `KeyMapping.releaseAll()` ride — the ToggleKeyMapping's own
/// release override the flag dance banks: the toggle ON the
/// `(isDown() || releasedByScreenWhenDown) -> flag = true` arm the
/// release-while-down records, the `reset()` the unlatch speaks — the
/// latched sprint the inventory's open the release + flag wears).
#[test]
fn the_screen_open_the_latched_toggles_the_release_wears() {
    use crate::screens::{TOGGLE_SPRINT, TOGGLE_USE};
    let mut app = App::new(3);
    app.toggle_sprint = true;
    app.toggle_use = true;
    // THE LATCH'S OWN PRE-STATE (the world press the flip wears)
    assert!(
        app.toggle_key_state(TOGGLE_SPRINT, true, true),
        "the press the latch wears"
    );
    assert!(
        app.toggle_key_state(TOGGLE_SPRINT, false, true),
        "the release the latch keeps"
    );
    // THE SCREEN'S OWN OPEN (the releaseAll ride — the latch the
    // reset() clears, the flag the release-while-down banks)
    app.open(ScreenKind::Inventory);
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the screen open the latch clears"
    );
    // THE RELEASE'S OWN VERDICT (the unlatched state the world reads —
    // the physical law the screen-eaten frames the bank tracks)
    assert!(
        !app.toggle_key_state(TOGGLE_SPRINT, false, false),
        "the screen frames the unlatched word keeps"
    );
    // THE USE'S OWN RELEASE (the same open the use latch clears — the
    // flag the same release law banks)
    assert!(
        app.toggle_key_state(TOGGLE_USE, true, true),
        "the use press the latch wears"
    );
    app.open(ScreenKind::Options);
    assert!(
        !app.toggle_latched(TOGGLE_USE),
        "the second open the use latch clears"
    );
}

/// THE SCREEN DANCE'S OWN RESTORE WALK (the vanilla Gui.setScreen's
/// pc 265 `restoreToggleStatesOnScreenClosed()` ride — the verdict's
/// four gates the AND keeps: the shouldRestore flag the ctor's final
/// boolean banks (the keyUse iconst_0 FALSE), the toggle option's ON,
/// the key TYPE the KEYSYM compare keeps (the mouse-bind the restore
/// refuses), the releasedByScreenWhenDown flag — THE FLAG THE
/// ALWAYS-CLEAR the putfield-0 walk wears, the re-latch the
/// setDown(true) flip law serves).
#[test]
fn the_screen_close_the_restore_trio_the_type_gate_wears() {
    use crate::screens::{TOGGLE_ATTACK, TOGGLE_SPRINT, TOGGLE_USE};
    let mut app = App::new(3);
    app.toggle_sprint = true;
    app.toggle_attack = true;
    app.toggle_use = true;
    // THE KEYBOARD REBINDS (the type gate's own seats — the attack
    // and the use the keys the rebind arms, the sprint's keyboard
    // default the LeftCtrl row wears)
    app.key_binds[crate::options_file::key_binds::ATTACK] = Some(
        crate::options_file::key_binds::Bind::Keyboard(minifb::Key::R),
    );
    app.key_binds[crate::options_file::key_binds::USE] = Some(
        crate::options_file::key_binds::Bind::Keyboard(minifb::Key::T),
    );
    // THE THREE LATCHES (the world presses the flips wear)
    assert!(app.toggle_key_state(TOGGLE_SPRINT, true, true));
    assert!(app.toggle_key_state(TOGGLE_ATTACK, true, true));
    assert!(app.toggle_key_state(TOGGLE_USE, true, true));
    // THE SCREEN'S OPEN + CLOSE (the release + the restore walk — the
    // shouldRestore trio the keyboard binds the re-latch wears, the
    // use's own FALSE the restore refuses)
    app.open(ScreenKind::Inventory);
    assert!(!app.toggle_latched(TOGGLE_SPRINT));
    assert!(!app.toggle_latched(TOGGLE_ATTACK));
    assert!(!app.toggle_latched(TOGGLE_USE));
    app.open(ScreenKind::World);
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the keyboard sprint the restore re-latches"
    );
    assert!(
        app.toggle_latched(TOGGLE_ATTACK),
        "the keyboard-rebound attack the restore re-latches"
    );
    assert!(
        !app.toggle_latched(TOGGLE_USE),
        "the keyUse's shouldRestore FALSE the restore refuses"
    );
    // THE MOUSE-TYPE'S OWN GATE (the attack's default mouse bind the
    // restore refuses — the flagged release the world the latch keeps)
    let mut mouse_app = App::new(3);
    mouse_app.toggle_attack = true;
    assert!(mouse_app.toggle_key_state(TOGGLE_ATTACK, true, true));
    mouse_app.open(ScreenKind::Inventory);
    assert!(!mouse_app.toggle_latched(TOGGLE_ATTACK));
    mouse_app.open(ScreenKind::World);
    assert!(
        !mouse_app.toggle_latched(TOGGLE_ATTACK),
        "the mouse-type bind the restore refuses"
    );
    // THE FLAG'S OWN ALWAYS-CLEAR (the unlatched release the fresh
    // flag never arms — the open-close the restore never re-speaks;
    // the LATCHED cycle the opposite walks: the restore the re-latch
    // wears, the next open the flag re-arms the same law the round-trip
    // speaks)
    app.toggle_key_state(TOGGLE_SPRINT, false, true); // the held press the bank lifts
    app.toggle_key_state(TOGGLE_SPRINT, true, true); // the unlatch press
    app.toggle_key_state(TOGGLE_SPRINT, false, true);
    app.open(ScreenKind::Inventory);
    app.open(ScreenKind::World);
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the unlatched release the restore never re-speaks"
    );
    // THE LATCHED ROUND-TRIP (the re-latched state the second cycle
    // the same dance wears)
    assert!(app.toggle_key_state(TOGGLE_SPRINT, true, true));
    app.open(ScreenKind::Inventory);
    app.open(ScreenKind::World);
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the latched round-trip the restore re-speaks"
    );
}

/// THE DEATH'S OWN RESET WALK (the vanilla LocalPlayer.respawn's pc 17
/// `KeyMapping.resetToggleKeys()` ride — the ALL map iterates, the
/// ToggleKeyMapping instances the `reset()` the setDown(false)
/// unlatch speaks: the death the latches AND the screen-release flags
/// both clear, the respawn the fresh keys the no-restore law keeps).
#[test]
fn the_respawn_the_toggle_keys_the_reset_wears() {
    use crate::screens::TOGGLE_SPRINT;
    let mut app = App::new(3);
    app.toggle_sprint = true;
    assert!(app.toggle_key_state(TOGGLE_SPRINT, true, true));
    // THE SCREEN'S OWN FLAG (the release ride the bank arms)
    app.open(ScreenKind::Inventory);
    // THE DEATH'S OWN RESET (the respawn the banks clear — the
    // no-restore law the close the fresh banks never re-latches)
    app.respawn();
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the death the latch clears"
    );
    app.open(ScreenKind::World);
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the respawn's cleared flag the restore never re-latches"
    );
}

/// THE PLAIN KEYS' OWN SCREEN RELEASE (the vanilla Gui.setScreen's
/// pc 215 `KeyMapping.releaseAll()` walk the non-toggle instances
/// wear — the inventory open the body's raw banks the all-false
/// record clears: the movement fields zero, the sprint key bank
/// false, the dig hold cleared, the wire the change-law the
/// ServerboundPlayerInputPacket release speaks, the standing-still
/// open the unchanged record never re-sends).
#[test]
fn the_screen_open_the_plain_keys_release_all_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE HELD WORDS (the walk + the jump + the crouch + the sprint
    // bank + the dig the poll's last feed banks — the inventory the
    // release clears)
    if let Some(b) = app.body.as_mut() {
        b.forward = 1.0;
        b.left = -1.0;
        b.jumping = true;
        b.want_crouch = true;
    }
    app.sprint_key_down = true;
    app.dig_held = true;
    app.last_sent_input = mc_protocol::InputFlags {
        forward: true,
        ..Default::default()
    };
    // THE WIRE SINK (the release packet the change-law speaks)
    let (tx, rx) = std::sync::mpsc::channel();
    app.attach_live_clicks(tx);
    app.open(ScreenKind::Inventory);
    // THE BODY'S OWN CLEAR (the all-false record the raw banks wear —
    // the sprinting STATE the tick law alone owns [the frozen gate
    // the stop lane the re-entry speaks], the release never touches)
    let b = app.body.as_ref().unwrap();
    assert_eq!(b.forward, 0.0, "the release the forward zeroes");
    assert_eq!(b.left, 0.0, "the release the strafe zeroes");
    assert!(!b.jumping, "the release the jump clears");
    assert!(!b.want_crouch, "the release the crouch clears");
    assert!(
        !app.sprint_key_down,
        "the release the sprint key bank clears"
    );
    assert!(!app.dig_held, "the release the dig hold clears");
    // THE WIRE'S OWN RELEASE (the change-detector the all-false delta
    // answers — one packet, the standing-still open never re-sends)
    assert_eq!(
        rx.try_recv().unwrap(),
        crate::world_view::ContainerCommand::PlayerInput {
            forward: false,
            backward: false,
            left: false,
            right: false,
            jump: false,
            shift: false,
            sprint: false,
        },
        "the release the all-false input packet speaks"
    );
    assert!(
        rx.try_recv().is_err(),
        "the release the one packet alone wears"
    );
    // THE STANDING-STILL OPEN'S OWN SILENCE (the record the release
    // matched — the change-detector the packet never mints)
    let (tx2, rx2) = std::sync::mpsc::channel();
    app.attach_live_clicks(tx2);
    app.open(ScreenKind::Chest);
    assert!(
        rx2.try_recv().is_err(),
        "the unchanged record the release never re-sends"
    );
}

/// THE RELEASE ARMS' OWN MAP + THE WORLD ENTRY'S OWN KEEP (the
/// vanilla Gui.setScreen walk the screen-takeover arms the releaseAll
/// wear — the CHATSCREEN's open the T// lanes speak, the PAUSESCREEN
/// the ESC + the focus-loss arms wear, the unpause the release never
/// re-rides, the world ENTRY [the close arm] the plain keys the
/// poll's own setAll lane re-reads — the clear never speaks).
#[test]
fn the_release_arms_the_chat_and_pause_the_world_entry_keeps() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE CHAT ARM (the T-open the releaseAll speaks — the held walk
    // the bank clears, the close the world's return the poll lane
    // owns, the double-clear never rides)
    if let Some(b) = app.body.as_mut() {
        b.forward = 1.0;
    }
    app.dig_held = true;
    app.on_chat_key();
    assert!(app.chat_open, "the chat the open keeps");
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.0,
        "the chat open the held walk clears"
    );
    assert!(!app.dig_held, "the chat open the dig hold clears");
    app.close_chat();
    assert!(!app.chat_open, "the close the world's return speaks");
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.0,
        "the close the banks the cleared word keeps [the poll re-feeds]"
    );
    // THE COMMAND ARM (the '/'-open the same releaseAll wears)
    if let Some(b) = app.body.as_mut() {
        b.forward = 1.0;
    }
    app.on_command_key();
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.0,
        "the command open the held walk clears"
    );
    app.close_chat();
    // THE ESC PAUSE ARM (the pause side the release speaks, the
    // unpause the re-clear never rides)
    if let Some(b) = app.body.as_mut() {
        b.forward = 1.0;
    }
    app.on_escape();
    assert!(app.paused, "the escape the pause arms");
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.0,
        "the pause the held walk clears"
    );
    app.on_escape();
    assert!(!app.paused, "the second escape the unpause arms");
    // THE WORLD ENTRY'S OWN KEEP (the close arm the release never
    // wears — the restore lane the toggles alone serves, the plain
    // keys the poll's own setAll re-read)
    if let Some(b) = app.body.as_mut() {
        b.forward = 0.7;
    }
    app.open(ScreenKind::World);
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.7,
        "the world entry the banked word keeps"
    );
}

/// THE CHAT LANES' OWN TOGGLE DANCE (the vanilla Gui.setScreen walk
/// the chat open the releaseAll's ALL-map iteration wears — the
/// ToggleKeyMapping instances the release() override serves: the
/// latched seats the reset() unlatches, the releasedByScreenWhenDown
/// flag the release-while-down banks; the SUBMIT's close the
/// setScreen(null) null arm the restoreToggleStatesOnScreenClosed
/// rides — the keyboard-bound sprint the re-latch wears, the keyUse's
/// shouldRestore FALSE the restore refuses, the flags the always-clear
/// law consumes; the empty-submit early-return the restore never
/// skips).
#[test]
fn the_chat_open_the_latched_toggles_the_release_wears() {
    use crate::screens::{TOGGLE_SPRINT, TOGGLE_USE};
    let mut app = App::new(3);
    app.toggle_sprint = true;
    app.toggle_use = true;
    // THE LATCHES (the world presses the flips wear — the release the
    // latched words keep)
    assert!(app.toggle_key_state(TOGGLE_SPRINT, true, true));
    assert!(app.toggle_key_state(TOGGLE_SPRINT, false, true));
    assert!(app.toggle_key_state(TOGGLE_USE, true, true));
    assert!(app.toggle_key_state(TOGGLE_USE, false, true));
    // THE CHAT'S OWN OPEN (the T lane the releaseAll speaks — the
    // toggle override the latch clears, the flag the release banks)
    app.on_chat_key();
    assert!(app.chat_open, "the chat the open keeps");
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the chat open the sprint latch clears"
    );
    assert!(
        !app.toggle_latched(TOGGLE_USE),
        "the chat open the use latch clears"
    );
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the chat open the flag the release banks"
    );
    // THE SCREEN FRAMES (the world_reads false the flip never speaks —
    // the bank the physical stream keeps)
    assert!(
        !app.toggle_key_state(TOGGLE_SPRINT, false, false),
        "the chat frames the unlatched word keeps"
    );
    // THE SUBMIT'S OWN CLOSE (the DONE verdict the setScreen(null) the
    // restore rides — the empty input the early-return the restore
    // never skips; the keyboard-bound sprint the re-latch wears, the
    // keyUse's shouldRestore FALSE the restore refuses)
    app.on_chat_submit();
    assert!(!app.chat_open, "the submit the close speaks");
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the submit close the sprint re-latches"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the restore the flag the always-clear consumes"
    );
    assert!(
        !app.toggle_latched(TOGGLE_USE),
        "the keyUse the shouldRestore FALSE the restore refuses"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_USE],
        "the refused verdict the flag the same clear wears"
    );
}

/// THE CHAT CLOSE LANES' OWN RESTORE MAP (the ESC close the
/// close_chat_intentional verdict the same null arm rides — the
/// re-latch the flagged seats wear; the FIRST-ESC draft arm the screen
/// the open keeps, the restore never rides; the TAKEOVER chain the
/// interrupted close the release arms the flag keeps — the world's
/// return the restore re-latch the chain completes).
#[test]
fn the_chat_close_lanes_the_restore_map_the_takeover_keeps() {
    use crate::screens::TOGGLE_SPRINT;
    let mut app = App::new(3);
    app.toggle_sprint = true;
    // THE ESC CLOSE LANE (the INTENTIONAL verdict the restore rides)
    assert!(app.toggle_key_state(TOGGLE_SPRINT, true, true));
    assert!(app.toggle_key_state(TOGGLE_SPRINT, false, true));
    app.on_chat_key();
    assert!(!app.toggle_latched(TOGGLE_SPRINT));
    app.on_chat_escape();
    assert!(!app.chat_open, "the esc close the chat folds");
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the esc close the sprint re-latches"
    );
    // THE FIRST-ESC DRAFT ARM (the untouched draft the clear wears —
    // the screen the open keeps, the restore the close rides alone)
    app.on_chat_key();
    app.chat_input_is_draft = true;
    assert!(app.on_chat_escape(), "the draft esc the consumption keeps");
    assert!(app.chat_open, "the draft arm the screen the open keeps");
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the draft arm the restore never rides"
    );
    app.chat_input_is_draft = false;
    app.on_chat_escape();
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the second esc the close the re-latch wears"
    );
    // THE TAKEOVER CHAIN (the chest the interrupted close speaks — the
    // release arms the banked flag the open keeps, the world's return
    // the restore the chain completes)
    let mut app2 = App::new(3);
    app2.toggle_sprint = true;
    assert!(app2.toggle_key_state(TOGGLE_SPRINT, true, true));
    app2.on_chat_key();
    assert!(!app2.toggle_latched(TOGGLE_SPRINT));
    app2.open(ScreenKind::Chest);
    assert!(
        app2.toggle_released_by_screen[TOGGLE_SPRINT],
        "the takeover's release the banked flag keeps"
    );
    assert!(
        !app2.toggle_latched(TOGGLE_SPRINT),
        "the takeover the unlatched word keeps"
    );
    app2.open(ScreenKind::World);
    assert!(
        app2.toggle_latched(TOGGLE_SPRINT),
        "the world's return the restore the chain completes"
    );
}

/// THE PAUSE LANES' OWN TOGGLE DANCE (the vanilla PauseScreen the
/// setScreen arms the pause lanes wear — the ESC pause the OPEN arm
/// the releaseAll's ALL-map iteration speaks: the latched seats the
/// flag the release banks + the latch clears, the plain walk the held
/// banks zeroes; the ESC unpause the setScreen(null) null arm the
/// restoreToggleStatesOnScreenClosed rides — the keyboard-bound
/// sprint the re-latch wears, the keyUse's shouldRestore FALSE the
/// restore refuses, the flags the always-clear law consumes, the
/// plain keys the poll's own setAll re-read the re-clear never wears).
#[test]
fn the_pause_arms_the_latched_toggles_the_release_banks() {
    use crate::screens::{TOGGLE_SPRINT, TOGGLE_USE};
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.toggle_sprint = true;
    app.toggle_use = true;
    // THE LATCHES (the world's own press+release pairs wear)
    assert!(app.toggle_key_state(TOGGLE_SPRINT, true, true));
    assert!(app.toggle_key_state(TOGGLE_SPRINT, false, true));
    assert!(app.toggle_key_state(TOGGLE_USE, true, true));
    assert!(app.toggle_key_state(TOGGLE_USE, false, true));
    // THE HELD WALK (the plain banks the pause arm clears)
    if let Some(b) = app.body.as_mut() {
        b.forward = 1.0;
    }
    app.dig_held = true;
    // THE ESC PAUSE ARM (the toggle release the plain walk's BEFORE
    // seat wears — the pc 215 order the open() dance's own shape keeps)
    app.on_escape();
    assert!(app.paused, "the escape the pause arms");
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the pause the sprint latch clears"
    );
    assert!(
        !app.toggle_latched(TOGGLE_USE),
        "the pause the use latch the same walk clears"
    );
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the pause the flag the release banks"
    );
    assert!(
        app.toggle_released_by_screen[TOGGLE_USE],
        "the pause the use the same flag banks"
    );
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.0,
        "the pause the plain release wears"
    );
    assert!(!app.dig_held, "the pause the dig hold clears");
    // THE PAUSED FRAMES (the world_reads false the flip never speaks)
    assert!(
        !app.toggle_key_state(TOGGLE_SPRINT, false, false),
        "the pause frames the unlatched word keeps"
    );
    // THE ESC UNPAUSE ARM (the setScreen(null) close arm the restore
    // rides — the keyboard-bound sprint the re-latch wears, the keyUse
    // the shouldRestore FALSE refuses, the flags the always-clear
    // consume, the plain keys the poll's own re-read the re-clear
    // never wears)
    app.on_escape();
    assert!(!app.paused, "the second escape the unpause arms");
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the unpause the sprint re-latches"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the restore the flag the always-clear consumes"
    );
    assert!(
        !app.toggle_latched(TOGGLE_USE),
        "the keyUse the shouldRestore FALSE the restore refuses"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_USE],
        "the refused verdict the flag the same clear wears"
    );
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.0,
        "the unpause the re-clear never rides"
    );
}

/// THE PAUSE LANES' OWN RESTORE MAP (the focus pause the same release
/// pair the ESC arm wears; the backtogame button the setScreen(null)
/// close arm the restore rides; the DEBUG-CHORD chain the
/// interrupt-over-pause the banked word keeps — the chord's close the
/// restore the chain completes; the OPTIONS takeover the pause road
/// the open's release arm the banked word keeps, the world's return
/// the open(World) arm the re-latch completes).
#[test]
fn the_focus_pause_the_backtogame_the_chord_close_the_map_wears() {
    use crate::screens::TOGGLE_SPRINT;
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.toggle_sprint = true;
    assert!(app.toggle_key_state(TOGGLE_SPRINT, true, true));
    assert!(app.toggle_key_state(TOGGLE_SPRINT, false, true));
    // THE FOCUS PAUSE ARM (the same pair the ESC pause wears — the
    // latch clears, the flag banks, the plain banks the held walk
    // zeroes)
    if let Some(b) = app.body.as_mut() {
        b.forward = 1.0;
    }
    app.on_window_focus(true, 0);
    app.on_window_focus(false, 501);
    assert!(app.paused, "the focus loss the pause arms");
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the focus pause the latch clears"
    );
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the focus pause the flag banks"
    );
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.0,
        "the focus pause the plain release wears"
    );
    // THE BACKTOGAME BUTTON (the setScreen(null) close arm the
    // restore rides — the re-latch the world's return speaks)
    app.render_title_buttons_for_test();
    let back = app
        .buttons
        .iter()
        .find(|b| b.id == "backtogame")
        .expect("backtogame")
        .clone();
    app.on_click(back.x + back.w / 2, back.y + back.h / 2);
    assert!(!app.paused, "the backtogame the unpause speaks");
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the backtogame the re-latch wears"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the restore the flag the always-clear consumes"
    );
    // THE DEBUG-CHORD CHAIN (the interrupt the paused world serves —
    // the banked word the open keeps — the chord's close the restore
    // the chain completes)
    app.on_escape();
    assert!(app.paused, "the second pause arms");
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the pause the flag re-banks"
    );
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the interrupt the debug options opens"
    );
    assert!(!app.paused, "the interrupt the unpause speaks");
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the open the banked word keeps"
    );
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the open the unlatched word keeps"
    );
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the chord close the world returns"
    );
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the close the restore the chain completes"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the close the flag consumes"
    );
    // THE OPTIONS TAKEOVER (the pause road the open's release arm the
    // banked word keeps — the back the open(World) the re-latch
    // completes, the paused seat the return keeps)
    app.on_escape();
    assert!(app.paused, "the third pause arms");
    app.render_title_buttons_for_test();
    let opt = app
        .buttons
        .iter()
        .find(|b| b.id == "options")
        .expect("pauseoptions")
        .clone();
    app.on_click(opt.x + opt.w / 2, opt.y + opt.h / 2);
    assert_eq!(app.screen, ScreenKind::Options, "the options opens");
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the takeover's release the banked word keeps"
    );
    app.render_title_buttons_for_test();
    let back = app
        .buttons
        .iter()
        .find(|b| b.id == "back")
        .expect("back")
        .clone();
    app.on_click(back.x + back.w / 2, back.y + back.h / 2);
    assert_eq!(app.screen, ScreenKind::World, "the world returns");
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the world's return the re-latch completes"
    );
    assert!(app.paused, "returning from options stays paused");
}

/// THE DEBUG-CHORD'S OWN OPEN ARM (the vanilla setScreen(non-null)
/// walk the interrupt lane wears — the pc 205-215 releaseMouse +
/// releaseAll the F3+F6 open speaks: the latched seats the flag
/// banks + the latch clears, the plain banks the held walk zeroes,
/// the dig hold clears; the second tap's close the 221 restore law
/// the banked flag the re-latch serves — the round-trip the latch
/// survives; the ESC close lane the same restore wears).
#[test]
fn the_debug_chord_open_the_held_walk_the_release_wears() {
    use crate::screens::TOGGLE_SPRINT;
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.toggle_sprint = true;
    assert!(app.toggle_key_state(TOGGLE_SPRINT, true, true));
    assert!(app.toggle_key_state(TOGGLE_SPRINT, false, true));
    if let Some(b) = app.body.as_mut() {
        b.forward = 1.0;
    }
    app.dig_held = true;
    // THE OPEN WALK (the interrupt arm the release pair wears — the
    // toggle release the plain walk's BEFORE seat the pc 215 order
    // keeps)
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the chord the debug options opens"
    );
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the open the sprint latch clears"
    );
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the open the flag the release banks"
    );
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.0,
        "the open the held walk zeroes"
    );
    assert!(!app.dig_held, "the open the dig hold clears");
    // THE SECOND TAP'S OWN CLOSE (the instanceof arm the restore
    // rides — the re-latch the banked flag the world's return serves)
    app.on_debug_options_key();
    assert_eq!(app.screen, ScreenKind::World, "the chord closes");
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the close the re-latch wears"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the close the flag the always-clear consumes"
    );
    // THE SECOND CYCLE (the re-latched seat the open re-banks, the
    // ESC close lane the same restore the re-latch wears)
    app.on_debug_options_key();
    assert_eq!(app.screen, ScreenKind::DebugOptions, "the reopen serves");
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the second open the flag re-banks"
    );
    app.on_escape();
    assert_eq!(app.screen, ScreenKind::World, "the ESC lane closes");
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the ESC close the re-latch wears"
    );
}

/// THE DEBUG-CHORD OPEN'S OWN ARMS MAP (the dead world the interrupt
/// refuses — the release never rides, the held banks the word keeps;
/// the lock-confirm seat the same refusal wears; the pause seat the
/// interrupt serves — the banked word the release the `latch || flag`
/// arm keeps, the plain banks the pause arm's zero the no-op wears).
#[test]
fn the_debug_chord_open_the_arms_map_the_refusals_keep() {
    use crate::screens::TOGGLE_SPRINT;
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.toggle_sprint = true;
    assert!(app.toggle_key_state(TOGGLE_SPRINT, true, true));
    assert!(app.toggle_key_state(TOGGLE_SPRINT, false, true));
    if let Some(b) = app.body.as_mut() {
        b.forward = 1.0;
    }
    // THE DEATH SCREEN'S OWN REFUSAL (the canInterrupt FALSE verdict
    // the release never rides — the held banks the word keeps, the
    // latch the unlatched word never wears)
    app.dead = true;
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::World,
        "the dead the interrupt refuses"
    );
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        1.0,
        "the refusal the held word keeps"
    );
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the refusal the latch keeps"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the refusal the flag never banks"
    );
    app.dead = false;
    // THE LOCK CONFIRM'S OWN REFUSAL (the same FALSE verdict wears —
    // the modal question the release never steals)
    app.screen = ScreenKind::DifficultyLockConfirm;
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DifficultyLockConfirm,
        "the lock confirm refuses"
    );
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        1.0,
        "the refusal the held word keeps"
    );
    app.screen = ScreenKind::World;
    // THE PAUSE SEAT (the interrupt the paused world serves — the
    // banked word the release the `latch || flag` arm keeps, the
    // plain banks the pause arm's zero the no-op wears, the close
    // the re-latch the chain completes)
    app.on_escape();
    assert!(app.paused, "the pause arms");
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the pause the flag banks"
    );
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the pause the interrupt serves"
    );
    assert!(!app.paused, "the interrupt the unpause speaks");
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the open the banked word keeps"
    );
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the open the unlatched word keeps"
    );
    app.on_debug_options_key();
    assert_eq!(app.screen, ScreenKind::World, "the chord closes");
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the close the re-latch completes"
    );
}

/// THE DEBUG-CHORD INTERRUPT'S OWN REMOVED WALK (the vanilla
/// setScreen walk the OLD screen's removed() the interrupt FIRST
/// answers — the chat-open seat the ChatScreen's interrupted close
/// the draft bank serves: the chat folds, the typed text the
/// INTERRUPTED verdict never burns, the T-reopen the createScreen
/// restorable arm the draft restores; the chord's close the folded
/// chat the world's return keeps).
#[test]
fn the_debug_chord_interrupt_the_chat_draft_the_removed_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    // THE CHAT'S OWN OPEN (the T lane — the typed text the edit wears)
    app.on_chat_key();
    assert!(app.chat_open, "the chat the open keeps");
    app.on_chat_char('h');
    app.on_chat_char('i');
    assert_eq!(app.chat_input, "hi", "the typed text the input wears");
    // THE INTERRUPT'S OWN REMOVED WALK (the F3+F6 the old
    // ChatScreen's removed() the interrupted close serves — the chat
    // folds, the draft the bank keeps)
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the chord the debug options opens"
    );
    assert!(!app.chat_open, "the interrupt the chat folds");
    assert_eq!(
        app.chat_latest_draft,
        Some(("hi".to_string(), false)),
        "the interrupted close the draft banks"
    );
    // THE CHORD'S OWN CLOSE (the world's return the folded chat the
    // takeover law keeps — the removed walk the fold already spoke)
    app.on_debug_options_key();
    assert_eq!(app.screen, ScreenKind::World, "the chord closes");
    assert!(!app.chat_open, "the close the folded chat keeps");
    // THE T-REOPEN'S OWN RESTORE (the createScreen restorable arm —
    // the banked draft the input wears, the purity flag the true arg
    // banks)
    app.on_chat_key();
    assert!(app.chat_open, "the reopen serves");
    assert_eq!(app.chat_input, "hi", "the reopen the draft restores");
    assert!(
        app.chat_input_is_draft,
        "the restored draft the purity flag wears"
    );
}

/// THE DEBUG-CHORD INTERRUPT'S OWN REMOVED MAP (the options-family
/// seat the removed() save ride keeps — the flipped row the file
/// banks; the widget state the drag deaths the removed() walk
/// discards — the armed slider drag the screen swap never inherits).
#[test]
fn the_debug_chord_interrupt_the_options_save_and_drag_death_wear() {
    let dir = std::env::temp_dir().join(format!("mc2rs_chordrem_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("options.txt");
    let mut app = App::new(3);
    app.options_file = Some(path.clone());
    app.enter_world();
    // THE OPTIONS SEAT (the Options screen the chord interrupts — the
    // removed() moment the save ride the file's rows keep)
    app.open(ScreenKind::Options);
    app.pause_on_lost_focus = false;
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the chord the options screen interrupts"
    );
    let rows = std::fs::read_to_string(&path).expect("the interrupt the save writes");
    assert!(
        rows.contains("pauseOnLostFocus:false"),
        "the removed() save the flipped row keeps"
    );
    // THE DRAG-DEATH SEAT (the MouseSettings slider the armed drag —
    // the removed() walk the widget state discards, the fresh screen
    // the swap never inherits)
    app.open(ScreenKind::MouseSettings);
    app.sensitivity_drag = true;
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the chord the mouse settings interrupts"
    );
    assert!(
        !app.sensitivity_drag,
        "the removed() walk the armed drag discards"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE DEBUG-CHORD INTERRUPT'S OWN CONTAINER SEAT (javap 26.2 the
/// setScreen walk the `AbstractContainerScreen.removed()` override
/// answers — the menu's removed law the carried stack home returns,
/// the container's own close the whole ride serves): the chest seat
/// the cursor stack arms, the F3+F6 interrupt the removed() walk
/// rides — the carried home returns, the store flush the edited
/// slots' disk bank rides, the lid the release clears, the debug
/// options the SAME walk installs; the chord's close the world
/// returns (the restore ride the banked toggles keep — no banked
/// word the plain seat carries).
#[test]
fn the_debug_chord_interrupt_the_chest_seat_the_carried_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_chordchest_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let stores = dir.join("stores.nbt");
    let mut app = App::new(3);
    app.container_stores_file = Some(stores);
    let cell = (7, 40, 7);
    // THE CHEST SEAT (the right-click lane's own open — the flush key
    // the cell banks, the grid the fresh 27 the store inserts)
    app.open_chest_for(cell);
    assert_eq!(app.screen, ScreenKind::Chest, "the chest seat arms");
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0); // the deterministic store the add-back reads
    }
    app.chest_carried = (dirt, 8);
    // THE INTERRUPT'S OWN REMOVED WALK (the F3+F6 the chest's own
    // close ride the removed() law serves — the carried home, the
    // flush, the lid, the debug seat the SAME walk installs)
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the chord the chest seat interrupts"
    );
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the removed() walk the carried home returns"
    );
    assert_eq!(
        app.inventory.items[0],
        (dirt, 8),
        "the add-back walk the first empty slot takes"
    );
    assert_eq!(
        app.hotbar_slots[0],
        (dirt, 8),
        "the HUD row the add-back syncs"
    );
    assert!(
        !app.chest_is_live,
        "the window belief the removed() discards"
    );
    // THE CHORD'S OWN CLOSE (the world's return the restore ride
    // wears — the plain seat the banked word never carries)
    app.on_debug_options_key();
    assert_eq!(app.screen, ScreenKind::World, "the chord closes");
    // THE STORE'S OWN FLUSH (the interrupt's close ride the bank
    // serves — the GUI's own edit the removed() walk's flush banks,
    // the reopen the disk bank's own slot reads)
    app.open_chest_for(cell);
    app.chest_slots[3] = (dirt, 2);
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the chord the chest seat interrupts again"
    );
    app.on_debug_options_key();
    assert_eq!(app.screen, ScreenKind::World, "the chord closes again");
    app.open_chest_for(cell);
    assert_eq!(
        app.chest_slots[3],
        (dirt, 2),
        "the interrupted close's flush the edit banks"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE DEBUG-CHORD INTERRUPT'S OWN CONTAINER MAP (the inventory seat
/// the doubleclick state the removed() discards whole, the carried
/// the add-back ride serves; the crafting seat the grid the
/// clearContainer walk home returns — the CraftingMenu.removed's own
/// law; the chat seat the draft bank keeps, the plain world the
/// silence keeps — the container seats the removed law ALONE wears).
#[test]
fn the_debug_chord_interrupt_the_container_map_the_removed_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    // THE INVENTORY SEAT (the E lane's own open — the cursor stack +
    // the doubleclick state the interrupted screen instance carries)
    app.open_inventory();
    assert_eq!(app.screen, ScreenKind::Inventory, "the inventory arms");
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.inv_carried = (dirt, 5);
    app.inv_doubleclick = true;
    app.inv_last_click_slot = Some(3);
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the chord the inventory seat interrupts"
    );
    assert_eq!(
        app.inv_carried,
        (0, 0),
        "the removed() walk the carried home returns"
    );
    assert_eq!(
        app.inventory.items[0],
        (dirt, 5),
        "the add-back walk the store takes"
    );
    assert!(
        !app.inv_doubleclick,
        "the removed() walk the doubleclick discards"
    );
    assert!(
        app.inv_last_click_slot.is_none(),
        "the removed() walk the last-click seat discards"
    );
    // THE CRAFTING SEAT (the grid + the carried the clearContainer
    // walk home returns — the removed() law the menu's own override
    // speaks, the result cell the clear never re-banks)
    app.open_crafting_table();
    assert_eq!(app.screen, ScreenKind::Crafting, "the table arms");
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.table_grid[0] = (dirt, 3);
    app.table_grid[4] = (dirt, 2);
    app.inv_carried = (dirt, 1);
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the chord the table seat interrupts"
    );
    assert_eq!(
        app.table_grid[0],
        (0, 0),
        "the clearContainer walk the grid cell drains"
    );
    assert_eq!(
        app.table_grid[4],
        (0, 0),
        "the clearContainer walk the second cell drains"
    );
    assert_eq!(
        app.inv_carried,
        (0, 0),
        "the removed() walk the carried home returns"
    );
    assert_eq!(
        app.inventory.items[0],
        (dirt, 6),
        "the add-back walk the six the merges speak"
    );
    assert_eq!(
        app.inventory.items[1],
        (0, 0),
        "the merges the single stack speaks"
    );
    // THE CHAT SEAT'S OWN LAW KEEPS (the draft bank the 223 walk
    // banks — the container seats the removed law the chat's own
    // close never displaces)
    app.on_debug_options_key();
    assert_eq!(app.screen, ScreenKind::World, "the chord closes");
    app.on_chat_key();
    app.on_chat_char('x');
    app.on_debug_options_key();
    assert_eq!(
        app.screen,
        ScreenKind::DebugOptions,
        "the chord the chat seat interrupts"
    );
    assert_eq!(
        app.chat_latest_draft,
        Some(("x".to_string(), false)),
        "the chat seat the draft bank keeps"
    );
}

/// THE OPEN()'S OWN CONTAINER RIDE (javap 26.2 the Minecraft.setScreen
/// walk the OLD screen's removed() FIRST answers — the autopush-224
/// law the F3+F6 interrupt lane alone wore, the open() ride the WHOLE
/// client's setScreen walk serves): the chest seat the disconnect
/// takeover overtakes — the carried home returns, the store flush the
/// GUI's own edit banks, the window belief dies, the SAME walk the
/// back seat installs; the tail's flag the re-entry consumes (the
/// removed law the transition answers ONCE).
#[test]
fn the_open_ride_the_chest_takeover_the_removed_wears() {
    let dir = std::env::temp_dir().join(format!("mc2rs_openride_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let stores = dir.join("stores.nbt");
    let mut app = App::new(3);
    app.container_stores_file = Some(stores);
    let cell = (9, 41, 9);
    // THE CHEST SEAT (the right-click lane's own open — the flush key
    // the cell banks, the grid the fresh 27 the store inserts)
    app.open_chest_for(cell);
    assert_eq!(app.screen, ScreenKind::Chest, "the chest seat arms");
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.chest_slots[2] = (dirt, 4); // the GUI's own edit the flush banks
    app.chest_carried = (dirt, 8);
    // THE TAKEOVER (the open() ride — the disconnect the chest seat
    // overtakes, the removed() law the walk FIRST answers)
    app.open(ScreenKind::Disconnect);
    assert_eq!(
        app.screen,
        ScreenKind::Disconnect,
        "the takeover the SAME walk installs"
    );
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the removed() walk the carried home returns"
    );
    assert_eq!(
        app.inventory.items[0],
        (dirt, 8),
        "the add-back walk the first empty slot takes"
    );
    assert_eq!(
        app.hotbar_slots[0],
        (dirt, 8),
        "the HUD row the add-back syncs"
    );
    assert!(
        !app.chest_is_live,
        "the window belief the removed() discards"
    );
    assert!(
        !app.container_removed_rode,
        "the tail's flag the re-entry consumes"
    );
    // THE STORE'S OWN FLUSH (the removed walk's flush the GUI's own
    // edit the map bank rides — the reopen the banked slot reads)
    assert_eq!(
        app.container_stores.get(&cell).map(|slots| slots[2]),
        Some((dirt, 4)),
        "the flush the GUI's own edit banks"
    );
    app.open_chest_for(cell);
    assert_eq!(
        app.chest_slots[2],
        (dirt, 4),
        "the reopen the banked slot reads"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// THE CLOSE LANE'S OWN TAIL THE FLAG CONSUMES (the Escape walk the
/// close_*(World) direct serves — the tail's `open(back)` the open()
/// re-enters, the flag the tail set the re-entry consumes: the removed
/// law the transition answers ONCE, the double-open churn the second
/// ride never speaks); THE LAW RE-ARMS (the next takeover the
/// flag-false seat the ride re-serves — the merge the add-back speaks).
#[test]
fn the_close_lane_tail_the_single_ride_the_reentry_wears() {
    let mut app = App::new(3);
    let cell = (11, 42, 11);
    app.open_chest_for(cell);
    assert_eq!(app.screen, ScreenKind::Chest, "the chest seat arms");
    let dirt = mc_registry::item_id_by_name("dirt").unwrap();
    for it in app.inventory.items.iter_mut() {
        *it = (0, 0);
    }
    app.chest_carried = (dirt, 3);
    // THE DIRECT CLOSE LANE (the Escape walk the close_*(World) serves)
    app.close_chest_container(ScreenKind::World);
    assert_eq!(app.screen, ScreenKind::World, "the close lane installs");
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the removed() walk the carried home returns"
    );
    assert_eq!(
        app.inventory.items[0],
        (dirt, 3),
        "the add-back walk the store takes"
    );
    assert!(
        !app.container_removed_rode,
        "the tail's flag the re-entry consumes — the ride ONCE"
    );
    // THE LAW RE-ARMS (the next takeover the flag-false seat rides —
    // the merge the add-back speaks)
    app.open_chest_for(cell);
    app.chest_carried = (dirt, 2);
    app.open(ScreenKind::World);
    assert_eq!(app.screen, ScreenKind::World, "the takeover installs");
    assert_eq!(
        app.chest_carried,
        (0, 0),
        "the re-armed ride the carried homes"
    );
    assert_eq!(
        app.inventory.items[0],
        (dirt, 5),
        "the merge the add-back speaks"
    );
    assert!(!app.container_removed_rode, "the flag the ride consumes");
}

/// THE SWITCHER'S OWN TOGGLE-DANCE (the F3+F4 chord the two arms the
/// direct swap never rode — the open arm the release pair the pc 205-215
/// walk wears: the held banks the zeroes speak, the latched seats the
/// flag banks + the latch clears; the apply close the restore the
/// shouldRestore trio the re-latch wears, the keyUse the FALSE verdict
/// refuses, the flags the always-clear consume; the gates' refusals
/// the release never rides — the pause and the death the held word
/// keeps).
#[test]
fn the_switcher_open_the_release_dance_the_apply_close_the_restore_wears() {
    use crate::screens::{TOGGLE_SPRINT, TOGGLE_USE};
    let mut app = App::new(3);
    app.enter_world();
    app.spawn_offline_body();
    app.toggle_sprint = true;
    app.toggle_use = true;
    // THE LATCHES (the world's own press+release pairs wear)
    assert!(app.toggle_key_state(TOGGLE_SPRINT, true, true));
    assert!(app.toggle_key_state(TOGGLE_SPRINT, false, true));
    assert!(app.toggle_key_state(TOGGLE_USE, true, true));
    assert!(app.toggle_key_state(TOGGLE_USE, false, true));
    // THE HELD WALK (the plain banks the open arm clears)
    if let Some(b) = app.body.as_mut() {
        b.forward = 1.0;
    }
    app.dig_held = true;
    // THE F3+F4 OPEN ARM (the release pair the screen install's BEFORE
    // seat wears — the pc 205-215 order the open() dance's own shape
    // keeps: the latch clears, the flag banks, the plain banks zero,
    // the dig hold clears, the switcher's own state arms the same
    // walk keeps)
    app.on_open_game_mode_switcher();
    assert_eq!(
        app.screen,
        ScreenKind::GameModeSwitcher,
        "the chord the switcher opens"
    );
    assert_eq!(
        app.gm_hovered, 1,
        "the default hover the current mode speaks"
    );
    assert!(
        app.gm_first_mouse.is_none(),
        "the first-mouse bank the open resets"
    );
    assert!(app.debug_modifier_used, "the chord consumes the modifier");
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the open the sprint latch clears"
    );
    assert!(
        !app.toggle_latched(TOGGLE_USE),
        "the open the use latch the same walk clears"
    );
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the open the flag the release banks"
    );
    assert!(
        app.toggle_released_by_screen[TOGGLE_USE],
        "the open the use the same flag banks"
    );
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.0,
        "the open the plain release wears"
    );
    assert!(!app.dig_held, "the open the dig hold clears");
    // THE CYCLE WALK (the open screen's own F4 arm — the banks the
    // cycle never re-arms, the hover the getNext law cycles)
    app.on_game_mode_switcher_cycle();
    assert_eq!(app.gm_hovered, 2, "the cycle the adventure hover speaks");
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the cycle the cleared latch keeps"
    );
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the cycle the banked flag keeps"
    );
    // THE F3 RELEASE'S OWN APPLY (the same-mode arm the silent close
    // keeps — hover 2 = adventure, mode survival... the cycle the
    // hover moved: the apply LANDS the adventure, the close the
    // restore rides)
    app.on_debug_modifier_released();
    assert_eq!(app.screen, ScreenKind::World, "the release closes");
    assert_eq!(
        app.game_mode,
        GameMode::Adventure,
        "the cycle the apply lands"
    );
    assert_eq!(
        app.previous_game_mode,
        Some(GameMode::Survival),
        "the bank the old mode fills"
    );
    // THE APPLY CLOSE'S OWN RESTORE (the setScreen(null) walk — the
    // keyboard-bound sprint the re-latch wears, the keyUse the
    // shouldRestore FALSE refuses, the flags the always-clear consume,
    // the plain banks the poll's own re-read the re-clear never wears)
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the close the sprint re-latches"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the restore the flag the always-clear consumes"
    );
    assert!(
        !app.toggle_latched(TOGGLE_USE),
        "the keyUse the shouldRestore FALSE the restore refuses"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_USE],
        "the refused verdict the flag the same clear wears"
    );
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        0.0,
        "the close the re-clear never rides"
    );
    // THE DANCE RE-ARMS (the re-open the re-latched seat the flag
    // re-banks — the flick apply the Survival returns, the close the
    // re-latch the SECOND round-trip completes)
    app.on_open_game_mode_switcher();
    assert_eq!(app.screen, ScreenKind::GameModeSwitcher, "the re-open arms");
    assert_eq!(app.gm_hovered, 1, "the previous bank the reopen hovers");
    assert!(
        !app.toggle_latched(TOGGLE_SPRINT),
        "the re-open the latch re-clears"
    );
    assert!(
        app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the re-open the flag re-banks"
    );
    app.on_debug_modifier_released();
    assert_eq!(app.screen, ScreenKind::World, "the second close speaks");
    assert_eq!(
        app.game_mode,
        GameMode::Survival,
        "the flick the survival returns"
    );
    assert_eq!(
        app.previous_game_mode,
        Some(GameMode::Adventure),
        "the bank the adventure fills"
    );
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the second close the re-latch wears"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the second restore the flag consumes"
    );
    // THE PAUSE GATE'S OWN REFUSAL (the open the gate refuses — the
    // release never rides: the held word keeps, the latch keeps, the
    // flag never banks)
    if let Some(b) = app.body.as_mut() {
        b.forward = 1.0;
    }
    app.paused = true;
    app.on_open_game_mode_switcher();
    assert_eq!(app.screen, ScreenKind::World, "the pause refuses the open");
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the refusal the latch keeps"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the refusal the flag never banks"
    );
    assert_eq!(
        app.body.as_ref().unwrap().forward,
        1.0,
        "the refusal the held word keeps"
    );
    app.paused = false;
    // THE DEATH GATE'S OWN REFUSAL (the same verdict the dead world
    // wears — the release never rides)
    app.dead = true;
    app.on_open_game_mode_switcher();
    assert_eq!(app.screen, ScreenKind::World, "the death refuses the open");
    assert!(
        app.toggle_latched(TOGGLE_SPRINT),
        "the dead refusal the latch keeps"
    );
    assert!(
        !app.toggle_released_by_screen[TOGGLE_SPRINT],
        "the dead refusal the flag never banks"
    );
    app.dead = false;
}

/// THE TRI-STATE'S OWN PILL COLORS (javap 26.2 the clinit's copies
/// the three pills wear — the never pill's ON the `-1` WHITE speaks,
/// the in-overlay pill's ON the `-171` YELLOW (0xFFFFFF55), the
/// always pill's ON the `-2142128` CORAL (0xFFDF6550), the
/// unselected arms the tan `-4539718` (0xFFBAA13A) wear — the glyph
/// band the render's own pixels probe).
#[test]
fn the_debug_entries_the_pill_colors_the_tri_state_wears() {
    use crate::screens::DebugEntryStatus;
    fn rect_has(
        fb: &crate::raster::Framebuf,
        x0: i32,
        y0: i32,
        w: i32,
        h: i32,
        rgb: [u8; 3],
    ) -> bool {
        for y in (y0.max(0) as usize)..((y0 + h).min(fb.h as i32) as usize) {
            for x in (x0.max(0) as usize)..((x0 + w).min(fb.w as i32) as usize) {
                let i = (y * fb.w + x) * 3;
                if fb.color[i] == rgb[0] && fb.color[i + 1] == rgb[1] && fb.color[i + 2] == rgb[2] {
                    return true;
                }
            }
        }
        false
    }
    const WHITE: [u8; 3] = [255, 255, 255];
    const YELLOW: [u8; 3] = [255, 255, 85];
    const CORAL: [u8; 3] = [223, 101, 80];
    const TAN: [u8; 3] = [186, 161, 58];
    let mut app = App::new(3);
    app.enter_world();
    app.on_debug_options_key();
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let mut assets = GuiAssets::open(assets_root());
    // THE FRESH ROW (the NEVER statuses the Off pill's ON arm wears —
    // the white word the band speaks, the overlay + always pills the
    // tan unselected arms keep; the chunk_borders row the 700px
    // offset the visible window brings)
    app.debug_scroll = 700.0;
    app.render(&mut fb, &mut assets);
    let off = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgoff" && b.label == "31")
        .expect("the off pill the row wears")
        .clone();
    let overlay = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgoverlay" && b.label == "31")
        .expect("the overlay pill the row wears")
        .clone();
    let always = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgalways" && b.label == "31")
        .expect("the always pill the row wears")
        .clone();
    assert!(
        rect_has(&fb, off.x, off.y + 6, off.w, 7, WHITE),
        "the fresh NEVER the off pill's white ON arm wears"
    );
    assert!(
        !rect_has(&fb, off.x, off.y + 6, off.w, 7, TAN),
        "the ON pill the tan unselected arm never wears"
    );
    assert!(
        rect_has(&fb, overlay.x, overlay.y + 6, overlay.w, 7, TAN),
        "the unselected overlay pill the tan wears"
    );
    assert!(
        !rect_has(&fb, overlay.x, overlay.y + 6, overlay.w, 7, YELLOW),
        "the unselected overlay pill the yellow never wears"
    );
    assert!(
        rect_has(&fb, always.x, always.y + 6, always.w, 7, TAN),
        "the unselected always pill the tan wears"
    );
    assert!(
        !rect_has(&fb, always.x, always.y + 6, always.w, 7, CORAL),
        "the unselected always pill the coral never wears"
    );
    // THE OVERLAY ARM (the yellow pill the IN_OVERLAY status wears —
    // the ON arm the -171 fold speaks, the siblings the tan keep)
    app.on_click(overlay.x + overlay.w / 2, overlay.y + 10);
    assert_eq!(
        app.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY],
        DebugEntryStatus::Overlay,
        "the click the IN_OVERLAY banks"
    );
    app.render(&mut fb, &mut assets);
    let overlay2 = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgoverlay" && b.label == "31")
        .expect("the rebuild wears")
        .clone();
    assert!(
        rect_has(&fb, overlay2.x, overlay2.y + 6, overlay2.w, 7, YELLOW),
        "the IN_OVERLAY status the yellow ON arm wears"
    );
    assert!(
        !rect_has(&fb, overlay2.x, overlay2.y + 6, overlay2.w, 7, TAN),
        "the ON pill the tan never wears"
    );
    // THE ALWAYS ARM (the coral pill the ALWAYS_ON status wears — the
    // ON arm the -2142128 fold speaks)
    app.render(&mut fb, &mut assets);
    let always2 = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgalways" && b.label == "31")
        .expect("the rebuild wears")
        .clone();
    app.on_click(always2.x + always2.w / 2, always2.y + 10);
    assert_eq!(
        app.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY],
        DebugEntryStatus::Always,
        "the click the ALWAYS_ON banks"
    );
    app.render(&mut fb, &mut assets);
    let always3 = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgalways" && b.label == "31")
        .expect("the rebuild wears")
        .clone();
    assert!(
        rect_has(&fb, always3.x, always3.y + 6, always3.w, 7, CORAL),
        "the ALWAYS_ON status the coral ON arm wears"
    );
    assert!(
        !rect_has(&fb, always3.x, always3.y + 6, always3.w, 7, TAN),
        "the ON pill the tan never wears"
    );
}

/// THE ENTRIES LIST'S OWN WALK (javap 26.2 the updateSearch law —
/// the 45-row bank the clinit's register calls speak, the sorted
/// order the comparator serves (the TEXT sortKey the head keeps,
/// the Identifier the alphabetical path speaks within), the
/// OptionList's own scroll the wheel step answers, the visible
/// window the rows the clip carries).
#[test]
fn the_debug_entries_list_the_bank_the_scroll_the_rows_wear() {
    use crate::screens::DebugEntryStatus;
    let mut app = App::new(3);
    app.enter_world();
    app.on_debug_options_key();
    // THE BANK'S OWN LAW (the sorted order the comparator serves: the
    // TEXT head the biome row keeps, the RENDERER head the
    // 3d_crosshair seat wears, the two chord consts the registry ids
    // speak)
    assert_eq!(
        crate::screens::DEBUG_ENTRY_ROWS.len(),
        45,
        "the clinit's own count"
    );
    assert_eq!(
        crate::screens::DEBUG_ENTRY_ROWS[0].0,
        "biome",
        "the TEXT block the head keeps"
    );
    assert_eq!(
        crate::screens::DEBUG_ENTRY_ROWS[30].0,
        "3d_crosshair",
        "the RENDERER block the head wears"
    );
    assert_eq!(
        crate::screens::DEBUG_ENTRY_ROWS[crate::screens::CHUNK_BORDERS_ENTRY].0,
        "chunk_borders",
        "the F3+G chord's own seat"
    );
    assert_eq!(
        crate::screens::DEBUG_ENTRY_ROWS[crate::screens::ENTITY_HITBOXES_ENTRY].0,
        "entity_hitboxes",
        "the F3+B chord's own seat"
    );
    assert_eq!(
        crate::screens::DEBUG_ENTRY_ROWS[8].0,
        "fps",
        "the perf profile's own row the TEXT block carries"
    );
    // THE FRESH SCROLL'S OWN ROWS (the top band the fps row carries —
    // the TEXT row the same tri-state store wears)
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let mut assets = GuiAssets::open(assets_root());
    app.render(&mut fb, &mut assets);
    let fps_off = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgoff" && b.label == "8")
        .expect("the fps row's off pill the visible band carries")
        .clone();
    app.on_click(fps_off.x + fps_off.w / 2, fps_off.y + 10);
    assert_eq!(
        app.debug_entry_states[8],
        DebugEntryStatus::Never,
        "the off pill the NEVER banks — the TEXT row the same store wears"
    );
    assert_eq!(
        app.debug_profile, None,
        "the manual touch the identity breaks"
    );
    // THE OVERLAY PILL THE SAME ROW WEARS (the click the IN_OVERLAY
    // banks — the store the row bank's own seat keeps)
    app.render(&mut fb, &mut assets);
    let fps_overlay = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgoverlay" && b.label == "8")
        .expect("the rebuild wears")
        .clone();
    app.on_click(fps_overlay.x + fps_overlay.w / 2, fps_overlay.y + 10);
    assert_eq!(
        app.debug_entry_states[8],
        DebugEntryStatus::Overlay,
        "the overlay pill the IN_OVERLAY banks"
    );
    // THE WHEEL'S OWN WALK (the clamp law — the top the 0 floor
    // keeps, the bottom the content band's ceiling wears, the notch
    // the 24px step speaks)
    app.on_debug_options_scroll(1.0, 180);
    assert_eq!(app.debug_scroll, 0.0, "the top the floor keeps");
    app.on_debug_options_scroll(-1.0, 180);
    assert_eq!(app.debug_scroll, 24.0, "the notch the 24px step wears");
    app.on_debug_options_scroll(-1000.0, 180);
    assert_eq!(
        app.debug_scroll, 1032.0,
        "the ceiling the content band speaks"
    );
    // THE SCROLLED RENDER (the water_levels row the bottom band
    // carries — the RENDERER row the same store wears)
    app.render(&mut fb, &mut assets);
    let wl_always = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgalways" && b.label == "44")
        .expect("the scrolled row's always pill the band carries")
        .clone();
    app.on_click(wl_always.x + wl_always.w / 2, wl_always.y + 10);
    assert_eq!(
        app.debug_entry_states[44],
        DebugEntryStatus::Always,
        "the RENDERER row the ALWAYS_ON banks"
    );
}

#[test]
fn the_debug_search_the_responder_the_filter_wears() {
    use crate::screens::DebugEntryStatus;
    fn rect_has(
        fb: &crate::raster::Framebuf,
        x0: i32,
        y0: i32,
        w: i32,
        h: i32,
        rgb: [u8; 3],
    ) -> bool {
        for y in (y0.max(0) as usize)..((y0 + h).min(fb.h as i32) as usize) {
            for x in (x0.max(0) as usize)..((x0 + w).min(fb.w as i32) as usize) {
                let i = (y * fb.w + x) * 3;
                if fb.color[i] == rgb[0] && fb.color[i + 1] == rgb[1] && fb.color[i + 2] == rgb[2] {
                    return true;
                }
            }
        }
        false
    }
    const WHITE: [u8; 3] = [255, 255, 255];
    let mut app = App::new(3);
    app.enter_world();
    app.on_debug_options_key();
    // THE OPEN'S OWN FRESH CTOR (the setScreen re-creation the empty
    // box keeps — the stale word the scrolled band the re-open never
    // inherits)
    app.debug_search.push_str("stale");
    app.debug_scroll = 500.0;
    app.on_debug_options_key(); // the close arm the World serves
    app.on_debug_options_key(); // the re-open the fresh ctor arms
    assert_eq!(app.debug_search, "", "the fresh box the empty keeps");
    assert_eq!(app.debug_scroll, 0.0, "the fresh list the top keeps");
    // THE CHARS' OWN BANK (the responder the typed word wears — the
    // scroll the rebuild's top reset speaks: the pre-typed wheel the
    // 48px the zero the rebuild retires)
    app.on_debug_options_scroll(-2.0, 180);
    assert_eq!(app.debug_scroll, 48.0, "the pre-typed wheel the step wears");
    for c in "FPS".chars() {
        app.on_debug_options_char(c);
    }
    assert_eq!(app.debug_search, "FPS", "the box the typed word keeps");
    assert_eq!(app.debug_scroll, 0.0, "the rebuild the top wears");
    // THE FILTER'S OWN WALK (the contains law the render answers —
    // the fps TEXT row the hit keeps, the biome head the miss the
    // rebuild never banks, the renderer block the whole-miss the
    // header never emits)
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let mut assets = GuiAssets::open(assets_root());
    app.render(&mut fb, &mut assets);
    assert!(
        rect_has(&fb, 218, 26, 200, 9, WHITE),
        "the typed word the box's white glyphs wear"
    );
    assert!(
        app.buttons
            .iter()
            .any(|b| b.id == "dbgoff" && b.label == "8"),
        "the fps row the hit list keeps"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "dbgoff" && b.label == "0"),
        "the biome row the filter retires"
    );
    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id.starts_with("dbg") && b.label == "31"),
        "the renderer block the whole-miss retires"
    );
    // THE FILTERED SCROLL'S OWN CEILING (the one-row list the short
    // band the no-scroll keeps — the 24px row + the 14px header the
    // 38px content the visible band never out-grows)
    app.on_debug_options_scroll(-1000.0, 180);
    assert_eq!(
        app.debug_scroll, 0.0,
        "the filtered band the short list the floor keeps"
    );
    // THE PILL THE FILTERED ROW WEARS (the click the same tri-state
    // store — the searched list the chord consts' store keeps)
    let off = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgoff" && b.label == "8")
        .expect("the fps row's off pill the hit list wears")
        .clone();
    app.on_click(off.x + off.w / 2, off.y + 10);
    assert_eq!(
        app.debug_entry_states[8],
        DebugEntryStatus::Never,
        "the filtered row's pill the same store wears"
    );
    // THE FOCUS TRANSFER'S OWN LAW (the outside click the widget the
    // focus loses — the pill the next focusable the vanilla walk
    // arms, the caret the unfocused frame never draws)
    assert_eq!(
        app.debug_search_focus, false,
        "the pill click the box the focus retires"
    );
    // THE FOCUSED GATE'S OWN VERDICT (the unfocused box the chars
    // never banks — the canConsumeInput law the lanes keep)
    app.on_debug_options_char('z');
    assert_eq!(
        app.debug_search, "FPS",
        "the unfocused box the char refuses"
    );
    // THE BOX CLICK'S OWN RE-ARM (the inside click the setFocused(true)
    // serves — the rowWidth/3 width the gui lane passes, the typing
    // the re-focused box wears)
    assert!(
        app.on_debug_options_box_click(300, 25, 640),
        "the inside click the box the focus re-arms"
    );
    // THE BACKSPACE'S OWN WALK (the pops the word retires, the empty
    // query the WHOLE list keeps — the rebuild the biome row banks)
    for _ in 0..3 {
        app.on_debug_options_backspace();
    }
    assert_eq!(app.debug_search, "", "the pops the word retires");
    app.render(&mut fb, &mut assets);
    assert!(
        app.buttons
            .iter()
            .any(|b| b.id == "dbgoff" && b.label == "0"),
        "the empty query the whole list keeps"
    );
    app.on_debug_options_scroll(-1000.0, 180);
    assert_eq!(app.debug_scroll, 1032.0, "the whole band the ceiling wears");
    // THE CAP'S OWN LAW (the 50 chars the picker's own law keeps —
    // the 51st the responder never banks) + THE GATE'S OWN VERDICT
    // (the closed screen the chars never ride)
    for _ in 0..51 {
        app.on_debug_options_char('a');
    }
    assert_eq!(
        app.debug_search.chars().count(),
        50,
        "the cap the box keeps"
    );
    app.on_debug_options_key(); // the close arm the World serves
    app.on_debug_options_char('x');
    assert_eq!(
        app.debug_search.chars().count(),
        50,
        "the closed screen the gate keeps"
    );
}

#[test]
fn the_debug_search_box_the_click_focus_wears() {
    fn count_white(fb: &crate::raster::Framebuf, x0: i32, y0: i32, w: i32, h: i32) -> usize {
        let mut n = 0usize;
        for y in (y0.max(0) as usize)..((y0 + h).min(fb.h as i32) as usize) {
            for x in (x0.max(0) as usize)..((x0 + w).min(fb.w as i32) as usize) {
                let i = (y * fb.w + x) * 3;
                if fb.color[i] == 255 && fb.color[i + 1] == 255 && fb.color[i + 2] == 255 {
                    n += 1;
                }
            }
        }
        n
    }
    let mut app = App::new(3);
    app.enter_world();
    // the open arm the DebugOptions serves
    app.on_debug_options_key();
    // THE INITIAL FOCUS'S OWN SEAT (the vanilla init's setInitialFocus
    // the fresh screen arms — the typed word the caret the blink law
    // draws)
    assert_eq!(
        app.debug_search_focus, true,
        "the fresh screen the box the focus wears"
    );
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let mut assets = GuiAssets::open(assets_root());
    for c in "fps".chars() {
        app.on_debug_options_char(c);
    }
    assert_eq!(app.debug_search, "fps", "the focused box the word banks");
    // THE CARET'S OWN FOCUS GATE (the blink law ONLY the focused
    // frame draws — the SAME frame the two renders keep (the
    // scramble + the blink phase the identical render wears), the
    // focused band the caret's white wears, the unfocused the glyphs
    // alone)
    app.frame = 0; // the (frame/6)%2 phase the caret frame wears
    app.render(&mut fb, &mut assets);
    let focused = count_white(&fb, 214, 20, 213, 20);
    app.debug_search_focus = false;
    app.render(&mut fb, &mut assets);
    let unfocused = count_white(&fb, 214, 20, 213, 20);
    assert!(
        focused > unfocused,
        "the focused frame the caret's white wears"
    );
    // THE OUTSIDE CLICK'S OWN VERDICT (the FALSE the caller the
    // focus-transfer answers — the box the focus keeps OFF)
    assert!(
        !app.on_debug_options_box_click(30, 100, 640),
        "the outside click the FALSE the caller wears"
    );
    assert_eq!(
        app.debug_search_focus, false,
        "the outside click the focus keeps off"
    );
    // THE INSIDE CLICK'S OWN RE-ARM (the TRUE the consumed answer
    // wears — the typing the re-focused box wears)
    assert!(
        app.on_debug_options_box_click(300, 25, 640),
        "the inside click the TRUE wears"
    );
    assert_eq!(
        app.debug_search_focus, true,
        "the inside click the focus re-arms"
    );
    app.on_debug_options_char('x');
    assert_eq!(
        app.debug_search, "fpsx",
        "the re-focused box the char banks"
    );
    // THE CLOSED SCREEN'S OWN GATE (the non-debug screen the lane
    // never arms — the FALSE the focus walk keeps)
    app.on_debug_options_key(); // the close arm
    assert!(
        !app.on_debug_options_box_click(300, 25, 640),
        "the closed screen the walk refuses"
    );
}

/// THE DEBUG OPTIONS' OWN TAB CYCLE (the autopush/dev-173 walk — the
/// Screen keyPressed's 258 case the focus navigation serves, the
/// handleTabTraversal law the add-order seats wear, the wrap the
/// clearFocus re-walk the end arms answer, the inactive preset the
/// isActive refusal skips, the Enter the focused button's own press
/// serves, the focused render the isHoveredOrFocused sprite wears).
#[test]
fn the_debug_options_tab_cycle_the_box_the_pills_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_debug_options_key(); // the open arm the fresh screen wears
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    app.render(&mut fb, &mut assets);
    // THE FRESH OPEN'S OWN SEAT (the setInitialFocus(searchBox) law —
    // the box the flag wears, the button seats the None keeps)
    assert!(
        app.debug_search_focus,
        "the fresh open the box the focus wears"
    );
    assert_eq!(
        app.debug_focus, None,
        "the fresh open the button seats the None keeps"
    );
    // THE FORWARD CYCLE (the handleTabNavigation law — the add order
    // the traversal wears: the box the head, the DEFAULT preset the
    // ctor's own profile the INACTIVE seat the skip law answers, the
    // PERFORMANCE preset the next, the first row's pills the
    // off/overlay/always x-order the add banks)
    app.on_debug_options_tab(true);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::PresetPerf),
        "the inactive DEFAULT preset the skip law answers"
    );
    assert!(
        !app.debug_search_focus,
        "the box the flag the widget-transfer retires"
    );
    app.on_debug_options_tab(true);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 0 }),
        "the first row's Off pill the LEFTMOST seat wears"
    );
    app.on_debug_options_tab(true);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 1 }),
        "the In Overlay pill the middle seat keeps"
    );
    app.on_debug_options_tab(true);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 2 }),
        "the Always pill the RIGHTMOST seat keeps"
    );
    app.on_debug_options_tab(true);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 1, pill: 0 }),
        "the second row's Off pill the next seat wears"
    );
    // THE BACKWARD WALK (the Shift+Tab law — the seats the reverse
    // order answer, the off-the-head backward the TAIL seat the
    // Screen's clearFocus re-walk serves)
    app.on_debug_options_tab(false);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 2 }),
        "the backward walk the previous seat wears"
    );
    for _ in 0..4 {
        app.on_debug_options_tab(false);
    }
    assert!(
        app.debug_search_focus,
        "the backward walk the box the head seat re-arms"
    );
    assert_eq!(
        app.debug_focus, None,
        "the box seat the button field never re-answers"
    );
    // THE BACKWARD WRAP (the box's backward walk the TAIL seat serves
    // — the last visible row's Always pill the re-walk arms)
    app.on_debug_options_tab(false);
    let last_always = app
        .buttons
        .iter()
        .filter(|b| b.id == "dbgalways")
        .last()
        .expect("the visible rows the render bank keeps")
        .label
        .parse::<usize>()
        .expect("the carrier label the index wears");
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill {
            entry: last_always,
            pill: 2
        }),
        "the backward wrap the LAST visible pill arms"
    );
    // THE FORWARD WRAP (the off-the-tail forward the HEAD seat — the
    // clearFocus re-walk the box re-arms)
    app.on_debug_options_tab(true);
    assert!(
        app.debug_search_focus,
        "the forward wrap the box the head seat wears"
    );
    // THE OTHER PRESET'S OWN SKIP (the PERFORMANCE preset in use the
    // perf button the traversal never arms — the DEFAULT the skip's
    // own next seat answers)
    app.debug_preset_press(true, DebugFocusSeat::PresetPerf);
    app.render(&mut fb, &mut assets);
    assert!(
        !app.debug_search_focus && app.debug_focus == Some(DebugFocusSeat::PresetPerf),
        "the preset press the clicked seat arms"
    );
    app.debug_search_focus = true;
    app.debug_focus = None;
    app.on_debug_options_tab(true);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::PresetDflt),
        "the inactive PERFORMANCE preset the skip law answers"
    );
    // THE ENTER'S OWN ACTIVATION (the AbstractButton isSelection law —
    // the focused preset the keyboard press the same fold serves)
    app.on_debug_options_activate();
    assert_eq!(
        app.debug_profile,
        Some(false),
        "the focused DEFAULT preset the Enter press banks"
    );
    assert!(
        app.debug_focus == Some(DebugFocusSeat::PresetDflt),
        "the Enter press the current seat keeps"
    );
    // THE PILL'S OWN ENTER RIDE (the pill seat the activation the
    // setValue walk serves — the status the store banks, the profile
    // identity the manual touch breaks)
    app.debug_focus = Some(DebugFocusSeat::Pill { entry: 8, pill: 2 });
    app.debug_search_focus = false;
    app.on_debug_options_activate();
    assert_eq!(
        app.debug_entry_states[8],
        DebugEntryStatus::Always,
        "the focused pill the Enter press banks"
    );
    assert_eq!(
        app.debug_profile, None,
        "the manual touch the profile identity breaks"
    );
    assert!(
        !app.debug_search_focus,
        "the pill seat the box the flag keeps off"
    );
    // THE BOX SEAT'S OWN SILENCE (the EditBox the selection keys the
    // typing never serves — the Enter the box seat the nothing
    // answer keeps)
    app.debug_search_focus = true;
    app.debug_focus = None;
    app.on_debug_options_activate();
    assert!(
        app.debug_search_focus,
        "the box seat the Enter the silence keeps"
    );
    // THE FOCUSED RENDER'S OWN SPRITE (the isHoveredOrFocused law —
    // the focused pill the highlighted frame the pixel bank wears,
    // the unfocused the plain sprite the same rect keeps)
    app.debug_focus = Some(DebugFocusSeat::Pill { entry: 0, pill: 0 });
    app.debug_search_focus = false;
    app.render(&mut fb, &mut assets);
    let pill = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgoff" && b.label == "0")
        .expect("the first row the render bank keeps")
        .clone();
    let mut snap = |app: &mut App, focused: bool| -> Vec<[u8; 3]> {
        app.debug_focus = if focused {
            Some(DebugFocusSeat::Pill { entry: 0, pill: 0 })
        } else {
            None
        };
        app.render(&mut fb, &mut assets);
        let mut out = Vec::new();
        for y in pill.y..(pill.y + pill.h) {
            for x in pill.x..(pill.x + pill.w) {
                let i = (y as usize * fb.w + x as usize) * 3;
                out.push([fb.color[i], fb.color[i + 1], fb.color[i + 2]]);
            }
        }
        out
    };
    let focused_px = snap(&mut app, true);
    let plain_px = snap(&mut app, false);
    assert_ne!(
        focused_px, plain_px,
        "the focused pill the highlighted sprite wears"
    );
    // THE CLOSED SCREEN'S OWN GATE (the non-debug screen the Tab walk
    // never arms — the seat the close keeps)
    app.on_debug_options_key(); // the close
    let seat_before = app.debug_focus;
    app.on_debug_options_tab(true);
    assert_eq!(
        app.debug_focus, seat_before,
        "the closed screen the Tab the silence keeps"
    );
    assert!(
        !app.on_debug_options_box_click(300, 25, 640),
        "the closed screen the click the same refusal keeps"
    );
}

/// THE ARROW KEYS' OWN FOCUS RIDE (the autopush-240 walk — the Screen
/// keyPressed's 262-265 cases the ArrowNavigation events serve, the
/// nextFocusPathInDirection walk the geometric law answers: the
/// orthogonal-overlap gate the row band keeps, the trailing-edge
/// isAfter law the beyond candidates banks, the sort the nearest
/// trailing edge wears, the vague walk the no-overlap fallback the
/// nearest synthetic position arms, the box seat the cursor law the
/// silence keeps).
#[test]
fn the_debug_options_arrow_ride_the_geometric_walk_wears() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_debug_options_key(); // the open arm the fresh screen wears
    let root = crate::screens::tests::assets_root();
    let mut assets = GuiAssets::open(&root);
    let mut fb = crate::raster::Framebuf::new(640, 360);
    app.render(&mut fb, &mut assets);
    // THE BOX SEAT'S OWN SILENCE (the EditBox the arrow keys the
    // cursor law eats — the tableswitch 259..269 the four arrows the
    // box's own walk consumes, the focus the move never wears)
    assert!(
        app.debug_search_focus,
        "the fresh open the box the focus wears"
    );
    app.on_debug_options_arrow(1, 0);
    assert!(
        app.debug_search_focus,
        "the box seat the arrow the silence keeps"
    );
    // THE TAB'S OWN SEAT (the cycle walk the perf preset arms — the
    // ctor's own DEFAULT profile the inactive skip serves)
    app.on_debug_options_tab(true);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::PresetPerf),
        "the tab walk the perf preset arms"
    );
    // THE VAGUE WALK'S OWN RIDE (the RIGHT arrow the strict walk the
    // y-band refuses — the presets row the pills row the overlap
    // never shares — the no-overlap fallback the nearest synthetic
    // position arms: the first row's Off pill the (direction-bound,
    // orthogonal-center) distance law answers)
    app.on_debug_options_arrow(1, 0);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 0 }),
        "the RIGHT arrow the vague walk the nearest pill arms"
    );
    // THE STRICT WALK'S OWN ROW BAND (the RIGHT arrow the same-row
    // band the y-overlap answers — the trailing-edge law the overlay
    // pill's left edge beyond wears, the nearest trailing the sort
    // arms)
    app.on_debug_options_arrow(1, 0);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 1 }),
        "the RIGHT arrow the strict walk the overlay pill arms"
    );
    app.on_debug_options_arrow(1, 0);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 2 }),
        "the RIGHT arrow the Always pill the nearest trailing arms"
    );
    // THE RIGHTMOST'S OWN WALL (the RIGHT arrow the no-candidate walk
    // the silence keeps — the always pill the rightmost widget the
    // beyond set never wears)
    app.on_debug_options_arrow(1, 0);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 2 }),
        "the rightmost seat the arrow the silence keeps"
    );
    // THE LEFT'S OWN WALK (the trailing edge the right edges answer —
    // the overlay pill the nearest beyond wears, the walk the seats
    // the reverse order serves)
    app.on_debug_options_arrow(-1, 0);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 1 }),
        "the LEFT arrow the overlay pill the nearest right edge arms"
    );
    app.on_debug_options_arrow(-1, 0);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 0 }),
        "the LEFT arrow the Off pill the walk retires"
    );
    // THE LEFT'S OWN VAGUE RIDE (the strict walk the row band refuses
    // — the no-overlap fallback the nearest synthetic position arms:
    // the PERFORMANCE preset the (right-edge, center) distance law
    // answers [the dflt/box the farther centers the same law never
    // wears])
    app.on_debug_options_arrow(-1, 0);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::PresetPerf),
        "the LEFT arrow the vague walk the nearest preset arms"
    );
    // THE UP'S OWN STRICT RIDE (the vertical direction the x-band —
    // the perf seat the box's own x-span the overlap answers: the
    // trailing-edge law the box's top beyond wears, the dflt preset
    // the x-gap the same band never shares; the BOX seat the flag +
    // the None field the apply law wears)
    app.on_debug_options_arrow(0, -1);
    assert!(
        app.debug_search_focus,
        "the UP arrow the strict walk the box's x-span arms"
    );
    assert_eq!(
        app.debug_focus, None,
        "the box seat the field the None law keeps"
    );
    // THE BOX SEAT'S OWN ARROW SILENCE (the EditBox the arrow keys the
    // cursor law eats — the tableswitch 259..269 the four arrows the
    // box's own walk consumes, the focus the move never wears: the
    // DOWN arrow the box the same silence keeps)
    app.on_debug_options_arrow(0, 1);
    assert!(
        app.debug_search_focus,
        "the box seat the DOWN arrow the same silence keeps"
    );
    // THE TAB'S OWN RE-ARM (the cycle walk the perf preset the seat
    // re-banks — the inactive dflt the skip law serves)
    app.on_debug_options_tab(true);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::PresetPerf),
        "the tab walk the perf preset the seat re-banks"
    );
    // THE DOWN'S OWN VAGUE RIDE (the perf seat the pills' column
    // the x-overlap refuses — the fallback the nearest pair arms: the
    // first row's Off pill the (bottom-edge, center) distance law
    // answers)
    app.on_debug_options_arrow(0, 1);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 0 }),
        "the DOWN arrow the vague walk the nearest pill arms"
    );
    // THE UP'S OWN VAGUE RIDE (the pill seat the x-band the strict
    // walk refuses — the presets/box the x-gap the same band never
    // shares — the fallback the nearest (top-edge, center-x) pair:
    // the perf preset the nearest center keeps [the box the farther
    // center the same law never wears])
    app.on_debug_options_arrow(0, -1);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::PresetPerf),
        "the UP arrow the vague walk the nearest preset arms"
    );
    // THE DOWN'S OWN STRICT RIDE (the pill column the x-overlap band
    // the whole column shares — the nearest top edge the next row's
    // own pill wears)
    app.on_debug_options_arrow(0, 1);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 0, pill: 0 }),
        "the DOWN arrow the pill the same seat re-arms"
    );
    app.on_debug_options_arrow(0, 1);
    assert_eq!(
        app.debug_focus,
        Some(DebugFocusSeat::Pill { entry: 1, pill: 0 }),
        "the DOWN arrow the next row's Off pill the nearest top wears"
    );
    // THE CLOSED SCREEN'S OWN GATE (the non-debug screen the arrow
    // walk the silence keeps — the seat the close never re-arms)
    app.on_debug_options_key(); // the close
    let seat_before = app.debug_focus;
    app.on_debug_options_arrow(0, 1);
    assert_eq!(
        app.debug_focus, seat_before,
        "the closed screen the arrow the silence keeps"
    );
}
// THE DEBUG PROFILE FILE'S OWN ROUND TRIP (javap 26.2 the
// DebugScreenEntryList load/save walks the persistence law — the
// debug-profile.json sibling the options.txt game dir wears: the
// profile word the preset's own map the restart outlives, the custom
// map the manual touches the ALL-45 rows the file carries, the
// MUTUAL EXCLUSION the save's own shape speaks — the profile present
// the custom never writes, the custom the profile key never wears)
#[test]
fn the_debug_profile_file_the_statuses_restart_outlives() {
    let dir = std::env::temp_dir().join(format!("mc2rs_debug_profile_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("debug-profile.json");
    let _ = std::fs::remove_file(&path);
    let mut app = App::new(262);
    app.enter_world();
    app.debug_profile_file = Some(path.clone());
    // THE MISSING FILE (vanilla load pc 0-21 — the isFile gate the
    // resetToProfile(DEFAULT) arm answers: the nine IN_OVERLAY rows
    // the fresh boot wears, the file the lane never writes)
    app.debug_profile_load();
    assert_eq!(
        app.debug_entry_states[30], // the 3d_crosshair renderer row
        crate::screens::DebugEntryStatus::Overlay,
        "the default preset the crosshair row wears"
    );
    assert_eq!(
        app.debug_entry_states[8], // the fps row
        crate::screens::DebugEntryStatus::Overlay,
        "the default preset the fps row wears"
    );
    assert_eq!(
        app.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY],
        crate::screens::DebugEntryStatus::Never,
        "the unpreset row the NEVER keeps"
    );
    assert_eq!(
        app.debug_profile,
        Some(false),
        "the default the identity wears"
    );
    assert!(!path.exists(), "the missing file the save never spends");
    // THE PILL TOUCH'S OWN SAVE RIDE (the real render the buttons
    // arm, the coral pill the flip the file the same moment writes
    // — the fps row the preset's Overlay seat the ALWAYS pill
    // raises, the manual touch the profile identity breaks)
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let mut assets = GuiAssets::open(assets_root());
    app.on_debug_options_key(); // the open arm the DebugOptions serves
    app.render(&mut fb, &mut assets);
    let always_fps = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgalways" && b.label == "8")
        .expect("the fps row the always pill wears")
        .clone();
    app.on_click(always_fps.x + always_fps.w / 2, always_fps.y + 10);
    assert_eq!(
        app.debug_entry_states[8],
        crate::screens::DebugEntryStatus::Always,
        "the coral pill the ALWAYS_ON banks"
    );
    assert_eq!(app.debug_profile, None, "the touch the identity breaks");
    let text = std::fs::read_to_string(&path).expect("the pill the file writes");
    let root: serde_json::Value = serde_json::from_str(&text).expect("the file the JSON wears");
    assert!(
        root.get("profile").is_none(),
        "the custom shape the profile key never wears"
    );
    assert_eq!(
        root["custom"]["fps"], "ALWAYS_ON",
        "the touched row the vanilla word banks"
    );
    assert_eq!(
        root["custom"]["biome"], "NEVER",
        "the untouched row the NEVER word keeps"
    );
    assert_eq!(
        root["custom"].as_object().map(|m| m.len()),
        Some(45),
        "the whole bank the custom map carries"
    );
    assert!(
        !text.contains('\n'),
        "the compact line the JsonElement.toString law speaks"
    );
    // THE RESTART (the fresh boot the custom map's truth wears)
    let mut app2 = App::new(262);
    app2.debug_profile_file = Some(path.clone());
    app2.debug_profile_load();
    assert_eq!(
        app2.debug_entry_states[8],
        crate::screens::DebugEntryStatus::Always,
        "the touched seat the restart outlives"
    );
    assert_eq!(
        app2.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY],
        crate::screens::DebugEntryStatus::Never,
        "the untouched seat the NEVER keeps"
    );
    assert_eq!(
        app2.debug_profile, None,
        "the custom shape the identity keeps"
    );
    // THE CHORD'S OWN RIDE (the F3+G toggleStatus the file the same
    // flip moment writes — the close the DebugOptions the World
    // serves first, the chord the world's own lane wears)
    app.on_debug_options_key(); // the close arm
    app.on_chunk_borders_key();
    assert_eq!(
        app.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY],
        crate::screens::DebugEntryStatus::Always,
        "the chord the ALWAYS_ON raises"
    );
    let text2 = std::fs::read_to_string(&path).expect("the chord the file writes");
    let root2: serde_json::Value =
        serde_json::from_str(&text2).expect("the rewrite the JSON keeps");
    assert_eq!(
        root2["custom"]["chunk_borders"], "ALWAYS_ON",
        "the chord's own word the file wears"
    );
    // THE PRESET BUTTON'S OWN RIDE (the dbgprofperf click the
    // PERFORMANCE identity the preset map wears — the fps ALWAYS_ON
    // + the file the profile word ALONE carries)
    app.on_debug_options_key(); // the re-open
    app.render(&mut fb, &mut assets);
    let perf = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgprofperf")
        .expect("the performance row wears")
        .clone();
    app.on_click(perf.x + perf.w / 2, perf.y + 10);
    assert_eq!(
        app.debug_profile,
        Some(true),
        "the preset the PERFORMANCE banks"
    );
    assert!(app.fps_charts, "the performance arm the charts flag wears");
    let text3 = std::fs::read_to_string(&path).expect("the preset the file writes");
    let root3: serde_json::Value =
        serde_json::from_str(&text3).expect("the rewrite the JSON keeps");
    assert_eq!(
        root3["profile"], "performance",
        "the preset's own word the file wears"
    );
    assert!(
        root3.get("custom").is_none(),
        "the profile shape the custom key never wears"
    );
    // THE PRESET RESTART (the profile word the resetToProfile the
    // fresh boot answers — the preset rows the map's own words wear)
    let mut app6 = App::new(262);
    app6.debug_profile_file = Some(path.clone());
    app6.debug_profile_load();
    assert_eq!(
        app6.debug_entry_states[8],
        crate::screens::DebugEntryStatus::Always,
        "the performance fps the ALWAYS_ON restart wears"
    );
    assert_eq!(
        app6.debug_entry_states[29], // the tps row
        crate::screens::DebugEntryStatus::Overlay,
        "the performance tps the IN_OVERLAY restart wears"
    );
    assert_eq!(
        app6.debug_profile,
        Some(true),
        "the profile word the restart outlives"
    );
    // THE MALFORMED FILE (vanilla pc 122-146 — the catch the DEFAULT
    // reset + the save re-write answers)
    std::fs::write(&path, "not json at all").expect("the probe the file writes");
    let mut app3 = App::new(262);
    app3.debug_profile_file = Some(path.clone());
    app3.debug_profile_load();
    assert_eq!(
        app3.debug_entry_states[8],
        crate::screens::DebugEntryStatus::Overlay,
        "the malformed file the DEFAULT preset keeps"
    );
    assert_eq!(
        app3.debug_profile,
        Some(false),
        "the malformed file the identity keeps"
    );
    let text4 = std::fs::read_to_string(&path).expect("the catch the save re-writes");
    let root4: serde_json::Value =
        serde_json::from_str(&text4).expect("the re-write the JSON keeps");
    assert_eq!(
        root4["profile"], "default",
        "the re-write the default word wears"
    );
    // THE CUSTOM MAP'S OWN READ (the profile-absent shape the
    // resetStatuses walk serves — the unknown id the skip law keeps,
    // the unknown word the seat never touches)
    std::fs::write(
        &path,
        r#"{"custom":{"nonsense":"ALWAYS_ON","fps":"SOMETIMES","tps":"ALWAYS_ON"}}"#,
    )
    .expect("the probe the file writes");
    let mut app4 = App::new(262);
    app4.debug_profile_file = Some(path.clone());
    app4.debug_profile_load();
    assert_eq!(
        app4.debug_entry_states[29], // the tps row
        crate::screens::DebugEntryStatus::Always,
        "the known row the ALWAYS_ON word wears"
    );
    assert_eq!(
        app4.debug_entry_states[8],
        crate::screens::DebugEntryStatus::Never,
        "the unknown word the seat keeps"
    );
    assert_eq!(
        app4.debug_profile, None,
        "the profile-absent shape the identity keeps"
    );
    // THE NONE SEAT the silent keep answers (the headless boot the
    // determinism bank never spends)
    let mut app5 = App::new(262);
    app5.debug_profile_save();
    app5.debug_profile_load();
    assert_eq!(
        app5.debug_entry_states[8],
        crate::screens::DebugEntryStatus::Never,
        "the none seat the defaults keep"
    );
}

// THE PROFILES' OWN PRESET WALK (javap 26.2 the DebugScreenEntries
// clinit the PROFILES map banks — the DEFAULT arm the nine
// IN_OVERLAY rows speak, the PERFORMANCE arm the fps ALWAYS_ON word
// the perf rows wear; the resetToProfile walk the whole bank the
// sparse-map law zeroes, the preset rows the known seats wear)
#[test]
fn the_debug_profiles_the_presets_the_vanilla_maps_wear() {
    let mut app = App::new(3);
    app.enter_world();
    app.on_debug_options_key(); // the open arm
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let mut assets = GuiAssets::open(assets_root());
    // THE PERFORMANCE BUTTON'S OWN WALK FIRST (the fresh ctor the
    // DEFAULT identity wears — the DEFAULT button the isUsingProfile
    // law DISABLES, the dbgprofperf click the OTHER arm the preset
    // map serves)
    app.render(&mut fb, &mut assets);
    let perf0 = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgprofperf")
        .expect("the performance row wears")
        .clone();
    app.on_click(perf0.x + perf0.w / 2, perf0.y + 10);
    assert_eq!(
        app.debug_profile,
        Some(true),
        "the button the PERFORMANCE identity banks"
    );
    assert!(app.fps_charts, "the performance arm the charts flag wears");
    assert_eq!(
        app.debug_entry_states[8], // the fps row
        crate::screens::DebugEntryStatus::Always,
        "the performance preset the fps ALWAYS_ON wears"
    );
    assert_eq!(
        app.debug_entry_states[29], // the tps row
        crate::screens::DebugEntryStatus::Overlay,
        "the performance preset the tps IN_OVERLAY wears"
    );
    assert_eq!(
        app.debug_entry_states[10], // the gpu_utilization row
        crate::screens::DebugEntryStatus::Overlay,
        "the performance preset the gpu row wears"
    );
    assert_eq!(
        app.debug_entry_states[30], // the 3d_crosshair row
        crate::screens::DebugEntryStatus::Never,
        "the crosshair row the performance preset retires"
    );
    // THE DEFAULT BUTTON'S OWN WALK (the dbgprofdflt click the
    // PROFILES map's DEFAULT arm the nine IN_OVERLAY rows serves —
    // the identity flip the button's active law re-arms)
    app.render(&mut fb, &mut assets);
    let dflt = app
        .buttons
        .iter()
        .find(|b| b.id == "dbgprofdflt")
        .expect("the default row wears")
        .clone();
    app.on_click(dflt.x + dflt.w / 2, dflt.y + 10);
    assert_eq!(
        app.debug_profile,
        Some(false),
        "the button the DEFAULT identity banks"
    );
    assert!(!app.fps_charts, "the default arm the charts flag keeps off");
    for (idx, id) in [
        (30, "3d_crosshair"),
        (9, "game_version"),
        (29, "tps"),
        (8, "fps"),
        (20, "memory"),
        (28, "system_specs"),
        (22, "player_position"),
        (23, "player_section_position"),
        (25, "simple_performance_impactors"),
    ] {
        assert_eq!(
            app.debug_entry_states[idx],
            crate::screens::DebugEntryStatus::Overlay,
            "the default preset the {} row the IN_OVERLAY wears",
            id
        );
    }
    assert_eq!(
        app.debug_entry_states[crate::screens::CHUNK_BORDERS_ENTRY],
        crate::screens::DebugEntryStatus::Never,
        "the unpreset row the NEVER keeps"
    );
    assert_eq!(
        app.debug_entry_states[1], // the biome row
        crate::screens::DebugEntryStatus::Never,
        "the biome row the default preset keeps off"
    );
    assert_eq!(
        app.debug_entry_states[8], // the fps row
        crate::screens::DebugEntryStatus::Overlay,
        "the default preset the fps row the IN_OVERLAY wears"
    );
}

// THE OPERATOR ITEMS TAB'S OWN PERMISSION GATE (javap 26.2 the
// CreativeModeInventoryScreen hasPermissions walk the conjunction law
// — the canUseGameMasterBlocks ask the permission level 2 answers,
// the displayOperatorCreativeTab ctor arg the operatorItemsTab
// option's word carries; the strip's tabs() list the shouldDisplay
// hasAnyItems law the EMPTY palette the hidden seat speaks, the
// getTabX the tab's own static column the inventory corner the
// re-flow never wears, the lambda$bootstrap$33 displayItems the
// twelve rows the hasPermissions gate accepts)
#[test]
fn the_operator_items_tab_the_permission_gate_wears() {
    fn rect_sig(fb: &crate::raster::Framebuf, x0: i32, y0: i32, w: i32, h: i32) -> u64 {
        let mut hash: u64 = 1469598103934665603;
        for y in (y0.max(0) as usize)..((y0 + h).min(fb.h as i32) as usize) {
            for x in (x0.max(0) as usize)..((x0 + w).min(fb.w as i32) as usize) {
                let i = (y * fb.w + x) * 3;
                hash = (hash ^ fb.color[i] as u64).wrapping_mul(1099511628211);
                hash = (hash ^ fb.color[i + 1] as u64).wrapping_mul(1099511628211);
                hash = (hash ^ fb.color[i + 2] as u64).wrapping_mul(1099511628211);
            }
        }
        hash
    }
    let mut app = App::new(3);
    app.enter_world();
    // THE OWNER'S OWN LEVEL (the offline world the integrated server's
    // owner law the level 4 wears) + THE OPTION'S FRESH SEAT (the
    // createBoolean default the FALSE keeps) — the gate the OPTION's
    // half the closed verdict wears
    assert_eq!(
        app.permission_level, 4,
        "the offline world the owner's level wears"
    );
    assert!(
        !app.operator_items_tab,
        "the fresh boot the option keeps off"
    );
    assert!(
        !app.creative_operator_gate(),
        "the closed gate the option's half keeps"
    );
    // THE CLOSED GATE'S OWN WALKS (the tabs() list the hidden seat
    // never banks): the strip the op tab never paints, the hit-test
    // the seat never answers, the select the silence keeps
    let mut fb = crate::raster::Framebuf::new(640, 360);
    let mut assets = GuiAssets::open(assets_root());
    app.open_creative_picker();
    app.render(&mut fb, &mut assets);
    let (px, py) = app.creative_panel;
    let op_seat = creative_tab_origin(&CREATIVE_TABS[12], 195, 136);
    let inv_seat = creative_tab_origin(&CREATIVE_TABS[13], 195, 136);
    assert_eq!(
        op_seat,
        (142, 132),
        "the op tab the bottom column-5 seat wears"
    );
    assert_eq!(
        inv_seat,
        (169, 132),
        "the inventory tab the corner column-6 seat wears"
    );
    let sig_op_closed = rect_sig(&fb, px + op_seat.0, py + op_seat.1, 26, 32);
    let sig_inv_closed = rect_sig(&fb, px + inv_seat.0, py + inv_seat.1, 26, 32);
    assert!(
        app.creative_tab_at(px + op_seat.0 + 13, py + op_seat.1 + 16)
            .is_none(),
        "the closed gate the seat the hit-test never answers"
    );
    app.select_creative_tab(12);
    assert_eq!(
        app.creative_tab, 0,
        "the closed gate the selection never banks"
    );
    // THE OPTION'S OWN HALF (the TRUE word the gate the option's half
    // opens — the strip the op tab the unselected sprite paints, the
    // INVENTORY tab the corner seat the SAME pixels keep [the getTabX
    // the static column the re-flow never wears])
    app.operator_items_tab = true;
    assert!(
        app.creative_operator_gate(),
        "the owner level 4 + the true option the gate wears"
    );
    app.render(&mut fb, &mut assets);
    let sig_op_open = rect_sig(&fb, px + op_seat.0, py + op_seat.1, 26, 32);
    let sig_inv_open = rect_sig(&fb, px + inv_seat.0, py + inv_seat.1, 26, 32);
    assert_ne!(
        sig_op_closed, sig_op_open,
        "the open gate the op sprite paints"
    );
    assert_eq!(
        sig_inv_closed, sig_inv_open,
        "the inventory tab the corner seat the re-flow never wears"
    );
    assert_eq!(
        app.creative_tab_at(px + op_seat.0 + 13, py + op_seat.1 + 16),
        Some(12),
        "the open gate the seat the hit-test answers"
    );
    // THE SELECT'S OWN PALETTE (the selectTab walk the op rows the
    // TWELVE-row walk refills — the lambda$bootstrap$33's own order
    // the registry's whole walk the palette wears)
    app.select_creative_tab(12);
    assert_eq!(app.creative_tab, 12, "the open gate the selection banks");
    let painted: Vec<&str> = app
        .creative_items
        .iter()
        .filter(|&&(id, count)| count > 0 && id != 0)
        .map(|&(id, _)| {
            mc_registry::generated::item_order::ITEM_WIRE_ORDER
                .get(id as usize)
                .copied()
                .unwrap_or("")
        })
        .collect();
    assert_eq!(
        painted,
        vec![
            "command_block",
            "chain_command_block",
            "repeating_command_block",
            "command_block_minecart",
            "jigsaw",
            "structure_block",
            "structure_void",
            "barrier",
            "debug_stick",
            "test_instance_block",
            "test_block",
            "light",
        ],
        "the op palette the twelve rows the generator's own order wears"
    );
    // THE RE-OPEN'S OWN RECONCILE (the stale hidden seat the reset
    // retires — the bounds law the gated seat the same answer wears,
    // the open gate the persisted seat the re-open keeps)
    app.operator_items_tab = false;
    app.open_creative_picker();
    assert_eq!(
        app.creative_tab, 0,
        "the stale hidden seat the re-open retires"
    );
    app.operator_items_tab = true;
    app.select_creative_tab(12);
    app.open_creative_picker();
    assert_eq!(
        app.creative_tab, 12,
        "the open gate the persisted seat the re-open keeps"
    );
    // THE PERMISSION'S OWN HALF (the canUseGameMasterBlocks ask the
    // level 2 the boundary answers — the level 1 the closed verdict
    // wears, the level 0 the fresh join's un-opped word keeps)
    app.permission_level = 1;
    assert!(
        !app.creative_operator_gate(),
        "the level 1 the canUseGameMasterBlocks ask refuses"
    );
    assert!(
        app.creative_tab_at(px + op_seat.0 + 13, py + op_seat.1 + 16)
            .is_none(),
        "the level 1 the seat the hit-test never answers"
    );
    app.permission_level = 2;
    assert!(
        app.creative_operator_gate(),
        "the level 2 the boundary the ask answers"
    );
    app.permission_level = 0;
    assert!(
        !app.creative_operator_gate(),
        "the un-opped level the gate keeps closed"
    );
}

/// THE PERMISSION ECHO'S OWN INSTALLER (javap 26.2
/// `LocalPlayer.setPermissions` — the entity-event 24..28 word the
/// wire lane serves): the networked world the level re-banks (the
/// byte-24 fold the GAMEMASTER/OWNER words speak), the pre-echo
/// window the fresh join's 0 keeps, the OFFLINE world's owner-4 word
/// the lane NEVER touches (the enter arm the authority keeps), the
/// gate the re-banked level the next read re-answers.
#[test]
fn the_permission_echo_the_wire_word_the_installer_banks() {
    let mut app = App::new(3);
    // THE NETWORKED JOIN'S OWN FRESH SEAT (the ctor's 0 the
    // un-opped word keeps) — the flag the wire world's own law wears
    app.world_is_network = true;
    app.permission_level = 0;
    app.operator_items_tab = true;
    assert!(
        !app.creative_operator_gate(),
        "the fresh join the un-opped word keeps the gate closed"
    );
    // THE PRE-ECHO WINDOW: the None refresh the bank never wears
    // (the idempotent silence the pre-packet session keeps)
    app.live_permission_level_updated(None);
    assert_eq!(
        app.permission_level, 0,
        "the None refresh the pre-echo window keeps"
    );
    // THE OP TURN'S OWN BYTE (the /op the byte-26 the GAMEMASTER
    // word speaks): the bank the level-2 word wears — the
    // canUseGameMasterBlocks ask the conjunction's own half
    // re-answers, the gate the option's TRUE word the OPEN verdict
    // wears
    app.live_permission_level_updated(Some(2));
    assert_eq!(
        app.permission_level, 2,
        "the op turn the GAMEMASTER level banks"
    );
    assert!(
        app.creative_operator_gate(),
        "the re-banked level the gate's own half re-answers"
    );
    // THE OWNER'S RUNG (the byte-28 the level-4 word speaks): the
    // newest packet the bank replaces
    app.live_permission_level_updated(Some(4));
    assert_eq!(
        app.permission_level, 4,
        "the owner rung the level-4 word banks"
    );
    // THE IDEMPOTENT RE-SHIP: the unchanged word the same bank keeps
    // (the acmp silence the vanilla putfield wears)
    app.live_permission_level_updated(Some(4));
    assert_eq!(
        app.permission_level, 4,
        "the idempotent re-ship the same bank keeps"
    );
    // THE DEOP TURN'S OWN BYTE (the /deop the byte-24 the
    // NO_PERMISSIONS word speaks): the gate the level-0 word the
    // closed verdict re-wears
    app.live_permission_level_updated(Some(0));
    assert_eq!(
        app.permission_level, 0,
        "the deop turn the NO_PERMISSIONS level banks"
    );
    assert!(
        !app.creative_operator_gate(),
        "the de-banked level the gate keeps closed"
    );
    // THE OFFLINE WORLD'S OWN GUARD (the enter arm the owner-4 word
    // the authority keeps): the lane the wire world ALONE wears —
    // the offline session the echo never touches
    app.world_is_network = false;
    app.permission_level = 4;
    app.live_permission_level_updated(Some(0));
    assert_eq!(
        app.permission_level, 4,
        "the offline world the owner-4 word the lane never touches"
    );
}

/// THE SCREEN-REACTION'S OWN WALK (javap 26.2 `LocalPlayer.setPermissions`
/// the TWO lanes' second — the 237 probe banks: the current screen the
/// `HasGamemasterPermissionReaction` ask answers, the
/// `onGamemasterPermissionChanged` fold the LIVE re-arm speaks): the
/// OPTIONS family the fresh-ctor self-recreate the setScreen walk
/// re-opens (the removed-moment the save the open() head speaks, the
/// fresh ctor the drag/buttons state re-arms), the CREATIVE picker the
/// tryRefreshInvalidatedTabs law the containerTick rides — the deop'd
/// gate the stale op seat the building-blocks re-select retires, the
/// category seats the packet never disturbs, the None refresh the
/// pre-echo window the fold never speaks, the offline world the lane
/// never wears.
#[test]
fn the_permission_echo_the_screen_reaction_the_live_rearm_wears() {
    let mut app = App::new(3);
    app.world_is_network = true;
    app.permission_level = 0;
    app.operator_items_tab = true;
    // THE OPTIONS SEAT'S OWN RECREATE (the fresh-ctor self-recreate —
    // the stale drag arm the mid-screen bank carries, the recreate the
    // fresh ctor's false answer wears, the buttons the fresh list
    // keeps)
    app.open(ScreenKind::Options);
    app.fov_drag = true;
    app.live_permission_level_updated(Some(2));
    assert_eq!(
        app.permission_level, 2,
        "the op turn the GAMEMASTER level banks"
    );
    assert_eq!(
        app.screen,
        ScreenKind::Options,
        "the recreate the same seat wears"
    );
    assert!(!app.fov_drag, "the fresh ctor the stale drag arm discards");
    assert!(
        app.buttons.is_empty(),
        "the fresh ctor the button list re-arms"
    );
    // THE CREATIVE PICKER'S OWN LIVE SEAT (the open gate the op seat
    // the palette banks — the GAMEMASTER re-ship the same seat keeps)
    app.open_creative_picker();
    app.select_creative_tab(12);
    assert_eq!(app.creative_tab, 12, "the open gate the op seat banks");
    app.live_permission_level_updated(Some(2));
    assert_eq!(
        app.creative_tab, 12,
        "the idempotent re-ship the open gate keeps"
    );
    // THE DEOP TURN'S OWN RETIRE (the byte-24 the NO_PERMISSIONS word
    // speaks — the gate the closed verdict re-wears, the stale op seat
    // the invalid law the building-blocks re-select retires: the full
    // selectTab walk the search clear + the source refill + the fresh
    // scroll tail wears)
    app.live_permission_level_updated(Some(0));
    assert_eq!(
        app.creative_tab, 0,
        "the deop'd gate the stale op seat retires"
    );
    assert!(
        app.creative_search.is_empty(),
        "the re-select the search clear keeps"
    );
    assert_eq!(
        app.creative_items, app.creative_source,
        "the re-select the source list refills"
    );
    assert!(
        !app.creative_operator_gate(),
        "the de-banked level the gate keeps closed"
    );
    // THE RE-OPEN'S OWN LAW (the retired seat the bounds arm the same
    // answer wears — the persisted seat 0 the re-open keeps)
    app.open_creative_picker();
    assert_eq!(app.creative_tab, 0, "the retired seat the re-open keeps");
    // THE CATEGORY SEAT'S OWN IMMUNITY (the invalid law the op seat
    // ALONE answers — the deop'd packet the category selection never
    // disturbs)
    app.live_permission_level_updated(Some(2));
    app.open_creative_picker();
    app.select_creative_tab(6);
    assert_eq!(app.creative_tab, 6, "the category seat banks");
    app.live_permission_level_updated(Some(0));
    assert_eq!(
        app.creative_tab, 6,
        "the category seat the deop never disturbs"
    );
    // THE NONE REFRESH'S OWN SILENCE (the pre-echo window the fold
    // never speaks — the packet-less call the setPermissions body
    // never wears, the stale drag arm the same keep)
    app.open(ScreenKind::Options);
    app.fov_drag = true;
    app.live_permission_level_updated(None);
    assert!(
        app.fov_drag,
        "the None refresh the screen-reaction never speaks"
    );
    // THE OFFLINE WORLD'S OWN GUARD (the enter arm the owner-4 word
    // the authority keeps — the lane the wire world ALONE wears, the
    // reaction fold the offline session never speaks)
    let mut off = App::new(3);
    off.enter_world();
    off.open(ScreenKind::Options);
    off.fov_drag = true;
    off.live_permission_level_updated(Some(0));
    assert_eq!(
        off.permission_level, 4,
        "the offline owner-4 word the enter arm keeps"
    );
    assert!(
        off.fov_drag,
        "the offline world the reaction fold never speaks"
    );
    assert_eq!(
        off.screen,
        ScreenKind::Options,
        "the offline seat the recreate never wears"
    );
}

//! THE OPTIONS FILE'S OWN LAW (the 26.2 jar's own facts the probes bank —
//! scripts/jar_work/options_save_262.py the `save()`/`load()` walks,
//! options_file_law_262.py the codec lanes, options_keys5_262.py the ldc
//! key strings, options_concat_262.py + options_indy2_262.py the BSM126
//! recipe, options_keys3_262.py the UnitDouble/IntRange codecs):
//! - THE FILE THE UTF-8 TEXT THE `key:value` LINES KEEP (the Options$3
//!   writePrefix walk — `print(key); print(':')` the bipush 58 speaks, the
//!   value the println wears); the FIRST line the `version:{dataVersion}`
//!   the SharedConstants walk prints (the jar's version.json `world_version`
//!   the dataVersion field keeps — 4903 the 26.2 answer).
//! - THE KEYS THE processOptions LANE PASSES (the ldc strings resolved):
//!   `mouseSensitivity` the sensitivity field wears (the
//!   OptionInstance$UnitDouble the doubleRange(0,1) codec — the raw double
//!   the line keeps, the default 0.5 the clinit's ldc2_w wears), `fov` the
//!   IntRange(30, 110) keeps (the codec the `Codec.intRange(30, 111)` build
//!   speaks — the RAW INT the line stores, the default 70 the bipush wears;
//!   the legacy `fov:0.0` the pre-1.19 file wrote NEVER returns),
//!   `soundCategory_master` the SoundSource loop's BSM126 recipe
//!   `'soundCategory_' + name` builds (the createSoundSliderOptionInstance
//!   walk the UnitDouble default 1.0 wears — the dconst_1 the map factory
//!   speaks), `pauseOnLostFocus` the boolean row the round-trip wears
//!   (the "true"/"false" the codec vocabulary), `fovEffectScale` the
//!   AccessibilityOptionsScreen row the UnitDouble.xmap slider wears
//!   (the doubleRange(0.0, 1.0) codec the range keeps — the BOOL
//!   alternative the codec NEVER carries, the strict double the decode
//!   answers, the default 1.0 the dconst_1 banks; the row the
//!   fov_scale_tail_262.py probe the Camera.tickFov read passes into
//!   getFieldOfViewModifier's ZF pair the tail's Mth.lerp speaks),
//!   `chatOpacity` the chat text's own row the array's index-6 seat
//!   wears (chatopacity_178.py the registration + the consumer pair
//!   bank — the UnitDouble slider the default 1.0 the dconst_1 the
//!   Options ctor pcs 741-818 speaks, the noTooltip lane the
//!   registration's own arm keeps, the strict double the decode
//!   answers; the ChatComponent's text-alpha base the consumer walk
//!   keeps — the extractRenderState fstore-12 law the
//!   `chatOpacity * 0.9f + 0.1f` composition speaks, the
//!   ChatComponent$1.accept `fload_3 * val$textOpacity` pair the
//!   per-line fade the glyph alpha multiplies wears),
//!   `textBackgroundOpacity` the chat backgrounds' own row the
//!   array's index-4 seat wears (textbg_262.py + textbg_262v2.py the
//!   registration + the consumer pair bank — the UnitDouble slider
//!   the default 0.5 the `ldc2_w double 4602678819172646912` bits
//!   speak, 0x3FE0000000000000 the IEEE-754 answer; the noTooltip
//!   lane the registration's own arm keeps, the strict double the
//!   decode answers, the ChatComponent's per-line fill the consumer
//!   walk keeps; the file row the visit-order adjacency the
//!   processOptions walk keeps — chatOpacity the ldc@480 the
//!   textBackgroundOpacity the ldc@506 the
//!   seat before answers), `backgroundForChatOnly` the chat band's
//!   own scope row the array's index-5 seat wears (chatbgonly_179.py
//!   + 179b-e the registration + the consumer pair bank — the raw
//!   ctor the BOOLEAN_VALUES value set the default TRUE the iconst_1
//!   pcs 2092-2120 speaks, the noTooltip lane the registration's own
//!   arm keeps, the bool decode the "true"/"false" vocabulary the
//!   pauseOnLostFocus row the same law wears; the file row the
//!   visit-order adjacency the processOptions walk keeps — the
//!   textBackgroundOpacity the ldc@506 the backgroundForChatOnly the
//!   ldc@523 the seat AFTER answers, the hideServerAddress@537 the
//!   next row the walk banks; the consumer the ChatScreen's own
//!   input band the `getBackgroundColor(0x80000000)` call speaks —
//!   the (I)I law the `backgroundForChatOnly ? raw :
//!   ARGB.colorFromFloat(textbg,0,0,0)` pair keeps, the caption the
//!   lambda$new$76 law the VALUE display answers — the
//!   `options.accessibility.text_background.chat`/`.everywhere` keys
//!   the lang table keeps, the CycleButton's DisplayState.VALUE the
//!   label law the name field the narration only wears).
//! - THE LOAD THE MISSING FILE THE DEFAULTS KEEP (the
//!   `if (!optionsFile.exists()) return` the head walk speaks), the lines
//!   the `Splitter.on(':').limit(2)` law splits, the malformed line the
//!   warn-and-skip the catch handler answers; the DECODE FAILURE the
//!   default keeps (the DataResult.ifError lane — the out-of-range value
//!   the clamp NEVER wears); the UnitDouble's BOOL alternative the legacy
//!   file reads (`"true"`/`"false"` the 1.0/0.0 the lambda answers).
//! - THE TIMING THE VANILLA SPEAKS: the load the `Minecraft.<init>` boot
//!   answers (before the menus), the save the OptionsScreen.onClose chord
//!   wears — the client's seat the run_gui_app boot + the Done chord ride.

/// THE VERSION LINE'S OWN INT (the jar's version.json `world_version` the
/// `SharedConstants.getCurrentVersion().dataVersion().version()` the save's
/// first line prints).
pub const OPTIONS_DATA_VERSION: i64 = 4903;

/// THE THREE STORES THE CLIENT'S OPTIONS SCREEN WEARS — the file's own
/// shape the load/save lanes keep (the vanilla keys the store fields
/// mirror; the rest of the jar's option table the client's seats never
/// carry — the unknown keys the load's skip law answers).
pub struct OptionsFile {
    /// `showSubtitles` — the closed captions' own boolean (the default
    /// FALSE the Options ctor's `iconst_0` pcs 2053-2067 banks, the
    /// `createBoolean` law the CLOSED_CAPTIONS_TOOLTIP tooltip wears,
    /// the caption the `options.showSubtitles` key the jar's en_us
    /// "Closed Captions" row speaks; the processOptions the ldc@149 the
    /// visit seat keeps — the row the version line's AFTER the
    /// mouseSensitivity@292 row's BEFORE wears (the visit ladder the
    /// 182 probe banks); the consumer the SubtitleOverlay's own
    /// extractRenderState the isListening gate speaks — the TRUE the
    /// listener the addListener arm wears, the render the 0.8F band +
    /// the fade the rows the walk serves).
    pub show_subtitles: bool,
    /// `mouseSensitivity` — the 0..1 unit double (the default 0.5 the
    /// clinit banks).
    pub mouse_sensitivity: f64,
    /// `fov` — the raw int degrees 30..=110 (the default 70 the bipush
    /// banks; the IntRange codec the integer the line stores).
    pub fov: f64,
    /// `soundCategory_master` — the 0..1 master volume (the default 1.0
    /// the createSoundSliderOptionInstance dconst_1 banks).
    pub sound_category_master: f64,
    /// `pauseOnLostFocus` — the focus-loss pause's own boolean (the
    /// default TRUE the Options ctor's iconst_1 banks, the
    /// processOptions pc 567-574 the `process("pauseOnLostFocus", Z)Z`
    /// round-trip wears — the 26.2's own seat the debug chord toggles,
    /// the screen row the accessibility lane retired).
    pub pause_on_lost_focus: bool,
    /// `fovEffectScale` — the FOV effects' own 0..1 scale (the default
    /// 1.0 the dconst_1 banks, the doubleRange(0.0, 1.0) codec the
    /// range keeps — the strict double the decode answers, the BOOL
    /// alternative the UnitDouble-withAlternative lane the option
    /// never registers; the AccessibilityOptionsScreen row the 26.2
    /// seat keeps, the client's flat options screen the row never
    /// hosts — the file + the camera's read the round-trip answers).
    pub fov_effect_scale: f64,
    /// `chatOpacity` — the chat text's own 0..1 scale (the default 1.0
    /// the Options ctor's dconst_1 pcs 741-818 banks, the
    /// OptionInstance$UnitDouble.INSTANCE value set the strict double
    /// keeps — the out-of-range the default answers; the noTooltip
    /// lane the registration wears, the accessibility array the
    /// index-6 seat keeps (narrator 0, showSubtitles 1, highContrast
    /// 2, menuBackgroundBlurriness 3, textBackgroundOpacity 4,
    /// backgroundForChatOnly 5, chatOpacity 6 the banked walk speaks);
    /// the ChatComponent's text-alpha base the consumer speaks — the
    /// extractRenderState fstore-12 law the `chatOpacity * 0.9f +
    /// 0.1f` composition keeps, the ChatComponent$1.accept the
    /// `fload_3 * val$textOpacity` pair the per-line fade the glyph
    /// alpha multiplies wears — the default 1.0 the 1.0 fold the
    /// byte-identical answer keeps).
    pub chat_opacity: f64,
    /// `chatLineSpacing` — the chat rows' own 0..1 pitch scale (the
    /// default 0.0 the Options ctor's dconst_0 pcs 813-842 banks, the
    /// OptionInstance$UnitDouble.INSTANCE value set the strict double
    /// keeps — the out-of-range the default answers; the noTooltip
    /// lane the registration wears, the SAME indy@622
    /// CaptionBasedToString the chatOpacity/textbg rows the percent/
    /// OFF words keep, the accessibility array the index-7 seat hosts
    /// (chatOpacity 6 the row BEFORE, fovEffectScale 12 the row AFTER
    /// the banked walk speaks)); the ChatComponent's row pitch the
    /// consumer speaks — the extractRenderState istore-18 law the
    /// `trunc(9.0 * (spacing + 1.0))` stride keeps (the same law the
    /// getLineHeight()I body wears), the istore-19 the glyph offset
    /// the `round(8.0 * (spacing + 1.0) - 4.0 * spacing)` rows answer
    /// — the zero spacing the 9/8 pair the byte-identical answer
    /// keeps; the `9 + round(4.0 * spacing)` and `round(8 - 4 *
    /// spacing)` guesses the old notes wore the bytecode refuses).
    pub chat_line_spacing: f64,
    /// `textBackgroundOpacity` — the chat backgrounds' own 0..1 scale
    /// (the default 0.5 the Options ctor's `ldc2_w double
    /// 4602678819172646912` banks, the doubleRange(0.0, 1.0) codec the
    /// strict double keeps — the out-of-range the default answers; the
    /// noTooltip lane the registration wears, the accessibility array
    /// the index-4 seat keeps; the ChatComponent's per-line fill the
    /// lambda$extractRenderState$1 walk the consumer speaks — the
    /// `ARGB.black(fade * textbg)` composition the
    /// textbg_lambda_262.py probe banks, the ChatScreen input band's
    /// own band the `backgroundForChatOnly ? raw : black(textbg)`
    /// pair the (I)I variant keeps — the default TRUE lane the raw
    /// 0x80000000 the client's own fill wears).
    pub text_background_opacity: f64,
    /// `backgroundForChatOnly` — the chat band's own scope boolean (the
    /// default TRUE the Options ctor's `iconst_1` pcs 2092-2120 banks,
    /// the raw OptionInstance ctor the BOOLEAN_VALUES value set the
    /// NO_ACTION listener wears — the noTooltip lane the same
    /// registration keeps; the processOptions pc 523-526 the
    /// `process("backgroundForChatOnly", Z)Z` round-trip wears, the
    /// visit-order adjacency the textBackgroundOpacity@506 the row
    /// AFTER keeps, the hideServerAddress@537 the next row banks; the
    /// consumer the ChatScreen.extractRenderState walk speaks — the
    /// input band the `getBackgroundColor(0x80000000)` call the raw
    /// TRUE lane wears, the Everywhere lane the
    /// `ARGB.colorFromFloat(textbg,0,0,0)` pair the (I)I variant
    /// answers, the 0.5 default the byte-identical fold the band's
    /// own alpha keeps; the caption the lambda$new$76 law the VALUE
    /// display answers — the "Chat"/"Everywhere" keys the lang table
    /// keeps, the CycleButton's name field the narration only wears).
    pub background_for_chat_only: bool,
}

impl Default for OptionsFile {
    fn default() -> Self {
        OptionsFile {
            show_subtitles: false,
            mouse_sensitivity: 0.5,
            fov: 70.0,
            sound_category_master: 1.0,
            pause_on_lost_focus: true,
            fov_effect_scale: 1.0,
            chat_opacity: 1.0,
            chat_line_spacing: 0.0,
            text_background_opacity: 0.5,
            background_for_chat_only: true,
        }
    }
}

impl OptionsFile {
    /// THE FILE'S OWN LOAD WALK (the vanilla `Options.load()` law): the
    /// missing/unreadable file the defaults keep, the lines the
    /// `key:value` split law parses (the limit-2 the first ':' the value's
    /// rest keeps — the readLine law the trailing '\r' strips), the
    /// malformed line the skip answers, the DECODE FAILURE the default
    /// keeps (the out-of-range the clamp never wears — the DataResult
    /// error lane the vanilla log walks).
    pub fn load(path: &std::path::Path) -> OptionsFile {
        let mut out = OptionsFile::default();
        let Ok(text) = std::fs::read_to_string(path) else {
            return out;
        };
        for line in text.lines() {
            let line = line.strip_suffix('\r').unwrap_or(line);
            let Some((key, value)) = line.split_once(':') else {
                continue; // the malformed line the warn-and-skip law answers
            };
            match key {
                "showSubtitles" => {
                    // THE BOOLEAN'S OWN DECODE (the "true"/"false" the
                    // codec's own vocabulary wears — the malformed the
                    // default keeps, the skip law the row answers; the
                    // default FALSE the Options ctor's iconst_0 the
                    // missing row keeps)
                    if value == "true" {
                        out.show_subtitles = true;
                    } else if value == "false" {
                        out.show_subtitles = false;
                    }
                }
                "mouseSensitivity" => {
                    if let Some(v) = parse_unit_double(value) {
                        out.mouse_sensitivity = v;
                    }
                }
                "fov" => {
                    if let Some(v) = parse_fov_int(value) {
                        out.fov = v;
                    }
                }
                "soundCategory_master" => {
                    if let Some(v) = parse_unit_double(value) {
                        out.sound_category_master = v;
                    }
                }
                "pauseOnLostFocus" => {
                    // THE BOOLEAN'S OWN DECODE (the "true"/"false" the
                    // codec's own vocabulary wears — the malformed the
                    // default keeps, the skip law the row answers)
                    if value == "true" {
                        out.pause_on_lost_focus = true;
                    } else if value == "false" {
                        out.pause_on_lost_focus = false;
                    }
                }
                "fovEffectScale" => {
                    // THE STRICT DOUBLE'S OWN DECODE (the
                    // doubleRange(0.0, 1.0) codec the in-range double
                    // answers — the BOOL alternative the option never
                    // registers, the out-of-range/malformed the default
                    // keeps, the clamp never wears)
                    if let Ok(v) = value.parse::<f64>() {
                        if (0.0..=1.0).contains(&v) {
                            out.fov_effect_scale = v;
                        }
                    }
                }
                "chatOpacity" => {
                    // THE SAME STRICT DOUBLE'S OWN DECODE (the
                    // doubleRange(0.0, 1.0) codec the in-range double
                    // answers — the out-of-range/malformed the default
                    // keeps, the clamp never wears; the default 1.0
                    // the Options ctor's dconst_1 the missing row
                    // keeps)
                    if let Ok(v) = value.parse::<f64>() {
                        if (0.0..=1.0).contains(&v) {
                            out.chat_opacity = v;
                        }
                    }
                }
                "chatLineSpacing" => {
                    // THE SAME STRICT DOUBLE'S OWN DECODE (the
                    // doubleRange(0.0, 1.0) codec the in-range double
                    // answers — the out-of-range/malformed the default
                    // keeps, the clamp never wears; the default 0.0
                    // the Options ctor's dconst_0 the missing row
                    // keeps)
                    if let Ok(v) = value.parse::<f64>() {
                        if (0.0..=1.0).contains(&v) {
                            out.chat_line_spacing = v;
                        }
                    }
                }
                "textBackgroundOpacity" => {
                    // THE SAME STRICT DOUBLE'S OWN DECODE (the
                    // doubleRange(0.0, 1.0) codec the in-range double
                    // answers — the out-of-range/malformed the default
                    // keeps, the clamp never wears; the default 0.5 the
                    // Options ctor's ldc2_w bits the missing row keeps)
                    if let Ok(v) = value.parse::<f64>() {
                        if (0.0..=1.0).contains(&v) {
                            out.text_background_opacity = v;
                        }
                    }
                }
                "backgroundForChatOnly" => {
                    // THE BAND SCOPE'S OWN DECODE (the "true"/"false"
                    // the codec's own vocabulary wears — the same bool
                    // law the pauseOnLostFocus row keeps, the malformed
                    // the default keeps, the skip law the row answers;
                    // the default TRUE the Options ctor's iconst_1 the
                    // missing row keeps)
                    if value == "true" {
                        out.background_for_chat_only = true;
                    } else if value == "false" {
                        out.background_for_chat_only = false;
                    }
                }
                _ => {} // the jar's other rows the client's seats never carry
            }
        }
        out
    }

    /// THE FILE'S OWN SAVE WALK (the vanilla `Options.save()` law): the
    /// truncating rewrite the FileOutputStream speaks, the version line
    /// the first row keeps, the keys the processOptions lane passes, the
    /// failure the silent keep answers (the in-memory stores the truth
    /// stays — the repo's file lanes the same verdict wear).
    pub fn save(&self, path: &std::path::Path) {
        let mut text = String::new();
        text.push_str(&format!("version:{}\n", OPTIONS_DATA_VERSION));
        // THE CLOSED CAPTIONS' OWN ROW (the "true"/"false" the Rust
        // Display the vanilla BooleanValue's codec vocabulary wears —
        // the row the visit-order seat keeps: the showSubtitles the
        // ldc@149 the version line the row AFTER answers, the
        // mouseSensitivity@292 the row BEFORE the same ladder speaks)
        text.push_str(&format!("showSubtitles:{}\n", self.show_subtitles));
        text.push_str(&format!(
            "mouseSensitivity:{}\n",
            java_double(self.mouse_sensitivity)
        ));
        text.push_str(&format!("fov:{}\n", self.fov as i64));
        text.push_str(&format!(
            "soundCategory_master:{}\n",
            java_double(self.sound_category_master)
        ));
        // THE BOOLEAN'S OWN ROW (the "true"/"false" the Rust Display the
        // vanilla BooleanValue's codec vocabulary wears)
        text.push_str(&format!("pauseOnLostFocus:{}\n", self.pause_on_lost_focus));
        // THE SCALE'S OWN ROW (the double the java_double print keeps —
        // the 1.0 the default the decimal speaks)
        text.push_str(&format!(
            "fovEffectScale:{}\n",
            java_double(self.fov_effect_scale)
        ));
        // THE CHAT TEXT'S OWN ROW (the double the java_double print
        // keeps — the 1.0 the default the decimal speaks; the row the
        // visit-order adjacency the processOptions walk keeps — the
        // chatOpacity the ldc@480 the textBackgroundOpacity the
        // ldc@506 the seat before answers)
        text.push_str(&format!("chatOpacity:{}\n", java_double(self.chat_opacity)));
        // THE CHAT LINES' OWN ROW (the double the java_double print
        // keeps — the 0.0 the default the decimal speaks; the row the
        // visit-order adjacency the processOptions walk keeps — the
        // chatLineSpacing the ldc@493 the chatOpacity@480 the row
        // AFTER answers, the textBackgroundOpacity@506 the row BEFORE
        // the same ladder speaks)
        text.push_str(&format!(
            "chatLineSpacing:{}\n",
            java_double(self.chat_line_spacing)
        ));
        // THE CHAT BACKGROUNDS' OWN ROW (the double the java_double
        // print keeps — the 0.5 the default the decimal speaks)
        text.push_str(&format!(
            "textBackgroundOpacity:{}\n",
            java_double(self.text_background_opacity)
        ));
        // THE BAND SCOPE'S OWN ROW (the "true"/"false" the Rust Display
        // the vanilla BooleanValue's codec vocabulary wears — the row
        // the visit-order adjacency the textBackgroundOpacity row's
        // AFTER keeps, the processOptions the bgOnly@523 the walk
        // banks, the hideServerAddress@537 the next row the same
        // ladder speaks)
        text.push_str(&format!(
            "backgroundForChatOnly:{}\n",
            self.background_for_chat_only
        ));
        let _ = std::fs::write(path, text);
    }
}

/// THE UNIT DOUBLE'S OWN READ (the OptionInstance$UnitDouble codec law —
/// the doubleRange(0, 1) decode the in-range double answers, the
/// withAlternative BOOL lane the legacy file reads: `"true"` the 1.0 the
/// lambda speaks, `"false"` the 0.0; the out-of-range/non-numeric the
/// None the default-keeps law wears).
fn parse_unit_double(value: &str) -> Option<f64> {
    if let Ok(v) = value.parse::<f64>() {
        return (0.0..=1.0).contains(&v).then_some(v);
    }
    match value {
        "true" => Some(1.0),
        "false" => Some(0.0),
        _ => None,
    }
}

/// THE FOV INT'S OWN READ (the OptionInstance$IntRange codec law — the
/// `Codec.intRange(30, 111)` decode the integer 30..=110 answers, the
/// fractional/out-of-range the None the default-keeps law wears).
fn parse_fov_int(value: &str) -> Option<f64> {
    let v = value.parse::<f64>().ok()?;
    if v.fract() != 0.0 || !(30.0..=110.0).contains(&v) {
        return None;
    }
    Some(v)
}

/// THE JAVA DOUBLE'S OWN PRINT (the `println(Object)` the
/// `Double.toString` shape wears — the integral value the ".0" the
/// decimal keeps: 1.0 the "1.0" the line stores, Rust's bare "1" the
/// `{}` never wears).
fn java_double(v: f64) -> String {
    let s = format!("{}", v);
    if s.contains('.') || s.contains('e') || s.contains("inf") || s.contains("NaN") {
        s
    } else {
        format!("{}.0", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // THE LINE LAW'S OWN TRUTH (the vanilla save's own rows — the version
    // line the first row keeps, the three keys the processOptions lane
    // passes, the defaults the clinit banks)
    #[test]
    fn the_options_file_wears_the_vanilla_line_law() {
        let dir = std::env::temp_dir().join(format!("mc2rs_options_file_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("options.txt");
        OptionsFile::default().save(&path);
        let text = std::fs::read_to_string(&path).expect("the save the file writes");
        assert_eq!(
            text, "version:4903\nshowSubtitles:false\nmouseSensitivity:0.5\nfov:70\nsoundCategory_master:1.0\npauseOnLostFocus:true\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\n",
            "the file the vanilla rows wear"
        );
        let loaded = OptionsFile::load(&path);
        assert_eq!(loaded.mouse_sensitivity, 0.5);
        assert_eq!(loaded.fov, 70.0);
        assert_eq!(loaded.sound_category_master, 1.0);
        assert_eq!(
            loaded.show_subtitles, false,
            "the captions the default the round-trip wears"
        );
        assert_eq!(
            loaded.pause_on_lost_focus, true,
            "the default the round-trip wears"
        );
        assert_eq!(
            loaded.fov_effect_scale, 1.0,
            "the scale the default the round-trip wears"
        );
        assert_eq!(
            loaded.chat_line_spacing, 0.0,
            "the chat lines the default the round-trip wears"
        );
        assert_eq!(
            loaded.text_background_opacity, 0.5,
            "the chat background the default the round-trip wears"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    // THE READ'S OWN LAWS (the decode failure the default keeps — the
    // out-of-range the clamp never wears, the malformed line the skip
    // answers, the legacy bool the 1.0/0.0 the alternative lane speaks,
    // the missing file the defaults keep)
    #[test]
    fn the_options_file_reads_the_real_rows() {
        let dir = std::env::temp_dir().join(format!("mc2rs_options_read_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("options.txt");
        std::fs::write(
            &path,
            "version:4903\n\
             mouseSensitivity:0.25\n\
             fov:110\n\
             soundCategory_master:0.0\n\
             soundCategory_music:0.4\n\
             lang:en_us\n\
             no_colon_line\n\
             mouseSensitivity:1.5\n\
             fov:70.5\n\
             fov:5\n\
             soundCategory_master:true\n\
             pauseOnLostFocus:maybe\n\
             pauseOnLostFocus:false\n\
             showSubtitles:false\n\
             showSubtitles:maybe\n\
             showSubtitles:true\n\
             fovEffectScale:0.5\n\
             fovEffectScale:1.5\n\
             fovEffectScale:true\n\
             fovEffectScale:0.25\n\
             chatOpacity:0.6\n\
             chatOpacity:1.5\n\
             chatOpacity:-0.25\n\
             chatOpacity:word\n\
             chatOpacity:0.4\n\
             chatLineSpacing:0.5\n\
             chatLineSpacing:1.5\n\
             chatLineSpacing:-0.25\n\
             chatLineSpacing:word\n\
             chatLineSpacing:0.25\n\
             textBackgroundOpacity:0.75\n\
             textBackgroundOpacity:1.5\n\
             textBackgroundOpacity:-0.25\n\
             textBackgroundOpacity:word\n\
             textBackgroundOpacity:0.25\n\
             backgroundForChatOnly:false\n\
             backgroundForChatOnly:maybe\n\
             backgroundForChatOnly:true\n",
        )
        .expect("the probe file writes");
        let loaded = OptionsFile::load(&path);
        assert_eq!(
            loaded.mouse_sensitivity, 0.25,
            "the in-range the read wears"
        );
        assert_eq!(loaded.fov, 110.0, "the int the codec decode answers");
        // the master row the LAST valid read wears — the later `true` the
        // BOOL alternative answers (the legacy file the 1.0 speaks)
        assert_eq!(
            loaded.sound_category_master, 1.0,
            "the bool alternative the legacy row wears"
        );
        // the unknown/malformed/out-of-range rows the skip law answers —
        // no panic, the defaults keep
        // THE BOOLEAN'S OWN DECODE (the last valid row the parse wears —
        // the `false` the decode speaks, the malformed `maybe` the skip
        // keeps, the legacy rows the same law the doubles answer)
        assert_eq!(
            loaded.pause_on_lost_focus, false,
            "the boolean the decode wears — the malformed row the skip keeps"
        );
        // THE CLOSED CAPTIONS' OWN DECODE (the same bool law the
        // pauseOnLostFocus row keeps — the last valid row the parse
        // wears, the malformed the skip answers, the first `false` the
        // decode speaks then the `true` the LAST valid banks)
        assert_eq!(
            loaded.show_subtitles, true,
            "the captions the last valid row wears — the malformed row the skip keeps"
        );
        // THE SCALE'S OWN DECODE (the in-range double the read wears —
        // the out-of-range 1.5 the skip keeps, the BOOL row the strict
        // codec never answers, the LAST valid the 0.25 the read banks)
        assert_eq!(
            loaded.fov_effect_scale, 0.25,
            "the scale the last valid row wears — the out-of-range/bool the skip keeps"
        );
        // THE CHAT TEXT'S OWN DECODE (the same strict double the
        // in-range 0.6 the first read wears — the out-of-range 1.5/
        // -0.25 pair + the malformed word the skip keeps, the LAST
        // valid the 0.4 the read banks)
        assert_eq!(
            loaded.chat_opacity, 0.4,
            "the chat text the last valid row wears — the out-of-range/malformed the skip keeps"
        );
        // THE CHAT LINES' OWN DECODE (the same strict double the
        // in-range 0.5 the first read wears — the out-of-range 1.5/
        // -0.25 pair + the malformed word the skip keeps, the LAST
        // valid the 0.25 the read banks)
        assert_eq!(
            loaded.chat_line_spacing, 0.25,
            "the chat lines the last valid row wears — the out-of-range/malformed the skip keeps"
        );
        // THE CHAT BACKGROUND'S OWN DECODE (the same strict double the
        // in-range 0.75 the first read wears — the out-of-range 1.5/
        // -0.25 pair + the malformed word the skip keeps, the LAST
        // valid the 0.25 the read banks)
        assert_eq!(
            loaded.text_background_opacity, 0.25,
            "the chat background the last valid row wears — the out-of-range/malformed the skip keeps"
        );
        // THE BAND SCOPE'S OWN DECODE (the last valid row the parse
        // wears — the `true` the decode speaks, the malformed `maybe`
        // the skip keeps, the `false` the first row the same law the
        // bool rows answer)
        assert_eq!(
            loaded.background_for_chat_only, true,
            "the band scope the last valid row wears — the malformed row the skip keeps"
        );
        let _ = std::fs::remove_dir_all(&dir);
        // THE MISSING FILE the defaults keep (the exists() gate the head
        // walk speaks)
        let missing = OptionsFile::load(&dir.join("absent.txt"));
        assert_eq!(
            missing.show_subtitles, false,
            "the missing file the default captions keeps"
        );
        assert_eq!(missing.mouse_sensitivity, 0.5);
        assert_eq!(missing.fov, 70.0);
        assert_eq!(missing.sound_category_master, 1.0);
        assert_eq!(
            missing.pause_on_lost_focus, true,
            "the missing file the default true keeps"
        );
        assert_eq!(
            missing.fov_effect_scale, 1.0,
            "the missing file the default scale keeps"
        );
        assert_eq!(
            missing.chat_opacity, 1.0,
            "the missing file the default chat text keeps"
        );
        assert_eq!(
            missing.chat_line_spacing, 0.0,
            "the missing file the default chat lines keeps"
        );
        assert_eq!(
            missing.text_background_opacity, 0.5,
            "the missing file the default chat background keeps"
        );
        assert_eq!(
            missing.background_for_chat_only, true,
            "the missing file the default band scope keeps"
        );
    }

    // THE JAVA DOUBLE'S OWN PRINT (the integral the ".0" the decimal
    // keeps — the Rust bare "1" the line never stores)
    #[test]
    fn the_java_double_print_keeps_the_decimal() {
        assert_eq!(java_double(1.0), "1.0");
        assert_eq!(java_double(0.0), "0.0");
        assert_eq!(java_double(0.5), "0.5");
        assert_eq!(java_double(0.25), "0.25");
    }
}

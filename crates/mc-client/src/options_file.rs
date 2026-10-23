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
//!   speaks).
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
}

impl Default for OptionsFile {
    fn default() -> Self {
        OptionsFile {
            mouse_sensitivity: 0.5,
            fov: 70.0,
            sound_category_master: 1.0,
            pause_on_lost_focus: true,
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
            text, "version:4903\nmouseSensitivity:0.5\nfov:70\nsoundCategory_master:1.0\npauseOnLostFocus:true\n",
            "the file the vanilla rows wear"
        );
        let loaded = OptionsFile::load(&path);
        assert_eq!(loaded.mouse_sensitivity, 0.5);
        assert_eq!(loaded.fov, 70.0);
        assert_eq!(loaded.sound_category_master, 1.0);
        assert_eq!(
            loaded.pause_on_lost_focus, true,
            "the default the round-trip wears"
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
             pauseOnLostFocus:false\n",
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
        let _ = std::fs::remove_dir_all(&dir);
        // THE MISSING FILE the defaults keep (the exists() gate the head
        // walk speaks)
        let missing = OptionsFile::load(&dir.join("absent.txt"));
        assert_eq!(missing.mouse_sensitivity, 0.5);
        assert_eq!(missing.fov, 70.0);
        assert_eq!(missing.sound_category_master, 1.0);
        assert_eq!(
            missing.pause_on_lost_focus, true,
            "the missing file the default true keeps"
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

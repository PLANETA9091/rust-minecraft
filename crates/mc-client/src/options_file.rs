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
    /// `notificationDisplayTime` — the notifications' own time multiplier
    /// (the default 1.0 the Options ctor's dconst_1 pc 1352 banks, the
    /// `Codec.doubleRange(0.5, 10.0)` codec the strict double keeps —
    /// the out-of-range the default answers, the clamp never wears; the
    /// value set the `OptionInstance$IntRange(5, 100).xmap(i -> i /
    /// 10.0, v -> (int)(v * 10.0), true)` slider walk the 183 probes
    /// bank — the 96 seats the 0.1 step the lambda$new$50/51 pair
    /// speaks, the iconst_1 the sliderable flag the xmap's third arg
    /// wears); the caption the lambda$new$49 law answers — the
    /// `genericValueLabel(label, translatable("options.multiplier",
    /// value))` composition the "%sx" row keeps (the lang rows the
    /// `options.notifications.display_time` = "Notification Time" + the
    /// `options.multiplier` = "%sx" pair the jar's en_us table speaks,
    /// the '1.0x' the default caption answers); the accessibility array
    /// the index-9 seat keeps (chatDelay 8 the row BEFORE, bobView 10
    /// the row AFTER the banked walk speaks); the processOptions the
    /// ldc@682 the visit seat keeps — the walk's LAST seat the client's
    /// carried rows wear (pauseOnLostFocus@567 the row BEFORE, the
    /// file's own END the save row answers); the readers the value's
    /// own consumers speak — the SubtitleOverlay's extractRenderState
    /// dstore-9 the `3000.0 * displayTime` window multiplier keeps, the
    /// Hud's toolHighlightTimer the `40.0 * value` ticks the row banks,
    /// the NowPlayingToast/ToastManager the `5000.0 * value` ms the
    /// toast lane wears).
    pub notification_display_time: f64,
    /// THE KEYMAPPINGS' OWN ROWS (the movement category the 186 round
    /// banks — the `processOptions` walk's key loop the pc 1064-1137
    /// seats keep: the `keyMappings` array the row order wears, the
    /// `"key_" + name` prefix the indy stringConcat composition speaks,
    /// the `KeyMapping.saveString()` the value answers — the
    /// `InputConstants.Key.getName()` row the `key.keyboard.*` format
    /// keeps). The store the `Option<minifb::Key>` pairs wear — the
    /// None the `InputConstants.UNKNOWN` seat answers (the
    /// `key.keyboard.unknown` row the save/decode round-trip keeps),
    /// the missing rows the DEFAULTS the load law keeps (the vanilla
    /// `if (!optionsFile.exists()) return` head walk). The lane the
    /// movement categories the rebind walk serves — the OTHER
    /// categories (the UI/gameplay/creative/debug rows the 57-instance
    /// ctor array carries) the follow-up rounds' own stores bank.
    pub key_binds: [Option<key_binds::Bind>; key_binds::COUNT],
}

/// THE MOVEMENT CATEGORY'S OWN STORE (the Options ctor's registrations
/// pc 2402-2545 the array order keeps — the `new KeyMapping(name, GLFW
/// code, Category.MOVEMENT)` triples the walk speaks: key.forward/W,
/// key.left/A, key.back/S, key.right/D, key.jump/Space, the
/// ToggleKeyMapping pair the sneak/sprint rows wear (LEFT_SHIFT=340,
/// LEFT_CONTROL=341); the client's `minifb::Key` enum the OWN
/// discriminants the GLFW codes the store holds — W=32, A=10, S=28,
/// D=13, Space, LeftShift, LeftCtrl the minifb values answer).
pub mod key_binds {
    use minifb::{Key, MouseButton};

    /// THE BIND'S OWN TYPE (the `InputConstants$Key` walk the store
    /// wears — the keyboard AND the mouse buttons the ONE type keeps:
    /// the type the enum's own arms answer, the Keyboard the KEYSYM
    /// arm the `key.keyboard.*` names speak, the Mouse the MOUSE arm
    /// the seeded `key.mouse.*` names wear). The vanilla
    /// `KeyMapping.key` the one field holds — the trio's own defaults
    /// the MOUSE values the ctor walk banks (`new ToggleKeyMapping(
    /// "key.use", Type.MOUSE, 1, Category.GAMEPLAY, ...)` the Options
    /// ctor pc 2628/2664/2700 speak).
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Bind {
        Keyboard(Key),
        Mouse(MouseButton),
    }

    pub const FORWARD: usize = 0;
    pub const LEFT: usize = 1;
    pub const BACK: usize = 2;
    pub const RIGHT: usize = 3;
    pub const JUMP: usize = 4;
    pub const SNEAK: usize = 5;
    pub const SPRINT: usize = 6;
    /// THE INVENTORY TRIO'S OWN SLOTS (the ctor's registrations the
    /// movement pair's AFTER wear — the 186 walk's array order: the
    /// `key.inventory`/`key.swapOffhand`/`key.drop` rows the
    /// INVENTORY category answers, the E/F/Q defaults the ctor's
    /// GLFW codes the minifb enum's own discriminants keep)
    pub const INVENTORY: usize = 7;
    pub const SWAP_OFFHAND: usize = 8;
    pub const DROP: usize = 9;
    /// THE GAMEPLAY TRIO'S OWN SLOTS (the ctor's registrations the
    /// inventory trio's AFTER wear — the ldc walk's own order: keyUse
    /// pc 2628, keyAttack pc 2664, keyPickItem pc 2700, the GAMEPLAY
    /// category the all three wear). THE MOUSE DEFAULTS THE CTOR
    /// BANKS: the use the MOUSE 1 (Right), the attack the MOUSE 0
    /// (Left — the `ToggleKeyMapping` the `toggleAttack` supplier
    /// carries), the pickItem the MOUSE 2 (Middle — the plain
    /// `KeyMapping` the toggle-less walk speaks).
    pub const USE: usize = 10;
    pub const ATTACK: usize = 11;
    pub const PICK_ITEM: usize = 12;
    /// THE HOTBAR NINE'S OWN SLOTS (the ctor's `keyHotbarSlots` array
    /// the literal walk banks — pc 2980-3144 the block registers: the
    /// `new KeyMapping("key.hotbar.1", bipush 49, Category.INVENTORY)`
    /// the GLFW 1..9 codes the bipush 49..57 pairs keep, the
    /// INVENTORY category the all nine wear, the aastore indices the
    /// iconst_0..bipush_8 rows answer — the gameplay trio's AFTER the
    /// array order banks)
    pub const HOTBAR_1: usize = 13;
    pub const HOTBAR_2: usize = 14;
    pub const HOTBAR_3: usize = 15;
    pub const HOTBAR_4: usize = 16;
    pub const HOTBAR_5: usize = 17;
    pub const HOTBAR_6: usize = 18;
    pub const HOTBAR_7: usize = 19;
    pub const HOTBAR_8: usize = 20;
    pub const HOTBAR_9: usize = 21;
    /// THE ACTIVATOR PAIR'S OWN SLOTS (the ctor's putfields the hotbar
    /// array's AFTER the walk banks — the save/load pair the pc 3147
    /// / 3166 rows wear, the CREATIVE category the `bipush 67` (the
    /// GLFW C) / `bipush 88` (the GLFW X) codes the DEFAULTS keep,
    /// the ldcs the `key.saveToolbarActivator` / `key.loadToolbar
    /// Activator` names resolve (the cp#1382/#1390 utf8s speak — the
    /// FIELD name the HotbarActivator word keeps, the LANG key the
    /// Toolbar word wears))
    pub const SAVE_ACTIVATOR: usize = 22;
    pub const LOAD_ACTIVATOR: usize = 23;
    /// THE SPECTATOR PAIR'S OWN SLOTS (the ctor's putfields the
    /// activator pair's AFTER the walk banks — the pc 3205/3226 rows
    /// wear: the outlines the `getstatic InputConstants.UNKNOWN` the
    /// unbound default answers, the hotbar the `Type.MOUSE, iconst_2`
    /// the middle button the seeded name wears, the SPECTATOR
    /// category the pair's rows the clinit's ordinal 6 keeps)
    pub const SPECTATOR_OUTLINES: usize = 24;
    pub const SPECTATOR_HOTBAR: usize = 25;
    /// THE DEBUG OVERLAY'S OWN SLOT (the ctor's pc 3229-3251 walk the
    /// spectator pair's AFTER the walk banks — the `ldc_w cp#1407`
    /// the `key.debug.overlay` string answers, the `KEYSYM, sipush
    /// 292` the F3 default wears, the Category.DEBUG the group's own
    /// ordinal 7 keeps, the FIFTH ctor arg the `bipush -2` the
    /// `orderI` field the sort walk banks — the modifier-aware
    /// KeyMapping ctor overload the 5-arg shape the walk answers, the
    /// modifier row the -1 arg the next walk's own seat wears)
    pub const DEBUG_OVERLAY: usize = 26;
    /// THE DEBUG MODIFIER'S OWN SLOT (the ctor's pc 3254-3275 walk the
    /// overlay's putfield the AFTER banks — the `ldc_w cp#1421` the
    /// `key.debug.modifier` string answers, the `KEYSYM, sipush 292`
    /// the F3 default the overlay pair shares, the Category.DEBUG the
    /// same group's ordinal keeps, the FIFTH ctor arg the `iconst_m1`
    /// the orderI field the sort walk banks — the 5-arg overload the
    /// -1 the modifier-aware pair's own arg wears)
    pub const DEBUG_MODIFIER: usize = 27;
    /// THE DEBUG CRASH'S OWN SLOT (the ctor's pc 3278-3297 walk the
    /// modifier's putfield the AFTER banks — the `ldc_w cp#1426` the
    /// `key.debug.crash` string answers, the `KEYSYM, bipush 67` the
    /// GLFW C default wears, the Category.DEBUG the same group's
    /// ordinal keeps, the FOUR-arg ctor the call speaks — the
    /// `iconst_0` the orderI default the 4-arg delegate banks, the
    /// ladder's first plain-shape row the pair's own 5-arg shape
    /// answers)
    pub const DEBUG_CRASH: usize = 28;
    /// THE DEBUG RELOAD CHUNK'S OWN SLOT (the ctor's pc 3300-3319
    /// walk the crash's putfield the AFTER banks — the `ldc_w
    /// cp#1431` the `key.debug.reloadChunk` string answers, the
    /// `KEYSYM, bipush 65` the GLFW A default wears, the Category.DEBUG
    /// the same group's ordinal keeps, the FOUR-arg ctor the call
    /// speaks — the `iconst_0` the orderI default the 4-arg delegate
    /// banks, the crash row's own plain shape the row shares)
    pub const DEBUG_RELOAD_CHUNK: usize = 29;
    /// THE DEBUG SHOW HITBOXES' OWN SLOT (the ctor's pc 3322-3341
    /// walk the reload's putfield the AFTER banks — the `ldc_w
    /// cp#1436` the `key.debug.showHitboxes` string answers, the
    /// `KEYSYM, bipush 66` the GLFW B default wears, the Category.DEBUG
    /// the same group's ordinal keeps, the FOUR-arg ctor the call
    /// speaks — the `iconst_0` the orderI default the 4-arg delegate
    /// banks, the reload row's own plain shape the row shares, the
    /// next registration the pc 3344 seat the later round banks —
    /// the cp#1441 the `key.debug.clearChat` string the bipush 68
    /// the D default the walk banks)
    pub const DEBUG_SHOW_HITBOXES: usize = 30;
    /// THE DEBUG CLEAR CHAT'S OWN SLOT (the ctor's pc 3344-3363
    /// walk the hitboxes' putfield the AFTER banks — the `ldc_w
    /// cp#1441` the `key.debug.clearChat` string answers, the
    /// `KEYSYM, bipush 68` the GLFW D default wears, the Category.DEBUG
    /// the same group's ordinal keeps, the FOUR-arg ctor the call
    /// speaks — the `iconst_0` the orderI default the 4-arg delegate
    /// banks, the hitboxes row's own plain shape the row shares, the
    /// next registration the pc 3366 seat the later round banks —
    /// the cp#1446 the `key.debug.showChunkBorders` string the bipush
    /// 71 the G default the walk banks)
    pub const DEBUG_CLEAR_CHAT: usize = 31;
    /// THE DEBUG SHOW CHUNK BORDERS' OWN SLOT (the ctor's pc 3366-3385
    /// walk the clearChat's putfield the AFTER banks — the `ldc_w
    /// cp#1446` the `key.debug.showChunkBorders` string answers, the
    /// `KEYSYM, bipush 71` the GLFW G default wears, the Category.DEBUG
    /// the same group's ordinal keeps, the FOUR-arg ctor the call
    /// speaks — the `iconst_0` the orderI default the 4-arg delegate
    /// banks, the clearChat row's own plain shape the row shares, the
    /// next registration the pc 3388 seat the later round banks —
    /// the cp#1451 the `key.debug.showAdvancedTooltips` string the
    /// bipush 72 the H default the walk banks)
    pub const DEBUG_SHOW_CHUNK_BORDERS: usize = 32;
    /// THE DEBUG SHOW ADVANCED TOOLTIPS' OWN SLOT (the ctor's
    /// pc 3388-3407 walk the chunkBorders' putfield the AFTER banks
    /// — the `ldc_w cp#1451` the `key.debug.showAdvancedTooltips`
    /// string answers, the `KEYSYM, bipush 72` the GLFW H default
    /// wears, the Category.DEBUG the same group's ordinal keeps, the
    /// FOUR-arg ctor the call speaks — the `iconst_0` the orderI
    /// default the 4-arg delegate banks, the chunkBorders row's own
    /// plain shape the row shares, the next registration the pc 3410
    /// seat the later round banks — the cp#1456 the
    /// `key.debug.copyRecreateCommand` string the bipush 73 the I
    /// default the walk banks)
    pub const DEBUG_SHOW_ADV_TOOLTIPS: usize = 33;
    pub const COUNT: usize = 34;

    /// THE VANILLA BINDING NAMES (the `key_` + name the file row's
    /// prefix wears, the lang captions the `key.*` rows answer — the
    /// en_us table the "Walk Forward"/"Strafe Left" pair speaks).
    pub const NAMES: [&str; COUNT] = [
        "key.forward",
        "key.left",
        "key.back",
        "key.right",
        "key.jump",
        "key.sneak",
        "key.sprint",
        "key.inventory",
        "key.swapOffhand",
        "key.drop",
        "key.use",
        "key.attack",
        "key.pickItem",
        "key.hotbar.1",
        "key.hotbar.2",
        "key.hotbar.3",
        "key.hotbar.4",
        "key.hotbar.5",
        "key.hotbar.6",
        "key.hotbar.7",
        "key.hotbar.8",
        "key.hotbar.9",
        // THE ACTIVATOR PAIR'S OWN NAMES (the ctor's ldc strings the
        // cp#1382/#1390 utf8s resolve — the Toolbar word the lang key
        // wears, the HotbarActivator word the FIELD name keeps)
        "key.saveToolbarActivator",
        "key.loadToolbarActivator",
        // THE SPECTATOR PAIR'S OWN NAMES (the ctor's ldc strings the
        // cp#1394/#1402 utf8s resolve — the pc 3190/3213 rows the
        // activator pair's AFTER wear)
        "key.spectatorOutlines",
        "key.spectatorHotbar",
        // THE DEBUG OVERLAY'S OWN NAME (the ctor's `ldc_w cp#1407` the
        // pc 3234 string answers — the overlay row the pair's AFTER
        // the ladder wears)
        "key.debug.overlay",
        // THE DEBUG MODIFIER'S OWN NAME (the ctor's `ldc_w cp#1421` the
        // pc 3259 string answers — the modifier row the overlay's
        // AFTER the pair wears)
        "key.debug.modifier",
        // THE DEBUG CRASH'S OWN NAME (the ctor's `ldc_w cp#1426` the
        // pc 3283 string answers — the crash row the modifier's
        // AFTER the ladder wears)
        "key.debug.crash",
        // THE DEBUG RELOAD CHUNK'S OWN NAME (the ctor's `ldc_w
        // cp#1431` the pc 3305 string answers — the reload row the
        // crash's AFTER the ladder wears)
        "key.debug.reloadChunk",
        // THE DEBUG SHOW HITBOXES' OWN NAME (the ctor's `ldc_w
        // cp#1436` the pc 3327 string answers — the hitboxes row the
        // reload's AFTER the ladder wears)
        "key.debug.showHitboxes",
        // THE DEBUG CLEAR CHAT'S OWN NAME (the ctor's `ldc_w
        // cp#1441` the pc 3349 string answers — the clearChat row the
        // hitboxes' AFTER the ladder wears)
        "key.debug.clearChat",
        // THE DEBUG SHOW CHUNK BORDERS' OWN NAME (the ctor's `ldc_w
        // cp#1446` the pc 3371 string answers — the chunkBorders row
        // the clearChat's AFTER the ladder wears)
        "key.debug.showChunkBorders",
        // THE DEBUG SHOW ADVANCED TOOLTIPS' OWN NAME (the ctor's
        // `ldc_w cp#1451` the pc 3393 string answers — the advTooltips
        // row the chunkBorders' AFTER the ladder wears)
        "key.debug.showAdvancedTooltips",
    ];

    /// THE CAPTIONS' OWN FALLBACK (the en_us values the assets-absent
    /// boot keeps — the lang rows the same words speak).
    pub const NAME_FALLBACKS: [&str; COUNT] = [
        "Walk Forward",
        "Strafe Left",
        "Walk Backward",
        "Strafe Right",
        "Jump",
        "Sneak",
        "Sprint",
        "Open/Close Inventory",
        "Swap Item With Off Hand",
        "Drop Selected Item",
        "Use Item/Place Block",
        "Attack/Destroy",
        "Pick Block",
        "Hotbar Slot 1",
        "Hotbar Slot 2",
        "Hotbar Slot 3",
        "Hotbar Slot 4",
        "Hotbar Slot 5",
        "Hotbar Slot 6",
        "Hotbar Slot 7",
        "Hotbar Slot 8",
        "Hotbar Slot 9",
        // THE PAIR'S OWN CAPTIONS (the en_us rows speak — the same
        // Hotbar words the field name hides the lang rows answer)
        "Save Hotbar Activator",
        "Load Hotbar Activator",
        // THE SPECTATOR PAIR'S OWN CAPTIONS (the en_us rows speak —
        // key.spectatorOutlines the 'Highlight Players' row wears,
        // key.spectatorHotbar the 'Select On Hotbar' row answers)
        "Highlight Players",
        "Select On Hotbar",
        // THE DEBUG ROW'S OWN CAPTION (the en_us row speaks —
        // key.debug.overlay the 'Toggle Overlay' words wear)
        "Toggle Overlay",
        // THE DEBUG MODIFIER'S OWN CAPTION (the en_us row speaks —
        // key.debug.modifier the 'Debug Modifier Key' words wear)
        "Debug Modifier Key",
        // THE DEBUG CRASH'S OWN CAPTION (the en_us row speaks —
        // key.debug.crash the 'Debug Crash' words wear)
        "Debug Crash",
        // THE DEBUG RELOAD CHUNK'S OWN CAPTION (the en_us row speaks
        // — key.debug.reloadChunk the 'Reload Chunks' words wear)
        "Reload Chunks",
        // THE DEBUG SHOW HITBOXES' OWN CAPTION (the en_us row speaks
        // — key.debug.showHitboxes the 'Show Hitboxes' words wear)
        "Show Hitboxes",
        // THE DEBUG CLEAR CHAT'S OWN CAPTION (the en_us row speaks
        // — key.debug.clearChat the 'Clear Chat' words wear)
        "Clear Chat",
        // THE DEBUG SHOW CHUNK BORDERS' OWN CAPTION (the en_us row
        // speaks — key.debug.showChunkBorders the 'Show Chunk
        // Boundaries' words wear)
        "Show Chunk Boundaries",
        // THE DEBUG SHOW ADVANCED TOOLTIPS' OWN CAPTION (the en_us
        // row speaks — key.debug.showAdvancedTooltips the 'Show
        // Advanced Tooltips' words wear)
        "Show Advanced Tooltips",
    ];

    /// THE CATEGORY'S OWN ROW (the CategoryEntry the list groups the
    /// rows under — the `key.categories.movement` key the lang
    /// "Movement" row answers).
    pub const CATEGORY_NAME: &str = "key.categories.movement";
    pub const CATEGORY_FALLBACK: &str = "Movement";
    /// THE INVENTORY GROUP'S OWN HEADER ROW (the CategoryEntry the
    /// category boundary adds — the `key.categories.inventory` key the
    /// lang "Inventory" row answers)
    pub const CATEGORY_INV_NAME: &str = "key.categories.inventory";
    pub const CATEGORY_INV_FALLBACK: &str = "Inventory";
    /// THE GAMEPLAY GROUP'S OWN HEADER ROW (the CategoryEntry the
    /// category boundary adds — the `key.categories.gameplay` key the
    /// lang "Gameplay" row answers; the enum's own SORT_ORDER walk the
    /// clinit banks: movement, misc, multiplayer, GAMEPLAY, INVENTORY
    /// — the gameplay the movement/inventory the ordinals BETWEEN
    /// keep, the misc/multiplayer rows the store never carries, the
    /// group walk the categories WITH rows the render answers)
    pub const CATEGORY_GAMEPLAY_NAME: &str = "key.categories.gameplay";
    pub const CATEGORY_GAMEPLAY_FALLBACK: &str = "Gameplay";
    /// THE CREATIVE GROUP'S OWN HEADER ROW (the CategoryEntry the
    /// category boundary adds — the `key.categories.creative` key the
    /// lang "Creative Mode" row answers; the SORT_ORDER walk the
    /// clinit banks: movement, misc, multiplayer, gameplay, INVENTORY,
    /// CREATIVE — the creative the pair's group the inventory rows'
    /// AFTER keeps, the misc/multiplayer/spectator/debug rows the
    /// store never carries)
    pub const CATEGORY_CREATIVE_NAME: &str = "key.categories.creative";
    pub const CATEGORY_CREATIVE_FALLBACK: &str = "Creative Mode";
    /// THE SPECTATOR GROUP'S OWN HEADER ROW (the CategoryEntry the
    /// category boundary adds — the `key.categories.spectator` key the
    /// lang "Spectator" row answers; the SORT_ORDER walk the clinit
    /// banks: movement, misc, multiplayer, gameplay, INVENTORY,
    /// CREATIVE, SPECTATOR, debug — the spectator the creative rows'
    /// AFTER the ordinal 6 keeps, the misc/multiplayer/debug rows the
    /// store never carries)
    pub const CATEGORY_SPECTATOR_NAME: &str = "key.categories.spectator";
    pub const CATEGORY_SPECTATOR_FALLBACK: &str = "Spectator";
    /// THE DEBUG GROUP'S OWN HEADER ROW (the CategoryEntry the
    /// category boundary adds — the `KeyMapping$Category.label()`
    /// walk the NEW format speaks: the `id.toLanguageKey("key.category")`
    /// the `key.category.minecraft.debug` key answers (the lang row
    /// the 'Debug' word wears — the OLD `key.categories.*` rows the
    /// legacy format keeps, the debug row the table never carried);
    /// the SORT_ORDER walk the clinit banks: movement, misc,
    /// multiplayer, gameplay, INVENTORY, CREATIVE, SPECTATOR, DEBUG —
    /// the debug the spectator rows' AFTER the ordinal 7 keeps, the
    /// misc/multiplayer rows the store never carries)
    pub const CATEGORY_DEBUG_NAME: &str = "key.category.minecraft.debug";
    pub const CATEGORY_DEBUG_FALLBACK: &str = "Debug";

    /// THE DEFAULTS' OWN ROW (the ctor's codes the minifb Key enum's
    /// own discriminants answer).
    pub const DEFAULTS: [Option<Bind>; COUNT] = [
        Some(Bind::Keyboard(Key::W)),
        Some(Bind::Keyboard(Key::A)),
        Some(Bind::Keyboard(Key::S)),
        Some(Bind::Keyboard(Key::D)),
        Some(Bind::Keyboard(Key::Space)),
        Some(Bind::Keyboard(Key::LeftShift)),
        Some(Bind::Keyboard(Key::LeftCtrl)),
        Some(Bind::Keyboard(Key::E)),
        Some(Bind::Keyboard(Key::F)),
        Some(Bind::Keyboard(Key::Q)),
        // THE MOUSE TRIO'S OWN SEATS (the ctor's codes the MOUSE arm
        // the seeded names answer — getOrCreate(0/1/2) the pre-seeded
        // map the key.mouse.left/right/middle Keys wear)
        Some(Bind::Mouse(MouseButton::Right)),
        Some(Bind::Mouse(MouseButton::Left)),
        Some(Bind::Mouse(MouseButton::Middle)),
        // THE HOTBAR NINE'S OWN SEATS (the ctor's codes the GLFW 1..9
        // the minifb enum's own discriminants keep)
        Some(Bind::Keyboard(Key::Key1)),
        Some(Bind::Keyboard(Key::Key2)),
        Some(Bind::Keyboard(Key::Key3)),
        Some(Bind::Keyboard(Key::Key4)),
        Some(Bind::Keyboard(Key::Key5)),
        Some(Bind::Keyboard(Key::Key6)),
        Some(Bind::Keyboard(Key::Key7)),
        Some(Bind::Keyboard(Key::Key8)),
        Some(Bind::Keyboard(Key::Key9)),
        // THE ACTIVATOR PAIR'S OWN SEATS (the ctor's `bipush 67` the
        // GLFW C answers, the `bipush 88` the GLFW X — the CREATIVE
        // category the pair's rows wear)
        Some(Bind::Keyboard(Key::C)),
        Some(Bind::Keyboard(Key::X)),
        // THE SPECTATOR PAIR'S OWN SEATS (the ctor's pc 3193 the
        // `getstatic InputConstants.UNKNOWN` the UNBOUND default
        // answers — the None the `key.keyboard.unknown` the save row
        // wears, the pc 3216 the `Type.MOUSE, iconst_2` the MIDDLE
        // button the seeded name speaks — the SPECTATOR category the
        // pair's rows wear)
        None,
        Some(Bind::Mouse(MouseButton::Middle)),
        // THE DEBUG ROW'S OWN SEAT (the ctor's pc 3240 the `sipush
        // 292` the GLFW F3 answers — the KEYSYM type the keyboard
        // bind wears, the orderI the -2 arg the sort walk banks)
        Some(Bind::Keyboard(Key::F3)),
        // THE DEBUG MODIFIER'S OWN SEAT (the ctor's pc 3265 the same
        // `sipush 292` the F3 the pair shares — the KEYSYM type the
        // keyboard bind wears, the orderI the `iconst_m1` arg the
        // sort walk banks)
        Some(Bind::Keyboard(Key::F3)),
        // THE DEBUG CRASH'S OWN SEAT (the ctor's pc 3289 the `bipush
        // 67` the GLFW C answers — the KEYSYM type the keyboard bind
        // wears, the 4-arg ctor the call speaks, the orderI the
        // `iconst_0` default the delegate banks)
        Some(Bind::Keyboard(Key::C)),
        // THE DEBUG RELOAD CHUNK'S OWN SEAT (the ctor's pc 3311 the
        // `bipush 65` the GLFW A answers — the KEYSYM type the
        // keyboard bind wears, the 4-arg ctor the call speaks, the
        // orderI the `iconst_0` default the delegate banks)
        Some(Bind::Keyboard(Key::A)),
        // THE DEBUG SHOW HITBOXES' OWN SEAT (the ctor's pc 3333 the
        // `bipush 66` the GLFW B answers — the KEYSYM type the
        // keyboard bind wears, the 4-arg ctor the call speaks, the
        // orderI the `iconst_0` default the delegate banks)
        Some(Bind::Keyboard(Key::B)),
        // THE DEBUG CLEAR CHAT'S OWN SEAT (the ctor's pc 3355 the
        // `bipush 68` the GLFW D answers — the KEYSYM type the
        // keyboard bind wears, the 4-arg ctor the call speaks, the
        // orderI the `iconst_0` default the delegate banks)
        Some(Bind::Keyboard(Key::D)),
        // THE DEBUG SHOW CHUNK BORDERS' OWN SEAT (the ctor's pc 3377
        // the `bipush 71` the GLFW G answers — the KEYSYM type the
        // keyboard bind wears, the 4-arg ctor the call speaks, the
        // orderI the `iconst_0` default the delegate banks)
        Some(Bind::Keyboard(Key::G)),
        // THE DEBUG SHOW ADVANCED TOOLTIPS' OWN SEAT (the ctor's pc
        // 3399 the `bipush 72` the GLFW H answers — the KEYSYM type
        // the keyboard bind wears, the 4-arg ctor the call speaks,
        // the orderI the `iconst_0` default the delegate banks)
        Some(Bind::Keyboard(Key::H)),
    ];

    /// THE LIST'S OWN GEOMETRY (the entries the walk's y positions
    /// keep — the movement group the 186 render's own shape wears
    /// (the header the 30 seat, the rows the 42-start stride), the
    /// inventory group the category boundary the SAME shape keeps
    /// (the header 210 = 42 + 7*24, the rows the 222 start +12 the
    /// movement group's own gap answers)); the scroll the list-space
    /// y the shift reads.
    pub const LIST_TOP: i32 = 30;
    pub const ROW_STRIDE: i32 = 24;
    pub const MOVEMENT_ROWS_START: i32 = 42;
    /// THE GROUPS' OWN GEOMETRY (the sorted walk's own y positions —
    /// the CategoryEntry the group boundaries the SORT_ORDER ordinals
    /// answer: the movement rows the 42-start stride keeps, the
    /// GAMEPLAY header the 210 seat the movement rows' end the next
    /// stride slot wears (42 + 7*24), the gameplay rows the 222 start
    /// +12 the group's own gap answers, the INVENTORY header the 294
    /// seat the gameplay rows' end the next stride slot wears (222 +
    /// 3*24), the inventory rows the 306 start the same +12 keeps)
    pub const GAMEPLAY_HEADER_Y: i32 = 210;
    pub const GAMEPLAY_ROWS_START: i32 = 222;
    pub const INVENTORY_HEADER_Y: i32 = 294;
    pub const INVENTORY_ROWS_START: i32 = 306;
    /// THE CREATIVE GROUP'S OWN SEATS (the sorted walk's own y — the
    /// hotbar nine the inventory rows the 378..570 stride wear, the
    /// CREATIVE header the 594 seat the nine's own bottom the next
    /// stride slot answers (570 + 24), the pair rows the 606 start
    /// the same +12 the groups' own gap keeps)
    pub const CREATIVE_HEADER_Y: i32 = 594;
    pub const CREATIVE_ROWS_START: i32 = 606;
    /// THE SPECTATOR GROUP'S OWN SEATS (the sorted walk's own y — the
    /// creative pair the 606-start stride wears, the SPECTATOR header
    /// the pair's own bottom the next stride slot answers (630 + 24),
    /// the rows the +12 the groups' own gap keeps)
    pub const SPECTATOR_HEADER_Y: i32 = 654;
    pub const SPECTATOR_ROWS_START: i32 = 666;
    /// THE DEBUG GROUP'S OWN SEATS (the sorted walk's own y — the
    /// spectator pair the 666-start stride wears, the DEBUG header
    /// the pair's own bottom the next stride slot answers (690 +
    /// 24), the row the +12 the groups' own gap keeps)
    pub const DEBUG_HEADER_Y: i32 = 714;
    pub const DEBUG_ROWS_START: i32 = 726;

    /// THE CONTENT'S OWN HEIGHT (the walk's own bottom — the LAST
    /// group's last row's bottom minus the list top; the maxScrollAmount
    /// law the band height subtracts: `max(0, contentHeight - height)`
    /// the AbstractScrollArea walk banks; the array order the groups the
    /// DISPLAY walk re-orders — the inventory group the walk's own
    /// bottom keeps (the trio + the hotbar nine the 306-start stride
    /// wear: 12 rows the 11 strides speak, the CREATIVE pair the
    /// 594-header the 606-start rows wear, the SPECTATOR pair the
    /// 654-header the 666-start rows wear, the DEBUG octet the
    /// 714-header the 726-start rows the walk's own bottom keeps
    /// (the overlay the 726 seat, the modifier the 750 seat, the
    /// crash the 774 seat, the reloadChunk the 798 seat, the
    /// showHitboxes the 822 seat, the clearChat the 846 seat, the
    /// showChunkBorders the 870 seat, the showAdvancedTooltips the
    /// 894 seat — the row's own bottom the content answers))
    pub fn content_height() -> i32 {
        DEBUG_ROWS_START + (COUNT - DEBUG_OVERLAY - 1) as i32 * ROW_STRIDE + 20 - LIST_TOP
    }

    /// THE MOUSE'S OWN NAME TABLE (the InputConstants$Type clinit the
    /// seeded map banks — the `addKey(MOUSE, "key.mouse.left", 0)`
    /// walk the pc 75-99 pairs keep: the left the 0, the right the 1,
    /// the middle the 2; the seeded NAME the saveString's own row
    /// wears — the `key.mouse.4..8` rows the buttons 3..7 the seed
    /// keeps, the minifb enum the three the client's own boundary
    /// banks)
    pub const MOUSE_NAMES: &[(MouseButton, &str)] = &[
        (MouseButton::Left, "key.mouse.left"),
        (MouseButton::Right, "key.mouse.right"),
        (MouseButton::Middle, "key.mouse.middle"),
    ];

    /// THE SAVE STRING'S OWN LAW (the `KeyMapping.saveString()` walk —
    /// the `InputConstants$Key.getName()` the row's own format keeps;
    /// the None the UNKNOWN seat the `key.keyboard.unknown` row
    /// answers, the Keyboard the table's own pair speaks, the Mouse
    /// the seeded name wears).
    pub fn bind_save(k: Option<Bind>) -> &'static str {
        match k {
            None => "key.keyboard.unknown",
            Some(Bind::Mouse(b)) => match b {
                MouseButton::Left => "key.mouse.left",
                MouseButton::Right => "key.mouse.right",
                MouseButton::Middle => "key.mouse.middle",
            },
            Some(Bind::Keyboard(k)) => KEYBOARD_NAMES
                .iter()
                .find(|(key, _)| *key == k)
                .map(|(_, n)| *n)
                .unwrap_or("key.keyboard.unknown"),
        }
    }

    /// THE DECODE'S OWN LAW (the `InputConstants.getKey(String)` walk —
    /// the NAME_MAP the FIRST hit answers: the seeded `key.mouse.*`
    /// rows the Mouse speaks, the `key.keyboard.*` rows the Keyboard
    /// keeps; the MISS the prefix walk wears — the `mouse.` prefix the
    /// parseInt MINUS ONE the computed buttons answer (`lambda$
    /// getOrCreate$0` the `mouse. + (value + 1)` names build, the
    /// decode the same offset unwears — the button 8+ the computed
    /// rows the client's own 3-button minifb boundary never carries,
    /// the unknown the None keeps); the table-miss the None the
    /// honest decode wears).
    pub fn bind_from_save(s: &str) -> Option<Bind> {
        if let Some((_, n)) = MOUSE_NAMES.iter().find(|(_, n)| *n == s) {
            return Some(Bind::Mouse(match *n {
                "key.mouse.left" => MouseButton::Left,
                "key.mouse.right" => MouseButton::Right,
                _ => MouseButton::Middle,
            }));
        }
        if let Some(rest) = s.strip_prefix("mouse.") {
            // the prefix walk the computed rows wear — the minus-one
            // the getKey law keeps, the 0/1/2 the minifb enum speaks
            let v: i32 = rest.parse().ok()?;
            return match v - 1 {
                0 => Some(Bind::Mouse(MouseButton::Left)),
                1 => Some(Bind::Mouse(MouseButton::Right)),
                2 => Some(Bind::Mouse(MouseButton::Middle)),
                _ => None,
            };
        }
        KEYBOARD_NAMES
            .iter()
            .find(|(_, n)| *n == s)
            .map(|(k, _)| Bind::Keyboard(*k))
    }

    /// THE KEYBOARD'S OWN NAME TABLE (the InputConstants names the
    /// `key.keyboard.*` rows wear — the minifb Key the row's own pair
    /// keeps; the rebind capture the table's keys the pressed walk
    /// scans (the ESCAPE the clear the isEscape-first law keeps — the
    /// table the decode round-trip serves only), the save/decode the
    /// SAME table the round-trip answers).
    pub const KEYBOARD_NAMES: &[(Key, &str)] = &[
        (Key::Key0, "key.keyboard.0"),
        (Key::Key1, "key.keyboard.1"),
        (Key::Key2, "key.keyboard.2"),
        (Key::Key3, "key.keyboard.3"),
        (Key::Key4, "key.keyboard.4"),
        (Key::Key5, "key.keyboard.5"),
        (Key::Key6, "key.keyboard.6"),
        (Key::Key7, "key.keyboard.7"),
        (Key::Key8, "key.keyboard.8"),
        (Key::Key9, "key.keyboard.9"),
        (Key::A, "key.keyboard.a"),
        (Key::B, "key.keyboard.b"),
        (Key::C, "key.keyboard.c"),
        (Key::D, "key.keyboard.d"),
        (Key::E, "key.keyboard.e"),
        (Key::F, "key.keyboard.f"),
        (Key::G, "key.keyboard.g"),
        (Key::H, "key.keyboard.h"),
        (Key::I, "key.keyboard.i"),
        (Key::J, "key.keyboard.j"),
        (Key::K, "key.keyboard.k"),
        (Key::L, "key.keyboard.l"),
        (Key::M, "key.keyboard.m"),
        (Key::N, "key.keyboard.n"),
        (Key::O, "key.keyboard.o"),
        (Key::P, "key.keyboard.p"),
        (Key::Q, "key.keyboard.q"),
        (Key::R, "key.keyboard.r"),
        (Key::S, "key.keyboard.s"),
        (Key::T, "key.keyboard.t"),
        (Key::U, "key.keyboard.u"),
        (Key::V, "key.keyboard.v"),
        (Key::W, "key.keyboard.w"),
        (Key::X, "key.keyboard.x"),
        (Key::Y, "key.keyboard.y"),
        (Key::Z, "key.keyboard.z"),
        (Key::F1, "key.keyboard.f1"),
        (Key::F2, "key.keyboard.f2"),
        (Key::F3, "key.keyboard.f3"),
        (Key::F4, "key.keyboard.f4"),
        (Key::F5, "key.keyboard.f5"),
        (Key::F6, "key.keyboard.f6"),
        (Key::F7, "key.keyboard.f7"),
        (Key::F8, "key.keyboard.f8"),
        (Key::F9, "key.keyboard.f9"),
        (Key::F10, "key.keyboard.f10"),
        (Key::F11, "key.keyboard.f11"),
        (Key::F12, "key.keyboard.f12"),
        (Key::F13, "key.keyboard.f13"),
        (Key::F14, "key.keyboard.f14"),
        (Key::F15, "key.keyboard.f15"),
        (Key::Down, "key.keyboard.down"),
        (Key::Left, "key.keyboard.left"),
        (Key::Right, "key.keyboard.right"),
        (Key::Up, "key.keyboard.up"),
        (Key::Apostrophe, "key.keyboard.apostrophe"),
        (Key::Backquote, "key.keyboard.grave.accent"),
        (Key::Backslash, "key.keyboard.backslash"),
        (Key::Comma, "key.keyboard.comma"),
        (Key::Equal, "key.keyboard.equal"),
        (Key::LeftBracket, "key.keyboard.left.bracket"),
        (Key::Minus, "key.keyboard.minus"),
        (Key::Period, "key.keyboard.period"),
        (Key::RightBracket, "key.keyboard.right.bracket"),
        (Key::Semicolon, "key.keyboard.semicolon"),
        (Key::Slash, "key.keyboard.slash"),
        (Key::Backspace, "key.keyboard.backspace"),
        (Key::Delete, "key.keyboard.delete"),
        (Key::End, "key.keyboard.end"),
        (Key::Enter, "key.keyboard.enter"),
        (Key::Escape, "key.keyboard.escape"),
        (Key::Home, "key.keyboard.home"),
        (Key::Insert, "key.keyboard.insert"),
        (Key::PageDown, "key.keyboard.page.down"),
        (Key::PageUp, "key.keyboard.page.up"),
        (Key::Pause, "key.keyboard.pause"),
        (Key::Space, "key.keyboard.space"),
        (Key::Tab, "key.keyboard.tab"),
        (Key::NumLock, "key.keyboard.num.lock"),
        (Key::CapsLock, "key.keyboard.caps.lock"),
        (Key::ScrollLock, "key.keyboard.scroll.lock"),
        (Key::LeftShift, "key.keyboard.left.shift"),
        (Key::RightShift, "key.keyboard.right.shift"),
        (Key::LeftCtrl, "key.keyboard.left.control"),
        (Key::RightCtrl, "key.keyboard.right.control"),
        (Key::NumPad0, "key.keyboard.keypad.0"),
        (Key::NumPad1, "key.keyboard.keypad.1"),
        (Key::NumPad2, "key.keyboard.keypad.2"),
        (Key::NumPad3, "key.keyboard.keypad.3"),
        (Key::NumPad4, "key.keyboard.keypad.4"),
        (Key::NumPad5, "key.keyboard.keypad.5"),
        (Key::NumPad6, "key.keyboard.keypad.6"),
        (Key::NumPad7, "key.keyboard.keypad.7"),
        (Key::NumPad8, "key.keyboard.keypad.8"),
        (Key::NumPad9, "key.keyboard.keypad.9"),
        (Key::NumPadDot, "key.keyboard.keypad.decimal"),
        (Key::NumPadSlash, "key.keyboard.keypad.divide"),
        (Key::NumPadAsterisk, "key.keyboard.keypad.multiply"),
        (Key::NumPadMinus, "key.keyboard.keypad.subtract"),
        (Key::NumPadPlus, "key.keyboard.keypad.add"),
        (Key::NumPadEnter, "key.keyboard.keypad.enter"),
        (Key::LeftAlt, "key.keyboard.left.alt"),
        (Key::RightAlt, "key.keyboard.right.alt"),
        (Key::Menu, "key.keyboard.menu"),
        (Key::LeftSuper, "key.keyboard.left.win"),
        (Key::RightSuper, "key.keyboard.right.win"),
    ];
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
            notification_display_time: 1.0,
            key_binds: key_binds::DEFAULTS,
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
                "notificationDisplayTime" => {
                    // THE MULTIPLIER'S OWN DECODE (the doubleRange(0.5,
                    // 10.0) codec the in-range double answers — the
                    // out-of-range/malformed the default keeps, the
                    // clamp never wears; the default 1.0 the Options
                    // ctor's dconst_1 pc 1352 the missing row keeps)
                    if let Ok(v) = value.parse::<f64>() {
                        if (0.5..=10.0).contains(&v) {
                            out.notification_display_time = v;
                        }
                    }
                }
                // THE KEYMAPPINGS' OWN DECODE (the processOptions walk's
                // key loop the pc 1114-1131 the read side serves — the
                // `InputConstants.getKey(value)` the decode wears, the
                // changed string the setKey speaks; the unknown row the
                // None the unbound seat answers, the malformed the same
                // blank the getKey law keeps)
                "key_key.forward" => {
                    out.key_binds[key_binds::FORWARD] = key_binds::bind_from_save(value);
                }
                "key_key.left" => {
                    out.key_binds[key_binds::LEFT] = key_binds::bind_from_save(value);
                }
                "key_key.back" => {
                    out.key_binds[key_binds::BACK] = key_binds::bind_from_save(value);
                }
                "key_key.right" => {
                    out.key_binds[key_binds::RIGHT] = key_binds::bind_from_save(value);
                }
                "key_key.jump" => {
                    out.key_binds[key_binds::JUMP] = key_binds::bind_from_save(value);
                }
                "key_key.sneak" => {
                    out.key_binds[key_binds::SNEAK] = key_binds::bind_from_save(value);
                }
                "key_key.sprint" => {
                    out.key_binds[key_binds::SPRINT] = key_binds::bind_from_save(value);
                }
                "key_key.inventory" => {
                    out.key_binds[key_binds::INVENTORY] = key_binds::bind_from_save(value);
                }
                "key_key.swapOffhand" => {
                    out.key_binds[key_binds::SWAP_OFFHAND] = key_binds::bind_from_save(value);
                }
                "key_key.drop" => {
                    out.key_binds[key_binds::DROP] = key_binds::bind_from_save(value);
                }
                "key_key.use" => {
                    out.key_binds[key_binds::USE] = key_binds::bind_from_save(value);
                }
                "key_key.attack" => {
                    out.key_binds[key_binds::ATTACK] = key_binds::bind_from_save(value);
                }
                "key_key.pickItem" => {
                    out.key_binds[key_binds::PICK_ITEM] = key_binds::bind_from_save(value);
                }
                "key_key.hotbar.1" => {
                    out.key_binds[key_binds::HOTBAR_1] = key_binds::bind_from_save(value);
                }
                "key_key.hotbar.2" => {
                    out.key_binds[key_binds::HOTBAR_2] = key_binds::bind_from_save(value);
                }
                "key_key.hotbar.3" => {
                    out.key_binds[key_binds::HOTBAR_3] = key_binds::bind_from_save(value);
                }
                "key_key.hotbar.4" => {
                    out.key_binds[key_binds::HOTBAR_4] = key_binds::bind_from_save(value);
                }
                "key_key.hotbar.5" => {
                    out.key_binds[key_binds::HOTBAR_5] = key_binds::bind_from_save(value);
                }
                "key_key.hotbar.6" => {
                    out.key_binds[key_binds::HOTBAR_6] = key_binds::bind_from_save(value);
                }
                "key_key.hotbar.7" => {
                    out.key_binds[key_binds::HOTBAR_7] = key_binds::bind_from_save(value);
                }
                "key_key.hotbar.8" => {
                    out.key_binds[key_binds::HOTBAR_8] = key_binds::bind_from_save(value);
                }
                "key_key.hotbar.9" => {
                    out.key_binds[key_binds::HOTBAR_9] = key_binds::bind_from_save(value);
                }
                // THE ACTIVATOR PAIR'S OWN DECODE (the `key_key.` +
                // name prefix the save rows wear — the Toolbar lang
                // names the keys keep, the restart the pair decodes)
                "key_key.saveToolbarActivator" => {
                    out.key_binds[key_binds::SAVE_ACTIVATOR] = key_binds::bind_from_save(value);
                }
                "key_key.loadToolbarActivator" => {
                    out.key_binds[key_binds::LOAD_ACTIVATOR] = key_binds::bind_from_save(value);
                }
                // THE SPECTATOR PAIR'S OWN DECODE (the `key_key.` +
                // name prefix the save rows wear — the UNKNOWN row the
                // None the unbound law keeps, the `key.mouse.middle`
                // row the seeded name the decode answers)
                "key_key.spectatorOutlines" => {
                    out.key_binds[key_binds::SPECTATOR_OUTLINES] = key_binds::bind_from_save(value);
                }
                "key_key.spectatorHotbar" => {
                    out.key_binds[key_binds::SPECTATOR_HOTBAR] = key_binds::bind_from_save(value);
                }
                // THE DEBUG ROW'S OWN DECODE (the `key_key.` + name
                // prefix the save rows wear — the F3 row the restart
                // the decode answers)
                "key_key.debug.overlay" => {
                    out.key_binds[key_binds::DEBUG_OVERLAY] = key_binds::bind_from_save(value);
                }
                // THE DEBUG MODIFIER'S OWN DECODE (the same prefix the
                // row wears — the F3 row the restart the decode
                // answers, the pair the same key the seats keep)
                "key_key.debug.modifier" => {
                    out.key_binds[key_binds::DEBUG_MODIFIER] = key_binds::bind_from_save(value);
                }
                // THE DEBUG CRASH'S OWN DECODE (the same prefix the
                // row wears — the C row the restart the decode
                // answers, the plain-shape row the saveString keeps)
                "key_key.debug.crash" => {
                    out.key_binds[key_binds::DEBUG_CRASH] = key_binds::bind_from_save(value);
                }
                // THE DEBUG RELOAD CHUNK'S OWN DECODE (the same prefix
                // the row wears — the A row the restart the decode
                // answers, the plain-shape row the saveString keeps)
                "key_key.debug.reloadChunk" => {
                    out.key_binds[key_binds::DEBUG_RELOAD_CHUNK] = key_binds::bind_from_save(value);
                }
                // THE DEBUG SHOW HITBOXES' OWN DECODE (the same prefix
                // the row wears — the B row the restart the decode
                // answers, the plain-shape row the saveString keeps)
                "key_key.debug.showHitboxes" => {
                    out.key_binds[key_binds::DEBUG_SHOW_HITBOXES] =
                        key_binds::bind_from_save(value);
                }
                // THE DEBUG CLEAR CHAT'S OWN DECODE (the same prefix
                // the row wears — the D row the restart the decode
                // answers, the plain-shape row the saveString keeps)
                "key_key.debug.clearChat" => {
                    out.key_binds[key_binds::DEBUG_CLEAR_CHAT] = key_binds::bind_from_save(value);
                }
                // THE DEBUG SHOW CHUNK BORDERS' OWN DECODE (the same
                // prefix the row wears — the G row the restart the
                // decode answers, the plain-shape row the saveString
                // keeps)
                "key_key.debug.showChunkBorders" => {
                    out.key_binds[key_binds::DEBUG_SHOW_CHUNK_BORDERS] =
                        key_binds::bind_from_save(value);
                }
                // THE DEBUG SHOW ADVANCED TOOLTIPS' OWN DECODE (the
                // same prefix the row wears — the H row the restart
                // the decode answers, the plain-shape row the
                // saveString keeps)
                "key_key.debug.showAdvancedTooltips" => {
                    out.key_binds[key_binds::DEBUG_SHOW_ADV_TOOLTIPS] =
                        key_binds::bind_from_save(value);
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
        // THE MULTIPLIER'S OWN ROW (the double the java_double print
        // keeps — the 1.0 the default the decimal speaks; the row the
        // visit-order seat the processOptions walk keeps — the
        // notificationDisplayTime the ldc@682 the walk's LAST carried
        // seat wears, the pauseOnLostFocus@567 the row BEFORE the same
        // ladder speaks, the file's own END the row answers)
        text.push_str(&format!(
            "notificationDisplayTime:{}\n",
            java_double(self.notification_display_time)
        ));
        // THE KEYMAPPINGS' OWN BLOCK (the processOptions walk's key
        // loop — the pc 1064-1137 the `keyMappings` array iterates, the
        // row the `"key_" + name` prefix the indy composition + the
        // saveString value wears, the block the option rows' AFTER the
        // soundCategory rows' BEFORE the vanilla ladder seats — the
        // client's own partial ladder the END the block wears, the
        // soundCategory_master seat the earlier rounds' order keeps)
        for i in 0..key_binds::COUNT {
            text.push_str(&format!(
                "key_{}:{}\n",
                key_binds::NAMES[i],
                key_binds::bind_save(self.key_binds[i])
            ));
        }
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
pub(crate) fn java_double(v: f64) -> String {
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
            text, "version:4903\nshowSubtitles:false\nmouseSensitivity:0.5\nfov:70\nsoundCategory_master:1.0\npauseOnLostFocus:true\nfovEffectScale:1.0\nchatOpacity:1.0\nchatLineSpacing:0.0\ntextBackgroundOpacity:0.5\nbackgroundForChatOnly:true\nnotificationDisplayTime:1.0\nkey_key.forward:key.keyboard.w\nkey_key.left:key.keyboard.a\nkey_key.back:key.keyboard.s\nkey_key.right:key.keyboard.d\nkey_key.jump:key.keyboard.space\nkey_key.sneak:key.keyboard.left.shift\nkey_key.sprint:key.keyboard.left.control\nkey_key.inventory:key.keyboard.e\nkey_key.swapOffhand:key.keyboard.f\nkey_key.drop:key.keyboard.q\nkey_key.use:key.mouse.right\nkey_key.attack:key.mouse.left\nkey_key.pickItem:key.mouse.middle\nkey_key.hotbar.1:key.keyboard.1\nkey_key.hotbar.2:key.keyboard.2\nkey_key.hotbar.3:key.keyboard.3\nkey_key.hotbar.4:key.keyboard.4\nkey_key.hotbar.5:key.keyboard.5\nkey_key.hotbar.6:key.keyboard.6\nkey_key.hotbar.7:key.keyboard.7\nkey_key.hotbar.8:key.keyboard.8\nkey_key.hotbar.9:key.keyboard.9\nkey_key.saveToolbarActivator:key.keyboard.c\nkey_key.loadToolbarActivator:key.keyboard.x\nkey_key.spectatorOutlines:key.keyboard.unknown\nkey_key.spectatorHotbar:key.mouse.middle\nkey_key.debug.overlay:key.keyboard.f3\nkey_key.debug.modifier:key.keyboard.f3\nkey_key.debug.crash:key.keyboard.c\nkey_key.debug.reloadChunk:key.keyboard.a\nkey_key.debug.showHitboxes:key.keyboard.b\nkey_key.debug.clearChat:key.keyboard.d\nkey_key.debug.showChunkBorders:key.keyboard.g\nkey_key.debug.showAdvancedTooltips:key.keyboard.h\n",
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
        assert_eq!(
            loaded.notification_display_time, 1.0,
            "the multiplier the default the round-trip wears"
        );
        // THE KEYMAPPINGS' OWN ROUND-TRIP (the save strings the defaults
        // print, the decode the same keys the read answers — the ctor's
        // codes the array order keeps)
        assert_eq!(
            loaded.key_binds,
            crate::options_file::key_binds::DEFAULTS,
            "the key rows the defaults the round-trip wears"
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
             backgroundForChatOnly:true\n\
             notificationDisplayTime:2.5\n\
             notificationDisplayTime:0.25\n\
             notificationDisplayTime:10.5\n\
             notificationDisplayTime:word\n\
             notificationDisplayTime:0.5\n\
             key_key.forward:key.keyboard.l\n\
             key_key.forward:garbage\n\
             key_key.forward:key.keyboard.q\n\
             key_key.sneak:key.keyboard.unknown\n\
             key_key.sprint:garbage\n",
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
        // THE MULTIPLIER'S OWN DECODE (the same strict double the
        // in-range 2.5 the first read wears — the out-of-range 0.25/
        // 10.5 pair + the malformed word the skip keeps, the LAST
        // valid the 0.5 the read banks; the codec the 0.5..=10.0
        // range the strict law keeps, the clamp never wears)
        assert_eq!(
            loaded.notification_display_time, 0.5,
            "the multiplier the last valid row wears — the out-of-range/malformed the skip keeps"
        );
        // THE KEYMAPPINGS' OWN DECODE (the last valid row the parse
        // wears — the rebind the file's own truth speaks: the forward
        // the LAST valid `key.keyboard.q` the read banks, the UNKNOWN
        // row the sneak the None seat answers, the `garbage` row the
        // sprint the same blank the getKey law keeps, the missing
        // rows the defaults the load law keeps)
        assert_eq!(
            loaded.key_binds[crate::options_file::key_binds::FORWARD],
            Some(crate::options_file::key_binds::Bind::Keyboard(
                minifb::Key::Q
            )),
            "the forward the last valid row wears"
        );
        assert_eq!(
            loaded.key_binds[crate::options_file::key_binds::SNEAK],
            None,
            "the UNKNOWN row the unbound seat keeps"
        );
        assert_eq!(
            loaded.key_binds[crate::options_file::key_binds::SPRINT],
            None,
            "the garbage row the blank bind the getKey law keeps"
        );
        assert_eq!(
            loaded.key_binds[crate::options_file::key_binds::JUMP],
            Some(crate::options_file::key_binds::Bind::Keyboard(
                minifb::Key::Space
            )),
            "the missing row the default keeps"
        );
        assert_eq!(
            loaded.key_binds[crate::options_file::key_binds::LEFT],
            Some(crate::options_file::key_binds::Bind::Keyboard(
                minifb::Key::A
            )),
            "the missing row the default keeps"
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
        assert_eq!(
            missing.notification_display_time, 1.0,
            "the missing file the default multiplier keeps"
        );
        assert_eq!(
            missing.key_binds,
            crate::options_file::key_binds::DEFAULTS,
            "the missing file the default key rows keeps"
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

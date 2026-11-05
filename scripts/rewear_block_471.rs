#[test]
fn the_anvil_field_the_clipboard_walk_wears() {
    let mut app = App::new(3);
    app.enter_world();
    // THE DOOR'S OWN OPEN (the fresh field the tail seat wears)
    let size = app.offline_world.as_ref().unwrap().size_blocks();
    let c = size / 2;
    let surface = app.spawn_y - 1;
    let cell = (c, surface + 1, c);
    app.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::ANVIL);
    app.open_anvil_for(cell);
    let sword = mc_registry::item_id_by_name("iron_sword").unwrap();
    app.anvil_slots[0] = (sword, 1);
    for ch in "Doom Bringer".chars() {
        app.on_anvil_char(ch);
    }
    // THE COPY'S OWN RANGE LAW (the 471 re-wear the 470 isCopy arm's
    // law the family answers — the [0, 2) live range the "Do" the
    // store banks, the whole-value fold the standing law the box
    // never wears)
    app.on_anvil_caret_home_end(false);
    app.on_anvil_caret_shift_arrow(false); // the right walk the seat-1 rides
    app.on_anvil_caret_shift_arrow(false); // the second step the seat-2 banks
    assert_eq!(app.anvil_caret, 2, "the shift walk the seat-2 wears");
    assert_eq!(app.anvil_highlight, Some(0), "the range the anchor banks");
    app.on_anvil_copy();
    assert_eq!(
        app.clipboard, "Do",
        "the highlighted range the copy banks [the whole-value fold the box never wears]"
    );
    assert_eq!(
        app.anvil_name, "Doom Bringer",
        "the copy the field never trims"
    );
    assert_eq!(app.anvil_caret, 2, "the copy the caret never moves");
    // THE CUT'S OWN RANGE REMOVAL (the store the range banks first,
    // the insertText("") walk the live range alone clears — the
    // whole-value wipe the standing law the box never wears)
    app.on_anvil_cut();
    assert_eq!(app.clipboard, "Do", "the cut the range the store banks");
    assert_eq!(
        app.anvil_name, "om Bringer",
        "the cut the range alone removes [the whole-value wipe the box never wears]"
    );
    assert_eq!(app.anvil_caret, 0, "the removal the lo seat parks");
    assert_eq!(app.anvil_highlight, None, "the removal the anchor drops");
    // THE COLLAPSED CUT'S OWN NO-OP (the replace(lo, lo, "") the value
    // never moves — the cut the box the value keeps, the store the
    // empty word re-banks)
    app.on_anvil_cut();
    assert_eq!(app.clipboard, "", "the collapsed cut the store empties");
    assert_eq!(
        app.anvil_name, "om Bringer",
        "the collapsed cut the value keeps"
    );
    assert_eq!(app.anvil_caret, 0, "the no-op the seat keeps");
    // THE SELECTION'S OWN REPLACE (the select-all the [0, len) range
    // opens — the store's word the range the answer replaces, the
    // caret the inserted walk the tail rides)
    app.clipboard = "Doom Bringer".to_string();
    app.on_anvil_select_all();
    app.on_anvil_paste();
    assert_eq!(
        app.anvil_name, "Doom Bringer",
        "the paste the range replaces"
    );
    assert_eq!(app.anvil_caret, 12, "the replace the caret rides");
    assert_eq!(app.anvil_highlight, None, "the replace the anchor drops");
    // THE EMPTY-COPY'S OWN UNGUARDED BANK (the collapsed pair the
    // EMPTY word the store wears — the whole-value fold the box
    // never wears)
    app.clipboard = "sentinel".to_string();
    app.on_anvil_copy();
    assert_eq!(
        app.clipboard, "",
        "the collapsed copy the store empties [the unguarded write]"
    );
    // THE MID-SEAT'S OWN INSERT (the second door the word the seat-1
    // the walk answers — the caret the +1 step banks)
    let mut app2 = App::new(3);
    app2.enter_world();
    app2.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::ANVIL);
    app2.open_anvil_for(cell);
    app2.anvil_slots[0] = (sword, 1);
    for ch in "Doom".chars() {
        app2.on_anvil_char(ch);
    }
    app2.on_anvil_caret_home_end(false);
    app2.on_anvil_caret_arrow(false); // the right walk the seat-1 parks
    app2.clipboard = "X".to_string();
    app2.on_anvil_paste();
    assert_eq!(app2.anvil_name, "DXoom", "the mid-seat the X inserts");
    assert_eq!(app2.anvil_caret, 2, "the insert the seat-2 banks");
    // THE SELECTION'S OWN REPLACE (the [0, 2) live range the paste the
    // first step removes — the 'Q' the lo seat the walk seeds)
    app2.on_anvil_caret_home_end(false);
    app2.on_anvil_caret_shift_arrow(false); // the right walk the range opens
    app2.on_anvil_caret_shift_arrow(false);
    assert_eq!(app2.anvil_highlight, Some(0), "the range re-banks");
    app2.clipboard = "Q".to_string();
    app2.on_anvil_paste();
    assert_eq!(app2.anvil_name, "Qoom", "the range the Q replaces");
    assert_eq!(app2.anvil_caret, 1, "the replace the lo seat banks");
    assert_eq!(app2.anvil_highlight, None, "the replace the anchor drops");
    // THE EMPTY STORE'S OWN WALK (the no-empty-guard the 26.2 bytes
    // speak — the live range the removal alone serves, the word the
    // store the silence keeps)
    app2.on_anvil_caret_home_end(false);
    app2.on_anvil_caret_shift_arrow(false); // the range the removal answers
    app2.on_anvil_caret_shift_arrow(false);
    app2.clipboard = String::new();
    app2.on_anvil_paste();
    assert_eq!(
        app2.anvil_name, "om",
        "the empty store the live range alone removes"
    );
    assert_eq!(app2.anvil_caret, 0, "the removal the lo seat parks");
    assert_eq!(
        app2.anvil_highlight, None,
        "the removal the anchor drops"
    );
    // THE CAP'S OWN CLAMP (the 60-char flood the tail appends — the
    // 50-tail the walk wears, the second paste the cap re-answers)
    app2.on_anvil_caret_home_end(true);
    app2.clipboard = "y".repeat(60);
    app2.on_anvil_paste();
    assert_eq!(
        app2.anvil_name.chars().count(),
        50,
        "the flood the cap clamps"
    );
    assert_eq!(app2.anvil_caret, 50, "the clamp the caret parks");
    app2.on_anvil_paste();
    assert_eq!(
        app2.anvil_name.chars().count(),
        50,
        "the second flood the cap re-answers"
    );
    assert_eq!(app2.anvil_caret, 50, "the re-answer the caret keeps");
    // THE STRIP + THE IDENTITY'S OWN DOOR (the third door the fresh
    // field the walks wear)
    let mut app3 = App::new(3);
    app3.enter_world();
    app3.offline_world
        .as_mut()
        .unwrap()
        .set(cell.0, cell.1, cell.2, crate::world::ANVIL);
    app3.open_anvil_for(cell);
    app3.anvil_slots[0] = (sword, 1);
    // THE RESPONDER'S OWN STRIP RIDE (the DEL char the filter
    // re-shapes — the AB the head the walk seeds, the seat the fresh
    // bounds the clamp keeps)
    app3.clipboard = "A\u{7f}B".to_string();
    app3.on_anvil_paste();
    assert_eq!(app3.anvil_name, "AB", "the strip the AB the seeds");
    assert_eq!(app3.anvil_caret, 2, "the strip the clamp re-keys");
    // THE IDENTITY RESET'S OWN RIDE (the select-all the [0, 2) range
    // opens — the cut the range alone removes, the display word the
    // paste the plain stack's own hover law answers — the empty word
    // the reset wears)
    app3.on_anvil_select_all();
    app3.on_anvil_cut();
    assert_eq!(
        app3.clipboard, "AB",
        "the cut the range the store banks"
    );
    assert_eq!(app3.anvil_name, "", "the cut the whole range removes");
    app3.clipboard = "Iron Sword".to_string();
    app3.on_anvil_paste();
    assert_eq!(app3.anvil_name, "", "the display word the reset wears");
    assert_eq!(app3.anvil_caret, 0, "the reset the caret parks");
    // THE GATE'S OWN SILENCE (the stale screen the three walks the
    // store never serves)
    app3.screen = ScreenKind::World;
    app3.clipboard = "the gate never serves".to_string();
    app3.on_anvil_copy();
    app3.on_anvil_paste();
    app3.on_anvil_cut();
    assert_eq!(
        app3.clipboard, "the gate never serves",
        "the gate the store never touches"
    );
    assert_eq!(app3.anvil_name, "", "the gate the word never wears");
}

// THE CHAT FIELD'S OWN CLIPBOARD SIBLING WALK (the 471 re-wear the
// 470 census's isCopy/isPaste/isCut arms the 26.2 bytes speak — the
// 468 debug law the family re-wears: the highlighted range the copy
// banks [the collapsed pair the EMPTY word the unguarded write
// wears], the insertText("") walk the live range alone removes, the
// no-empty-guard the paste walk wears, the 256 cap the tail clamps,
// the is_control refusal the paste word wears, the closed chat the
// gate never passes).
#[test]
fn the_chat_field_the_clipboard_sibling_wears() {
    let mut app = App::new(3);
    app.enter_world();
    // THE DOOR'S OWN OPEN (the fresh field the empty store wears)
    app.chat_open = true;
    app.chat_cursor = 0;
    app.chat_highlight = None;
    for ch in "hi there".chars() {
        app.on_chat_char(ch);
    }
    assert_eq!(app.chat_input, "hi there", "the typed word the seat seeds");
    assert_eq!(app.chat_cursor, 8, "the typed walk the caret rides");
    // THE SELECT-ALL'S OWN LIVE RANGE (the [0, len) the field wears)
    app.on_chat_select_all();
    assert_eq!(app.chat_highlight, Some(0), "the all the anchor parks");
    // THE COPY'S OWN RANGE LAW (the [0, 8) range the whole value the
    // store banks — the range law the walk answers)
    app.on_chat_copy();
    assert_eq!(
        app.clipboard, "hi there",
        "the [0, 8) range the copy banks [the range law the walk answers]"
    );
    assert_eq!(app.chat_input, "hi there", "the copy the field never trims");
    assert_eq!(app.chat_cursor, 8, "the copy the caret never moves");
    // THE CUT'S OWN RANGE REMOVAL (the [0, 8) range the whole value
    // clears, the caret the lo seat parks, the draft disclaims)
    app.chat_input_is_draft = true;
    app.on_chat_cut();
    assert_eq!(
        app.clipboard, "hi there",
        "the cut the range the store banks"
    );
    assert_eq!(
        app.chat_input, "",
        "the cut the range alone removes [the whole-value wipe the box never wears]"
    );
    assert_eq!(app.chat_cursor, 0, "the removal the lo seat parks");
    assert_eq!(app.chat_highlight, None, "the removal the anchor drops");
    assert!(!app.chat_input_is_draft, "the edit the draft disclaims");
    // THE PASTE'S OWN SEAT INSERT (the empty field the store's word
    // seeds, the caret the tail rides)
    app.on_chat_paste();
    assert_eq!(app.chat_input, "hi there", "the paste the word seeds");
    assert_eq!(app.chat_cursor, 8, "the paste the caret rides");
    // THE MID-SEAT'S OWN INSERT (the HOME the seat 0 parks, the
    // paste the head the word wears)
    app.on_chat_caret_home_end(false);
    assert_eq!(app.chat_cursor, 0, "the home the seat parks");
    app.on_chat_paste();
    assert_eq!(
        app.chat_input, "hi therehi there",
        "the paste the head wears"
    );
    assert_eq!(app.chat_cursor, 8, "the insert the caret the tail rides");
    // THE SELECTION'S OWN REPLACE (the live range the first step
    // removes, the store's word the range the answer replaces)
    app.on_chat_select_all();
    app.on_chat_paste();
    assert_eq!(app.chat_input, "hi there", "the paste the range replaces");
    assert_eq!(app.chat_cursor, 8, "the replace the caret rides");
    // THE EMPTY STORE'S OWN WALK (the no-empty-guard the 26.2 bytes
    // speak — the live range the removal alone serves)
    app.clipboard = String::new();
    app.on_chat_select_all();
    app.on_chat_paste();
    assert_eq!(
        app.chat_input, "",
        "the empty store the live range alone removes"
    );
    assert_eq!(app.chat_cursor, 0, "the removal the lo seat parks");
    assert_eq!(
        app.chat_highlight, None,
        "the removal the anchor drops"
    );
    // THE EMPTY COPY'S OWN UNGUARDED BANK (the collapsed pair the
    // EMPTY word the store wears)
    app.clipboard = "sentinel".to_string();
    app.on_chat_copy();
    assert_eq!(
        app.clipboard, "",
        "the collapsed copy the store empties [the unguarded write]"
    );
    // THE COLLAPSED CUT'S OWN NO-OP (the value the no-op keeps, the
    // store the empty word re-banks, the draft the responder keeps)
    app.chat_input_is_draft = true;
    app.on_chat_cut();
    assert_eq!(app.clipboard, "", "the collapsed cut the store empties");
    assert_eq!(app.chat_input, "", "the collapsed cut the value keeps");
    assert_eq!(app.chat_cursor, 0, "the no-op the seat keeps");
    assert_eq!(app.chat_highlight, None, "the re-park the anchor drops");
    assert!(
        app.chat_input_is_draft,
        "the equal word the responder never fires [the draft stays]"
    );
    // THE CONTROL CHAR'S OWN DROP (the typed-char refusal the paste
    // word wears — the \n the strip answers, the rest seeds)
    app.clipboard = "A\nB".to_string();
    app.on_chat_paste();
    assert_eq!(app.chat_input, "AB", "the control strip the AB seeds");
    assert_eq!(app.chat_cursor, 2, "the strip the caret re-keys");
    // THE CAP'S OWN CLAMP (the 256 the chat's own setMaxLength wears —
    // the flood the tail clamps, the second paste the cap re-answers)
    let flood: String = "x".repeat(300);
    app.clipboard = flood.clone();
    app.on_chat_paste();
    assert_eq!(
        app.chat_input.chars().count(),
        256,
        "the flood the tail clamps"
    );
    assert_eq!(app.chat_cursor, 256, "the clamp the caret parks");
    app.on_chat_paste();
    assert_eq!(
        app.chat_input.chars().count(),
        256,
        "the second paste the cap re-answers"
    );
    // THE FRESH FIELD'S OWN SECOND VERDICT (the standalone app the
    // cap walk re-answers at the empty seed)
    let mut app2 = App::new(3);
    app2.enter_world();
    app2.chat_open = true;
    app2.clipboard = flood;
    app2.on_chat_paste();
    assert_eq!(
        app2.chat_input.chars().count(),
        256,
        "the fresh seed the cap keeps"
    );
    // THE GATE'S OWN SILENCE (the closed chat the three walks the
    // store and field never serve)
    let app3 = App::new(3);
    let mut app3 = app3;
    app3.enter_world();
    app3.clipboard = "the gate never passes".to_string();
    app3.on_chat_copy();
    app3.on_chat_paste();
    app3.on_chat_cut();
    assert_eq!(
        app3.clipboard, "the gate never passes",
        "the gate the store never touches"
    );
    assert_eq!(app3.chat_input, "", "the gate the field never wears");
    assert_eq!(app3.chat_cursor, 0, "the gate the caret never moves");
}

// THE DIRECT FIELD'S OWN CLIPBOARD SIBLING WALK (the 471 re-wear the
// 470 census's isCopy/isPaste/isCut arms the 26.2 bytes speak — the
// 468 debug law the family re-wears: the highlighted range the copy
// banks [the collapsed pair the EMPTY word the unguarded write
// wears], the insertText("") walk the live range alone removes, the
// no-empty-guard the paste walk wears, the 128 cap the tail clamps,
// the unfocused box the stale screen the gate never passes).
#[test]
fn the_direct_field_the_clipboard_sibling_wears() {
    let mut app = App::new(3);
    app.open(ScreenKind::DirectConnect);
    // THE DOOR'S OWN OPEN (the fresh form the empty field the focus
    // arms)
    assert!(app.direct_focus, "the fresh form the focus arms");
    for ch in "mc.example.org".chars() {
        app.on_direct_char(ch);
    }
    assert_eq!(
        app.direct_address, "mc.example.org",
        "the typed word the seat seeds"
    );
    // THE SELECT-ALL'S OWN LIVE RANGE (the [0, len) the field wears)
    app.on_direct_select_all();
    assert_eq!(app.direct_highlight, Some(0), "the all the anchor parks");
    // THE COPY'S OWN RANGE LAW (the [0, 14) range the whole value the
    // store banks — the range law the walk answers)
    app.on_direct_copy();
    assert_eq!(
        app.clipboard, "mc.example.org",
        "the [0, 14) range the copy banks [the range law the walk answers]"
    );
    assert_eq!(
        app.direct_address, "mc.example.org",
        "the copy the field never trims"
    );
    // THE CUT'S OWN RANGE REMOVAL (the [0, 14) range the whole value
    // clears, the caret the lo seat parks, the anchor drops)
    app.on_direct_cut();
    assert_eq!(
        app.clipboard, "mc.example.org",
        "the cut the range the store banks"
    );
    assert_eq!(
        app.direct_address, "",
        "the cut the range alone removes [the whole-value wipe the box never wears]"
    );
    assert_eq!(app.direct_cursor, 0, "the removal the lo seat parks");
    assert_eq!(app.direct_highlight, None, "the removal the anchor drops");
    // THE PASTE'S OWN SEAT INSERT (the empty field the store's word
    // seeds, the caret the tail rides)
    app.on_direct_paste();
    assert_eq!(
        app.direct_address, "mc.example.org",
        "the paste the word seeds"
    );
    assert_eq!(app.direct_cursor, 14, "the paste the caret rides");
    // THE MID-SEAT'S OWN INSERT (the HOME the seat 0 parks, the
    // paste the head the word wears)
    app.on_direct_caret_home_end(false);
    assert_eq!(app.direct_cursor, 0, "the home the seat parks");
    app.on_direct_paste();
    assert_eq!(
        app.direct_address, "mc.example.orgmc.example.org",
        "the paste the head wears"
    );
    assert_eq!(app.direct_cursor, 14, "the insert the caret rides");
    // THE SELECTION'S OWN REPLACE (the live range the first step
    // removes, the store's word the range the answer replaces)
    app.on_direct_select_all();
    app.on_direct_paste();
    assert_eq!(
        app.direct_address, "mc.example.org",
        "the paste the range replaces"
    );
    // THE EMPTY STORE'S OWN WALK (the no-empty-guard the 26.2 bytes
    // speak — the live range the removal alone serves)
    app.clipboard = String::new();
    app.on_direct_select_all();
    app.on_direct_paste();
    assert_eq!(
        app.direct_address, "",
        "the empty store the live range alone removes"
    );
    assert_eq!(app.direct_cursor, 0, "the removal the lo seat parks");
    assert_eq!(
        app.direct_highlight, None,
        "the removal the anchor drops"
    );
    // THE EMPTY COPY'S OWN UNGUARDED BANK (the collapsed pair the
    // EMPTY word the store wears)
    app.clipboard = "sentinel".to_string();
    app.on_direct_copy();
    assert_eq!(
        app.clipboard, "",
        "the collapsed copy the store empties [the unguarded write]"
    );
    // THE COLLAPSED CUT'S OWN NO-OP (the value the no-op keeps, the
    // store the empty word re-banks)
    app.on_direct_cut();
    assert_eq!(app.clipboard, "", "the collapsed cut the store empties");
    assert_eq!(app.direct_address, "", "the collapsed cut the value keeps");
    assert_eq!(app.direct_cursor, 0, "the no-op the seat keeps");
    assert_eq!(
        app.direct_highlight, None,
        "the re-park the anchor drops"
    );
    // THE CAP'S OWN CLAMP (the 128 the entry box's own setMaxLength
    // wears — the flood the tail clamps, the second paste the cap
    // re-answers)
    let flood: String = "x".repeat(150);
    app.clipboard = flood.clone();
    app.on_direct_paste();
    assert_eq!(
        app.direct_address.chars().count(),
        128,
        "the flood the tail clamps"
    );
    assert_eq!(app.direct_cursor, 128, "the clamp the caret parks");
    app.on_direct_paste();
    assert_eq!(
        app.direct_address.chars().count(),
        128,
        "the second paste the cap re-answers"
    );
    // THE FRESH FORM'S OWN SECOND VERDICT (the standalone app the
    // cap walk re-answers at the empty seed)
    let mut app2 = App::new(3);
    app2.open(ScreenKind::DirectConnect);
    app2.clipboard = flood;
    app2.on_direct_paste();
    assert_eq!(
        app2.direct_address.chars().count(),
        128,
        "the fresh seed the cap keeps"
    );
    // THE GATE'S OWN SILENCE (the stale screen the three walks the
    // store and field never serve)
    let mut app3 = App::new(3);
    app3.open(ScreenKind::DirectConnect);
    app3.screen = ScreenKind::Servers;
    app3.clipboard = "the gate never passes".to_string();
    app3.on_direct_copy();
    app3.on_direct_paste();
    app3.on_direct_cut();
    assert_eq!(
        app3.clipboard, "the gate never passes",
        "the gate the store never touches"
    );
    assert_eq!(app3.direct_address, "", "the gate the field never wears");
}

// THE EDITSERVER PAIR'S OWN CLIPBOARD SIBLINGS WALK (the 471 re-wear
// the 470 census's isCopy/isPaste/isCut arms the 26.2 bytes speak —
// the 468 debug law the pair re-wears: the focused box the
// highlighted range the copy banks [the collapsed pair the EMPTY
// word the unguarded write wears], the insertText("") walk the live
// range alone removes, the no-empty-guard the paste walk wears, the
// name's 32 / the address's 128 caps the tail clamps, the
// neither-focus arm the walk never answers).
#[test]
fn the_editserver_pair_the_clipboard_siblings_wear() {
    let mut app = App::new(3);
    app.open(ScreenKind::EditServer);
    // THE DOOR'S OWN OPEN (the fresh ctor the defaultName the name
    // box wears, the focus the name box arms)
    assert!(app.es_name_focus, "the fresh form the name focus arms");
    assert!(!app.es_address_focus, "the address box the focus lacks");
    assert_eq!(
        app.es_name, "Minecraft Server",
        "the defaultName the fresh ctor keeps"
    );
    // THE COPY'S OWN RANGE LAW (the select-all the [0, 16) range
    // opens — the whole value the store banks, the range law the
    // walk answers)
    app.on_editserver_select_all();
    app.on_editserver_copy();
    assert_eq!(
        app.clipboard, "Minecraft Server",
        "the [0, 16) range the copy banks [the range law the walk answers]"
    );
    assert_eq!(
        app.es_name, "Minecraft Server",
        "the copy the field never trims"
    );
    // THE CUT'S OWN RANGE REMOVAL (the [0, 16) range the whole value
    // clears, the caret the lo seat parks, the anchor drops)
    app.on_editserver_cut();
    assert_eq!(
        app.clipboard, "Minecraft Server",
        "the cut the range the store banks"
    );
    assert_eq!(
        app.es_name, "",
        "the cut the range alone removes [the whole-value wipe the box never wears]"
    );
    assert_eq!(app.es_name_cursor, 0, "the removal the lo seat parks");
    assert_eq!(app.es_name_highlight, None, "the removal the anchor drops");
    // THE PASTE'S OWN SEAT INSERT (the empty name the store's word
    // seeds, the caret the tail rides)
    app.on_editserver_paste();
    assert_eq!(app.es_name, "Minecraft Server", "the paste the word seeds");
    assert_eq!(app.es_name_cursor, 16, "the paste the caret rides");
    // THE NAME'S OWN CAP (the 32 the name box's setMaxLength wears —
    // the flood the tail clamps)
    app.es_name.clear();
    app.es_name_cursor = 0;
    app.clipboard = "y".repeat(40);
    app.on_editserver_paste();
    assert_eq!(
        app.es_name.chars().count(),
        32,
        "the flood the name cap clamps"
    );
    // THE FOCUS'S OWN TRANSFER (the address box the focused seat
    // wears — the pair's exclusivity the flags speak)
    app.es_name_focus = false;
    app.es_address_focus = true;
    // THE ADDRESS ARM'S OWN SEED (the empty box the store's word
    // wears — the store the name's own value keeps)
    app.clipboard = "mc.example.net".to_string();
    app.on_editserver_paste();
    assert_eq!(
        app.es_address, "mc.example.net",
        "the address box the word seeds"
    );
    assert_eq!(app.es_address_cursor, 14, "the seed the caret rides");
    // THE ADDRESS'S OWN RANGE COPY + CUT + PASTE (the [0, 14) range
    // the store banks, the range the cut alone removes, the
    // re-paste the seat seeds)
    app.on_editserver_select_all();
    app.on_editserver_copy();
    assert_eq!(
        app.clipboard, "mc.example.net",
        "the address [0, 14) range the copy banks"
    );
    app.on_editserver_cut();
    assert_eq!(
        app.es_address, "",
        "the cut the range alone removes [the whole-value wipe the box never wears]"
    );
    assert_eq!(
        app.es_address_cursor, 0,
        "the removal the lo seat parks"
    );
    app.on_editserver_paste();
    assert_eq!(
        app.es_address, "mc.example.net",
        "the re-paste the word seeds"
    );
    // THE EMPTY STORE'S OWN WALK (the no-empty-guard the 26.2 bytes
    // speak — the live range the removal alone serves)
    app.clipboard = String::new();
    app.on_editserver_select_all();
    app.on_editserver_paste();
    assert_eq!(
        app.es_address, "",
        "the empty store the live range alone removes"
    );
    assert_eq!(
        app.es_address_cursor, 0,
        "the removal the lo seat parks"
    );
    assert_eq!(
        app.es_address_highlight, None,
        "the removal the anchor drops"
    );
    // THE EMPTY COPY'S OWN UNGUARDED BANK (the collapsed pair the
    // EMPTY word the store wears — the address focus the empty arm)
    app.clipboard = "sentinel".to_string();
    app.on_editserver_copy();
    assert_eq!(
        app.clipboard, "",
        "the collapsed copy the store empties [the unguarded write]"
    );
    // THE COLLAPSED CUT'S OWN NO-OP (the value the no-op keeps, the
    // store the empty word re-banks)
    app.on_editserver_cut();
    assert_eq!(app.clipboard, "", "the collapsed cut the store empties");
    assert_eq!(app.es_address, "", "the collapsed cut the value keeps");
    assert_eq!(app.es_address_cursor, 0, "the no-op the seat keeps");
    // THE NEITHER-FOCUS ARM'S OWN SILENCE (the pair's both-clear
    // state the three walks never answer)
    app.es_address_focus = false;
    app.clipboard = "the neither-focus arm".to_string();
    app.es_address = "untouched".to_string();
    app.es_address_cursor = 9;
    app.on_editserver_copy();
    app.on_editserver_paste();
    app.on_editserver_cut();
    assert_eq!(
        app.clipboard, "the neither-focus arm",
        "the neither-focus the store never touches"
    );
    assert_eq!(
        app.es_address, "untouched",
        "the neither-focus the boxes never wear"
    );
    // THE GATE'S OWN SILENCE (the stale screen the three walks the
    // store and boxes never serve)
    app.es_address_focus = true;
    app.screen = ScreenKind::Servers;
    app.on_editserver_copy();
    app.on_editserver_paste();
    app.on_editserver_cut();
    assert_eq!(
        app.clipboard, "the neither-focus arm",
        "the gate the store never touches"
    );
    assert_eq!(app.es_address, "untouched", "the gate the boxes never wear");
}

// THE CREATIVE SEARCH'S OWN CLIPBOARD SIBLING WALK (the 471 re-wear
// the 470 census's isCopy/isPaste/isCut arms the 26.2 bytes speak —
// the 468 debug law the box re-wears: the highlighted range the copy
// banks [the collapsed pair the EMPTY word the unguarded write
// wears], the insertText("") walk the live range alone removes [the
// refresh + the scroll reset the same tails the backspace wears],
// the no-empty-guard the paste walk wears, the 50 cap the tail
// clamps, the non-search tab the gate never passes).
#[test]
fn the_creative_search_the_clipboard_sibling_wears() {
    let mut app = App::new(3);
    // THE DOOR'S OWN OPEN (the picker the tab-6 seat arms, the typed
    // word the box keeps)
    app.open_creative_picker();
    app.select_creative_tab(6);
    assert_eq!(app.creative_tab, 6, "the search tab the seat wears");
    for c in "stone".chars() {
        app.on_creative_char(c);
    }
    assert_eq!(
        app.creative_search, "stone",
        "the typed word the seat seeds"
    );
    // THE SELECT-ALL'S OWN LIVE RANGE (the [0, len) the box wears)
    app.on_creative_select_all();
    assert_eq!(
        app.creative_search_highlight,
        Some(0),
        "the all the anchor parks"
    );
    // THE COPY'S OWN RANGE LAW (the [0, 5) range the whole value the
    // store banks — the range law the walk answers)
    app.on_creative_copy();
    assert_eq!(
        app.clipboard, "stone",
        "the [0, 5) range the copy banks [the range law the walk answers]"
    );
    assert_eq!(app.creative_search, "stone", "the copy the box never trims");
    // THE CUT'S OWN RANGE REMOVAL (the [0, 5) range the whole value
    // clears, the caret the lo seat parks, the anchor drops, the
    // rebuild the scroll top wears)
    app.creative_scroll = 48.0; // the pre-cut wheel the rebuild retires
    app.on_creative_cut();
    assert_eq!(
        app.clipboard, "stone",
        "the cut the range the store banks"
    );
    assert_eq!(
        app.creative_search, "",
        "the cut the range alone removes [the whole-value wipe the box never wears]"
    );
    assert_eq!(
        app.creative_search_cursor, 0,
        "the removal the lo seat parks"
    );
    assert_eq!(
        app.creative_search_highlight, None,
        "the removal the anchor drops"
    );
    assert_eq!(
        app.creative_scroll, 0.0,
        "the rebuild the top wears [the responder the changed value answers]"
    );
    // THE PASTE'S OWN SEAT INSERT (the empty box the store's word
    // seeds, the caret the tail rides)
    app.on_creative_paste();
    assert_eq!(app.creative_search, "stone", "the paste the word seeds");
    assert_eq!(app.creative_search_cursor, 5, "the paste the caret rides");
    // THE MID-SEAT'S OWN INSERT (the HOME the seat 0 parks, the
    // paste the head the word wears)
    app.on_creative_caret_home_end(false);
    assert_eq!(app.creative_search_cursor, 0, "the home the seat parks");
    app.on_creative_paste();
    assert_eq!(
        app.creative_search, "stonestone",
        "the paste the head wears"
    );
    assert_eq!(app.creative_search_cursor, 5, "the insert the caret rides");
    // THE SELECTION'S OWN REPLACE (the live range the first step
    // removes, the store's word the range the answer replaces)
    app.on_creative_select_all();
    app.on_creative_paste();
    assert_eq!(app.creative_search, "stone", "the paste the range replaces");
    // THE EMPTY STORE'S OWN WALK (the no-empty-guard the 26.2 bytes
    // speak — the live range the removal alone serves)
    app.clipboard = String::new();
    app.on_creative_select_all();
    app.on_creative_paste();
    assert_eq!(
        app.creative_search, "",
        "the empty store the live range alone removes"
    );
    assert_eq!(
        app.creative_search_cursor, 0,
        "the removal the lo seat parks"
    );
    assert_eq!(
        app.creative_search_highlight, None,
        "the removal the anchor drops"
    );
    // THE EMPTY COPY'S OWN UNGUARDED BANK (the collapsed pair the
    // EMPTY word the store wears)
    app.clipboard = "sentinel".to_string();
    app.on_creative_copy();
    assert_eq!(
        app.clipboard, "",
        "the collapsed copy the store empties [the unguarded write]"
    );
    // THE COLLAPSED CUT'S OWN NO-OP (the value the no-op keeps, the
    // store the empty word re-banks)
    app.on_creative_cut();
    assert_eq!(app.clipboard, "", "the collapsed cut the store empties");
    assert_eq!(
        app.creative_search, "",
        "the collapsed cut the value keeps"
    );
    assert_eq!(
        app.creative_search_cursor, 0,
        "the no-op the seat keeps"
    );
    // THE CAP'S OWN CLAMP (the 50 the search box's setMaxLength wears
    // — the flood the tail clamps, the second paste the cap
    // re-answers)
    let flood: String = "z".repeat(60);
    app.clipboard = flood;
    app.on_creative_paste();
    assert_eq!(
        app.creative_search.chars().count(),
        50,
        "the flood the tail clamps"
    );
    assert_eq!(app.creative_search_cursor, 50, "the clamp the caret parks");
    app.on_creative_paste();
    assert_eq!(
        app.creative_search.chars().count(),
        50,
        "the second paste the cap re-answers"
    );
    // THE GATE'S OWN SILENCE (the non-search tab the three walks the
    // store and box never serve)
    app.select_creative_tab(0);
    assert_eq!(
        app.creative_tab, 0,
        "the building-blocks tab the seat wears"
    );
    app.clipboard = "the gate never passes".to_string();
    app.creative_search = "untouched".to_string();
    app.creative_search_cursor = 9;
    app.on_creative_copy();
    app.on_creative_paste();
    app.on_creative_cut();
    assert_eq!(
        app.clipboard, "the gate never passes",
        "the gate the store never touches"
    );
    assert_eq!(
        app.creative_search, "untouched",
        "the gate the box never wears"
    );
}

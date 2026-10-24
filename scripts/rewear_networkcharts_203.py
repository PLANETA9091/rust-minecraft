#!/usr/bin/env python3
"""THE PART_4 NETWORK CHARTS RE-WEAR (the autopush-203 round): the
keyDebugNetworkCharts row the debug ladder's END appends — the +24
shift the scroll laws the seats re-wear, the byte-exact file laws the
row the tail appends, the END-seat ends_with law the '3' row answers,
the END-row rebind block the networkCharts's own seat retargets, the
restart decode the '3' default keeps."""
import io

P = "crates/mc-client/src/screens/tests/part_4.rs"
t = io.open(P, encoding="utf-8").read()
n_rules = 0


def rep(old, new, expect):
    global t, n_rules
    n = t.count(old)
    if n != expect:
        raise SystemExit(f"FATAL: rule expects {expect} hits, found {n}: {old[:90]!r}")
    t = t.replace(old, new)
    n_rules += 1
    print(f"rule {n_rules}: {n} hits re-worn")


# 1. THE FULL-FILE LAWS (the 10 asserts the row's tail appends —
# the 9 full-file strings + the contains() ladder)
rep(
    'key_key.debug.fpsCharts:key.keyboard.2\\n",',
    'key_key.debug.fpsCharts:key.keyboard.2\\n'
    'key_key.debug.networkCharts:key.keyboard.3\\n",',
    9,
)
rep(
    'key_key.debug.fpsCharts:key.keyboard.2\\n"\n        ),',
    'key_key.debug.fpsCharts:key.keyboard.2\\n'
    'key_key.debug.networkCharts:key.keyboard.3\\n"\n        ),',
    1,
)

# 2. THE ENDS_WITH LAWS (the END-seat laws the '3' row answers)
rep(
    '.ends_with("key_key.debug.fpsCharts:key.keyboard.2")',
    '.ends_with("key_key.debug.networkCharts:key.keyboard.3")',
    5,
)

# 3. THE MAX SCROLL'S OWN LAW (content 1196 - band 182 = 1014)
rep(
    "app.kb_scroll, 990.0,",
    "app.kb_scroll, 1014.0,",
    6,
)

# 4. THE CONTENT'S OWN HEIGHT (the row's own bottom the +24 speaks)
rep(
    '"the clamp the maxScroll law keeps (content 1172 - band 182)"',
    '"the clamp the maxScroll law keeps (content 1196 - band 182)"',
    1,
)
rep(
    "        1172,\n",
    "        1196,\n",
    4,
)
rep(
    "the content 1172\n    // the walk banks)",
    "the content 1196\n    // the walk banks)",
    1,
)
rep(
    "the content 1172 keeps",
    "the content 1196 keeps",
    4,
)
rep(
    "the max 392 speaks",
    "the max 416 speaks",
    1,
)

# 5. THE NOTCH WALKS (the +24 the count re-wears — the same seats
# the same 534/558/630/678 scrolls keep)
rep("app.on_kb_scroll(13.0, 240);", "app.on_kb_scroll(14.0, 240);", 1)
rep("app.on_kb_scroll(19.0, 240);", "app.on_kb_scroll(20.0, 240);", 1)
rep("app.on_kb_scroll(18.0, 240);", "app.on_kb_scroll(19.0, 240);", 1)
rep("app.on_kb_scroll(15.0, 240);", "app.on_kb_scroll(16.0, 240);", 1)
rep("the NINETEEN notches up the 534 scroll", "the TWENTY notches up the 534 scroll", 1)
rep(
    '"the nineteen notches up the walk wears"',
    '"the twenty notches up the walk wears"',
    1,
)
rep("THE EIGHTEEN NOTCHES' OWN RETURN", "THE NINETEEN NOTCHES' OWN RETURN", 1)
rep('"the eighteen notches the walk wears"', '"the nineteen notches the walk wears"', 1)
rep("the FIFTEEN notches up the outlines row", "the SIXTEEN notches up the outlines row", 1)
rep(
    "the THIRTEEN\n    // notches up the 678 scroll",
    "the FOURTEEN\n    // notches up the 678 scroll",
    1,
)
rep(
    "the max 990 the extra notch the walk\n    // wears",
    "the max 1014 the extra notch the walk\n    // wears",
    1,
)
rep(
    "the\n    // max 990 the extra notch the walk wears",
    "the\n    // max 1014 the extra notch the walk wears",
    1,
)

# 6. THE MAX-SCROLL FULLY-VISIBLE LAW (the trio test the END row the
# networkCharts seat answers)
rep(
    '.any(|b| b.id == "kb_change_debugfpscharts" && b.y + b.h <= 240 - 28),',
    '.any(|b| b.id == "kb_change_debugnetworkcharts" && b.y + b.h <= 240 - 28),',
    1,
)

# 7. THE FRESH-LIST CLIP LAW (the networkCharts row the fresh band
# refuses — the fpsCharts assert the AFTER wears)
rep(
    '        "the clipped fpsCharts row the band render refuses"\n    );',
    '        "the clipped fpsCharts row the band render refuses"\n    );\n'
    "    assert!(\n"
    "        !app.buttons\n"
    "            .iter()\n"
    '            .any(|b| b.id == "kb_change_debugnetworkcharts"),\n'
    '        "the clipped networkCharts row the band render refuses"\n'
    "    );",
    1,
)

# 8. THE MAX-SCROLL CLIP LAWS (the +24 the FIRST the wall speaks —
# the dumpDynamicTextures row the list-top clip refuses, the
# reloadResourcePacks row the first rendered seat wears)
rep(
    """    let dt_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugdumptextures")
        .expect("the scrolled dumpDynamicTextures row the band restores")
        .clone();
    assert_eq!(
        dt_row.label, "S",
        "the dumpDynamicTextures's own default the display speaks"
    );
    assert_eq!(
        dt_row.y, 48,
        "the dumpDynamicTextures row the END-6 seat wears"
    );
    assert!(
        dt_row.y + dt_row.h <= 240 - 28,
        "the last row the max scroll the fully-visible seat wears"
    );
""",
    """    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugdumptextures"),
        "the dumpDynamicTextures row the max scroll the list-top clip refuses"
    );
""",
    1,
)

# 9. THE END-SEAT CHAIN (the +24 the seats re-wear — the rr the
# first rendered the 48 seat the END-6 wears, the chain the one-row
# shift speaks, the networkCharts row the END the 192 seat banks)
rep(
    'assert_eq!(\n        rr_row.y, 72,\n        "the reloadResourcePacks row the END-5 seat wears"\n    );',
    'assert_eq!(\n        rr_row.y, 48,\n        "the reloadResourcePacks row the END-6 seat wears"\n    );',
    1,
)
rep(
    'assert_eq!(pf_row.y, 96, "the profiling row the END-4 seat wears");',
    'assert_eq!(pf_row.y, 72, "the profiling row the END-5 seat wears");',
    1,
)
rep(
    'assert_eq!(cl_row.y, 120, "the copyLocation row the END-3 seat wears");',
    'assert_eq!(cl_row.y, 96, "the copyLocation row the END-4 seat wears");',
    1,
)
rep(
    'assert_eq!(dv_row.y, 144, "the dumpVersion row the END-2 seat wears");',
    'assert_eq!(dv_row.y, 120, "the dumpVersion row the END-3 seat wears");',
    1,
)
rep(
    'assert_eq!(pc_row.y, 168, "the profilingChart row the END-1 seat wears");',
    'assert_eq!(pc_row.y, 144, "the profilingChart row the END-2 seat wears");',
    1,
)
rep(
    'assert_eq!(fc_row.y, 192, "the fpsCharts row the END seat wears");',
    'assert_eq!(fc_row.y, 168, "the fpsCharts row the END-1 seat wears");',
    1,
)

# the networkCharts row the END seat banks (the fc_row block the
# AFTER the new row's own walk wears)
rep(
    """    assert_eq!(fc_row.y, 168, "the fpsCharts row the END-1 seat wears");
    assert!(
        fc_row.y + fc_row.h <= 240 - 28,
        "the last row the max scroll the fully-visible seat wears"
    );
    app.on_kb_scroll(14.0, 240);""",
    """    assert_eq!(fc_row.y, 168, "the fpsCharts row the END-1 seat wears");
    assert!(
        fc_row.y + fc_row.h <= 240 - 28,
        "the last row the max scroll the fully-visible seat wears"
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
    assert_eq!(nc_row.y, 192, "the networkCharts row the END seat wears");
    assert!(
        nc_row.y + nc_row.h <= 240 - 28,
        "the last row the max scroll the fully-visible seat wears"
    );
    app.on_kb_scroll(14.0, 240);""",
    1,
)

# 10. THE END-ROW REBIND BLOCK (the networkCharts's own seat the
# retarget wears — the fpsCharts block the same lanes the row
# answers)
rep(
    """    // THE FPS CHARTS'S OWN REBIND WALK (the twenty-row
    // walk's own row the SAME lanes the capture honors — the ctor's
    // pc 3660 the `ldc_w cp#1511` the `key.debug.fpsCharts`
    // answers, the pc 3666 the `bipush 50` the '2' the FIVE-arg
    // default wears, the `iconst_2` the orderI the sort walk banks,
    // the notch down the max the row's own 192 seat the band
    // restores, the change the click the seat arms, the capture the
    // bind wears, the reset the '2' the plain-shape default
    // restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let fcrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugfpscharts")
        .expect("the fpsCharts row the band keeps")
        .clone();
    assert_eq!(fcrow.y, 192, "the fpsCharts row the END seat wears");
    app.on_click(fcrow.x + fcrow.w / 2, fcrow.y + fcrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_FPS_CHARTS),
        "the change the fpsCharts's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::U)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_FPS_CHARTS],
        Some(kb::Bind::Keyboard(minifb::Key::U)),
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
    );""",
    """    // THE NETWORK CHARTS'S OWN REBIND WALK (the twenty-one-row
    // walk's own row the SAME lanes the capture honors — the ctor's
    // pc 3683 the `ldc_w cp#1516` the `key.debug.networkCharts`
    // answers, the pc 3689 the `bipush 51` the '3' the FIVE-arg
    // default wears, the `iconst_3` the orderI the sort walk banks,
    // the notch down the max the row's own 192 seat the band
    // restores, the change the click the seat arms, the capture the
    // bind wears, the reset the '3' the plain-shape default
    // restores)
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let ncrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugnetworkcharts")
        .expect("the networkCharts row the band keeps")
        .clone();
    assert_eq!(ncrow.y, 192, "the networkCharts row the END seat wears");
    app.on_click(ncrow.x + ncrow.w / 2, ncrow.y + ncrow.h / 2);
    assert_eq!(
        app.kb_selected,
        Some(kb::DEBUG_NETWORK_CHARTS),
        "the change the networkCharts's seat arms"
    );
    assert!(
        app.kb_capture(Some(minifb::Key::U)),
        "the capture the bind consumes"
    );
    assert_eq!(
        app.key_binds[kb::DEBUG_NETWORK_CHARTS],
        Some(kb::Bind::Keyboard(minifb::Key::U)),
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
    );""",
    1,
)

# 11. THE FILE'S OWN ROWS COMMENT (the twenty-one-row walk the
# networkCharts row the END answers)
rep(
    """    // THE FILE'S OWN ROWS (the family save the exit banks — the
    // twenty-row walk the spectatorHotbar row's AFTER the ladder
    // appends, the fpsCharts row the file's own END the
    // last row wears)""",
    """    // THE FILE'S OWN ROWS (the family save the exit banks — the
    // twenty-one-row walk the spectatorHotbar row's AFTER the ladder
    // appends, the networkCharts row the file's own END the
    // last row wears)""",
    1,
)
rep(
    '"the debug twenty-row walk the ladder\'s own tail the file wears"',
    '"the debug twenty-one-row walk the ladder\'s own tail the file wears"',
    1,
)
rep(
    "the debug twenty-row's own reach the nine's rows the walk",
    "the debug twenty-one-row's own reach the nine's rows the walk",
    1,
)

# 12. THE COMMENT SEATS (the 1182 the 1206 the list-space y answers —
# the networkCharts row the END seat the comments wear)
rep(
    "fpsCharts row the 1182 y the 192 seat",
    "networkCharts row the 1206 y the 192 seat",
    2,
)

# 13. THE RESTART'S OWN DECODE (the networkCharts the '3' default
# the fresh boot keeps — the fpsCharts assert the AFTER wears)
rep(
    """    assert_eq!(
        re.key_binds[kb::DEBUG_FPS_CHARTS],
        Some(kb::Bind::Keyboard(minifb::Key::Key2)),
        "the restart the fpsCharts's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::SPECTATOR_HOTBAR],""",
    """    assert_eq!(
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
        re.key_binds[kb::SPECTATOR_HOTBAR],""",
    1,
)

# 14. THE OVERLAY WALK'S COMMENT (the -264 seat the list-top clip
# refuses — the same negative walk the row's own shift keeps)
rep(
    "the overlay\n    // row the -264 seat the list-top clip refuses",
    "the overlay\n    // row the -288 seat the list-top clip refuses",
    1,
)

io.open(P, "w", encoding="utf-8").write(t)
print(f"done: {n_rules} rules worn")

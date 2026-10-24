#!/usr/bin/env python3
"""THE PART_4 + OPTIONS_FILE LIGHTMAP TEXTURE RE-WEAR (the autopush-204
round): the keyDebugLightmapTexture row the debug ladder's END appends —
the +24 shift the scroll laws the seats re-wear, the byte-exact file
laws the row the tail appends, the END-seat ends_with law the '4' row
answers, the END-row rebind block the lightmapTexture's own seat
retargets, the restart decode the '4' default keeps."""
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


# 1. THE FULL-FILE LAWS (the 9 asserts the row's tail appends —
# the 9 full-file strings)
rep(
    'key_key.debug.networkCharts:key.keyboard.3\\n",',
    'key_key.debug.networkCharts:key.keyboard.3\\n'
    'key_key.debug.lightmapTexture:key.keyboard.4\\n",',
    9,
)

# 2. THE CONTAINS-LADDER LAW (the 1 assert the row's tail appends)
rep(
    'key_key.debug.networkCharts:key.keyboard.3\\n"\n        ),',
    'key_key.debug.networkCharts:key.keyboard.3\\n'
    'key_key.debug.lightmapTexture:key.keyboard.4\\n"\n        ),',
    1,
)

# 3. THE ENDS_WITH LAWS (the END-seat laws the '4' row answers)
rep(
    '.ends_with("key_key.debug.networkCharts:key.keyboard.3")',
    '.ends_with("key_key.debug.lightmapTexture:key.keyboard.4")',
    5,
)

# 4. THE MAX SCROLL'S OWN LAW (content 1220 - band 182 = 1038)
rep(
    "app.kb_scroll, 1014.0,",
    "app.kb_scroll, 1038.0,",
    6,
)

# 5. THE CONTENT'S OWN HEIGHT (the row's own bottom the +24 speaks)
rep(
    '"the clamp the maxScroll law keeps (content 1196 - band 182)"',
    '"the clamp the maxScroll law keeps (content 1220 - band 182)"',
    1,
)
rep(
    "        1196,\n",
    "        1220,\n",
    4,
)
rep(
    "the content 1196 keeps",
    "the content 1220 keeps",
    4,
)
rep(
    "the fpsCharts row the END the content 1196\n    // the walk banks)",
    "the lightmapTexture row the END the content 1220\n    // the walk banks)",
    1,
)
rep(
    "the 1172 content the 780-tall band the max 416 speaks",
    "the 1220 content the 780-tall band the max 440 speaks",
    1,
)
rep(
    "the max the 990 clamp keeps",
    "the max the 1038 clamp keeps",
    4,
)

# 6. THE MAX-SCROLL FULLY-VISIBLE LAW (the trio test the END row the
# lightmapTexture seat answers)
rep(
    '.any(|b| b.id == "kb_change_debugnetworkcharts" && b.y + b.h <= 240 - 28),',
    '.any(|b| b.id == "kb_change_debuglightmaptexture" && b.y + b.h <= 240 - 28),',
    1,
)

# 7. THE NOTCH WALKS (the +24 the count re-wears — the same seats
# the same 534/558/630/678 scrolls keep)
rep("app.on_kb_scroll(20.0, 240);", "app.on_kb_scroll(21.0, 240);", 1)
rep("app.on_kb_scroll(19.0, 240);", "app.on_kb_scroll(20.0, 240);", 1)
rep("app.on_kb_scroll(16.0, 240);", "app.on_kb_scroll(17.0, 240);", 1)
rep("app.on_kb_scroll(14.0, 240);", "app.on_kb_scroll(15.0, 240);", 1)
rep("the TWENTY notches up the 534 scroll", "the TWENTY-ONE notches up the 534 scroll", 1)
rep(
    '"the twenty notches up the walk wears"',
    '"the twenty-one notches up the walk wears"',
    1,
)
rep("THE NINETEEN NOTCHES' OWN RETURN", "THE TWENTY NOTCHES' OWN RETURN", 1)
rep('"the nineteen notches the walk wears"', '"the twenty notches the walk wears"', 1)
rep("the SIXTEEN notches up the outlines row", "the SEVENTEEN notches up the outlines row", 1)
rep(
    "the max 1014 the extra notch the walk\n    // wears",
    "the max 1038 the extra notch the walk\n    // wears",
    1,
)
rep(
    "the\n    // max 1014 the extra notch the walk wears",
    "the\n    // max 1038 the extra notch the walk wears",
    1,
)
rep(
    "the overlay\n    // row the -288 seat the list-top clip refuses",
    "the overlay\n    // row the -312 seat the list-top clip refuses",
    1,
)
rep(
    "the FOURTEEN\n    // notches up the 678 scroll the overlay row the 48 seat",
    "the FIFTEEN\n    // notches up the 678 scroll the overlay row the 48 seat",
    1,
)

# 8. THE SPECTATOR TEST'S OWN CHAIN COMMENT (the +24 the seats
# re-wear — the dumpDynamicTextures/reloadResourcePacks the max
# scroll the list-top clip refuses, the profiling the END-6 the
# first the wall answers, the lightmapTexture the END the row's
# own bottom the content answers)
rep(
    """    // the dumpDynamicTextures row the 1038 y the 48 seat the
    // END-6 the walk keeps, the reloadResourcePacks row the 1062
    // y the 72 seat the END-5 the walk keeps, the profiling row
    // the 1086 y the 96 seat the END-4 the walk keeps, the
    // copyLocation row the 1110 y the 120 seat the END-3 the walk
    // keeps, the dumpVersion row the 1134 y the 144 seat the
    // END-2 wears, the profilingChart row the 1158 y the 168 seat
    // the END-1 wears, the networkCharts row the 1206 y the 192 seat
    // the fully-visible law keeps — the row's own bottom the
    // band's own bottom the walk aligns)""",
    """    // the dumpDynamicTextures row the 1038 y the 0 seat the
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
    // band's own bottom the walk aligns)""",
    1,
)

# 9. THE OVERLAY TEST'S OWN LAST-ROW COMMENT (the lightmapTexture
# row the END the 1230 y the 192 seat the LAST row wears)
rep(
    "networkCharts row the 1206 y the 192 seat the LAST row",
    "lightmapTexture row the 1230 y the 192 seat the LAST row",
    1,
)

# 10. THE FRESH-LIST CLIP LAW (the lightmapTexture row the fresh
# band refuses — the networkCharts assert the AFTER wears)
rep(
    '        "the clipped networkCharts row the band render refuses"\n    );',
    '        "the clipped networkCharts row the band render refuses"\n    );\n'
    "    assert!(\n"
    "        !app.buttons\n"
    "            .iter()\n"
    '            .any(|b| b.id == "kb_change_debuglightmaptexture"),\n'
    '        "the clipped lightmapTexture row the band render refuses"\n'
    "    );",
    1,
)

# 11. THE MAX-SCROLL CLIP LAWS (the +24 the FIRST the wall speaks —
# the reloadResourcePacks row the list-top clip refuses, the
# profiling row the first rendered seat wears)
rep(
    """    let rr_row = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debugreloadpacks")
        .expect("the scrolled reloadResourcePacks row the band restores")
        .clone();
    assert_eq!(
        rr_row.label, "T",
        "the reloadResourcePacks's own default the display speaks"
    );
    assert_eq!(
        rr_row.y, 48,
        "the reloadResourcePacks row the END-6 seat wears"
    );
    assert!(
        rr_row.y + rr_row.h <= 240 - 28,
        "the last row the max scroll the fully-visible seat wears"
    );
""",
    """    assert!(
        !app.buttons
            .iter()
            .any(|b| b.id == "kb_change_debugreloadpacks"),
        "the reloadResourcePacks row the max scroll the list-top clip refuses"
    );
""",
    1,
)

# 12. THE END-SEAT CHAIN (the +24 the seats re-wear — the pf the
# first rendered the 48 seat the END-6 wears, the chain the one-row
# shift speaks, the lightmapTexture row the END the 192 seat banks)
rep(
    'assert_eq!(pf_row.y, 72, "the profiling row the END-5 seat wears");',
    'assert_eq!(pf_row.y, 48, "the profiling row the END-6 seat wears");',
    1,
)
rep(
    'assert_eq!(cl_row.y, 96, "the copyLocation row the END-4 seat wears");',
    'assert_eq!(cl_row.y, 72, "the copyLocation row the END-5 seat wears");',
    1,
)
rep(
    'assert_eq!(dv_row.y, 120, "the dumpVersion row the END-3 seat wears");',
    'assert_eq!(dv_row.y, 96, "the dumpVersion row the END-4 seat wears");',
    1,
)
rep(
    'assert_eq!(pc_row.y, 144, "the profilingChart row the END-2 seat wears");',
    'assert_eq!(pc_row.y, 120, "the profilingChart row the END-3 seat wears");',
    1,
)
rep(
    'assert_eq!(fc_row.y, 168, "the fpsCharts row the END-1 seat wears");',
    'assert_eq!(fc_row.y, 144, "the fpsCharts row the END-2 seat wears");',
    1,
)
rep(
    """    assert_eq!(nc_row.y, 192, "the networkCharts row the END seat wears");
    assert!(
        nc_row.y + nc_row.h <= 240 - 28,
        "the last row the max scroll the fully-visible seat wears"
    );""",
    """    assert_eq!(nc_row.y, 168, "the networkCharts row the END-1 seat wears");
    assert!(
        nc_row.y + nc_row.h <= 240 - 28,
        "the last row the max scroll the fully-visible seat wears"
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
        "the lightmapTexture row the END seat wears"
    );
    assert!(
        lt_row.y + lt_row.h <= 240 - 28,
        "the last row the max scroll the fully-visible seat wears"
    );""",
    1,
)

# 13. THE END-ROW REBIND BLOCK (the lightmapTexture's own seat the
# retarget wears — the networkCharts block the same lanes the row
# answers, the THREE notches down the max the row's own 192 seat
# the band restores)
rep(
    """    // THE NETWORK CHARTS'S OWN REBIND WALK (the twenty-one-row
    // walk's own row the SAME lanes the capture honors — the ctor's
    // pc 3683 the `ldc_w cp#1516` the `key.debug.networkCharts`
    // answers, the pc 3689 the `bipush 51` the '3' the FIVE-arg
    // default wears, the `iconst_3` the orderI the sort walk banks,
    // the TWO notches down the max the row's own 192 seat the band
    // restores, the change the click the seat arms, the capture the
    // bind wears, the reset the '3' the plain-shape default
    // restores)
    app.on_kb_scroll(-1.0, 240);
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
    );
    app.on_kb_scroll(1.0, 240);
    app.on_kb_scroll(1.0, 240);""",
    """    // THE LIGHTMAP TEXTURE'S OWN REBIND WALK (the twenty-two-row
    // walk's own row the SAME lanes the capture honors — the ctor's
    // pc 3706 the `ldc_w cp#1521` the `key.debug.lightmapTexture`
    // answers, the pc 3712 the `bipush 52` the '4' the FIVE-arg
    // default wears, the `iconst_4` the orderI the sort walk banks,
    // the THREE notches down the max the row's own 192 seat the band
    // restores, the change the click the seat arms, the capture the
    // bind wears, the reset the '4' the plain-shape default
    // restores)
    app.on_kb_scroll(-1.0, 240);
    app.on_kb_scroll(-1.0, 240);
    app.on_kb_scroll(-1.0, 240);
    app.render(&mut fb, &mut assets);
    let ltrow = app
        .buttons
        .iter()
        .find(|b| b.id == "kb_change_debuglightmaptexture")
        .expect("the lightmapTexture row the band keeps")
        .clone();
    assert_eq!(
        ltrow.y, 192,
        "the lightmapTexture row the END seat wears"
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
    app.on_kb_scroll(1.0, 240);""",
    1,
)

# 14. THE FILE'S OWN ROWS COMMENT (the twenty-two-row walk the
# lightmapTexture row the END answers)
rep(
    "the\n    // twenty-one-row walk the spectatorHotbar row's AFTER",
    "the\n    // twenty-two-row walk the spectatorHotbar row's AFTER",
    1,
)
rep(
    '"the debug twenty-one-row walk the ladder\'s own tail the file wears"',
    '"the debug twenty-two-row walk the ladder\'s own tail the file wears"',
    1,
)
rep(
    "the debug twenty-one-row's own reach the nine's rows the walk",
    "the debug twenty-two-row's own reach the nine's rows the walk",
    1,
)

# 15. THE ENDS_WITH MESSAGES (the END-row retarget the words wear —
# the four named-file tests; the coverage's own key-block message the
# row-agnostic wording keeps)
rep(
    '"the fpsCharts row the file\'s own END wears"',
    '"the lightmapTexture row the file\'s own END wears"',
    4,
)

# 16. THE RESTART'S OWN DECODE (the lightmapTexture the '4' default
# the fresh boot keeps — the networkCharts assert the AFTER wears)
rep(
    """    assert_eq!(
        re.key_binds[kb::DEBUG_NETWORK_CHARTS],
        Some(kb::Bind::Keyboard(minifb::Key::Key3)),
        "the restart the networkCharts's default the decode keeps"
    );
    assert_eq!(
        re.key_binds[kb::SPECTATOR_HOTBAR],""",
    """    assert_eq!(
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
        re.key_binds[kb::SPECTATOR_HOTBAR],""",
    1,
)

io.open(P, "w", encoding="utf-8").write(t)
print(f"part_4 done: {n_rules} rules worn")

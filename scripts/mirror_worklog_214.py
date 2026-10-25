import os

ENTRY = """---
Task ID: autopush-214
Agent: GLM (Z.ai)
Task: Job 384923 — rust-maintenance cadence tick ([System 2026-10-25 13:48:13 +08:00], trace 1a0a02c3c1060631-cron-agent-loop-202610251355): git status check, ONE minimal step, cargo test --workspace green + push, worklog entry.

Work Log:
- git status re-check: HEAD 3ba02c4 (autopush-213 mirror, v0.3.964), clean tree, origin synced — the worklog's last autopush entry 213, this round 214 (no dev round landed in between, the dev cadence the next tick the 13:58 slot wears).
- THE ROUND'S OWN PICK: the keyDebugDumpVersion FUNCTIONAL lane (the autopush-213 RESUME PLAN's first candidate, the F3 family's twelfth implemented lane) — the F3+V chord the version-dump walk serves.
- PROBE: scripts/jar_work/dumpver_214.py (chained off controls_185.py helpers) -> the lane body pc 1163-1199 (the keyDebugDumpVersion `matches` gate, the HEADER FIRST the cp[606] `debug.version.header` row the debugFeedbackTranslated walk sends, the ROWS the `VersionCommand.dumpVersion(consumer)` static walk the indy's `debugFeedbackComponent` law binds — the YELLOW decorate the showDebugChat channel wears, the USED FLAG pc 1198-1199 the istore banks) + the dumpVersion body pc 0-264 (the NINE translatables: commands.version.id/.name/.data/.series/.protocol/.build_time/.pack.resource/.pack.data + the stable row the STABLE/UNSTABLE constants keep) + the version.json values (id/name = 26.2, world_version = 4903, series_id = main, protocol_version = 776, build_time = 2026-06-16T12:01:27+00:00, pack_version resource 88.0 / data 107.1, stable = true) + the Component.translationArg(Date) walk (`Date.toString()` the literal keeps — 'Tue Jun 16 12:01:27 UTC 2026' the format banks) + the PackFormat.toString walk (the minor==MAX_VALUE '%d.*' arm the '%d.%d' arm the 88.0/107.1 pair answers) + the protocol row's TWO args (`protocol = %s (%s)` — the id the pack_claim bank wears, the toHexString(776) the 308 hex answers) + the en_us rows (debug.version.header = 'Client version info:', debug.version.help = 'F3 + V = Client version info', the key.debug.dumpVersion bind the lang wears).
- IMPL: screens.rs on_dump_version_key (the header the §e word first sends, the nine rows the loop the yellow channel speaks — the id/name the client's MINECRAFT_VERSION build bank answers, the protocol the pack_claim PROTOCOL_VERSION wire bank + the {:x} hex fold wears, the data/series/build_time/pack pair the version.json's own values keep, the stable row the STABLE word closes, the flag the last statement banks); window.rs the F3+V dispatch arm (the f3_down + is_key_pressed(V, No) rising edge, the C-arm the AFTER seat the pc order 1004 < 1164 < 1201+ keeps).
- TEST: part_4.rs the_dump_version_key_the_header_the_nine_rows_the_flag_wear (the tail row the stable constant keeps, the flag the release consumes, the FULL QUEUE the header the first row wears + the nine translatables the exact strings bank — the §e yellow the family's channel speaks, the wire bank's identity the pack_claim equality banks) — part_4 174 passed (173 +1), the first-run verdict the cached-binary filter never needed.
- THE WALK: the twelve-window foreground walk rc=0 (part_1..11 + rest, one window per call the dev-166 answer wears), fast crates 337 (157 + 100 + 80), goldens 19/19 rms 0.000 NO regeneration (the chat rows the headless scenes never see — the fixed scenes the chord never wears).
- FMT: the touched files the rustfmt --check the ZERO new diffs wear (the part_4 10419 clip_crosshair block the HEAD's own drift the stash-verify keeps, the rustfmt shape the rows-collect one-line the diff's own answer adopts, the fmt-all the round NEVER runs [the autopush-205 100+-file drift the lesson]).
- Version 0.3.964 -> 0.3.965 (bump_version.py the six crates + lock wear).
- Commit 8aae94b (scripts/commit_msg_965.txt) pushed origin main: 3ba02c4..8aae94b.

Stage Summary:
- v0.3.965: the F3+V dump-version chord the FUNCTIONAL lane wears — the vanilla `Client version info:` header + the nine `id/name/data/series/protocol/build_time/pack_resource/pack_data/stable` rows the yellow chat speaks (the WorldVersion's own json values the client's banks answer, the Date.toString + the PackFormat '%d.%d' + the toHexString folds the exact strings keep), the client the real Minecraft's F3+V behavior the family's own shape speaks.
- The mc-client bank the part_4 window 174 (173 + the dump-version walk), the fast crates the 337 counts, the goldens the 19/19 rms 0.000.
- NEXT CANDIDATES: the key.hotbar.9's own reset arm, the orderI=0 rows' own alphabetical re-seat (the compareTo law the i18n-name secondary the groups re-order — the DebugOptionsScreen$OptionList's own lambda$static$0 comparator the same law wears), the 7-toggle rows the Controls array wears (toggleCrouch/toggleSprint the ToggleKeyMapping law), the MouseSettings' other 5 rows (mouseWheelSensitivity the extra walk), the DEBUG OPTIONS' OWN ENTRIES ROUND (the full registry rows the updateSearch walk keeps — the 40 entry ids the DebugScreenEntries clinit banks, the tri-state store the Always/In Overlay/Off arms speak, the search-box responder the filter serves, the save/load file the DebugScreenEntryList walk wears), the F3+V's own SERVER-ARM WALK (the `/version` command the register walk serves — the Brigadier tree the command's own row keeps).
"""

for path in [
    "/home/z/my-project/rust-minecraft/logs/worklog.md",
    "/home/z/my-project/worklog.md",
]:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "a", encoding="utf-8") as f:
        f.write(ENTRY)
    print(f"appended -> {path}")

# verify
for path in [
    "/home/z/my-project/rust-minecraft/logs/worklog.md",
    "/home/z/my-project/worklog.md",
]:
    data = open(path, encoding="utf-8").read()
    i = data.rindex("---\nTask ID: autopush-214")
    print(f"verified {path} rindex={i} len={len(data)}")

import os

ENTRY = """---
Task ID: autopush-215
Agent: GLM (Z.ai)
Task: Job 384923 — rust-maintenance cadence tick ([System 2026-10-25 14:18:14 +08:00], trace 1a0a02c3c1060631-cron-agent-loop-202610251420): git status check, ONE minimal step, cargo test --workspace green + push, worklog entry.

Work Log:
- git status re-check: HEAD fd30d08 (autopush-214 mirror, v0.3.965), clean tree, origin synced — the worklog's last autopush entry 214, this round 215 (no dev round landed in between; the compressed-context summary's own snapshot cf27bc2/v0.3.957 the TWO-HOUR drift the re-check answered — seven rounds the cadence landed between).
- THE ROUND'S OWN PICK: the /version SERVER-ARM walk (the autopush-214 RESUME PLAN's last candidate, the F3+V's own sibling lane) — the offline dispatch the VersionCommand.register walk serves.
- PROBE: scripts/jar_work/vercmd_215.py (chained off controls_185.py helpers, the cp_class_names walk the None-hole guard wears, the Commands clinit the try/except per-method walk keeps) -> the VersionCommand class (net/minecraft/server/commands/VersionCommand — the SERVER package the command's own seat keeps) + the register body (the `literal("version")` node, the requires walk the boolean flag arms — the `flag ? LEVEL_GAMEMASTERS : LEVEL_ALL` pair the iload_1 law answers) + the executes lambda (lambda$register$0 — the sendSystemMessage walk the HEADER constant sends FIRST, the `dumpVersion(source::sendSystemMessage)` walk the indy binds, the `return 1` verdict the extra feedback never adds) + the CALL-SITE window (Commands.<init>(CommandSelection, CommandBuildContext) — the `selection != INTEGRATED` fold the iconst_0/iconst_1 pair wears: the INTEGRATED selection the `false` flag banks = LEVEL_ALL, the dedicated/ALL the `true` re-arms = LEVEL_GAMEMASTERS) + the en_us rows (commands.version.header = 'Server version info:' — the SERVER header the F3+V's `Client version info:` the DISTINCT sibling wears; the nine translatables the SAME keys/values the client lane banks; command.unknown.command = 'Unknown or incomplete command. See below for error' the red verdict's own row).
- IMPL: screens.rs run_local_command the `["version"]` arm (the header the tr walk the plain channel sends FIRST, the nine rows the format_positional walk the en_us table answers — the id/name the build_info MINECRAFT_VERSION bank wears, the protocol the pack_claim PROTOCOL_VERSION + the {:x} hex fold keeps, the data/series/build_time/pack pair the version.json's own values bank, the stable row the STABLE word closes; the PLAIN CHANNEL the §e-yellow client debug decorate NEVER wears — the sendSystemMessage law the F3+V chord the yellow prefix alone speaks; NO debug_modifier_used bank — the command the debug store never touches; the EXACT literal the bare word alone the ten-row walk earns — the `version extra` trailing word the match's unknown-command arm the parse failure answers).
- TEST: part_4.rs the pair (the_version_command_the_server_header_the_plain_rows_wear — the ten rows the plain channel banks, the header the first wears, the flag the chord never banks; the_version_command_the_arg_word_the_parse_failure_wears — the single error row the §c-red verdict wears) — part_4 176 passed (174 +2), the first-run verdict the fix never needed.
- THE WALK: the twelve-window foreground walk rc=0 (part_1..11 + rest, one window per call the dev-166 answer wears), fast crates 337 (157 + 100 + 80), goldens 19/19 rms 0.000 NO regeneration (the chat rows the headless scenes never see — the fixed scenes the command never wears).
- FMT: the touched files the rustfmt --check the ZERO new diffs wear (the part_4 10419 clip_crosshair block the HEAD's own drift the stash-verify keeps, the rustfmt shape the data-row wrap the diff's own answer adopts, the fmt-all the round NEVER runs [the autopush-205 100+-file drift the lesson]).
- Version 0.3.965 -> 0.3.966 (bump_version.py the six crates + lock wear).
- Commit b7cfa50 (scripts/commit_msg_966.txt) pushed origin main: fd30d08..b7cfa50.

Stage Summary:
- v0.3.966: the /version command the SERVER-ARM lane wears — the offline dispatch the VersionCommand.register walk serves (the INTEGRATED selection the LEVEL_ALL no-gate law keeps, the header the `Server version info:` word the plain channel sends, the nine rows the dumpVersion walk the en_us table answers, the exact literal the parse-failure verdict keeps), the client the real Minecraft's /version behavior the server's own shape speaks.
- The mc-client bank the part_4 window 176 (174 + the version-command pair), the fast crates the 337 counts, the goldens the 19/19 rms 0.000.
- NEXT CANDIDATES: the key.hotbar.9's own reset arm, the orderI=0 rows' own alphabetical re-seat (the compareTo law the i18n-name secondary the groups re-order — the DebugOptionsScreen$OptionList's own lambda$static$0 comparator the same law wears), the 7-toggle rows the Controls array wears (toggleCrouch/toggleSprint the ToggleKeyMapping law), the MouseSettings' other 5 rows (mouseWheelSensitivity the extra walk), the DEBUG OPTIONS' OWN ENTRIES ROUND (the full registry rows the updateSearch walk keeps — the 40 entry ids the DebugScreenEntries clinit banks, the tri-state store the Always/In Overlay/Off arms speak, the search-box responder the filter serves, the save/load file the DebugScreenEntryList walk wears), the ADVANCED TOOLTIPS' OWN RENDER (the item tooltip the flag gates — the container hover the ID/durability rows the tooltip walk banks).
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
    i = data.rindex("---\nTask ID: autopush-215")
    print(f"verified {path} rindex={i} len={len(data)}")

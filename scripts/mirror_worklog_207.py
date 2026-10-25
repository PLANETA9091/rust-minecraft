import os

ENTRY = """---
Task ID: autopush-207
Agent: GLM (Z.ai)
Task: Job 384923 — rust-maintenance cadence tick ([System 2026-10-25 09:33:10 +08:00], trace 1a0a02c3c1060631-cron-agent-loop-202610250934): git status check, ONE minimal step, cargo test --workspace green + push, worklog entry.

Work Log:
- git status re-check: HEAD e8849f3 (dev-170 mirror), clean tree, origin synced — the live log the compression lag the re-check corrects (dev-169 v0.3.955 F3+I copy-recreate + dev-170 v0.3.956 F3+A reload chunks already banked, the worklog's last autopush entry 206, this round 207).
- THE ROUND'S OWN PICK: the keyDebugCopyLocation FUNCTIONAL lane (the RESUME PLAN's first candidate) — the F3+C chord the /tp copy walk serves.
- PROBE: scripts/jar_work/copyloc_207.py (chained off controls_185.py helpers, the script-dir load the cwd coupling fixes) -> the lane-map scan the FULL keyDebug family answers (101 reloadChunk < 130 hitboxes < 199 clearChat < 229 borders < 298 tooltips < 351 copyRecreate < 420 spectate < 557 switchGameMode < 651 debugOptions < 749 focusPause < 802 dumpDynamicTextures < 905 reloadResourcePacks < 933/988 profiling < 1004 copyLocation < 1164 dumpVersion < 1201+ the charts the silence keeps) + the lane body pc 1004-1162 (the ldcw@540/@550 the CONSTANT_String deref the cp[8] idx->utf8 chain reads — `debug.copy_location.message` the lang key, `/execute in %s run tp @s %.2f %.2f %.2f %.2f %.2f` the Locale.ROOT format the SIX args keep: the dim identifier first, then the feet XYZ + yRot/xRot the %.2f five) + the en_us rows (`Copied location to clipboard`, the `debug.copy_location.help` the 10-second crash hold the same lane's own law keeps — the crash walk the LATER candidate wears).
- IMPL: screens.rs on_copy_location_key (the player-null arm the body's Option keeps — the gate the flag never banks, the isReducedDebugInfo the client's belief never arms [the dev-169 constant-pass], the WORD FIRST the pc 1037 order the walk mirrors, the three-way Dimension the identifier match speaks — minecraft:overworld/the_nether/the_end the ResourceKey words —, the feet + angles the format the body's fields serve, the isEmpty gate the clipboard store wears, the flag the last statement banks); window.rs the F3+C dispatch arm (the f3_down + is_key_pressed(C, No) rising edge, the F3+F4 block the AFTER seat the ladder's tail keeps — the pc order 749 < 1004 the highest implemented lane speaks, the in_world gate the family's own law).
- TEST: part_4.rs the pair (the overworld format — the dyadic feet 12.25/70.0/-8.25/180.5/45.25 the %.2f never rounds, the exact command string the clipboard banks, the §eCopied-location word the queue keeps, the flag the release consumes; the gate arm — the body-None the previous copy keeps, the silence the count never grows, the flag the gated walk never banks, the nether swap the minecraft:the_nether identifier the same body speaks) — part_4 160 passed (158 +2), the two new tests the cached-binary filter the 2-pass verdict wears.
- THE WALK: the twelve-window foreground walk rc=0 (part_1..11 + rest, one window per call the dev-166 answer wears), fast crates 337 (157 + 100 + 80), goldens 19/19 rms 0.000 NO regeneration (the chat word + the clipboard string the goldens never open — the fixed scenes the chord never wears).
- FMT: the touched files the rustfmt --check the ZERO new diffs wear (the part_4 10419 clip_crosshair block the HEAD's own drift keeps — the dev-170 note the same block answers; the additions the rustfmt shapes adopted — the assert_eq one-line pairs + the split assert the fmt's own verdict banks, the fmt-all the round NEVER runs [the autopush-205 100+-file drift the lesson]).
- Version 0.3.956 -> 0.3.957 (bump_version.py the six crates + lock wear).
- Commit d25d7c6 (scripts/commit_msg_957.txt) pushed origin main: e8849f3..d25d7c6.

Stage Summary:
- v0.3.957: the F3+C copy-location chord the FUNCTIONAL lane wears — the vanilla `/execute in <dim> run tp @s <x> <y> <z> <yRot> <xRot>` walk the clipboard serves (the word the format PRECEDES, the gates the flag never banks, the yellow the family's channel keeps), the client the real Minecraft's F3+C behavior the family's own shape speaks.
- The mc-client bank the part_4 window 160 (156 + the reload pair [dev-170] + the copy-location pair), the fast crates the 337 counts, the goldens the 19/19 rms 0.000.
- NEXT CANDIDATES: the keyDebugReloadResourcePacks lane (the F3+T the reload walk the debug.reload_resourcepacks.message word wears — the `Reloaded resource packs` row the lang keeps), the keyDebugCopyLocation's own CRASH-HOLD walk (the F3+C the 10-second hold the debug crash the help row's own law keeps), the ADVANCED TOOLTIPS' OWN RENDER (the item tooltip the flag gates — the container hover the ID/durability rows the tooltip walk banks), the keyDebugSpectate lane (the pc 420 the cycle-spectator walk), the key.hotbar.9's own reset arm, the orderI=0 rows' own alphabetical re-seat (the compareTo law the i18n-name secondary the groups re-order), the 7-toggle rows the Controls array wears (toggleCrouch/toggleSprint the ToggleKeyMapping law), the MouseSettings' other 5 rows (mouseWheelSensitivity the extra walk).
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
    i = data.rindex("---\nTask ID: autopush-207")
    print(f"{path}: verified tail -> {data[i:i+40]!r}")

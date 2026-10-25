import os

ENTRY = """---
Task ID: dev-171
Agent: main agent (rust-minecraft 40-min dev round, [System 2026-10-25 10:22:38 +08:00], trace 1a0a02c3c1060631-cron-agent-loop-202610251031, Job 384925)
Task: ONE minimal interactive step — the keyDebugCrash FUNCTIONAL lane (the RESUME PLAN's first candidate): the F3+C hold the 10-second debug-crash clock serves, the warnings the two channels speak, the remaining < 2s the game's own end answers.

Work Log:
- git status re-check: HEAD d7b2cae (autopush-208 mirror, v0.3.958), clean tree, origin synced; the last dev entry dev-170, this round dev-171.
- THE ROUND'S OWN PICK: the keyDebugCrash lane (the RESUME PLAN's first candidate) — the F3+C hold the crash clock serves.
- PROBE: scripts/jar_work/crashhold_171.py-era dumps via the controls_185 chain (the fresh runs the two walks bank) — the keyPress pc 61-69 the bl7 the modifier==overlay comparison answers (the usedDebugKeyAsModifier the TRUE normally — the debug_modifier_used law), the arm pc 81-186 the RAW isKeyDown poll + the F3 gate + the `> 0` state + the `-1` release-clear + the start's clock/reported/count banks, the tick() pc 0-214 the FULL body (the remaining `10000 - (now - time)` the ldc2_w keeps, the `< 2000` the ReportedException "Manually triggered debug crash" + the CrashReport "Manual crash details" the athrow serves, the Blaze3D.youJustLostTheGame() the Ctrl easter egg the arm rides, the `>= 1000` cadence the count-0 the debugFeedbackComponent YELLOW held row speaks — the `debug.crash.message` key the en_us row `F3 + C is held down. This will crash the game unless released.` the baked text the args never bend — the rest the debugWarningComponent RED countdown speaks, the `debug.crash.warning` row `Crashing in %s...` the `Mth.ceil(remaining / 1000.0)` arg answers) + the en_us rows (the `debug.crash.help` the family keeps) + the NO-GATE law (the walks no screen/world check wear).
- IMPL: screens.rs the THREE fields (debug_crash_key_time / debug_crash_reported_time / debug_crash_reported_count — the ctor zeros) + tick_debug_crash_hold(crash_down, f3_down, now) the merged frame arm (the keyPress arm the state serves, the tick walk the expiry + the warnings the injected now answers — the testable clock the sleep-free walk banks, the stopping flag the crash the run loop's own exit speaks, the clock the crash walk never clears); window.rs the call the f3_down hoist the AFTER seat keeps (the raw isKeyDown(C) the poll serves, now_millis() the frame clock, the OUTSIDE-the-in-world seat the no-gate law wears — the hold the menus the same clock keeps).
- TEST: part_4.rs the_debug_crash_hold_the_clock_the_warnings_the_crash_wear (the start the clock + the zero count + the modifier flag banks; the idle clocks the lone-F3/lone-C gates refuse; the first second the §e held row speaks; the 2.5s the §cCrashing-in-8 the ceil answers; the release the -1 clear; the restart the re-arm; the remaining < 2s the stopping flag banks, the clock the crash walk never clears) — part_4 162 passed (161 +1).
- THE WALK: the twelve-window foreground walk rc=0 (part_1..11 + rest), fast crates 337 (157 + 100 + 80), goldens 19/19 rms 0.000 NO regeneration (the chat rows + the stopping flag the goldens never open).
- fmt: the touched files the ZERO new diffs (the part_4 10419 the HEAD's own drift keeps, the fmt-all the round NEVER runs).
- Version 0.3.958 -> 0.3.959 (bump_version.py the six crates + lock wear).
- Commit 141c520 (scripts/commit_msg_959.txt) pushed origin main: d7b2cae..141c520.

Stage Summary:
- v0.3.959: the F3+C crash-hold the FUNCTIONAL lane wears — the 10-second hold the warnings count down (the §e held row the first second, the §c Crashing-in-N the rest), the remaining < 2s the game's own end the stopping flag answers, the release the clock clears — the vanilla "Manually triggered debug crash" behavior the client's own loop speaks.
- The debug family now the FULL keyPress pair: the handleDebugKeys lanes + the crash-hold clock the keyPress side banks.
- NEXT CANDIDATES: the ADVANCED TOOLTIPS' OWN RENDER (the item tooltip the flag gates — the container hover the ID/durability rows the tooltip walk banks), the keyDebugSpectate lane (the pc 420 the cycle-spectator walk the SPECTATOR packet speaks), the keyDebugDumpDynamicTextures lane (the pc 802 the silent lane), the keyDebugDebugOptions lane (the pc 651 the debug-options screen walk), the key.hotbar.9's own reset arm, the orderI=0 rows' own alphabetical re-seat (the compareTo law the i18n-name secondary the groups re-order), the 7-toggle rows the Controls array wears (toggleCrouch/toggleSprint the ToggleKeyMapping law), the MouseSettings' other 5 rows (mouseWheelSensitivity the extra walk).
"""

for path in [
    "/home/z/my-project/rust-minecraft/logs/worklog.md",
    "/home/z/my-project/worklog.md",
]:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "a", encoding="utf-8") as f:
        f.write(ENTRY)
    print(f"appended -> {path}")

for path in [
    "/home/z/my-project/rust-minecraft/logs/worklog.md",
    "/home/z/my-project/worklog.md",
]:
    data = open(path, encoding="utf-8").read()
    i = data.rindex("---\nTask ID: dev-171")
    print(f"{path}: verified tail -> {data[i:i+36]!r}")

import os

ENTRY = """---
Task ID: autopush-209
Agent: GLM (Z.ai)
Task: Job 384923 — rust-maintenance cadence tick ([System 2026-10-25 10:48:12 +08:00], trace 1a0a02c3c1060631-cron-agent-loop-202610251057): git status check, ONE minimal step, cargo test --workspace green + push, worklog entry.

Work Log:
- git status re-check: HEAD 8404221 (dev-171 mirror), clean tree, origin synced — the live log the compression lag the re-check corrects (autopush-208 v0.3.958 F3+T reload resource packs + dev-171 v0.3.959 F3+C debug crash hold already banked by the interleaved rounds, the worklog's last autopush entry 208, this round 209).
- THE ROUND'S OWN PICK: the keyDebugSpectate FUNCTIONAL lane (the RESUME PLAN's next candidate, the F3 family's ninth implemented lane) — the F3+N cycle-spectator chord.
- PROBE: scripts/jar_work/spectate_209.py (chained off controls_185.py helpers, the cp list the index walk keeps) -> the lane body pc 420-555 (the `matches` arm the entry keeps; the gate chain pc 431-459: the player-null `ifnull -> 462` the ERROR WORD arm — the copyLocation's silent-null `ifnull -> 1163` the neighbor differs —, the `GameModeCommand.PERMISSION_CHECK.check(permissions())` gate the fall-through the same word wears; the cycle walk pc 472-551: the `!isSpectator()` arm the ServerboundChangeGameModePacket(GameType.SPECTATOR) send serves, the spectator arm the `MoreObjects.firstNonNull(gameMode.getPreviousPlayerMode(), GameType.CREATIVE)` restore serves; ALL arms the `goto -> 554` reach the istore_3 used flag the REGARDLESS bank) + the cp[349] `debug.creative_spectator.error` string + the en_us rows (`Unable to switch game mode; no permission` the error, `F3 + N = Cycle previous game mode <-> spectator` the help, `key.debug.spectate` = `Cycle Spectator` the label) + the options_file.rs DEBUG_SPECTATE slot 35 the GLFW N default the ctor walk keeps.
- IMPL: screens.rs on_cycle_spectator_key (the let-else gate the error word + the flag the null arm banks, the permission the constant-pass keeps [the apply_gamemode_hovered's own vocabulary], the isSpectator verdict the two arms split, the previous_game_mode.unwrap_or(CREATIVE) the fallback wears, the flag the last statement banks) + send_game_mode_request helper (the apply_gamemode_hovered's own seam the two worlds share: the WIRE world the ContainerCommand::ChangeGameMode{mode_id: wire_id()} the live_clicks send ALONE — no local prediction the echo lane fills; the OFFLINE world the changed-only previous bank + the mode bank + the abilities_regrant the updatePlayerAbilities walk owns); window.rs the F3+N dispatch arm (the f3_down + is_key_pressed(N, No) rising edge, the I/P MIDDLE seat the pc order 351 < 420 < 749 keeps).
- TEST: part_4.rs the trio (the survival default the spectator arm banks — mode Spectator + previous Some(Survival) + the flight/invulnerable regrant the offline seam keeps + the permission pass the word never speaks + the flag the release consumes; the restore the previous returns — Survival + previous Some(Spectator) + the regrant the flight/invulnerable revokes; the None previous the CREATIVE fallback restores; the null body the §eUnable-to-switch error word speaks — the mode the bank the word never moves, the flag the error arm TOO banks; the wire world the packet ALONE sends — the SPECTATOR id 3 + the CREATIVE fallback id 1 the GameType clinit banks, the local mode the previous bank the wire never touches) — part_4 165 passed (162 +3).
- THE WALK: the twelve-window foreground walk rc=0 (part_1..11 + rest, the 580s budget the 7..rest windows the split run wears), fast crates 337 (157 + 100 + 80), goldens 19/19 rms 0.000 NO regeneration (the chord the mode banks + the packet the wire sends — the fixed scenes the cycle never wears).
- FMT: the touched files the rustfmt --check the ZERO new diffs wear (the HEAD-vs-worktree delta the comparison banks — the part_4 10419 clip_crosshair + the part_1 test rows the HEAD's own drift keep; the six flagged assert shapes the one-arg-per-line form the rustfmt verdict fixes, the fmt-all the round NEVER runs [the autopush-205 100+-file drift the lesson]).
- Version 0.3.959 -> 0.3.960 (bump_version.py the six crates + lock wear).
- Commit 0dd1f84 (scripts/commit_msg_960.txt) pushed origin main: 8404221..0dd1f84.

Stage Summary:
- v0.3.960: the F3+N cycle-spectator chord the FUNCTIONAL lane wears — the vanilla `SPECTATOR <-> previous-or-CREATIVE` walk the packet send serves (the error word the no-permission/no-player arm speaks, the firstNonNull fallback the restore keeps, the flag ALL arms bank), the client the real Minecraft's F3+N behavior the family's own shape speaks.
- The mc-client bank the part_4 window 165 (162 + the spectator trio), the fast crates the 337 counts, the goldens the 19/19 rms 0.000.
- NEXT CANDIDATES: the ADVANCED TOOLTIPS' OWN RENDER (the item tooltip the flag gates — the container hover the ID/durability rows the tooltip walk banks), the keyDebugDumpDynamicTextures lane (the pc 802 the silent lane), the keyDebugDebugOptions lane (the pc 651 the debug-options screen walk), the key.hotbar.9's own reset arm, the orderI=0 rows' own alphabetical re-seat (the compareTo law the i18n-name secondary the groups re-order), the 7-toggle rows the Controls array wears (toggleCrouch/toggleSprint the ToggleKeyMapping law), the MouseSettings' other 5 rows (mouseWheelSensitivity the extra walk).
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
    i = data.rindex("---\nTask ID: autopush-209")
    print(f"{path}: rindex={i} ok, tail={data[i:i+60]!r}")

import os

ENTRY = """---
Task ID: autopush-206
Agent: GLM (Z.ai)
Task: Job 384923 — rust-maintenance cadence tick ([System 2026-10-25 08:03:09 +08:00], trace 1a0a02c3c1060631-cron-agent-loop-202610250809): git status check, ONE minimal step, cargo test --workspace green + push, worklog entry.

Work Log:
- State re-check: HEAD 30be095 (dev-168 mirror), clean tree, origin synced.
- THE ROUND'S OWN PICK: the debug family's feedback completion (the RESUME PLAN's second candidate) — the hitboxes/borders/focusPause lanes the debugFeedbackTranslated words the dev-168 showDebugChat shape wears.
- PROBE: scripts/jar_work/feedback_words_206.py -> .txt: the hdk_262.txt feedback mapping (the hitboxes@193/chunkBorders@292/focusPause@796 debugFeedbackTranslated pairs, the reloadChunk@124/spectate@466/switchGameMode@645/reloadResourcePacks@919/copyLocation@1041/dumpVersion@1178 single words, the charts@1201+1224+1247+1270 + dumpDynamicTextures@802 + clearChat@199 + copyRecreateCommand@351 the SILENT lanes the feedback never serves); the cp strings the ldcw@N String-deref the utf8 reads (the glass walk_code the tag-8 fallback the two-step indirection the probe's cp[idx][1] deref answers); the lang rows — `debug.show_hitboxes.on = Hitboxes: shown`/`.off = ...hidden`, `debug.chunk_boundaries.on = Chunk borders: shown`/`.off = ...hidden` (the BOUNDARIES key the chunk_borders guess corrects), `debug.pause_focus.on = Pause on lost focus: enabled`/`.off = ...disabled`.
- IMPL: screens.rs the three toggles the yellow word the push_chat_message serves (the §e prefix the dev-168 §eAdvanced-tooltips shape mirrors — the flip the word the same breath speaks, the debug_modifier_used the family's spend keeps, the F3+P save the removed()-moment law the advancedTooltips-arm-alone verdict banks) — the javap-comment verdicts the pc walks + the lang words each doc banks.
- TESTS: part_4 the_debug_feedback_words_the_family_wears (the three on/off pairs — the six-word queue count, the flag the last lane's spend, the restores each pair speaks) — part_4 window 153 passed (152 +1).
- THE WALK: the twelve-window foreground walk rc=0 (part_1..11 + rest), fast crates 337 (157 + 100 + 80), goldens 19/19 rms 0.000 NO regeneration (the feedback words the goldens never open), fmt the touched files the ZERO new diffs.
- Version 0.3.953 -> 0.3.954 (bump_version.py the six crates + lock wear).
- Commit 3fbff66 (scripts/commit_msg_954.txt) pushed origin main: 30be095..3fbff66.

Stage Summary:
- v0.3.954: the F3+B/G/P toggles the vanilla feedback words speak — the client the real Minecraft's debug-chord chat behavior the family's own completion wears (the charts' honest silence the vanilla walk's own mapping keeps).
- The full mc-client bank: part_4 153, the twelve-window walk rc=0 — 2425+1 = 2426-test bank, 0 failed.
- NEXT CANDIDATES: the keyDebugCopyRecreateCommand's own FUNCTIONAL lane (the F3+I the recreating command the clipboard walk — the player-null gate the pc 368 speaks), the keyDebugReloadChunk's own FUNCTIONAL lane (the F3+A the chunk-reload walk the debug.reload_chunks.message word wears), the ADVANCED TOOLTIPS' OWN RENDER (the item tooltip the flag gates — the container hover the ID/durability rows), the key.hotbar.9's own reset arm, the orderI=0 rows' own alphabetical re-seat (the compareTo law the i18n-name secondary), the 7-toggle rows the Controls array wears (toggleCrouch/toggleSprint the ToggleKeyMapping law), the MouseSettings' other 5 rows (mouseWheelSensitivity the extra walk).
"""

for path in [
    "/home/z/my-project/rust-minecraft/logs/worklog.md",
    "/home/z/my-project/worklog.md",
]:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    if not os.path.exists(path):
        open(path, "w", encoding="utf-8").write("")
    with open(path, "a", encoding="utf-8") as f:
        f.write(ENTRY)
    data = open(path, encoding="utf-8").read()
    i = data.rindex("---\nTask ID: autopush-")
    print(f"appended -> {path}: tail '{data[i:i+34]}...' len={len(data)}")

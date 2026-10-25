import os

ENTRY = """---
Task ID: autopush-213
Agent: GLM (Z.ai)
Task: Job 384923 — rust-maintenance cadence tick ([System 2026-10-25 13:18:13 +08:00], trace 1a0a02c3c1060631-cron-agent-loop-202610251318): git status check, ONE minimal step, cargo test --workspace green + push, worklog entry.

Work Log:
- git status re-check: HEAD 7301219 (autopush-212 mirror, v0.3.963), clean tree, origin synced — the worklog's last autopush entry 212, this round 213 (no dev round landed in between, the dev cadence the next tick the 13:38 slot wears).
- THE ROUND'S OWN PICK: the keyDebugDebugOptions FUNCTIONAL lane (the autopush-212 RESUME PLAN's first candidate, the F3 family's eleventh implemented lane) — the F3+F6 chord the DebugOptionsScreen walk serves.
- PROBE: scripts/jar_work/debugopt_213.py + canint_213.py (chained off controls_185.py helpers, the ref_name walk the try/except guard wears) -> the lane body pc 650-747 (the keyDebugDebugOptions `matches` gate, the NO-WORD lane the cp[415] `debug.gamemodes.error` word the switchGameMode tail owns — the probe's window walk the word BEFORE the gate banks, the INSTANCEOF arm pc 671-690 the onClose ride keeps, the ELSE arm pc 693-743 the canInterruptScreen gate + the close-old + the `new DebugOptionsScreen()` zero-arg ctor serves, the USED FLAG pc 746-747 the ALL-arms law keeps — the canInterrupt refusal the `ifeq -> 746` jump the same istore banks) + the en_us rows (debug.options.help = 'F3 + F6 = Edit debug options', the key.debug.debugOptions bind the lang wears, the title/search/warning/profile/category rows the screen keeps) + the DebugOptionsScreen class walk (the HeaderAndFooterLayout(61, 33) ctor, the search box the rowWidth/3 width, the SUBTITLE the withColor(-2142128) coral, the addProfileButton the 120px pair + the `!isUsingProfile` active law, the OptionList.updateSearch the allEntries sort + the path filter + the CategoryEntry/OptionEntry pair, the OptionEntry the literal-path label + the THREE CycleButtons the booleanBuilder colored words wear — the coral -2142128 the selected arm, the tan -4539718 the unselected pair) + the canint_213.py walk (Gui.canInterruptScreen the two-hop delegation, the DeathScreen shouldCloseOnEsc iconst_0 override the sweep's ONLY refusal banks, the DebugScreenEntryList resetToProfile/isUsingProfile + the DebugScreenEntries.PROFILES map — DEFAULT the nine IN_OVERLAY text rows, PERFORMANCE the FPS ALWAYS_ON + the TPS/GPU/MEMORY/IMPACTORS pair) + the registry id scan (the 40 entry ids the clinit banks — chunk_borders + entity_hitboxes the renderer entries the client's flags keep, NO charts/lightmap entries the overlay's own toggles the registry never wears).
- IMPL: screens.rs on_debug_options_key (the instanceof arm the open screen closes, the else arm the canInterrupt gate serves — the paused world the close ride releases, the DebugOptions variant opens, the flag ALL arms bank) + debug_options_can_interrupt (the screen-null world the !dead verdict keeps, the open screens the DifficultyLockConfirm refusal alone wears) + the ScreenKind::DebugOptions variant + the debug_profile_performance field + the on_escape arm (the base Screen's shouldCloseOnEsc default the close keeps) + the on_click arms (dbgprofdflt/dbgprofperf the profile walks — the FPS ALWAYS_ON raise + the renderer reset keeps, the DEFAULT return the fresh preset answers; dbgalways_*/dbgoff_* the pill walks the F3+G/F3+B flags the same stores serve) + render_debug_options (the dim the switcher's shape keeps, the title + the search row + the coral warning wrap + the profile pair the isUsingProfile active law wears, the rows the literal-path labels + the Always/Off pills the coral/tan words keep, the category header the RENDERER label keeps); window.rs the F3+F6 dispatch arm (the f3_down + is_key_pressed(F6, No) rising edge, the N/P middle seat the pc order 557 < 651 < 749 keeps) + the screen-open second-tap arm (the switcher's F4 cycle the neighbor seat wears — the exclusive screen states the double-fire never speak).
- TEST: part_4.rs the trio (the open walk the interrupt gate banks + the second tap the instanceof arm closes + the Escape lane the same close wears + the pause arm the close ride releases; the dead world the DeathScreen refusal the flag STILL banks + the lock-confirm the same refusal wears; the chrome walk the pills + the profiles wear — the coral/tan pills the flags' own states speak, the Performance click the fps ALWAYS_ON raise + the renderer reset keeps, the Default return the fresh preset answers, the inactive button's wall the `!isUsingProfile` law keeps, the render-rebuild the buttons-clear law the re-render arms) — part_4 173 passed (170 +3), the first-run verdict the inverted-active slip the perf-click test the fix's own verdict banks (the isUsingProfile law the FIRST draft flipped, the vanilla putfield the corrected answer wears).
- THE WALK: the twelve-window foreground walk rc=0 (part_1..11 + rest, one window per call the dev-166 answer wears), fast crates 337 (157 + 100 + 80), goldens 19/19 rms 0.000 NO regeneration (the chord-driven screen the headless scenes never wear — the fixed scenes the F3+F6 press never speaks).
- FMT: the touched files the rustfmt --check the ZERO new diffs wear (the part_4 10419 clip_crosshair block the HEAD's own drift the stash-verify keeps — the rustfmt pass the drift block the byte-exact restore wore, the diff the HEAD's own shape answers; the rustfmt shapes the assert one-lines the diff's own answer adopts, the fmt-all the round NEVER runs [the autopush-205 100+-file drift the lesson]).
- Version 0.3.963 -> 0.3.964 (bump_version.py the six crates + lock wear).
- Commit 46aeffd (scripts/commit_msg_964.txt) pushed origin main: 7301219..46aeffd.

Stage Summary:
- v0.3.964: the F3+F6 debug-options chord the FUNCTIONAL lane wears — the vanilla DebugOptionsScreen the open/close/interrupt walk serves (the instanceof arm the second tap closes, the canInterrupt gate the DeathScreen + lock-confirm refusals the flag still banks, the chrome the title/search/warning/profile pair the coral/tan pills the client's own entry flags keep), the client the real Minecraft's F3+F6 behavior the family's own shape speaks.
- The mc-client bank the part_4 window 173 (170 + the debug-options trio), the fast crates the 337 counts, the goldens the 19/19 rms 0.000.
- NEXT CANDIDATES: the keyDebugDumpVersion lane (the pc 1164 the version-dump walk — the debug.dump_version word the chat row keeps), the key.hotbar.9's own reset arm, the orderI=0 rows' own alphabetical re-seat (the compareTo law the i18n-name secondary the groups re-order — the DebugOptionsScreen$OptionList's own lambda$static$0 comparator the same law wears), the 7-toggle rows the Controls array wears (toggleCrouch/toggleSprint the ToggleKeyMapping law), the MouseSettings' other 5 rows (mouseWheelSensitivity the extra walk), the DEBUG OPTIONS' OWN ENTRIES ROUND (the full registry rows the updateSearch walk keeps — the 40 entry ids the DebugScreenEntries clinit banks, the tri-state store the Always/In Overlay/Off arms speak, the search-box responder the filter serves, the save/load file the DebugScreenEntryList walk wears).
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
    i = data.rindex("---\nTask ID: autopush-213")
    print(f"verified {path} rindex={i} len={len(data)}")

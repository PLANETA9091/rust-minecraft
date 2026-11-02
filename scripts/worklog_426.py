#!/usr/bin/env python3
"""autopush-426 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_426.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = "--- Task ID: autopush-426 (2026-11-02 21:36:53 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611022144) — v0.3.1219 THE ENCHANTING TABLE'S OWN DEFERRED PLACE\n- Calibration: the WARM sandbox the round answers (the repo the 425 round's own mirror commit the head wears — 887e097f clean + origin-synced, the cargo 1.99.1 the PATH keeps, the worklog's 105 autopush entries the count keeps (autopush-425 the last), the wheel the autopush-426 rotates): the tree the 425 walk's own verdict the head banks. Chosen step: THE ENCHANTING TABLE'S OWN DEFERRED PLACE (the 425 NEXT list's own first walk the round answers — the qc_release walk the family rounds bank: the Chest/Brewing/Dispenser/Crafting/Furnace/Beacon deferred places the empty-paint tail serves, the Enchanting canvas the walk SKIPPED — the armed empty-paint release the INVENTORY fallback the wrong panel wore: the inventory's own click lanes the mirror seats answered, the inv_carried belief the enchant canvas never holds, the place-back and the throw the wrong stack's walks spoke). The anvil's own deferred place the round's own survey DEFERS (the same walk the repair canvas owes — the NEXT list's own second lane the later round wears), the enchant quickcraft echo the same survey defers (the live drag lane the arm's own gate refuses — the QUICK_CRAFT echo the later round owes).\n- THE WALK (the same pc 325-449 walk the chest geometry answers): the release position the vanilla law reads [the last motion the cursor pos speaks], the seat the click walk's own PICKUP serves [click_enchanting_slot the armed button wears — the place-back the tables answer, the right drag's deferred place the place-ONE speaks, the pair the max-one ceiling the seat's own jar wears (the EnchantmentMenu$2 the getMaxStackSize()=1 override speaks), the mirror rows the whole stack the plain 64-ceiling takes], the outside the -999 THROW the 339 walk serves [throw_enchanting_carried_outside the whole carried the PRIMARY button drops, the ONE item the right wears — the offline sim the local throw IS], the between the -1 no-op keeps, THE LIVE WINDOW the arm never enters [the drag lane the arm's own gate refuses — the qc_arm_enchanting's own is_live the silence keeps, the QUICK_CRAFT echo the later round owes, the release the gate the silence keeps].\n- TESTS: part_11.rs NEW ONE (the_enchanting_table_the_deferred_place_wears — the arms wear: the place-back's own whole stack [the mirror row the menu-2 items-9 seat takes, the cursor the walk clears, the arm the tail clears], the right's own place-ONE [the ONE item the EMPTY row takes, the remainder the cursor keeps], the outside's own whole drop [the 339 walk the cursor's bank clears, the kind the drop's own NONE retires], the between's own no-op [the cursor the release never moves, the arm the tail clears either way], the live window's own silence [the outside release the throw never wears, no packet the release mints — the arm's own gate the drag never arms]). Verdict: green (the offline drag the release serves — the live window the arm's gate keeps). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1632/1435 passed, each rc=0, 0 failed — the counts the 425 baseline the w11 the deferred place's own walk +1 keeps, the other windows the 425 numbers keep) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (67 passed, 0 failed, scripts/directed_426.py the 71 filters the deferred place's own head the part_11 gate rides — the FINAL-code re-run the fmt's own whitespace walk the round re-wears) + the workspace's own full link (the --no-run walk the evidence chain keeps). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical on the FINAL code, the red line wears — the lane the release dispatch's own enchant arm the only branch moves, the render the round never edits, the golden scenes the bytes never see). FMT: the region law (scripts/fmt_probe_426.py — the transient pair's own deferred place + the round's own test, ALL-REGIONS-CLEAN — the outside let's own reflow the probe's first walk caught, the two-line shape the rustfmt answer wears). Bump 0.3.1218 -> 0.3.1219 (bump_version.py the workspace version + lock wear, the six members the same version keeps). Commit 250437eb pushed (887e097f..250437eb main).\n- NEXT CANDIDATES: THE ANVIL'S OWN DEFERRED PLACE (the empty release the repair canvas the same walk owes — the offline drag the release the enchanting's own shape serves, the rename field's own press the walk never touches), THE ENCHANTING'S OWN QUICKCRAFT ECHO (the live drag lane the arm's gate refuses — the arm's own un-refusal the wire round wears, the painted distribution the wire packets the chest's own shape re-wears), THE BREWING STAND'S OWN QUICKCRAFT ECHO (the live drag lane the wire round owes — the painted distribution the wire packets the chest's own shape re-wears), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears).\n"

MARK = "--- Task ID: autopush-426 "

auth = open(AUTH, encoding="utf-8").read()
mirror = open(MIRROR, encoding="utf-8").read()
assert MARK not in mirror, "the mirror the round's own marker already wears"
assert auth.count(MARK) == 1, "the authoritative file the marker once wears"
assert auth.endswith(ENTRY), "the authoritative tail the entry wears"
assert mirror.endswith("\n"), "the mirror's own tail the newline keeps"
head_mirror = mirror[:200]
head_auth = auth[:200]

new_mirror = mirror + ENTRY
open(MIRROR, "w", encoding="utf-8").write(new_mirror)

check_auth = open(AUTH, encoding="utf-8").read()
check_mirror = open(MIRROR, encoding="utf-8").read()
assert check_auth.startswith(head_auth), "the authoritative head the sync never moves"
assert check_mirror.startswith(head_mirror), "the mirror's own head the sync never moves"
assert check_auth.endswith(ENTRY), "the authoritative tail the entry wears"
assert check_mirror.endswith(ENTRY), "the mirror's own tail the entry wears"

n_auth = check_auth.count("\n--- Task ID: autopush-")
n_mirror = check_mirror.count("\n--- Task ID: autopush-")
assert n_auth == n_mirror == 106, f"the entry count the 106 wears ({n_auth}/{n_mirror})"
print(f"worklog_426: both files at {n_auth} entries, head/tail verified")

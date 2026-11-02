#!/usr/bin/env python3
"""autopush-427 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_427.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = "--- Task ID: autopush-427 (2026-11-02 22:21:55 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611022230) — v0.3.1220 THE ANVIL'S OWN DEFERRED PLACE\n- Calibration: the WARM sandbox the round answers (the repo the 426 round's own mirror commit the head wears — 5376a127 clean + origin-synced, the cargo 1.99.1 the PATH keeps, the worklog's 106 autopush entries the count keeps (autopush-426 the last), the wheel the autopush-427 rotates): the tree the 426 walk's own verdict the head banks. Chosen step: THE ANVIL'S OWN DEFERRED PLACE (the 426 NEXT list's own first walk the round answers — the qc_release walk the family rounds bank: the Chest/Brewing/Enchanting/Dispenser/Crafting/Furnace/Beacon deferred places the empty-paint tail serves, the Anvil canvas the walk SKIPPED — the armed empty-paint release the INVENTORY fallback the wrong panel wore: the inventory's own click lanes the mirror seats answered, the anvil_carried belief the repair canvas never holds, the place-back and the throw the wrong stack's walks spoke). The enchant + brewing quickcraft echoes the round's own survey DEFERS (the live drag lane the arm's own gate refuses — the wire round the later rounds wear), the hopper's face move-one index the same survey defers.\n- THE WALK (the same pc 325-449 walk the chest geometry answers): the release position the vanilla law reads [the last motion the cursor pos speaks], the seat the click walk's own PICKUP serves [click_anvil_slot the armed button wears — the place-back the tables answer, the right drag's deferred place the place-ONE speaks, the input pair the plain 64-ceiling takes, the result seat the held cursor's no-op the mayPlace-false keeps], the outside the -999 THROW the 339 walk serves [throw_anvil_carried_outside the whole carried the PRIMARY button drops, the ONE item the right wears — the offline sim the local throw IS], the between the -1 no-op keeps, THE LIVE WINDOW the arm never enters [the drag lane the arm's own gate refuses — qc_arm_anvil's own is_live the silence keeps, the wire echo the later round owes, the release the gate the silence keeps].\n- TESTS: part_11.rs NEW ONE (the_anvil_the_deferred_place_wears — the arms wear: the input seat's own whole stack [the menu 0 the anvil_slots pair takes, the cursor the walk clears, the arm the tail clears], the right's own place-ONE [the ONE item the material seat takes, the remainder the cursor keeps], the outside's own whole drop [the 339 walk the cursor's bank clears, the kind the drop's own NONE retires], the between's own no-op [the cursor the release never moves, the arm the tail clears either way], the live window's own silence [the outside release the throw never wears, no packet the release mints — the arm's own gate the drag never arms]). Verdict: green (the offline drag the release serves — the live window the arm's gate keeps). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1633/1435 passed, each rc=0, 0 failed — the counts the 426 baseline the w11 the deferred place's own walk +1 keeps, the other windows the 426 numbers keep) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (68 passed, 0 failed, scripts/directed_427.py the 72 filters the deferred place's own head the part_11 gate rides) + the workspace's own full link (the --no-run walk the evidence chain keeps). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical on the FINAL code — the post-bump re-verify the red line re-wears — the lane the release dispatch's own anvil arm the only branch moves, the render the round never edits, the golden scenes the bytes never see). FMT: the region law (scripts/fmt_probe_427.py — the repair walk's own deferred place + the round's own test, ALL-REGIONS-CLEAN — the seat assert's own collapse the probe's first walk caught, the one-line shape the rustfmt answer wears). Bump 0.3.1219 -> 0.3.1220 (bump_version.py the workspace version + lock wear, the six members the same version keeps). Commit e2b20452 pushed (5376a127..e2b20452 main).\n- NEXT CANDIDATES: THE ENCHANTING'S OWN QUICKCRAFT ECHO (the live drag lane the arm's gate refuses — the arm's own un-refusal the wire round wears, the painted distribution the wire packets the chest's own shape re-wears), THE BREWING STAND'S OWN QUICKCRAFT ECHO (the live drag lane the wire round owes — the painted distribution the wire packets the chest's own shape re-wears), THE ANVIL'S OWN QUICKCRAFT WIRE (the drag the offline window wears — the wire distribution the chest's own lanes may lend, the live echo the honesty re-keys), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears).\n"

MARK = "--- Task ID: autopush-427 "

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
assert n_auth == n_mirror == 107, f"the entry count the 107 wears ({n_auth}/{n_mirror})"
print(f"worklog_427: both files at {n_auth} entries, head/tail verified")

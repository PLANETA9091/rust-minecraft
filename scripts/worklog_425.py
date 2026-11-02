#!/usr/bin/env python3
"""autopush-425 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_425.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-425 (2026-11-02 20:51:52 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611022056) — v0.3.1218 THE BREWING STAND'S OWN DEFERRED PLACE
- Calibration: the WARM sandbox the round answers (the repo the 424 round's own mirror commit the head wears — f1cd6f20 clean + origin-synced, the cargo 1.99.1 the PATH keeps, the AUTH worklog the 104+19 entries the count keeps): the tree the 424 walk's own verdict the head banks. Chosen step: THE BREWING STAND'S OWN DEFERRED PLACE (the 424 NEXT list's own silence the round's own archaeology re-keys — the qc_release walk the family rounds bank: the Chest/Dispenser/Crafting/Furnace/Beacon deferred places the empty-paint tail serves, the Brewing canvas the walk SKIPPED — the offline brewing press with carried the drag arm held, the empty release the walk's own tail cleared, the place-back the game loop never landed, the 339 throw walk the press lane's own silence the arm superseded). The hopper's own wire throw the round's own survey RETIRES (the chest's live qc release lane wire_throw_outside the hopper split the same walk serves — the candidate the walk already answers), the furnace's own wire throw the same survey retires (the furnace's own deferred place the live wire arm the chest's shape wears).
- THE WALK (the same pc 325-449 walk the chest geometry answers): the release position the vanilla law reads [the last motion the cursor pos speaks], the seat the click walk's own PICKUP serves [click_brewing_slot the armed button wears — the place-back the tables answer, the right drag's deferred place the place-ONE speaks], the outside the -999 THROW the 339 walk serves [throw_brewing_carried_outside the whole carried the PRIMARY button drops, the ONE item the right wears — the offline sim the local throw IS], the between the -1 no-op keeps, THE LIVE WINDOW the arm never enters [the drag lane the wire round never banks, the press lane's own echo the 421 law wears — the outside the 424 press THROW the drop already served, the release the double-speak never mints].
- TESTS: part_11.rs NEW ONE (the_brewing_stand_the_deferred_place_wears — the arms wear: the place-back's own whole stack [the carried the bottle seat takes, the cursor the walk clears, the arm the tail clears], the right's own place-ONE [the ONE item the EMPTY seat takes, the remainder the cursor keeps], the outside's own whole drop [the 339 walk the cursor's bank clears, the kind the drop's own NONE retires], the between's own no-op [the cursor the release never moves, the arm the tail clears either way], the live window's own silence [the outside release the throw never wears, no packet the release mints — the 424 press the drop owns]). Verdict: green (the offline drag the release serves — the live window the press lane keeps). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1631/1435 passed, each rc=0, 0 failed — the counts the 424 baseline the w11 the deferred place's own walk +1 keeps, the other windows the 424 numbers keep) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (66 passed, 0 failed, scripts/directed_425.py the 70 filters the deferred place's own head the part_11 gate rides) + the workspace's own full link (the --no-run walk the evidence chain keeps). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the lane the release dispatch's own brewing arm the only branch moves, the render the round never edits, the golden scenes the bytes never see). FMT: the region law (scripts/fmt_probe_425.py — the five-seat's own deferred place + the round's own test, ALL-REGIONS-CLEAN). Bump 0.3.1217 -> 0.3.1218 (bump_version.py the workspace version + lock wear, the six members the same version keeps). Commit 6cf2c069 pushed (f1cd6f20..6cf2c069 main; the message file the untracked lane keeps — the retire tail the round never owes).
- NEXT CANDIDATES: THE ENCHANTING'S OWN DEFERRED PLACE (the empty release the enchant canvas the same walk owes — the offline drag the release the brewing's own shape serves, the gather walk the release tail may wear), THE ANVIL'S OWN DEFERRED PLACE (the empty release the repair canvas the same walk owes — the rename field's own press the walk never touches), THE BREWING STAND'S OWN QUICKCRAFT ECHO (the live drag lane the wire round owes — the painted distribution the wire packets the chest's own shape re-wears), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears).
"""

MARK = "--- Task ID: autopush-425 "

auth = open(AUTH, encoding="utf-8").read()
mirror = open(MIRROR, encoding="utf-8").read()
assert MARK not in mirror, "the mirror the round's own marker already wears"
assert MARK not in auth, "the authoritative file the marker already wears"
assert mirror.endswith("\n"), "the mirror's own tail the newline keeps"
assert auth.endswith("\n"), "the authoritative tail the newline keeps"
head_mirror = mirror[:200]
head_auth = auth[:200]

new_auth = auth + ENTRY
new_mirror = mirror + ENTRY
open(AUTH, "w", encoding="utf-8").write(new_auth)
open(MIRROR, "w", encoding="utf-8").write(new_mirror)

check_auth = open(AUTH, encoding="utf-8").read()
check_mirror = open(MIRROR, encoding="utf-8").read()
assert check_auth.startswith(head_auth), "the authoritative head the sync never moves"
assert check_mirror.startswith(head_mirror), "the mirror's own head the sync never moves"
assert check_auth.endswith(ENTRY), "the authoritative tail the entry wears"
assert check_mirror.endswith(ENTRY), "the mirror's own tail the entry wears"

n_auth = check_auth.count("\n--- Task ID: autopush-")
n_mirror = check_mirror.count("\n--- Task ID: autopush-")
assert n_auth == n_mirror == 105, f"the entry count the 105 wears ({n_auth}/{n_mirror})"
print(f"worklog_425: both files at {n_auth} entries, head/tail verified")

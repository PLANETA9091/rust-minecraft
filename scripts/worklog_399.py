#!/usr/bin/env python3
"""autopush-399 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_399.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-399 (2026-11-01 17:32:44 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611011651) — v0.3.1191 THE ANVIL'S OWN NUMBERING WALK
- Calibration: the session the round's own warmth keeps (the repo + the cargo + the .mc2rs cache the prior round's lanes wear, no reset the walk re-installed); HEAD 25237e6d clean + origin-synced (the autopush-398 round's own mirror commit the head wears); the worklog's 78 autopush entries the count keeps (autopush-398 the last), the wheel the autopush-399 rotates. Chosen step: THE ANVIL'S OWN NUMBERING WALK (the 398 NEXT list's own walk the round re-reads — the calibration the offline anvil's own lane the walk re-reads the deeper defect the kind tail's own ground the round first retires). THE NEXT LIST'S OWN CALIBRATION the round banks: THE BREWING STAND'S / THE ENCHANT TABLE'S OWN WIRE WHOLESALE LANDING the candidates the code the walk re-reads DEFER — the live_container_updated dispatch the brewing (5) / enchant (4) menu types the honest drop keeps [the world_model's own constants the wire never names], the wholesale landing the whole live lane's own open/sync/click walk first demands (the same-walk rounds the later wheel rides), THE DISPENSER SEAT'S OWN KIND BANK the block-entity store's own fields the bigger walk keeps.
- THE WALK (the anvil's offline lane the round re-keys — the enchant shape's own bounds the chimera the prior lane wore): the anvil's own map the three sites re-key (the main rows 3..29 the +6 shift the trio walk keeps, the hotbar 30..38 the -30 shift the drain speaks [the anvil_menu_to_inventory law + the render rects' 3+i chain the same numbering wear]), the quick-move write's own re-key (the menu 29 the items[0] the plain delta wrote [the main-last row the hotbar head the clobber rode], the hotbar tail rows the items[1..9] the shifted drain wore [the main head the spent, the head row the never-landed]), the believed read's own bound (the menu 29 the items[-1] the underflow panic rode — the plain left-click the main grid's last row the crash the client wore), the seat write's own bound (the same <29 wall the +6 shift the write never kept). The comment's own words the code finally answers (the lane's own doc the correct map the prior rounds wore, the code the chimera kept).
- TESTS: part_11.rs NEW ONE (the_anvil_numbering_the_walk_the_bounds_rekey_wears — the five arms wear: the main-last's own plain click [the underflow's own retire, the pickup the row the carry rides], the main-last's own ride [the drained row the move empties, the hotbar head the ride never clobbers], the hotbar head's own ride [the -30 shift the row lands, the neighbor the ride never spends], the hotbar tail's own ride [the main head the clobber retires], the main-last's own merge [the seat's room the merge takes, the row the drain wears]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1605/1435 passed, each rc=0, 0 failed — the counts the autopush-398 baseline the w11 the anvil numbering's own walk +1 keeps, the other windows the 398 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the anvil numbering walk + the anvil wire wholesale kind landing + the offline dispenser quick-move kind tail + the wire dispenser quick-move kind ride + the hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own thirty-nine — 39 passed, 0 failed, scripts/directed_399.py the 39th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the offline numbering's re-key walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_399.py — the anvil numbering walk's three bound sites + the part_11 tail, ALL-REGIONS-CLEAN). Bump 0.3.1190 -> 0.3.1191 (bump_version.py the workspace version + lock wear). Commit 2d221ff2 pushed (25237e6d..2d221ff2 main).
- NEXT CANDIDATES: THE ANVIL'S OWN OFFLINE KIND TAIL (the 398 NEXT list's own walk the numbering walk's ground now wears — the offline anvil quick-move lane the words the plain deltas never move, the drained mirror row the stale word keeps, the offline chest's own walk the same tail re-wears), THE TRIO'S OWN MAY-PLACE THE PREDICTION RETIRES (the quick-move grid the result seat the empty-dest pass the landings may wear — the result seat the mayPlace-false law the shared prediction's own blocked-seat walk first names), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps — the live lane's own open/sync/click walk the wholesale landing the same law answers), THE ENCHANT TABLE'S OWN WIRE LANE (the menu type 4 the same honest drop keeps — the brewing's own walk the same ride wears).
"""

MARK = "--- Task ID: autopush-399 "

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
assert check_auth.count(MARK) == 1 and check_mirror.count(MARK) == 1
print("worklog dual wear ok: authoritative + mirror, one entry each")
sys.exit(0)

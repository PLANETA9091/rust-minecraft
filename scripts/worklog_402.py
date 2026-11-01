#!/usr/bin/env python3
"""autopush-402 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_402.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-402 (2026-11-01 19:20:31 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611011841) — v0.3.1194 THE ENCHANT'S OWN PAIR MAY-PLACE
- Calibration: the session the round's own warmth keeps (the repo + the cargo + the .mc2rs cache the prior round's lanes wear, no reset the walk re-installed); HEAD 78f3dfe6 clean + origin-synced (the autopush-401 round's own mirror commit the head wears); the worklog's 81 autopush entries the count keeps (autopush-401 the last), the wheel the autopush-402 rotates. Chosen step: THE ENCHANT'S OWN PAIR MAY-PLACE (the 401 NEXT list's own first walk the round answers — the lapis seat the mayPlace law the prediction's destinations never wear, the dirt the lapis seat the empty-dest pass may land, the anvil's own blocked marker the same walk answers).
- THE PAIR (the vanilla EnchantmentMenu the quick-move arms the seats' own law keeps): the lapis source the (1,2) range the lapis seat answers, the non-lapis source the (0,1) range the item seat speaks, the shared prediction's own forward walk the illegal seat the blocked marker never reads (the (MAX, MAX) pair the same-item pass AND the empty-dest pass the seat never read — the merge the marker's id never rides, the empty gate the marker's count never opens, the dirt the lapis seat the empty-dest pass never lands, the remainder the source keeps, the illegal walk the no-entries silence the wire's own correction answers; the mirror source the marker's own scope wears — the seat source the player rows the vanilla's own arm keeps, the lapis the registry's own id the item_id_by_name lane answers).
- TESTS: part_11.rs NEW ONE (the_enchant_pair_the_may_place_wears — the four arms wear: the dirt's own landing [the item seat the non-lapis source wears, the lapis seat the may-place never opens, the drained row the move empties], the lapis's own route [the lapis seat the lapis source wears, the item seat the route never opens], the dirt's own silence [the occupied item seat the empty-dest pass refuses, the lapis seat the blocked marker the corruption retires, the no-room walk the silence the source keeps], the lapis's own merge [the same-id room the pass-one merge takes, the drained row the remainder keeps, the item seat the route never opens]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1608/1435 passed, each rc=0, 0 failed — the counts the autopush-401 baseline the w11 the enchant pair may-place's own walk +1 keeps, the other windows the 401 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the enchant pair may-place + the anvil quick-move kind ride + the trio may-place + the anvil numbering walk + the anvil wire wholesale kind landing + the offline dispenser quick-move kind tail + the wire dispenser quick-move kind ride + the hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own forty-two — 42 passed, 0 failed, scripts/directed_402.py the 42nd filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the quick-move lane's seat walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_402.py — the enchant pair may-place's own block + the part_11 tail, ALL-REGIONS-CLEAN). Bump 0.3.1193 -> 0.3.1194 (bump_version.py the workspace version + lock wear). Commit cdd6ba36 pushed (78f3dfe6..cdd6ba36 main).
- NEXT CANDIDATES: THE ENCHANT'S OWN QUICK-MOVE KIND RIDE (the enchant's own quick-move lane the words the plain deltas never move — the anvil's own ride the same tail re-wears, the enchant_comps the lanes' own ride the store keeps), THE ITEM SEAT'S OWN MAX-ONE (the enchant's item seat the vanilla's own getMaxStackSize-1 the prediction's item ceiling never wears — the 64-stack the seat the wholesale may land, the shared walk the slot-max hint answers), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps — the live lane's own open/sync/click walk the wholesale landing the same law answers).
"""

MARK = "--- Task ID: autopush-402 "

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

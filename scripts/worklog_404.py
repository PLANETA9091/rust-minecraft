#!/usr/bin/env python3
"""autopush-404 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_404.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-404 (2026-11-01 20:56:07 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611012008) — v0.3.1196 THE ENCHANT'S OWN ONE-ITEM SEAT LAW
- Calibration: the session the round's own warmth keeps (the repo + the cargo + the .mc2rs cache the prior round's lanes wear, no reset the walk re-installed); HEAD 936ade9f clean + origin-synced (the autopush-403 round's own mirror commit the head wears); the worklog's 83 autopush entries the count keeps (autopush-403 the last), the wheel the autopush-404 rotates. Chosen step: THE ENCHANT'S OWN ONE-ITEM SEAT LAW (the 403 NEXT list's own first walk the round answers — the jar the premise retires, the vanilla quickMoveStack the bytes the fresh evidence speaks).
- THE JAR'S OWN EVIDENCE (the javap the JRE headless never ships — the sandbox's own class-file parser the disasm answers [scripts/jar_work_parse_enchant_quickmove_404.py, scripts/jar_work/enchant_quickmove_404.txt the bank keeps]): the vanilla EnchantmentMenu.quickMoveStack the bytes speak — the seat sources the moveItemStackTo(2, 38, true) REVERSE order the shared walk already wears [the 403 candidate's forward premise the jar retires — no walk the round re-keys there], the lapis row the moveItemStackTo(1, 2, true) range the whole stack wears [the blocked marker the (1,2) range the 402 law keeps], the non-lapis row the hand-rolled copyWithCount(1)/shrink(1) arm the wholesale land retires [the occupied item seat the silence keeps, the empty seat the ONE takes, the remainder the source row keeps].
- THE WALK (the enchant lane's own two arms the round re-keys): the non-lapis row the hand walk the walker bypass answers (the empty gate the count the silence wears, the hasItem gate the occupied seat never spends, the ONE the copyWithCount shape takes, the remainder the (count - 1) the shrink shape keeps; the lapis seat the arm never reads), the lapis row + the seat sources the shared walk the marker + the reverse order keep.
- TESTS: part_11.rs NEW ONE (the_enchant_rows_the_one_item_seat_wears — the six arms wear: the one-item's own landing [the stackable row the empty seat the ONE takes, the remainder the source keeps], the occupied seat's own silence [the hasItem gate the walk never spends, the stack the source keeps], the single stack's own whole move [the count-1 row the shrink-to-empty tail wears], the lapis arm's own whole stack [the (1,2) range the shared walk keeps, the item seat the route never opens], the hotbar source's own one-item [the -29 shift the hand walk wears, the remainder the hotbar row keeps], the seat source's own whole stack [the reverse order the shared walk keeps, the mirror tail the landed stack wears]) + the PRIOR LAW'S OWN RETIRE (the_enchant_pair_the_may_place_wears the ARM A the one-item law re-wears [the dirt the ONE takes, the remainder the source keeps — the wholesale land the 402 walk wore], the_offline_enchanting_the_click_the_round_wear the quick-move walk the same law re-wears [the ONE banks, the remainder the source keeps, the ride-home totals the count-preservation keeps]). Verdict: green (the rework the jar's own evidence retires, the arms the fresh law first-run wears).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1610/1435 passed, each rc=0, 0 failed — the counts the autopush-403 baseline the w11 the one-item seat's own walk +1 keeps, the other windows the 403 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil quick-move kind ride + the anvil result-seat may-place + the anvil numbering walk + the anvil wire wholesale kind landing + the offline dispenser quick-move kind tail + the wire dispenser quick-move kind ride + the wire hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own forty-four — 44 passed, 0 failed, scripts/directed_404.py the 44th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the quick-move lane's walk shape alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_404.py — the rows' own two arms + the part_11 trio [the offline click's own rework + the 402 pair may-place's own ARM A rework + the one-item seat's six arms], ALL-REGIONS-CLEAN). Bump 0.3.1195 -> 0.3.1196 (bump_version.py the workspace version + lock wear). Commit 2afe9e3d pushed (936ade9f..2afe9e3d main).
- NEXT CANDIDATES: THE ITEM SEAT'S OWN MAY-PLACE (the enchant's item seat the vanilla's own isValidEnchantmentTarget the mayPlace gate the quick-move arm never reads — the jar's own 146 check the byte walk banks, the dirt the seat the enchantable-target law never opens), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps — the live lane's own open/sync/click walk the wholesale landing the same law answers), THE ANVIL'S OWN JAR RE-READ (the AnvilMenu.quickMoveStack the same parser the bytes may speak — the anvil's own arms the walker's assumptions the fresh evidence re-keys).
"""

MARK = "--- Task ID: autopush-404 "

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

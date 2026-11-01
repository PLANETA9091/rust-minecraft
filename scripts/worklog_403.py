#!/usr/bin/env python3
"""autopush-403 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_403.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-403 (2026-11-01 20:07:24 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611011921) — v0.3.1195 THE ENCHANT'S OWN QUICK-MOVE KIND RIDE
- Calibration: the session the round's own warmth keeps (the repo + the cargo + the .mc2rs cache the prior round's lanes wear, no reset the walk re-installed); HEAD 81a9ff75 clean + origin-synced (the autopush-402 round's own mirror commit the head wears); the worklog's 82 autopush entries the count keeps (autopush-402 the last), the wheel the autopush-403 rotates. Chosen step: THE ENCHANT'S OWN QUICK-MOVE KIND RIDE (the 402 NEXT list's own first walk the round answers — the enchant's own quick-move lane the kind words the plain deltas never move, the anvil's own ride the same tail re-wears, the enchant_comps the lanes' own ride the store keeps).
- THE RIDE (the 384 law the enchant's own lane joins — the source row's own PRE-READ the ride banks [the mirror rows the INVENTORY bank's normalized read speaks — the row_potion_kind the own gate the read wears, the pair seats the kindless store the honest NONE keeps — the dev-204 law the plain walk's read wears], the destinations' own BEFORE-COUNTS the empty-fill gate reads, the empty dest the placed stack's own word banks [the mirror rows the INVENTORY bank the positional map writes — the +7 main shift the 2..28 rows answers, the -29 hotbar shift the 29..37 rows speaks; the pair seats the kindless store the honest wall keeps — the stale word the ride's own correction retires], the occupied merge the original's word keeps, the kinded remainder the word keeps [the max-1 potion the one-seat fill wears — the remainder the source keeps, the word the same row wears], the drained row the NONE retires, the no-room walk the silence the stack's own word keeps [the empty changed the tail never wears]; the seat source's own tail the pair seat the no-bank honesty keeps — the menu >= 2 gate the write the real rows only answers).
- TESTS: part_11.rs NEW ONE (the_enchant_quick_move_the_kind_ride_wears — the five arms wear: the mirror main drain's own retire [the kindless seat the word never banks, the drained row the NONE retires], the hotbar drain's own retire [the -29 shift the mapped row reads], the kinded remainder's own word [the max-1 item the one-seat fill wears, the remainder the source keeps, the word the same row wears], the no-room's own silence [the occupied item seat the different-item pass refuses, the lapis seat the blocked marker the corruption retires, the word the same row wears], the kindless seat's own correction [the stale word the row bank keeps, the kindless ride the empty dest the NONE banks, the stale word the ride's own correction retires]). Verdict: green FIRST RUN (the max-1 potion the walker's own ceiling the merge-arm redesign wears — the (potion, 60) merge the ONE-stack table refuses, the one-seat fill the remainder arm answers).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1609/1435 passed, each rc=0, 0 failed — the counts the autopush-402 baseline the w11 the enchant kind ride's own walk +1 keeps, the other windows the 402 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the enchant quick-move kind ride + the enchant pair may-place + the anvil quick-move kind ride + the anvil result-seat may-place + the anvil numbering walk + the anvil wire wholesale kind landing + the offline dispenser quick-move kind tail + the wire dispenser quick-move kind ride + the wire hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own forty-three — 43 passed, 0 failed, scripts/directed_403.py the 43rd filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the quick-move lane's kind walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_403.py — the enchant quick-move kind ride's own lane [the two-line marker the anvil's own pre-read twin disambiguates] + the part_11 tail, ALL-REGIONS-CLEAN). Bump 0.3.1194 -> 0.3.1195 (bump_version.py the workspace version + lock wear). Commit f2f96be7 pushed (81a9ff75..f2f96be7 main).
- NEXT CANDIDATES: THE ENCHANT'S OWN FORWARD WALK (the seat source the vanilla moveItemStackTo(2, 38, false) the forward order speaks — the walker's rev() the hotbar-first order the enchant lane wears, the vanilla's own main-first walk the divergence keeps), THE ITEM SEAT'S OWN MAX-ONE (the enchant's item seat the vanilla's own getMaxStackSize-1 the prediction's item ceiling never wears — the 64-stack the seat the wholesale may land, the shared walk the slot-max hint answers), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps — the live lane's own open/sync/click walk the wholesale landing the same law answers).
"""

MARK = "--- Task ID: autopush-403 "

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

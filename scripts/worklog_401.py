#!/usr/bin/env python3
"""autopush-401 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_401.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-401 (2026-11-01 18:40:48 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611011809) — v0.3.1193 THE ANVIL'S OWN QUICK-MOVE KIND RIDE
- Calibration: the session the round's own warmth keeps (the repo + the cargo + the .mc2rs cache the prior round's lanes wear, no reset the walk re-installed); HEAD 3f3b67bf clean + origin-synced (the autopush-400 round's own mirror commit the head wears); the worklog's 80 autopush entries the count keeps (autopush-400 the last), the wheel the autopush-401 rotates. Chosen step: THE ANVIL'S OWN QUICK-MOVE KIND RIDE (the 400 NEXT list's own first walk the round answers — the 398 NEXT list's own walk the clean trio the blocked marker's walk leaves, the 397 round the dispenser's own tail the same ride speaks).
- THE RIDE (the 384 law the furnace's own trio wears): the source row's own PRE-READ the ride banks (the trio seats the seat store's own word the id gate answers [the anvil_seat_potion_kind the own split speaks], the mirror rows the INVENTORY bank's normalized read speaks [the row_potion_kind the own gate the read wears]), the destinations' own BEFORE-COUNTS the empty-fill gate reads (the same cells the deltas the ride below mutates), the empty dest the placed stack's own word banks (the trio seats the seat store's word the write-through keeps, the mirror rows the items bank's mapped row the same walk answers [the main rows the +6 shift speaks, the hotbar the -30 shift answers]), the occupied merge the original's word keeps, the kinded remainder the word keeps (the tail the src_new's own id the gate reads), the drained row the NONE retires, the no-room walk the silence the stack's own word keeps (the empty changed the tail never wears), the trio source's own ride the word the mirror row banks (the drained seat the NONE retires).
- TESTS: part_11.rs NEW ONE (the_anvil_quick_move_the_kind_ride_wears — the five arms wear: the mirror main drain's own retire [the placed stack's own word the seat banks, the drained row the NONE retires], the hotbar drain's own retire [the -30 shift the mapped row reads], the kinded remainder's own word [the one-seat trio the partial move wears, the remainder the source keeps], the no-room's own silence [the full trio the move refuses, the word the same row wears], the trio source's own ride [the word the seat store lends, the hotbar-tail row the bank keeps, the drained seat the NONE retires]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1607/1435 passed, each rc=0, 0 failed — the counts the autopush-400 baseline the w11 the anvil kind ride's own walk +1 keeps, the other windows the 400 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the anvil quick-move kind ride + the trio may-place + the anvil numbering walk + the anvil wire wholesale kind landing + the offline dispenser quick-move kind tail + the wire dispenser quick-move kind ride + the hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own forty-one — 41 passed, 0 failed, scripts/directed_401.py the 41st filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the quick-move lane's kind walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_401.py — the anvil quick-move kind ride's own lane + the part_11 tail, ALL-REGIONS-CLEAN). Bump 0.3.1192 -> 0.3.1193 (bump_version.py the workspace version + lock wear). Commit 7ad6ff88 pushed (3f3b67bf..7ad6ff88 main).
- NEXT CANDIDATES: THE ENCHANT'S OWN PAIR MAY-PLACE (the lapis seat the mayPlace law the prediction's destinations never wear — the dirt the lapis seat the empty-dest pass may land, the anvil's own blocked marker the same walk answers), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear), THE ENCHANT'S OWN QUICK-MOVE KIND RIDE (the enchant's own quick-move lane the words the plain deltas never move — the anvil's own ride the same tail re-wears, the enchant_comps the lanes' own ride the store keeps), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps — the live lane's own open/sync/click walk the wholesale landing the same law answers).
"""

MARK = "--- Task ID: autopush-401 "

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

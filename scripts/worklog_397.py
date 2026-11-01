#!/usr/bin/env python3
"""autopush-397 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_397.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-397 (2026-11-01 16:09:15 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611011536) — v0.3.1189 THE OFFLINE DISPENSER'S OWN QUICK-MOVE KIND TAIL
- Calibration: the session the round's own warmth keeps (the repo + the cargo + the .mc2rs cache the prior round's lanes wear, no reset the walk re-installed); HEAD 972fd2c1 clean + origin-synced (the autopush-396 round's own mirror commit the head wears); the worklog's 76 autopush entries the count keeps (autopush-396 the last), the wheel the autopush-397 rotates. Chosen step: THE OFFLINE DISPENSER'S OWN QUICK-MOVE KIND TAIL (the 396 NEXT list's own first walk the round answers — the offline dispenser quick-move lane the 396 live walk's own tail re-wears, the offline chest's own law the stale word the drain the plain deltas never moved the lane retires).
- THE TAIL (the 370 law the nine's own lane re-speaks): the source row's own PRE-READ the tail banks (the nine the kindless store the honest NONE wall keeps [the (id, count) store the word never carried], the inventory row the bank's normalized read answers — the id gate the read itself wears [row_potion_kind the own gate the read wears]), the drained mirror source the NONE retires (the stale word the drain retires — the shifted potion's own row the honest wall keeps), the kinded remainder the word keeps (the tail the src_new's own id the gate reads), the no-room walk the silence the stack's own word keeps (the empty changed the tail never wears), the grid seats the kindless store the silence keeps (the words the (id, count) store never carried — the landed stack the wordless wall wears, the grid source's own walk the menu wall the tail never wears).
- TESTS: part_11.rs NEW ONE (the_offline_dispenser_quick_move_the_kind_tail_wears — the five arms wear: the mirror main drain's own retire [the grid fill the forward order wears, the drained row the NONE retires], the hotbar drain's own retire [the -36 shift the mapped row reads, the drained row retires], the kinded remainder's own word [the one-seat grid the partial move wears, the remainder the source keeps], the no-room's own silence [the full grid the move refuses, the word the same row wears], the grid potion's own silent landing [the kindless wall the landing keeps, the hotbar-first order the tail row wears]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1603/1435 passed, each rc=0, 0 failed — the counts the autopush-396 baseline the w11 the offline dispenser quick-move's own tail +1 keeps, the other windows the 396 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the offline dispenser quick-move kind tail + the wire dispenser quick-move kind ride + the hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own thirty-seven — 37 passed, 0 failed, scripts/directed_397.py the 37th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the quick-move lane's tail walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_397.py — the offline dispenser quick-move lane's own tail + the part_11 tail, ALL-REGIONS-CLEAN). Bump 0.3.1188 -> 0.3.1189 (bump_version.py the workspace version + lock wear). Commit a513ceb3 pushed (972fd2c1..a513ceb3 main).
- NEXT CANDIDATES: THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend), THE WIRE CHEST'S OWN DROP KIND RETIRE (the outside drop the None the chest lane passes — the word the stale kind the emptied carried keeps, the furnace's own Some the chest lane re-wears), THE ANVIL'S OWN WIRE KIND WALK (the live lane's own projection the seat words the wholesale landing may re-bank — the offline store's own law the wire tail follows), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear).
"""

MARK = "--- Task ID: autopush-397 "

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

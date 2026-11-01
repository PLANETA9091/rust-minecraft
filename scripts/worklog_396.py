#!/usr/bin/env python3
"""autopush-396 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_396.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-396 (2026-11-01 15:36:16 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611011451) — v0.3.1188 THE WIRE DISPENSER'S OWN QUICK-MOVE KIND RIDE
- Calibration: the sandbox the round's own reset the clone re-wore (the repo + the cargo the fresh lanes re-installed, the restore_env.sh the .mc2rs cache the four display lanes re-wore — the worklog the mirror's own bytes the authoritative lane re-restored); HEAD 330dcc81 clean + origin-synced (the autopush-395 round's own mirror commit the head wears); the worklog's 75 autopush entries the count keeps (autopush-395 the last), the wheel the autopush-396 rotates. Chosen step: THE WIRE DISPENSER'S OWN QUICK-MOVE KIND RIDE (the 395 NEXT list's own first walk the round answers — the dispenser's own live quick-move lane the chest family's own ride the 3x3's own walk re-wears, the 384 law the plain deltas the kind words never moved the wire lane re-arms).
- THE RIDE (the 395 law the nine's own lane re-speaks — the shift-click's multi-slot deltas the kind words follow): the source row's own PRE-READ the ride banks (the nine the kindless store the honest NONE wall keeps [the (id, count) store the word never carried], the mirror rows the MIRROR's own id the gate keeps — the wire belief the real inventory's stale row never keys, the main rows the seat itself speaks, the hotbar the -36 shift answers), the destinations' own BEFORE-COUNTS the empty-fill gate reads (the old pairs the same cells the deltas mutate), the empty mirror dest (old.1 <= 0) the placed stack's own word banks (the stale word the kindless ride retires, the grid seats the silence the honest wall keeps — the store the word never carried), the occupied merge the original's word keeps (the write never wears), the drained mirror source the NONE retires (the kinded remainder the word keeps — the tail the src_new's own id the gate reads), the no-room walk the silence the stack's own word keeps. THE KINDLESS WALL the family tail answers (the chest family's own banks the container rows wear, the dispenser's nine the bank never carried — the same ride the bankless lane the honest NONE wall the ride keeps).
- TESTS: part_11.rs NEW ONE (the_wire_dispenser_quick_move_the_kind_ride_wears — the six arms wear: the mirror main row's own tail [the grid fill the forward order wears, the drained row retires], the hotbar row's own tail [the -36 shift the mapped row reads, the drained row retires], the grid source's own ride [the kindless stack the hotbar-first order rides, the empty mirror dest the NONE banks, the stale word the ride retires, the drained seat the store wall's silence wears], the occupied merge's own silence [the original's word keeps, the stale word the ride never re-keys], the kinded remainder's own word [the one-seat grid the partial move wears, the remainder the source keeps], the no-room's own silence [the full grid the move refuses, the word the same row wears]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1602/1435 passed, each rc=0, 0 failed — the counts the autopush-395 baseline the w11 the dispenser quick-move's own ride +1 keeps, the other windows the 395 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep — the fresh-sandbox restore_env the .mc2rs cache the four display lanes re-wore) + the directed families (the dispenser quick-move kind ride + the hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own thirty-six — 36 passed, 0 failed, scripts/directed_396.py the 36th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the quick-move lane's bank walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_396.py — the dispenser quick-move lane's own ride + the part_11 tail, ALL-REGIONS-CLEAN). Bump 0.3.1187 -> 0.3.1188 (bump_version.py the workspace version + lock wear). Commit 67e3c6d0 pushed (330dcc81..67e3c6d0 main).
- NEXT CANDIDATES: THE DISPENSER'S OWN OFFLINE KIND WALK (the offline dispenser quick-move lane the words the plain deltas never move — the mirror source's own drain the stale word the offline lane never retires, the offline chest's own walk the same tail re-wears), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend), THE WIRE CHEST'S OWN DROP KIND RETIRE (the outside drop the None the chest lane passes — the word the stale kind the emptied carried keeps, the furnace's own Some the chest lane re-wears), THE ANVIL'S OWN WIRE KIND WALK (the live lane's own projection the seat words the wholesale landing may re-bank — the offline store's own law the wire tail follows).
"""

MARK = "--- Task ID: autopush-396 "

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

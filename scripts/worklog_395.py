#!/usr/bin/env python3
"""autopush-395 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_395.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-395 (2026-11-01 13:36:55 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611011251) — v0.3.1187 THE HOPPER'S OWN QUICK-MOVE KIND RIDE
- Calibration: the sandbox the round's own reset the clone re-wore (the repo + the cargo the fresh lanes re-installed, the restore_env.sh the .mc2rs cache the four display lanes re-wore — the worklog the mirror's own bytes the authoritative lane re-restored); HEAD f567ded7 clean + origin-synced (the autopush-394 round's own mirror commit the head wears); the worklog's 74 autopush entries the count keeps (autopush-394 the last), the wheel the autopush-395 rotates. Chosen step: THE HOPPER'S OWN QUICK-MOVE KIND RIDE (the 394 NEXT list's own third walk the round answers — the chest family's live quick-move lane the dev-212 door's own walk the kind words the plain deltas never moved, the 384 law the furnace's own trio wears the five seats' own routing re-arms).
- THE RIDE (the 370 offline law the wire lane re-wears — the shift-click's multi-slot deltas the kind words follow): the source row's own PRE-READ the ride banks (the container rows the chest bank's normalized read answers [the chest_row_potion_kind the id gate wears], the mirror rows the MIRROR's own id the gate keeps — the wire belief the real inventory's stale row never keys), the destinations' own BEFORE-COUNTS the empty-fill gate reads (the old pairs the same cells the deltas mutate), the empty dest (old.1 <= 0) the placed stack's own word banks (the container rows the chest bank's own words answer, the store rows the items bank's mapped row the same closure maps [rel < 27 the 9+rel main rows, the -27 shift the hotbar answers]), the occupied merge the original's word keeps (the write never wears), the drained source the NONE retires (the kinded remainder the word keeps — the src_new's own id the tail gate reads), the no-room walk the silence the stack's own word keeps (the empty changed the ride never wears). THE SHARED LANE the family tail answers (the chest's own 27 + the double's 54 + the hopper's five the same n-aware count keys — the mapping the seat_kinds closure wears the words the same rows read).
- TESTS: part_11.rs NEW ONE (the_wire_hopper_quick_move_the_kind_ride_wears — the six arms wear: the five seat's own bank [the grid src the hotbar-first order the tail seat rides, the word the mapped row banks, the drained seat retires], the mirror main seat's own bank [the forward order the grid fill wears, the drained row retires], the occupied merge the original's word keeps [the stale word the ride never re-keys, the kindless src the NONE retires], the no-room walk the silence the stack's word keeps, the hotbar seat's own bank [the -27 shift the mapped row reads], the chest's own rows the same lane re-wears [the family tail the count re-keys]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1601/1435 passed, each rc=0, 0 failed — the counts the autopush-394 baseline +1 the w11 the hopper quick-move's own ride, the other windows the 394 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep — the fresh-sandbox restore_env the .mc2rs cache the four display lanes re-wore) + the directed families (the hopper quick-move kind ride + the furnace quick-move ride + the hopper paint gate + the wire hopper door + the wire generic/chest/dispenser/furnace/table places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip/drag rides' own thirty-five — 35 passed, 0 failed, scripts/directed_395.py the 35th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the quick-move lane's bank walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_395.py — the quick-move lane's own ride + the part_11 tail, the usize cast the parse wall the parens re-keys, ALL-REGIONS-CLEAN). Bump 0.3.1186 -> 0.3.1187 (bump_version.py the workspace version + lock wear). Commit 79aa4616 pushed (f567ded7..79aa4616 main).
- NEXT CANDIDATES: THE WIRE DISPENSER'S OWN QUICK-MOVE KIND RIDE (the dispenser's own live lane the same ride the family tail may re-wear — the shared lane's own law the 3x3's own walk may answer), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend), THE WIRE CHEST'S OWN DROP KIND RETIRE (the outside drop the None the chest lane passes — the word the stale kind the emptied carried keeps, the furnace's own Some the chest lane re-wears), THE ANVIL'S OWN WIRE KIND WALK (the live lane's own projection the seat words the wholesale landing may re-bank — the offline store's own law the wire tail follows).
"""

MARK = "--- Task ID: autopush-395 "

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

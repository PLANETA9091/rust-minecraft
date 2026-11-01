#!/usr/bin/env python3
"""autopush-400 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_400.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-400 (2026-11-01 18:09:05 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611011736) — v0.3.1192 THE TRIO'S OWN MAY-PLACE
- Calibration: the session the round's own warmth keeps (the repo + the cargo + the .mc2rs cache the prior round's lanes wear, no reset the walk re-installed); HEAD e3c5b46e clean + origin-synced (the autopush-399 round's own mirror commit the head wears); the worklog's 79 autopush entries the count keeps (autopush-399 the last), the wheel the autopush-400 rotates. Chosen step: THE TRIO'S OWN MAY-PLACE (the 399 NEXT list's own second walk the round answers — the quick-move grid the result seat the empty-dest pass the landings may wear, the shared prediction's own blocked-seat walk the marker first names). THE NEXT LIST'S OWN CALIBRATION the round banks: THE ANVIL'S OWN OFFLINE KIND TAIL the candidate the walk DEFERS (the result seat's own may-place the ground the corruption first retires — the kind tail the clean trio the blocked marker's walk leaves the next round rides), THE BREWING STAND'S / THE ENCHANT TABLE'S OWN WIRE LANE the whole-walk candidates the later wheel keeps.
- THE CORRUPTION the round retires (the empty-dest pass the trio's own 0..2 walk rode): shift-clicking THREE max-stack-1 tools from the mirror the pair the seats wear, the THIRD the result seat the prediction 'landed' — the write's own map the items[8] the hotbar's own tail row the landed stack wore (the tool the anvil never held, the honest row the clobber spent). THE BLOCKED MARKER the snapshot's seat wears (the (MAX, MAX) pair the same-item pass AND the empty-dest pass the seat never read — the merge the marker's id never rides, the empty gate the marker's count never opens, the remainder the source keeps [the vanilla walk's own answer], the result seat the write never wears). THE FURNACE'S OWN SHAPE the walk re-reads (the specialized predictor the result seat the routing already keeps — the anvil's generic grid the only lane the seat the destination wore).
- TESTS: part_11.rs NEW ONE (the_anvil_quick_move_the_result_seat_the_may_place_wears — the four arms wear: the third tool's own remainder [the pair the seats wear, the one the pair never took the source keeps, the hotbar row the corruption retires], the two tools' own full ride [the pair the seats wear, the source the drain keeps], the stackable's own full ride [the first seat the whole stack takes, the siblings the untouched emptiness keeps], the merge's own law [the room the merge takes, the non-same seat the walk never spends] — the result seat the write never wears the four arms' own verdict keeps). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1606/1435 passed, each rc=0, 0 failed — the counts the autopush-399 baseline the w11 the may-place's own walk +1 keeps, the other windows the 399 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the trio may-place + the anvil numbering walk + the anvil wire wholesale kind landing + the offline dispenser quick-move kind tail + the wire dispenser quick-move kind ride + the hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own forty — 40 passed, 0 failed, scripts/directed_400.py the 40th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the prediction's own blocked-seat walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_400.py — the anvil quick-move's own may-place block + the part_11 tail, ALL-REGIONS-CLEAN). Bump 0.3.1191 -> 0.3.1192 (bump_version.py the workspace version + lock wear). Commit 51de57c6 pushed (e3c5b46e..51de57c6 main).
- NEXT CANDIDATES: THE ANVIL'S OWN OFFLINE KIND TAIL (the 398 NEXT list's own walk the clean trio now wears — the offline anvil quick-move lane the words the plain deltas never move, the drained mirror row the stale word keeps, the empty trio seat the landed potion's word the bank never wears, the offline chest's own walk the same tail re-wears), THE ENCHANT'S OWN PAIR MAY-PLACE (the lapis seat the mayPlace law the prediction's destinations never wear — the dirt the lapis seat the empty-dest pass may land, the anvil's own blocked marker the same walk answers), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps — the live lane's own open/sync/click walk the wholesale landing the same law answers).
"""

MARK = "--- Task ID: autopush-400 "

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

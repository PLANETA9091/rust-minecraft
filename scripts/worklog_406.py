#!/usr/bin/env python3
"""autopush-406 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_406.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-406 (2026-11-01 22:17:22 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611012136) — v0.3.1198 THE SWAP ARM'S OWN COUNT GATE
- Calibration: the session the round's own warmth keeps (the repo + the cargo + the .mc2rs cache the prior round's lanes wear, no reset the walk re-installed); HEAD febe6110 clean + origin-synced (the autopush-405 round's own mirror commit the head wears); the worklog's 85 autopush entries the count keeps (autopush-405 the last), the wheel the autopush-406 rotates. Chosen step: THE SWAP ARM'S OWN COUNT GATE (the 405 NEXT list's own first walk the round answers — the vanilla PICKUP swap the carried-count gate the bytes speak).
- THE GATE (the evidence the round re-wears — the acm_doClick_262.txt the pcs 989-1001 bank keeps: the `carried.getCount() > slot.getMaxStackSize(carried)` the if_icmpgt the silence answers, the setCarried(slotStack) + slot.setByPlayer(carried) the exchange the fall-through wears): the max-aware predictor's own arms the round re-keys — the kind-pair's early return the same ceiling keeps, the both-filled different-item arm the same law answers [the carried 64 the max-1 seat the exchange never spends, the count-1 the stacks exchange wears]; the plain kind walk the base 64 the gate never binds [the lapis seat + the real rows the carried ceiling the stackable's own 64 keys]. THE FALL (the screens walk's own refusal lane the round re-wears — the component-refusal swap the gate the value branch answers, the refused fall the predictor's own room-0 merge silence answers [the max-1 seat the ONE holds — the merge arm the room the ceiling closes, the same silence the vanilla gate speaks], the store ride's own refusal the same gate conjunct the exchange retires — the silence the stores keep, no ride the values never moved).
- TESTS: part_11.rs NEW ONE (the_enchant_swap_arm_the_count_gate_wears — the five arms wear: the different-item swap's own silence [the 64-carried the max-1 seat the exchange never spends], the count-one's own swap [the single carried the gate opens, the stacks exchange], the component pair's own silence [the enchanted seat the 64-carried the swap never spends, the stores the silence keeps], the component pair's own count-one swap [the single carried the exchange the stores ride], the place arm's own verbatim [the empty seat the ONE takes, the gate the place never binds]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1612/1435 passed, each rc=0, 0 failed — the counts the autopush-405 baseline the w11 the count gate's own walk +1 keeps, the other windows the 405 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil quick-move kind ride + the anvil result-seat may-place + the anvil numbering walk + the anvil wire wholesale kind landing + the offline dispenser quick-move kind tail + the wire dispenser quick-move kind ride + the wire hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own forty-six — 46 passed, 0 failed, scripts/directed_406.py the 50 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the click lane's swap gate alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_406.py — the max predictor's own count gate + the plain arms' own re-key [the value branch + the store ride] + the part_11 test, ALL-REGIONS-CLEAN). Bump 0.3.1197 -> 0.3.1198 (bump_version.py the workspace version + lock wear). Commit 750d7264 pushed (febe6110..750d7264 main).
- NEXT CANDIDATES: THE LAPIS SEAT'S OWN MAY-PLACE (the plain place arm the mayPlace gate the vanilla 1213-1220 bytes keep — the non-lapis the lapis seat the plain place never lands, the quick-move's own marker the same law the 402 walk speaks), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps — the live lane's own open/sync/click walk the wholesale landing the same law answers), THE ANVIL'S OWN JAR RE-READ (the AnvilMenu.quickMoveStack the same parser the bytes may speak — the anvil's own arms the walker's assumptions the fresh evidence re-keys).
"""

MARK = "--- Task ID: autopush-406 "

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

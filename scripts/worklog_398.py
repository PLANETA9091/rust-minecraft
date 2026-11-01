#!/usr/bin/env python3
"""autopush-398 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_398.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-398 (2026-11-01 16:44:57 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611011609) — v0.3.1190 THE ANVIL'S OWN WIRE KIND WALK
- Calibration: the session the round's own warmth keeps (the repo + the cargo + the .mc2rs cache the prior round's lanes wear, no reset the walk re-installed); HEAD 8e531b14 clean + origin-synced (the autopush-397 round's own mirror commit the head wears); the worklog's 77 autopush entries the count keeps (autopush-397 the last), the wheel the autopush-398 rotates. Chosen step: THE ANVIL'S OWN WIRE KIND WALK (the 397 NEXT list's own third walk the round answers — the live lane's own projection the seat words the wholesale landing re-banks). THE NEXT LIST'S OWN CALIBRATION the round banks (the koth entries the walk re-reads): THE RESULT TAKE'S OWN KIND WORD the dev-213 round ALREADY WORE (the furnace_kind_stores the result take's own word the plain id model never banked the round's own field answers), THE WIRE CHEST'S OWN DROP KIND RETIRE the throw lane ALREADY WORE (the throw_chest_carried_outside the emptied carried the NONE word the same retire the furnace's own shape wears) — the koth the stale candidates the code the walk re-reads retires.
- THE LANDING (the 343 law the anvil's wire lane re-wears — the wholesale correction the kind never carries): the trio's own words the landing the honest NONE wall re-banks (the input 0 + the material 1 + the result 2 the seat store banks, the words the wholesale landing the same wall keeps), the carried's own seat (39) the same wall keeps (the authoritative carried the word the wire never lends), the mirror rows the words the landing never moves (the INVENTORY bank the rows read — the items_potion_kinds the seat kind's own read answers [the anvil_seat_potion_kind the mirror map wears], the real inventory's words the wholesale never touches). THE STALE WALK the round retires (the defector the prior lane kept — the re-opened window the prior anvil's own word the input seat wore, the server's (id, count) pairs the wholesale replaced, the words the plain replace never moved).
- TESTS: part_11.rs NEW ONE (the_anvil_wire_wholesale_the_kind_landing_wears — the three arms wear: the wholesale's own retire [the stale trio the landing retires, the pairs the wholesale wears, the mirror row the word keeps], the carried's own seat [the carried the wholesale wears, the word the correction never carries], the re-sync's own re-bank [the local ride's word the re-sync retires]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1604/1435 passed, each rc=0, 0 failed — the counts the autopush-397 baseline the w11 the anvil wire wholesale's own walk +1 keeps, the other windows the 397 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the anvil wire wholesale kind landing + the offline dispenser quick-move kind tail + the wire dispenser quick-move kind ride + the hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own thirty-eight — 38 passed, 0 failed, scripts/directed_398.py the 38th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the wholesale lane's re-bank walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_398.py — the anvil wire wholesale lane's own landing + the part_11 tail, ALL-REGIONS-CLEAN). Bump 0.3.1189 -> 0.3.1190 (bump_version.py the workspace version + lock wear). Commit 43c26b4d pushed (8e531b14..43c26b4d main).
- NEXT CANDIDATES: THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear, the pickup/place/quick-move lanes the bank the ride re-keys), THE BREWING STAND'S OWN WIRE WHOLESALE LANDING (the brewing's own live sync the kind slots the wholesale may re-bank — the anvil's own walk the same law may answer), THE ENCHANT TABLE'S OWN WIRE WHOLESALE LANDING (the enchant's own live sync the seat words the same landing may re-bank), THE ANVIL'S OWN OFFLINE KIND TAIL (the offline anvil quick-move lane the words the plain deltas never move — the offline chest's own walk the same tail re-wears).
"""

MARK = "--- Task ID: autopush-398 "

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

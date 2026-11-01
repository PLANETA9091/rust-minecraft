#!/usr/bin/env python3
"""autopush-410 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_409.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-410 (2026-11-02 02:25:40 +08:00, Job 384925, trace 1a0a02c3c1060631-cron-agent-loop-202611020231) — v0.3.1202 THE DISPENSER SEAT'S OWN KIND BANK
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the rustfmt + the assets-cache the 409 round's lanes wear — no reset the walk re-installed); HEAD ef1f8ffc clean + origin-synced (the 409 round's own mirror commit the head wears); the worklog's 89 autopush entries the count keeps (autopush-409 the last), the wheel the autopush-410 rotates. Chosen step: THE DISPENSER SEAT'S OWN KIND BANK (the 409 NEXT list's own second walk the round answers — the grid nine the kindless store the honest NONE wall the block-entity's own bank retires).
- THE WALK (the offline lane the chest's own 363 law the nine re-wears — the screens lane the field + the normalized read + the open load + the click rides + the close flush the bank's own lifecycle speaks): the field dispenser_row_potion_kinds the (id, count) store's parallel wears [the potion's own identity the store ride survives], the open the container_stores_potion_kinds entry loads [the fresh store the fresh NONE row inserts, the missing entry the resize answers], the normalized read the id wall the word never lends [the potion row the bank's own cell answers, the kindless stack the honest NONE keeps, the missing cell the same silence wears], the pickup walk the grid nine the BANK's own read answers [the word the cursor banks, the seat's own cell the NONE retires, the place the word the seat's own cell lays, the model's own swap the words the pairs exchange — the 378 same-id kind pair the component law the bank wears], the quick-move walk the ride the transfer lane wears [the inventory source the grid dest the word banks, the grid source the player rows the word lends, the drained rows the NONE retires, the kindless stacks the silence keeps], the close the edited words the map entry rides home to [the reopen the same words loads, the disk write the same lane banks]. THE LANES: the LIVE walk the honest NONE wall keeps [the wire belief the (id, count) pairs the word never carries — the block-entity store the words may carry, the live sync the wire alone answers].
- THE LAWS THE ROUND RE-KEYS (the wall's own tests the premise the bank retires): the_dispenser_carried_the_word_wears the re-pickup arm [the kindless seat the NONE word lends — the seat's own word the re-pickup banks], the_dispenser_seats_the_model_ride_wears the arm B [the same-wordless pair the carried word keeps — the seat's own word the carried lends, the seat's own cell the NONE wears].
- TESTS: part_11.rs NEW ONE (the_dispenser_seats_the_kind_bank_wears — the arms wear: the pickup's own bank ride [the word the cursor banks, the seat's cell the NONE retires], the place's own bank ride, the id wall's own read [the kindless stack the stale word never lends], the quick-move's own bank ride [the inventory source the grid dest the word banks, the grid source the player rows the word lends, the drained source the NONE retires], the model's own swap [the words the pairs exchange], the close's own ride home [the reopen the same words loads]). Verdict: green (the fresh-grid arm the stale-pair seat search the earlier arm's potion never names — the walk's own probe the arm state re-keys). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1616/1435 passed, each rc=0, 0 failed — the counts the 409 baseline the w11 the bank's own walk +1 keeps, the other windows the 409 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the dispenser kind bank + the enchant swap-key + the enchant gather + the lapis seat may-place + the offline enchanting round + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil/wire/furnace/dispenser/brewing/drag/store/craft/hopper rides' own fifty-one — 51 passed, 0 failed, scripts/directed_410.py the 55 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the bank the kind lanes' own words alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_410.py — the bank field + the open load + the close flush + the normalized read + the quick-move lane + the pickup lane + the round's own test, ALL-REGIONS-CLEAN [the rustfmt walk the test's own asserts the formatted tail wears]). Bump 0.3.1201 -> 0.3.1202 (bump_version.py the workspace version + lock wear). Commit 9b9822ef pushed (ef1f8ffc..9b9822ef main).
- NEXT CANDIDATES: THE ENCHANT GATHER'S OWN WIRE HALF (the live session the menu never banks — the wire lane the later round owes when the enchant table the live window opens), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps), THE DISPENSER SHOT'S OWN KIND RIDE (the shot beat the store's own pick consumes — the dispensed potion the world's own word the spawn lane may lend, the bank the shot's own drain retires), THE DROPPER'S OWN KIND BANK (the same nine the sibling menu wears — the open_dispenser_for the dropper cell the same bank the load wears, the title the container.dropper the walk banks).
"""

MARK = "--- Task ID: autopush-410 "

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

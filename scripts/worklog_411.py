#!/usr/bin/env python3
"""autopush-411 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_411.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-411 (2026-11-02 04:29:30 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611020321) — v0.3.1203 THE DISPENSER SHOT'S OWN KIND RIDE
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the rustfmt + the assets-cache the 410 round's lanes wear — no reset the walk re-installed); HEAD c9e990d8 clean + origin-synced (the 410 round's own mirror commit the head wears); the worklog's 90 autopush entries the count keeps (autopush-410 the last), the wheel the autopush-411 rotates. Chosen step: THE DISPENSER SHOT'S OWN KIND RIDE (the 410 NEXT list's own third walk the round answers — the shot beat the store's own pick consumes, the dispensed potion the world's own word the spawn lane lends, the bank the shot's own drain retires).
- THE WALK (the drops lane the ItemDrop's own kind field wears — the shot beat + the pickup walk the word's own two lanes speak): the field ItemDrop.potion_kind the entity's own kind answer wears [the vanilla ItemStack's NBT Potion tag the offline entity's honest answer keeps, the four ctor sites the NONE default wears — the spawn + the entity loot + the death drop + the container spill the same silence keeps], the shot beat the seat's own word the 410 normalized read re-wears [the potion stack the bank's own cell answers, the kindless stack the honest NONE keeps, the never-banked entry the same silence wears], the eject the spawn lane's own kind lend speaks [the dispensed drop the seat's own word wears — the potion the world's own entity the identity banks], the write-back the drain's own retire answers [the emptied seat the bank cell the NONE retires — the word the spent stack carried the shot retires, the kindless pick the cell never touches], the pickup walk the landing's own kind ride wears [the fill arm the drop's word the seat banks — the empty row the landed stack the identity answers, the merge arm the seat's own identity keeps, the kindless drop the ride never wears].
- TESTS: part_11.rs NEW ONE (the_dispenser_shot_the_kind_ride_wears — the arms wear: the eject's own kind lend [the drop the seat's word wears], the drain's own retire [the spent seat the write-back empties, the bank cell the NONE answers], the pickup's own ride [the landed stack the word banks, the whole entity the take discards], the id wall's own shot [the kindless stack the stale word never lends, the stale cell the wall never touches]). Verdict: green (the body's own spawn the mod harness's law the walk first wore — the pickup scan the body's own box the absent stage refused). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1617/1435 passed, each rc=0, 0 failed — the counts the 410 baseline the w11 the shot's own ride +1 keeps, the other windows the 410 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the dispenser shot ride + the dispenser kind bank + the enchant swap-key + the enchant gather + the lapis seat may-place + the offline enchanting round + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil/wire/furnace/dispenser/brewing/drag/store/craft/hopper rides' own fifty-two — 52 passed, 0 failed, scripts/directed_411.py the 56 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the field the data lanes' own words alone voices, the golden scenes' wordless drops the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_411.py — the kind field + the four ctor sites + the seat word read + the drain retire + the spawn lend + the pickup ride + the round's own test, ALL-REGIONS-CLEAN [the loot fn's own signature the pre-existing width the round's walk re-forms, the inv_before line + the test's own asserts the rustfmt tail wears, the region walk the position order re-keys]). Bump 0.3.1202 -> 0.3.1203 (bump_version.py the workspace version + lock wear). Commit d34bfd72 pushed (c9e990d8..d34bfd72 main).
- NEXT CANDIDATES: THE DROPPER'S OWN INSERT RIDE (the front container's bank the landing cell the word may bank — the insert walk's own index the lane may lend, the source seat's drain the same retire wears), THE DROPPER'S OWN KIND BANK (the same nine the sibling menu wears — the open_dispenser_for the dropper cell the same bank the load wears, the title the container.dropper the walk banks), THE ENCHANT GATHER'S OWN WIRE HALF (the live session the menu never banks — the wire lane the later round owes when the enchant table the live window opens), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps).
"""

MARK = "--- Task ID: autopush-411 "

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

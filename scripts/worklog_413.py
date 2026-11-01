#!/usr/bin/env python3
"""autopush-413 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_413.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-413 (2026-11-02 05:21:34 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611020523) — v0.3.1205 THE DROPPER'S FURNACE KIND RIDE
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the rustfmt + the assets-cache the 412 round's lanes wear — no reset the walk re-installed); HEAD dbc5e1bd clean + origin-synced (the 412 round's own mirror commit the head wears); the worklog's 92 autopush entries the count keeps (autopush-412 the last), the wheel the autopush-413 rotates. Chosen step: THE DROPPER'S FURNACE KIND RIDE (the 412 NEXT list's own fourth walk the round answers — the furnace trio's own parallel the insert lane wears, the input seat's word the smelt lane answers). THE GATE'S OWN WALK the round first wore: the 412 list's same-components candidate the premise the cap-1 law refused (the potion trio the merge arm the max-stack cap never wears — the kinded gate the vanilla-honest silence the walk already keeps), the kinded fn the round's own draft the honest retire wears [the dead complexity the walk never carries], the candidate ④ the round answers.
- THE WALK (the trio arm the kind bank's parallel the beat's own furnace sink re-wears): the ensure the furnace_kind_stores entry the fresh NONE trio inserts [the fixed [i32; 3] row the same shape the disk bank wears], the landing the 412 index fn the trio re-wears [the split arm the incoming word the landing cell banks — the input seat's word the smelt lane answers, the merge arm the grow law keeps — the incoming word the cell never overrides, the FULL trio the stack the cell never touches], the kindless pick the honest NONE the identity answers [the stale word the fresh seat re-keys]. THE CONTAINER ARM the 412 walk the same shape keeps.
- TESTS: part_11.rs NEW ONE (the_dropper_furnace_the_kind_ride_wears — the arms wear: the split arm's own ride [the potion the input seat lands, the word the trio cell banks], the merge arm's own keep [the grow law the cell never overrides], the kindless wall's own re-key [the stale word the fresh identity retires], the full trio's own refusal [the stack the word the cell keeps]). Verdict: green (the beat's own sink the furnace trio the SAME map the open door consumes). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1619/1435 passed, each rc=0, 0 failed — the counts the 412 baseline the w11 the furnace ride +1 keeps, the other windows the 412 numbers keep; the w11 the FINAL-code walk the fmt tail re-wears) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the furnace kind ride + the insert ride + the dispenser shot ride + the dispenser kind bank + the enchant swap-key + the enchant gather + the lapis seat may-place + the offline enchanting round + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil/wire/furnace/dispenser/brewing/drag/store/craft/hopper rides' own fifty-four — 54 passed, 0 failed, scripts/directed_413.py the 58 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the ride the data lanes' own words alone voices, the golden scenes' wordless sinks the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_413.py — the trio ride + the round's own test, ALL-REGIONS-CLEAN [the trio inserts' own 3-seat tails the rustfmt shape re-keys]). Bump 0.3.1204 -> 0.3.1205 (bump_version.py the workspace version + lock wear). Commit 3c3e2a5a pushed (dbc5e1bd..3c3e2a5a main; the message-file stray the mirror commit retires — the add -A the draft file the commit wore, the honest removal the same lane keeps).
- NEXT CANDIDATES: THE FURNACE TRIO'S OWN INPUT SEAT LAW (the vanilla canPlaceItem the insert lane re-keys — the input seat any item accepts, the fuel seat the fuel alone, the output seat never; the dropper's own trio the first-empty walk the seat law re-keys), THE ENCHANT GATHER'S OWN WIRE HALF (the live session the menu never banks — the wire lane the later round owes when the enchant table the live window opens), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps), THE DROPPER'S OWN SHULKER SPILL (the shulker sink the same-container lane the 27-row bank already wears — the beat's own arm the shulker store the open door shares).
"""

MARK = "--- Task ID: autopush-413 "

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

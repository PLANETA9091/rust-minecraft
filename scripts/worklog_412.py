#!/usr/bin/env python3
"""autopush-412 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_412.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-412 (2026-11-02 04:36:33 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611020436) — v0.3.1204 THE DROPPER'S OWN INSERT RIDE
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the rustfmt + the assets-cache the 411 round's lanes wear — no reset the walk re-installed); HEAD 23bfd769 clean + origin-synced (the 411 round's own mirror commit the head wears); the worklog's 91 autopush entries the count keeps (autopush-411 the last), the wheel the autopush-412 rotates. Chosen step: THE DROPPER'S OWN INSERT RIDE (the 411 NEXT list's own first walk the round answers — the front container's bank the landing cell the word banks, the insert walk's own index the lane lends, the source seat's drain the same retire wears).
- THE WALK (the index lane the world.rs walk's own pair wears — the beat's front arm the front bank's own row the ride speaks): the fn dispenser_insert_into_at the (idx, arm) pair the landing law answers [the merge arm the (idx, true) the grow law keeps — the same-item seat the count the cap answers, the split arm the (idx, false) the first-empty seat speaks — the fresh copy the landing cell wears, the FULL container the honest None keeps — the caller's own stack the refusal preserves], the old bool signature the same walk the wrapper wears [dispenser_insert_into the is_some verdict the belief-add lanes keep], the beat's front arm the front bank's own row re-wears [the 410 chest law the front cell re-wears — the same-cell key the kind map keeps, the fresh row the NONE law inserts, the length the front's own shape the resize answers; the split arm the incoming word the landing cell banks — the seat's own word the identity answers, the merge arm the seat's own word the grow law keeps — the incoming word the cell never overrides, the FULL refusal the stack the cell never touches], the source seat's drain the 411 retire the same lane wears [the emptied seat the bank cell the NONE answers]. THE FURNACE ARM the kindless wall keeps [the furnace trio the words never carry — the kindless lane the honest silence wears].
- TESTS: part_11.rs NEW ONE (the_dropper_insert_the_kind_ride_wears — the arms wear: the pure walk's own index [the split the merge the full the wrapper the four verdicts speak], the split arm's own ride [the landing cell the word banks], the merge arm's own keep [the grow law the seat's own cell wears], the kindless wall's own re-key [the stale word the fresh identity retires, the source cell the wall never touches], the full refusal's own wall [the stack the word the cell keeps]). Verdict: green (the pure arms' own premise the separate vecs the arm state re-keys — the merge arm's own first-seat scan the shared row the premise refused). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1618/1435 passed, each rc=0, 0 failed — the counts the 411 baseline the w11 the insert's own ride +1 keeps, the other windows the 411 numbers keep; the w11 + the rest the FINAL-code walk the fmt tail re-wears) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the insert ride + the dispenser shot ride + the dispenser kind bank + the enchant swap-key + the enchant gather + the lapis seat may-place + the offline enchanting round + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil/wire/furnace/dispenser/brewing/drag/store/craft/hopper rides' own fifty-three — 53 passed, 0 failed, scripts/directed_412.py the 57 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the ride the data lanes' own words alone voices, the golden scenes' wordless sinks the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_412.py — the index fn + the front bank row + the landing ride + the round's own test, ALL-REGIONS-CLEAN [the or_insert closure the one-line tail the rustfmt width wears, the kind rows' own 27 the split tail the assert lines the rustfmt shape re-keys]). Bump 0.3.1203 -> 0.3.1204 (bump_version.py the workspace version + lock wear). Commit 90331205 pushed (23bfd769..90331205 main).
- NEXT CANDIDATES: THE ENCHANT GATHER'S OWN WIRE HALF (the live session the menu never banks — the wire lane the later round owes when the enchant table the live window opens), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps), THE INSERT WALK'S OWN SAME-COMPONENTS LAW (the vanilla isSameItemSameComponents the merge gate the kind pair re-keys — the kindless potion the kinded seat the merge refuses), THE DROPPER'S OWN FURNACE KIND BANK (the furnace trio's own parallel the insert lane may wear — the input seat's word the smelt lane answers).
"""

MARK = "--- Task ID: autopush-412 "

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

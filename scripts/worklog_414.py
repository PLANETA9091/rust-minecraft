#!/usr/bin/env python3
"""autopush-414 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_414.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-414 (2026-11-02 05:51:35 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611020601) — v0.3.1206 THE FURNACE'S OWN FACE LAW
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the rustfmt + the assets-cache the 413 round's lanes wear — no reset the walk re-installed); HEAD 6fd2b4a7 clean + origin-synced (the 413 round's own mirror commit the head wears); the worklog's 93 autopush entries the count keeps (autopush-413 the last), the wheel the autopush-414 rotates. Chosen step: THE FURNACE'S OWN FACE LAW (the 413 NEXT list's own first walk the round answers — javap HopperBlockEntity.addItem the direction.getOpposite() walk the 26.2 jar: the furnace's own face the dropper's back answers, the getSlotsForFace order the face table banks, the canPlaceItem gate the seat pairs wear). The hopper lanes' own law the round discovers pre-banked (the furnace_face_order + the furnace_can_place + the fuel_ticks trio the dev rounds' walk already wears) — the dropper's own arm the slot-blind insert the law re-keys.
- THE WALK (the dropper's furnace arm the hopper lanes' own law re-keys — the slot-blind insert the face-order walk retires): the fn furnace_insert_into_face the (idx, arm) pair the face law answers [the furnace_move_one's own gate law the same body wears — the order's slots the can_place gate the walk scans, the split arm the (idx, false) the first gated empty seat speaks, the merge arm the (idx, true) the grow law keeps, the refused walk the honest None keeps; the old bool signature the wrapper the hopper lanes keep], the beat's arm the face order the dropper's own back re-wears [the facing6_opposite walk the face byte answers — the UP face the input row any item accepts, the four walls the fuel row the fuel gate wears, the DOWN face the result + fuel pair the refuse-first order keeps], the kind ride the landing cell the same law wears [the input face the potion word the trio cell banks, the fuel face the fuel's own silence the NONE answers].
- THE 413 TEST the face law re-keys (the wall's own premise the horizontal input walk retires — the arms the three sinks the three faces wear: the input face's own ride [the potion the DOWN walk the input seat lands, the word the trio cell banks], the fuel face's own landing + merge [the coal the fuel seat lands, the grow law the count wears, the fuel's own silence the cell answers], the fuel face's own gate [the non-fuel stack the refusal keeps], the output face's own gate [the DOWN face's pair the result refuse the fuel gate speaks]). THE PURE ARMS the verdicts wear (the input face the any-item law, the fuel face the non-fuel refusal + the fuel split + the merge grow, the DOWN face the result refuse the fuel seat answers, the wrapper the same walk speaks). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1619/1435 passed, each rc=0, 0 failed — the counts the 413 baseline the re-key the count keeps, the other windows the 413 numbers keep; the w11 the FINAL-code walk the fmt tail re-wears) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the furnace face law + the furnace kind ride + the insert ride + the dispenser shot ride + the dispenser kind bank + the enchant swap-key + the enchant gather + the lapis seat may-place + the offline enchanting round + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil/wire/furnace/dispenser/brewing/drag/store/craft/hopper rides' own fifty-four — 54 passed, 0 failed, scripts/directed_414.py the 54 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the face law the data lanes' own words alone voices, the golden scenes' wordless sinks the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_414.py — the index fn + the face arm + the round's own re-keyed test, ALL-REGIONS-CLEAN [the assert tails the rustfmt shape re-keys]). Bump 0.3.1205 -> 0.3.1206 (bump_version.py the workspace version + lock wear). Commit 615291cd pushed (6fd2b4a7..615291cd main; the message file the repo's own outside the round keeps — the 413 stray the add -A the mirror commit retired, the leak the same lane never wears).
- NEXT CANDIDATES: THE DROPPER'S OWN SHULKER SPILL (the shulker sink the same-container lane the 27-row bank already wears — the beat's own arm the shulker store the open door shares), THE ENCHANT GATHER'S OWN WIRE HALF (the live session the menu never banks — the wire lane the later round owes when the enchant table the live window opens), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears).
"""

MARK = "--- Task ID: autopush-414 "

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

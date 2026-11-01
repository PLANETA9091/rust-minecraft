#!/usr/bin/env python3
"""autopush-407 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_407.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-407 (2026-11-01 23:21:28 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611012321) — v0.3.1199 THE LAPIS SEAT'S OWN MAY-PLACE
- Calibration: the fresh sandbox the reset the round re-installed (the repo the clone re-wore [HEAD 698ae254 clean + origin-synced — the autopush-406 round's own mirror commit the head wears], the cargo 1.99.1 the rustup lane installed, the rustfmt component the same walk added, the jar cache the generate lane rebuilt [cargo run -p mc2rs-gen -- generate --version 26.2 — the assets-cache/jar the 4027 runtime files the sha1-verified walk answers], the atlas/skin codegen the same run clobbered the git restore retired [the committed evidence the manifest drift never wears], the killed-window mod.rs the gate's own restore law re-keyed); the worklog's 86 autopush entries the count keeps (autopush-406 the last), the wheel the autopush-407 rotates. Chosen step: THE LAPIS SEAT'S OWN MAY-PLACE (the 406 NEXT list's own first walk the round answers — the vanilla may-place the bytes the same call family speaks).
- THE GATE (the evidence the round re-wears — the jar's EnchantmentMenu$3 the mayPlace = stack.is(lapis) the enchant_menu_405.txt the anon-class census keeps [the item seat $2 the base mayPlace any item wears, the max-one the 405 round already banks], the doClick bytes the same call the walks pre-key [the acm_doClick_262.txt the both-filled arm the pcs 934-941 the mayPlace ifeq the silence answers — the gate the same-components split precedes, the plain place arm the safeInsert contract the same law wears, the 1213-1220 bytes the same call family speaks]): the screens walk's own pre-conjunct the round wears — the menu == 1 && carried.1 > 0 && carried.0 != lapis the branch split precedes, the both presses the same silence wear [the plain place the seat the press never lands, the both-filled pair the same pre-gate the swap never spends, the right press the safeInsert contract the one-place wears], the lapis carried the gate never binds [the base walk verbatim rides — the whole place the 64 ceiling keeps, the pair the merge answers, the count gate the 406 round never binds, the registry miss the -1 sentinel the quick-move's own read shares].
- TESTS: part_11.rs NEW ONE (the_lapis_seat_the_may_place_wears — the six arms wear: the plain place's own silence [the non-lapis carried the seat the press never lands, the cursor the stack keeps], the lapis carried's own landing [the gate the lapis never binds, the base 64 place the whole stack wears], the both-filled arm's own silence [the lapis seat the diamond carried the pc 934 pre-gate the swap never spends], the lapis pair's own merge [the same-id walk the room the base 64 answers], the right press's own silence [the safeInsert contract the one-place wears], the right press's own lapis one [the ONE the seat takes, the remainder the cursor keeps]) + the offline round's own place lane re-key (the_offline_enchanting_the_click_the_round_wear — the polished_andesite carried the may-place silence wears, the seat the empty keeps, the ride home the cursor's stack answers [the close walk's totals the same 10 the cursor ride wears]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1613/1435 passed, each rc=0, 0 failed — the counts the 406 baseline the w11 the may-place's own walk +1 keeps, the other windows the 406 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the lapis seat may-place + the offline enchanting round + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil quick-move kind ride + the anvil result-seat may-place + the anvil numbering walk + the anvil wire wholesale kind landing + the offline dispenser quick-move kind tail + the wire dispenser/hopper quick-move rides + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own forty-eight — 48 passed, 0 failed, scripts/directed_407.py the 52 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the may-place the click lane's silence alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_407.py — the pre-gate + the offline place lane re-key + the round's own test, ALL-REGIONS-CLEAN [the one long assert the rustfmt walk the pre-break re-keyed]). Bump 0.3.1198 -> 0.3.1199 (bump_version.py the workspace version + lock wear). Commit fd4d633e pushed (698ae254..fd4d633e main).
- NEXT CANDIDATES: THE LAPIS SEAT'S OWN GATHER GATE (the PICKUP_ALL spread the mayPlace the same call wears — the double-click gather the non-lapis carried the lapis seat never paints, the pcs 1675-1695 the direction walk the same bank keeps), THE SWAP-KEY'S OWN MAY-PLACE (the ContainerInput.SWAP the 1213-1220 bytes the number-key walk wears — the F-key the non-lapis the lapis seat never lands), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps — the live lane's own open/sync/click walk the wholesale landing the same law answers).
"""

MARK = "--- Task ID: autopush-407 "

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

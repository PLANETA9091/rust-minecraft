#!/usr/bin/env python3
"""autopush-418 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_418.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-418 (2026-11-02 09:36:42 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611020942) — v0.3.1210 THE ENCHANT COSTS' OWN DATA-WORD LANE
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the rustfmt + the assets-cache the 417 round's lanes wear — no reset the walk re-installed); HEAD b158be85 clean + origin-synced (the 417 round's own mirror commit the head wears); the worklog's 97 autopush entries the count keeps (autopush-417 the last), the wheel the autopush-418 rotates. Chosen step: THE ENCHANT COSTS' OWN DATA-WORD LANE (the 417 NEXT list's own first walk the round answers — the ContainerSetData lane the furnace's progress walk banks: the live window's costs the data words the model's data_slots carry, the zero-cost wall the 417 round's fresh reset kept the words the server speaks retire).
- THE WALK (the sync arm the words the projection's every refresh re-wears): the ids 0..2 the three costs speak [the take(3) walk the furnace_data's own shape wears, the words the wire never spoke the fresh zeros keep], the id 3 the seed word the standalone mirror the DataSlot doc banks [the enchant_seed the word banks — the stale seed the prior window left the fresh open retires, the sign-extended i16 the i32 field the Java int semantics the 1:1 map keeps], the clue rows the same seed+row stream the pure pick fn serves [the refresh's own second pass the wire words re-wear — the item seat the same gate reads, the empty seat the -1 clues keep, the enchanting_power the wire window the scan never rides the 0 keeps — the server's own words the rolls the LOCAL scan never wears, the costs the LOCAL roll never re-rolls], the offline refresh's own roll lane the wire arm never calls [the enchant_costs_refresh the offline door alone serves — the bookshelf scan the table pos the offline door records, the wire window the pos never banks].
- TESTS: part_11.rs NEW ONE (the_enchant_costs_the_data_words_wear — the arms wear: the words' own landing [the ids 0..2 the costs, the id 3 the seed, the clue rows the same pick fn's own answer wears, the no-command lane the sync speaks], the wordless fresh state [the stale seed the fresh open retires, the zeros the costs keep, the -1 the clues wear], the new words' own re-key [the fresh sync the fields re-keys, the clue rows the new stream's own answer wears], the empty seat's own gate [the words the costs still land, the item seat empty the clue picks the gate never feeds]). Verdict: green FIRST RUN. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1623/1435 passed, each rc=0, 0 failed — the counts the 417 baseline the w11 the data-word lane's own walk +1 keeps, the other windows the 417 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the enchant costs data words + the enchant gather wire half + the enchant gather offline + the enchant swap-key + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the lapis seat may-place + the offline enchanting round + the anvil wire + the anvil quick-move kind ride + the anvil result-seat may-place + the anvil numbering walk + the dispenser shulker place law + the dropper shulker place law + the furnace face law + the furnace kind ride + the insert ride + the dispenser shot ride + the dispenser kind bank + the wire/furnace/dispenser/brewing/drag/store/craft/hopper rides' own fifty-eight — 58 passed, 0 failed, scripts/directed_418.py the 62 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the data words the click lane's own display fields alone voice, the golden scenes' enchant canvases the offline stand-in the flag never wears, no render walk the round edits). FMT: the region law (scripts/fmt_probe_418.py — the wire sync arm's own data-word lane + the round's own test, ALL-REGIONS-CLEAN [the power assert the rustfmt shape re-keys]). Bump 0.3.1209 -> 0.3.1210 (bump_version.py the workspace version + lock wear). Commit 7236b444 pushed (b158be85..7236b444 main; the retire tail bc2ed961 the message file's own lane answers — the apostrophe lanes the -F wear, the 412 law the same file keeps).
- NEXT CANDIDATES: THE ENCHANT PICKUP'S OWN WIRE ECHO (the prediction-free echo the anvil's dev-202 walk banks — the live window's pickup press the empty delta the lane may echo, the server's own correction the projection re-banks), THE BREWING STAND'S OWN WIRE LANE (the menu type 11 the clinit order the constants' docs bank — the OpenScreen word the honest drop keeps), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears), THE ENCHANT OPTION'S OWN WIRE CLICK (the ClickMenuButton packet the spell lane the echo wears — the live window's option row the server's own verdict answers).
"""

MARK = "--- Task ID: autopush-418 "

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

n_auth = check_auth.count("\n--- Task ID: autopush-")
n_mirror = check_mirror.count("\n--- Task ID: autopush-")
assert n_auth == n_mirror == 98, f"the entry count the 98 wears ({n_auth}/{n_mirror})"
print(f"worklog_418: both files at {n_auth} entries, head/tail verified")

#!/usr/bin/env python3
"""autopush-422 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_422.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-422 (2026-11-02 16:06:46 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611021606) — v0.3.1214 THE BREWING STAND'S OWN WIRE TITLE
- Calibration: the COLD sandbox the round answers (the wipe ate the repo, the cargo, the AUTH worklog and the assets-cache — the clone + the rustup install + the one-command recovery the walk re-laid): the repo the token URL clones, the cargo 1.99.1 the rustup lane installs, scripts/recover_runtime_cache.py the 26.2 jar + the official store re-fetches [the jar 39193383 B sha1-verified, 10968 files the jar layout keeps, 3841 the flat copy answers, 7 the real panorama pixels, 1968 sounds.json events, 2147 wanted oggs fetched 9 not-in-index] + `mc2rs-gen cache` the font.bin + textures.bin re-mints, the AUTH worklog the MIRROR's own 101 entries the cp restores (the md5 the identity wears). HEAD 4b3150d4 clean + origin-synced (the 421 round's own mirror commit the head wears); the worklog's 101 autopush entries the count keeps (autopush-421 the last), the wheel the autopush-422 rotates. Chosen step: THE BREWING STAND'S OWN WIRE TITLE (the 421 NEXT list's own second walk the round answers — the render's own title read the furnace's own law wears: the live window the decode name the canvas banks, the None the kind's own lang row the offline canvas keeps).
- THE WALK (the title read the live gate the identity keeps): the render the brewing_is_live gate the furnace's own shape re-wears [the live window the live_brewing_title clone the or_else chain speaks — the None the container.brewing lang row the fallback answers, the offline canvas the same tr() read the bytes never move], the decode the 421 route's own bank the field already wears [the OpenScreen word's Component the text/translate rows the decode walk answers — the custom name the anvil rename's own bytes speak, the translatable the lang table answers, the empty bytes the None keeps], the close the retire the 421 walk already banks [the title dies with the window — the field the flag the same retire takes].
- TESTS: part_11.rs NEW ONE (the_brewing_stand_the_wire_title_wears — the arms wear: the wire's own text row [the custom name "Witch's Stand" the decode banks — the live flag the route wears], the render's own read [the 320x180 the real render the frame speaks], the translatable row [the container.brewing the lang table answers — the "Brewing Stand" the decode resolves], the empty bytes [the None the decode keeps — the fallback the lang row answers], the fallback's own identity [the None frame the translatable frame the SAME bytes wear — the whole canvas the identity keeps], the custom's own band [the title band the ONLY delta the frame paints — the band the custom name moves, the rest the bytes keep]). Verdict: green (the live window the decode name the canvas wears — the offline canvas the lang row keeps). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1627/1435 passed, each rc=0, 0 failed — the counts the 421 baseline the w11 the title lane's own walk +1 keeps, the other windows the 421 numbers keep) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (62 passed, 0 failed, scripts/directed_422.py the 66 filters the title lane's own head the part_11 gate rides) + the workspace's own full link (the --no-run walk the evidence chain keeps). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the lane the render's own read the live gate the only branch moves, the golden scenes' brewing canvases the offline stand-in the flag never wears). FMT: the region law (scripts/fmt_probe_422.py — the render's own title read + the round's own test, ALL-REGIONS-CLEAN). Bump 0.3.1213 -> 0.3.1214 (bump_version.py the workspace version + lock wear). Commit bf873796 pushed (4b3150d4..bf873796 main; the message file the untracked lane keeps — the retire tail the round never owes, the 420 law the cleaner walk answers).
- NEXT CANDIDATES: THE ENCHANT GATHER'S OWN WIRE PLACE-BACK (the release's own swing tail the wire lane may wear — the carried the gather left the press-back the same echo answers), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears), THE BREWING STAND'S OWN SLOT WALK (the ingredient/fuel/potion seats the SWAP input the wire may wear — the middle-press the pair the same ContainerClick echo answers), THE HOPPER'S OWN WIRE TITLE (the live hopper's own name the decode banks — the render's own title read the chest split the same live gate wears).
"""

MARK = "--- Task ID: autopush-422 "

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
assert n_auth == n_mirror == 102, f"the entry count the 102 wears ({n_auth}/{n_mirror})"
print(f"worklog_422: both files at {n_auth} entries, head/tail verified")

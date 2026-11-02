#!/usr/bin/env python3
"""autopush-423 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_423.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-423 (2026-11-02 18:21:50 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611021821) — v0.3.1216 THE HOPPER'S OWN WIRE TITLE
- Calibration: the WARM sandbox the round answers (the repo the HEAD keeps, the cargo 1.99.1 the PATH wears, the AUTH worklog the 102 entries the count keeps): the tree the IN-FLIGHT dev-215 walk the previous session kept [the budget the entry never wrote — the screens.rs the hopper title read the bump 0.3.1216 the version wears, the part_11 the round's own test the 125 lines the EOF banks, the mk_dev215 + directed_dev215 + fmt_probe_dev215 the round's own tools the scripts lane wears, the worklog the entry the round owes]. HEAD b53397a3 clean + origin-synced (the dev-214 round's own mirror commit the head wears); the worklog's 102 autopush entries the count keeps (autopush-422 the last), the wheel the autopush-423 rotates. Chosen step: THE HOPPER'S OWN WIRE TITLE — the in-flight dev-215 walk the round VERIFIES and LANDS (the dev-214 NEXT list's own third walk the round answers — the render's own title read the chest's own law wears: the live window the decode name the canvas banks, the None the kind's own lang row the offline canvas keeps).
- THE WALK (the title read the live gate the chest split re-wears): the hopper's own title read the chest_is_live gate the chest's own law wears [the live window the live_chest_title clone the or_else chain speaks — the None the container.hopper lang row the fallback answers, the offline canvas the same tr() read the bytes never move], the decode the chest route's own bank the field already wears [the OpenScreen word's Component the text/translate rows the decode walk answers — the custom name the anvil rename's own bytes speak, the translatable the lang table answers, the empty bytes the None keeps], the close the retire the chest walk already banks [the title dies with the window — the field the flag the same retire takes].
- TESTS: part_11.rs NEW ONE (the_hopper_the_wire_title_wears — the arms wear: the wire's own text row [the custom name "Grain Silo" the decode banks — the live flag the hopper canvas the route wears], the render's own read [the 320x180 the real render the frame speaks], the translatable row [the container.hopper the lang table answers — the "Item Hopper" the decode resolves], the empty bytes [the None the decode keeps — the fallback the lang row answers], the fallback's own identity [the None frame the translatable frame the SAME bytes wear — the whole canvas the identity keeps], the custom's own band [the title band the ONLY delta the frame paints — the band the custom name moves, the rest the bytes keep]). Verdict: green (the live window the decode name the canvas wears — the offline canvas the lang row keeps). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1629/1435 passed, each rc=0, 0 failed — the counts the dev-214 baseline the w11 the hopper title's own walk +1 keeps, the other windows the dev-214 numbers keep) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (64 passed, 0 failed, scripts/directed_dev215.py the 68 filters the hopper title's own head the part_11 gate rides) + the workspace's own full link (the --no-run walk the evidence chain keeps). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the lane the render's own read the live gate the only branch moves, the golden scenes' chest/hopper canvases the offline stand-in the flag never wears). FMT: the region law (scripts/fmt_probe_dev215.py — the hopper's own title read + the round's own test, ALL-REGIONS-CLEAN). Bump 0.3.1215 -> 0.3.1216 (the in-flight walk the bump already wore, the lock the six members the same version keeps). Commit 90226722 pushed (b53397a3..90226722 main; the message file the untracked lane keeps — the retire tail the round never owes).
- NEXT CANDIDATES: THE ENCHANT GATHER'S OWN WIRE PLACE-BACK (the release's own swing tail the wire lane may wear — the carried the gather left the press-back the same echo answers), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears), THE BREWING STAND'S OWN WIRE THROW (the outside press the THROW input the live lane may speak — the -999 walk the furnace's own shape the prediction-free law re-wears), THE BREWING STAND'S OWN SLOT WALK (the ingredient/fuel/potion seats the SWAP input the wire may wear — the middle-press the pair the same ContainerClick echo answers).
"""

MARK = "--- Task ID: autopush-423 "

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
assert n_auth == n_mirror == 103, f"the entry count the 103 wears ({n_auth}/{n_mirror})"
print(f"worklog_423: both files at {n_auth} entries, head/tail verified")

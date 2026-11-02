#!/usr/bin/env python3
"""autopush-421 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_421.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-421 (2026-11-02 12:51:42 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611021252) — v0.3.1213 THE BREWING STAND'S OWN WIRE LANE
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the rustfmt + the assets-cache the 420 round's lanes wear — no reset the walk re-installed); HEAD fced973e clean + origin-synced (the 420 round's own mirror commit the head wears); the worklog's 100 autopush entries the count keeps (autopush-420 the last), the wheel the autopush-421 rotates. Chosen step: THE BREWING STAND'S OWN WIRE LANE (the 420 NEXT list's own first walk the round answers — the menu type 11 the OpenScreen word routes, the honest drop the round retires: the stand's own five the wire banks, the rows the mirror feeds, the presses the prediction-free echo speaks, the close the retire walks).
- THE WALK (the stand lane the live gate the identity keys): the constant the clinit order banks [the MENU_TYPE_BREWING_STAND 11 the neighbor doc's own walk keeps], the menu's own numbering the class's own constants the javap brewmenu walk banks [BOTTLE_SLOT_START 0 / BOTTLE_SLOT_END 2 / INGREDIENT_SLOT 3 / FUEL_SLOT 4 — the SAME order the offline store's five keeps, the identity map the lanes never re-order; INV_SLOT_START 5 / USE_ROW_SLOT_END 41 the standard 36 the mirror feeds, the DATA_COUNT 2 the brewTime/fuel word pair the lane banks — the sim's own third word the start-ingredient the wire never carries], the projection the wholesale law wears [the five the wire's own (0,0) emptiness rides, the mirror the main 27 + the hotbar 9 the wire rows feed, the kinds the honest NONE wall keeps — the wire the kind word never carries, the sim's own walk the pos None retires], the presses the prediction-free echo speaks [the seat press the ContainerClick rides — the Pickup input the left press wears, the same Pickup the button 1's split halves speak, the QuickMove the shift press's own precedence wears, the empty delta list the press never mints, the believed cursor the packet carries, the corrections the wholesale lane re-banks; the outside press the THROW guard keeps silent, the no-session/no-window gates the command dies the frame thread keeps], the close the retire walks [the serverbound Close the sink echoes, the flag retires, the offline store the live window never serves — the flush the open_cell's None skips], the drag lane the honest silence keeps [the qc_arm gate the enchant's own law re-wears — the QUICK_CRAFT echo the later round owes].
- TESTS: part_11.rs NEW ONE (the_brewing_stand_the_wire_lane_wears — the arms wear: the route feed [the menu type 11 the window opens — the five the identity map banks, the mirror's head the wire's slot 5 wears, the tail the slot 32 answers, the words the pair banks, the NONE wall + the offline cell's retire + the open walk the lane speaks], the press's own echo [the bottle seat the left press takes — window 4 state 9 slot 0, the Pickup input the empty deltas ride, the seats + cursor the press never mints], the split's own echo [the ingredient seat the right press halves — button 1 the same Pickup wears], the shift's own echo [the fuel seat the QuickMove speaks], the outside's own silence, the no-session lane's own silence, the no-window lane's own silence, the close's own retire [the Close the window's own id wears, the flag + the identity + the name the retire takes], the server's own correction [the consumed ingredient + the ticked words the wholesale re-banks, the state id 10 the window re-keys, the reopen the lane speaks]). Verdict: green (the stand's window the wire the round banks — the five the identity map the lanes never re-order, the presses the echo speaks, the corrections the wholesale re-banks). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1626/1435 passed, each rc=0, 0 failed — the counts the 420 baseline the w11 the stand lane's own walk +1 keeps, the other windows the 420 numbers keep) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (61 passed, 0 failed, scripts/directed_421.py the 65 filters the stand lane's own head the part_11 gate rides) + the workspace's own full link (the --no-run walk the evidence chain keeps). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the lane the packet lane's own wire voice, the golden scenes' brewing canvases the offline stand-in the flag never wears, no render walk the round edits). FMT: the region law (scripts/fmt_probe_421.py — the close's own retire + the click's own wire echo + the stand's own wire lane + the round's own test, ALL-REGIONS-CLEAN — the whole-file rustfmt the pre-existing drift the region splice answers, the 421 region alone the formatted text re-forms). Bump 0.3.1212 -> 0.3.1213 (bump_version.py the workspace version + lock wear). Commit 51323e8a pushed (fced973e..51323e8a main; the message file the untracked lane keeps — the retire tail the round never owes, the 420 law the cleaner walk answers).
- NEXT CANDIDATES: THE BREWING STAND'S OWN SLOT WALK (the ingredient/fuel/potion seats the container walk the wire may wear — the live window's seat press the same ContainerClick echo answers, the local-belief walk the furnace's own shape keeps), THE BREWING STAND'S OWN WIRE TITLE (the live_brewing_title the decode banks — the render's own title read the offline canvas the golden law keeps), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears), THE ENCHANT GATHER'S OWN WIRE PLACE-BACK (the release's own swing tail the wire lane may wear — the carried the gather left the press-back the same echo answers).
"""

MARK = "--- Task ID: autopush-421 "

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
assert n_auth == n_mirror == 101, f"the entry count the 101 wears ({n_auth}/{n_mirror})"
print(f"worklog_421: both files at {n_auth} entries, head/tail verified")

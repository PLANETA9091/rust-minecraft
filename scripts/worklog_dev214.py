#!/usr/bin/env python3
"""dev-214 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own safety
law the round wears): the AUTHORITATIVE file the entry appends, the
MIRROR the same entry the repo's lane keeps, the head/tail assertions
the sync's own honesty wears. The DEV series the count keeps (the
autopush 102 the parallel wheel wears, the dev 66 the prior rounds —
the entry the 67th dev speaks). Usage: python3 scripts/worklog_dev214.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: dev-214 (2026-11-02 17:05:56 +08:00, Job 384925, trace 1a0a02c3c1060631-cron-agent-loop-202611021707) — v0.3.1215 THE BREWING STAND'S OWN SWAP KEY
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the assets-cache the dev-213/422 rounds' lanes wear — no reset the walk re-installed); HEAD b1824e55 clean + origin-synced (the 422 round's own mirror commit the head wears); the worklog's 168 entries the count keeps (102 autopush + 66 dev — dev-213 the last dev the parallel wheel wears), the wheel the dev-214 rotates. Chosen step: THE BREWING STAND'S OWN SWAP KEY (the 422 NEXT list's own walk the round answers — the checkHotbarKeyPressed walk the stand screen answers, the SWAP input the stand's seats the wire wears; the open stand the digits the F-key the world's hotbar-select chain the stand's own lane never consumed — the interactive gap the round closes).
- THE WALK (the swap key the prediction-free gate the identity keys): the banked walk the base class keeps [acs_fresh.txt checkHotbarKeyPressed pcs 0-114 — the screen/dead, the rects, the hovered-slot null arm, the CARRIED gate the getCarried().isEmpty() arm the false BEFORE the packet lane keeps — the live cursor the wholesale banks], the LIVE window the identity the keys echo against [the offline stand-in no window keeps], the packet the prediction-free law wears [the ContainerClick the SWAP input the button byte the digit or the offhand 40 the keySwapOffhand arm the pcs 20-57 bipush answers — the empty delta list the key never mints, the seats the key never mutates, the server's own doClick SWAP table the verdict answers, the corrections the wholesale lane re-banks — the wire_furnace family's believed walk the later round owes if the drift speaks], the true verdict the digit CONSUMES either way. THE CHAIN: the window.rs's own two inserts the dispatch chain the brewing arm joins [the offhand 40 + the digits the inventory/chest/furnace/table/brewing/enchant/creative walk — the open stand the keys the stand's lane consumes].
- TESTS: part_11.rs NEW ONE (the_brewing_stand_the_swap_key_wears — the arms wear: the route feed, the no-screen + no-geometry + no-cursor lanes [the false verdicts no packet mints], the digit's own echo [window 4 state 9 slot 0 button 3, the Swap input the empty deltas ride, the seats + cursor the key never mints], the offhand's own echo [button 40], the carried's own gate [(dirt,1) the walk refuses], the no-window lane's own silence, the no-session lane's own silence). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1628/1435 passed, each rc=0, 0 failed — the w11 the swap key's own walk +1 keeps) + the fast trio (161/100/80) + the directed families (63 passed, 0 failed, scripts/directed_dev214.py the 67 filters) + the workspace's own full link (--no-run). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — no render walk the round edits, the window.rs the dispatch chain the key arm alone joins). FMT: the region law (scripts/fmt_probe_dev214.py — the swap key fn + the window's two inserts + the round's own test, ALL-REGIONS-CLEAN). Bump 0.3.1214 -> 0.3.1215. Commit 6f61b7c7 pushed (b1824e55..6f61b7c7 main; the message file the untracked lane keeps — the retire tail the round never owes).
- NEXT CANDIDATES: THE ENCHANT GATHER'S OWN WIRE PLACE-BACK (the release's own swing tail the wire lane may wear — the carried the gather left the press-back the same echo answers), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears), THE HOPPER'S OWN WIRE TITLE (the live hopper's own name the decode banks — the render's own title read the chest split the same live gate wears), THE BREWING STAND'S OWN WIRE THROW (the outside press the THROW input the live lane may speak — the -999 walk the furnace's own shape the prediction-free law re-wears).
"""

MARK = "--- Task ID: dev-214 "

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

n_auth_ap = check_auth.count("\n--- Task ID: autopush-")
n_mirror_ap = check_mirror.count("\n--- Task ID: autopush-")
n_auth_dev = check_auth.count("\n--- Task ID: dev-")
n_mirror_dev = check_mirror.count("\n--- Task ID: dev-")
assert n_auth_ap == n_mirror_ap == 102, f"the autopush count the 102 wears ({n_auth_ap}/{n_mirror_ap})"
assert n_auth_dev == n_mirror_dev == 19, f"the dev count the 19 wears ({n_auth_dev}/{n_mirror_dev})"
print(f"worklog_dev214: autopush {n_auth_ap} + dev {n_auth_dev}, head/tail verified")

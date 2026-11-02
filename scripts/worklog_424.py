#!/usr/bin/env python3
"""autopush-424 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_424.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-424 (2026-11-02 20:06:51 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611022006) — v0.3.1217 THE BREWING STAND'S OWN WIRE THROW
- Calibration: the COLD sandbox the round answers (the wipe ate the repo, the cargo, the AUTH worklog AND the in-flight dev-216 walk's own uncommitted edit — the interrupted 19:06 session the THROW lane the screens.rs wore, the round the walk RE-WEARS from the 423 head): the repo the token URL clones (HEAD 87e6fc9e clean + origin-synced, the 423 round's own mirror commit the head wears), the cargo 1.99.1 the rustup lane installs, scripts/recover_runtime_cache.py the 26.2 jar + the official store re-fetches [the jar 39193383 B sha1-verified, 10968 files the jar layout keeps, 3841 the flat copy answers, 7 the real panorama pixels, 1968 sounds.json events, 2147 wanted oggs fetched 9 not-in-index] + `mc2rs-gen cache` the font.bin + textures.bin re-mints, verify_goldens 19/19 the recovery's own verdict wears, the AUTH worklog the MIRROR's own 103+19 entries the cp restores (the md5 the identity wears). Chosen step: THE BREWING STAND'S OWN WIRE THROW (the 423 NEXT list's own third walk the round answers — the outside press the live window the THROW input speaks, the -999 the table's own wire shape re-wears).
- THE WALK (the live lane's own throw the table's own shape re-wears): the outside press the strict rect compare gates [the panel the last render laid, the between-slots press the rect compare refuses — the silence the seats keep], the carried stack rides the packet [the whole stack the LEFT press drops, the one item the RIGHT splits, the belief clears or splits — the live_brewing_carried cursor the walk's own hands move], the drop entity the SERVER spawns [the wire's own Add Entity lane — the local spawn never rides here], the EMPTY cursor's press stays silent [the doClick THROW arm's own isEmpty guard — no packet the press mints], the no-window + no-session lanes the guard returns keep [the identity the retire took, the command the frame thread never serves — the belief the lanes never move].
- TESTS: part_11.rs NEW ONE (the_brewing_stand_the_wire_throw_wears — the arms wear: the left's own whole drop [slot -999 the Throw input the packet rides, the belief clears], the right's own split [the one item the packet drops, the remainder the belief keeps], the empty cursor's own silence, the between-slots lane's own silence [the rect compare the throw refuses — the cursor the press never moves], the no-window lane's own silence, the no-session lane's own silence [the belief never moves], the seats the throw never mints). Verdict: green (the live window the outside press the THROW speaks — the offline canvas the local spawn keeps). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1630/1435 passed, each rc=0, 0 failed — the counts the 423 baseline the w11 the throw's own walk +1 keeps, the other windows the 423 numbers keep) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (65 passed, 0 failed, scripts/directed_424.py the 69 filters the throw's own head the part_11 gate rides) + the workspace's own full link (the --no-run walk the evidence chain keeps). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the lane the click dispatch's own wire arm the only branch moves, the render the round never edits, the golden scenes the bytes never see). FMT: the region law (scripts/fmt_probe_424.py — the live lane's own throw + the round's own test, ALL-REGIONS-CLEAN). Bump 0.3.1216 -> 0.3.1217 (bump_version.py the workspace version + lock wear, the six members the same version keeps). Commit ff4b0b15 pushed (87e6fc9e..ff4b0b15 main; the message file the untracked lane keeps — the retire tail the round never owes).
- NEXT CANDIDATES: THE HOPPER'S OWN WIRE THROW (the live hopper's outside press the same -999 lane may speak — the chest split's own silence the same walk answers), THE FURNACE'S OWN WIRE THROW (the live furnace's outside press the table's own shape may wear — the smeltery's own silence the same packet answers), THE ENCHANT GATHER'S OWN WIRE PLACE-BACK (the release's own swing tail the wire lane may wear — the carried the gather left the press-back the same echo answers), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears).
"""

MARK = "--- Task ID: autopush-424 "

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
assert n_auth == n_mirror == 104, f"the entry count the 104 wears ({n_auth}/{n_mirror})"
print(f"worklog_424: both files at {n_auth} entries, head/tail verified")

#!/usr/bin/env python3
"""autopush-420 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_420.py"""

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-420 (2026-11-02 12:21:42 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611021221) — v0.3.1212 THE ENCHANT OPTION'S OWN WIRE CLICK
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the rustfmt + the assets-cache the 419 round's lanes wear — no reset the walk re-installed); HEAD 8c3f0668 + origin-synced (the 419 round's own mirror commit the head wears); the worklog's 99 autopush entries the count keeps (autopush-419 the last), the wheel the autopush-420 rotates. THE TREE THE WALK INHERITS: the working tree the 420 lane's own edits the prior cron session's walk left (the v0.3.1212 bump + the option lane + the button packet + the tests + the four scripts the tree wore, uncommitted), the round the verification re-runs the green the round banks (the whole chain the round's own walk answers — no step the round re-implements). Chosen step: THE ENCHANT OPTION'S OWN WIRE CLICK (the 419 NEXT list's own first walk the round answers — the honest silence the 417 arm kept the round retires: the live window's option row the ServerboundContainerButtonClickPacket speaks, the server's own clickMenuButton the verdict answers, the press the client's belief never mints).
- THE WALK (the option lane the live gate the wire wears): the honest-silence head the round retires [the enchant_is_live gate the spell lane the wire round banks — the no-session/no-window gates the command dies the frame thread keeps], the option's own echo the button packet wears [the live window's row press the MenuButton command rides — the window id the container field wears, the row id the button field speaks, the two VarInts the RECORD codec's own order keeps (container FIRST, button LAST — the handleInventoryButtonClick ctor's (II) the checked-in disasm banks, the id 17 minecraft:container_button_click the generated table banks), the wire the session's own click_container_button sends], the prediction-free lane the pickup's echo wears [the local fiction the press never mints — the xp spend + the lapis consume + the transmute the server's own clickMenuButton answers, the corrections the container lane re-banks (the seats the wholesale replacement banks, the state id the window re-keys)], the id bound + the offline sim the own gates keep [the costs.length bound the false keeps, the fresh-zero costs the offline refuse speaks — no packet the lane answers].
- TESTS: part_11.rs NEW ONE (the_enchant_option_the_wire_click_wears — the arms wear: the route feed [the menu type 13 the window opens, the identity the echo answers], the press's own echo [the row 0 the MenuButton rides — window 4 button 0, the seats + xp + cursor the press never mints], the rows' own echo [the ids 1 + 2 the same packet law wears], the id bound's own gate [the row 3 no command speaks], the no-session lane's own silence, the no-window lane's own silence, the offline sim's own refuse [the fresh-zero costs the false keeps], the server's own correction [the enchanted_book transmute + the lapis consume the sync lands, the state id 10 the window re-keys]) + mc-protocol NEW ONE (the_container_button_click_lane_the_option_word_wears — the id 17 the three fixtures wear: 11 04 02 the doc's own walk, 11 04 00 the first row, the window 11 the multi-byte container VarInt's own shape). Verdict: green (the option press the wire the round banks — the packet the two VarInts the lane speaks, the fiction the press never mints, the corrections the container lane re-banks). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1625/1435 passed, each rc=0, 0 failed — the counts the 419 baseline the w11 the option click's own walk +1 keeps, the other windows the 419 numbers keep) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep — the protocol bank the button lane's own test +1 wears) + the directed families (60 passed, 0 failed, scripts/directed_420.py the 64 filters the option click's own head the part_11 gate rides) + the workspace's own full link (the --no-run walk the evidence chain keeps). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the click the packet lane's own wire voice, the golden scenes' enchant canvases the offline stand-in the flag never wears, no render walk the round edits). FMT: the region law (scripts/fmt_probe_420.py — the option lane's own walk + the round's own test, ALL-REGIONS-CLEAN first run). Bump 0.3.1211 -> 0.3.1212 (bump_version.py the workspace version + lock wear). Commit 762a0c1c pushed (8c3f0668..762a0c1c main; the message file the untracked lane keeps — the retire tail the round never owes, the 412 law the cleaner walk answers).
- NEXT CANDIDATES: THE BREWING STAND'S OWN WIRE LANE (the menu type 11 the clinit order the constants' docs bank — the OpenScreen word the honest drop keeps), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears), THE ENCHANT GATHER'S OWN WIRE PLACE-BACK (the release's own swing tail the wire lane may wear — the carried the gather left the press-back the same echo answers), THE BREWING STAND'S OWN SLOT WALK (the ingredient/fuel/potion seats the container walk the wire may wear — the live window's seat press the same ContainerClick echo answers).
"""

MARK = "--- Task ID: autopush-420 "

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
assert n_auth == n_mirror == 100, f"the entry count the 100 wears ({n_auth}/{n_mirror})"
print(f"worklog_420: both files at {n_auth} entries, head/tail verified")

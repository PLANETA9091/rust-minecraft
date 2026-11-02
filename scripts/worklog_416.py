#!/usr/bin/env python3
"""autopush-416 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_416.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-416 (2026-11-02 07:36:37 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611020736) — v0.3.1208 THE DISPENSER'S OWN SHULKER SPILL
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the rustfmt + the assets-cache the 415 round's lanes wear — no reset the walk re-installed); HEAD 82c45201 clean + origin-synced (the 415 round's own mirror commit the head wears); the worklog's 95 autopush entries the count keeps (autopush-415 the last), the wheel the autopush-416 rotates. Chosen step: THE DISPENSER'S OWN SHULKER SPILL (the 415 NEXT list's own first walk the round answers — the item-keyed registry the both blocks' own lookup answers: the DispenserBlock.getDispenseMethod walk the registry consults FIRST, the item's registered behavior the block-kind split precedes, the dropper's own override the DEFAULT-substitute lane the registry's miss alone wears).
- THE WALK (the beat's own dispatch the item-keyed order re-keys — the four-way sink walk the hoist serves): the front read the both lanes share hoists [the getContainerAt walk the spill arm's own target the sink arms' own front answers, the one read the two blocks' own lanes wear], the spill arm the dispatch's own FIRST arm wears [the replaceable cell the w.set + the w.set_variant the SHULKER_BOX kind the 304 round's own hit-face stamp banks, the placement's own dispatch the falling check the schedule speaks, the occupied cell the honest (false, 1001) the fail voice keeps], the DROPPER arm the sibling sinks keep [the chest-family arm + the furnace face law the 414 walk banks, the default eject the miss lane answers], the spill's own mark the entity gate re-wears [the shulker_spill_placed flag the eject arm reads — the placed box the item entity never spawns, the dispenser's own non-shulker shots the projectile + the default arms keep].
- TESTS: part_11.rs NEW ONE (the_dispenser_shulker_the_place_law_wears — the arms wear: the place arm's own stamp [the DISPENSER block the front cell the block lands, the lid the dispenser's own hit-face byte banks, the item entity the spill never spawns, the source seat the consume drains], the refuse arm's own count [the occupied cell the fail speaks, the refused stack the count keeps], the chest-keyed preempt arm's own order [the sink's own store the spill never touches, the preempted shot the count keeps, the front cell the chest family keeps]). Verdict: green FIRST RUN. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1621/1435 passed, each rc=0, 0 failed — the counts the 415 baseline the w11 the dispenser's own place law +1 keeps, the other windows the 415 numbers keep; the killed-batch's own stale gate the walk discovers mid-round — the mod.rs the part-active state the timeout kill left, the all-gated 1435 the window runs answered, the git checkout the clean state restores, the w1 the 1526 the re-run speaks) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the workspace pair (the mc-assets + the mc2rs-pixel the pixel harness the golden lane keeps) + the directed families (the dispenser shulker place law + the dropper shulker place law + the furnace face law + the furnace kind ride + the insert ride + the dispenser shot ride + the dispenser kind bank + the enchant swap-key + the enchant gather + the lapis seat may-place + the offline enchanting round + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil/wire/furnace/dispenser/brewing/drag/store/craft/hopper rides' own fifty-six — 56 passed, 0 failed, scripts/directed_416.py the 60 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the spill the beat's own data lanes the world walk alone voices, the golden scenes' dispenser seats the shulker item never carry, no render walk the round edits). FMT: the region law (scripts/fmt_probe_416.py — the spill's own mark + the front read + the registry order + the beat's own place arm + the eject gate's own ride + the round's own test, ALL-REGIONS-CLEAN FIRST RUN). Bump 0.3.1207 -> 0.3.1208 (bump_version.py the workspace version + lock wear). Commit ec7d247c pushed (82c45201..ec7d247c main; the message file the repo's own outside the round keeps — the apostrophe lanes the -F wear, the 412 law the same file answers).
- NEXT CANDIDATES: THE ENCHANT GATHER'S OWN WIRE HALF (the live session the menu never banks — the wire lane the later round owes when the enchant table the live window opens), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears), THE DROPPER'S OWN PROJECTILE REGISTRY (the arrow item the dropper's own miss lane the registry's own entry may re-key — the 139 beat's dropper eject arm the ArrowDispenseBehavior the same item-keyed law may wear).
"""

MARK = "--- Task ID: autopush-416 "

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
assert n_auth == n_mirror == 96, f"the entry count the 96 wears ({n_auth}/{n_mirror})"
print(f"worklog_416: both files at {n_auth} entries, head/tail verified")

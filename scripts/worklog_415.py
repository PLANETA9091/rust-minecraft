#!/usr/bin/env python3
"""autopush-415 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_415.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-415 (2026-11-02 06:36:36 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611020638) — v0.3.1207 THE DROPPER'S OWN SHULKER SPILL
- Calibration: the WARM sandbox the round keeps (the repo + the cargo 1.99.1 + the rustfmt + the assets-cache the 414 round's lanes wear — no reset the walk re-installed); HEAD 0711d7d5 clean + origin-synced (the 414 round's own mirror commit the head wears); the worklog's 94 autopush entries the count keeps (autopush-414 the last), the wheel the autopush-415 rotates. Chosen step: THE DROPPER'S OWN SHULKER SPILL (the 414 NEXT list's own first walk the round answers — javap ShulkerBoxDispenseBehavior.execute the 26.3 jar the live source answers: the shulker box item the front cell the BLOCK place walk speaks, the item-keyed behavior the container-insert default never reaches, the OptionalDispenseItemBehavior's own playSound the success the 1000 voice the fail the 1001 keeps, the setSuccess(false) head the refused walk banks).
- THE WALK (the beat's own arm the item-keyed registry order wears — the four-way sink walk the spill preempts): the fn is_shulker_box_item the item's own kind the placeable walk resolves [the item's name the World::placeable_block lane answers, the resolved kind the shulker family the one predicate speaks, the colored rows the same family's own names answer], the beat's DROPPER arm the spill law the front cell wears [the replaceable cell the w.set + the w.set_variant the SHULKER_BOX kind the 304 round's own hit-face stamp banks — setValue(FACING, context.getClickedFace()) the DirectionalPlaceContext's own hit face the facing param answers, the placed lid the dispenser's own byte the shulker_variant's low three bits compose, the below-empty clickedFace param the context's look law carries the shulker's own stamp the 26.2 jar never reads; the placement's own dispatch the falling check the schedule speaks — the player placement walk's own law the beat's arm mirrors; the occupied cell the honest (false, 1001) the fail voice keeps], the spill's own mark the entity gate re-wears [the shulker_spill_placed flag the eject arm reads — the placed box the item entity never spawns, the default shot's own spawn the other arms keep], the fresh store the open door shares [the chest family's own lazy law — the open the fresh 27-slot grid inserts, the 412 insert lane's own entry arm the same map banks, the placed shulker the NBT-less item the honest empty wears]. THE 414 CANDIDATE'S own walk the registry order settles (the shulker item at the chest sink the insert walk never reaches — the item-keyed behavior the registry lookup answers first, the vanilla dropper's own law the chest-keyed preempt arm speaks).
- TESTS: part_11.rs NEW ONE (the_dropper_shulker_the_place_law_wears — the arms wear: the place arm's own stamp [the front cell the block lands, the lid the dispenser's own hit-face byte banks, the item entity the spill never spawns, the source seat the consume drains], the open door's own share [the placed cell the fresh 27-slot store the same map banks, the kind row the title lane banks], the refuse arm's own count [the occupied cell the fail speaks, the refused stack the count keeps], the chest-keyed preempt arm's own order [the sink's own store the spill never touches, the preempted shot the count keeps, the front cell the chest family keeps]). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1620/1435 passed, each rc=0, 0 failed — the counts the 414 baseline the w11 the spill's own place law +1 keeps, the other windows the 414 numbers keep; the w11 the FINAL-code walk the fmt tail re-wears; the killed-batch's own stale gate the walk discovers mid-round — the mod.rs the part_6-active state the timeout kill left, the all-gated 1435 the window-1 run answered, the git checkout the clean state restores, the w1 the 1526 the re-run speaks) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the shulker place law + the furnace face law + the furnace kind ride + the insert ride + the dispenser shot ride + the dispenser kind bank + the enchant swap-key + the enchant gather + the lapis seat may-place + the offline enchanting round + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil/wire/furnace/dispenser/brewing/drag/store/craft/hopper rides' own fifty-five — 55 passed, 0 failed, scripts/directed_415.py the 59 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the spill the beat's own data lanes the world walk alone voices, the golden scenes' dropper seats the shulker item never carry, no render walk the round edits). FMT: the region law (scripts/fmt_probe_415.py — the kind predicate + the spill's own mark + the beat's own place arm + the eject gate's own ride + the round's own test, ALL-REGIONS-CLEAN [the air-or-replaceable tail the rustfmt one-line merge wears, the kind rows' own 9 the split tail the rustfmt shape re-keys]). Bump 0.3.1206 -> 0.3.1207 (bump_version.py the workspace version + lock wear). Commit 7109927d pushed (0711d7d5..7109927d main; the message file the repo's own outside the round keeps — the apostrophe lanes the -F wear, the 412 law the same file answers).
- NEXT CANDIDATES: THE DISPENSER'S OWN SHULKER SPILL (the else arm's own lanes the item-keyed law may wear — the dispenser's shulker shot the same place walk answers, the 415's own DROPPER scope the sibling arm retires), THE ENCHANT GATHER'S OWN WIRE HALF (the live session the menu never banks — the wire lane the later round owes when the enchant table the live window opens), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears).
"""

MARK = "--- Task ID: autopush-415 "

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

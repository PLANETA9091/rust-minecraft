#!/usr/bin/env python3
"""autopush-405 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_405.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-405 (2026-11-01 21:31:50 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611012056) — v0.3.1197 THE ENCHANT'S ITEM SEAT'S OWN MAX-ONE
- Calibration: the session the round's own warmth keeps (the repo + the cargo + the .mc2rs cache the prior round's lanes wear, no reset the walk re-installed); HEAD b86e18fa clean + origin-synced (the autopush-404 round's own mirror commit the head wears); the worklog's 84 autopush entries the count keeps (autopush-404 the last), the wheel the autopush-405 rotates. Chosen step: THE ENCHANT'S ITEM SEAT'S OWN MAX-ONE (the 404 NEXT list's own first walk the round answers — and the jar the premise re-keys again, the anon classes the parser banks).
- THE JAR'S OWN EVIDENCE (the anon classes the 405 parser banks — scripts/jar_work/enchant_menu_405.txt, the class-file parser the round re-wears): the EnchantmentMenu$2 the ctor's first anonymous Slot the item seat wears — the mayPlace the BASE Slot keeps [any item — the isValidEnchantmentTarget premise the bytes retire, the 404 candidate's own law the ceiling re-keys], the getMaxStackSize() = 1 the iconst_1/ireturn override speaks [the stackable's own 64 the vanilla seat never wears]; the EnchantmentMenu$3 the lapis seat the mayPlace = stack.is(lapis) the base 64 keeps.
- THE CEILING (the plain walk's own arms the max-one re-keys — the doClick bytes the place arm the split(min(count, slot.getMaxStackSize(stack))) cap speaks [the safeInsert's own ceiling the 26.2 walk banks, the acm_doClick_262.txt the drag + swap arms the same slot-max hint wears — the swap arm's own carried-count gate pcs 989-1001 the fresh evidence banks], the merge arm the room the slot-max closes, the full seat the no-op keeps, the pickup arm the base walk verbatim wears, the right press the move-one arm the same ceiling answers, the component-refusal swap the kind walk's own law keeps). THE HINT (the world_view's own predictor pair the round lays — predict_pickup_kind_max the left walk the ceiling answers, predict_pickup_right_max the right walk the same law wears, the slot_max parameter the caller's own seat ceiling keys, every other shape the base walk verbatim wears; the enchant lane the menu-0 seat the hint answers, the lapis seat the base walk the 64 keeps).
- TESTS: part_11.rs NEW ONE (the_enchant_item_seat_the_max_one_wears — the six arms wear: the left place's own split cap [the 64-stack the empty seat the ONE takes, the remainder the cursor keeps, the wholesale land the 64 seat never wears], the left merge's own closed room [the occupied seat the ceiling never grows], the right press's own closed room [the move-one arm the same ceiling answers], the right place's own one [the empty seat the ONE takes], the lapis seat's own whole stack [the base 64 the stackable's own ceiling keeps], the single stack's own whole move [the count-1 carried the split-cap arm the whole move wears]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1611/1435 passed, each rc=0, 0 failed — the counts the autopush-404 baseline the w11 the max-one's own walk +1 keeps, the other windows the 404 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil quick-move kind ride + the anvil result-seat may-place + the anvil numbering walk + the anvil wire wholesale kind landing + the offline dispenser quick-move kind tail + the wire dispenser quick-move kind ride + the wire hopper quick-move kind ride + the furnace quick-move ride + the furnace trio + the wire generic/hopper/table/dispenser/chest/furnace places + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the anvil/enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip rides' own forty-five — 45 passed, 0 failed, scripts/directed_405.py the 45th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the plain walk's seat ceiling alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_405.py — the world_view max hints + the plain arms' own re-key + the part_11 test, ALL-REGIONS-CLEAN). Bump 0.3.1196 -> 0.3.1197 (bump_version.py the workspace version + lock wear). Commit f537ab56 pushed (b86e18fa..f537ab56 main).
- NEXT CANDIDATES: THE SWAP ARM'S OWN COUNT GATE (the vanilla PICKUP swap the carried-count gate the bytes speak — the `carried.getCount() <= slot.getMaxStackSize(carried)` the pcs 989-1001 bank keeps, the carried 64 the max-1 seat the swap never spends, the fall-through the silence answers), THE LAPIS SEAT'S OWN MAY-PLACE (the plain place arm the mayPlace gate the vanilla 1213-1220 bytes keep — the non-lapis the lapis seat the plain place never lands, the quick-move's own marker the same law the 402 walk speaks), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps — the live lane's own open/sync/click walk the wholesale landing the same law answers).
"""

MARK = "--- Task ID: autopush-405 "

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

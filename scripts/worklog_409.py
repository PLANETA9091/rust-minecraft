#!/usr/bin/env python3
"""autopush-409 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_409.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-409 (2026-11-02 01:36:30 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611020136) — v0.3.1201 THE SWAP-KEY'S OWN MAY-PLACE
- Calibration: the FRESH SANDBOX the reset the round re-wears (the repo + the authoritative worklog the reset lost — the clone the mirror re-lays, the 88 entries the mirror's own count keeps, the wheel the autopush-409 rotates [the 407 + the 408 rounds the parallel lane the reset between walked — the head 4d6b78e7 the 408's own mirror commit wears]); the cargo 1.99.1 the rustup re-install banks, the assets-cache the restore_env.sh lane re-lays [the sha1-honest jar 2dc72797 + the 5028-object index store + the 10967 jar textures BOTH layouts — the window-1's own 35 the colormap-dependent lanes the recovery greens, the biome/tint tests the fresh clone the empty cache reded]. Chosen step: THE SWAP-KEY'S OWN MAY-PLACE (the 408 NEXT list's own second walk the round answers — the enchant screen the number-key SWAP lane the whole build never wore).
- THE EVIDENCE (the raw walk the round re-reads — the acs_fresh.txt the checkHotbarKeyPressed the pcs 0-10 + 13-17 + 20-107 the disasm keeps): the getCarried().isEmpty() arm the walk opens with [the carried cursor the FALSE verdict keeps], the hoveredSlot null arm the same FALSE speaks, the keySwapOffhand match the slotClicked(hovered, index, 40, SWAP) call rides, the digits 0..8 the same slotClicked(hovered, index, digit, SWAP) walk serves; the doClick SWAP table the acm_doClick_262.txt the pcs 1130-1379 the rows keep [the 408 candidate's 1213-1220 bytes the CASE C mayPlace ifeq the arm's own gate speaks].
- THE WALK (the offline lane the enchant click family's own shape re-keys — the screens lane on_enchant_swap_key + enchant_swap_table the seats' own verdicts wear): the both-empty press the consumed silence answers [pcs 1130-1146], the same-seat press the identical pair sets back [the hovered hotbar row the button names — the table walk's own no-op the family keeps], CASE B pc 1149-1202 the row empty the hovered stack the row rides AS-IS [the onSwapCraft + setByPlayer(EMPTY) + onTake tail the bytes keep, the mayPickup the 408 census's base TRUE the whole 38 answers, the seat's comps store the ride wears — the mirror cell the menu button+29 the -29 mapping speaks], CASE C pc 1205-1273 the hovered empty the mayPlace gate pc 1213-1220 the row's stack places [the lapis seat the stack.is(lapis) verdict speaks — the non-lapis row the lapis seat never lands, the F-key the same refusal the digits keep; the SPLIT law the seat max wears — the item seat the getMaxStackSize()=1 override answers ONE, the lapis seat the base 64 the 70-row splits, the store rows the item's own law], CASE D pc 1276-1379 both filled the mayPickup && mayPlace pair the gates answer [the count > max arm the split + the ORIGINAL hovered stack the Inventory.add rides — the add the write-backs follow the vanilla order wears, the leftover the player.drop speaks; the count <= max arm the straight trade, the stores the exchange the ride law keeps], the every-mutation law the mirror re-syncs [the hotbar HUD the same walk answers]. THE LANES: the window.rs dispatch the F-key + the digits chains the enchant walk answers [the inventory/chest/furnace/table/creative family the enchant arm joins].
- TESTS: part_11.rs NEW ONE (the_enchant_seats_the_swap_wears — the arms wear: the carried gate's own false, the lapis seat's own may-place [the refusal + the accept + the 70-split], the item seat's own max-one split, CASE B the pickup ride [the store the ride wears, the seat's store the clear speaks], CASE D the gates' refusal + the straight trade + the split's own add ride [the remainder row the merge tops up], the both-empty silence, the same-seat no-op, the offhand's own arm [the F-key the refusal + the ride answers]). Verdict: green first walk (the one seat-coords walk the render's rects the scoped lookup keeps). THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1615/1435 passed, each rc=0, 0 failed — the counts the 408 baseline the w11 the swap's own walk +1 keeps, the other windows the 408 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the enchant swap-key + the enchant gather + the lapis seat may-place + the offline enchanting round + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil/wire/furnace/dispenser/brewing/drag/store/craft/hopper rides' own fifty — 50 passed, 0 failed, scripts/directed_409.py the 54 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the swap the key lane's own arms alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_409.py — the swap-key walk + the swap table + the dispatch arms + the round's own test, ALL-REGIONS-CLEAN [the rustfmt walk the add-call the one-line form the pre-break re-keyed, the test's own asserts the formatted tail wears]). Bump 0.3.1200 -> 0.3.1201 (bump_version.py the workspace version + lock wear). Commit b2f4ad7d pushed (4d6b78e7..b2f4ad7d main).
- NEXT CANDIDATES: THE ENCHANT GATHER'S OWN WIRE HALF (the live session the menu never banks — the wire lane the later round owes when the enchant table the live window opens), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps), THE SWAP'S OWN STORE RIDE (the real bank the close flush keeps — the offhand the comp-less seat the plain pair answers, the ride's own mirror law the real inventory_enchants the close wears).
"""

MARK = "--- Task ID: autopush-409 "

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

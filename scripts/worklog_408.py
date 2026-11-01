#!/usr/bin/env python3
"""autopush-408 — THE WORKLOG'S OWN DUAL WEAR (the 388 round's own
safety law the round wears): the AUTHORITATIVE file the entry appends,
the MIRROR the same entry the repo's lane keeps, the head/tail
assertions the sync's own honesty wears. Usage: python3
scripts/worklog_408.py"""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-408 (2026-11-02 00:21:29 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611020023) — v0.3.1200 THE ENCHANT SEAT'S OWN GATHER
- Calibration: the session the round's own warmth keeps (the repo + the cargo 1.99.1 + the rustfmt + the jar cache the 407 round's lanes wear, no reset the walk re-installed); HEAD 0e4e63e1 clean + origin-synced (the autopush-407 round's own mirror commit the head wears); the worklog's 87 autopush entries the count keeps (autopush-407 the last), the wheel the autopush-408 rotates. Chosen step: THE ENCHANT SEAT'S OWN GATHER (the 407 NEXT list's own first walk the round answers — the doClick PICKUP_ALL arm the enchant menu the offline lane answers).
- THE EVIDENCE (the raw walk the round re-reads — the acm_doClick_262.txt the pcs 1611-1873 the disasm keeps): the per-slot gate chain the hasItem -> canItemQuickReplace(slot, carried, TRUE) -> mayPickup -> canTakeItemForPickAll the pcs 1754-1791 speak, NO mayPlace the chain consults, NO paint arm the spread wears — the 407 NEXT list's own gather-gate premise the bytes retire [the candidate's "the non-lapis carried the lapis seat never paints" the paint arm the vanilla never had answers]; the gather the same-item stacks ALONE pulls, the lapis seat the same-item SOURCE the mayPickup/canTakeItemForPickAll base TRUE the pull answers (the EnchantmentMenu the 26.2 override census's base family wears — the merchant + the craft family alone refuse).
- THE WALK (the grid predictor the round wears): the world_view's predict_pickup_all_grid the generic slice the two-pass take answers [the partials the first pass drain in the walk order, the full stacks the same pass skip, the second pass the room the item's own max caps, the drained cell the (0, 0) pair speaks, the no-op gates the hovered-filled + the carried-empty + the out-of-range the None speak]; the screens lane the enchant menu's own 38 the flat projection feeds [the pair 0..2 the seats, the main rows 2..28 the items 9..35 the +7 mapping, the hotbar 29..37 the items 0..8 the -29 shift, the every-mutation law the mirror re-syncs]. THE LANES (the inventory gather's own shape the round re-wears): the doubleclick field the enchanting's own copy banks, the lastClickSlot the same pair keeps, the press walk the compute the dispatch BEFORE keeps [the doubled press the same seat the flag arms, the plain press the click keeps, the last-click seat the press remembers], the release walk the flag the gate opens [the canTakeItemForPickAll the base TRUE the whole 38 answers, the null-hover arm the flag keeps, the arm's own false the clear speaks], the close the pair the reset wears [the vanilla screen the removed() walk discards whole], the spread the release CONSUMES the qc clear the goto 452 law keeps.
- TESTS: part_11.rs NEW ONE (the_enchant_seats_the_gather_wears — the arms wear: the pure two-pass take [the partials drain first, the full the second pass answers, the different item the walk never names], the gates' own None [the hovered filled, the empty cursor, the out-of-range], the lapis seat's own source [the same-item stack the gather home pulls, the mayPlace the spread never consults], the release's own walk [the partial row the pull drains, the arm's own clear the release speaks], the null hover's own flag keep, the press's own arm [the doubled press the flag arms, the release the consumed walk speaks], the close's own reset). Verdict: green (the one mouse-pos seat the window-layer law the test re-keys — the press dispatch the coords alone reads, the release walk the stored pos the hover answers; the DBG probe the pairs the gates once named, the clean file the final walk wears).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1614/1435 passed, each rc=0, 0 failed — the counts the 407 baseline the w11 the gather's own walk +1 keeps, the other windows the 407 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the enchant gather + the lapis seat may-place + the offline enchanting round + the enchant swap arm count gate + the enchant item seat max-one + the enchant rows one-item seat + the enchant quick-move kind ride + the enchant pair may-place + the anvil/wire/furnace/dispenser/brewing/drag/store/craft/hopper rides' own forty-nine — 49 passed, 0 failed, scripts/directed_408.py the 53 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the gather the release lane's pull alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_408.py — the grid predictor + the field pair + the routing arm + the walk pair + the close reset + the press lane + the round's own test, ALL-REGIONS-CLEAN [the rustfmt walk the flag compute the one-line form the pre-break re-keyed]). Bump 0.3.1199 -> 0.3.1200 (bump_version.py the workspace version + lock wear). Commit a10222b2 pushed (0e4e63e1..a10222b2 main).
- NEXT CANDIDATES: THE ENCHANT GATHER'S OWN WIRE HALF (the live session the menu never banks — the wire lane the later round owes when the enchant table the live window opens), THE SWAP-KEY'S OWN MAY-PLACE (the ContainerInput.SWAP the 1213-1220 bytes the number-key walk wears — the F-key the non-lapis the lapis seat never lands, the enchant screen the SWAP lane the whole build answers), THE DISPENSER SEAT'S OWN KIND BANK (the nine the kindless store the honest NONE wall keeps — the block-entity store the words may carry, the chest's own container_row_kinds_load the same bank the store may wear), THE BREWING STAND'S OWN WIRE LANE (the menu type 5 the OpenScreen word the honest drop keeps).
"""

MARK = "--- Task ID: autopush-408 "

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

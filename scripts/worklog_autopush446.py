#!/usr/bin/env python3
"""autopush-446 — THE WORKLOG MIRROR SYNC (the one-shot lane the
round's own entry wears): the AUTH the round's own autopush-446
entry appends, the MIRROR the same bytes the copy answers, the
guards the double lane never wears. Usage: python3 scripts/worklog_autopush446.py"""
import hashlib
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: autopush-446"

ENTRY = """--- Task ID: autopush-446 (2026-11-04 01:58:40 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611040102) — v0.3.1240 THE FURNACE GATHER'S OWN DOUBLE-CLICK ARM
- Calibration: the WARM sandbox the round keeps (the e2a30c09 head the fetch the origin-synced clean tree verified — the 445 round's own mirror commit the head wears), the worklog's 125 autopush entries the count keeps — autopush-445 the last, the wheel the 446 rotates. Chosen step: THE FURNACE GATHER'S OWN DOUBLE-CLICK ARM — the 445 NEXT's own first lane the round answers (the dispenser family's own gather siblings the wheel's next seat the furnace canvas the most-used container the round banks first: the BLAST FURNACE + the SMOKER windows the same lane routes — the menu types 10/22 the furnace canvas the wire family shares, the one arm the whole trio the vanilla law answers).
- THE WALK (the chest gather's own law the furnace re-wears): the PAIR the round banks — the `furnace_doubleclick` flag + the `furnace_last_click_slot` seat the fields wear [the mouseClicked pcs 61-80 compute the dispatch BEFORE keeps, the pcs 386-389 the hovered slot the press remembers]; the PRESS WALK the flag arms — the furnace arm the double-click compute + the PICKUP dispatch the `!doubleclick` gate + the last-click seat the press banks; the RELEASE WALK the arm serves — the spread_fired chain the Furnace arm the gate trio answers [the flag + the mouse_pos + the hover seat], the gather the release CONSUMES [the goto 452 the quickcraft lane never enters]; the OFFLINE WALK the flat 39 feeds — the menu's own projection the predictor wears [the input/fuel/result trio the 0..3 rows, the main rows 3..30 the items 9..35, the hotbar 30..39 the items 0..8], the same-item stacks the gather home pulls, the different-item rows the walk never names, the every-mutation law the WRITE-THROUGH the standard rows keeps [the menu slot the inventory cell answers, the HUD row the hotbar follows]; the WIRE HALF the packet banks — the offline walk's twin over the LIVE belief [the live_furnace_carried the lane cursor the gate keys], the `ContainerInput::PickupAll` packet the command lane speaks [the changed map the ACTUAL mutations name, the no-op gates the ALWAYS-PACKET law keeps], the HUD row the hotbar rows 30..39 the sync follows; the RESETS the lifecycle banks — the open walk the stale pair retires, the close walk the removed() death discards, the route arm the fresh window re-arms, the window-death arm the None speaks.
- TESTS: part_11.rs NEW ONE (the_furnace_seats_the_gather_wears — the arms wear: the release's own walk [the pull the cursor grows, the drained row the write-through keeps, the different row the walk never names, the menu row the gather drains, the HUD row the hotbar follows, the arm's own clear the release speaks], the null hover's own flag keep, the press's own arm [the end-to-end double-click walk, the plain press the flag never arms], the close's own reset, the wire half's own walk [the route the window opens, the packet the command lane speaks, the HUD row the sync follows, the fresh window the stale gather pair retires]). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1528/1546/1504/1722/1517/1535/1550/1539/1527/1565/1654/1437 passed, each rc=0, 0 failed — the counts the 445 baseline the w11 the gather's own test +1 wears [1653 -> 1654 the prediction wore]; the post-bump re-walk the w11 the FINAL code re-wears) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (8 passed, 0 failed, scripts/directed_446.py the parts 9+10+11 gates the write_mod_multi the 8 filters the gather's own law family rides). GOLDEN: NO regeneration (19/19 rms 0.000 — the pre + the post-bump verify the red line re-wears — the gather the interaction logic alone moves, the canvas the bytes never see). FMT: the region law (scripts/fmt_probe_446.py — the round's own test region + the field pair + the gather trio + the four resets + the press walk, ALL-REGIONS-CLEAN — the first walk the wide assert the fold answered, the trailing-comment alignment the None arm the comment move answered). Bump 0.3.1239 -> 0.3.1240 (bump_version.py the workspace version + lock wear, the six members the same version keeps). Commit: 1f9ba372 (v0.3.1240, origin main the head wears).
- NEXT CANDIDATES: THE BREWING'S OWN GATHER SIBLING (the five-seat menu the same doubleclick field the 445/446 law re-wears — the bottle trio the ingredient seat the gather the same base law answers), THE DISPENSER'S OWN GATHER SIBLING (the 3x3 canvas the same arm the menu-type-6 window wears), THE BEACON'S OWN GATHER SIBLING (the payment seat the same arm the widget precedence the press walk keeps), THE ANVIL'S OWN GATHER SIBLING (the repair menu the same arm the rename field's own lane never walls).
"""


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


auth = open(AUTH, encoding="utf-8").read()
if MARK in auth:
    sys.exit("GUARD: AUTH already wears autopush-446 — the double lane never runs")
mirror0 = open(MIRROR, encoding="utf-8").read()
n_auth = auth.count("\n--- Task ID:") + auth.startswith("--- Task ID:")
n_mir = mirror0.count("\n--- Task ID:") + mirror0.startswith("--- Task ID:")
if n_auth != n_mir or md5(AUTH) != md5(MIRROR):
    sys.exit(f"GUARD: the pre-state diverges (auth {n_auth} vs mirror {n_mir})")
if not auth.endswith("\n"):
    auth += "\n"
open(AUTH, "w", encoding="utf-8").write(auth + ENTRY)
open(MIRROR, "w", encoding="utf-8").write(auth + ENTRY)
tail = open(MIRROR, encoding="utf-8").read()
if MARK not in tail or md5(AUTH) != md5(MIRROR):
    sys.exit("GUARD: the post-state diverges")
n = tail.count("\n--- Task ID:") + tail.startswith("--- Task ID:")
print(f"[worklog autopush-446] appended + mirrored :: {n} entries, md5 {md5(MIRROR)[:12]}")

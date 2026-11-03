#!/usr/bin/env python3
"""dev-215 — THE WORKLOG MIRROR SYNC (the one-shot lane the round's
own entry wears): the AUTH the round's own dev-215 entry appends,
the MIRROR the same bytes the copy answers, the guards the double
lane never wears. Usage: python3 scripts/worklog_dev215.py"""
import hashlib
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: dev-215"

ENTRY = """--- Task ID: dev-215 (2026-11-03 22:31:48 +08:00, Job 384925, trace 1a0a02c3c1060631-cron-agent-loop-202611032146) — v0.3.1237 THE LIVE QUICKCRAFT'S OWN STATE-ID RIDE
- Calibration: the COLD sandbox the 20:52 session the recovery lanes wore (the wipe ate the repo, the cargo AND the runtime cache — the interrupted autopush-444 walk the round RE-WEARS): the repo the token URL clones (HEAD 17046116 clean + origin-synced, the 443 round's own mirror commit the head wears), the cargo 1.99.1 the rustup lane installs, scripts/recover_runtime_cache.py the 26.2 jar + the official store re-fetches [3841 the flat copy answers, 7 the real panorama pixels, 1968 sounds.json events] + the gen cache the font.bin + textures.bin re-mints; the AUTH worklog the MIRROR the md5 identity keeps (123 autopush + 19 dev — autopush-443 the last, dev-214 the last dev), the wheel the dev-215 rotates. Chosen step: THE LIVE QUICKCRAFT'S OWN STATE-ID RIDE — the 443 NEXT's own first walk the round answers (the four-times-deferred candidate the wheel banks), the interrupted session's own 324-line test the tree KEPT [the uncommitted part_11 edit the wipe survived, the directed + fmt probes the scripts lane wears] — the round the walk VERIFIES and LANDS (the believed law the baseline already serves — the assertions the wire-split triple never wore).
- THE WALK (the believed state id the sync banks — the law the round's own test wears): the INVENTORY RIDE the window-0 drag wears — the live_inventory_updated feed the revision the live_inv_state_id the store keys [the Set Content's own last word the believed revision banks], the on_left_release the FIVE packets the believed revision each ride [the START the slot -999 the button 0 the QuickCraft input rides, the three ADDs the menu seats the button 1 each wear, the END the slot -999 the button 2 the changed deltas + the remainder the carried keeps]; the CORRECTION the sync banks — the wholesale re-feed the revision the store re-keys [the drag's own prediction the truth replaces, the cursor the wire's own authoritative pair carries], the next drag the corrected id re-rides; the CHEST PAIR the wholesale walk re-keys — the live_container_updated the (id, revision) pair the chest_live_window tuple banks, the chest drag the pair's revision each ride, the correction the pair re-keys, the second drag the corrected pair the release reads.
- TESTS: part_11.rs NEW ONE (the_live_quickcraft_the_state_id_ride_wears — the arms wear: the window-0 triple's own ride [the five packets the believed 7 each ride — the state-id assertions the wire-split triple never wore], the correction the sync banks [the re-feed the 8 the store speaks, the second drag the corrected id the triple re-rides], the chest pair's own ride [the (id, revision) pair the wholesale walk re-keys, the second drag the corrected pair the release reads]). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1527/1545/1503/1721/1516/1534/1549/1538/1526/1564/1650/1436 passed, each rc=0, 0 failed — the counts the 443 baseline the w11 the sibling's own test +1 wears [1649 -> 1650 the prediction wore]; the post-bump re-walk the w11 the FINAL code re-wears) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (6 passed, 0 failed, scripts/directed_215.py the part 9+11 gates the write_mod_multi the 6 filters the quickcraft's own law family rides — the wire-split kin the part_9 window wears). GOLDEN: NO regeneration (19/19 rms 0.000 — the pre + the post-bump verify the red line re-wears — the banking the assertions alone moves, the canvas the bytes never see). FMT: the region law (scripts/fmt_probe_215.py — the round's own test region, ALL-REGIONS-CLEAN). Bump 0.3.1236 -> 0.3.1237 (bump_version.py the workspace version + lock wear, the six members the same version keeps). Commit 154d9102 pushed (17046116..154d9102 main).
- NEXT CANDIDATES: THE OS CLIPBOARD'S OWN BRIDGE (the copypasta dep the headless risk may gate — the feature-gated backend the audio law re-wears), THE SEARCH BOX'S OWN SHIFT SEAT (the anchor ride the hasShiftDown query the 266 chat law re-wears — the shift-select the search box the same anchor banks), THE ENCHANT GATHER'S OWN WIRE PLACE-BACK (the release's own swing tail the wire lane may wear — the carried the gather left the press-back the same echo answers), THE HOPPER'S OWN FACE MOVE-ONE'S OWN INDEX (the hopper lanes' own furnace walk the (idx, arm) pair may lend — the suck lane's own kind ride the same law wears).
"""


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


auth = open(AUTH, encoding="utf-8").read()
if MARK in auth:
    sys.exit("GUARD: AUTH already wears dev-215 — the double lane never runs")
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
print(f"[worklog dev-215] appended + mirrored :: {n} entries, md5 {md5(MIRROR)[:12]}")

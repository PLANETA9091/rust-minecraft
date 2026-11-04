#!/usr/bin/env python3
"""dev-218 — THE WORKLOG MIRROR SYNC (the one-shot lane the round's
own entry wears): the AUTH the round's own dev-218 entry appends,
the MIRROR the same bytes the copy answers, the guards the double
lane never wears. Usage: python3 scripts/worklog_dev218.py"""
import hashlib
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: dev-218"

ENTRY = """--- Task ID: dev-218 (2026-11-04 10:52:21 +08:00, Job 384925, trace 1a0a02c3c1060631-cron-agent-loop-202611040949) — v0.3.1247 THE ANVIL GATHER'S OWN NAME RETIRE
- Calibration: the COLD sandbox the recycle wore — the local checkout the stale 443-mirror head the core.filemode-false + ff-merge answered (the origin the dev-217 head 2f7c4b3f the fetch verified, the 1694-file mode noise the false config answers, the local-only commits the none the walk verified), the cargo the wiped toolchain the rustup 1.99.1 reinstall answers, the assets-cache the wipe the restore_env.sh lane re-banks [the jar the sha1-verified fetch speaks (2dc72797...), the font.bin + the textures.bin the mc2rs-gen cache walk banks, the registry's four bake tests the missing cache the red wore, the restore the 100/100 the green re-wears], the worklog's 153 entries the count keeps — 131 autopush + 22 dev (dev-217 the last, the 09:47:32 lane the 0836 trace wore), the wheel the dev-218 rotates. Chosen step: THE ANVIL GATHER'S OWN NAME RETIRE — the 217 NEXT's own honest gap the round answers (the gather's (0,0) write-through the stale word keeps — the WIRE gather the drained cells the name lanes never retired, the OFFLINE gather the drained inventory row the same gap wore; the HOPPER LIVE DOOR candidate the census retires — the wire route the menu-type-15 window the chest lane's own dispatch already serves [the five-seat hopper flag the OpenScreen walk banks], the candidate the standing law wears).
- THE WALK (the gather's own name retire the two lanes wear): the WIRE ARM the changed map's own (0,0) write-through answers — the drained input the trio lanes retire [anvil_names the None, anvil_repair_costs the zero, anvil_enchants the empty, anvil_seat_potion_kinds the NONE — the offline walk's own trio law the wire gather re-wears], the drained mirror row the anvil_menu_to_inventory map the INVENTORY bank's own name cell retires [items_custom_names the None — the tooltip ghost the (0,0) row never speaks], the count>0 row the continue the word keeps; the OFFLINE ARM the drained row's own name retire answers — the changed loop's inventory branch the (0,0) pair the items_custom_names cell retires, the kept row the word the count speaks, the trio lanes' own input retire the standing law keeps, the mirror sync + the HUD row the tails re-wear.
- TESTS: part_11.rs NEW ONE (the_anvil_gather_the_name_retire_wears — the arms wear: the WIRE's own retire [the single gather the named input AND the named mirror row drain — the hovered empty seat the pull's own home wears, the trio lanes the input retires, the INVENTORY bank's own name cell the mirror row retires, the different dirt row the walk never names, the kept word the count speaks, the changed map the (1,(0,0)) + the (4,(0,0)) pair names, the carried the 15 sum wears, the single echo the lane exhausts], the OFFLINE's own row retire [the drained lapis row the name cell retires, the different dirt row the walk never names, the kept word the count speaks, the HUD row the sync follows]). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1528/1546/1504/1722/1517/1535/1550/1539/1527/1565/1661/1437 passed, each rc=0, 0 failed — the counts the 217 baseline the w11 the gather's own test +1 wears [1660 -> 1661 the prediction wore]; the post-bump re-walk the w11 the FINAL code re-wears) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep — the registry's four bake tests the COLD sandbox's wiped assets-cache the red wore, the restore_env.sh lane the jar + the font.bin + the textures.bin the recovery re-banks, the 100/100 the green re-wears) + the directed families (13 passed, 0 failed, scripts/directed_218.py the parts 9+10+11 gates the write_mod_multi the 13 filters the gather's own law family rides — the 445..451 gather kin the same shape the windows bank). GOLDEN: NO regeneration (19/19 rms 0.000 — the verify the red line re-wears — the retire the interaction logic alone moves, the canvas the bytes never see). FMT: the region law (scripts/fmt_probe_218.py — the offline gather's drained-row arm + the wire gather's retire block + the round's own test region, ALL-REGIONS-CLEAN — the hotbar assert's own single line the rustfmt law wears). Bump 0.3.1246 -> 0.3.1247 (bump_version.py the workspace version + lock wear, the six members the same version keeps). Commit f7eff403 pushed (2f7c4b3f..f7eff403 main).
- NEXT CANDIDATES: THE ANVIL WIRE OPEN'S OWN FRESH LAY (the live wire open walk the gather pair + the rename field's fresh word never re-lays — the 202-era lane the chest/brewing/enchant kin's own fresh-open reset the anvil lane lacks, the stale pair the stale word the fresh window may keep, the vanilla fresh-screen law the AnvilScreen ctor's own empty field the honest answer), THE CARRIED'S OWN GATHER RIDE (the gather into the empty cursor the named stack's lanes the carried seat may wear — the single-click pickup's whole_moved law the gather's own carried-gained arm re-wears, the merged stack the original's word keeps), THE SERVER-DROP'S OWN ECHO GATE (the 217 NEXT's carry-over — the initiator the packet lane may name, the client-side close the serverbound echo speaks), THE MERCHANT'S OWN REFUSAL ARM (the canTakeItemForPickAll FALSE the villager menu the gather's silence wears — the merchant screen the later round the wire route may bank first).
"""


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


auth = open(AUTH, encoding="utf-8").read()
if MARK in auth:
    sys.exit("GUARD: AUTH already wears dev-218 — the double lane never runs")
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
print(f"[worklog dev-218] appended + mirrored :: {n} entries, md5 {md5(MIRROR)[:12]}")

#!/usr/bin/env python3
"""dev-217 — THE WORKLOG MIRROR SYNC (the one-shot lane the round's
own entry wears): the AUTH the round's own dev-217 entry appends,
the MIRROR the same bytes the copy answers, the guards the double
lane never wears. Usage: python3 scripts/worklog_dev217.py"""
import hashlib
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: dev-217"

ENTRY = """--- Task ID: dev-217 (2026-11-04 09:47:32 +08:00, Job 384925, trace 1a0a02c3c1060631-cron-agent-loop-202611040836) — v0.3.1246 THE BREWING-ANVIL WINDOWS' OWN DEATH RETIRE
- Calibration: the WARM sandbox the round keeps — the 451 head the fetch the origin-synced clean tree verified (the 27251150 mirror commit the head wears, the e97753ea the round's own titled commit the walk landed, the cargo + the assets-cache the prior rounds' recovery the tree still banks), the worklog's 152 entries the count keeps — 131 autopush + 21 dev (autopush-451 the last autopush, dev-216 the last dev), the wheel the dev-217 rotates. Chosen step: THE BREWING-ANVIL WINDOWS' OWN DEATH RETIRE — the 451 NEXT's own honest gap the round answers (the brewing window-death candidate the wheel banks, the anvil lane the census's own twin the same walk banks — the None arm the table/chest/furnace/dispenser/enchant lanes the death walk retired, the menu-type-11 and the menu-type-8 windows the wire routes the screens the dead windows kept standing).
- THE WALK (the chest-family death law the two lanes re-wear): the BREWING ARM the None walk answers — the screen the world seat answers [the Brewing screen the live flag's own gate the open(ScreenKind::World) speaks, the open() tail the container_seats_removed walk the close_brewing ride serves — the removed() walk the offline carried's placeItemBackInInventory home ride the server-initiated drop also wears], the brewing_live_window the death Nones, the authoritative cursor the live_brewing_carried zero retires, the gather pair the fresh open's own reset re-arms [the brewing_doubleclick flag + the brewing_last_click_slot seat the death clears], the live_brewing_title the window's death retires; the ANVIL ARM the same law the repair lane answers — the screen the world seat answers, the anvil_live_window the death Nones, the single carried field the anvil_carried zero retires, the gather pair the death clears, the live_anvil_title the window's death retires [the OLD KEEP-STANDING VERDICT the 202 route wore the census retires — the route test's own final arm the honest walk re-verdicts].
- TESTS: part_11.rs NEW ONE (the_brewing_anvil_windows_the_death_retires — the arms wear: the brewing window's own open [the route the stand window opens, the five the wire banks, the mirror rows the wire's own tail feeds, the data words the wire's own pair banks, the wordless component the honest None banks], the store's own drop [the world seat the drop answers, the live lane the death retires, the offline carried the removed() walk's own ride home serves, the ridden bottle the inventory cell answers, the rows the drop never clears], the fresh open's own reset [every fresh window the pair retires], the anvil window's own open [the route the repair window opens, the authoritative cursor the wire lands, the result seat the wire's own row banks], the anvil's own drop [the same walk the repair lane answers], the offline door the death never wears [the fresh open the live lane stays down]) + the ROUTE TEST'S OWN RE-VERDICT (the_anvil_wire_route_the_menu_type8_wear — the old keep-standing answer the window-death's own retire re-wears [the world seat + the live flag + the window identity the drop answers]). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1528/1546/1504/1722/1517/1535/1550/1539/1527/1565/1660/1437 passed, each rc=0, 0 failed — the counts the 451 baseline the w11 the window-death's own test +1 wears [1659 -> 1660 the prediction wore]; the post-bump re-walk the w11 the FINAL code re-wears) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (13 passed, 0 failed, scripts/directed_217.py the parts 9+10+11 gates the write_mod_multi the 13 filters the window-death's own law family rides — the 445..451 gather kin the same shape the windows bank). GOLDEN: NO regeneration (19/19 rms 0.000 — the verify the red line re-wears — the retire the interaction logic alone moves, the canvas the bytes never see). FMT: the region law (scripts/fmt_probe_217.py — the route test's own re-verdict arm + the round's own test region + the brewing arm + the anvil arm the None walk's retire blocks wear, ALL-REGIONS-CLEAN — the first walk the vec fold + the comment glue the blank lines answered; the whole-tree cargo fmt the NOT-OUR-LAW the revert answered — the region law the file's own style keeps). Bump 0.3.1245 -> 0.3.1246 (bump_version.py the workspace version + lock wear, the six members the same version keeps). Commit e97753ea pushed (27251150..e97753ea main).
- NEXT CANDIDATES: THE SERVER-DROP'S OWN ECHO GATE (the None walk the close ride the serverbound Close echo the server-initiated drop speaks — the chest family's own lanes the echo the honest gate may answer, the initiator the packet lane may name), THE ANVIL GATHER'S OWN NAME RETIRE (the drained mirror row the custom-name lane the same retire the input seats wear — the (0,0) write-through the stale word the honest gap keeps), THE MERCHANT'S OWN REFUSAL ARM (the override census the merchant alone refuses — the canTakeItemForPickAll FALSE the villager menu the gather's own silence wears, the merchant screen the later round the wire route may bank first), THE HOPPER'S OWN LIVE DOOR (the wire session the hopper route the chest_window_hopper flag wears — the live five-seat window the chest wire lane's own tests the hopper's own names may bank).
"""


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


auth = open(AUTH, encoding="utf-8").read()
if MARK in auth:
    sys.exit("GUARD: AUTH already wears dev-217 — the double lane never runs")
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
print(f"[worklog dev-217] appended + mirrored :: {n} entries, md5 {md5(MIRROR)[:12]}")

#!/usr/bin/env python3
"""autopush-452 — THE WORKLOG MIRROR SYNC (the one-shot lane the
round's own entry wears): the AUTH the round's own autopush-452 entry
appends, the MIRROR the same bytes the copy answers, the guards the
double lane never wears. Usage: python3 scripts/worklog_autopush452.py"""
import hashlib
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: autopush-452"

ENTRY = """--- Task ID: autopush-452 (2026-11-04 14:49:57 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611041409) — v0.3.1250 THE ANVIL COST'S OWN WIRE WORD
- Calibration: the WARM sandbox the dev-220 round the same session kept — the tree the round's own HEAD 9dde533c the fetch verified (the origin the 220 mirror the walk answers, the local-only commits the none), the worklog's 156 entries the count keeps (131 autopush + 25 dev, dev-220 the last the 14:07:53 lane wore, autopush-451 the last sync the 08:33 lane wore), the wheel the autopush-452 rotates. Chosen step: THE ANVIL COST'S OWN WIRE WORD — the 220 NEXT's own fourth lane the round answers (the anvil fill arm the enchant lane's own data-words law the sibling never wore: the AnvilMenu's own DataSlot the addSlotData registers — the single cost word the ContainerSetData lane speaks — the live walk the wire never banked, the fresh lay's honest zero the only word the cost lane wore; the carried-name lane the ① candidate the absent lane the survey keeps deferred, the echo gate + the merchant refusal the later rounds).
- THE WALK (the wire word the cost lane banks): the fill arm's own COST WORD landing [the data slot the wire speaks anvil_cost re-lays — the zero the fresh lay banks the real word the next sync answers, the word the wire never spoke the honest zero keeps], the WALK the server owns keeps (the result seat the wire's own rows answer — the cost the data word the same ownership wears, the local compute the live lane never rides).
- TESTS: part_11.rs NEW ONE (the_anvil_cost_the_wire_word_wears — the arms wear: the wire word the fresh window re-lays [data slot 5 the cost lane banks], the word rides every sync [the recomputed word 7 the re-sync banks], the wordless wire the honest zero keeps [the absent word the zero never re-lays]). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1528/1546/1504/1722/1517/1535/1550/1539/1527/1565/1664/1437 passed, each rc=0, 0 failed — the counts the 220 baseline the w11 the wire word's own test +1 wears [1663 -> 1664 the prediction wore]; the post-bump re-walk the w11 the FINAL code re-wears) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (16 passed, 0 failed, scripts/directed_452.py the parts 9+10+11 gates the 16 filters — the cost's own test + the enchant data-words kin + the 220/219/218 anvil kin + the route + the 449 hopper kin + the 445..451 gather kin). GOLDEN: NO regeneration (19/19 rms 0.000 — the cost word the interaction logic alone moves, the canvas the bytes never see). FMT: the region law (scripts/fmt_probe_452.py — the wire walk's own cost word block + the round's own test region, ALL-REGIONS-CLEAN — the first walk the clean verdict wore). Bump 0.3.1249 -> 0.3.1250 (bump_version.py the workspace version + lock wear, the six members the same version keeps). NOTE: the round the INTERRUPTED w11 walk the mid-gate mod.rs the checkout answered — the canceled gate's finally restore the recycler ate, the fresh walk the same window the baselines re-wore. Commit 0dcfba6f pushed (9dde533c..0dcfba6f main).
- NEXT CANDIDATES: THE CARRIED'S OWN GATHER RIDE (the gather into the empty cursor the named stack's word the carried seat may wear — the carried-name lane the survey found ABSENT, the new lane the whole_moved arm may first invent, the merged stack the original's word keeps), THE SERVER-DROP'S OWN ECHO GATE (the carry-over — the window.rs Close command the session.close_container echo already wears, the OTHER client-side close paths the initiator law the survey answers), THE MERCHANT'S OWN REFUSAL ARM (the canTakeItemForPickAll FALSE the villager menu the gather's silence wears — the merchant wire route the later round banks first), THE FURNACE/BEACON DATA-WORDS' OWN CENSUS (the enchant + the anvil the data-word lanes wear — the furnace/brewing/beacon the words the wire banks the census the sibling law the honest walls re-wears).
"""


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


auth = open(AUTH, encoding="utf-8").read()
if MARK in auth:
    sys.exit("GUARD: AUTH already wears autopush-452 — the double lane never runs")
mirror0 = open(MIRROR, encoding="utf-8").read()
n_auth = auth.count("\n--- Task ID:") + auth.startswith("--- Task ID:")
n_mir = mirror0.count("\n--- Task ID:") + mirror0.startswith("--- Task ID:")
if n_auth != n_mir or md5(AUTH) != md5(MIRROR):
    sys.exit(f"GUARD: pre-state diverges (auth {n_auth} vs mirror {n_mir})")
if not auth.endswith("\n"):
    auth += "\n"
open(AUTH, "w", encoding="utf-8").write(auth + ENTRY)
open(MIRROR, "w", encoding="utf-8").write(auth + ENTRY)
tail = open(MIRROR, encoding="utf-8").read()
if MARK not in tail or md5(AUTH) != md5(MIRROR):
    sys.exit("GUARD: post-state diverges")
n = tail.count("\n--- Task ID:") + tail.startswith("--- Task ID:")
print(f"[worklog autopush-452] appended + mirrored :: {n} entries, md5 {md5(MIRROR)[:12]}")

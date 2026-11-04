#!/usr/bin/env python3
"""dev-220 — THE WORKLOG MIRROR SYNC (the one-shot lane the round's
own entry wears): the AUTH the round's own dev-220 entry appends,
the MIRROR the same bytes the copy answers, the guards the double
lane never wears. Usage: python3 scripts/worklog_dev220.py"""
import hashlib
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "logs/worklog.md"
MARK = "Task ID: dev-220"

ENTRY = """--- Task ID: dev-220 (2026-11-04 14:07:53 +08:00, Job 384925, trace 1a0a02c3c1060631-cron-agent-loop-202611041306) — v0.3.1249 THE ANVIL DEATH'S OWN FIELD RETIRE
- Calibration: the COLD sandbox the recycle wore AGAIN — the local checkout the stale 443-mirror head 17046116 the core.filemode-false + ff-merge answered (the origin the dev-219 head a0b5600f the fetch verified — the 219 round the PARALLEL session the 1106 trace wore, the mode noise the false config answers, the local-only commits the none the walk verified), the AUTH worklog the stale 142 the mirror's own 155 re-restores [the md5 the 0a23efcb the copy wears — the 218 + 219 entries the mirror the recovery banks], the cargo the wiped toolchain the rustup 1.99.1 reinstall answers + the rustfmt the component lane re-adds, the assets-cache the wipe the restore_env.sh lane re-banks [the jar the sha1-honest cache speaks, the font.bin + the textures.bin the gen walk banks, the 5057 objects + the 10967 jar textures the walks bank — the background lane the session reaper kept killing, the foreground walk the restore finished], the FIRST gate walk the incomplete objects the 26/15/14/33 reds wore [the title/font assertions the missing objects the honest reds spoke], the finished restore the same windows the baselines re-wears, the worklog's 155 entries the count keeps — 131 autopush + 24 dev (dev-219 the last, the 11:55:45 lane the 1106 trace wore), the wheel the dev-220 rotates. Chosen step: THE ANVIL DEATH'S OWN FIELD RETIRE — the 219 NEXT's own fourth lane the round answers (the 217 death walk the window identity + the gather pair + the cursor the retire wears, the rename field's stale word the death never banks — the close_anvil walk + the offline open_anvil_for + the 219 wire fresh lay the SAME SIX the honest zero speak, the server-initiated None arm the field family the walk never wore; the CARRIED'S OWN GATHER RIDE the ① candidate the absent carried-name lane the survey vetoed — the death field the honest gap the round banks).
- THE WALK (the field retire the stale words wear): the FIELD FAMILY the death walk re-lays [anvil_name the clear the stale word the death retires, the caret the fresh field's own zero park keeps, the highlight the None, the cost the honest zero, the only-renaming verdict the false, the repair count the zero — the SAME SIX the close_anvil walk + the offline open_anvil_for + the 219 wire fresh lay speak, the server-initiated None arm the walk never wore], the ROWS the words the count keeps (the mirror cells the inventory's own bank wears — the field the rows never answer, the death the cells never clears).
- TESTS: part_11.rs NEW ONE (the_anvil_death_the_field_words_retire — the arms wear: the stale words the live window banks [the typed word the caret 3 wears + the highlight + the cost 5 + the rename-only verdict + the repair count 2 + the gather pair + the title + the cursor the walk banks], the store's own drop [the field family the same honest zero the death re-lays, the row's own word the mirror keeps], the second window's own death [the re-typed word the SECOND session the same walk answers]). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1528/1546/1504/1722/1517/1535/1550/1539/1527/1565/1663/1437 passed, each rc=0, 0 failed — the counts the 219 baseline the w11 the field retire's own test +1 wears [1662 -> 1663 the prediction wore]; the post-bump re-walk the w11 the FINAL code re-wears) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (13 passed, 0 failed, scripts/directed_220.py the parts 9+10+11 gates the 13 filters — the death's own test + the route test + the 449/218/217 anvil kin + the 445..451 gather kin the same shape the windows bank). GOLDEN: NO regeneration (19/19 rms 0.000 — the verify the red line re-wears — the field retire the interaction logic alone moves, the canvas the bytes never see). FMT: the region law (scripts/fmt_probe_220.py — the death arm's own field block + the round's own test region, ALL-REGIONS-CLEAN — the verdict assert's own single line + the title lane's own comment order the rustfmt law wears). Bump 0.3.1248 -> 0.3.1249 (bump_version.py the workspace version + lock wear, the six members the same version keeps). Commit d2ff2cea pushed (a0b5600f..d2ff2cea main).
- NEXT CANDIDATES: THE CARRIED'S OWN GATHER RIDE (the gather into the empty cursor the named stack's word the carried seat may wear — the carried-name lane the survey found ABSENT, the new lane the whole_moved arm may first invent, the merged stack the original's word keeps), THE SERVER-DROP'S OWN ECHO GATE (the 218/219 NEXT's carry-over — the window.rs Close command the session.close_container echo already wears, the OTHER client-side close paths the initiator law the survey answers), THE MERCHANT'S OWN REFUSAL ARM (the canTakeItemForPickAll FALSE the villager menu the gather's silence wears — the merchant wire route the later round banks first), THE ANVIL COST'S OWN WIRE WORD (the 219 walk's own confession — the live lane the data-word cost lane never banks, the wire's ContainerData the cost the fresh window may re-lay the real word the zero compute never speaks).
"""


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


auth = open(AUTH, encoding="utf-8").read()
if MARK in auth:
    sys.exit("GUARD: AUTH already wears dev-220 — the double lane never runs")
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
print(f"[worklog dev-220] appended + mirrored :: {n} entries, md5 {md5(MIRROR)[:12]}")

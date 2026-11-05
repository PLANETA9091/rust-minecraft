#!/usr/bin/env python3
"""autopush-470 — THE WORKLOG APPEND (the round's own entry the AUTH
banks, the MIRROR the same bytes the copy answers, the md5 the pair
verifies). Usage: python3 scripts/worklog_autopush470.py"""
import hashlib

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-470 (2026-11-06 01:31:00 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611060031) — v0.3.1272 THE MERCHANT'S OWN TRADE-SOUND SILENCE
- Calibration: the WARM sandbox the round keeps (the repo the origin's own 469 mirror head 58b3cc01 the fetch verified — the origin the v0.3.1271 head the wheel keeps, the tree the walk opened CLEAN, the cargo 1.99.1 the toolchain keeps, the client.jar + the gen cache + the index store the standing cache wears), the worklog's 178 entries the count wears — 149 autopush + 29 dev (autopush-469 the last, the 2307 trace wore), the wheel the 470 rotates. Chosen step: THE MERCHANT'S OWN TRADE-SOUND SILENCE — the 469 NEXT's own second lane the round ANSWERS (the playTradeSound the quick-move tail the census the jar's own bytes speak — the sound the port's lane the exact fidelity the verdict banks: the dead code the bytes THEMSELVES wear).
- THE WALK (the 26.2 bytes the census speaks — scripts/jar_work/tradesound_472.txt + the 2..8 siblings + the parse scripts, all rc=0): the call site quickMoveStack pc 74 the ONLY one the class wears [the onQuickCraft tail the move's own success answers — the tryMoveItems body the walk never names, the postButtonClick pc 25 the tryMoveItems invocation alone speaks], the guard trader.isClientSide() ifne 54 the client's own predicted walk the EARLY RETURN keeps [the ClientSideMerchant.isClientSide the source player's level word reads — ALWAYS true on the client], the server's own body the Level.playLocalSound(DDD...) base the EMPTY code=1 return wears [the ServerLevel the override never declares], the ClientLevel override the REAL body wears [the camera-position attenuation the playSound(DDD...J) walk answers] — the override the guarded path NEVER reaches: the sound the vanilla bytes THEMSELVES never play. THE SOUND THE CENSUS NAMES: getNotifyTradeSound the VILLAGER_YES event answers [AbstractVillager + ClientSideMerchant both — the entity.villager.yes the mob/villager/yes1-3.ogg files the cache keeps], the real post-trade voice the server's own AbstractVillager.notifyTrade the ambientSoundTime = -getAmbientSoundInterval() reset the ambient tick the soon voice arms [pc 4-12 — the entity-sound packet the standard lane broadcasts, the port's drains the sounds already bank], the ClientSideMerchant.notifyTrade the increaseUses() ALONE the code=5 body wears [the port's merchant_result_take walk the exact bytes mirror]. THE PORT'S OWN WALK: the stale no-audio-infra clause the quick-move docstring the census truth REPLACES [the DEAD-CODE SILENCE the 470 census the docstring speaks], the merchant_result_quick_move walk the same bytes keeps [the silence no code change wears].
- TESTS: part_11.rs NEW ONE (the_merchant_trade_sound_the_dead_code_silence_wears — ARM A the plain take's own silence [the row press + the take click the trade applies — the uses the notify increments, the xp the reward speaks, the ui_sounds lane the EMPTY law keeps], ARM B the quick-move's own silence [the refill's press the fresh result wears, the shift-click the result rides home — the slots[38] the reversed walk banks, the uses the second notify speaks, the ui_sounds lane the press walk alone wears]). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — each rc=0, 0 failed — w1 1528 / w2 1546 / w3 1504 / w4 1722 / w5 1517 / w6 1535 / w7 1550 / w8 1539 / w9 1527 / w10 1565 / w11 1685 / rest 1437 — the w11 count the 469's 1684 baseline + the round's own test +1 wears) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (14 passed, 0 failed, scripts/directed_472.py the parts 9+10+11 gates the 14 filters — the round's own pin + the 469 result-take/try-move/slots kin + the 466 stale-selection kin + the 467 max-fold kin + the 465 stale-scroll kin + the 464 visible-gate kin + the 463 echo-gate kin + the 462 title kin + the 461 sprites kin + the 459 offers kin + the 469 instant-ship kin + the state-id ride kin the same shape the windows bank). GOLDEN: NO regeneration (19/19 rms 0.000 — ALL GOLDENS PASS — the census + the docstring + the test the interaction-logic bytes alone move, the render canvas the bytes never see). FMT: the region law (scripts/fmt_probe_472.py — the round's own regions [the quick-move docstring + the round's own test], ALL-REGIONS-CLEAN). Bump 0.3.1271 -> 0.3.1272 (bump_version.py the workspace version + lock wear, the six members the same version keeps, the js-sys/web-sys ecosystem the own band keeps). Commit: 2f24a27c (v0.3.1272, origin main the head wears).
- NEXT CANDIDATES: THE VANILLA CRASH'S OWN WEAR (the 466 census the fidelity round may re-answer — the deliberate panic the vanilla render crash the round may wear or the silence the quirk the port keeps), THE INSTANT SHIP'S OWN HEADLESS WALK (the headless pump the golden lane wears — the correction word the headless scene never rides, the lane the later round may re-answer), THE STAT'S OWN TRADED LANE (the awardStat TRADED_WITH_VILLAGER pc 69-73 the port's stat lane never wears — the stats screen the later round may re-lay).
"""


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


auth = open(AUTH, encoding="utf-8").read()
if "Task ID: autopush-470" in auth:
    raise SystemExit("GUARD: the AUTH already wears the autopush-470 entry")
if not auth.endswith("\n"):
    auth += "\n"
auth += ENTRY
open(AUTH, "w", encoding="utf-8").write(auth)
# THE MIRROR the same bytes the copy answers
mirror = open(MIRROR, encoding="utf-8").read()
if not mirror.endswith("\n"):
    mirror += "\n"
mirror += ENTRY
open(MIRROR, "w", encoding="utf-8").write(mirror)
if md5(AUTH) != md5(MIRROR):
    raise SystemExit("GUARD: the md5 the pair never wears")
print("autopush-470 entry appended: the AUTH + the MIRROR the same bytes wear")
print(f"AUTH md5 {md5(AUTH)}")

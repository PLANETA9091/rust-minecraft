#!/usr/bin/env python3
"""autopush-461 BACKFILL — the interrupted lane's own tail the worklog
re-wears (the 441-round precedent: the code the origin head banks, the
entry the later auto-sync lands). The 461 session pushed the code commit
d097dcad (v0.3.1261) and the context the push consumed — the entry the
walk never appended. This script appends the SAME entry the both
worklogs wear (AUTH + MIRROR, the md5 the pair verifies).
Usage: python3 scripts/worklog_backfill_461.py"""
import hashlib

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

ENTRY = """--- Task ID: autopush-461 (2026-11-05 07:22:59 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202611050722) — v0.3.1261 THE OFFER ROWS' OWN SPRITES
- Calibration: the round the 459 NEXT's own remaining lane answered; the code commit d097dcad the origin head wears [BACKFILL NOTE: the walk's own context the code push consumed — THIS entry the 462 round's own auto-sync backfills, the 441-round precedent the interrupted tail's mirror entry wears; the round's own NEXT list the 462 entry banks]. Chosen step: THE OFFER ROWS' OWN SPRITES — the out-of-stock overlay + the button sprites + the scroller + the arrows + the tooltips the 456 render's minimal round deferred, the canvas walk the round re-lays.
- THE WALK (the widget states the lanes bank): THE SPRITE BANK'S OWN WORDS (javap 26.2 MerchantScreen <clinit> — the container/villager/* atlas names the eight ldc rows speak, the generated defs the exact dims keep: the trade_arrow pair the 10x9 seat, the out_of_stock the 28x21 band, the scroller pair the 6x27 handle), THE OUT-OF-STOCK OVERLAY (javap extractBackground tail — the SELECTED offer's own stock verdict the shopItem the press banks, the 28x21 seat the (leftPos+182, topPos+35) offset wears), THE SCROLLER'S OWN LANE (javap extractScroller — the scrollable count the size+1-7 walk speaks, the travel the 139-27-(139*(i-1))/i pair banks, the offset the min(113, scrollOff*k) clamp wears, the max-scroll seat the scrollOff==i-1 force speaks, the >7 bank the enabled handle wears, the <=7 bank the disabled sprite answers), THE ROW BUTTONS' OWN SPRITE (javap init — the SEVEN 88x20 rows the topPos+18 base wears [the port's +20 drift the round retires], the press's own offer the getIndex() + scrollOff fold the lambda$init$0 banks, the visible law the RAW seat the size compare answers, the hovered frame the widget/button_highlighted sprite answers), THE ARROWS' OWN PAIR (javap extractButtonArrows — the out-of-stock the stale sprite wears, the seat (row+60, y+3) the 10x9 blit keeps), THE WHEEL'S OWN RIDE (javap mouseScrolled — the canScroll gate the <=7 bank keeps, the (int)((double) scrollOff - delta) fold the [0, size-7] clamp wears), THE DRAG'S OWN ARM + FOLD (javap mouseClicked head — the button-blind walk the ANY press the bar band arms [the strict (94,100) x (18,158] pairs]; javap mouseDragged — the (y - topPos - 18 - 13.5)/112 fraction the f*max + 0.5 fold wears, the clamp the band keeps; javap mouseReleased — the iconst_0 the disarm speaks), THE TOOLTIPS' OWN TAIL (javap TradeOfferButton.extractToolTip — the hovered row's x band the offer's own stack answers [the [0,20)/(30,50)/(65,88] bands the 20..30 + 50..65 gaps keep]; javap extractContents tail — the deprecated row the stock verdict + the isHovering(186,35,22,21) band + the canRestock word the merchant.deprecated lang row answers). THE JAR'S OWN EVIDENCE (scripts/jar_work/sprites_461.txt + scripts/jar_work_parse_sprites_461.py — all rc=0).
- TESTS: part_11.rs NEW ONE (the_merchant_rows_own_sprites_the_button_states_wear — the sprite bank's words + the scroller's math + the wheel's offset + the drag's fold + the row seat's walk + the bands + the deprecated hover + the app's own walk the scrolled window's press the OFFER index banks). Verdict: green. THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1528/1546/1504/1722/1517/1535/1550/1539/1527/1565/1675/1437 passed, each rc=0, 0 failed — the counts the 460 baseline the w11 the sprites' own test +1 wears [1674 -> 1675 the prediction wore]) + the fast trio (the 161/100/80 the protocol/registry/gen banks keep) + the directed families (25 passed, 0 failed, scripts/directed_462.py the parts 9+10+11 gates the 25 filters). GOLDEN: NO regeneration (19/19 rms 0.000 — the widget states the merchant canvas alone moves, the world canvas the bytes never see). FMT: the region law (scripts/fmt_probe_462.py — the screens + window + part_11 regions, ALL-REGIONS-CLEAN — scripts/fmt_fix_462.py the surgical region formatter the round's regions only applies). Bump 0.3.1260 -> 0.3.1261 (bump_version.py the workspace version + lock wear, the six members the same version keeps). Commit: d097dcad (v0.3.1261, origin main the head wears).
- NEXT CANDIDATES: THE MERCHANT.TITLE'S OWN ARG ROW (the composite the level word the label wears — the "%s — %s" row the lang table the text round re-lays), THE TRADE'S OWN LIVE ECHO BANK (the server's SetSlot/SetContent corrections the seats re-bank — the drift the prediction wears, the re-bank walk the container lane serves), THE LIVE QUICKCRAFT'S OWN STATE-ID RIDE (the wire folds' believed state id the window re-keys — the server's own correction the sync the five-times-deferred round banks), THE OS CLIPBOARD'S OWN BRIDGE (the copypasta dep the client store the OS store the later round wears — the headless build risk the lane first answers).
"""


def md5(path):
    return hashlib.md5(open(path, "rb").read()).hexdigest()


for path in (AUTH, MIRROR):
    src = open(path, encoding="utf-8").read()
    if "Task ID: autopush-461" in src:
        print(f"{path}: autopush-461 already present, skip")
        continue
    if not src.endswith("\n"):
        src += "\n"
    src += ENTRY
    open(path, "w", encoding="utf-8").write(src)
    print(f"{path}: appended")

if md5(AUTH) != md5(MIRROR):
    raise SystemExit("GUARD: the md5 the pair never wears")
print("autopush-461 backfill appended: the AUTH + the MIRROR the same bytes wear")
print(f"AUTH md5 {md5(AUTH)}")

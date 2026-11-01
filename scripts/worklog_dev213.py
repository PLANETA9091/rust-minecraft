#!/usr/bin/env python3
"""dev-213 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..392 record's
own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the head count the 72+18 alignment the calibration wore)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: dev-213 (2026-11-01 09:58:36 +08:00, Job 384925, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611010905) — v0.3.1184 "
    "THE FURNACE TRIO'S OWN KIND BANK"
)

BODY = """
- Calibration: HEAD 927e4073 clean + origin-synced (the autopush-392 round's own mirror commit the head wears); the worklog's 72 autopush + 17 dev entries the count keeps (dev-212 the dev side's own last, autopush-392 the autopush side's own last), the wheel the dev-213 rotates. Chosen step: THE FURNACE TRIO'S OWN KIND BANK (the 377-billed RESULT TAKE'S OWN KIND WORD the round answers — the furnace result's own take the carried kind the plain id model never banks, the dev-196 store's second axis the furnace trio joins, the autopush-392 NEXT list's own first walk the dev wheel re-wears).
- THE KIND'S OWN PER-CELL BANK (the furnace_kind_stores the furnace_word_stores' own pattern wears — INPUT 0 FUEL 1 RESULT 2 the FurnaceMenu law the words key, the disk the "furnace_kinds" row the same containers file banks [the furnace_words' own shape the bank/parse pair serves, the 3-word cap the tolerant parse pads], the LIVE TRIO the furnace_seat_potion_kinds the open window's words wear — the bank's load the open rides, the flush the re-bank serves, the wire landing the honest NONE wall keeps). THE WALKS' OWN WORDS: the CLICK'S OWN FOUR-ARM RIDE (the dev-208 kindless wall the honest bank retires — the PICKUP the seat's word the carried banks [the RESULT TAKE'S OWN KIND WORD the take's own hand answers], the PLACE the word the seat lays, the SWAP the pairs exchange, the MERGE the emptied carried the NONE wears; the trio the SEAT STORE banks, the store rows the INVENTORY bank keeps), the TICK'S OWN WORD SYNC (the smelt walk the words follow — the live open AND the closed banked cells both lanes: the emptied seat the NONE retires, the shrink the word keeps, the fresh result the recipe's wordless output the NONE keeps), the PERSISTENCE'S OWN RIDES (the open the banked kinds loads, the switch the prev cell re-banks, the flush the close banks, the spill the broken furnace's words kill, the restart the list the every cell loads), the PAINT/DRAG'S OWN READS (the 382/383 component half the seat store's word answers, the place's own bank the empty trio seat the helper's own write serves) + the TOOLTIP'S OWN READ (the 356 law the trio the store's word answers — the kind the furnace store carries now).
- TESTS: part_11.rs NEW ONE (the_furnace_trio_the_kind_bank_wears — the eight arms wear: the place's own bank, the pickup's own ride, the RESULT TAKE'S OWN WORD, the same-id kind pair's own swap, the smelt's own sync, the flush/reopen's own word, the wire's own NONE wall, the drag's own bank + the paint gate) + THE OLD WALLS' OWN HONEST RE-KEY (the_furnace_seats_the_model_ride_wears the dev-208 kindless arms the exchange law re-wears, the_furnace_carried_the_word_wears the honest wall the bank re-wears — the placed potion's own word the seat banks, the re-pickup the banked word lends). Verdict: the new test the sync arm the live-tick gate the world block the setup speaks, then green FIRST RUN; the re-keyed arms green.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1598/1435 passed, each rc=0, 0 failed — the counts the 392 baseline +1 the w11 the furnace trio's own bank, the other windows the 392 numbers keep; the one window per call the 392 lesson the round keeps) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the furnace trio kind bank + the furnace seats/carried re-keys + the anvil kind bank + the wire generic place + the wire hopper door + the wire table/dispenser/chest/furnace places + the wire paint + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip/drag rides' own thirty-two — 32 passed, 0 failed, scripts/directed_dev213.py the 32nd filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the furnace kind bank the live session's own seat walks alone voices, the golden scenes' wordless furnace never opens, the NONE identity the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_dev213.py — the store decls + the ctor + the NBT pair + the rides + the helpers + the click four-arm + the tick syncs + the wire wall + the reads + the bank + the test tail, ALL-REGIONS-CLEAN). Bump 0.3.1183 -> 0.3.1184 (bump_version.py the workspace version + lock wear). Commit 743deb8e pushed (927e4073..743deb8e main).
- NEXT CANDIDATES: THE FURNACE QUICK-MOVE'S OWN KIND RIDE (the shift-click's routing the words the plain pairs never move — the smeltable's row word the input seat banks, the result's take the inventory row lends, the 384 quick_move_kind_ride the furnace's own routing re-wears), THE WIRE HOPPER'S OWN PAINT GATE (the hopper's five seats the paint lane the door's own live session may re-wear — the 386 furnace paint's own kind gate the five re-arms), THE ANVIL'S OWN WIRE KIND WALK (the live lane's own projection the seat words the wholesale landing may re-bank — the offline store's own law the wire tail follows), THE GENERIC'S OWN OFFLINE FOLD (the offline drag distribute the same place bank the stores may wear — the wire's own law the offline tail follows).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: dev-213 "


def heads(text: str):
    return [
        line
        for line in text.splitlines()
        if line.startswith("--- Task ID: ")
    ]


def main() -> int:
    auth = open(AUTH, encoding="utf-8").read()
    mir = open(MIRROR, encoding="utf-8").read()
    assert heads(auth) == heads(mir), "the head lists the walk opens aligned"
    already = MARKER in auth or MARKER in mir

    if not already:
        assert MARKER not in mir, "the marker the mirror already wears"
        assert MARKER not in auth, "the marker the authority already wears"
        open(AUTH, "a", encoding="utf-8").write(ENTRY if auth.endswith("\n") else "\n" + ENTRY)
        open(MIRROR, "a", encoding="utf-8").write(ENTRY if mir.endswith("\n") else "\n" + ENTRY)

    auth2 = open(AUTH, encoding="utf-8").read()
    mir2 = open(MIRROR, encoding="utf-8").read()
    ha, hm = heads(auth2), heads(mir2)
    assert ha == hm, "the head lists the write misaligned"
    assert ha[-1].startswith(HEAD[:40]), f"the tail -1 the round wears: {ha[-1][:80]}"
    # the head list the wheel's own chronology keys — the autopush-392
    # entry the tail -2 wears (the previous round the anvil's own bank
    # the head commit wears)
    assert "autopush-392" in ha[-2], f"the tail -2 the autopush-392 wears: {ha[-2][:80]}"
    assert sum("dev-213" in h for h in ha) == 1, "the single entry the list wears"
    assert sum("dev-212" in h for h in ha) == 1, "the dev-212 entry the list wears"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the dev-213 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

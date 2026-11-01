#!/usr/bin/env python3
"""autopush-392 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..391
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the head count the 71+17 alignment the calibration wore)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-392 (2026-11-01 09:04:14 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611010806) — v0.3.1183 "
    "THE ANVIL'S OWN KIND BANK"
)

BODY = """
- Calibration: HEAD 43931bcd clean + origin-synced (the autopush-391 round's own mirror commit the head wears); the worklog's 71 autopush + 17 dev entries the count keeps (dev-212 the last dev, autopush-391 the autopush side's own last), the wheel the autopush-392 rotates. Chosen step: THE ANVIL'S OWN KIND BANK (the 391 NEXT list's own first walk — the repair walk's own trio the kind words the plain model never wore, the dev-196 comps store the second axis the kinds join, the 383 place-bank + the dev-204 four-arm family's own law the anvil's own seats complete).
- THE KIND'S OWN SEAT STORE (the anvil_seat_potion_kinds the [40] lane the SAME menu numbering keys wears — 0/1 the input seats, 2 the result, 3..38 the mirror rows the items_potion_kinds bank reads [the store the transient seats alone wears], 39 the carried cursor): the SEAT KIND READ the id-gated normalized word the trio + the carried answers (the stale word the non-potion seat the honest NONE keeps), the mirror rows the row_potion_kind's own bank read speaks, the SEAT KIND WRITE the same split the banks serve, the open's own FRESH LAY the wordless open keeps, the ctor the fresh-empty lays.
- THE WALKS' OWN WORDS (the family's own laws the anvil's own seats wear): the CLICK'S OWN FOUR-ARM RIDE (the dev-204 law re-wears — the reads the hoist pre-keys, the left press the predict_pickup_kind the same-id kind pair the vanilla swap answers, the PICKUP the seat's word the carried banks, the PLACE the word the seat lays, the MODEL'S OWN SWAP the pairs exchange, the MERGE the emptied carried the NONE wears; the trio the SEAT STORE banks, the mirror rows the INVENTORY bank the write-through keeps), the RESULT'S OWN WORD (the create walk the copy's own kind wears, the four EMPTY verdicts the word never banks, the success tail the result seat lays; the TAKE the word the carried hands, the consumed seats the words retire), the RIDE-HOME'S OWN WORD (the close walk the seat's kind the first-empty placement banks, the ridden seat the word retires, the close's result death the word the compute never banks), the DRAG'S OWN BANK (the 383 law re-wears — the BEFORE-COUNTS the after-walk reads, the pre-drag word the honest order reads, the empty seat the placed stack's word banks, the PAINT + the COLLECT the same-id kind pair the component half refuses, the emptied carried the NONE retires), the THROW'S OWN RETIRE + the WIRE'S OWN NONE WALL (the live landing the correction the kind never carries) + the TOOLTIP'S OWN READ (the hovered seat's kind the store answers — the NONE identity the plain seats keep).
- TESTS: part_11.rs NEW ONE (the_anvil_kind_bank_the_words_the_walks_wear — the eight arms wear: the place's own bank, the pickup's own ride, the mirror row's own bank, the same-id kind pair's own swap, the result's own hand, the empty verdict's own retire, the ride-home's own word, the drag's own bank + the paint gate). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1597/1435 passed, each rc=0, 0 failed — the counts the 391 baseline +1 the w11 the anvil's own bank, the other windows the 391 numbers keep; the killed six-window walk's own GATED-mod lesson the round banks — the write_mod the already-gated line never un-gates, the restored clean mod the walk re-opens) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the anvil kind bank + the anvil tooltip/door/rename/wire + the wire generic place + the wire hopper door + the wire table/dispenser/chest/furnace places + the wire paint + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip/drag rides' own thirty-one — 31 passed, 0 failed, scripts/directed_392.py the 31st filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the anvil kind bank the live session's own seat walks alone voices, the golden scenes' wordless anvil never opens, the NONE identity the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_392.py — the seat store + the helpers + the fresh lay + the ride-home word + the close death + the create's copy kind + the four arms + the take hand + the throw retire + the tooltip read + the click hoist + the four-arm ride + the paint gate + the distribute bank + the wire wall + the test tail, ALL-REGIONS-CLEAN). Bump 0.3.1182 -> 0.3.1183 (bump_version.py the workspace version + lock wear). Commit 17f1ed64 pushed (43931bcd..17f1ed64 main).
- NEXT CANDIDATES: THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend, the anvil's own hand the same walk re-wears), THE WIRE HOPPER'S OWN PAINT GATE (the hopper's five seats the paint lane the door's own live session may re-wear — the 386 furnace paint's own kind gate the five re-arms), THE ANVIL'S OWN WIRE KIND WALK (the live lane's own projection the seat words the wholesale landing may re-bank — the offline store's own law the wire tail follows), THE GENERIC'S OWN OFFLINE FOLD (the offline drag distribute the same place bank the stores may wear — the wire's own law the offline tail follows).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-392 "


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
    # the head list the wheel's own chronology keys — the autopush-391
    # entry the tail -2 wears (the previous round the anvil's own bank
    # the head commit wears)
    assert "autopush-391" in ha[-2], f"the tail -2 the autopush-391 wears: {ha[-2][:80]}"
    assert sum("autopush-392" in h for h in ha) == 1, "the single entry the list wears"
    assert sum("dev-212" in h for h in ha) == 1, "the dev-212 entry the list wears"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the autopush-392 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

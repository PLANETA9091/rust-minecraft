#!/usr/bin/env python3
"""autopush-386 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..autopush-385
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the autopush-385 entry the tail -2 the pair wears)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-386 (2026-11-01 03:21:04 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611010321) — v0.3.1176 "
    "THE WIRE FURNACE PAINT'S OWN KIND GATE"
)

BODY = """
- Calibration: HEAD 3c9cc1c6 clean + origin-synced (the autopush-385 round's own mirror commit the head wears); the worklog's 65 autopush + 16 dev entries the count keeps (autopush-385 the last), the wheel the autopush-386 rotates. Chosen step: THE WIRE FURNACE PAINT'S OWN KIND GATE (the autopush-385 NEXT list's own third walk — the 385 bank the drag's own lane re-wears, the qc paint the wire world the same component half the click walk answers; the `!world_is_network` wall the paint still wore).
- THE PAINT'S OWN GATE (the 382 round's component half the wire wall wore — the 385 bank the wall lifts): the carried word the field banks (the wire walk the open the fresh door re-banks, the close the word retires), the seat rows the items bank's mapped word the shared helper answers — the BOTH lanes the same verdict wears (the trio the kindless store the dev-208 wall the map's None arm keeps, the non-potion id the stale word never keys). THE WIRE DISTRIBUTE'S OWN WORDS (the 384 round's signature the hardcoded NONE the caller wore — the real words the ride answers): the carried word the field reads, the seat words the +6/-30 shifts the items bank lends, the helper's own component half the REAL verdict speaks; the tail the chest's own law re-wears (the emptied carried the NONE retires, the remainder the same stack the word keeps).
- TESTS: part_11.rs NEW ONE (the_wire_furnace_paint_the_kind_word_refuses_wears — the six arms wear: the wire paint the kinded pair refuses [the 385 bank the gate lifts], the same-kind pair the paint joins, the empty seat no false refusal wears + the release the wire distribute the place serves [the remainder the word keeps], the whole-stack place the tail the word retires, the seeded drag the distribute's own gate the real words the refusal speaks, the plain id the stale word never keys the paint joins). Verdict: green FIRST RUN (the one walk the test's own word reset wore — the arm the word re-banks, the green the honest verdict wears).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1590/1435 passed, each rc=0, 0 failed — the counts the autopush-385 baseline +1 the w11 the wire paint's own gate, the other windows the 385 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the wire paint + the wire furnace + the wire dispenser + the wire chest + the wire table + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own twenty-four — 24 passed, 0 failed, scripts/directed_373.py the 24th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the drag lane's refusals alone voice, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_386.py — the paint's kind gate + the wire distribute's honest words + the part_11 tail, ALL-REGIONS-CLEAN). Bump 0.3.1175 -> 0.3.1176 (bump_version.py the workspace version + lock wear). Commit 132480f2 pushed (3c9cc1c6..132480f2 main).
- NEXT CANDIDATES: THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend), THE WIRE FURNACE PLACE'S OWN WORD BANK (the wire drag's own place the empty seat the placed stack's word banks — the offline fold's own law the wire tail may re-wear [the 386 round the reads the honest words wears, the write the bank never wears]), THE WIRE CHEST'S OWN FOLD SYNC (the quick-move + the drag folds the chest's wire walk the same kind words the lane may bank — the 384 folds' own reads the field rides may follow).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-386 "


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
    # the head list the wheel's own chronology keys — the autopush-385
    # entry the tail -2 wears, the autopush-386 the pair closes
    assert "autopush-385" in ha[-2], f"the tail -2 the autopush-385 wears: {ha[-2][:80]}"
    assert sum("autopush-386" in h for h in ha) == 1, "the single 386 head the list wears"
    assert sum("autopush-385" in h for h in ha) == 1, "the autopush-385 entry the list wears"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the autopush-386 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

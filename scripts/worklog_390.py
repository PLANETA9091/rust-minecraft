#!/usr/bin/env python3
"""autopush-390 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..autopush-389
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the head count the 69+16 alignment the calibration wore)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-390 (2026-11-01 05:51:08 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611010555) — v0.3.1180 "
    "THE WIRE TABLE PLACE'S OWN WORD BANK"
)

BODY = """
- Calibration: HEAD d729e612 clean + origin-synced (the autopush-389 round's own mirror commit the head wears); the worklog's 69 autopush + 16 dev entries the count keeps (autopush-389 the last), the wheel the autopush-390 rotates. Chosen step: THE WIRE TABLE PLACE'S OWN WORD BANK (the 389 NEXT list's own first walk — the 389 dispenser's own lane the place law wore, the table's wire distribute the same reads the honest words wear, the write the bank the wire caller never wore; the 389 dispenser's own law the table's tail re-wears).
- THE PLACE'S OWN WORD BANK (the 389 dispenser's own law the table's wire caller answers — the chest + the dispenser walks the bank the empty row the placed stack's own word banks, the table's wire lane the same law never wore): the painted seats' own BEFORE-COUNTS the snapshot speaks (the helper the same cells the place math mutates), the after-walk the empty store row (old.1 == 0) the placed stack's own word banks (the mapped row the menu-1/-37 shifts the closure's own mapping answers [menu 10..36 the main rows 9..35 the menu-1 speaks, the hotbar 37..45 the -37 shift keeps — the same mapping the seat_kinds reads wear], the occupied row the merge the original's word keeps [the write never wears], the GRID NINE the kindless wall the write never wears [the craft cells the transient wall keeps — the 0 result seat the mayPlace wall keeps — the menu >= 10 gate the bank wears]), the PRE-RETIRE word the honest order reads (the bank the carried_kind copy the pre-drag word wears, the drag's own word retire the AFTER the emptied carried the NONE wears — the split's emptied carried the words the rows the retire never spends).
- TESTS: part_11.rs NEW ONE (the_wire_table_place_the_kind_word_banks_wears — the six arms wear: the store row's own bank [the cap-1 place the row the word wears, the remainder the cursor keeps], the store merge the original's word keeps, the split the both rows' words banks [the emptied carried the pre-retire word the honest order banks], the hotbar seat the stale word retires [the -37 shift the mapped row answers], the refused pair the bank never wears [the distribute's own gate the no place the row's word keeps], the kindless grid the sentinel row the wrong-row write never keys). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1594/1435 passed, each rc=0, 0 failed — the counts the autopush-389 baseline +1 the w11 the wire table's own bank, the other windows the 389 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the wire table place + the wire dispenser place + the wire chest place + the wire furnace place + the wire paint + the wire furnace + the wire dispenser + the wire chest + the wire table + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own twenty-eight — 28 passed, 0 failed, scripts/directed_390.py the 28th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the drag lane's bank walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_390.py — the table's place word bank + the part_11 tail, the &10 assert the 60-column wall the one-line collapse wears, ALL-REGIONS-CLEAN). Bump 0.3.1179 -> 0.3.1180 (bump_version.py the workspace version + lock wear). Commit 3f208965 pushed (d729e612..3f208965 main).
- NEXT CANDIDATES: THE WIRE HOPPER'S OWN PLACE WORD BANK (the hopper's wire walk the same place law the family tail may re-wear — the transfer's own lanes the same bank may follow), THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend), THE WIRE GENERIC'S OWN PLACE WORD BANK (the generic window-0's wire distribute the same place law the store rows may bank — the 382 folds' own reads the write may follow).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-390 "


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
    # the head list the wheel's own chronology keys — the autopush-389
    # entry the tail -2 wears (the 389 round the dispenser's own bank
    # the head commit wears)
    assert "autopush-389" in ha[-2], f"the tail -2 the autopush-389 wears: {ha[-2][:80]}"
    assert sum("autopush-390" in h for h in ha) == 1, "the single entry the list wears"
    assert sum("autopush-389" in h for h in ha) == 1, "the 389 entry the list wears"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the autopush-390 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

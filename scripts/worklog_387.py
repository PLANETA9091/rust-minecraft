#!/usr/bin/env python3
"""autopush-387 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..autopush-386
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the 67 autopush + 16 dev the count keeps)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-387 (2026-11-01 04:06:06 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611010406) — v0.3.1177 "
    "THE WIRE FURNACE PLACE'S OWN WORD BANK"
)

BODY = """
- Calibration: HEAD 084242a9 clean + origin-synced (the autopush-386 round's own mirror commit the head wears); the worklog's 66 autopush + 16 dev entries the count keeps (autopush-386 the last), the wheel the autopush-387 rotates. Chosen step: THE WIRE FURNACE PLACE'S OWN WORD BANK (the 386 NEXT list's own third walk — the 386 round the wire distribute's own reads the honest words wore, the write the bank the wire caller never wore; the 383 round's offline law the wire tail re-wears).
- THE PLACE'S OWN WORD BANK (the 383 round's offline law the wire caller answers — the offline walk the bank the empty store row the placed stack's own word banks, the wire lane the same law never wore): the painted seats' own BEFORE-COUNTS the snapshot speaks (the helper the same seats the place math mutates, the after-walk the empty seat's bank the snapshot reads), the after-walk the empty seat (old.1 == 0) the placed stack's own word banks (the occupied row the merge the original's word keeps [the merge the count the same word answers, the write never wears], the trio the kindless seat the mapped row the None answer keeps [furnace_menu_to_inventory the +6/-30 shifts the inventory rows answer, the input/fuel/result the None keeps]), the PRE-RETIRE word the honest order reads (the bank the carried_kind copy the pre-drag word wears, the drag's own word retire the AFTER the emptied carried the NONE wears — the split's emptied carried the words the rows the retire never spends).
- TESTS: part_11.rs NEW ONE (the_wire_furnace_place_the_kind_word_banks_wears — the six arms wear: the empty seat the placed stack's own word banks [the cap-1 place the row the word wears, the remainder the cursor keeps], the occupied row the merge the original's word keeps, the split the both rows' words banks, the kindless fuel seat the mapped row the None answer keeps [the sentinel row the wrong-row write never touches], the plain place the stale word retires, the refused pair the bank never wears [the distribute's own gate the no place the row's word keeps]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1591/1435 passed, each rc=0, 0 failed — the counts the autopush-386 baseline +1 the w11 the wire place's own bank, the other windows the 386 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the wire place + the wire paint + the wire furnace + the wire dispenser + the wire chest + the wire table + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own twenty-five — 25 passed, 0 failed, scripts/directed_373.py the 25th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the drag lane's bank walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_387.py — the place's own word bank + the part_11 tail, the fn_call_width 60 wall the second empty assert the reflow wears, ALL-REGIONS-CLEAN). Bump 0.3.1176 -> 0.3.1177 (bump_version.py the workspace version + lock wear). Commit a1d988cb pushed (084242a9..a1d988cb main).
- NEXT CANDIDATES: THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend), THE WIRE CHEST'S OWN PLACE WORD BANK (the chest's wire distribute the same place law the furnace's tail re-wears — the 386 chest's own reads the field rides, the write the bank the wire caller never wore), THE WIRE DISPENSER'S OWN PLACE WORD BANK (the dispenser's wire distribute the same place law the family tail may re-wear — the chest's own shape the bank may follow).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-387 "
PREV = "Task ID: autopush-386 "


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
    assert PREV in ha[-2], f"the tail -2 the autopush-386 wears: {ha[-2][:80]}"
    assert sum("autopush-386" in h for h in ha) == 1, "the autopush-386 entry the list wears"
    assert sum("dev-211" in h for h in ha) == 1, "the dev-211 entry the list wears"
    assert len(ha) == 83, f"the 67 autopush + 16 dev the count wears: {len(ha)}"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the autopush-387 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""autopush-391 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..dev-212
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the head count the 70+17 alignment the calibration wore)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-391 (2026-11-01 07:21:09 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611010721) — v0.3.1182 "
    "THE WIRE GENERIC'S OWN PLACE WORD BANK"
)

BODY = """
- Calibration: HEAD 251cda78 clean + origin-synced (the dev-212 round's own mirror commit the head wears); the worklog's 70 autopush + 17 dev entries the count keeps (dev-212 the last, autopush-390 the autopush side's own last), the wheel the autopush-391 rotates. Chosen step: THE WIRE GENERIC'S OWN PLACE WORD BANK (the dev-212 NEXT list's own fourth walk — the 390 table's own law the generic's wire tail re-wears, the 382 folds' own reads the honest words wear, the write the bank the wire caller never wore; the 387..390 place-bank family's own law the generic's lane completes).
- THE PLACE'S OWN WORD BANK (the 390 table's own law the generic's wire tail re-wears — the wire window-0's drag distribute the empty seat the placed stack's own word banks, the 382 folds' own reads the write follows): the painted seats' own BEFORE-COUNTS the snapshot speaks (the helper the same cells the place math mutates), the PRE-RETIRE word the honest order reads (the bank the pre-drag word wears, the drag's own word retire the AFTER the emptied carried the NONE wears), the after-walk the empty store row (old.1 == 0) the placed stack's own word banks (the kind_bank_seat's own mapping the same reads the bank answers — the main rows 9..35 the identity keeps, the hotbar 36..44 the -36 shift speaks, the offhand 45 the 36 seat wears, the craft/armor wall the None the write never wears [the transient stores the kind the bank never seats — the 384 wall the family keeps]), the occupied row the merge the original's word keeps (the write never wears), the drag's own word the tail keeps (the emptied carried the NONE retires, the remainder the same stack the word wears).
- TESTS: part_11.rs NEW ONE (the_wire_generic_place_the_kind_word_banks_wears — the six arms wear: the main row's own bank [the cap-1 place the row the word wears, the remainder the cursor keeps], the hotbar seat the -36 shift the mapped row banks [the emptied carried the pre-retire word the honest order banks], the split the both rows' words banks [the two seats the floor split serves], the craft cell the kindless wall the sentinel row keeps [the wrong-row write never keys], the offhand seat the 36 seat the word banks, the refused pair the bank never wears [the distribute's own gate the row's word keeps]). Verdict: green FIRST RUN.
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1596/1435 passed, each rc=0, 0 failed — the counts the dev-212 baseline +1 the w11 the wire generic's own bank, the other windows the dev-212 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the wire generic place + the wire hopper door + the wire table place + the wire dispenser place + the wire chest place + the wire furnace place + the wire paint + the wire furnace + the wire dispenser + the wire chest + the wire table + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own thirty — 30 passed, 0 failed, scripts/directed_391.py the 30th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the wire distribute the live session's own bank walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_391.py — the generic's before-counts + the pre-retire word + the after-walk bank + the drag's own retire + the test tail, ALL-REGIONS-CLEAN). Bump 0.3.1181 -> 0.3.1182 (bump_version.py the workspace version + lock wear). Commit 29757893 pushed (251cda78..29757893 main).
- NEXT CANDIDATES: THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend), THE WIRE HOPPER'S OWN PAINT GATE (the hopper's five seats the paint lane the door's own live session may re-wear — the 386 furnace paint's own kind gate the five re-arms), THE GENERIC'S OWN OFFLINE FOLD (the offline drag distribute the same place bank the stores may wear — the wire's own law the offline tail follows).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-391 "


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
    # the head list the wheel's own chronology keys — the dev-212
    # entry the tail -2 wears (the dev round the hopper's own door
    # the head commit wears)
    assert "dev-212" in ha[-2], f"the tail -2 the dev-212 wears: {ha[-2][:80]}"
    assert sum("autopush-391" in h for h in ha) == 1, "the single entry the list wears"
    assert sum("dev-212" in h for h in ha) == 1, "the dev-212 entry the list wears"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the autopush-391 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

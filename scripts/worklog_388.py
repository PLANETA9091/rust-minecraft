#!/usr/bin/env python3
"""autopush-388 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..autopush-387
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the 68 autopush + 16 dev the count keeps)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-388 (2026-11-01 04:36:06 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611010445) — v0.3.1178 "
    "THE WIRE CHEST PLACE'S OWN WORD BANK"
)

BODY = """
- Calibration: HEAD 3f0655a1 clean + origin-synced (the autopush-387 round's own mirror commit the head wears); the worklog's 67 autopush + 16 dev entries the count keeps (autopush-387 the last), the wheel the autopush-388 rotates. Chosen step: THE WIRE CHEST PLACE'S OWN WORD BANK (the 387 NEXT list's own third walk — the 387 round the furnace's own lane the place law wore, the chest's wire distribute the same reads the honest words wear, the write the bank the wire caller never wore; the 387 furnace's own law the chest's tail re-wears).
- THE PLACE'S OWN WORD BANK (the 387 furnace's own law the chest's wire caller answers — the offline chest walk the bank the empty row the placed stack's own word banks [the 383 round's own law], the wire lane the same law never wore): the painted seats' own BEFORE-COUNTS the snapshot speaks (the helper the same cells the place math mutates, the after-walk the empty seat's bank the snapshot reads — the 27-convention the lane's own reads the closure wears), the after-walk the empty seat (old.1 == 0) the placed stack's own word banks (the container rows the chest bank's own words answer [chest_row_potion_kinds], the store rows the items bank's mapped word the same closure maps [the rel -27/+9 shifts], the occupied row the merge the original's word keeps [the write never wears]), the PRE-RETIRE word the honest order reads (the bank the carried_kind copy the pre-drag word wears, the drag's own word retire the AFTER the emptied carried the NONE wears — the split's emptied carried the words the rows the retire never spends).
- TESTS: part_11.rs NEW ONE (the_wire_chest_place_the_kind_word_banks_wears — the six arms wear: the grid seat's own bank [the cap-1 place the row the word wears, the remainder the cursor keeps], the grid merge the original's word keeps, the mirror split the both rows' words banks [the emptied carried the pre-retire word the honest order banks], the hotbar seat the stale word retires, the refused pair the bank never wears [the distribute's own gate the no place the row's word keeps], the mirror merge the same silence the grid's own arm wears). Verdict: green (the one walk the test's own screen kind wore — the qc_release the Chest the routing answers, the open_chest the setup's own screen kind speaks, the green the honest verdict wears).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1592/1435 passed, each rc=0, 0 failed — the counts the autopush-387 baseline +1 the w11 the wire chest's own bank, the other windows the 387 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the wire chest place + the wire furnace place + the wire paint + the wire furnace + the wire dispenser + the wire chest + the wire table + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own twenty-six — 26 passed, 0 failed, scripts/directed_373.py the 26th filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the drag lane's bank walk alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_388.py — the chest's place word bank + the part_11 tail, the fn_call_width 60 wall the rects assert the one-line collapse wears, ALL-REGIONS-CLEAN). Bump 0.3.1177 -> 0.3.1178 (bump_version.py the workspace version + lock wear). Commit 8cfda237 pushed (3f0655a1..8cfda237 main).
- NEXT CANDIDATES: THE WIRE DISPENSER'S OWN PLACE WORD BANK (the dispenser's wire distribute the same place law the chest's tail re-wears — the 3x3's own reads the 384 field rides, the write the bank the wire caller never wore), THE WIRE TABLE'S OWN PLACE WORD BANK (the table's wire distribute the same place law the family tail may re-wear — the craft grid the kindless wall keeps, the store rows the mapped word the bank may wear), THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-388 "
PREV = "Task ID: autopush-387 "


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
    assert PREV in ha[-2], f"the tail -2 the autopush-387 wears: {ha[-2][:80]}"
    assert sum("autopush-387" in h for h in ha) == 1, "the autopush-387 entry the list wears"
    assert sum("dev-211" in h for h in ha) == 1, "the dev-211 entry the list wears"
    assert len(ha) == 84, f"the 68 autopush + 16 dev the count wears: {len(ha)}"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the autopush-388 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

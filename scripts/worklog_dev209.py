#!/usr/bin/env python3
"""dev-209 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..384 record's
own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: dev-209 (2026-10-31 23:05:12 +08:00, Job 384925, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202610312305) — v0.3.1172 "
    "THE WIRE TABLE'S OWN KIND SYNC"
)

BODY = """
- Calibration: HEAD 3c39cf98 clean + origin-synced (the 384 round's own mirror commit the head wears); the worklog's 64 autopush + 14 dev entries the count keeps (autopush-384 + dev-208 the last pair), the wheel the dev-209 rotates. Chosen step: THE WIRE TABLE'S OWN KIND SYNC (the 384 NEXT list's own first walk — the live table lane the carried word the field banks, the 384 wall's own silence the field voices; the dev-208 NEXT's own third candidate the same lane names).
- THE FIELD (live_table_carried_potion_kind — the chest's own field the table's walk re-wears: the pickup rides the kind the cursor banks, the place the word the seat lays, the swap the pairs exchange, the emptied carried the NONE retires; the syncs the wholesale replacement the NONE wall resets [the chest's own law the correction the kind never carries — the Set Content walk, the window's death the close + the no-window resets wear, the double-click collect's own wall the cell's stack the word never names]). THE RIDES (the shared free fn ride_pickup_kind the four arms the before/after pairs move): the store rows' own pickup (wire_table_store_pickup the seat word the caller's own bank seat answers — the main rows 10..36 the items 9..35 the menu-1 speaks, the hotbar 37..45 the items 0..8 the -37 shift keeps — the same mapping the offline fold's reads wear), the craft cells' own wall ride (wire_table_cell_pickup the None arm the clear alone answers — the take the honest NONE the wall keeps), the result take's own crafted wall (the recipe's output the kind never carries — the stale word the honest NONE replaces, the merge the same stack the cursor's word keeps), the throw's own kind ride (the whole drop the kind retires, the split the remainder the same word keeps). THE RIDE'S OWN COUNT KEY (the shared fn's merge/swap discrimination the id equality alone never keyed — the same-id kinded pair the predict's own swap the exchange serves: the merge the seat's count growth keys, the merge remainder the cursor's count drain keys, the different ids the swap branch the honest words exchange).
- TESTS: part_11.rs NEW ONE (the_wire_table_the_kind_word_banks_wears — the seven arms wear: the store row's take the seat's kind the word banks, the place the word the seat lays the emptied cursor retires, the swap the pairs the words exchange, the craft cell's wall take the honest NONE wears, the result take's crafted wall the stale word replaces, the throw the whole drop retires the split keeps, the 384 wall's own voice the store row's real word the gate answers the wordless cell the kinded carried refuses). Verdict: green (the directed families the fmt folds the final code re-verifies — 20 passed both walks).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1586/1435 passed, each rc=0, 0 failed — the counts the 384 baseline +1 the w11 the wire table, the other windows the 384 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the wire table + the wire folds + the drag place + the drag collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own twenty — 20 passed, 0 failed, scripts/directed_373.py the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the wire table's kind words alone voice, the golden scenes' wordless seats the NONE-identity law the bytes never move). FMT: the region law (scripts/fmt_probe_dev209.py — the field + the rides + the callers + the resets + the ride's own count key + the part_11 tail, the throw's if-fold the single line rustfmt answers, the place assert the multi-line fold wears, the probe the ALL-REGIONS-CLEAN verdict wears). Bump 0.3.1171 -> 0.3.1172 (bump_version.py the workspace version + lock wear). Commit 42de3d0d pushed (3c39cf98..42de3d0d main).
- NEXT CANDIDATES: THE WIRE CHEST'S OWN PICKUP RIDE (the chest's wire click the plain predict the kind words never rides — the generic's own shape the wire_chest walk may re-wear, the carried word the field the rides bank), THE WIRE DISPENSER'S OWN PICKUP RIDE (the same plain walk the dispenser's field the rides may bank), THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: dev-209 "
PREV = "Task ID: dev-208 "


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
    # the head list the wheel's own chronology keys — the autopush-384
    # entry the 22:05 walk the tail -2 wears, the dev-208 the pair before
    assert "autopush-384" in ha[-2], f"the tail -2 the 384 wears: {ha[-2][:80]}"
    assert sum("dev-208" in h for h in ha) == 1, "the dev-208 entry the list wears"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the dev-209 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

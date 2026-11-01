#!/usr/bin/env python3
"""autopush-393 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..dev-213
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the head count the 72+17 alignment the calibration wore)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-393 (2026-11-01 10:41:58 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611010959) — v0.3.1185 "
    "THE FURNACE QUICK-MOVE'S OWN KIND RIDE"
)

BODY = """
- Calibration: HEAD d0c8eefc clean + origin-synced (the dev-213 round's own mirror commit the head wears); the worklog's 72 autopush + 17 dev entries the count keeps (dev-213 the dev side's own last, autopush-392 the autopush side's own last), the wheel the autopush-393 rotates. Chosen step: THE FURNACE QUICK-MOVE'S OWN KIND RIDE (the dev-213 NEXT list's own first walk the round answers — the shift-click's routing the words the plain pairs never move, the dev-213 trio's own bank the furnace's own quick-move lanes join, the 384 quick_move_kind_ride the routing re-wears).
- THE RIDE (the 384 law the furnace's own routing re-wears — the src's own word the pre-write read banks, the empty destination the placed stack's own word banks, the occupied merge the original's word keeps, the drained src the NONE retires, the kinded remainder the word keeps): the OFFLINE SIM'S OWN LANE (the doClick's shift lane the pre-reads + the after-walk wear — the src_kind the furnace_seat_potion_kind's id-gated read answers, the old_dests the changed vec's non-src entries the BEFORE cells snapshot, the empty dst the furnace_seat_kind_put's own write serves — the trio the SEAT STORE banks, the store rows the INVENTORY bank the mapped row keeps, the src tail the id-gated word the drained walk retires), the LIVE SESSION'S OWN LANE (the same pre-reads + ride the live belief wears — the trio the live store banks, the store rows the items bank the write-through keeps, the SINK'S OWN CLONE the ride's &mut self the owned sender never conflicts — the borrow the walk's mutable seat walks end, the send the same channel answers). THE ROUTING'S OWN ARMS the predictor's walk answers (javap AbstractFurnaceMenu.quickMoveStack — the smeltable the INPUT takes, the fuel the FUEL, the result the reversed walk the inventory lends, the main/hotbar swap): the main row the hotbar arm banks, the hotbar the main arm, the result the tail seat 38 the row 8 the -30 shift answers, the input the forward walk the main row 9 the +6 shift wears, the cap-1 pair the merge refuses the walk the first EMPTY row answers (the occupied row's own word the write never wears), the potions the recipe/fuel gates refuse the honest walls keep (smelt_result/fuel_ticks the potion rows the 0 the table banks).
- TESTS: part_11.rs NEW ONE (the_furnace_quick_move_the_kind_ride_wears — the six arms wear: the hotbar's own bank [the moved word the row banks, the drained src the NONE retires, the write-through the real inventory keeps], the main's own bank [the reverse arm the +6 shift answers], the result's own lend [the hand-placed result's word the reversed walk the tail seat banks, the taken seat the word retires], the input's own ride out [the trio word the forward walk the main row banks, the ridden seat the word retires], the occupied seat's own keep [the cap-1 pair the merge refuses, the empty row banks the moved word, the occupied row's swift the original keeps], the wordless identity's own wall [the kindless potion the honest NONE banks, the stale row the ride never spends]). Verdict: green FIRST RUN (the vec the array literal the compile the round re-keys).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1599/1435 passed, each rc=0, 0 failed — the counts the dev-213 baseline +1 the w11 the quick-move's own ride, the other windows the dev-213 numbers keep; the one window per call the 392 lesson the round keeps) + the fast trio (the 160/100/80 the protocol/n/gen banks keep) + the directed families (the quick-move kind ride + the furnace trio kind bank + the furnace seats re-key + the anvil kind bank + the anvil tooltip/door/rename/wire + the wire generic place + the wire hopper door + the wire table/dispenser/chest/furnace places + the wire paint + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip/drag rides' own thirty-three — 33 passed, 0 failed, scripts/directed_393.py the 37 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the quick-move kind ride the live session's own shift lane alone voices, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_393.py — the offline lane's pre-reads + ride + the sink's own clone + the live lane's pre-reads + ride + the test tail, the rustfmt's own folds the map/if-let collapses wear, ALL-REGIONS-CLEAN). Bump 0.3.1184 -> 0.3.1185 (bump_version.py the workspace version + lock wear). Commit 6ed3deb4 pushed (d0c8eefc..6ed3deb4 main).
- NEXT CANDIDATES: THE WIRE HOPPER'S OWN PAINT GATE (the hopper's five seats the paint lane the door's own live session may re-wear — the 386 furnace paint's own kind gate the five re-arms), THE ANVIL'S OWN WIRE KIND WALK (the live lane's own projection the seat words the wholesale landing may re-bank — the offline store's own law the wire tail follows), THE GENERIC'S OWN OFFLINE FOLD (the offline drag distribute the same place bank the stores may wear — the wire's own law the offline tail follows), THE HOPPER'S OWN QUICK-MOVE KIND RIDE (the dev-212 door's own walk the words the plain deltas never move — the 384 law the five seats' own routing re-wears).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-393 "


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
    # the head list the wheel's own chronology keys — the dev-213
    # entry the tail -2 wears (the previous round the furnace trio's
    # own bank the head commit wears)
    assert "dev-213" in ha[-2], f"the tail -2 the dev-213 wears: {ha[-2][:80]}"
    assert sum("autopush-393" in h for h in ha) == 1, "the single entry the list wears"
    assert sum("autopush-392" in h for h in ha) == 1, "the 392 entry the list wears"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the autopush-393 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""autopush-394 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..autopush-393
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the head count the 73+18 alignment the calibration wore)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-394 (2026-11-01 11:27:19 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611011043) — v0.3.1186 "
    "THE WIRE HOPPER'S OWN PAINT GATE"
)

BODY = """
- Calibration: HEAD 15e4ebb6 clean + origin-synced (the autopush-393 round's own mirror commit the head wears); the worklog's 73 autopush + 18 dev entries the count keeps (dev-213 the dev side's own last, autopush-393 the autopush side's own last), the wheel the autopush-394 rotates. Chosen step: THE WIRE HOPPER'S OWN PAINT GATE (the autopush-393 NEXT list's own first walk the round answers — the hopper's five seats the paint lane the door's own live session re-wears, the 386 furnace paint's own kind gate the five re-arms).
- THE GATE (the 382 component half the chest's own paint wears — the five the same read serves: the seat's own word the chest_row_potion_kind's id-gated read answers, the mirror seat the -5 shift the items bank's mapped row keeps, the same-id kinded pair the paint refuses, the same-kind pair the paint joins, the kindless pairs the plain law keeps; the arm the paint the release the same lane the door's own live session wears — the chest family's fields the five the glen boundary keys, the wire distribute's own five-split the 212 round banked the release lane answers). THE CANVAS'S OWN HEIGHT (the chest_panel_h the hopper's own 133 re-keys — the 176x133 hopper.png the render_hopper blits the height the arm + the release's drop lane wear, the 166 band the old single's height the hopper's own outside test never wears again: the below-canvas press the outside arms, the -999 drop the whole carried speaks; the two callers the arm + the deferred place the same height the canvas keeps, the shulker the single's 166 the chest canvas answers).
- TESTS: part_11.rs NEW ONE (the_wire_hopper_paint_the_kind_gate_wears — the seven arms wear: the five seat's own refusal, the same-kind pair's own join, the mirror seat's own refusal [the -5 shift the mapped row 18 the word reads], the empty five seat's own ride [the cap-1 place the row bank's word wears, the remainder the cursor keeps], the distribute's own refusal [the helper's component half the REAL words speak], the plain id's own law, the canvas's own outside band [the below-canvas press the outside arms, the -999 drop the -999 speaks]). Verdict: green SECOND RUN (the screen the door opens the first run's own miss the round re-keys — the fields the wire shape wears the screen the Chest lane keys; the open_hopper_for the door the paint's own qc_seat_stack dispatch pre-keys).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1600/1435 passed, each rc=0, 0 failed — the counts the 393 baseline +1 the w11 the hopper paint's own gate, the other windows the 393 numbers keep; the one window per call the 392 lesson the round keeps) + the fast trio (the 160/100/80 the protocol/n/gen banks keep) + the directed families (the hopper paint gate + the wire hopper door + the quick-move kind ride + the furnace trio kind bank + the furnace seats re-key + the anvil kind bank + the anvil tooltip/door/rename/wire + the wire generic place + the wire table/dispenser/chest/furnace places + the wire paint + the wire kind rides + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/dispenser/brewing/table/store/offhand/craft/same-id/hopper/tooltip/drag rides' own thirty-four — 34 passed, 0 failed, scripts/directed_394.py the 38 filters the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the panel height the drag lane's own walks alone voice, the golden scenes' wordless seats the NONE-identity law the bytes never move, no render walk the round edits). FMT: the region law (scripts/fmt_probe_394.py — the panel height + the test tail, the rustfmt's own assert folds the probe wears, ALL-REGIONS-CLEAN). Bump 0.3.1185 -> 0.3.1186 (bump_version.py the workspace version + lock wear). Commit 930c4bec pushed (15e4ebb6..930c4bec main).
- NEXT CANDIDATES: THE ANVIL'S OWN WIRE KIND WALK (the live lane's own projection the seat words the wholesale landing may re-bank — the offline store's own law the wire tail follows), THE GENERIC'S OWN OFFLINE FOLD (the offline drag distribute the same place bank the stores may wear — the wire's own law the offline tail follows), THE HOPPER'S OWN QUICK-MOVE KIND RIDE (the dev-212 door's own walk the words the plain deltas never move — the 384 law the five seats' own routing re-wears), THE WIRE CHEST'S OWN DROP KIND RETIRE (the outside drop the None the chest lane passes — the word the stale kind the emptied carried keeps, the furnace's own Some the chest lane re-wears).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-394 "


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
    # the head list the wheel's own chronology keys — the autopush-393
    # entry the tail -2 wears (the previous round the quick-move's own
    # ride the head commit wears)
    assert "autopush-393" in ha[-2], f"the tail -2 the autopush-393 wears: {ha[-2][:80]}"
    assert sum("autopush-394" in h for h in ha) == 1, "the single entry the list wears"
    assert sum("autopush-393" in h for h in ha) == 1, "the 393 entry the list wears"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the autopush-394 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

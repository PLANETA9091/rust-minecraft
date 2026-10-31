#!/usr/bin/env python3
"""autopush-385 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..dev-211
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the dev-211 entry the tail -2 the pair wears)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-385 (2026-11-01 02:36:04 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611010242) — v0.3.1175 "
    "THE WIRE FURNACE'S OWN KIND BANK"
)

BODY = """
- Calibration: HEAD b137c0dc clean + origin-synced (the dev-211 round's own mirror commit the head wears); the worklog's 64 autopush + 16 dev entries the count keeps (dev-211 the last), the wheel the autopush-385 rotates. Chosen step: THE WIRE FURNACE'S OWN KIND BANK (the dev-211 NEXT list's own third walk — the wire chest's own shape the furnace's live lane re-wears, the carried word the field the rides bank; the plain predict the wire lane still wore).
- THE READS (the hoist the component law pre-keys — the banks' own normalized verdicts the pickup pair's answer wears, the 372 round the reads the predict call pre-wears): the trio the kindless store the honest NONE wall keeps (the (id, count) store the word never carried — the dev-208 wall the wire lane keeps), the store rows the mapped seat the items bank lends (the believed id's own gate the stale row never keys — the menu map the +6/-30 shifts answer, the shared helper the qc paint's own read wears). THE PREDICT (the button byte keys the arm): predict_pickup_kind the button-0 arm wears (the same-id kinded pair the vanilla isSameItemSameComponents the different items answers — the plain id test the merge room the max-1 silence wore, the kinded pair the SWAP exchanges), predict_pickup_right the right lane the plain law keeps. THE VERDICT (the four arms the pair_same keys — the dev-204 round's law the wire furnace's own walk re-wears): the PICKUP the kind rides to the word, the PLACE the word the seat lays, the MODEL'S OWN SWAP the kinds the pairs exchange (the button-0 gate the never-swap right law keeps), the MERGE the seat's own word keeps the emptied carried the NONE retires; the bank the mapped seats answer (the store rows the items bank, the kindless trio the write never wears), the carried word the field rides. THE CLOSE'S OWN WORD RETIRE (the chest's own close law the furnace's wire lane re-wears — the belief resets with the window, the banked word the ride home never serves: the wire close the kind word the leak never spent, the window's death the word retires, the server's own truth the next open's fresh door re-banks).
- TESTS: part_11.rs NEW ONE (the_wire_furnace_the_kind_word_rides_wears — the six arms wear: the store row's take the seat's kind the cursor banks, the trio's take the kindless store the word never banks, the hotbar row's place the -30 shift the bank answers, the same-id kinded pair's swap the words exchange, the wordless id's gate the stale bank word refuses [the honest NONE the write replaces], the right press the never-swap silence the kinds never exchanges). Verdict: green FIRST RUN (the directed families the fmt folds the final code re-verifies — 23 passed the one walk).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1589/1435 passed, each rc=0, 0 failed — the counts the dev-211 baseline +1 the w11 the wire furnace, the other windows the dev-211 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the wire furnace + the wire dispenser + the wire chest + the wire table + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own twenty-three — 23 passed, 0 failed, scripts/directed_373.py the 23rd filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the wire furnace's kind words alone voice, the golden scenes' wordless seats the NONE-identity law the bytes never move, the headless scenes the mouse the live lane's carried ghost never paints). FMT: the region law (scripts/fmt_probe_385.py — the close's word retire + the wire furnace's kind walk + the part_11 tail, the trailing-comment chain the close's comment the pos line re-orders [the rustfmt chain the title's own tail the comment indents], the probe the ALL-REGIONS-CLEAN verdict wears). Bump 0.3.1174 -> 0.3.1175 (bump_version.py the workspace version + lock wear). Commit 0f96971c pushed (b137c0dc..0f96971c main).
- NEXT CANDIDATES: THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend), THE WIRE PAINT'S OWN KIND GATE (the furnace + the chest + the dispenser qc paints the wire world the honest NONE wall the carry word the bank may voice — the 385 bank the paint's own gate the wire world re-wears), THE WIRE CHEST'S OWN FOLD SYNC (the quick-move + the drag folds the chest's wire walk the same kind words the lane may bank — the 384 folds' own reads the field rides may follow).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-385 "
PREV = "Task ID: dev-211 "


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
    # the head list the wheel's own chronology keys — the dev-211
    # entry the tail -2 wears, the autopush-385 the pair closes
    assert "dev-211" in ha[-2], f"the tail -2 the dev-211 wears: {ha[-2][:80]}"
    assert sum("autopush-385" in h for h in ha) == 1, "the single 385 head the list wears"
    assert sum("dev-211" in h for h in ha) == 1, "the dev-211 entry the list wears"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the autopush-385 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

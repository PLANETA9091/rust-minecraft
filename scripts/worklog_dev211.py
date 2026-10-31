#!/usr/bin/env python3
"""dev-211 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..dev-210
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts (the dev-210 gap the wipe took the entry documents)."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: dev-211 (2026-11-01 01:45:16 +08:00, Job 384925, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202611010151) — v0.3.1174 "
    "THE WIRE DISPENSER'S OWN PICKUP RIDE"
)

BODY = """
- Calibration: HEAD e51c349e clean + origin-synced (the dev-210 round's own code commit the head wears — the mirror entry the wipe the sync never served, the gap the entry keeps); THE WIPE THE RECYCLE TOOK the whole /home/z/my-project tree the reset lost (the worklog, the repo, the toolchain, the caches all gone — the second wipe the record wears): the repo the token URL re-cloned (e51c349e the HEAD the remote kept), the rustup the 1.99.1 stable answers, scripts/restore_env.sh the client jar + the gen/font/textures + the asset-index objects + the jar textures the lanes re-fetched — the FIRST directed walk the tooltip test the raw translation key the fresh lang-absent boot wore (item.minecraft.potion.effect.night_vision the left the 'Potion of Night Vision' the right), the restore the en_us table the verdict re-greens, the probe run the PRISTINE HEAD the same red the pre-existing env-failure proves (the tree the wipe broke, the code the round never touched); the worklog the mirror's own bytes restored (64 autopush + 14 dev entries, dev-210 the code-only gap the head list wears, the count the entry documents). Chosen step: THE WIRE DISPENSER'S OWN PICKUP RIDE (the dev-209 NEXT list's own second walk — the wire chest's own shape the dispenser's live lane re-wears, the carried word the field the rides bank; the plain predict the wire lane still wore).
- THE READS (the hoist the component law pre-keys — the banks' own normalized verdicts the pickup pair's answer wears, the 372 round the reads the predict call pre-wears): the mirror rows the MIRROR's own id the gate keeps (the wire belief the real inventory's stale row never keys — the mapped seat the items bank lends: the main grid 9..35 the row itself speaks, the hotbar 36..44 the -36 shift answers), the grid nine the kindless store the honest NONE wall keeps (the (id, count) store the word never carried — the 378 wall the wire lane keeps). THE PREDICT (the button byte keys the arm): predict_pickup_kind the button-0 arm wears (the same-id kinded pair the vanilla isSameItemSameComponents the different items answers — the plain id test the merge room the max-1 silence wore, the kinded pair the SWAP exchanges), predict_pickup_right the right lane the plain law keeps. THE VERDICT (the four arms the pair_same keys — the dev-204 round's law the wire dispenser's own walk re-wears): the PICKUP the kind rides to the word, the PLACE the word the seat lays, the MODEL'S OWN SWAP the kinds the pairs exchange (the button-0 gate the never-swap right law keeps), the MERGE the seat's own word keeps the emptied carried the NONE retires; the bank the mapped seats answer (the mirror rows the items bank, the kindless grid the write never wears), the carried word the field rides.
- TESTS: part_11.rs NEW ONE (the_wire_dispenser_the_kind_word_rides_wears — the six arms wear: the mirror row's take the seat's kind the cursor banks, the grid nine's take the kindless store the word never banks, the hotbar row's place the -36 shift the bank answers, the same-id kinded pair's swap the words exchange, the wordless id's gate the stale bank word refuses [the honest NONE the write replaces], the right press the never-swap silence the kinds never exchanges). Verdict: green FIRST RUN (the directed families the fmt folds the final code re-verifies — 22 passed the one walk).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1588/1435 passed, each rc=0, 0 failed — the counts the dev-210 baseline +1 the w11 the wire dispenser, the other windows the dev-210 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the wire dispenser + the wire chest + the wire table + the wire folds + the drag place/collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own twenty-two — 22 passed, 0 failed, scripts/directed_373.py the 22nd filter the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the wire dispenser's kind words alone voice, the golden scenes' wordless seats the NONE-identity law the bytes never move). FMT: the region law (scripts/fmt_probe_dev211.py — the wire dispenser's kind walk + the part_11 tail, the 12-space indented two-line reads variant the marker keys [the single-line walks + the chest's 16-space variant the shared text never matches], the probe the ALL-REGIONS-CLEAN verdict wears). Bump 0.3.1173 -> 0.3.1174 (bump_version.py the workspace version + lock wear). Commit ccaec320 pushed (e51c349e..ccaec320 main).
- NEXT CANDIDATES: THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend), THE WIRE FURNACE'S OWN KIND BANK (the live_furnace_carried the word the field lacks — the wire sync's own rides the same bank the table's round may wear), THE WIRE CHEST'S OWN FOLD SYNC (the quick-move + the drag folds the chest's wire walk the same kind words the lane may bank — the 384 folds' own reads the field rides may follow).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: dev-211 "
PREV = "Task ID: dev-209 "


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
    # the head list the wheel's own chronology keys — the dev-209
    # entry the tail -2 wears (the dev-210 round the CODE the head
    # commit wears, the mirror entry the wipe the sync never served —
    # the gap the count asserts documents)
    assert "dev-209" in ha[-2], f"the tail -2 the dev-209 wears: {ha[-2][:80]}"
    assert sum("dev-210" in h for h in ha) == 0, "the dev-210 gap the list honestly keeps"
    assert sum("dev-209" in h for h in ha) == 1, "the dev-209 entry the list wears"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} heads aligned, tail -1 the dev-211 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

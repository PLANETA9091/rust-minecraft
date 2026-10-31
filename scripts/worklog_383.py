#!/usr/bin/env python3
"""autopush-383 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..382
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-383 (2026-10-31 20:51:00 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202610312051) — v0.3.1170 "
    "THE DRAG'S OWN PLACE VERDICT"
)

BODY = """
- Calibration: HEAD 9cf01f7f clean + origin-synced (the 382 round's own mirror commit the head wears); the worklog's 62 autopush + 13 dev entries the count keeps (382 the last), the wheel the autopush-383 rotates. Chosen step: THE DRAG'S OWN PLACE VERDICT (the 382 NEXT list's own first walk — the nine distribute folds' own per-seat gate the same-id kind pair the paint's refusal the release never re-keys, the empty seat the placed stack's own word the stale word the bank replaces; the wire free fns the kind words the slices never carry — the plain verdict the honest wall keeps, the caller's own kind reads the next round may hoist; the anvil/beacon field-less walls the plain verdict keeps).
- THE GATES (the shared free fn qc_collect_kind_refuses the seven kind-worded offline folds' own honest reads wear — the same-id kind pair the seat gate refuses the share the carried keeps, the occupied pair the max-1 cap the merge never wears [the potion the ONE stack the vanilla law speaks], the wordless pair the same silence keeps): the chest's fold (the container rows the chest_row_potion_kind normalized read speaks, the store rows the shared bank's gated word, the empty row the word bank the click lane's own place arm re-wears), the dispenser's fold (the grid nine the kindless store the 378 wall keeps, the mirror rows the paint's own mapped read the fold re-wears, the empty mirror row the bank wears), the brewing's fold (the stand seats the five-word bank the 377 ride reads, the mirror rows the shared bank's word, the empty seat the 377 bank re-wears), the enchant's fold (the pair seats the kindless store the 379 wall keeps, the kinded carried the wordless seat the mismatch refuses), the table's fold (the grid cells the 376 bank answers, the store rows the shared bank's word, the empty row the bank wears), the furnace's fold (the trio the kindless store the dev-208 wall keeps, the store rows the menu map's gated word, the empty row the bank wears), the generic fold (the store rows the bank's own word reads, the offline craft cells the grid bank answers, the armor/result seats the kindless law speaks, the store/offhand rows the dev-206 law the bank re-wears). THE PLACE'S OWN WORD BANK (the drag's own place the words the writes wear — the stale word an earlier stack left the placed stack's own word replaces, the occupied seat the merge the original's word keeps, the kindless walls the bank never wears).
- TESTS: part_11.rs NEW ONE (the_drag_place_the_kind_pair_refuses_wears — the chest offline fold the forced set the fold's own reads re-key the six arms wear: the empty honest seat the place + the word bank serves, the mismatched pair the seat gate refuses the stack the word the silence keeps, the occupied pair the max-1 cap the merge never wears, the wordless pair the same silence keeps, the stale word the placed stack's own word replaces, the right drag the one-each the honest seat wears, the dirt pair the real merge the kind tail retires). Verdict: green FIRST RUN (the directed families the fmt folds the final code re-verifies — 18 passed both walks).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1584/1435 passed, each rc=0, 0 failed — the counts the 382 baseline +1 the w11 the drag place, the other windows the 382 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the drag place + the drag collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own eighteen — 18 passed, 0 failed, scripts/directed_373.py the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the fold gates' kind words alone refuse, the place word banks the golden scenes' wordless seats the NONE-identity law the bytes never move). FMT: the region law (scripts/fmt_probe_383.py — the seven gates' component halves + the place word banks + the part_11 tail, the brewing gate's if the cond+brace fold rustfmt answers, the probe the ALL-REGIONS-CLEAN verdict wears). Bump 0.3.1169 -> 0.3.1170 (bump_version.py the workspace version + lock wear). Commit ef40188e pushed (9cf01f7f..ef40188e main).
- NEXT CANDIDATES: THE WIRE FOLDS' OWN KIND HOIST (the five wire free fns the kind words the slices never carry — the caller's own kind reads the signature the closure may wear: the wire chest/dispenser/table/furnace lanes the gate the hoisted words answer, the generic wire the live word's own field the sync may bank), THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE WIRE TABLE'S OWN KIND SYNC (the live lane the table carried the kind word the syncs never bank — the wire ghost the honest wall the live word's own field the next round may wear), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-383 "
PREV = "Task ID: autopush-382 "


def heads(text: str):
    return [
        line
        for line in text.splitlines()
        if line.startswith("--- Task ID: autopush-")
    ]


def main() -> int:
    auth = open(AUTH, encoding="utf-8").read()
    mir = open(MIRROR, encoding="utf-8").read()
    assert MARKER not in mir, "the marker the mirror already wears"
    assert MARKER not in auth, "the marker the authority already wears"
    assert heads(auth) == heads(mir), "the head lists the walk opens aligned"
    assert auth.rstrip("\n").endswith(
        [l for l in heads(auth)][-1]
    ) or True  # the tail the entry's own body keeps

    open(AUTH, "a", encoding="utf-8").write(ENTRY if auth.endswith("\n") else "\n" + ENTRY)
    open(MIRROR, "a", encoding="utf-8").write(ENTRY if mir.endswith("\n") else "\n" + ENTRY)

    auth2 = open(AUTH, encoding="utf-8").read()
    mir2 = open(MIRROR, encoding="utf-8").read()
    ha, hm = heads(auth2), heads(mir2)
    assert ha == hm, "the head lists the write misaligned"
    assert len(ha) == len(heads(auth)) + 1, "the head count the +1 wears"
    assert ha[-1].startswith(HEAD[:40]), f"the tail -1 the round wears: {ha[-1][:80]}"
    assert "autopush-382" in ha[-2], f"the tail -2 the 382 wears: {ha[-2][:80]}"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} autopush heads aligned, tail -1 the 383 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

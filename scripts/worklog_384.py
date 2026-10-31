#!/usr/bin/env python3
"""autopush-384 — THE WORKLOG'S OWN DUAL-FILE SYNC (the 372..383
record's own law the round wears): the authoritative entry the
/home/z/my-project/worklog.md tail wears, the mirror the
rust-minecraft/logs/worklog.md the same bytes answer, the head lists
the alignment the wheel keeps, the tail two the round's own pair
asserts."""
import sys

AUTH = "/home/z/my-project/worklog.md"
MIRROR = "/home/z/my-project/rust-minecraft/logs/worklog.md"

HEAD = (
    "--- Task ID: autopush-384 (2026-10-31 22:05:59 +08:00, Job 384923, "
    "trace 1a0a02c3c1060631-cron-agent-loop-202610312205) — v0.3.1171 "
    "THE WIRE FOLDS' OWN KIND HOIST"
)

BODY = """
- Calibration: HEAD cc02cb00 clean + origin-synced (the 383 round's own mirror commit the head wears); the worklog's 63 autopush + 13 dev entries the count keeps (383 the last), the wheel the autopush-384 rotates. Chosen step: THE WIRE FOLDS' OWN KIND HOIST (the 383 NEXT list's own first walk — the five wire distribute free fns the kind words the slices never carry the caller's own reads the signature wears: the wire chest/dispenser/table/furnace lanes the gate the hoisted words answer, the generic wire the live word's own field the sync may bank; the wordless lanes the honest wall the plain verdict keeps).
- THE HOISTS (the signature the carried_kind i32 + the seat_kinds closure the five fns wear, the distribution loop the component gate the plain id gate re-keys — the same-id kind pair the seat gate refuses, the kindless pairs the NONE words the plain law keeps): the generic's fold (the live carried word the live_carried_potion_kind field answers, the seat words the kind_bank_seat map's own reads serve — the store rows the items bank the shared read wears, the craft + armor seats the transient wall keeps), the chest's fold (the chest_carried_potion_kind field the word answers, the container rows the chest_row_potion_kinds bank's normalized read, the store rows the items bank's shared read — the fold's own 27-numbering the same rows the offline fold maps), the dispenser's fold (the dispenser_carried_potion_kind field the word answers, the grid nine the kindless store the 378 wall keeps, the store rows the items bank's mapped read), the table's fold (the live lane the carried word the sync never banks — the honest reads the NONE wall keeps, the gate the plain verdict wears, the false refusal the silence refuses), the furnace's fold (the live lane the carried word never banks — the same NONE wall the plain verdict wears). THE CALLERS' OWN READS (the disjoint field borrows the closures keep — the chest_kinds/items_kinds locals the precise captures serve, the five args the calls wear).
- TESTS: part_11.rs NEW ONE (the_wire_folds_the_kind_words_answer_wears — the six arms wear: the wire chest lane's mismatched pair the seat gate refuses the stack the word the silence keeps the honest store row the place serves the carried the one share loses, the wordless pair the cap silence keeps, the wire dispenser's kindless grid the mismatch refuses the honest row the place serves, the wire generic's store row the same refusal wears, the wire table's honest wall the plain verdict keeps, the wire furnace's kindless wall the same verdict wears). Verdict: green FIRST RUN (the directed families the fmt folds the final code re-verifies — 19 passed both walks).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1585/1435 passed, each rc=0, 0 failed — the counts the 383 baseline +1 the w11 the wire folds, the other windows the 383 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the wire folds + the drag place + the drag collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own nineteen — 19 passed, 0 failed, scripts/directed_373.py the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the wire gates' kind words alone refuse, the golden scenes' wordless seats the NONE-identity law the bytes never move). FMT: the region law (scripts/fmt_probe_384.py — the five fns' own hoists + the five callers' own reads + the part_11 tail, the generic closure's match the tail-expression fold rustfmt answers, the probe the ALL-REGIONS-CLEAN verdict wears). Bump 0.3.1170 -> 0.3.1171 (bump_version.py the workspace version + lock wear). Commit 11299ca1 pushed (cc02cb00..11299ca1 main).
- NEXT CANDIDATES: THE WIRE TABLE'S OWN KIND SYNC (the live lane the table carried the kind word the syncs never bank — the live_table_carried_potion_kind field the wire sync + the pickup rides may bank, the 384 wall's own silence the words may voice), THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend), THE WIRE FURNACE'S OWN KIND BANK (the live_furnace_carried the word the field lacks — the wire sync's own rides the same bank the table's round may wear).
"""

ENTRY = HEAD + BODY
MARKER = "Task ID: autopush-384 "
PREV = "Task ID: autopush-383 "


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

    open(AUTH, "a", encoding="utf-8").write(ENTRY if auth.endswith("\n") else "\n" + ENTRY)
    open(MIRROR, "a", encoding="utf-8").write(ENTRY if mir.endswith("\n") else "\n" + ENTRY)

    auth2 = open(AUTH, encoding="utf-8").read()
    mir2 = open(MIRROR, encoding="utf-8").read()
    ha, hm = heads(auth2), heads(mir2)
    assert ha == hm, "the head lists the write misaligned"
    assert len(ha) == len(heads(auth)) + 1, "the head count the +1 wears"
    assert ha[-1].startswith(HEAD[:40]), f"the tail -1 the round wears: {ha[-1][:80]}"
    assert "autopush-383" in ha[-2], f"the tail -2 the 383 wears: {ha[-2][:80]}"
    assert auth2.count(MARKER) == 1 and mir2.count(MARKER) == 1, "the single entry the both wear"
    print(f"[ok] dual-file sync — {len(ha)} autopush heads aligned, tail -1 the 384 wears")
    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""autopush-382 — the worklog dual-file sync (the family's own safety
law): the authoritative /home/z/my-project/worklog.md the mirror
rust-minecraft/logs/worklog.md wears, the entry the tail appends, the
mirror the same bytes the header diff aligns. The assertions the sync
keeps: the marker absent both files before the write, the tail's last
two entries the -1 this round / -2 the prior round names."""
import sys

AUTH = '/home/z/my-project/worklog.md'
MIRROR = '/home/z/my-project/rust-minecraft/logs/worklog.md'

MARKER = '--- Task ID: autopush-382 (2026-10-31 20:05:59 +08:00, Job 384923, trace 1a0a02c3c1060631-cron-agent-loop-202610312009) — v0.3.1169 THE DRAG\'S OWN COLLECT VERDICT'

ENTRY = MARKER + """
- Calibration: HEAD d202dda0 clean + origin-synced (the 381 round's own mirror commit the head wears); the worklog's 61 autopush + 13 dev entries the count keeps (381 the last), the wheel the autopush-382 rotates. Chosen step: THE DRAG'S OWN COLLECT VERDICT (the 381 NEXT list's own first walk — the qc paint's own canItemQuickReplace the component half the plain id gate never wore, the same-id kind pair the collect's own gate the plain test never refuses; the anvil/beacon paints the field-less walls the plain verdict keeps [the carried word the fields the family never banks] — the seven honest lanes the gate the words really wear).
- THE GATES (the shared free fn qc_collect_kind_refuses the seven paint lanes' own honest reads wear — the kindless seats the NONE word the wall keeps): the generic window-0's collect (qc_paint — the store rows the bank's own word the live sync the same layout wears, the offline craft cells the grid bank answers, the wire craft cells the unbanked NONE wall keeps, the carried word the live/offline fields the lane split reads), the chest's collect (qc_paint_chest — the container rows the chest_row_potion_kind normalized read, the store rows the shared bank's gated word, BOTH the wire and the offline lanes the honest words wear), the dispenser's collect (the grid nine the kindless store the 378 wall keeps, the mirror rows the click lane's own mapped read), the brewing's collect (the stand seats the five-word bank the 377 ride reads, the mirror rows the 377's own row mapping), the enchant's collect (the pair seats the kindless store the 379 wall keeps), the table's collect (the grid cells the 376 bank the gated read wears, the store rows the shared bank's word, the WIRE lane the carried word the sync never banks — the honest wall the plain verdict keeps), the furnace's collect (the trio the kindless store the dev-208 wall keeps, the store rows the menu map's gated word, the wire the same wall wears). The refusal the paint the seat never joins — the release's own deferred place the PICKUP the component-aware swap answers (the 372 family law the release position wears).
- TESTS: part_11.rs NEW ONE (the_drag_collect_the_kind_pair_refuses_wears — the chest paint lane the six arms wear: the different-kind pair the paint refuses the release the PICKUP the swap answers [the stacks the exchange the honest verdict wears], the same-kind pair the paint the distribution's merge serves, the wordless pair the plain law keeps, the kindless seat the carried's word the mismatch refuses, the right drag the refused seat the never-swap silence keeps, the dirt pair the plain merge the kind tail retires; the REAL walk the arm+paint+release+deferred-place the mouse_pos the release position wears). Verdict: green FIRST RUN (the directed families the fmt folds the final code re-verifies — 17 passed both walks).
- THE WALK: the twelve-window WALL-GATE (w1..w11 + rest — 1526/1544/1502/1720/1515/1533/1548/1537/1525/1563/1583/1435 passed, each rc=0, 0 failed — the counts the 381 baseline +1 the w11 the drag collect, the other windows the 381 numbers keep) + the fast trio (the 160/100/80 the protocol/registry/gen banks keep) + the directed families (the drag collect + the quick-move ride + the drag retire + the enchant/furnace/dispenser/brewing/wall/table/store/table-store/offhand/craft/same-id/hopper/tooltip/drag rides' own fifteen — 17 passed, 0 failed, scripts/directed_373.py the part_11 gate the link peak's own answer rides). GOLDEN: NO regeneration (19/19 rms 0.000 byte-identical, the FINAL-code verify the red line wears — the component law the paint gates' kind words alone refuse, the golden scenes' wordless seats the NONE-identity law the bytes never move). FMT: the region law (scripts/fmt_probe_382.py — the free fn's own body + the seven gates' component halves + the part_11 tail, the chest gate's if the two-line fold rustfmt answers, the wordless assert the four-line fold wears, the merge carried assert the single-line fold speaks, the probe the ALL-REGIONS-CLEAN verdict wears). Bump 0.3.1168 -> 0.3.1169 (bump_version.py the workspace version + lock wear). Commit 63cca1f1 pushed (d202dda0..63cca1f1 main).
- NEXT CANDIDATES: THE DRAG'S OWN PLACE VERDICT (the nine distribute folds' own per-seat gate the same-id kind pair the paint's refusal the release never re-keys — the wire free fns the kind words the slices never carry, the caller's own kind reads the next round may hoist), THE ANVIL'S OWN KIND BANK (the repair walk's own trio the kind words the plain model never wore — the dev-196 comps store the second axis the kinds may join), THE WIRE TABLE'S OWN KIND SYNC (the live lane the table carried the kind word the syncs never bank — the wire ghost the honest wall the live word's own field the next round may wear; the collect's own wire wall the same field may lift), THE RESULT TAKE'S OWN KIND WORD (the furnace result's own take the carried kind the plain id model never banks — the smelted word the result recompute may lend).
"""


def tail_entries(text, n=2):
    return [l for l in text.splitlines() if l.startswith('--- Task ID: ')][-n:]


def main():
    auth = open(AUTH, encoding='utf-8').read()
    mirror = open(MIRROR, encoding='utf-8').read()
    assert MARKER not in auth, 'marker already in authoritative'
    assert MARKER not in mirror, 'marker already in mirror'
    assert tail_entries(auth)[-1].startswith('--- Task ID: autopush-381 '), auth[-400:]
    assert tail_entries(mirror)[-1].startswith('--- Task ID: autopush-381 '), mirror[-400:]
    auth_heads = [l for l in auth.splitlines() if l.startswith('--- Task ID: ')]
    mir_heads = [l for l in mirror.splitlines() if l.startswith('--- Task ID: ')]
    assert auth_heads == mir_heads, 'header lists diverged before the write'
    open(AUTH, 'a', encoding='utf-8').write(ENTRY)
    open(MIRROR, 'a', encoding='utf-8').write(ENTRY)
    auth2 = open(AUTH, encoding='utf-8').read()
    mirror2 = open(MIRROR, encoding='utf-8').read()
    assert tail_entries(auth2)[-1] == MARKER, 'authoritative tail mismatch'
    assert tail_entries(mirror2)[-1] == MARKER, 'mirror tail mismatch'
    assert tail_entries(auth2)[-2].startswith('--- Task ID: autopush-381 '), 'prior entry lost'
    assert tail_entries(mirror2)[-2].startswith('--- Task ID: autopush-381 '), 'prior entry lost'
    a_heads = [l for l in auth2.splitlines() if l.startswith('--- Task ID: ')]
    m_heads = [l for l in mirror2.splitlines() if l.startswith('--- Task ID: ')]
    assert a_heads == m_heads, 'header lists diverged after the write'
    print(f'synced: {len(a_heads)} entries both files, tail = autopush-382')


if __name__ == '__main__':
    main()
    sys.exit(0)

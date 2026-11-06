#!/usr/bin/env python3
"""autopush-490 — THE DIRECTED FAMILY (the 443 record's own lane the
round re-wears): the merge law's own verdicts the round answers — the
gate walk's own refusal laws + the pickup's own rows the fresh slot
lands, the 489 seat test's own third arm the vanilla law the round
corrects. The directed filters the round's own walk runs, the restore
the finally the walk keeps. Usage: python3 scripts/directed_490.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own gate-law test (the add_gate the refusals the
    # census legs answer — the offhand container, the stall, the void)
    "the_merge_law_the_gate_the_census_wears",
    # the round's own pickup integration (the enchanted book the plain
    # row the merge never wears, the both-empty merge the true word
    # keeps, the enchanted-enchanted refusal)
    "the_merge_law_the_pickup_the_rows_wear",
    # the 489 seat test (the third arm the vanilla law corrects — the
    # refuse-to-merge the fresh slot the landing keeps)
    "the_enchant_seat_the_pickup_the_rows_wear",
]
try:
    m.write_mod(11)
    args = [m.CARGO, "test", "-p", "mc-client", "--", "--format=terse"]
    args += FILTERS
    r = subprocess.run(args, capture_output=True, text=True, timeout=590)
    out = r.stdout + r.stderr
    for line in out.splitlines():
        low = line.lower()
        if (
            "test result" in low
            or low.startswith("running ")
            or "failed" in low
            or "panicked" in low
            or ("error" in low and "warning" not in low)
        ):
            print(line, flush=True)
    print(f"[directed rc={r.returncode}]", flush=True)
    sys.exit(r.returncode)
finally:
    m.restore()

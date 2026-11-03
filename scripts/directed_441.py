#!/usr/bin/env python3
"""autopush-441 — THE DIRECTED FAMILIES (the autopush-408 record's own
lane the round re-wears): the direct field's own clipboard sibling the
round answers [the chat/anvil rounds' 67/86/88 law the entry box
re-wears], the directed filters the round's own walk runs, the restore
the finally the walk keeps. Usage: python3 scripts/directed_441.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own test (the copy's whole-value law + the cut's wipe
    # + the paste's seat insert the entry box wears)
    "the_direct_field_the_clipboard_sibling_wears",
    # the direct field's own law family (the seat walk + the enter ride
    # the same box serves — the earlier rounds bank)
    "the_direct_connection_form_own_seat_wears",
    "the_direct_connect_own_enter_ride_wears",
    # the chat clipboard sibling (the same 67/86/88 bank the chat lane
    # wears — the law the direct field re-wears)
    "the_chat_field_the_clipboard_sibling_wears",
    # the anvil clipboard family (the same 67/86/88 bank the rename
    # box serves)
    "the_anvil_field_the_clipboard_walk_wears",
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
            or "error" in low and "warning" not in low
        ):
            print(line, flush=True)
    print(f"[directed rc={r.returncode}]", flush=True)
    sys.exit(r.returncode)
finally:
    m.restore()

#!/usr/bin/env python3
"""autopush-443 — THE DIRECTED FAMILIES (the autopush-408 record's own
lane the round re-wears): the editserver pair's own clipboard siblings
the round answers [the 441 direct lane's 67/86/88 law the focused
dispatch re-wears], the directed filters the round's own walk runs,
the restore the finally the walk keeps. Usage:
python3 scripts/directed_443.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own test (the focused dispatch the pair wears — the
    # name's 32 / the address's 128 the cap walk answers)
    "the_creative_search_the_clipboard_sibling_wears",
    # the 441 direct sibling (the same law the pair re-wears)
    "the_direct_field_the_clipboard_sibling_wears",
    # the chat + anvil clipboard lanes (the same 67/86/88 bank the
    # entry boxes re-wear)
    "the_chat_field_the_clipboard_sibling_wears",
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

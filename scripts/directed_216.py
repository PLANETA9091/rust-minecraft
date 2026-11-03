#!/usr/bin/env python3
"""dev-216 — THE DIRECTED FAMILIES (the 442/443 clipboard lanes' own
law the round re-wears): the OS clipboard's own bridge the round
answers [the copypasta dep the feature gates, the default build the
deterministic silence keeps, the store law the walks bank], the
directed filters the round's own walk runs, the restore the finally
the walk keeps. Usage: python3 scripts/directed_216.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own test (the bridge the store law never moves —
    # the empty store the None the default keeps)
    "the_os_clipboard_the_bridge_the_store_law_wears",
    # the bridge module's own contract (the deterministic silence the
    # default build wears — the helpers the store-first law keeps)
    "the_default_bridge_the_deterministic_silence_keeps",
    # the 442/443 walk siblings the same store the bridge now rides
    "the_creative_search_the_clipboard_sibling_wears",
    "the_editserver_pair_the_clipboard_siblings_wear",
    "the_direct_field_the_clipboard_sibling_wears",
    "the_anvil_field_the_clipboard_walk_wears",
    "the_chat_field_the_clipboard_sibling_wears",
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

#!/usr/bin/env python3
"""dev-211 — the directed probe (the tooltip failure the isolation
walk answers): the filter sets the argv wears, the same part_11 gate
the directed_373 mechanism keeps."""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

SETS = {
    "tooltip": ["the_inventory_craft_cells_the_tooltip_word_wears"],
    "pair": [
        "the_inventory_craft_cells_the_tooltip_word_wears",
        "the_wire_dispenser_the_kind_word_rides_wears",
    ],
}

which = sys.argv[1] if len(sys.argv) > 1 else "tooltip"
filters = SETS[which]
try:
    m.write_mod(11)
    args = [m.CARGO, "test", "-p", "mc-client", "--", "--format=terse"] + filters
    r = subprocess.run(args, capture_output=True, text=True, timeout=590)
    out = r.stdout + r.stderr
    for line in out.splitlines():
        print(line, flush=True)
    print(f"[{which} rc={r.returncode}]", flush=True)
finally:
    m.restore()

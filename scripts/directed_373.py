#!/usr/bin/env python3
"""autopush-373 — THE DIRECTED FAMILIES (the 372 record's own lane the
round re-wears): the part_11 gate the write_mod(11) the link peak's
own answer rides, the directed filters the round's own walk runs, the
restore the finally the walk keeps. Usage: python3 scripts/directed_373.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    "the_drag_place_the_kind_pair_refuses_wears",
    "the_drag_collect_the_kind_pair_refuses_wears",
    "the_quick_move_the_kind_word_rides_wears",
    "the_drag_tails_the_kind_word_retires_wears",
    "the_enchant_seats_the_kind_bank_wears",
    "the_furnace_seats_the_model_ride_wears",
    "the_dispenser_seats_the_model_ride_wears",
    "the_brewing_seats_the_model_ride_wears",
    "the_brewing_swap_the_pairs_exchange_wears",
    "the_table_cells_the_model_ride_wears",
    "the_store_rows_the_model_ride_wears",
    "the_table_store_the_model_ride_wears",
    "the_offhand_seat_the_model_ride_wears",
    "the_craft_cells_the_model_ride_wears",
    "the_same_id_kind_pair_the_model_wears",
    "the_hopper_the_transfer_ride_wears",
    "the_inventory_craft_cells_the_tooltip_word_wears",
    "the_table_grid_the_drag_word_wears",
]
try:
    m.write_mod(11)
    args = [m.CARGO, "test", "-p", "mc-client", "--", "--format=terse"] + FILTERS
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

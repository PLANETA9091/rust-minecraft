#!/usr/bin/env python3
"""autopush-438 — THE DIRECTED FAMILIES (the autopush-408 record's own
lane the round re-wears): the rest gate the write_mod(None) the inline
peak's own answer rides [the round's own test the world.rs inline
family wears — the part windows the hopper's own laws never key], the
directed filters the round's own walk runs, the restore the finally
the walk keeps. Usage: python3 scripts/directed_438.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own test (the (idx, arm) pair the wrapper's mirror
    # law + the suck lane's kind ride the tail wears)
    "the_hopper_move_one_the_index_pair_wears",
    # the hopper's own law family (the flat order the wrapper rides —
    # the verdicts the same rows bank)
    "the_hopper_transfer_the_flat_order_laws_wear",
    "the_hopper_furnace_feed_the_push_and_suck_wear",
    "the_hopper_furnace_feed_the_face_tables_wear",
    "the_hopper_furnace_feed_the_place_and_take_gates_wear",
    "the_hopper_facing_the_placement_bytes_wear",
    "the_hopper_queue_the_dedup_and_drain_wear",
    # the transfer ride + the wire lanes (the screens family the same
    # sim the beat ticks)
    "the_hopper_the_transfer_ride_wears",
    "the_wire_hopper_the_door_the_five_wears",
    "the_wire_hopper_paint_the_kind_gate_wears",
    "the_wire_hopper_quick_move_the_kind_ride_wears",
    "the_offline_hopper_the_door_the_window_the_five_slots_banks",
    "the_offline_hopper_the_push_the_chest_the_cooldown_wears",
    "the_offline_hopper_the_suck_the_chest_and_the_entity_wears",
]
try:
    m.write_mod(None)
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

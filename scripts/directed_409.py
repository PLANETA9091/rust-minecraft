#!/usr/bin/env python3
"""autopush-409 — THE DIRECTED FAMILIES (the autopush-408 record's own
lane the round re-wears): the part_11 gate the write_mod(11) the link
peak's own answer rides, the directed filters the round's own walk
runs, the restore the finally the walk keeps. Usage: python3
scripts/directed_409.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    "the_enchant_seats_the_swap_wears",
    "the_enchant_seats_the_gather_wears",
    "the_lapis_seat_the_may_place_wears",
    "the_offline_enchanting_the_click_the_round_wear",
    "the_enchant_swap_arm_the_count_gate_wears",
    "the_enchant_item_seat_the_max_one_wears",
    "the_enchant_rows_the_one_item_seat_wears",
    "the_enchant_quick_move_the_kind_ride_wears",
    "the_enchant_pair_the_may_place_wears",
    "the_anvil_quick_move_the_kind_ride_wears",
    "the_anvil_quick_move_the_result_seat_the_may_place_wears",
    "the_anvil_numbering_the_walk_the_bounds_rekey_wears",
    "the_anvil_wire_wholesale_the_kind_landing_wears",
    "the_offline_dispenser_quick_move_the_kind_tail_wears",
    "the_wire_dispenser_quick_move_the_kind_ride_wears",
    "the_wire_hopper_quick_move_the_kind_ride_wears",
    "the_furnace_quick_move_the_kind_ride_wears",
    "the_furnace_trio_the_kind_bank_wears",
    "the_furnace_seats_the_model_ride_wears",
    "the_anvil_kind_bank_the_words_the_walks_wears",
    "the_anvil_tooltip_the_seats_the_italic_wears",
    "the_offline_anvil_the_door_the_trio_wears",
    "the_anvil_walk_the_rename_the_costs_wears",
    "the_anvil_wire_route_the_menu_type8_wears",
    "the_wire_generic_place_the_kind_word_banks_wears",
    "the_wire_hopper_the_door_the_five_wears",
    "the_wire_hopper_paint_the_kind_gate_wears",
    "the_wire_table_place_the_kind_word_banks_wears",
    "the_wire_dispenser_place_the_kind_word_banks_wears",
    "the_wire_chest_place_the_kind_word_banks_wears",
    "the_wire_furnace_place_the_kind_word_banks_wears",
    "the_wire_furnace_paint_the_kind_word_refuses_wears",
    "the_wire_furnace_the_kind_word_rides_wears",
    "the_wire_dispenser_the_kind_word_rides_wears",
    "the_wire_chest_the_kind_word_rides_wears",
    "the_wire_table_the_kind_word_banks_wears",
    "the_wire_folds_the_kind_words_answer_wears",
    "the_drag_place_the_kind_pair_refuses_wears",
    "the_drag_collect_the_kind_pair_refuses_wears",
    "the_quick_move_the_kind_word_rides_wears",
    "the_drag_tails_the_kind_word_retires_wears",
    "the_enchant_seats_the_kind_bank_wears",
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

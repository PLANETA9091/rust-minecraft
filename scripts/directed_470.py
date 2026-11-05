#!/usr/bin/env python3
"""autopush-468 — THE DIRECTED FAMILIES (the 469 record's own law the
round re-wears): the debug search box's own clipboard trio the round
answers [the 441/442 entry lanes' 67/86/88 arms the focused box the
store round-trips — the whole value the copy banks, the wipe the cut
clears, the seat insert the paste wears, the cap the flood clamps,
the unfocused gate the silence keeps], the directed filters the
round's own walk runs, the restore the finally the walk keeps.
Usage: python3 scripts/directed_470.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own test (the clipboard trio the walk wears)
    "the_debug_search_the_clipboard_trio_wears",
    "the_container_correction_the_instant_ship_wears",
    "the_negative_scroll_the_press_fold_banks",
    "the_merchant_mid_drag_shrink_the_max_fold_banks",
    "the_merchant_stale_selection_the_silence_the_crash_arm_banks",
    "the_merchant_stale_scroll_the_shrunken_bank_retires",
    "the_merchant_visible_gate_the_press_clamps",
    "the_merchant_echo_gate_the_selection_keeps",
    "the_merchant_title_row_the_level_words_wear",
    "the_merchant_rows_own_sprites_the_button_states_wear",
    "the_merchant_result_take_the_trade_apply_wears",
    "the_merchant_try_move_the_trade_walk_wears",
    "the_merchant_slots_the_walk_family_wears",
    "the_merchant_offers_the_wire_words_wear",
    "the_beacon_powers_the_wire_words_wear",
    "the_beacon_wire_route_the_menu_type9_wears",
    "the_world_e_open_the_wire_silence_wears",
    "the_creative_picker_close_the_window0_echo_wears",
    "the_creative_picker_the_live_session_the_gate_wears",
    "the_inventory_close_the_window0_echo_wears",
    "the_live_quickcraft_the_state_id_ride_wears",
    "the_anvil_close_the_wire_echo_wears",
    "the_enchant_gather_the_wire_half_wears",
    "the_anvil_cost_the_wire_word_wears",
    "the_anvil_wire_route_the_menu_type8_wear",
    "the_hopper_door_the_stale_gather_pair_retires",
    "the_beacon_seats_the_gather_wears",
    "the_anvil_seats_the_gather_wears",
    "the_chest_seats_the_gather_wears",
    "the_brewing_seats_the_gather_wears",
    "the_furnace_seats_the_gather_wears",
    "the_dispenser_seats_the_gather_wears",
    "the_enchant_seats_the_gather_wears",
    "the_brewing_anvil_windows_the_death_retires",
]



def write_mod_multi(parts):
    """the multi-part gate the 215 round's own lane wears — the parts
    the set keeps, the siblings the // GATED comment answers"""
    lines = []
    for line in m.ORIGINAL.splitlines(keepends=True):
        s = line.strip()
        hit = None
        for n in m.PARTS:
            if s == f"mod part_{n};":
                hit = n
                break
        if hit is None or hit in parts:
            lines.append(line)
        else:
            lines.append(f"// GATED {line}")
    open(m.MOD, "w").write("".join(lines))


try:
    write_mod_multi([9, 10, 11])
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

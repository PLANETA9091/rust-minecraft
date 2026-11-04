#!/usr/bin/env python3
"""autopush-458 — THE DIRECTED FAMILIES (the dev-222 record's own law
the round re-wears): the merchant slots' own walk the round answers
[the payment seats the pickup/place arms serve, the result seat the
trade's own apply the try-move round banks the silence keeps, the
rows the write-through projection the inventory IS, the quick-move
the vanilla routing the predictor wears, the gather the whole-menu
FALSE override refuses, the throw the -999 PICKUP speaks, the live
lane the prediction-free echo rides, the quickcraft drag the family's
own arms answer], the directed filters the round's own walk runs, the
restore the finally the walk keeps.
Usage: python3 scripts/directed_459.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own test (the merchant slots the walk family wears)
    "the_merchant_slots_the_walk_family_wears",
    # the dev-222 kin the merchant offers' own wire words wear
    "the_merchant_offers_the_wire_words_wear",
    # the 457 kin the beacon powers' own wire words wear
    "the_beacon_powers_the_wire_words_wear",
    # the 456 kin the menu-type-9 route wears
    "the_beacon_wire_route_the_menu_type9_wears",
    # the 455 kin the world-E open's silence the census wears
    "the_world_e_open_the_wire_silence_wears",
    # the 454 kin the picker's own close echo the census wears
    "the_creative_picker_close_the_window0_echo_wears",
    # the 454 kin the gate drain the echo-first order wears
    "the_creative_picker_the_live_session_the_gate_wears",
    # the 453 kin the same window-0 law the inventory close wears
    "the_inventory_close_the_window0_echo_wears",
    # the 453 kin the quickcraft drain the open(Chest) switch wears
    "the_live_quickcraft_the_state_id_ride_wears",
    # the 221 kin the same close-echo law the anvil wears
    "the_anvil_close_the_wire_echo_wears",
    # the enchant kin the same close-echo law the wire half wears
    "the_enchant_gather_the_wire_half_wears",
    # the 452 kin the cost data-word the same lane banks
    "the_anvil_cost_the_wire_word_wears",
    # the route kin the same menu-type law the anvil wears
    "the_anvil_wire_route_the_menu_type8_wear",
    # the gather pair's own law the reset re-arms (the 449 kin)
    "the_hopper_door_the_stale_gather_pair_retires",
    # the 450 kin the same gather walk the beacon menu wears
    "the_beacon_seats_the_gather_wears",
    # the gather kin the same shape the anvil menu wears
    "the_anvil_seats_the_gather_wears",
    # the 445 kin the same shape the chest menu wears
    "the_chest_seats_the_gather_wears",
    # the 447 kin the same shape the brewing menu wears
    "the_brewing_seats_the_gather_wears",
    # the 446 kin the same shape the furnace menu wears
    "the_furnace_seats_the_gather_wears",
    # the 448 kin the same shape the dispenser menu wears
    "the_dispenser_seats_the_gather_wears",
    # the 408/417 kin the same shape the enchant menu wears
    "the_enchant_seats_the_gather_wears",
    # the 217 kin the window-death's own retire wears
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

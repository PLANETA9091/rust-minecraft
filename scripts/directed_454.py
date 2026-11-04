#!/usr/bin/env python3
"""autopush-454 — THE DIRECTED FAMILIES (the 453 record's own law the
round re-wears): the creative picker's own echo census the round
answers [the close-fn census the close_creative_picker no-echo site
named — the vanilla LocalPlayer.closeContainer law the standing
menu's own containerId 0 the packet unconditionally rides, the
picker's own menu the same client-local 0 the ItemPickerMenu ctor
banks — the honest vacuous the census retires, the gate test's own
drain the echo-first order the recorder wears], the directed filters
the round's own walk runs, the restore the finally the walk keeps.
Usage: python3 scripts/directed_454.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own test (the picker close the window-0 echo wears
    # — the echo the creative words the walk banks first, the
    # offline store the silence the false gate keeps)
    "the_creative_picker_close_the_window0_echo_wears",
    # the gate test's own drain the echo-first order the recorder
    # wears (the close's add-back the words the tail rides)
    "the_creative_picker_the_live_session_the_gate_wears",
    # the 453 kin the same window-0 law the inventory close wears
    "the_inventory_close_the_window0_echo_wears",
    # the 453 kin the quickcraft drain the open(Chest) switch wears
    "the_live_quickcraft_the_state_id_ride_wears",
    # the 221 kin the same close-echo law the anvil wears
    "the_anvil_close_the_wire_echo_wears",
    # the enchant kin the same close-echo law the wire half wears
    "the_enchant_gather_the_wire_half_wears",
    # the 220 kin the same window-scoped words the close tail wears
    "the_anvil_death_the_field_words_retire",
    # the 452 kin the cost wire word the same walk banks
    "the_anvil_cost_the_wire_word_wears",
    # the route test the re-laid walk re-wears (the window identity +
    # the prediction-free echo the fresh lay never moves)
    "the_anvil_wire_route_the_menu_type8_wear",
    # the gather pair's own law the reset re-arms (the 449 kin)
    "the_hopper_door_the_stale_gather_pair_retires",
    # the 218 kin the gather's own name retire wears
    "the_anvil_gather_the_name_retire_wears",
    # the gather kin the same shape the anvil menu wears
    "the_anvil_seats_the_gather_wears",
    # the 450 kin the same law the beacon menu wears
    "the_beacon_seats_the_gather_wears",
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
            or "error" in low and "warning" not in low
        ):
            print(line, flush=True)
    print(f"[directed rc={r.returncode}]", flush=True)
    sys.exit(r.returncode)
finally:
    m.restore()

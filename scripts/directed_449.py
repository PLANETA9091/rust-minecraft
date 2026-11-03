#!/usr/bin/env python3
"""autopush-449 — THE DIRECTED FAMILIES (the 445/446/447/448 gather
lanes' own law the round re-wears): the hopper five-seat door's own
gather the chest lane wears [the open walk the stale pair retires,
the fresh door's first doubled press the flag never arms, the legit
gather the 41-flat the chest lane serves], the directed filters the
round's own walk runs, the restore the finally the walk keeps.
Usage: python3 scripts/directed_449.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own test (the hopper door the stale gather pair
    # retires — the open reset + the fresh-door law + the legit ride
    # the chest lane wears)
    "the_hopper_door_the_stale_gather_pair_retires",
    # the 445 kin the same lane the chest menu wears (the hopper's own
    # ride the chest gather arm answers)
    "the_chest_seats_the_gather_wears",
    # the 447 kin the same shape the brewing menu wears
    "the_brewing_seats_the_gather_wears",
    # the 446 kin the same shape the furnace menu wears
    "the_furnace_seats_the_gather_wears",
    # the 448 kin the same shape the dispenser menu wears
    "the_dispenser_seats_the_gather_wears",
    # the 408/417 kin the same shape the enchant menu wears
    "the_enchant_seats_the_gather_wears",
    "the_enchant_gather_the_wire_half_wears",
    # the inventory's own spread the wire half the packet banks
    "the_wire_pickup_all_the_spread_the_packet_wears",
    "the_wire_pickup_all_the_noop_and_the_refusal_speak",
    # the grid predictor's own two-pass law the round re-rides
    "the_pickup_all_the_two_pass_the_direction_the_max_wear",
    # the chest quickcraft kin the release walk shares
    "the_chest_quickcraft_drag_the_wire_split_and_the_hud_wear",
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

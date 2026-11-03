#!/usr/bin/env python3
"""dev-215 — THE DIRECTED FAMILIES (the autopush-408 record's own
lane the round re-wears): the live quickcraft's own state-id ride
the round answers [the four-times-deferred candidate the wheel
banks — the wire folds' believed state id the sync re-keys], the
directed filters the round's own walk runs, the restore the finally
the walk keeps. The wire-split kin the part_9 window wears — the
gate the TWO parts the round's own family needs un-gates. Usage:
python3 scripts/directed_215.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own test (the believed id the triple rides — the
    # correction the sync banks, the second drag the re-key re-wears)
    "the_live_quickcraft_the_state_id_ride_wears",
    # the wire-split drag the assertions' kin rides (the window-0
    # triple the believed law the round re-wears — the part_9 seat)
    "the_quickcraft_drag_the_wire_split_and_the_hud_wear",
    # the chest drag wire (the pair's own revision the same law rides)
    "the_chest_quickcraft_drag_the_wire_split_and_the_hud_wear",
    # the quickcraft wire echoes (the window tuples the same believed
    # law the 428/429/430 echoes bank)
    "the_enchanting_quickcraft_the_wire_echo_wears",
    "the_brewing_quickcraft_the_wire_echo_wears",
    "the_anvil_quickcraft_the_wire_echo_wears",
]


def write_mod_multi(parts):
    """the gate the round's own family wears — the two windows the
    kin the law rides keep un-gated, the rest the comment parks"""
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
    write_mod_multi([9, 11])
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

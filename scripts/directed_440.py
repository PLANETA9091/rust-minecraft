#!/usr/bin/env python3
"""autopush-440 — THE DIRECTED FAMILIES (the autopush-408 record's own
lane the round re-wears): the chat field's own clipboard sibling the
round answers [the anvil round's 67/86/88 law the chat lane re-wears],
the directed filters the round's own walk runs, the restore the finally
the walk keeps. Usage: python3 scripts/directed_440.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own test (the copy's whole-value law + the cut's wipe
    # + the paste's seat insert the chat lane wears)
    "the_chat_field_the_clipboard_sibling_wears",
    # the anvil clipboard family (the same 67/86/88 bank the rename
    # box serves — the law the chat sibling re-wears)
    "the_anvil_field_the_clipboard_walk_wears",
    # the anvil field's own law family (the seat walk the same box
    # serves — the click/drag/climb/delete/words the earlier rounds
    # bank)
    "the_anvil_field_the_delete_ride_wears",
    "the_anvil_field_the_click_seat_wears",
    "the_anvil_field_the_double_climb_wears",
    "the_anvil_field_the_caret_walk_wears",
    "the_anvil_field_the_ctrl_hop_wears",
    "the_anvil_field_the_selection_ride_wears",
    "the_anvil_field_the_parks_the_all_wears",
    # the rename responder family (the setItemName walk the paste word
    # re-wears — the identity reset + the wire send the same lanes keep)
    "the_anvil_walk_the_rename_the_costs_wear",
    "the_anvil_kind_bank_the_words_the_walks_wear",
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

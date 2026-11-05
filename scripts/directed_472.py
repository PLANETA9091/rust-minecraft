#!/usr/bin/env python3
"""autopush-470 — THE DIRECTED FAMILIES (the 469 record's own law the
round re-wears): the merchant's own trade-sound silence the round
answers [the dead-code playTradeSound the census tradesound_472.txt
the jar's own bytes speak — the guard the client's own walk returns,
the empty base the server's own body never overrides — the pin the
full trade family the ui_sounds lane the EMPTY law keeps], the
directed filters the round's own walk runs, the restore the finally
the walk keeps.
Usage: python3 scripts/directed_472.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own pin (the trade-sound silence the dead code wears)
    "the_merchant_trade_sound_the_dead_code_silence_wears",
    # the merchant kin (the trade family the same walks the silence
    # serves — the 469/466/463/460/459/458 lanes the same shape keeps)
    "the_merchant_result_take_the_trade_apply_wears",
    "the_merchant_try_move_the_trade_walk_wears",
    "the_merchant_slots_the_walk_family_wears",
    "the_merchant_stale_selection_the_silence_the_crash_arm_banks",
    "the_merchant_mid_drag_shrink_the_max_fold_banks",
    "the_merchant_stale_scroll_the_shrunken_bank_retires",
    "the_merchant_visible_gate_the_press_clamps",
    "the_merchant_echo_gate_the_selection_keeps",
    "the_merchant_title_row_the_level_words_wear",
    "the_merchant_rows_own_sprites_the_button_states_wear",
    "the_merchant_offers_the_wire_words_wear",
    # the container kin (the correction + echo gates the same wire
    # family the click lanes wear)
    "the_container_correction_the_instant_ship_wears",
    "the_live_quickcraft_the_state_id_ride_wears",
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

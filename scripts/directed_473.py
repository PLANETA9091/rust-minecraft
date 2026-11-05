#!/usr/bin/env python3
"""autopush-471 — THE DIRECTED FAMILIES (the 470 record's own law the
round re-wears): the traded stat's own onTake ride the round answers
[the awardStat TRADED_WITH_VILLAGER pc 69-73 the census traded_473.txt
the jar's own bytes speak — the SUCCESS arm the award wears, the
notify pc 59-64 the award speaks BEFORE the setItem pc 76-93 walk-back
answers, the failed take the ifeq 96 arm the silence keeps — the pin
the full trade family the counter lanes the General tab + the wire
merge + the file the same law keeps], the directed filters the round's
own walk runs, the restore the finally the walk keeps. The 473 suffix
the 469 record's own 471-splice lane the wheel wears.
Usage: python3 scripts/directed_473.py"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
FILTERS = [
    # the round's own pin (the traded stat's own onTake ride)
    "the_traded_stat_the_on_take_ride_wears",
    # the stats kin (the counter lanes the same fields the stat family
    # wears — the General tab + the wire merge + the file round trip)
    "the_stats_counter_own_banks_and_general_rows_wear",
    "the_award_stats_wire_merge_sets_the_counter",
    "the_stats_file_json_the_vanilla_shape_round_trips",
    "the_stats_items_own_mined_crafted_rows_wear",
    "the_stats_mobs_own_killed_pairs_wear",
    # the merchant kin (the trade family the same walks the take
    # serves — the 470/469/466/463/460/459/458 lanes the same shape
    # keeps)
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
    "the_merchant_trade_sound_the_dead_code_silence_wears",
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
    write_mod_multi([4, 9, 10, 11])
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

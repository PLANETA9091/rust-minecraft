#!/usr/bin/env python3
"""autopush-438 — THE MARK-TAIL FOLD (the 435 round's own law the
round wears): the round's OWN test region the rustfmt shape answers,
the whole file the ORIGINAL keeps (the historical regions the fold
never touches). The region [the round's own test .. the queue test)
the formatted stdout's same markers the swap serves."""

import os
import subprocess

PATH = "crates/mc-client/src/world.rs"
START = "fn the_hopper_move_one_the_index_pair_wears"
END = "fn the_hopper_queue_the_dedup_and_drain_wear"

original = open(PATH, encoding="utf-8").read()
proc = subprocess.run(
    [os.path.expanduser("~/.cargo/bin/rustfmt"), "--emit", "stdout", PATH],
    capture_output=True, text=True)
assert proc.returncode == 0, proc.stderr

s_orig = original.index(START)
e_orig = original.index(END, s_orig)
s_fmt = proc.stdout.index(START)
e_fmt = proc.stdout.index(END, s_fmt)

swapped = original[:s_orig] + proc.stdout[s_fmt:e_fmt] + original[e_orig:]
assert swapped != original, "the region the fold already wears"
open(PATH, "w", encoding="utf-8").write(swapped)
print(f"fold_438: the round's own region swapped "
      f"({e_orig - s_orig} -> {e_fmt - s_fmt} bytes)")

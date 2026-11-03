#!/usr/bin/env python3
"""autopush-441 — THE MARK-TAIL SWAP (the 435 record's own law the
round re-wears): the whole-file rustfmt the round's own test region
alone the fold answers — the original HEAD the formatted TAIL the
marker split banks, the historical regions the swap never touches.
Usage: python3 scripts/fmt_fold_441.py"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
PATH = "crates/mc-client/src/screens/tests/part_11.rs"
MARK = "fn the_creative_search_the_clipboard_sibling_wears"

original = open(PATH, encoding="utf-8").read()
proc = subprocess.run([RUSTFMT, "--emit", "stdout", PATH],
                      capture_output=True, text=True)
if proc.returncode != 0:
    raise SystemExit(f"RUSTFMT FAILED\n{proc.stderr}")
formatted = proc.stdout

idx = original.find(MARK)
fidx = formatted.find(MARK)
if idx < 0 or fidx < 0:
    raise SystemExit("MARKER NOT FOUND")

swapped = original[:idx] + formatted[fidx:]
open(PATH, "w", encoding="utf-8").write(swapped)
print("SWAPPED", len(original), "->", len(swapped))

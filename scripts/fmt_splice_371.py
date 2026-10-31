#!/usr/bin/env python3
"""autopush-369 — THE FMT SPLICE (the 358/362/363/365/366/367/368
record's own law the assert-split folds wear): the part_11 tail the
formatted stdout splices — original[:s] + formatted[fs:] the marker
pair keeps (the doc comment head the both texts share, the hunks the
tail alone wears).
"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
PATH = "crates/mc-client/src/screens/tests/part_11.rs"
MARKER = "/// THE TABLE GRID'S OWN BANK DRAG (the autopush-371 round — the"

original = open(PATH, encoding="utf-8").read()
proc = subprocess.run(
    [RUSTFMT, "--emit", "stdout", PATH], capture_output=True, text=True)
assert proc.returncode == 0, proc.stderr
formatted = proc.stdout

s = original.find(MARKER)
assert s >= 0, "marker not in original"
fs = formatted.find(MARKER)
assert fs >= 0, "marker not in formatted"

spliced = original[:s] + formatted[fs:]
open(PATH, "w", encoding="utf-8").write(spliced)
print(f"SPLICED from byte {s} (formatted {fs}); tail bytes {len(formatted) - fs}")

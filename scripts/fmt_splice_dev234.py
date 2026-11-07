#!/usr/bin/env python3
"""dev-234 — THE FMT SPLICE (the 502/503 record's own law the round
wears): the part_11 tail's own region the rustfmt answer the ORIGINAL
file splices back — the marker start the EOF end the round keeps, the
pre-marker text the original file never touches.
"""
import os
import subprocess

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
PATH = "crates/mc-client/src/screens/tests/part_11.rs"
START = "/// THE TAKE WALK'S OWN KIND RETIRE (the dev-234 ride — the onTake's"

original = open(PATH, encoding="utf-8").read()
proc = subprocess.run([RUSTFMT, "--emit", "stdout", PATH], capture_output=True, text=True)
if proc.returncode != 0:
    raise SystemExit(f"RUSTFMT FAILED\n{proc.stderr[:400]}")
formatted = proc.stdout

s = original.find(START)
fs = formatted.find(START)
if s < 0 or fs < 0:
    raise SystemExit("MARKER NOT FOUND")

spliced = original[:s] + formatted[fs:]
open(PATH, "w", encoding="utf-8").write(spliced)
print(f"SPLICED: {len(original) - s} bytes -> {len(formatted) - fs} bytes (the tail the answer wears)")

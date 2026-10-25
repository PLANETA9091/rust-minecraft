#!/usr/bin/env python3
"""ONE gate window with the full result lines (the counts the 205 label
discards). Usage: python3 scripts/gate_window_full.py {1..11|rest}"""
import importlib.util
import subprocess
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
arg = sys.argv[1] if len(sys.argv) > 1 else "rest"
only = None if arg == "rest" else int(arg)
try:
    m.write_mod(only)
    r = subprocess.run(
        [m.CARGO, "test", "-p", "mc-client", "--", "--format=terse"],
        capture_output=True, text=True, timeout=590,
    )
    out = r.stdout + r.stderr
    for line in out.splitlines():
        low = line.lower()
        if (
            "test result" in low
            or low.startswith("running ")
            or "failed" in low
            or "panicked" in low
        ):
            print(line, flush=True)
    print(f"[window {arg} rc={r.returncode}]", flush=True)
    sys.exit(r.returncode)
finally:
    m.restore()

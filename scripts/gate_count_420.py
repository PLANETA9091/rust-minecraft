#!/usr/bin/env python3
"""autopush-420 — the count lens (the 416 round's own wrapper the round
re-wears): the part_gate window the run answers, the test-result line
the count the worklog banks. Usage: python3 scripts/gate_count_420.py 11
"""
import importlib.util
import re
import sys

spec = importlib.util.spec_from_file_location("pg", "scripts/part_gate_205.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)

arg = sys.argv[1]
rc = 0
try:
    if arg == "rest":
        rc, out = m.run(None)
        label = "rest"
    else:
        rc, out = m.run(int(arg))
        label = f"part_{arg}"
    hits = re.findall(r"test result: ok\. (\d+) passed; (\d+) failed", out)
    print(f"[{label}] rc={rc} results={hits}", flush=True)
finally:
    m.restore()
sys.exit(rc)

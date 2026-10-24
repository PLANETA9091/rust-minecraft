#!/usr/bin/env python3
"""THE CHUNK ONE (the dev-160 round): ONE chunk the foreground run
wears — the --list the names the N-sized slice keeps, the binary the
chunk runs, the result the line speaks. The walk the tool call per
chunk answers (the setsid'd child the tool session kills).

Usage: python3 scripts/chunk_one_160.py <test-binary> <chunk-index>
"""
import re
import subprocess
import sys

BIN = sys.argv[1]
CI = int(sys.argv[2])
N = 350
CWD = "/home/z/my-project/rust-minecraft"

out = subprocess.run(
    [BIN, "--list"], capture_output=True, text=True, cwd=CWD
).stdout
names = sorted(
    ln.rsplit(": test", 1)[0].strip() for ln in out.splitlines() if ln.endswith(": test")
)
chunk = names[CI * N : (CI + 1) * N]
if not chunk:
    sys.exit(f"chunk {CI} empty (total {len(names)})")
r = subprocess.run(
    [BIN] + chunk + ["--test-threads", "4"],
    capture_output=True, text=True, cwd=CWD,
)
tail = (r.stdout + r.stderr).splitlines()
res = [ln for ln in tail if ln.startswith("test result")]
joined = "\n".join(res)
m = re.search(r"(\d+) passed", joined)
p = int(m.group(1)) if m else 0
m = re.search(r"(\d+) failed", joined)
f = int(m.group(1)) if m else 0
m = re.search(r"(\d+) ignored", joined)
ig = int(m.group(1)) if m else 0
print(f"chunk {CI} ({len(chunk)} tests): {p} passed, {f} failed, {ig} ignored")
if f:
    infail = False
    for ln in tail:
        if ln.strip() == "failures:":
            infail = True; continue
        if infail and ln.startswith("    "):
            print("  FAILED:", ln.strip())
sys.exit(1 if f else 0)

#!/usr/bin/env python3
"""THE CHUNK WALK (the dev-160 round): the mc-client full run the
600s tool timeout the chunks the union answers — the --list the sorted
names the N-sized chunks wear, each chunk the BINARY run the log
appends, the union the summary speaks. The binary the cargo wrapper's
overhead never wears (the direct run the round's own lanes keep).

Usage: python3 scripts/chunk_walk_160.py <test-binary>
"""
import re
import subprocess
import sys

BIN = sys.argv[1]
N = 350
CWD = "/home/z/my-project/rust-minecraft"

out = subprocess.run(
    [BIN, "--list"], capture_output=True, text=True, cwd=CWD
).stdout
names = sorted(
    ln.rsplit(": test", 1)[0].strip() for ln in out.splitlines() if ln.endswith(": test")
)
print(f"total tests: {len(names)}")
total_passed = total_failed = total_ignored = 0
failed_names = []
for ci in range(0, len(names), N):
    chunk = names[ci : ci + N]
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
    total_passed += p; total_failed += f; total_ignored += ig
    if f:
        infail = False
        for ln in tail:
            if ln.strip() == "failures:":
                infail = True; continue
            if infail and ln.startswith("    "):
                failed_names.append(ln.strip())
    print(f"chunk {ci//N}: {p} passed, {f} failed, {ig} ignored")
    sys.stdout.flush()
print(f"UNION: {total_passed} passed, {total_failed} failed, {total_ignored} ignored")
if failed_names:
    print("FAILED NAMES:")
    for n in sorted(set(failed_names)):
        print(f"  {n}")

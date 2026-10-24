#!/usr/bin/env python3
"""THE CHUNK WALK (the autopush-190 round): the mc-client full run the
600s tool timeout the chunks the union answers — the --list the sorted
names the N-sized chunks wear, each chunk the foreground run the log
appends, the union the summary speaks."""
import subprocess
import sys

CARGO = "/home/z/.cargo/bin/cargo"
N = 350

out = subprocess.run(
    [CARGO, "test", "-p", "mc-client", "--bin", "mc-client", "--", "--list"],
    capture_output=True, text=True, cwd="/home/z/my-project/rust-minecraft",
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
        [CARGO, "test", "-p", "mc-client", "--bin", "mc-client", "--"] + chunk
        + ["--test-threads", "4"],
        capture_output=True, text=True, cwd="/home/z/my-project/rust-minecraft",
    )
    tail = (r.stdout + r.stderr).splitlines()
    res = [ln for ln in tail if ln.startswith("test result")]
    p = f = ig = 0
    for ln in res:
        for part in ln.split():
            pass
    import re
    m = re.search(r"(\d+) passed", "\n".join(res))
    p = int(m.group(1)) if m else 0
    m = re.search(r"(\d+) failed", "\n".join(res))
    f = int(m.group(1)) if m else 0
    m = re.search(r"(\d+) ignored", "\n".join(res))
    ig = int(m.group(1)) if m else 0
    total_passed += p; total_failed += f; total_ignored += ig
    if f:
        for ln in tail:
            if ln.startswith("failures:") and ln != "failures:":
                failed_names.append(ln)
        # the failure list the block's lines wear
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

#!/usr/bin/env python3
"""fmt probe for the 330 round's own part_11 tail (the two appended
hopper-highlight tests the top-level walk spans — the 329 harness the
same law keeps)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SCRATCH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "_probe_330_p11.rs")
SRC = "crates/mc-client/src/screens/tests/part_11.rs"

lines = open(SRC).read().split("\n")


def find(marker: str) -> int:
    for i, ln in enumerate(lines):
        if marker in ln:
            return i
    raise SystemExit(f"marker not found: {marker}")


# the appended region: from the first test's doc marker to EOF
a0 = find("THE HOPPER'S OWN HIGHLIGHT SEAT (the 330 round")
body = lines[a0:]
while body and body[-1].strip() == "":
    body.pop()

open(SCRATCH, "w").write("\n".join(body) + "\n")
r = subprocess.run(
    [RUSTFMT, "--edition", "2021", "--check", SCRATCH],
    capture_output=True, text=True,
)
os.remove(SCRATCH)
if r.returncode == 0:
    print("part_11 tail: CLEAN")
    sys.exit(0)
print(f"part_11 tail: DIRTY rc={r.returncode}")
print(r.stdout[:4000])
print(r.stderr[:500])
sys.exit(1)

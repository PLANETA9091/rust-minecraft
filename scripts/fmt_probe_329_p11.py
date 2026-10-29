#!/usr/bin/env python3
"""fmt probe for the 328 round's own part_11 tail (the two appended
chest-highlight tests the top-level walk spans)."""
import os
import subprocess
import sys

RUSTFMT = os.path.expanduser("~/.cargo/bin/rustfmt")
SRC = "crates/mc-client/src/screens/tests/part_11.rs"

lines = open(SRC).read().split("\n")


def find(marker: str) -> int:
    for i, ln in enumerate(lines):
        if marker in ln:
            return i
    raise SystemExit(f"marker not found: {marker}")


# the appended region: from the first test's doc marker to EOF
a0 = find("THE FURNACE'S OWN HIGHLIGHT SEAT (the 328 round")
body = lines[a0:]
while body and body[-1].strip() == "":
    body.pop()

open("/tmp/probe_328_p11.rs", "w").write("\n".join(body) + "\n")
r = subprocess.run(
    [RUSTFMT, "--edition", "2021", "--check", "/tmp/probe_328_p11.rs"],
    capture_output=True, text=True,
)
if r.returncode == 0:
    print("part_11 tail: CLEAN")
    sys.exit(0)
print(f"part_11 tail: DIRTY rc={r.returncode}")
print(r.stdout[:4000])
print(r.stderr[:500])
sys.exit(1)

#!/usr/bin/env python3
"""THE CONTAINER FAMILY'S OWN BLAST RESISTANCE v2 (dev-77): walk the
Blocks clinit (26.2 jar, sha1 2dc72797) BACK from each family field's
putstatic register to the nearest Properties.strength call and speak
the float + pc — the (hardness, resistance) pair the single-arg
strength row keeps (both fields the same value).

Usage: python3 scripts/jar_work/blastresist_704.py <client.jar>
"""
import re
import subprocess
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else "tools/client.jar"
z = zipfile.ZipFile(JAR)

data = z.read("net/minecraft/world/level/block/Blocks.class")
open("/tmp/_blocks.class", "wb").write(data)
out = subprocess.run(
    ["javap", "-c", "-p", "/tmp/_blocks.class"], capture_output=True, text=True
).stdout
lines = out.splitlines()

FAMILY = [
    "CHEST",
    "TRAPPED_CHEST",
    "ENDER_CHEST",
    "FURNACE",
    "BLAST_FURNACE",
    "SMOKER",
]

for fam in FAMILY:
    reg = None
    for i, line in enumerate(lines):
        if f"putstatic" in line and f"Field {fam}:" in line:
            reg = i
    if reg is None:
        print(f"{fam}: NO REGISTER FOUND")
        continue
    # walk back to the nearest strength call
    for j in range(reg, max(0, reg - 40), -1):
        s = lines[j].strip()
        m = re.search(r"float ([0-9.]+)f", s)
        if "strength" in s and m:
            pc = s.split(":")[0]
            print(f"{fam}: strength({m.group(1)}) @ pc {pc} (register pc "
                  f"{lines[reg].strip().split(':')[0]})")
            break
    else:
        print(f"{fam}: no strength row within 40 lines of register — "
              f"instabreak/default family? (dump tail)")
        for k in range(reg - 12, reg + 1):
            print("   ", lines[k].strip())

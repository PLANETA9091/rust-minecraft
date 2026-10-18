#!/usr/bin/env python3
"""The stairs property chains v2: window = (previous register call,
next register call); every stairs row's full chain is inside."""
import re
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import struct  # noqa: E402
import zipfile  # noqa: E402

z = zipfile.ZipFile(".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785")
clinit = list(clinit_lines(z, "net.minecraft.world.level.block.Blocks"))

reg_idx = [i for i, l in enumerate(clinit) if "Blocks.register:" in l]
print(f"register calls: {len(reg_idx)}")

def fconst_val(l):
    m = re.search(r"fconst_(\d)", l)
    if m:
        return float(m.group(1))
    m = re.search(r"ldc_w? float ([\d.eE+-]+)", l)
    if m:
        return float(m.group(1))
    m = re.search(r"ldc2_w double ([\d.eE+-]+)", l)
    if m:
        return float(m.group(1))
    return None

out = []
for k, i in enumerate(reg_idx):
    window = clinit[reg_idx[k - 1] + 1 if k else 0 : i + 1]
    joined = "\n".join(window)
    m = re.search(r"references/BlockItemIds\.(\w+)", joined)
    if not m or not m.group(1).endswith("STAIRS"):
        continue
    name = m.group(1)
    sounds = re.findall(r"SoundType\.(\w+)", joined)
    strengths = []
    for j, l in enumerate(window):
        if "Properties.strength:" in l or "strength:(FF)" in l:
            vals = []
            # floats are the two preceding const loads
            for back in range(j - 1, max(j - 5, -1), -1):
                v = fconst_val(window[back])
                if v is not None:
                    vals.insert(0, v)
                if len(vals) == 2:
                    break
            strengths.append(tuple(vals))
    flags = []
    for flag in ("requiresCorrectToolForDrops", "noOcclusion", "pushReaction",
                 "forceSolidOn", "ignitedByLava", "randomTicks", "requiresTool",
                 "instabreak", "noLootTable"):
        if flag in joined:
            flags.append(flag)
    lams = re.findall(r"lambda\$static\$\d+", joined)
    ctor = "WeatheringCopperStairBlock" if "WeatheringCopperStairBlock" in joined else (
           "StairBlock" if "StairBlock" in joined else "?")
    out.append((name, sounds, strengths, flags, ctor, len(lams)))

for name, sounds, strengths, flags, ctor, n in out:
    print(f"{name}: sound={sounds} strength={strengths} ctor={ctor} x{n} flags={flags}")
print(f"TOTAL stairs rows: {len(out)}")

print()
print("=== copper collection row (CUT_COPPER_STAIRS walk) ===")
for name, sounds, strengths, flags, ctor, n in out:
    if name == "CUT_COPPER_STAIRS":
        idx = [i for i, l in enumerate(clinit)
               if "Blocks.register:" in l]
        # print the raw window
        for k, i in enumerate(idx):
            window = clinit[idx[k - 1] + 1 if k else 0 : i + 1]
            if "CUT_COPPER_STAIRS" in "\n".join(window):
                for l in window:
                    print("   ", l)
                break

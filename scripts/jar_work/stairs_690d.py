#!/usr/bin/env python3
"""The stairs property chains: for each of the 64 *_stairs rows in
Blocks clinit, print the full property chain (mapColor / forceSolidOn /
instrument / strength / sound / requiresCorrectTool / ignitedByLava /
pushReaction / noOcclusion + which constructor lambda serves)."""
import re
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

z = zipfile.ZipFile(".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785")
clinit = list(clinit_lines(z, "net.minecraft.world.level.block.Blocks"))

# find register rows: invokestatic Blocks.register with BlockItemIds.<NAME>_STAIRS
rows = []
for i, l in enumerate(clinit):
    m = re.search(r"references/BlockItemIds\.(\w*_STAIRS)", l)
    if m:
        rows.append((m.group(1), i))

print(f"rows found: {len(rows)}")
INTEREST = ("sound", "strength", "requiresCorrectTool", "noOcclusion",
            "pushReaction", "forceSolidOn", "mapColor", "instrument",
            "ignitedByLava", "lightLevel", "requiresTool", "randomTicks",
            "register", "indy", "lambda", "instabreak", "friction", "speedFactor")
for name, idx in rows:
    # walk back to the previous register/putstatic boundary (row start)
    start = idx
    j = idx - 1
    depth = 0
    while j > 0:
        l = clinit[j]
        if ("register:" in l or "putstatic" in l) and depth == 0 and j < idx - 8:
            start = j + 1
            break
        j -= 1
    window = clinit[start:idx + 2]
    parts = []
    for l in window:
        for key in INTEREST:
            if key in l:
                # condense: SoundType.X / strength(f, f) / register(BlockItemIds.Y)
                if key == "sound":
                    mm = re.search(r"SoundType\.(\w+)", l)
                    if mm:
                        parts.append(f"sound={mm.group(1)}")
                elif key == "strength":
                    mm = re.search(r"strength", l)
                    if mm:
                        parts.append("strength()")
                elif key == "register":
                    mm = re.search(r"BlockItemIds\.(\w+)", l)
                    if mm:
                        parts.append(f"register({mm.group(1)})")
                elif key == "mapColor":
                    mm = re.search(r"MapColor\.(\w+)|material/MapColor", l)
                    parts.append("mapColor")
                elif key == "instrument":
                    mm = re.search(r"NoteBlockInstrument\.(\w+)", l)
                    if mm:
                        parts.append(f"instr={mm.group(1)}")
                else:
                    parts.append(key)
                break
    # strength float values from raw window
    floats = []
    for l in window:
        mm = re.search(r"fconst_(\d)|ldc(?:_w)? (?:float )([\d.]+)", l)
        if mm and "strength" in clinit[clinit.index(l) + 1] if l in clinit else False:
            pass
    print(f"{name}: " + " | ".join(parts))

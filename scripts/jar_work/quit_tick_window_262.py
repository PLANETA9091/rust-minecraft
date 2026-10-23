#!/usr/bin/env python3
"""autopush-170 recon followup: dump Minecraft.tick() pc window around
the Options.save call (the guard the save's own law wears)."""
import sys, struct
sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')
ms = methods_of(z, 'net.minecraft.client.Minecraft')
cp, body = ms['tick()V']
clen = struct.unpack_from('>I', body, 4)[0]
code = body[8:8 + clen]
lines = walk_code(cp, code)
print(f"total instrs: {len(lines)}")
start = 0
for i, l in enumerate(lines):
    if 'Options.save' in l:
        start = i
        break
lo = max(0, start - 22)
for l in lines[lo:start + 6]:
    print(l)

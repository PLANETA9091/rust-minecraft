#!/usr/bin/env python3
"""autopush-219 recon part b: the KeyMapping.shouldSetOnIngameFocus gate
the subclasses the override map speaks + the setScreen signature the
26.x class the package move wears + MouseHandler.grabMouse whole walk."""
import struct
import sys

sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')

# 1. KeyMapping.shouldSetOnIngameFocus body
ms = methods_of(z, 'net.minecraft.client.KeyMapping')
for sig in sorted(ms):
    if 'nFocus' in sig or 'ategory' in sig:
        print("  KeyMapping:", sig)
cp, body = ms['shouldSetOnIngameFocus()Z']
if body is None:
    print("--- KeyMapping.shouldSetOnIngameFocus: abstract")
else:
    clen = struct.unpack_from('>I', body, 4)[0]
    code = body[8:8 + clen]
    print(f"--- KeyMapping.shouldSetOnIngameFocus ({len(code)} bytes)")
    for l in walk_code(cp, code):
        print("  ", l)
print()

# 2. subclasses of KeyMapping (the override map) — scan names in the jar
print("=== KeyMapping subclasses (jar name scan):")
for n in z.namelist():
    if n.endswith('.class') and 'KeyMapping' in n:
        print("  ", n)
print()

# 3. MouseHandler.grabMouse whole walk
ms2 = methods_of(z, 'net.minecraft.client.MouseHandler')
cp, body = ms2['grabMouse()V']
clen = struct.unpack_from('>I', body, 4)[0]
code = body[8:8 + clen]
print(f"--- MouseHandler.grabMouse ({len(code)} bytes)")
for l in walk_code(cp, code):
    print("  ", l)
print()

# 4. Minecraft screen-setter: methods containing 'creen'
ms3 = methods_of(z, 'net.minecraft.client.Minecraft')
print("=== Minecraft screen-named methods:")
for sig in sorted(ms3):
    if 'creen' in sig.lower():
        print("  ", sig)

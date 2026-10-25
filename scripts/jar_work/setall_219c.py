#!/usr/bin/env python3
"""autopush-219 recon part c: ToggleKeyMapping's own override + Gui.
setScreen (the releaseAll lane) + the releaseAll caller map the open
side wears."""
import struct
import sys

sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')

# 1. ToggleKeyMapping — whole class methods
ms = methods_of(z, 'net.minecraft.client.ToggleKeyMapping')
print("=== ToggleKeyMapping methods:")
for sig in sorted(ms):
    print("  ", sig)
for sig in sorted(ms):
    if 'nFocus' in sig or 'isDown' in sig:
        cp, body = ms[sig]
        if body is None:
            print(f"--- ToggleKeyMapping.{sig}: abstract")
            continue
        clen = struct.unpack_from('>I', body, 4)[0]
        code = body[8:8 + clen]
        print(f"--- ToggleKeyMapping.{sig} ({len(code)} bytes)")
        for l in walk_code(cp, code):
            print("  ", l)
        print()

# 2. Gui.setScreen walk
ms2 = methods_of(z, 'net.minecraft.client.gui.Gui')
for sig in sorted(ms2):
    if 'setScreen' in sig:
        print("  Gui:", sig)
cp, body = ms2['setScreen(Lnet/minecraft/client/gui/screens/Screen;)V']
if body is not None:
    clen = struct.unpack_from('>I', body, 4)[0]
    code = body[8:8 + clen]
    print(f"--- Gui.setScreen ({len(code)} bytes)")
    for l in walk_code(cp, code):
        print("  ", l)
print()

# 3. releaseAll callers across the gui/client family
print("=== releaseAll callers:")
for cls in ('net.minecraft.client.gui.Gui', 'net.minecraft.client.Minecraft',
            'net.minecraft.client.MouseHandler', 'net.minecraft.client.KeyboardHandler'):
    ms3 = methods_of(z, cls)
    if not ms3:
        continue
    for sig, (cp3, body3) in sorted(ms3.items()):
        if body3 is None:
            continue
        clen = struct.unpack_from('>I', body3, 4)[0]
        code = body3[8:8 + clen]
        try:
            lines = walk_code(cp3, code)
        except Exception:
            continue
        hits = [l for l in lines if '.releaseAll' in l]
        if hits:
            print(f"  {cls}.{sig}:")
            for h in hits:
                print("   ", h)

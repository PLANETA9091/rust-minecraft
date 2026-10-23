#!/usr/bin/env python3
"""autopush-173 recon: the vanilla Minecraft.setWindowActive walk —
the focus-change law the pause (or its absence) the bytecode speaks."""
import sys, struct
sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')
ms = methods_of(z, 'net.minecraft.client.Minecraft')
print("=== windowActive/focus-named methods:")
for sig in sorted(ms):
    if 'indow' in sig.lower() or 'ctive' in sig.lower() or 'ocus' in sig.lower():
        print("  ", sig)

for sig in ('pauseIfInactive()V', 'isWindowActive()Z'):
    if sig not in ms:
        continue
    cp, body = ms[sig]
    if body is None:
        print(f"--- Minecraft.{sig}: abstract/native")
        continue
    clen = struct.unpack_from('>I', body, 4)[0]
    code = body[8:8 + clen]
    print(f"--- Minecraft.{sig} ({len(code)} bytes)")
    for l in walk_code(cp, code):
        print(l)
    print()

# who calls pauseIfInactive?
print("=== pauseIfInactive callers:")
for cls in ('net.minecraft.client.Minecraft',):
    for s, (cp2, body) in ms.items():
        if body is None:
            continue
        clen = struct.unpack_from('>I', body, 4)[0]
        code = body[8:8 + clen]
        try:
            lines = walk_code(cp2, code)
        except Exception:
            continue
        hits = [l for l in lines if 'pauseIfInactive' in l]
        if hits:
            print(f"  Minecraft.{s}:")
            for h in hits:
                print("   ", h)

#!/usr/bin/env python3
"""autopush-170 recon final: OptionsScreen save sites — which methods
wear Options.save (the Done/Escape onClose chord's own shape)."""
import sys, struct
sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')
cls = 'net.minecraft.client.gui.screens.options.OptionsScreen'
ms = methods_of(z, cls)
print("=== OptionsScreen methods:", sorted(ms.keys()))
for sig, (cp, body) in ms.items():
    if body is None:
        continue
    clen = struct.unpack_from('>I', body, 4)[0]
    code = body[8:8 + clen]
    try:
        lines = walk_code(cp, code)
    except Exception:
        continue
    hits = [l for l in lines if 'save' in l.lower() or 'onClose' in l]
    if hits:
        print(f"--- {sig}:")
        for h in hits:
            print("   ", h)

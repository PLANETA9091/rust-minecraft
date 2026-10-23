#!/usr/bin/env python3
"""autopush-173 recon 2: Minecraft.pauseGame(Z)V — the pause's own gates
(SP/MP) and the arg's meaning."""
import sys, struct
sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')
ms = methods_of(z, 'net.minecraft.client.Minecraft')
for sig in sorted(ms):
    if sig.startswith('pauseGame'):
        cp, body = ms[sig]
        if body is None:
            print(f"--- {sig}: abstract/native")
            continue
        clen = struct.unpack_from('>I', body, 4)[0]
        code = body[8:8 + clen]
        print(f"--- Minecraft.{sig} ({len(code)} bytes)")
        for l in walk_code(cp, code):
            print(l)

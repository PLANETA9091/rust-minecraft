#!/usr/bin/env python3
"""autopush-264 recon: the createPauseMenu() OWN GRID — the 26.2 jar's
PauseScreen.createPauseMenu the GridLayout the rows the bounds speak.
javap the bytecode the exact law answers (the 204w backtogame, the
adv/stats pair, the sprite row, the options pair, the disconnect)."""
import sys, struct
sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')
ms = methods_of(z, 'net.minecraft.client.gui.screens.PauseScreen')
for sig in sorted(ms):
    if 'createPauseMenu' in sig:
        cp, body = ms[sig]
        clen = struct.unpack_from('>I', body, 4)[0]
        code = body[8:8 + clen]
        print(f"--- PauseScreen.{sig} ({len(code)} bytes)")
        for l in walk_code(cp, code):
            print(l)

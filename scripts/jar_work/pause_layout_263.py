#!/usr/bin/env python3
"""autopush-263 recon: the PauseScreen's OWN LAYOUT — the 26.2 jar's
PauseScreen.init the buttons the bounds the rows speak (the
advancements/stats pair the vanilla law banks). javap the ctor + init
the bytecode the exact grid answers."""
import sys, struct
sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')
ms = methods_of(z, 'net.minecraft.client.gui.screens.PauseScreen')
for sig in sorted(ms):
    if sig.startswith('init(') or sig.startswith('<init>'):
        cp, body = ms[sig]
        if body is None:
            print(f"--- PauseScreen.{sig}: abstract/native")
            continue
        clen = struct.unpack_from('>I', body, 4)[0]
        code = body[8:8 + clen]
        print(f"--- PauseScreen.{sig} ({len(code)} bytes)")
        for l in walk_code(cp, code):
            print(l)

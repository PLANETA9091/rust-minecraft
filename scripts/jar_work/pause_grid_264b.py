#!/usr/bin/env python3
"""autopush-264 recon 2: the GridLayout OWN ARITHMETIC — arrangeElements +
CellInhabitant the span/stretch law the pause grid wears; openScreenButton
the bounds law; the SpriteIconButton builder defaults."""
import sys, struct
sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')

targets = [
    ('net.minecraft.client.gui.layouts.GridLayout', 'arrangeElements'),
    ('net.minecraft.client.gui.layouts.GridLayout$CellInhabitant', None),
    ('net.minecraft.client.gui.screens.PauseScreen', 'openScreenButton'),
]
for cls, want in targets:
    ms = methods_of(z, cls)
    for sig in sorted(ms):
        if want is not None and want not in sig:
            continue
        cp, body = ms[sig]
        if body is None:
            print(f"--- {cls}.{sig}: abstract/native")
            continue
        clen = struct.unpack_from('>I', body, 4)[0]
        code = body[8:8 + clen]
        print(f"--- {cls}.{sig} ({len(code)} bytes)")
        for l in walk_code(cp, code):
            print(l)

#!/usr/bin/env python3
"""autopush-264 recon 3: GridLayout$ChildContainer setX/setY the stretch law;
PauseScreen.openScreenButton the bounds law; the player-reporting ldcw sprite."""
import sys, struct
sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')

def dump(cls, only=None):
    try:
        ms = methods_of(z, cls)
    except Exception as e:
        print(f"--- {cls}: ERR {e}")
        return
    for sig in sorted(ms):
        if only and not any(o in sig for o in only):
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

dump('net.minecraft.client.gui.layouts.GridLayout$ChildContainer', ['setX', 'setY', 'getWidth', 'getHeight'])
dump('net.minecraft.client.gui.screens.PauseScreen', ['openScreenButton'])

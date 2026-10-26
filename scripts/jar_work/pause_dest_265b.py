#!/usr/bin/env python3
"""autopush-265 recon 2: the HeaderAndFooterLayout + MenuTabBar geometry
laws the destination screens wear; the StatsScreen field keys."""
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
            continue
        clen = struct.unpack_from('>I', body, 4)[0]
        code = body[8:8 + clen]
        if len(code) > 2200:
            print(f"--- {cls.split('/')[-1]}.{sig} ({len(code)} bytes) SKIPPED-LARGE")
            continue
        print(f"--- {cls.split('/')[-1]}.{sig} ({len(code)} bytes)")
        for l in walk_code(cp, code):
            print(l)

dump('net/minecraft/client/gui/layouts/HeaderAndFooterLayout', ['addTitleHeader', 'addToFooter', 'arrangeElements'])
dump('net/minecraft/client/gui/screens/achievement/StatsScreen', ['<init>(Lnet/minecraft/client/gui/screens/Screen;Lnet/minecraft/stats/StatsCounter'])

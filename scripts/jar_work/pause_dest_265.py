#!/usr/bin/env python3
"""autopush-265 recon: the PAUSE DESTINATIONS' OWN SCREENS — the 26.2 jar's
AdvancementsScreen + StatsScreen the init/window/tab laws speak."""
import sys, struct
sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')

def dump(cls, only=None, maxb=40):
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
        if len(code) > 3000:
            continue
        print(f"--- {cls.split('/')[-1]}.{sig} ({len(code)} bytes)")
        for l in walk_code(cp, code):
            print(l)

dump('net/minecraft/client/gui/screens/advancements/AdvancementsScreen', ['init()', '<init>'])
dump('net/minecraft/client/gui/screens/achievement/StatsScreen', ['init()'])

#!/usr/bin/env python3
"""autopush-174 recon: resolve the processOptions key string for
pauseOnLostFocus (ldcw@1908) + find the runtime toggle site (the F3+P
debug chord the 26.2 key.debug.focusPause law wears)."""
import sys, struct
sys.path.insert(0, 'scripts/jar_work')
_helpers = open('scripts/jar_work/glass_685.py').read().split('z = zipfile.ZipFile(JAR)')[0]
exec(_helpers)
import zipfile

z = zipfile.ZipFile('.mc2rs/classpath/client.jar')

# 1. resolve the key string in Options.processOptions
ms = methods_of(z, 'net.minecraft.client.Options')
cp, body = ms['processOptions(Lnet/minecraft/client/Options$FieldAccess;)V']
clen = struct.unpack_from('>I', body, 4)[0]
code = body[8:8 + clen]
lines = walk_code(cp, code)
for i, l in enumerate(lines):
    if 'auseOnLostFocus' in l:
        # the ldc_w key line sits 2-5 lines above
        for c in lines[max(0, i - 6):i]:
            if 'ldcw' in c and ('pause' in c.lower() or 'lost' in c.lower()):
                print('KEY STRING:', c)
print('(no ldcw print above = key is a nearby non-matching ldcw)')

# 2. the toggle site: who calls Options.pauseOnLostFocus putfield outside ctor/processOptions
for cls in ('net.minecraft.client.Minecraft',):
    ms2 = methods_of(z, cls)
    for sig, (cp2, body) in sorted(ms2.items()):
        if body is None:
            continue
        clen = struct.unpack_from('>I', body, 4)[0]
        code = body[8:8 + clen]
        try:
            lines = walk_code(cp2, code)
        except Exception:
            continue
        for i, l in enumerate(lines):
            if 'auseOnLostFocus' in l and 'putfield' in l:
                print(f'=== Minecraft.{sig} pc {l.split(":")[0]}:')
                for c in lines[max(0, i - 8):i + 1]:
                    print('   ', c)
                print()

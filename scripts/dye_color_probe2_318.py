#!/usr/bin/env python3
"""autopush-318 probe 2 (final) — the 16 DyeColor rows' textureDiffuseColor
the raw CP ints resolve. Exit 0 iff the 16 rows print."""
import struct
import sys
import zipfile

sys.path.insert(0, 'scripts')
from raw_disasm import parse_class, disasm

data = zipfile.ZipFile('.mc2rs/classpath/client.jar').read(
    'net/minecraft/world/item/DyeColor.class')
lines = None
for mname, mdesc, mcode, mcp in parse_class(data, None):
    if mname == '<clinit>':
        lines = list(disasm(mcode, mcp))

p = 10
(n,) = struct.unpack_from('>H', data, 8)
cp = [None] * n
i = 1
while i < n:
    tag = data[p]
    if tag == 1:
        (l,) = struct.unpack_from('>H', data, p + 1)
        cp[i] = data[p + 3:p + 3 + l].decode('utf-8', 'replace')
        p += 3 + l
    elif tag == 3:
        (v,) = struct.unpack_from('>i', data, p + 1)
        cp[i] = ('int', v)
        p += 5
    elif tag == 4:
        p += 5
    elif tag == 5:
        p += 9
        i += 1
    elif tag == 6:
        p += 9
        i += 1
    elif tag in (7, 8, 16, 19, 20):
        p += 3
    elif tag in (9, 10, 11, 12, 17, 18):
        p += 5
    elif tag == 15:
        p += 4
    else:
        raise Exception(f'tag {tag} at {p}')
    i += 1

ICONST = {'iconst_0': 0, 'iconst_1': 1, 'iconst_2': 2, 'iconst_3': 3,
          'iconst_4': 4, 'iconst_5': 5, 'iconst_m1': -1}
cur = []
count = 0
for ln in lines:
    s = ln.strip()
    op = s.split()[0] if s else ''
    if op in ('ldc_w', 'ldc'):
        idx = int(s.split('#')[1].split()[0])
        cur.append(cp[idx])
    elif op in ('sipush', 'bipush'):
        cur.append(('int', int(s.split()[1])))
    elif op in ICONST:
        cur.append(('int', ICONST[op]))
    elif 'putstatic' in ln and 'DyeColor.' in ln:
        name = ln.split('DyeColor.')[1].split()[0]
        intvals = [v[1] for v in cur
                   if isinstance(v, tuple) and v[0] == 'int']
        if name.isupper() and len(intvals) >= 3:
            print(f"{name:12s} diffuse=0x{intvals[0] & 0xFFFFFF:06X} "
                  f"firework=0x{intvals[1] & 0xFFFFFF:06X} "
                  f"text=0x{intvals[2] & 0xFFFFFF:06X}")
            count += 1
        cur = []
print(f"== {count} rows ==")
sys.exit(0 if count == 16 else 1)

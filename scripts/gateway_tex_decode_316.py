#!/usr/bin/env python3
"""autopush-316 — the gateway beam texture's own pixels decode (the
palette rows the beam's vertical band look speaks)."""
import struct
import zlib
import zipfile

zf = zipfile.ZipFile('.mc2rs/classpath/client.jar')
d = zf.read('assets/minecraft/textures/entity/end_portal/end_gateway_beam.png')
w, h = 16, 16
pos = 8
idat = b''
plte = None
trns = None
while pos < len(d):
    ln = struct.unpack_from('>I', d, pos)[0]
    typ = d[pos + 4:pos + 8]
    if typ == b'IDAT':
        idat += d[pos + 8:pos + 8 + ln]
    if typ == b'PLTE':
        plte = d[pos + 8:pos + 8 + ln]
    if typ == b'tRNS':
        trns = d[pos + 8:pos + 8 + ln]
    pos += 12 + ln
raw = zlib.decompress(idat)
stride = w + 1
rows = []
prev = [0] * w


def paeth(a, b, c):
    p = a + b - c
    pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
    return a if pa <= pb and pa <= pc else (b if pb <= pc else c)


for y in range(h):
    f = raw[y * stride]
    line = list(raw[y * stride + 1:(y + 1) * stride])
    for x in range(w):
        a = line[x - 1] if x >= 1 else 0
        b = prev[x]
        c = prev[x - 1] if x >= 1 else 0
        if f == 0:
            pass
        elif f == 1:
            line[x] = (line[x] + a) & 255
        elif f == 2:
            line[x] = (line[x] + b) & 255
        elif f == 3:
            line[x] = (line[x] + (a + b) // 2) & 255
        elif f == 4:
            line[x] = (line[x] + paeth(a, b, c)) & 255
    prev = line
    rows.append(line)

print('palette entries:', len(plte) // 3, 'trns len:', len(trns) if trns else 0)
for i in range(len(plte) // 3):
    a = trns[i] if trns and i < len(trns) else 255
    print(f'pal[{i:2d}] = {plte[i*3]:02X}{plte[i*3+1]:02X}{plte[i*3+2]:02X}{a:02X}')
for y in range(0, 16, 2):
    print(f'y={y:2d}:', ' '.join(f'{rows[y][x]:2d}' for x in range(16)))

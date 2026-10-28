#!/usr/bin/env python3
"""autopush-316 — scout 10: bootstrap 1/2 impl lambdas resolve (the
glow/solid pairing)."""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from full_disasm import parse_cp, utf8

zf = zipfile.ZipFile('.mc2rs/classpath/client.jar')
data = zf.read('net/minecraft/client/renderer/blockentity/BeaconRenderer.class')
cp, q = parse_cp(data)

target = None
for k, e in cp.items():
    if isinstance(e, tuple) and e and e[0] == "utf8" and e[1] == 'BootstrapMethods':
        target = k
        break
print('BootstrapMethods cp index:', target)

q += 6
(nif,) = struct.unpack_from('>H', data, q); q += 2 + 2 * nif
(nfields,) = struct.unpack_from('>H', data, q); q += 2
for _ in range(nfields):
    q += 6
    (na,) = struct.unpack_from('>H', data, q); q += 2
    for _ in range(na):
        q += 2
        (alen,) = struct.unpack_from('>I', data, q)
        q += 4 + alen
(nmethods,) = struct.unpack_from('>H', data, q); q += 2
for _ in range(nmethods):
    q += 6
    (na,) = struct.unpack_from('>H', data, q); q += 2
    for _ in range(na):
        q += 2
        (alen,) = struct.unpack_from('>I', data, q)
        q += 4 + alen
(ncls,) = struct.unpack_from('>H', data, q); q += 2
for _ in range(ncls):
    name_idx = struct.unpack_from('>H', data, q)[0]
    q += 2
    (alen,) = struct.unpack_from('>I', data, q)
    q += 4
    if name_idx == target:
        body = data[q:q + alen]
        (nb,) = struct.unpack_from('>H', body, 0)
        off = 2
        for b in range(nb):
            mh_idx, nargs = struct.unpack_from('>HH', body, off); off += 4
            args = []
            for _a in range(nargs):
                args.append(struct.unpack_from('>H', body, off)[0])
                off += 2
            if b not in (1, 2):
                continue
            print(f'bsm {b}: {nargs} args')
            for a in args:
                e = cp.get(a)
                if isinstance(e, tuple) and e and e[0] == 15:
                    ref = cp.get(e[2])
                    if isinstance(ref, tuple) and ref and ref[0] == 'method':
                        cls_i, nt_i = ref[1]
                        nt = cp.get(nt_i)
                        print(f'   #{a}: kind={e[1]} -> lambda {utf8(cp, nt[1][0])}')
                    else:
                        print(f'   #{a}: kind={e[1]} ref={ref}')
                else:
                    print(f'   #{a}:', e)
    q += alen

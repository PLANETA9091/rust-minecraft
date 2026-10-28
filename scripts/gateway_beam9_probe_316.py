#!/usr/bin/env python3
"""autopush-316 — scout 9: the bootstrap 1/2 impl lambdas resolve fully."""
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
import struct
from full_disasm import parse_cp, utf8

zf = zipfile.ZipFile('.mc2rs/classpath/client.jar')
data = zf.read('net/minecraft/client/renderer/blockentity/BeaconRenderer.class')
cp, q = parse_cp(data)


def skip_class_attr(data, cp, q):
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
        if utf8(cp, name_idx) == 'BootstrapMethods':
            return q
    return None


pos = skip_class_attr(data, cp, q)
(nb,) = struct.unpack_from('>H', data, pos); pos += 2
for b in range(nb):
    mh_idx, nargs = struct.unpack_from('>HH', data, pos); pos += 4
    args = []
    for _a in range(nargs):
        args.append(struct.unpack_from('>H', data, pos)[0])
        pos += 2
    if b not in (1, 2):
        continue
    print(f'bsm {b}: {nargs} args')
    for a in args:
        e = cp.get(a)
        if isinstance(e, tuple) and e and e[0] == 15:
            kind = e[1]
            ref = cp.get(e[2])
            if ref and ref[0] == 'method':
                cls_i, nt_i = ref[1]
                nt = cp.get(nt_i)
                cls = cp.get(cls_i)
                print(f'   #{a}: MethodHandle kind={kind} -> {utf8(cp, cls[1]) if cls else "?"}.{utf8(cp, nt[1][0])}')
            else:
                print(f'   #{a}: MethodHandle kind={kind} ref={ref}')
        else:
            print(f'   #{a}:', e)

#!/usr/bin/env python3
"""autopush-316 — scout 4: resolve the BeaconRenderer float constants
(the ldc pool rows the beam geometry law speaks) + the CompileTime
static finals.

Exit 0 iff the walks parse.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'


def floats(cp):
    out = {}
    for k, e in cp.items():
        if isinstance(e, tuple) and e and e[0] == 4:  # Float
            out[k] = e[1]
    return out


def main():
    zf = zipfile.ZipFile(JAR)
    data = zf.read('net/minecraft/client/renderer/blockentity/BeaconRenderer.class')
    cp, _ = parse_cp(data)
    fl = floats(cp)
    print("=== the BeaconRenderer float pool ===")
    for k in sorted(fl):
        v = fl[k]
        print(f"  #{k} = {v!r}")
    # the named lookups the disasm speaks
    print("\n=== the named rows ===")
    for idx, label in [('#139', 'rot-x-pi?'), ('#140', 'rot-x-mul?'), ('#155', 'frac-mul?'),
                       ('#172', 'rot-deg?'), ('#173', 'rot-deg2?'), ('#182', 'v-base?'),
                       ('#183', 'v-scroll?'), ('#238', 'light?'), ('#107', 'beam-radius-div?')]:
        n = int(idx[1:])
        if n in fl:
            print(f"  {idx} ({label}) = {fl[n]!r}")
        elif n in cp:
            print(f"  {idx} ({label}) = {cp[n]!r}")
        else:
            print(f"  {idx} ({label}) = NOT IN POOL")
    # the static-final fields' ConstantValue attributes
    print("\n=== the CompileTime fields ===")
    _, q = parse_cp(data)
    q += 6
    (nif,) = struct.unpack_from('>H', data, q); q += 2 + 2 * nif
    (nfields,) = struct.unpack_from('>H', data, q); q += 2
    for _ in range(nfields):
        fflags, fname_i, fdesc_i = struct.unpack_from('>HHH', data, q)
        q += 6
        (na,) = struct.unpack_from('>H', data, q); q += 2
        name = utf8(cp, fname_i)
        for _ in range(na):
            aname_i, alen = struct.unpack_from('>HI', data, q)
            q += 6
            body = data[q:q + alen]
            q += alen
            aname = utf8(cp, aname_i)
            if aname == 'ConstantValue':
                (cvi,) = struct.unpack_from('>H', body, 0)
                print(f"  {name} = {cp.get(cvi)!r}")


if __name__ == '__main__':
    main()

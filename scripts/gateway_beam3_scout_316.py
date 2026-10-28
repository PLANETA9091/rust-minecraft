#!/usr/bin/env python3
"""autopush-316 — scout 3: BeaconRenderer.submitBeaconBeam (the beam
geometry law the gateway's own submit rides) + the beam texture's own
pixels.

Exit 0 iff the walks parse.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'

CLASSES = [
    'net/minecraft/client/renderer/blockentity/BeaconRenderer',
]


def main():
    zf = zipfile.ZipFile(JAR)
    # the beam texture's own size + the pixel rows
    print("=== the beam texture ===")
    d = zf.read('assets/minecraft/textures/entity/end_portal/end_gateway_beam.png')
    print(f"end_gateway_beam.png: {len(d)} bytes, w={struct.unpack_from('>I', d, 16)[0]} h={struct.unpack_from('>I', d, 20)[0]}")
    try:
        d2 = zf.read('assets/minecraft/textures/entity/beacon_beam.png')
        print(f"beacon_beam.png: {len(d2)} bytes, w={struct.unpack_from('>I', d2, 16)[0]} h={struct.unpack_from('>I', d2, 20)[0]}")
    except KeyError:
        print("beacon_beam.png MISSING")

    for cls in CLASSES:
        print(f"\n########## {cls}")
        data = zf.read(cls + '.class')
        cp, q = parse_cp(data)
        q += 6
        (nif,) = struct.unpack_from('>H', data, q); q += 2 + 2 * nif
        (nfields,) = struct.unpack_from('>H', data, q); q += 2
        for _ in range(nfields):
            fflags, fname_i, fdesc_i = struct.unpack_from('>HHH', data, q)
            print(f"  FIELD {utf8(cp, fname_i)}")
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
        for mname, mdesc, mcode, mcp in parse_class(data, None):
            print(f"--- {mname} {mdesc}")
            if mcode is None:
                continue
            for ln in disasm(mcode, mcp):
                print(ln)


if __name__ == '__main__':
    main()

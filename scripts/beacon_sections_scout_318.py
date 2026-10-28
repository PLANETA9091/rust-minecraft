#!/usr/bin/env python3
"""autopush-318 — THE BEACON'S OWN BEAM SECTIONS scout (the evidence walk).

  - BeaconBlockEntity — the beam sections walk (the client tick's own
    color ladder: the stained-glass multiplier, the mix fold, the
    opaque stop), the BeaconBeamSection record,
  - BeaconRenderState — the sections list the renderer reads,
  - BeaconRenderer.submit — the per-section beam consumption.

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
    'net/minecraft/world/level/block/entity/BeaconBlockEntity',
    'net/minecraft/world/level/block/entity/BeaconBlockEntity$BeaconBeamSection',
    'net/minecraft/client/renderer/blockentity/state/BeaconRenderState',
    'net/minecraft/client/renderer/blockentity/BeaconRenderer',
]


def dump_class(zf, cls):
    print(f"\n########## {cls}")
    try:
        data = zf.read(cls + '.class')
    except KeyError:
        print('!! MISSING')
        return
    cp, _ = parse_cp(data)
    _, q = parse_cp(data)
    q += 6
    (nif,) = struct.unpack_from('>H', data, q); q += 2 + 2 * nif
    (nfields,) = struct.unpack_from('>H', data, q); q += 2
    for _ in range(nfields):
        fflags, fname, fdesc = struct.unpack_from('>HHH', data, q)
        q += 6
        print(f"  FIELD {utf8(cp, fname)} {utf8(cp, fdesc)}")
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


def main():
    zf = zipfile.ZipFile(JAR)
    for cls in CLASSES:
        dump_class(zf, cls)


if __name__ == '__main__':
    main()

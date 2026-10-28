#!/usr/bin/env python3
"""autopush-316 — scout 2: TheEndPortalBlockEntity.shouldRenderFace +
TheEndGatewayRenderer.render + EndGatewayRenderState fields (the beam law).

Exit 0 iff the walks parse.
"""
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'

CLASSES = [
    'net/minecraft/world/level/block/entity/TheEndPortalBlockEntity',
    'net/minecraft/client/renderer/blockentity/TheEndGatewayRenderer',
    'net/minecraft/client/renderer/blockentity/state/EndGatewayRenderState',
]


def main():
    zf = zipfile.ZipFile(JAR)
    for cls in CLASSES:
        print(f"\n########## {cls}")
        try:
            data = zf.read(cls + '.class')
        except KeyError:
            print('!! MISSING')
            continue
        cp, q = parse_cp(data)
        q += 6
        (nif,) = struct.unpack_from('>H', data, q) if (struct := __import__('struct')) else (0,)
        q += 2 + 2 * nif
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

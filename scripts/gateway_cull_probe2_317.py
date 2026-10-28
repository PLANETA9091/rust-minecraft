#!/usr/bin/env python3
"""autopush-317 probe 2 — the static Block.shouldRenderFace law +
the two renderers' consumption of the verdicts (the live walk).

  - Block.shouldRenderFace(BlockState, BlockState, Direction)Z — the
    3-arg static the gateway's own verdict invokes,
  - TheEndGatewayRenderer — how the render walk consumes shouldRenderFace,
  - TheEndPortalRenderer — the portal's own consumption.

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
    ('net/minecraft/world/level/block/Block', 'shouldRenderFace'),
    ('net/minecraft/client/renderer/blockentity/TheEndGatewayRenderer', None),
    ('net/minecraft/client/renderer/blockentity/TheEndPortalRenderer', None),
]


def dump_class(zf, cls, only=None):
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
        if only and mname != only:
            continue
        print(f"--- {mname} {mdesc}")
        if mcode is None:
            continue
        for ln in disasm(mcode, mcp):
            print(ln)


def main():
    zf = zipfile.ZipFile(JAR)
    for cls, only in CLASSES:
        dump_class(zf, cls, only)


if __name__ == '__main__':
    main()

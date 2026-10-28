#!/usr/bin/env python3
"""autopush-316 — THE GATEWAY'S OWN BEAM STAR LANE scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - EndGatewayBlock.animateTick the starfield's own particle rows
    (the PORTAL spawns the client lane plants),
  - TheEndGatewayRenderer.render the BEAM law (the hasActiveBeam /
    renderState walk the beam's own texture the vanilla draw wears),
  - TheEndGatewayBlockEntity the beam fields the state walk serves
    (the beamHeight / isCoolingDown fold the renderer reads),
  - EndGatewayRenderState the renderer's own state record fields.

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
    ('net/minecraft/world/level/block/EndGatewayBlock', None),
    ('net/minecraft/client/renderer/blockentity/TheEndGatewayRenderer', None),
    ('net/minecraft/client/renderer/blockentity/state/EndGatewayRenderState', None),
    ('net/minecraft/world/level/block/entity/TheEndGatewayBlockEntity', None),
]


def dump_assets(zf):
    print("=== the beam texture (the jar's own PNG + the mcmeta) ===")
    for path in [
        'assets/minecraft/textures/entity/end_portal/end_gateway_beam.png',
        'assets/minecraft/textures/entity/end_portal/end_gateway_beam.png.mcmeta',
    ]:
        try:
            d = zf.read(path)
            print(f"--- {path}: {len(d)} bytes")
        except KeyError:
            print(f"!! {path} MISSING")


def main():
    zf = zipfile.ZipFile(JAR)
    dump_assets(zf)

    for cls, want in CLASSES:
        print(f"\n########## {cls}")
        try:
            data = zf.read(cls + '.class')
        except KeyError:
            print('!! MISSING')
            continue
        cp, _ = parse_cp(data)
        methods = set()
        # first pass: list all method names
        p = data.find(b'\x00')  # cheap — instead walk properly
        # list methods via a full walk
        _, q = parse_cp(data)
        q += 6
        (nif,) = struct.unpack_from('>H', data, q); q += 2 + 2 * nif
        (nfields,) = struct.unpack_from('>H', data, q); q += 2
        for _ in range(nfields):
            fflags, fname, fdesc = struct.unpack_from('>HHH', data, q)
            q += 6
            name = utf8(cp, fname)
            print(f"  FIELD {name}")
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

#!/usr/bin/env python3
"""autopush-317 — THE GATEWAY'S OWN BEAM FACE CULL audit (the evidence walk).

The 316 round banked the gateway's own no-gate law (shouldRenderFace =
the axis-Y verdict alone) while the END_PORTAL mesher arm wears the
dev-29 transparent-neighbor gate. The audit disasms, live:
  - TheEndPortalBlockEntity.shouldRenderFace — the portal's own gate,
  - TheEndGatewayBlockEntity.shouldRenderFace + getParticleAmount —
    the gateway's own pair,
  - every static helper the verdicts invoke (the follow chain prints).

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
    ('net/minecraft/world/level/block/entity/TheEndPortalBlockEntity', None),
    ('net/minecraft/world/level/block/entity/TheEndGatewayBlockEntity', None),
]


def dump_class(zf, cls, grep=None):
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
        lines = list(disasm(mcode, mcp))
        if grep and not any(grep(mname, ln) for ln in lines):
            # still print short bodies; skip long non-matching ones
            if len(lines) > 4:
                continue
        for ln in lines:
            print(ln)


def main():
    zf = zipfile.ZipFile(JAR)
    for cls, _ in CLASSES:
        dump_class(zf, cls)


if __name__ == '__main__':
    main()

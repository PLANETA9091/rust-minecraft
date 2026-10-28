#!/usr/bin/env python3
"""autopush-316 — scout 5: the remaining laws (DyeColor MAGENTA/PURPLE
diffuse colors, the gateway fsh/vsh shaders, AbstractEndPortalRenderer
facesToShow, the getCooldownPercent constant).

Exit 0 iff the walks parse.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'


def dump_class(zf, cls, wants=None):
    print(f"\n########## {cls}")
    try:
        data = zf.read(cls + '.class')
    except KeyError:
        print('!! MISSING')
        return
    cp, q = parse_cp(data)
    q += 6
    (nif,) = struct.unpack_from('>H', data, q); q += 2 + 2 * nif
    (nfields,) = struct.unpack_from('>H', data, q); q += 2
    for _ in range(nfields):
        fflags, fname_i, fdesc_i = struct.unpack_from('>HHH', data, q)
        print(f"  FIELD {utf8(cp, fname_i)} {utf8(cp, fdesc_i)}")
        q += 6
        (na,) = struct.unpack_from('>H', data, q); q += 2
        for _ in range(na):
            q += 2
            (alen,) = struct.unpack_from('>I', data, q)
            q += 4 + alen
    (nmethods,) = struct.unpack_from('>H', data, q); q += 2
    for _ in range(nmethods):
        mflags, mname_i, mdesc_i = struct.unpack_from('>HHH', data, q)
        mname = utf8(cp, mname_i)
        q += 6
        (na,) = struct.unpack_from('>H', data, q); q += 2
        for _ in range(na):
            q += 2
            (alen,) = struct.unpack_from('>I', data, q)
            q += 4 + alen
        if wants is not None and mname not in wants:
            continue
        for mn, md, mcode, mcp in parse_class(data, {mname}):
            if mn != mname or md != mdesc_i:
                continue
            print(f"--- {mname} {md}")
            if mcode is None:
                continue
            for ln in disasm(mcode, mcp):
                print(ln)


def main():
    zf = zipfile.ZipFile(JAR)
    # shaders
    print("=== the gateway shader pair ===")
    for p in ['assets/minecraft/shaders/post/blend.json']:
        pass
    names = [n for n in zf.namelist() if 'end_gateway' in n and ('shader' in n)]
    for n in sorted(names):
        print(f"--- {n}")
        print(zf.read(n).decode('utf-8', 'replace'))

    # DyeColor clinit — the MAGENTA/PURPLE rows
    dump_class(zf, 'net/minecraft/world/item/DyeColor', wants={'<clinit>', '<init>', 'getTextureDiffuseColor'})

    # the parent renderer's facesToShow
    dump_class(zf, 'net/minecraft/client/renderer/blockentity/AbstractEndPortalRenderer')


if __name__ == '__main__':
    main()

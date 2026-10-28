#!/usr/bin/env python3
"""autopush-316 — scout 6: DyeColor clinit rows + AbstractEndPortalRenderer
extractRenderState + RenderTypes.endGateway the shader program speaks.

Exit 0 iff the walks parse.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'


def dump_all_methods(zf, cls, only=None):
    print(f"\n########## {cls}")
    try:
        data = zf.read(cls + '.class')
    except KeyError:
        print('!! MISSING')
        return
    for mname, mdesc, mcode, mcp in parse_class(data, only):
        if only is not None and mname not in only:
            continue
        print(f"--- {mname} {mdesc}")
        if mcode is None:
            continue
        for ln in disasm(mcode, mcp):
            print(ln)


def main():
    zf = zipfile.ZipFile(JAR)
    # shaders — the post/core layout
    print("=== shader files mentioning gateway/end_portal ===")
    for n in sorted(zf.namelist()):
        if n.endswith(('.fsh', '.vsh', '.json')) and ('gateway' in n.lower() or 'end_portal' in n.lower() or 'endportal' in n.lower()):
            print(f"--- {n}")
            try:
                print(zf.read(n).decode('utf-8', 'replace'))
            except Exception:
                pass

    # RenderTypes.endGateway — the program the render type binds
    dump_all_methods(zf, 'net/minecraft/client/renderer/rendertype/RenderTypes', only={'endGateway', 'endPortal', 'beaconBeam'})

    # DyeColor clinit (the MAGENTA/PURPLE ctor rows) + the getter
    dump_all_methods(zf, 'net/minecraft/world/item/DyeColor', only={'<clinit>', 'getTextureDiffuseColor'})

    # AbstractEndPortalRenderer — facesToShow's extract
    dump_all_methods(zf, 'net/minecraft/client/renderer/blockentity/AbstractEndPortalRenderer', only={'extractRenderState'})


if __name__ == '__main__':
    main()

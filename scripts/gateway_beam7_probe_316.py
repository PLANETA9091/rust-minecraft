#!/usr/bin/env python3
"""autopush-316 — scout 7: DyeColor clinit's MAGENTA + PURPLE rows the
int constants resolve (the textureDiffuseColor the beam color speaks).

Exit 0 iff the walks parse.
"""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm
from full_disasm import parse_cp, utf8

JAR = '.mc2rs/classpath/client.jar'


def main():
    zf = zipfile.ZipFile(JAR)
    data = zf.read('net/minecraft/world/item/DyeColor.class')
    cp, _ = parse_cp(data)

    for mname, mdesc, mcode, mcp in parse_class(data, {'<clinit>'}):
        lines = disasm(mcode, mcp)
        # walk the rows: a new DyeColor row starts at `new #1`; the row's
        # name tag = the first ldc_w string. Rows: WHITE..BLACK in order.
        names_in_order = []
        current = None
        ints = []
        for ln in lines:
            if 'new #1' in ln and 'DyeColor' in ln:
                if current is not None:
                    names_in_order.append((current, ints))
                current = None
                ints = []
            if 'ldc_w' in ln or 'ldc' in ln:
                # the constant idx
                tok = ln.split('#')[-1].split()[0]
                idx = int(tok)
                e = cp.get(idx)
                if isinstance(e, tuple) and e and e[0] == 'string':
                    current = e[1]
                elif isinstance(e, tuple) and e and e[0] == 4:
                    ints.append(('f', struct.unpack('>f', struct.pack('>I', e[1]))[0]))
                elif isinstance(e, tuple) and e and e[0] == 3:
                    ints.append(('i', e[1]))
                elif isinstance(e, tuple) and e and e[0] == 5:
                    ints.append(('l', e[1]))
        if current is not None:
            names_in_order.append((current, ints))

    for name, ints in names_in_order:
        if name in ('MAGENTA', 'PURPLE'):
            print(f"=== {name}: {ints}")


if __name__ == '__main__':
    main()

#!/usr/bin/env python3
"""autopush-323 — the client quickcraft walk's own methods (the paint
gates + the ghost recalc + the release arm)."""
import io
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from full_disasm import parse_cp, utf8, disasm

JAR = '.mc2rs/classpath/client.jar'
OUT = 'scripts/jar_work/beacon_qc_screen_323.txt'

TARGETS = [
    ('net/minecraft/client/gui/screens/inventory/AbstractContainerScreen',
     'recalculateQuickCraftRemaining'),
    ('net/minecraft/client/gui/screens/inventory/AbstractContainerScreen',
     'shouldAddSlotToQuickCraft'),
    ('net/minecraft/client/gui/screens/inventory/AbstractContainerScreen',
     'quickCraftToSlots'),
    ('net/minecraft/client/gui/screens/inventory/AbstractContainerScreen',
     'mouseDragged'),
]


def main():
    zf = zipfile.ZipFile(JAR)
    out = open(OUT, 'w')
    for cls, needle in TARGETS:
        data = zf.read(f'{cls}.class')
        cp, n = parse_cp(data)
        out.write(f"\n#### {cls} needle={needle!r}\n")
        i = n + 6
        nif = struct.unpack_from('>H', data, i)[0]
        i += 2 + 2 * nif
        nf = struct.unpack_from('>H', data, i)[0]
        i += 2
        for _ in range(nf):
            i += 6
            na = struct.unpack_from('>H', data, i)[0]
            i += 2
            for _ in range(na):
                i += 2
                al = struct.unpack_from('>I', data, i)[0]
                i += 4 + al
        nm = struct.unpack_from('>H', data, i)[0]
        i += 2
        for _ in range(nm):
            name = utf8(cp, struct.unpack_from('>H', data, i + 2)[0])
            desc = utf8(cp, struct.unpack_from('>H', data, i + 4)[0])
            i += 6
            na = struct.unpack_from('>H', data, i)[0]
            i += 2
            for _ in range(na):
                aname = utf8(cp, struct.unpack_from('>H', data, i)[0])
                al = struct.unpack_from('>I', data, i + 2)[0]
                if aname == 'Code' and needle in name:
                    body = data[i + 6:i + 6 + al]
                    max_stack, max_locals, clen = struct.unpack_from('>HHI', body, 0)
                    code = body[8:8 + clen]
                    out.write(f"\n=== {name}{desc} max_stack={max_stack} max_locals={max_locals} ===\n")
                    for start, mn, args in disasm(code, cp):
                        out.write(f"  {start:5d}: {mn} {args}\n")
                    out.write("\n")
                i += 6 + al
    out.close()
    print(f"banked {OUT}")
    return 0


if __name__ == '__main__':
    sys.exit(main())

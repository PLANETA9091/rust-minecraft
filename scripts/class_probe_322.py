#!/usr/bin/env python3
"""autopush-322 helper — list every method of a class + optional probe."""
import struct
import sys
import zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from full_disasm import parse_cp, utf8, disasm

JAR = '.mc2rs/classpath/client.jar'


def main():
    cls = sys.argv[1]
    probe = sys.argv[2] if len(sys.argv) > 2 else None
    zf = zipfile.ZipFile(JAR)
    data = zf.read(f'{cls}.class')
    cp, n = parse_cp(data)
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
            i += 4 + struct.unpack_from('>I', data, i)[0]
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
            alen = struct.unpack_from('>I', data, i + 2)[0]
            body = data[i + 6:i + 6 + alen]
            if probe and aname == 'Code' and probe in name:
                clen = struct.unpack_from('>I', body, 4)[0]
                print(f'=== {name}{desc} ===')
                for s, mn, ar in disasm(body[8:8 + clen], cp):
                    print(f'  {s:5d}: {mn} {ar}')
                print()
            i += 6 + alen
            if not probe:
                print(f'  method {name}{desc}')
    return 0


if __name__ == '__main__':
    sys.exit(main())

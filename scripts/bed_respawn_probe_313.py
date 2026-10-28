#!/usr/bin/env python3
"""autopush-313 probe — the floor walk's own branch targets, the
javap-style absolute offsets the raw operands mislead.

Decodes DismountHelper.isBlockFloorValid + CollisionGetter.getBlockFloorHeight
+ the dismount's supplier lambda with ABSOLUTE branch targets.

Exit 0 always (the probe banks).
"""
import sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class

JAR = '.mc2rs/classpath/client.jar'

def dump_raw(zf, path, wants):
    data = zf.read(path + '.class')
    for name, desc, code, cp in parse_class(data, set(wants)):
        if code is None:
            continue
        print(f"== {path.rsplit('/',1)[-1]} :: {name} {desc} ==")
        b = bytes(code)
        i = 0
        while i < len(b):
            op = b[i]
            print(f"  {i:4d}: op=0x{op:02x}", end='')
            if op in (0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0xa7):
                rel = int.from_bytes(b[i+1:i+3], 'big', signed=True)
                print(f" -> ABS {i + rel}")
                i += 3
            elif op in (0xc8,):
                rel = int.from_bytes(b[i+1:i+5], 'big', signed=True)
                print(f" -> ABS {i + rel}")
                i += 5
            elif op in (0xb8, 0xb6, 0xb7, 0xb2, 0xb4, 0xb5, 0xbb, 0xbd, 0xc0, 0xc1, 0xba, 0x13, 0x14, 0x19, 0xb9):
                # various 2-operand forms; print next 2 bytes
                print(f" operand={int.from_bytes(b[i+1:i+3], 'big')}")
                i += 3
            elif op == 0xc6 or op == 0xc7:
                rel = int.from_bytes(b[i+1:i+3], 'big', signed=True)
                print(f" -> ABS {i + rel}")
                i += 3
            else:
                # use the NAME table sizes: print the rest heuristically one byte
                print()
                i += 1

def main():
    zf = zipfile.ZipFile(JAR)
    dump_raw(zf, 'net/minecraft/world/level/BlockGetter', [
        'getBlockFloorHeight',
    ])
    dump_raw(zf, 'net/minecraft/world/level/CollisionGetter', [
        'getBlockFloorHeight',
    ])
    dump_raw(zf, 'net/minecraft/world/entity/vehicle/DismountHelper', [
        'getBlockFloorHeight', 'nonClimbableShape',
    ])
    return 0

if __name__ == '__main__':
    sys.exit(main())

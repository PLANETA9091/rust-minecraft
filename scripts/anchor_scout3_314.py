#!/usr/bin/env python3
"""autopush-314 scout3 — the charge lane + the anchor's own stand-up scan."""
import sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm

JAR = '.mc2rs/classpath/client.jar'

def dump(zf, path, wants):
    try:
        data = zf.read(path + '.class')
    except KeyError:
        print(f"!! {path} MISSING")
        return
    for name, desc, code, cp in parse_class(data, set(wants)):
        if code is None:
            continue
        print(f"== {path.rsplit('/',1)[-1]} :: {name} {desc} ==")
        for ln in disasm(code, cp):
            print(ln)

def main():
    zf = zipfile.ZipFile(JAR)
    print("=== the charge lane (useItemOn + fuel + charge walk) ===")
    dump(zf, 'net/minecraft/world/level/block/RespawnAnchorBlock', [
        'useItemOn', 'isRespawnFuel', 'canBeCharged', 'charge',
    ])
    print("=== the anchor's own stand-up scan (the two overloads) ===")
    dump(zf, 'net/minecraft/world/level/block/RespawnAnchorBlock', [
        'findStandUpPosition', 'isWaterThatWouldFlow',
        'lambda$explode$0', 'getScaledChargeLevel',
    ])

if __name__ == '__main__':
    main()

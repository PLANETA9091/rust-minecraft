#!/usr/bin/env python3
"""autopush-305 — THE HOPPER'S OWN FIVE-SLOT BANK scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - HopperBlockEntity the container size (the withSize(5) walk),
  - HopperBlock.useItemOn the menu open walk (the HopperMenu provider),
  - HopperMenu the 5-slot grid the layout speaks.

Exit 0 iff the walks parse.
"""
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
        print()

def main():
    zf = zipfile.ZipFile(JAR)
    names = zf.namelist()
    hits = [n for n in names if 'Hopper' in n and n.endswith('.class') and 'renderer' not in n and 'screen' not in n.lower()]
    print("== Hopper classes ==")
    for n in sorted(hits)[:20]:
        print(" ", n)
    dump(zf, 'net/minecraft/world/level/block/entity/HopperBlockEntity',
         {'<init>', 'createContainer', 'getContainerSize', 'setItems', 'loadAdditional', 'saveAdditional'})
    dump(zf, 'net/minecraft/world/level/block/HopperBlock',
         {'useItemOn', 'menuProvider'})
    dump(zf, 'net/minecraft/world/inventory/HopperMenu', {'<init>'})
    return 0

if __name__ == '__main__':
    sys.exit(main())

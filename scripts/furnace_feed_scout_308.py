#!/usr/bin/env python3
"""autopush-308 — THE HOPPER'S OWN FURNACE FEED scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - AbstractFurnaceBlockEntity the face tables (SLOTS_FOR_UP/DOWN/SIDES
    the <clinit> rows answer),
  - the canPlaceItem walk (the slot verdicts: 2 refuses, 1 the fuel
    gate, 0 accepts),
  - canPlaceItemThroughFace / canTakeItemThroughFace (the hopper push
    and suck gates the furnace's own answers bank),
  - FuelValues.isFuel (the fuel verdict the table answers).

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

def main():
    zf = zipfile.ZipFile(JAR)
    print("=== AbstractFurnaceBlockEntity: the face tables + the gates ===")
    dump(zf, 'net/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity', [
        'getSlotsForFace', 'canPlaceItem', 'canPlaceItemThroughFace',
        'canTakeItemThroughFace', '<clinit>',
    ])
    print("\n=== FuelValues: the fuel verdict ===")
    dump(zf, 'net/minecraft/world/item/crafting/FuelValues', [
        'isFuel', 'burnDuration',
    ])
    return 0

if __name__ == '__main__':
    sys.exit(main())

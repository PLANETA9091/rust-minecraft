#!/usr/bin/env python3
"""autopush-309 — THE HOPPER'S OWN BREWING STAND FEED scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - BrewingStandBlockEntity the face tables (SLOTS_FOR_UP/DOWN/SIDES
    the <clinit> rows answer),
  - the canPlaceItem walk (the slot verdicts: the bottles the ingredient
    gate the fuel slot the blaze powder row answers),
  - canPlaceItemThroughFace / canTakeItemThroughFace (the hopper push
    and suck gates the stand's own answers bank),
  - the slot layout (the ctor the NonNullList the 5 slots wears).

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
    print("=== BrewingStandBlockEntity: the ctor + the face tables + the gates ===")
    dump(zf, 'net/minecraft/world/level/block/entity/BrewingStandBlockEntity', [
        'getSlotsForFace', 'canPlaceItem', 'canPlaceItemThroughFace',
        'canTakeItemThroughFace', '<clinit>', '<init>',
        'getContainerSize', 'isFuel', 'isIngredient',
    ])
    print("\n=== PotionBrewing: the ingredient + the fuel verdicts ===")
    dump(zf, 'net/minecraft/world/item/alchemy/PotionBrewing', [
        'isIngredient', 'isFuel', 'hasMix',
    ])
    return 0

if __name__ == '__main__':
    sys.exit(main())

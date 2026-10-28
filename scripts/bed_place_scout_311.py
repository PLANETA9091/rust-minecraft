#!/usr/bin/env python3
"""autopush-311 — THE BED'S OWN PLACE WALK scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - BedItem the placement walk (the getPlacementContext gate the head
    cell the canBeReplaced verdict answers, the placeBlock the two
    setBlock calls the head the PART=HEAD value banks),
  - BedBlock the state walk (the getStateForPlacement the FACING the
    raw look answers, the updateShape the mate-gone the AIR return
    keeps, the canSurvive the below-block verdict wears),
  - the facing's own quadrant (the getHorizontalDirection the RAW
    look — the head the look's own far side lands).

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
    print("=== BedItem: the placement walk ===")
    dump(zf, 'net/minecraft/world/item/BedItem', [
        'getPlacementContext', 'placeBlock', '<init>', 'appendHoverText',
    ])
    print("\n=== BedBlock: the state walk ===")
    dump(zf, 'net/minecraft/world/level/block/BedBlock', [
        'getStateForPlacement', 'updateShape', 'canSurvive', '<clinit>',
        'getConnectedDirection', 'getNeighbourDirection',
    ])
    return 0

if __name__ == '__main__':
    sys.exit(main())

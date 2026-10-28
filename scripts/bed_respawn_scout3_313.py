#!/usr/bin/env python3
"""autopush-313 scout3 — the offset tables + the dismount law.

Dumps the 26.2 bytecode for:
  - BedBlock.bedSurroundStandUpOffsets (the surround rows),
  - BedBlock.bedAboveStandUpOffsets (the above rows),
  - DismountHelper.findSafeDismountLocation (the per-cell verdict),
  - BedRule.canSetSpawn (the bed-rule gate),
  - LevelData.RespawnData (the record shape).

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
    print("=== BedBlock: the offset tables ===")
    dump(zf, 'net/minecraft/world/level/block/BedBlock', [
        'bedSurroundStandUpOffsets', 'bedAboveStandUpOffsets',
    ])
    print("=== DismountHelper: the safe-dismount verdict ===")
    dump(zf, 'net/minecraft/world/entity/vehicle/DismountHelper', [
        'findSafeDismountLocation', 'canDismountInBox',
    ])
    print("=== BedRule: the canSetSpawn gate ===")
    dump(zf, 'net/minecraft/world/attribute/BedRule', [
        'canSetSpawn', 'canSleep', '<init>', '<clinit>',
    ])
    print("=== LevelData.RespawnData: the record shape ===")
    dump(zf, 'net/minecraft/world/level/storage/LevelData$RespawnData', [
        '<init>', 'globalPos', 'pos', 'yaw', 'pitch', 'dimension',
    ])
    return 0

if __name__ == '__main__':
    sys.exit(main())

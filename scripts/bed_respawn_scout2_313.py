#!/usr/bin/env python3
"""autopush-313 scout2 — the respawn walk's own deep folds.

Dumps the 26.2 bytecode for:
  - ServerPlayer.findRespawnAndUseSpawnBlock (the static core),
  - ServerPlayer.RespawnConfig (the record the getDimensionOrDefault
    the isSamePosition the isSameDimension answers),
  - BedBlock.bedStandUpOffsets (the offset table the [[I rows speak),
  - BedBlock.findStandUpPositionAtOffset (the two-pass arms the strict
    the loose verdicts wear),
  - BedBlock.isBunkBed + findBunkBedStandUpPosition (the bunk lane),
  - Direction.isFacingAngle (the yaw quadrant the side pick serves),
  - TeleportTransition.missingRespawnBlock + createDefault (the
    fallback transitions the message arm speaks).

Exit 0 iff the walks parse.
"""
import sys, zipfile, json

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
    print("=== ServerPlayer: the static core walk ===")
    dump(zf, 'net/minecraft/server/level/ServerPlayer', [
        'findRespawnAndUseSpawnBlock',
    ])
    print("=== ServerPlayer.RespawnConfig: the record ===")
    dump(zf, 'net/minecraft/server/level/ServerPlayer$RespawnConfig', [
        '<init>', 'getDimensionOrDefault', 'isSamePosition', 'isSameDimension',
        'position', 'dimension', 'angle',
    ])
    print("=== BedBlock: the offset table + the at-offset scan ===")
    dump(zf, 'net/minecraft/world/level/block/BedBlock', [
        'bedStandUpOffsets', 'findStandUpPositionAtOffset', 'isBunkBed',
        'findBunkBedStandUpPosition', 'canSetSpawn',
    ])
    print("=== Direction: the yaw quadrant ===")
    dump(zf, 'net/minecraft/core/Direction', [
        'isFacingAngle', 'getClockWise',
    ])
    print("=== TeleportTransition: the fallback arms ===")
    dump(zf, 'net/minecraft/world/level/portal/TeleportTransition', [
        'missingRespawnBlock', 'createDefault',
    ])
    print("=== the respawn words (the obstructions the messages) ===")
    data = json.loads(zf.read('assets/minecraft/lang/en_us.json'))
    for k in sorted(data):
        if 'block.minecraft.set_spawn' in k or 'spawn.not_valid' in k:
            print(f"{k} = {data[k]}")
    return 0

if __name__ == '__main__':
    sys.exit(main())

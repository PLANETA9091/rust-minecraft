#!/usr/bin/env python3
"""autopush-313 — THE BED'S OWN RESPAWN RETURN WALK scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - ServerPlayer.findRespawnPositionAndUseSpawnBlock the STATIC walk
    (the anchor arm the bed arm the else arm the RespawnPosAngle the
    map verdicts answer),
  - ServerPlayer the respawn readers (getRespawnPosition getRespawnAngle
    isRespawnForced the banked spot the fields serve),
  - BedBlock.findStandUpPosition the TWO-PASS law (the strict pass the
    collision pair the refusal keeps, the loose pass the scan answers),
  - CollisionGetter.findFreePosition the spiral scan (the offset pair
    the plain pos the verdict answers),
  - BedBlock.canSetSpawn the still-there verdict,
  - the language rows (selectWorld respawn the bed-missing words speak).

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
    print("=== ServerPlayer: the static respawn walk ===")
    dump(zf, 'net/minecraft/server/level/ServerPlayer', [
        'findRespawnPositionAndUseSpawnBlock', 'findRespawnPosition',
    ])
    print("=== ServerPlayer: the respawn readers ===")
    dump(zf, 'net/minecraft/server/level/ServerPlayer', [
        'getRespawnPosition', 'getRespawnAngle', 'isRespawnForced',
        'getRespawnDimension', 'setRespawnPosition', 'respawn',
    ])
    print("=== BedBlock: the stand-up scan ===")
    dump(zf, 'net/minecraft/world/level/block/BedBlock', [
        'findStandUpPosition', 'canSetSpawn', '<clinit>',
    ])
    print("=== CollisionGetter: the free-position spiral ===")
    dump(zf, 'net/minecraft/world/phys/shapes/CollisionGetter', [
        'findFreePosition', 'collides',
    ])
    print("=== ServerLevel: the shared spawn ===")
    dump(zf, 'net/minecraft/server/level/ServerLevel', [
        'getSharedSpawnPos', 'getSharedSpawnAngle', 'getLevelSharedSpawnPos',
    ])
    print("=== RespawnPosAngle: the record ===")
    dump(zf, 'net/minecraft/server/level/ServerPlayer$RespawnPosAngle', [
        '<init>', 'values',
    ])
    print("=== the language rows (the respawn words) ===")
    for lang in ('assets/minecraft/lang/en_us.json',):
        data = json.loads(zf.read(lang))
        for k in sorted(data):
            if 'spawn' in k and ('block' in k or 'obstruct' in k or 'missing' in k):
                print(f"{k} = {data[k]}")
        for k in sorted(data):
            if k in ('death.screen.title', 'death.respawn', 'menu.returnToGame'):
                print(f"{k} = {data[k]}")
    return 0

if __name__ == '__main__':
    sys.exit(main())

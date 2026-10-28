#!/usr/bin/env python3
"""autopush-312 scout round 3 — the night-skip fold + the reach bounds.

  - BedRule$Rule (WHEN_DARK/ALWAYS/NEVER the tests speak),
  - Avatar the full method walk (startSleeping the pose the pos
    stamps — the class the names bank),
  - ServerPlayer.isReachableBedBlock / freeAt (the TOO_FAR_AWAY the
    OBSTRUCTED bounds),
  - ServerLevel the 24000 fold (grep EVERY method's disasm for the
    constant — the enclosing method the fold banks),
  - ServerLevel.announceSleepStatus (the sleep chat the percentage
    speaks),
  - Level.isDay (the WHEN_DARK the verdict serves).
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
    print("=== BedRule$Rule ===")
    dump(zf, 'net/minecraft/world/attribute/BedRule$Rule', [
        'test', '<init>', '<clinit>', 'lambda$',
    ])
    print("=== Avatar: ALL methods (the names the pose banks) ===")
    data = zf.read('net/minecraft/world/entity/Avatar.class')
    for name, desc, code, cp in parse_class(data, set()):
        print(f"-- method: {name} {desc}")
    print("=== LivingEntity: startSleeping / stopSleeping ===")
    dump(zf, 'net/minecraft/world/entity/LivingEntity', [
        'startSleeping', 'stopSleeping',
    ])
    print("=== ServerPlayer: isReachableBedBlock / freeAt ===")
    dump(zf, 'net/minecraft/server/level/ServerPlayer', [
        'isReachableBedBlock', 'freeAt',
    ])
    print("=== ServerLevel: the 24000 fold (every method, grep) ===")
    data = zf.read('net/minecraft/server/level/ServerLevel.class')
    for name, desc, code, cp in parse_class(data, set()):
        if code is None:
            continue
        lines = list(disasm(code, cp))
        if any('24000' in ln for ln in lines):
            print(f"-- method with 24000: {name} {desc}")
            for ln in lines:
                print(ln)
    print("=== ServerLevel: announceSleepStatus ===")
    dump(zf, 'net/minecraft/server/level/ServerLevel', [
        'announceSleepStatus',
    ])
    print("=== Level: isDay / getSkyDarken ===")
    dump(zf, 'net/minecraft/world/level/Level', [
        'isDay', 'getSkyDarken',
    ])
    return 0

if __name__ == '__main__':
    sys.exit(main())

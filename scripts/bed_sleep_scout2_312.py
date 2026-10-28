#!/usr/bin/env python3
"""autopush-312 scout round 2 — the sleep walk's own tails.

  - net/minecraft/server/players/SleepStatus (the all-asleep verdict),
  - ServerLevel.updateSleepingPlayerList + the setDayTime fold
    (grep the whole ServerLevel disasm for the 24000 constant),
  - net/minecraft/world/attribute/BedRule (canSleep/canSetSpawn/
    asProblem — the 26.2 day/night verdict the environment attribute
    speaks),
  - Avatar.startSleeping/stopSleeping + isSleeping (the sleep pose
    mechanics the client's own body wears),
  - ServerPlayer.bedInRange/bedBlocked (the TOO_FAR_AWAY the
    OBSTRUCTED bounds),
  - the lang rows (sleep.not_possible, the spawn-set message,
    bed.obstructed).
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
    print("=== SleepStatus (server/players) ===")
    dump(zf, 'net/minecraft/server/players/SleepStatus', [
        'areAllSleeping', 'areAllFullyAsleep', 'update', 'tick',
        'removeAllSleepers', 'amountSleeping',
    ])
    print("=== ServerLevel: updateSleepingPlayerList + the fold ===")
    dump(zf, 'net/minecraft/server/level/ServerLevel', [
        'updateSleepingPlayerList', 'setDayTime', 'resetWeatherCycle',
    ])
    print("=== BedRule ===")
    dump(zf, 'net/minecraft/world/attribute/BedRule', [
        'canSleep', 'canSetSpawn', 'asProblem', 'explodes', 'errorMessage',
        '<init>', '<clinit>', 'lambda$',
    ])
    print("=== Avatar: the sleep pose walk ===")
    dump(zf, 'net/minecraft/world/entity/Avatar', [
        'startSleeping', 'stopSleeping', 'isSleeping', 'getSleepingPos',
        'setSleepingPos', 'clearSleepingPos', 'setPose', 'sleepCounter',
    ])
    print("=== ServerPlayer: bedInRange / bedBlocked ===")
    dump(zf, 'net/minecraft/server/level/ServerPlayer', [
        'bedInRange', 'bedBlocked',
    ])
    print("=== the language rows ===")
    data = json.loads(zf.read('assets/minecraft/lang/en_us.json'))
    for k in sorted(data):
        if k in ('sleep.not_possible', 'block.minecraft.bed.obstructed',
                 'block.minecraft.bed.no_sleep') or 'spawn.set' in k or 'selectWorld' in k.lower():
            print(f"{k} = {data[k]}")
    return 0

if __name__ == '__main__':
    sys.exit(main())

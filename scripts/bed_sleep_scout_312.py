#!/usr/bin/env python3
"""autopush-312 — THE BED'S OWN SLEEP LANE scout (the evidence walk).

Re-derives from the 26.2 jar, live:
  - BedBlock the use walk (useWithoutItem: the PART != HEAD arm the
    relative(FACING) re-seat answers, the bedWorks gate, the occupied
    arm, the startSleepInBed dispatch the displayClientMessage arm),
  - Player the sleep walk (startSleepInBed: the isDay the
    NOT_POSSIBLE_NOW verdict, the TOO_FAR_AWAY the 3/2/3 bounds
    answer, the NOT_SAFE the monsters arm, the setSleepingPos +
    setPose(SLEEPING) + setPos + ZERO delta the sleep stamps),
  - BedSleepingProblem the message table (getMessage the resource
    keys the en_us.json rows answer),
  - ServerPlayer the respawn arm (startSleepInBed the
    setRespawnPosition call the spawn point banks),
  - ServerLevel the night-skip walk (wakeUpAllPlayers / setDayTime
    the +24000 - dayTime % 24000 the morning fold answers,
    SleepStatus the areAllSleeping verdict serves),
  - the language rows (block.minecraft.bed.* the en_us.json the
    message words speak).

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
    print("=== BedBlock: the use walk ===")
    dump(zf, 'net/minecraft/world/level/block/BedBlock', [
        'useWithoutItem', 'use', 'canSetSpawn', 'occupied',
        'explodeBed', 'updateShape', '<clinit>',
    ])
    print("=== Player: the sleep walk ===")
    dump(zf, 'net/minecraft/world/entity/player/Player', [
        'startSleepInBed', 'stopSleepInBed', 'isSleeping', 'getSleepingPos',
        'setSleepingPos', 'clearSleepingPos', 'isSleepingLongEnough',
        'sleepCounter', '<clinit>',
    ])
    print("=== Player.BedSleepingProblem: the message table ===")
    dump(zf, 'net/minecraft/world/entity/player/Player$BedSleepingProblem', [
        'getMessage', 'values', '<clinit>',
    ])
    print("=== ServerPlayer: the respawn arm ===")
    dump(zf, 'net/minecraft/server/level/ServerPlayer', [
        'startSleepInBed', 'setRespawnPosition', 'stopSleepInBed',
    ])
    print("=== ServerLevel: the night-skip walk ===")
    dump(zf, 'net/minecraft/server/level/ServerLevel', [
        'wakeUpAllPlayers', 'setDayTime', 'tickTime', 'allPlayersSleeping',
    ])
    print("=== SleepStatus: the all-asleep verdict ===")
    dump(zf, 'net/minecraft/server/level/SleepStatus', [
        'areAllSleeping', 'areAllFullyAsleep', 'update', 'tick',
    ])
    print("=== ClientLevel / Level: isDay the verdict ===")
    dump(zf, 'net/minecraft/world/level/Level', [
        'isDay', 'isNight', 'getDayTime',
    ])
    print("=== the language rows ===")
    for lang in ('assets/minecraft/lang/en_us.json',):
        data = json.loads(zf.read(lang))
        for k in sorted(data):
            if 'bed' in k and ('sleep' in k or 'no_sleep' in k or 'not_safe' in k
                               or 'too_far' in k or 'occluded' in k or 'occupied' in k):
                print(f"{k} = {data[k]}")
    return 0

if __name__ == '__main__':
    sys.exit(main())

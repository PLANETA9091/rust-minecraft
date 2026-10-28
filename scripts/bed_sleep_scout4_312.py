#!/usr/bin/env python3
"""autopush-312 scout round 4 — the night-skip fold's own home.

Grep EVERY class in the server jar (client.jar merges) for the
24000 constant in disasm — the enclosing method the fold banks.
Also: Level.isDarkOutside, SleepStatus.areEnoughSleeping/sleepersNeeded,
the skip walk's weather arm (resetWeatherCycle callers).
"""
import sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm

JAR = '.mc2rs/classpath/client.jar'

WANT_CLASSES = [
    'net/minecraft/server/MinecraftServer',
    'net/minecraft/server/players/PlayerList',
    'net/minecraft/server/players/SleepStatus',
    'net/minecraft/world/level/Level',
]

def main():
    zf = zipfile.ZipFile(JAR)
    print("=== the 24000 fold (the classes the skip banks) ===")
    for path in WANT_CLASSES:
        try:
            data = zf.read(path + '.class')
        except KeyError:
            print(f"!! {path} MISSING")
            continue
        for name, desc, code, cp in parse_class(data, set()):
            if code is None:
                continue
            lines = list(disasm(code, cp))
            if any('24000' in ln for ln in lines):
                print(f"-- {path.rsplit('/',1)[-1]} :: {name} {desc}")
                for ln in lines:
                    print(ln)
    print("=== Level.isDarkOutside / isDay ===")
    data = zf.read('net/minecraft/world/level/Level.class')
    for name, desc, code, cp in parse_class(data, {'isDarkOutside', 'isDay'}):
        if code is None:
            continue
        print(f"== Level :: {name} {desc} ==")
        for ln in disasm(code, cp):
            print(ln)
    print("=== SleepStatus: areEnoughSleeping / sleepersNeeded ===")
    data = zf.read('net/minecraft/server/players/SleepStatus.class')
    for name, desc, code, cp in parse_class(data, {'areEnoughSleeping', 'sleepersNeeded'}):
        if code is None:
            continue
        print(f"== SleepStatus :: {name} {desc} ==")
        for ln in disasm(code, cp):
            print(ln)
    return 0

if __name__ == '__main__':
    sys.exit(main())

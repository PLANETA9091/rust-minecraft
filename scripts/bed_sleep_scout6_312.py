#!/usr/bin/env python3
"""autopush-312 scout round 6 — the client's own wake keys.

LocalPlayer the isSleeping reads the client-side wears: the
wake-on-input law, the sleep packet walk. Dump every LocalPlayer
method whose disasm mentions isSleeping / stopSleep / sleepCounter.
(parse_class(want=None) = all methods — the helper's new door.)
"""
import sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm

JAR = '.mc2rs/classpath/client.jar'

def main():
    zf = zipfile.ZipFile(JAR)
    data = zf.read('net/minecraft/client/player/LocalPlayer.class')
    for name, desc, code, cp in parse_class(data, None):
        if code is None:
            continue
        lines = list(disasm(code, cp))
        joined = '\n'.join(lines)
        if 'isSleeping' in joined or 'stopSleep' in joined:
            print(f"== LocalPlayer :: {name} {desc} ==")
            for ln in lines:
                print(ln)
    return 0

if __name__ == '__main__':
    sys.exit(main())

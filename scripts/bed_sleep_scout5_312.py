#!/usr/bin/env python3
"""autopush-312 scout round 5 — the night-skip fold's own home (26.x clock).

parse_class(want=empty) yields NOTHING (name-filter), so this round:
  - ServerLevel the ALL-method walk (grep for wakeUpAllPlayers /
    setDayTime / ClockManager / sleepStatus the callers bank),
  - the world/clock family (ServerClockManager the skip's own write
    the clock speaks, WorldClocks the overworld clock's own table,
    ClockState the frozen/marker laws),
  - Level.isBrightOutside (the WHEN_DARK the isDarkOutside serves).
"""
import sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm

JAR = '.mc2rs/classpath/client.jar'

def all_methods(zf, path):
    data = zf.read(path + '.class')
    # first pass: harvest every method name
    names = set()
    for name, desc, code, cp in parse_class(data, set()):
        pass  # yields nothing; harvest below via second trick
    # parse_class filters by name; instead re-parse with a broad want:
    # brute-force: yield all methods by intercepting the filter —
    # simplest: call parse_class with want=ALL_NAMES via raw scan.
    return data

def dump_all(zf, path, grep_terms):
    """dump every method whose disasm mentions any grep term"""
    data = zf.read(path + '.class')
    # harvest names with a permissive pass: monkey wants = all utf8 strings
    # cheap trick: parse twice — collect candidate names from cp
    names = harvest_names(data)
    for name, desc, code, cp in parse_class(data, names):
        if code is None:
            continue
        lines = list(disasm(code, cp))
        joined = '\n'.join(lines)
        if any(t in joined for t in grep_terms):
            print(f"== {path.rsplit('/',1)[-1]} :: {name} {desc} ==")
            for ln in lines:
                print(ln)

def harvest_names(data):
    """collect every method name (heuristic: utf8 strings that look like methods)"""
    out = set()
    # reuse parse_class's cp parse by scanning with want=set() won't work;
    # do a second approach: names appear as Utf8 cp entries — just take ALL
    # utf8 strings (safe: parse_class yields only real methods).
    # Instead: parse the class file manually for method names.
    import struct
    class R:
        def __init__(s, b): s.b, s.i = b, 0
        def u1(s): v = s.b[s.i]; s.i += 1; return v
        def u2(s): v = struct.unpack_from('>H', s.b, s.i)[0]; s.i += 2; return v
        def u4(s): v = struct.unpack_from('>I', s.b, s.i)[0]; s.i += 4; return v
        def raw(s, n): v = s.b[s.i:s.i+n]; s.i += n; return v
    r = R(data)
    r.raw(4); r.raw(4)
    n = r.u2(); i = 1
    utf8s = [None] * n
    while i < n:
        tag = r.u1()
        if tag == 1:
            ln = r.u2(); utf8s[i] = r.raw(ln).decode('utf-8', 'replace')
        elif tag == 3: r.raw(4)
        elif tag == 4: r.raw(4)
        elif tag == 5: r.raw(8); i += 1
        elif tag == 6: r.raw(8); i += 1
        elif tag in (7, 8, 16, 19, 20): r.u2()
        elif tag in (9, 10, 11, 12, 17, 18): r.raw(4)
        elif tag == 15: r.u1(); r.u2()
        else: raise ValueError(f'tag {tag}')
        i += 1
    # stop after the cp walk — ALL utf8 strings become the `want` set
    return set(s for s in utf8s if s and not s.startswith('(') and '/' not in s and ';' not in s and '[' not in s)

def main():
    zf = zipfile.ZipFile(JAR)
    print("=== ServerLevel: the sleep/clock callers ===")
    dump_all(zf, 'net/minecraft/server/level/ServerLevel', [
        'wakeUpAllPlayers', 'setDayTime', 'ClockManager', 'sleepStatus',
        'announceSleepStatus', 'resetWeatherCycle', 'allPlayersSleeping',
        'updateSleepingPlayerList',
    ])
    print("=== ServerClockManager: the skip write ===")
    dump_all(zf, 'net/minecraft/world/clock/ServerClockManager', [
        'setDayTime', 'skip', 'advance', '24000',
    ])
    print("=== WorldClocks: the overworld clock's table ===")
    dump_all(zf, 'net/minecraft/world/clock/WorldClocks', [
        'overworld', '24000', 'dayTime',
    ])
    print("=== Level.isBrightOutside ===")
    dump_all(zf, 'net/minecraft/world/level/Level', [
        'isBrightOutside',
    ])
    return 0

if __name__ == '__main__':
    sys.exit(main())

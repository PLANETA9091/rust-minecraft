#!/usr/bin/env python3
"""autopush-313 probe3 — the constant #129 the floor walk returns.

Reads BlockGetter.class's constant pool, prints the double the
ldc2_w #129 row loads (the getBlockFloorHeight empty-arm's own
no-floor verdict).

Exit 0 always.
"""
import struct, sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])

JAR = '.mc2rs/classpath/client.jar'

def main():
    zf = zipfile.ZipFile(JAR)
    data = zf.read('net/minecraft/world/level/BlockGetter.class')
    # constant pool count at offset 8 (magic u4 + minor u2 + major u2)
    n = int.from_bytes(data[8:10], 'big')
    i = 10
    pool = {}
    idx = 1
    while idx < n:
        tag = data[i]
        if tag == 1:  # utf8
            ln = int.from_bytes(data[i+1:i+3], 'big')
            pool[idx] = ('utf8', data[i+3:i+3+ln].decode('utf-8', 'replace'))
            i += 3 + ln
        elif tag in (7, 8, 16, 19, 20):  # class/str/method-type/module/pkg ref
            pool[idx] = ('ref', int.from_bytes(data[i+1:i+3], 'big'))
            i += 3
        elif tag in (3, 4):  # int/float
            pool[idx] = ('num', data[i+1:i+5])
            i += 5
        elif tag in (5, 6):  # long/double (takes 2 slots)
            pool[idx] = ('num8', data[i+1:i+9])
            idx += 1
            i += 9
        elif tag in (9, 10, 11, 12, 17, 18):  # field/method/iface/name+type
            pool[idx] = ('member', int.from_bytes(data[i+1:i+3], 'big'),
                         int.from_bytes(data[i+3:i+5], 'big'))
            i += 5
        else:
            print(f"unknown tag {tag} at cp {idx}")
            break
        idx += 1
    # resolve entry 129 (and whatever it points at)
    for want in (129, 111, 117):
        e = pool.get(want)
        print(f"cp#{want} = {e}")
        if e and e[0] in ('ref', 'member'):
            for sub in e[1:]:
                print(f"   -> cp#{sub} = {pool.get(sub)}")
    # the double at #129 if numeric
    e = pool.get(129)
    if e and e[0] == 'num8':
        print(f"cp#129 double = {struct.unpack('>d', e[1])[0]}")
    return 0

if __name__ == '__main__':
    sys.exit(main())

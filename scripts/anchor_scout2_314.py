#!/usr/bin/env python3
"""autopush-314 scout2 — the RespawnAnchorBlock method census + the
constant-pool floats + the map lambdas + the charge walk."""
import sys, zipfile

sys.path.insert(0, __file__.rsplit('/', 1)[0])
from raw_disasm import parse_class, disasm

JAR = '.mc2rs/classpath/client.jar'

zf = zipfile.ZipFile(JAR)
data = zf.read('net/minecraft/world/level/block/RespawnAnchorBlock.class')

# 1. the method census
methods, fields, cp = None, None, None
for name, desc, code, cp in parse_class(data, set()):
    print(f"METHOD {name} {desc} code={code is not None}")

# 2. the raw constant pool floats (the explosion radius the pool keeps)
print("=== constant pool (floats + utf8 method refs) ===")
import struct
# quick cp walk: reuse parse_class internals via raw read
idx = 10  # after magic+version
count = struct.unpack('>H', data[8:10])[0]
i = 1
while i < count:
    tag = data[idx]
    if tag == 1:  # utf8
        ln = struct.unpack('>H', data[idx+1:idx+3])[0]
        s = data[idx+3:idx+3+ln]
        try:
            t = s.decode()
        except UnicodeDecodeError:
            t = repr(s)
        if 'explode' in t or 'charge' in t.lower() or 'lambda' in t:
            print(f"  #{i} Utf8 {t[:110]}")
        idx += 3 + ln
    elif tag in (3, 4):  # int/float
        val = struct.unpack('>f', data[idx+1:idx+5])[0]
        print(f"  #{i} {'Float' if tag==4 else 'Int'} {val}")
        idx += 5
    elif tag in (5, 6):  # long/double
        val = struct.unpack('>d', data[idx+1:idx+9])[0]
        print(f"  #{i} {'Double' if tag==6 else 'Long'} {val}")
        idx += 9
        i += 1
    elif tag in (7, 8, 16, 19, 20):
        idx += 3
    elif tag in (9, 10, 11, 12, 17, 18):
        idx += 5
    elif tag == 15:
        idx += 4
    else:
        print(f"  tag {tag} at #{i}")
        break
    i += 1

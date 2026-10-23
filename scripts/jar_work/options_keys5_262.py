#!/usr/bin/env python3
"""THE OPTIONS KEYS PROBE (autopush-168 round 5): tag-8 String entries the
walk_code miss — the REAL key strings the processOptions lane passes to
FieldAccess.process, plus the IntRange codec's full bytecode.
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

def body_lines(cp, body):
    clen = struct.unpack_from(">I", body, 4)[0]
    return [str(t) for t in walk_code(cp, body[8:8 + clen])]

def ldc_string(cp, tok):
    """tok like '292: ldc_w ldcw@1866' -> the string it loads."""
    tail = tok.split("ldcw@")[-1].split()[0]
    idx = int(tail)
    e = cp[idx]
    if e[0] == 8:
        return utf(cp, e[1])
    if e[0] == "u":
        return e[1]
    return None

ms = methods_of(z, "net.minecraft.client.Options")
print("=== processOptions: process(key, instance) call sites — the REAL keys")
for s, (cp, body) in ms.items():
    if "processOptions" in s:
        lines = body_lines(cp, body)
        cur_key = None
        cur_field = None
        for i, t in enumerate(lines):
            if "ldcw@" in t:
                k = ldc_string(cp, t)
                if k:
                    cur_key = k
            if "getfield" in t:
                cur_field = t.split("getfield")[-1].split(":")[0].split(".")[-1]
            if ".process:" in t:
                print(f"  pc{i:5d}  {cur_field or '?':30s}  key='{cur_key}'")
        break

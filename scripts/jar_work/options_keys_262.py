#!/usr/bin/env python3
"""THE OPTIONS KEYS PROBE (autopush-168 round 2): the processOptions visitor
lane — the per-option visit(key, instance) calls the real option keys wear,
the Options$3.visit the key:value line the file keeps, the fov/volume codec
values the OptionInstance classes encode. The jar's own facts — no memory.
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")
ms = methods_of(z, "net.minecraft.client.Options")

def body_lines(cp, body):
    clen = struct.unpack_from(">I", body, 4)[0]
    return [str(t) for t in walk_code(cp, body[8:8 + clen])]

# 1) processOptions: the visit calls carry the REAL keys.
print("=== processOptions: field -> visit key")
for s, (cp, body) in ms.items():
    if "processOptions" in s:
        lines = body_lines(cp, body)
        for i, t in enumerate(lines):
            if "visit" in t and "invokevirtual" in t or "visit" in t and "invokeinterface" in t:
                window = " ~ ".join(lines[max(0, i - 14):i + 1])
                getfields = [w.split("getfield")[1].split(":")[0].split(".")[-1]
                             for w in window.split(" ~ ") if "getfield" in w and "OptionInstance" in w]
                ldcs = [w for w in window.split(" ~ ") if "ldc" in w]
                print(f"  {getfields[-1] if getfields else '?':32s} <- {ldcs[-1] if ldcs else '?'}")
        break

# 2) Options$3.visit: the line format.
print()
print("=== Options$3.visit (the file's own line writer)")
try:
    ms3 = methods_of(z, "net.minecraft.client.Options$3")
    for s, (cp, body) in ms3.items():
        print("--- method:", s)
        for t in body_lines(cp, body):
            print("   ", t)
except Exception as e:
    print("  (no Options$3):", e)

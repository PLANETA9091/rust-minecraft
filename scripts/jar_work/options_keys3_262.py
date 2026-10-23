#!/usr/bin/env python3
"""THE OPTIONS KEYS PROBE (autopush-168 round 3): the processOptions lane's
own process(key, instance) calls — the REAL option keys the constant pool
keeps — and the OptionInstance$IntRange / $UnitDouble codec bytecode (the
encoded value the options.txt line stores). The jar's own facts.
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

# 1) processOptions -> FieldAccess.process(key, instance): the REAL keys.
ms = methods_of(z, "net.minecraft.client.Options")
print("=== processOptions: field <- process key")
for s, (cp, body) in ms.items():
    if "processOptions" in s:
        lines = body_lines(cp, body)
        for i, t in enumerate(lines):
            if ".process:" in t or "process:(Ljava/lang/String;Lnet/minecraft/client/OptionInstance" in t:
                window = " ~ ".join(lines[max(0, i - 16):i + 1])
                getfields = [w.split("getfield")[-1].split(":")[0].split(".")[-1]
                             for w in window.split(" ~ ") if "getfield" in w]
                ldcs = [w for w in window.split(" ~ ") if "ldc" in w]
                print(f"  {getfields[-1] if getfields else '?':34s} <- {ldcs[-1] if ldcs else '?'}")
        break

# 2) The codecs: UnitDouble (sensitivity) and IntRange (fov).
for cls in ("net.minecraft.client.OptionInstance$UnitDouble",
            "net.minecraft.client.OptionInstance$IntRange"):
    print()
    print(f"=== {cls}")
    ms2 = methods_of(z, cls)
    for s, (cp, body) in ms2.items():
        if "codec" in s:
            print("--- method:", s)
            for t in body_lines(cp, body):
                print("   ", t)

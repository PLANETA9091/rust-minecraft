#!/usr/bin/env python3
"""THE OPTIONS FILE LAW PROBE (autopush-168 final): everything the
persistence seat needs — the resolved key strings, the IntRange codec's
encoded value (the fov line's own shape), the SoundSource concat pattern,
the load() read lane. The jar's own facts — the constant pool speaks.
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

def ldc_str_at(cp, tok):
    tail = tok.split("ldcw@")[-1].split()[0].split(":")[0]
    e = cp[int(tail)]
    if e[0] == 8:
        return utf(cp, e[1])
    return None

# A) processDumpedOptions keys (the fov row lives here)
ms = methods_of(z, "net.minecraft.client.Options")
for s, (cp, body) in ms.items():
    if "processDumpedOptions" in s:
        print("=== processDumpedOptions keys")
        for t in body_lines(cp, body):
            if "ldcw@" in t:
                k = ldc_str_at(cp, t)
                if k:
                    print(f"  '{k}'")
        break

# B) IntRange codec + its lambdas: the encoded value shape
print()
print("=== OptionInstance$IntRange codec() full")
ms2 = methods_of(z, "net.minecraft.client.OptionInstance$IntRange")
for s, (cp, body) in ms2.items():
    if "codec" in s or "lambda" in s:
        print("--- method:", s)
        for t in body_lines(cp, body):
            print("   ", t)

# C) SoundSource names (the volume map keys' own ingredient)
print()
print("=== SoundSource clinit (the enum names)")
try:
    ms3 = methods_of(z, "net.minecraft.sounds.SoundSource")
    cp, body = next(b for s, b in ms3.items() if "<clinit>" in s)
    for t in body_lines(cp, body):
        if "ldc" in t or "String" in t:
            print("   ", t)
except Exception as e:
    print("  fail:", e)

# D) load(): the read lane's shape
print()
print("=== Options.load head")
for s, (cp, body) in ms.items():
    if s.startswith("load"):
        for t in body_lines(cp, body)[:45]:
            print("   ", t)
        break

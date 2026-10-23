#!/usr/bin/env python3
"""THE OPTIONS FILE LAW PROBE (round 6): the concat recipe (the SoundSource
key the indy builds — the utf8 constants the cp keeps), the load lambda's
line-split rule (the ':' law the reader wears).
"""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

# A) every utf8 constant in Options + Options$3 + OptionInstance$IntRange
for cls in ("net.minecraft.client.Options", "net.minecraft.client.Options$3"):
    print(f"=== {cls}: utf8 constants with 'olume' / 'soundCategory' / 'Category' / concat-ish")
    try:
        data = z.read(cls.replace(".", "/") + ".class")
        cp, _ = parse_cp(data)
        seen = set()
        for e in cp:
            if e and e[0] == "u":
                s = e[1]
                if ("olume" in s or "oundCategory" in s or "aster" in s or "\\u0001" in s
                        or "soundC" in s):
                    if s not in seen:
                        seen.add(s)
                        print(f"   {s!r}")
    except Exception as ex:
        print("  fail:", ex)

# B) load lambdas: the line-split rule
print()
print("=== Options synthetic lambdas (load side)")
ms = methods_of(z, "net.minecraft.client.Options")
for s, (cp, body) in ms.items():
    if "lambda$load" in s or "lambda$new" in s or ("lambda" in s and "Tag" in s):
        print("--- method:", s)
        clen = struct.unpack_from(">I", body, 4)[0]
        for t in walk_code(cp, body[8:8 + clen]):
            print("   ", t)

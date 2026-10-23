#!/usr/bin/env python3
"""THE OPTIONS KEYS PROBE (autopush-168 round 4): the constant pool's real
key strings, the IntRange codec's full bytecode (the encoded value the fov
line stores), the processOptions special rows (fov/gamma/soundSourceVolumes).
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

# helper: resolve an "ldcw@N" mention to its utf8 string if possible
def ldc_str(cp, tok):
    tok = tok.strip()
    for pref in ("ldcw@", "ldc@", "ldc_w@", "ldc_w ", "ldc "):
        if tok.startswith(pref):
            idx = tok[len(pref):].strip().split()[0]
            try:
                return f"{tok} -> '{utf(cp, int(idx))}'"
            except Exception as e:
                return f"{tok} (utf fail: {e})"
    return tok

ms = methods_of(z, "net.minecraft.client.Options")
print("=== processOptions: keys of interest (raw window dumps)")
for s, (cp, body) in ms.items():
    if "processOptions" in s:
        lines = body_lines(cp, body)
        for i, t in enumerate(lines):
            if "ldc" in t:
                # try to resolve every ldc token in the line
                print(f"  pc{i}: {ldc_str(cp, t.strip())}")
        break

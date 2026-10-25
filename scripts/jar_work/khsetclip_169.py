#!/usr/bin/env python3
"""deref builder CP strings + walk setClipboard (dev-169)"""
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]
methods_of = ns["methods_of"]
walk_code = ns["walk_code"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

d = dict(methods_of(z, "net/minecraft/client/KeyboardHandler"))
sig, (cp, body) = next((s, v) for s, v in d.items() if s.startswith("copyCreateBlockCommand"))
print("===== CP derefs via copyCreateBlockCommand's cp =====")
for idx in (810, 820, 828, 839, 853, 1263, 1261, 715, 795):
    try:
        e = cp[idx]
        s = utf(cp, e[1])
        print(f"  ldcw@{idx} -> {s!r}")
    except Exception as ex:
        print(f"  ldcw@{idx} -> <{ex}>")

print("\n===== setClipboard walk =====")
for s, (c, b) in sorted(d.items()):
    if "setClipboard" in s and b is not None:
        clen = struct.unpack_from(">I", b, 4)[0]
        code = b[8 : 8 + clen]
        print(f"  --- {s} (max_locals={struct.unpack_from('>H', b, 2)[0]})")
        for t in walk_code(c, code):
            print(f"    {t}")

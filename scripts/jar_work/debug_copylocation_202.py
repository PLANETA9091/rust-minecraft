#!/usr/bin/env python3
"""THE DEBUG COPY LOCATION'S OWN WALK (autopush-202): the next
registration the ctor's pc 3588 seat banks — the `ldc_w cp#1496` the
`key.debug.copyLocation` name answers, the `KEYSYM, bipush 67` the C
default the 4-arg ctor (the iconst_0 the orderI default) wears, the
putfield keyDebugCopyLocation the pc 3607 the slot speaks. The en_us
lang row the display word keeps. The NEXT registration after the
putfield (the cp#1501's `bipush 86` the V answers) the later round's
seat banks.

Usage: python3 scripts/jar_work/debug_copylocation_202.py <client.jar>
"""
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

OPT = "net.minecraft.client.Options"

print("########## I. the next registration (pc 3585-3650)")
d = dict(methods_of(z, OPT))
cp, body = d["<init>(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V"]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
for t in walk_code(cp, code):
    pc = t.split(":", 1)[0]
    try:
        p = int(pc)
    except ValueError:
        continue
    if 3585 <= p <= 3650:
        print(f"  {t}")

print()
print("########## II. the cp strings (the two-step indirection)")
for idx in (1496, 1501):
    e = cp[idx]
    print(f"  cp#{idx} -> {utf(cp, e[1])!r}")

print()
print("########## III. the en_us lang rows (key.debug.*)")
lang = None
for n in z.namelist():
    if n.endswith("lang/en_us.json") and "minecraft" in n:
        lang = n
        break
print(f"lang file: {lang}")
if lang:
    import json
    table = json.loads(z.read(lang).decode("utf-8"))
    for k in sorted(table):
        if k.startswith("key.debug"):
            print(f"  {k} = {table[k]}")

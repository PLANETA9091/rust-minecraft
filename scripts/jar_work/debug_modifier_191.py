#!/usr/bin/env python3
"""THE DEBUG MODIFIER'S OWN WALK (autopush-191): the keyDebugModifier
registration the ctor's pc 3254-3275 seat banks — the `ldc_w` the
`key.debug.modifier` name answers, the `KEYSYM, sipush 292` the F3
default the overlay pair shares, the 5-arg ctor the `bipush -1` the
orderI field wears. The en_us lang row the display word keeps.

Usage: python3 scripts/jar_work/debug_modifier_191.py <client.jar>
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

print("########## I. the keyDebugModifier registration (pc 3240-3300)")
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
    if 3240 <= p <= 3300:
        print(f"  {t}")

print()
print("########## II. the en_us lang rows (key.debug.overlay / modifier)")
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

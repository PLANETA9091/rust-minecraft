#!/usr/bin/env python3
"""THE DEBUG WALK CONT'D (dev-160): the ldc strings the cp resolves,
the FULL debug registration count (pc 3229..), the Category clinit's
SORT_ORDER seat, the 5-arg ctor's int arg the sort/modifier law wears.

Usage: python3 scripts/jar_work/debug_overlay_160b.py <client.jar>
"""
import json
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]
methods_of = ns["methods_of"]
class_utf8_words = ns["class_utf8_words"]; list_methods = ns["list_methods"]
walk_code = ns["walk_code"]
# the cp helpers the controls_185 walk wears
import inspect
print("helpers:", [n for n in ns if not n.startswith("__")][:20])

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

OPT = "net.minecraft.client.Options"
KM = "net.minecraft.client.KeyMapping"

cls_bytes = z.read(OPT.replace(".", "/") + ".class")
cp2 = parse_cp(cls_bytes)

print()
print("########## B2. raw cp entries 1390-1430 (the type the tag wears)")
for idx in range(1390, 1431):
    try:
        e = cp2[idx]
        print(f"  cp#{idx} tag={e[0]!r} val={e[1:]!r}"[:110])
    except Exception as ex:
        print(f"  cp#{idx} !! {ex}")

print()
print("########## C2. the ctor walk verbatim (pc 3300-3460) — the debug rows' tail")
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
    if 3300 <= p <= 3460:
        print(f"  {t}")

print()
print("########## F. the 5-arg ctor walk (the int arg the law wears)")
dk = dict(methods_of(z, KM))
sig5 = "<init>(Ljava/lang/String;Lcom/mojang/blaze3d/platform/InputConstants$Type;ILnet/minecraft/client/KeyMapping$Category;I)V"
cp5, body5 = dk[sig5]
clen5 = struct.unpack_from(">I", body5, 4)[0]
code5 = body5[8 : 8 + clen5]
for t in walk_code(cp5, code5):
    print(f"  {t}")

print()
print("########## G. the Category clinit walk (the SORT_ORDER the debug seat wears)")
CAT = "net.minecraft.client.KeyMapping$Category"
dc = dict(methods_of(z, CAT))
cpc, bodyc = dc["<clinit>()V"]
clenc = struct.unpack_from(">I", bodyc, 4)[0]
codec = bodyc[8 : 8 + clenc]
for t in walk_code(cpc, codec):
    print(f"  {t}")

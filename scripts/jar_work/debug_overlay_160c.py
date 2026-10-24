#!/usr/bin/env python3
"""THE DEBUG WALK III (dev-160): the categories' lang rows the table
answers, the FULL debug registration ladder (pc 3454..3720), the 4-arg
ctor's order default the walk wears, the Category's own fields the
display key speaks.

Usage: python3 scripts/jar_work/debug_overlay_160c.py <client.jar>
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

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

OPT = "net.minecraft.client.Options"
KM = "net.minecraft.client.KeyMapping"
CAT = "net.minecraft.client.KeyMapping$Category"

lang = json.loads(z.read("assets/minecraft/lang/en_us.json"))
print("########## A2. ALL key.categories.* lang rows")
for k in sorted(lang):
    if k.startswith("key.categories"):
        print(f"  {k} = {lang[k]!r}")

print()
print("########## C3. the ctor walk verbatim (pc 3454-3760) — the debug ladder's tail")
d = dict(methods_of(z, OPT))
cp, body = d["<init>(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V"]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
last_debug = None
for t in walk_code(cp, code):
    pc = t.split(":", 1)[0]
    try:
        p = int(pc)
    except ValueError:
        continue
    if 3454 <= p <= 3760:
        print(f"  {t}")

print()
print("########## F2. the 4-arg ctor walk (the order default the walk wears)")
dk = dict(methods_of(z, KM))
sig4 = "<init>(Ljava/lang/String;Lcom/mojang/blaze3d/platform/InputConstants$Type;ILnet/minecraft/client/KeyMapping$Category;)V"
cp4, body4 = dk[sig4]
clen4 = struct.unpack_from(">I", body4, 4)[0]
code4 = body4[8 : 8 + clen4]
for t in walk_code(cp4, code4):
    print(f"  {t}")

print()
print("########## H. the Category class (fields + methods + the display key)")
try:
    print(list_methods(z, CAT))
    class_utf8_words(z, CAT, ["key.categories", "getKey", "name"])
except Exception as e:
    print("!!", e)

#!/usr/bin/env python3
"""THE SPECTATOR KEYS PROBE (the autopush-190 round): the pair of
spectator rows the activator pair's AFTER the ctor registers (the
putfields the 189 walk already banked the NEXT candidates' first seat)
— the ldc strings the cp index resolves, the category the ctor's
getstatic answers, the lang rows the captions speak, the save walk the
getfield order answers.

Usage: python3 scripts/jar_work/spectator_190.py <client.jar>
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

print("########## A. lang rows (the spectator keys + the categories speak)")
lang = json.loads(z.read("assets/minecraft/lang/en_us.json"))
for k in sorted(lang):
    if "spectator" in k.lower() and k.startswith("key"):
        print(f"  {k} = {lang[k]!r}")
for k in sorted(lang):
    if k.startswith("key.categories"):
        print(f"  {k} = {lang[k]!r}")

print()
print("########## B. the ctor's ldc strings after the pair (cp walk 1391..1420)")
cls_bytes = z.read(OPT.replace(".", "/") + ".class")
cp2 = parse_cp(cls_bytes)
for idx in range(1391, 1421):
    try:
        w = utf(cp2, idx)
        if w and any(c.isalpha() for c in w) and len(w) < 80:
            print(f"  cp#{idx} = {w!r}")
    except Exception:
        pass

print()
print("########## C. the ctor walk verbatim (pc 3182-3260)")
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
    if 3182 <= p <= 3260:
        print(f"  {t}")

print()
print("########## D. the InputConstants.UNKNOWN law (the field the probe wears)")
try:
    ic = "net.minecraft.client.InputConstants"
    class_utf8_words(z, ic, ["UNKNOWN"])
except Exception as e:
    print("!!", e)

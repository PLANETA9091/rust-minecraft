#!/usr/bin/env python3
"""THE SAVE/LOAD ACTIVATOR PROBE (the autopush-189 round): the pair the
hotbar array's AFTER the ctor registers (the putfields the 188 walk
already banked) — the ldc strings the cp index resolves, the lang rows
the captions speak, the save walk the getfield order answers, the
CREATIVE category the sort seat re-wears.

Usage: python3 scripts/jar_work/save_load_189.py <client.jar>
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

print("########## A. lang rows (the pair + the creative category speak)")
lang = json.loads(z.read("assets/minecraft/lang/en_us.json"))
for k in sorted(lang):
    if "otbarActivator" in k or "categories.creative" in k:
        print(f"  {k} = {lang[k]!r}")

print()
print("########## B. the ctor's ldc strings (cp 1381 / 1389 resolve)")
d = dict(methods_of(z, OPT))
cp, body = d["<init>(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V"]
# the parse_cp table — walk the utf8 entries the indices wear
entries = parse_cp(z.read(OPT.replace(".", "/") + ".class")) if False else None
# simpler: re-parse the class cp directly via parse_cp on the class bytes
cls_bytes = z.read(OPT.replace(".", "/") + ".class")
cp2 = parse_cp(cls_bytes)
for idx in (1381, 1389):
    try:
        print(f"  cp#{idx} = {utf(cp2, idx)!r}")
    except Exception as e:
        print(f"  cp#{idx} !! {e}")

print()
print("########## C. the ctor walk verbatim (pc 3144-3190)")
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
for t in walk_code(cp, code):
    pc = t.split(":", 1)[0]
    try:
        p = int(pc)
    except ValueError:
        continue
    if 3144 <= p <= 3190:
        print(f"  {t}")

print()
print("########## D. the Category clinit — the sort seat re-wears")
CAT = "net.minecraft.client.KeyMapping$Category"
try:
    d2 = dict(methods_of(z, CAT))
    for sig in d2:
        if "<clinit>" in sig or "ordinal" in sig or "name" in sig.lower():
            print(f"*** {sig}")
            c2, b2 = d2[sig]
            l2 = struct.unpack_from(">I", b2, 4)[0]
            for t in walk_code(c2, b2[8 : 8 + l2]):
                print(f"  {t}")
except Exception as e:
    print("!!", e)
class_utf8_words(z, CAT, ["key.categories"])

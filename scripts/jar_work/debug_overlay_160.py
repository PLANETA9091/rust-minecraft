#!/usr/bin/env python3
"""THE DEBUG OVERLAY PROBE (the dev-160 round): the keyDebugOverlay row
the Options ctor's pc 3229-3251 walk the NEXT candidates' first seat
wears — the ldc string the cp resolves, the KEYSYM 292 the F3 answers,
the DEBUG category the ctor overload the -2 arg the walk banks.

Usage: python3 scripts/jar_work/debug_overlay_160.py <client.jar>
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

print("########## A. lang rows (the debug keys + the category speak)")
lang = json.loads(z.read("assets/minecraft/lang/en_us.json"))
for k in sorted(lang):
    if k.startswith("key.debug") or k == "key.categories.debug":
        print(f"  {k} = {lang[k]!r}")

print()
print("########## B. the ctor's ldc strings after the pair (cp walk 1391..1440)")
cls_bytes = z.read(OPT.replace(".", "/") + ".class")
cp2 = parse_cp(cls_bytes)
for idx in range(1391, 1441):
    try:
        w = utf(cp2, idx)
        if w and any(c.isalpha() for c in w) and len(w) < 80:
            print(f"  cp#{idx} = {w!r}")
    except Exception:
        pass

print()
print("########## C. the ctor walk verbatim (pc 3216-3300)")
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
    if 3216 <= p <= 3300:
        print(f"  {t}")

print()
print("########## D. the KeyMapping ctor overloads (the modifier the -2 arg wears)")
try:
    print(list_methods(z, KM)[:40])
except Exception as e:
    print("!!", e)

print()
print("########## E. the category enum's clinit (the SORT_ORDER the debug seat wears)")
try:
    cand = [
        "net.minecraft.client.KeyMapping$Category",
        "net.minecraft.client.KeyMapping$1",
    ]
    for c in cand:
        try:
            z.read(c.replace(".", "/") + ".class")
            print("  class found:", c)
            class_utf8_words(z, c, ["SORT_ORDER", "debug", "Debug", "categories.debug"])
            break
        except KeyError:
            continue
except Exception as e:
    print("!!", e)

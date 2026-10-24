#!/usr/bin/env python3
"""THE MOUSE-KEY LAW PROBE (the autopush-188 round, part 2): the
com/mojang/blaze3d/platform/InputConstants$Type MOUSE the save prefix
('mouse.' the getName row wears) + the Key the display names speak,
and the class the 'key.attack' registration answers (the trio's own
defaults the MOUSE codes wear).

Usage: python3 scripts/jar_work/mousekey_188.py <client.jar>
"""
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]
methods_of = ns["methods_of"]; dump_method = ns["dump_method"]
class_utf8_words = ns["class_utf8_words"]; list_methods = ns["list_methods"]
walk_code = ns["walk_code"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

IC = "com.mojang.blaze3d.platform.InputConstants"
ICT = IC + "$Type"
ICK = IC + "$Key"

def show(cp, body, label):
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8 : 8 + clen]
    print(f"--- {label}")
    for t in walk_code(cp, code):
        print(t)

print("########## A. InputConstants$Type — the enum walk")
try:
    list_methods(z, ICT)
    d = dict(methods_of(z, ICT))
    for sig in d:
        print(f"*** {sig}")
        cp, body = d[sig]
        show(cp, body, sig)
except Exception as e:
    print("!!", e)
try:
    class_utf8_words(z, ICT, ["mouse", "scancode", "key.keyboard", "key.mouse", "prefix"])
except Exception as e:
    print("!!", e)

print()
print("########## B. InputConstants$Key — the name/display law")
try:
    list_methods(z, ICK)
    d = dict(methods_of(z, ICK))
    for sig in d:
        if "getName" in sig or "<init>" in sig or "getDisplayName" in sig or "Type" in sig:
            print(f"*** {sig}")
            cp, body = d[sig]
            show(cp, body, sig)
except Exception as e:
    print("!!", e)

print()
print("########## C. InputConstants <clinit> — the keyboard map + UNKNOWN")
try:
    d = dict(methods_of(z, IC))
    for sig in d:
        if sig.startswith("<clinit>") or "getByKey" in sig or "getByName" in sig or "getOrCreate" in sig or "getKey" in sig:
            print(f"*** {sig}")
            cp, body = d[sig]
            show(cp, body, sig)
except Exception as e:
    print("!!", e)
try:
    class_utf8_words(z, IC, ["mouse.", "scancode.", "key.keyboard.unknown", "UNKNOWN"])
except Exception as e:
    print("!!", e)

print()
print("########## D. the class 'key.attack' answers (the registration walk)")
names = z.namelist()
hits = []
for n in names:
    if n.endswith(".class") and n.startswith("net/minecraft/client/"):
        b = z.read(n)
        if b"key.attack" in b or b"key.pickItem" in b:
            hits.append(n)
for n in hits:
    print("HIT:", n)
for n in hits[:4]:
    cls = n[:-6].replace("/", ".")
    print(f"=== {cls}")
    try:
        class_utf8_words(z, cls, ["key.attack", "key.use", "key.pickItem", "MOUSE"])
        d = dict(methods_of(z, cls))
        for sig in d:
            if sig.startswith("<init>") or sig.startswith("<clinit>") or "register" in sig or "create" in sig:
                print(f"*** {sig}")
                cp, body = d[sig]
                show(cp, body, sig)
    except Exception as e:
        print("!!", e)

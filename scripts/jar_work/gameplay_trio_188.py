#!/usr/bin/env python3
"""THE GAMEPLAY TRIO PROBE (the autopush-188 round): the
key.attack/key.use/key.pickItem MOUSE binds the Options ctor the
array wears, the Category enum's own ordinal order (the list's group
sort the gameplay BETWEEN movement/inventory banks), the
InputConstants$Type.MOUSE the save prefix ('mouse.') + the display
rows (key.mouse.left/right/middle) speak, and the KeyBindsScreen
mouseClicked the selectedKey gate the capture answers.

Usage: python3 scripts/jar_work/gameplay_trio_188.py <client.jar>
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

OPTS = "net.minecraft.client.Options"
CAT = "net.minecraft.client.KeyMapping$Category"
KM = "net.minecraft.client.KeyMapping"
ICT = "net.minecraft.client.InputConstants$Type"
KBS = "net.minecraft.client.gui.screens.options.controls.KeyBindsScreen"

def show(cp, body, label):
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8 : 8 + clen]
    print(f"--- {label}")
    for t in walk_code(cp, code):
        print(t)

print("########## A. lang rows (the trio + the mouse buttons speak)")
import json
lang = json.loads(z.read("assets/minecraft/lang/en_us.json"))
for k in sorted(lang):
    if k in (
        "key.attack", "key.use", "key.pickItem",
        "key.categories.gameplay",
        "key.mouse.left", "key.mouse.right", "key.mouse.middle",
        "key.mouse.button",
    ) or k.startswith("key.mouse.button."):
        print(f"  {k} = {lang[k]!r}")

print()
print("########## B. the Category enum's own <clinit> (the ordinals)")
try:
    d = dict(methods_of(z, CAT))
    for sig in d:
        if sig.startswith("<clinit>") or sig.startswith("<init>") or "values" in sig or "byName" in sig or "id" in sig.lower():
            print(f"*** {sig}")
            cp, body = d[sig]
            show(cp, body, sig)
except Exception as e:
    print("!!", e)
try:
    class_utf8_words(z, CAT, ["movement", "gameplay", "inventory", "creative", "multiplayer", "interface", "misc"])
except Exception as e:
    print("!!", e)

print()
print("########## C. Options ctor — the trio's own registrations")
d = dict(methods_of(z, OPTS))
ctor_sig = next(s for s in d if s.startswith("<init>"))
cp, body = d[ctor_sig]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
lines = list(walk_code(cp, code))
hits = [
    i for i, t in enumerate(lines)
    if "key.attack" in t or "key.use\"" in t or "key.pickItem" in t
]
shown = set()
for i in hits:
    for j in range(max(0, i - 6), min(len(lines), i + 10)):
        if j not in shown:
            print(lines[j])
            shown.add(j)
    print("     ----")

print()
print("########## D. KeyMapping the saveString/getName law")
try:
    d = dict(methods_of(z, KM))
    for sig in d:
        if "saveString" in sig or "getName" in sig or "isDefault" in sig or "setKey" in sig or "getDefaultKey" in sig or "getTranslatedKeyMessage" in sig or "getCategory" in sig:
            print(f"*** {sig}")
            cp, body = d[sig]
            show(cp, body, sig)
except Exception as e:
    print("!!", e)

print()
print("########## E. InputConstants$Type — the MOUSE arm (the prefix + the names)")
try:
    d = dict(methods_of(z, ICT))
    for sig in d:
        print(f"*** {sig}")
        cp, body = d[sig]
        show(cp, body, sig)
except Exception as e:
    print("!!", e)
try:
    class_utf8_words(z, "net.minecraft.client.InputConstants", ["mouse.", "scancode", "key.mouse", "key.keyboard.unknown"])
except Exception as e:
    print("!!", e)

print()
print("########## F. KeyBindsScreen — the mouseClicked selectedKey gate")
try:
    list_methods(z, KBS)
    d = dict(methods_of(z, KBS))
    for sig in d:
        if "mouseClicked" in sig or "keyPressed" in sig:
            print(f"*** {sig}")
            cp, body = d[sig]
            show(cp, body, sig)
except Exception as e:
    print("!!", e)
try:
    class_utf8_words(z, KBS, ["selectedKey", "mouse", "InputConstants"])
except Exception as e:
    print("!!", e)

#!/usr/bin/env python3
"""THE INVENTORY TRIO + THE LIST SCROLL PROBE (the autopush-187 round):
the trio the Options ctor registers after the movement pair (the
keyInventory/keySwapOffhand/keyDrop rows the INVENTORY category wears),
the lang rows the captions answer, the Category enum the list's group
order keeps, and the AbstractSelectionList/AbstractScrollArea the
mouseScrolled wheel law serves (the scrollStep the list scroll wears).

Usage: python3 scripts/jar_work/keymaps_trio_187.py <client.jar>
"""
import json
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
ASL = "net.minecraft.client.gui.components.AbstractSelectionList"
ASA = "net.minecraft.client.gui.components.AbstractScrollArea"
KBL = "net.minecraft.client.gui.screens.options.controls.KeyBindsList"

print("########## A. lang rows (the trio + the categories speak)")
lang = json.loads(z.read("assets/minecraft/lang/en_us.json"))
for k in sorted(lang):
    if k in (
        "key.inventory",
        "key.swapOffhand",
        "key.drop",
        "key.categories.inventory",
        "key.categories.movement",
        "key.categories.gameplay",
        "key.keyboard.unknown",
        "controls.reset",
        "controls.resetAll",
    ):
        print(f"  {k} = {lang[k]!r}")

print()
print("########## B. Options ctor — the trio's own registrations")
d = dict(methods_of(z, OPTS))
ctor_sig = next(s for s in d if s.startswith("<init>"))
cp, body = d[ctor_sig]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
lines = list(walk_code(cp, code))
hits = [
    i
    for i, t in enumerate(lines)
    if "key.inventory" in t
    or "key.swapOffhand" in t
    or "key.drop" in t
    or "key.categories.inventory" in t
]
shown = set()
for i in hits:
    for j in range(max(0, i - 8), min(len(lines), i + 9)):
        if j not in shown:
            print(lines[j])
            shown.add(j)
    print("     ----")

print()
print("########## C. the Category enum's own order (the list groups)")
try:
    class_utf8_words(z, CAT, ["key.categories"])
    d = dict(methods_of(z, CAT))
    if "<clinit>" in d:
        cp, body = d["<clinit>"]
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        for txt in walk_code(cp, code):
            print(txt)
except Exception as e:
    print("!!", e)

print()
print("########## D. the scroll law (mouseScrolled the wheel wears)")
for cls in (ASL, ASA):
    try:
        print(f"-- {cls}")
        list_methods(z, cls)
        d = dict(methods_of(z, cls))
        for sig in d:
            if "mouseScrolled" in sig or "scrollStep" in sig or "maxScrollAmount" in sig:
                print(f"   *** {sig}")
                cp, body = d[sig]
                clen = struct.unpack_from(">I", body, 4)[0]
                code = body[8 : 8 + clen]
                for txt in walk_code(cp, code):
                    print(txt)
    except Exception as e:
        print("!!", e)

print()
print("########## D2. the scrollRate law (the step the wheel multiplies)")
for cls in (ASA, ASL):
    try:
        d = dict(methods_of(z, cls))
        for sig in d:
            if "scrollRate" in sig or "cellHeight" in sig or sig.startswith("setScrollAmount"):
                print(f"-- {cls} *** {sig}")
                cp, body = d[sig]
                clen = struct.unpack_from(">I", body, 4)[0]
                code = body[8 : 8 + clen]
                for txt in walk_code(cp, code):
                    print(txt)
    except Exception as e:
        print("!!", e)

print()
print("########## E. KeyBindsList ctor (the group walk the headers wear)")
try:
    d = dict(methods_of(z, KBL))
    for sig in d:
        if sig.startswith("<init>") or "update" in sig:
            print(f"*** {sig}")
            cp, body = d[sig]
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8 : 8 + clen]
            out = list(walk_code(cp, code))
            hits = [
                i
                for i, t in enumerate(out)
                if "Category" in t or "addEntry" in t or "categories" in t
            ]
            shown = set()
            for i in hits:
                for j in range(max(0, i - 3), min(len(out), i + 4)):
                    if j not in shown:
                        print(out[j])
                        shown.add(j)
                print("     ----")
except Exception as e:
    print("!!", e)

#!/usr/bin/env python3
"""THE SCROLL RATE PROBE (autopush-187 follow-up): the
AbstractScrollArea$ScrollbarSettings the scrollRate() default keeps,
the KeyBindsList the cellHeight the rows wear, the CategoryEntry the
header's own cell keeps.

Usage: python3 scripts/jar_work/scrollrate_187.py <client.jar>
"""
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
methods_of = ns["methods_of"]; dump_method = ns["dump_method"]
class_utf8_words = ns["class_utf8_words"]; list_methods = ns["list_methods"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

SBS = "net.minecraft.client.gui.components.AbstractScrollArea$ScrollbarSettings"
KBL = "net.minecraft.client.gui.screens.options.controls.KeyBindsList"
CE = KBL + "$CategoryEntry"
ASL = "net.minecraft.client.gui.components.AbstractSelectionList"

for cls in (SBS, CE):
    try:
        print(f"===== {cls}")
        list_methods(z, cls)
        d = dict(methods_of(z, cls))
        for sig in d:
            if "scrollRate" in sig or sig.startswith("<init>") or "height" in sig:
                print(f"*** {sig}")
                cp, body = d[sig]
                clen = struct.unpack_from(">I", body, 4)[0]
                code = body[8 : 8 + clen]
                for txt in ns["walk_code"](cp, code):
                    print(txt)
    except Exception as e:
        print("!!", e)

try:
    print("===== KeyBindsList <init> (the cellHeight the super call wears)")
    d = dict(methods_of(z, KBL))
    for sig in d:
        if sig.startswith("<init>"):
            cp, body = d[sig]
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8 : 8 + clen]
            out = list(ns["walk_code"](cp, code))
            for t in out[:40]:
                print(t)
            break
except Exception as e:
    print("!!", e)

try:
    print("===== AbstractSelectionList: scrollRate/cellHeight overrides")
    d = dict(methods_of(z, ASL))
    for sig in d:
        if "scrollRate" in sig or "cellHeight" in sig or "contentHeight" in sig:
            print(f"*** {sig}")
            cp, body = d[sig]
            clen = struct.unpack_from(">I", body, 4)[0]
            code = body[8 : 8 + clen]
            for txt in ns["walk_code"](cp, code):
                print(txt)
except Exception as e:
    print("!!", e)

print("===== AbstractScrollArea <clinit> + <init> (the DEFAULT rate the list wears)")
d = dict(methods_of(z, "net.minecraft.client.gui.components.AbstractScrollArea"))
for sig in d:
    if sig == "<clinit>()V" or sig.startswith("<init>"):
        print(f"*** {sig}")
        cp, body = d[sig]
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        for txt in ns["walk_code"](cp, code)[:60]:
            print(txt)

print("===== AbstractSelectionList <init> (the ScrollbarSettings the list constructs)")
d = dict(methods_of(z, "net.minecraft.client.gui.components.AbstractSelectionList"))
for sig in d:
    if sig.startswith("<init>"):
        print(f"*** {sig}")
        cp, body = d[sig]
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        out = list(ns["walk_code"](cp, code))
        hits = [i for i, t in enumerate(out) if "ScrollbarSettings" in t]
        shown = set()
        for i in hits:
            for j in range(max(0, i - 12), min(len(out), i + 3)):
                if j not in shown:
                    print(out[j])
                    shown.add(j)
            print("----")

print("===== AbstractScrollArea.defaultSettings(I) (the rate the divisor answers)")
d = dict(methods_of(z, "net.minecraft.client.gui.components.AbstractScrollArea"))
for sig in d:
    if "defaultSettings" in sig:
        print(f"*** {sig}")
        cp, body = d[sig]
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        for txt in ns["walk_code"](cp, code):
            print(txt)

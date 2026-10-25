#!/usr/bin/env python3
"""THE SLIDER PROBE'S SECOND WALK (autopush-241): the getSprite body,
the setFocused body, the class fields (canChangeValue the seat the
setFocused arms), the AbstractWidget isHoveredOrFocused + renderWidget
the sprite ask serves, the setValue the canChangeValue writers bank.

Usage: python3 scripts/jar_work/slider2_241.py
"""
import sys, zipfile, json, re
import os

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)
methods_of = NS["methods_of"]; walk_code = NS["walk_code"]
utf = NS["utf"]; ref_name = NS["ref_name"]
parse_cp = NS["parse_cp"]; struct = NS["struct"]

JAR = ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)


def dump_bodies(label, cls, wants, exclude=None):
    print("=" * 72)
    print(f"[{label}] {cls}")
    try:
        meths = methods_of(z, cls)
    except Exception as e:
        print("  MISSING:", e)
        return
    for sig, (cp, body) in meths.items():
        if body is None:
            continue
        if exclude and any(x in sig for x in exclude):
            continue
        if not any(w in sig for w in wants):
            continue
        print("-" * 72)
        print(f"--- {sig}")
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        try:
            for ln in walk_code(cp, code):
                print("  ", ln)
        except Exception as e:
            print("  walk fail:", e)


ASB = "net/minecraft/client/gui/components/AbstractSliderButton"
AW = "net/minecraft/client/gui/components/AbstractWidget"

print("[FIELDS] AbstractSliderButton:")
import json as _json
try:
    methods, fields = parse_cp(z, ASB) if False else (None, None)
except Exception:
    pass
# crude: read fields from class bytes
try:
    cls_bytes = z.read(ASB + ".class")
    import struct as _s
    minor, major = _s.unpack_from(">HH", cls_bytes, 4)
    off = 8
    # constant pool
    n_cp = _s.unpack_from(">H", cls_bytes, off)[0]
    off += 2
    idx = 1
    cpool = {}
    while idx < n_cp:
        tag = cls_bytes[off]
        if tag == 1:
            ln = _s.unpack_from(">H", cls_bytes, off + 1)[0]
            cpool[idx] = ("utf8", cls_bytes[off+3:off+3+ln].decode("utf-8", "replace"))
            off += 3 + ln
        elif tag in (7, 8, 16, 19, 20):
            cpool[idx] = ("ref", _s.unpack_from(">H", cls_bytes, off + 1)[0])
            off += 3
        elif tag == 15:
            off += 5
        elif tag in (3, 4):
            off += 5
        elif tag in (5, 6):
            cpool[idx] = ("long", None)
            off += 9
            idx += 1
        elif tag in (9, 10, 11, 12, 17, 18):
            cpool[idx] = ("member", _s.unpack_from(">HH", cls_bytes, off + 1))
            off += 5
        else:
            print("  ?? tag", tag, "at", idx)
            break
        idx += 1
    acc = _s.unpack_from(">H", cls_bytes, off)[0]; off += 2
    this_i, sup_i = _s.unpack_from(">HH", cls_bytes, off); off += 4
    n_if = _s.unpack_from(">H", cls_bytes, off)[0]; off += 2 + 2 * n_if
    n_f = _s.unpack_from(">H", cls_bytes, off)[0]; off += 2
    for _ in range(n_f):
        f_acc, name_i, desc_i = _s.unpack_from(">HHH", cls_bytes, off)
        off += 6
        n_at = _s.unpack_from(">H", cls_bytes, off)[0]; off += 2
        for _ in range(n_at):
            an_i = _s.unpack_from(">H", cls_bytes, off)[0]
            alen = _s.unpack_from(">I", cls_bytes, off + 2)[0]
            off += 6 + alen
        nm = cpool.get(name_i, ("?",))[1] if name_i in cpool else "?"
        ds = cpool.get(desc_i, ("?",))[1] if desc_i in cpool else "?"
        print(f"  field {nm} {ds}")
except Exception as e:
    print("  field walk fail:", e)

dump_bodies("C", ASB, ["getSprite", "setFocused", "canChangeValue", "setValue", "onDrag", "keyPressed"])
dump_bodies("D", AW, ["isHoveredOrFocused", "renderWidget", "setFocused"])

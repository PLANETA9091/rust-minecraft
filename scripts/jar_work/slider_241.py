#!/usr/bin/env python3
"""THE SLIDER'S OWN FOCUS RENDER PROBE (the autopush-241 round): the
AbstractSliderButton renderWidget walk the isHoveredOrFocused law the
track/handle sprites wear — the exact sprite-selection arms the round's
own impl banks (the hovered-only the stale shape, the focused the
highlighted frame joins the law).

Chain-loads controls_185.py helpers. Usage: python3 scripts/jar_work/slider_241.py
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


def dump_bodies(label, cls, wants):
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
dump_bodies("A", ASB, ["renderWidget", "getTrackSprite", "getHandleSprite", "updateSliderValue", "render"])
print("=" * 72)
print("[B] the class's own method inventory (render-adjacent names):")
meths = methods_of(z, ASB)
for sig in sorted(meths):
    if "render" in sig or "prite" in sig or "ocus" in sig:
        print("  ", sig)

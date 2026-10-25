#!/usr/bin/env python3
"""THE ARROW NAVIGATION'S OWN PROBE (the autopush-240 round): the
ContainerEventHandler's geometric walk the arrow keys serve — the
handleArrowNavigation default, the nextFocusPathInDirection the full
body (the focus2_173 dump the head carried), the two lambda sort keys,
the getBoundInDirection/ScreenAxis helpers the math keeps.

Chain-loads controls_185.py helpers. Usage: python3 scripts/jar_work/arrow_240.py
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


CEH = "net/minecraft/client/gui/components/events/ContainerEventHandler"
dump_bodies("A", CEH, ["handleArrowNavigation", "nextFocusPathInDirection", "nextFocusPathVaguelyInDirection"])
dump_bodies("B", "net/minecraft/client/gui/navigation/ScreenRectangle", ["getBoundInDirection"])
dump_bodies("C", "net/minecraft/client/gui/navigation/ScreenAxis", [])
dump_bodies("D", "net/minecraft/client/gui/navigation/ScreenDirection", ["getAxis", "getPositive", "getOpposite"])

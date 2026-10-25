#!/usr/bin/env python3
"""THE TAB-KEY FOCUS CYCLE'S OWN PROBE, PART TWO (the dev-173 round):
the traversal law's own bodies — the container nextFocusPath walk the
children order serves, the changeFocus apply, the list's row walk,
the DebugOptionsScreen.setInitialFocus the box's own initial seat.

Chain-loads controls_185.py helpers. Usage: python3 scripts/jar_work/focus2_173.py
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


dump_bodies("A", "net/minecraft/client/gui/components/events/AbstractContainerEventHandler",
            ["nextFocusPath", "changeFocus"])
dump_bodies("B", "net/minecraft/client/gui/screens/Screen", ["changeFocus", "setInitialFocus"])
dump_bodies("C", "net/minecraft/client/gui/screens/debug/DebugOptionsScreen",
            ["setInitialFocus", "init()", "addProfileButton"])
dump_bodies("D", "net/minecraft/client/gui/components/AbstractSelectionList",
            ["nextFocusPath", "changeFocus"])
dump_bodies("E", "net/minecraft/client/gui/components/ObjectSelectionList",
            ["nextFocusPath", "changeFocus"])

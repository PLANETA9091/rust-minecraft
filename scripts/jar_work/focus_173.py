#!/usr/bin/env python3
"""THE TAB-KEY FOCUS CYCLE'S OWN PROBE (the dev-173 round): the Screen
keyPressed walk the Tab arm answers — the createTabEvent(Z) the
FocusNavigationEvent$TabNavigation the walk serves, the changeFocus
(ComponentPath) the container's own law speaks, the traversal the
FocusNavigationEvent classes the order wears, the DebugOptionsScreen
the own override (or never), the AbstractWidget render the isFocused
arm the highlight answers.

Chain-loads controls_185.py helpers. Usage: python3 scripts/jar_work/focus_173.py
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


def list_methods(label, cls):
    print("=" * 72)
    print(f"[{label}] {cls} methods:")
    try:
        meths = methods_of(z, cls)
    except Exception as e:
        print("  MISSING:", e)
        return
    for sig in sorted(meths):
        print("  ", sig)


# [A] Screen: the Tab arm + the focus walk
dump_bodies("A", "net/minecraft/client/gui/screens/Screen",
            ["keyPressed", "changeFocus", "createTabEvent", "clearFocus"])

# [B] The container's default focus walk
dump_bodies("B", "net/minecraft/client/gui/components/events/AbstractContainerEventHandler",
            ["changeFocus", "setFocused", "getFocused"])

# [C] The navigation classes (the traversal law)
list_methods("C-list", "net/minecraft/client/gui/navigation/FocusNavigationEvent$TabNavigation")
dump_bodies("C-body", "net/minecraft/client/gui/navigation/FocusNavigationEvent$TabNavigation", ["<init>", "handler"])

# [D] The debug options screen's OWN key walk (the override the screen
# wears or never)
list_methods("D", "net/minecraft/client/gui/screens/debug/DebugOptionsScreen")

# [E] The widget focus render arm
dump_bodies("E", "net/minecraft/client/gui/components/AbstractWidget",
            ["render", "isFocused", "setFocused", "nextFocusPath"])

# [F] The EditBox's keyPressed (the box the Tab the walk consumes)
dump_bodies("F", "net/minecraft/client/gui/components/EditBox", ["keyPressed", "setFocused"])

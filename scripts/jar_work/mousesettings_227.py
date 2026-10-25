#!/usr/bin/env python3
"""THE MOUSE SETTINGS' OWN ARRAY (autopush-227): the
MouseSettingsScreen.options(Options) the 6-instance array the walk
banks — the rows' getters the Options fields answer, the caption
lambdas the wheel/bool rows speak.
"""
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]
methods_of = ns["methods_of"]
walk_code = ns["walk_code"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

def dump_method(cls, needle):
    d = dict(methods_of(z, cls))
    for sig, (cp, body) in sorted(d.items()):
        if needle not in sig:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        print(f"=== {cls}.{sig}")
        for t in walk_code(cp, code):
            print(f"  {t}")
        print()

MSC = "net.minecraft.client.gui.screens.options.MouseSettingsScreen"
dump_method(MSC, "options")
dump_method(MSC, "addOptions")
print("########## the Options getters the rows wear")
for g in [
    "mouseWheelSensitivity()",
    "invertMouse()",
    "discreteMouseScroll()",
    "touchscreen()",
    "toggleCrouch()",
    "toggleSprint()",
    "sensitivity()",
]:
    dump_method("net.minecraft.client.Options", g)

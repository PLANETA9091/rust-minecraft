#!/usr/bin/env python3
"""THE BAND'S OWN CALL SITES (autopush-179d): the ChatComponent rows the
Options.getBackgroundColor / getBackgroundOpacity calls wear — the input
band's raw operand the lane answers, the message-BG lane the opacity law
the lambda rides.
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

def dis(cp, body):
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    return core["disasm"](cp, code)

CLASSES = [
    "net.minecraft.client.gui.components.ChatComponent",
    "net.minecraft.client.gui.components.ChatComponent$1",
    "net.minecraft.client.gui.components.ChatComponent$2",
    "net.minecraft.client.gui.components.ChatComponent$State",
]
for cls in CLASSES:
    got = core["method_bodies"](z, cls)
    if got is None:
        print("!! no class", cls)
        continue
    cp, bs = got
    if bs is None:
        print("!! no class", cls)
        continue
    for (nm, desc), b in sorted(bs.items()):
        if b is None:
            continue
        txt = "\n".join(l for _, l in dis(cp, b))
        if "getBackgroundColor" in txt or "getBackgroundOpacity" in txt:
            print("==", cls, "::", nm, desc)
            for pc, l in dis(cp, b):
                print("   %4d: %s" % (pc, l))
            print()

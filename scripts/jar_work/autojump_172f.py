#!/usr/bin/env python3
"""THE AUTOJUMP GATES PROBE f (dev-172): isMoving + isStayingOnGroundSurface
+ makeJump the canAutoJump/aiStep gates walk.

Usage: python3 scripts/jar_work/autojump_172f.py [client.jar]
"""
import os
import sys
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)

walk_code = NS["walk_code"]
methods_of = NS["methods_of"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)


def find_method(z, cls, name_sub, max_hits=1):
    ms = methods_of(z, cls)
    out = []
    for sig, (cp, body) in ms.items():
        if name_sub in sig.split("(")[0] and body:
            clen = int.from_bytes(body[4:8], "big")
            code = body[8:8 + clen]
            try:
                lines = walk_code(cp, code)
            except Exception as ex:
                lines = [f"!! desync {ex}"]
            out.append((sig, lines))
        if len(out) >= max_hits:
            break
    return out


LP = "net.minecraft.client.player.LocalPlayer"
IP = "net.minecraft.world.entity.player.Input"

for cls, name in [(LP, "isMoving"), (LP, "isStayingOnGroundSurface"), (LP, "makeJump"),
                  (IP, "makeJump"), (IP, "jump")]:
    hits = find_method(z, cls, name, 3)
    if not hits:
        print(f"!! {cls}: {name} not found")
        continue
    for sig, lines in hits:
        print(f"--- {cls}.{sig}")
        for ln in lines[:40]:
            print(ln)
        print()

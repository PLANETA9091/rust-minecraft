#!/usr/bin/env python3
"""THE AUTOJUMP LAW PROBE c (the dev-172 round): the full updateAutoJump +
canAutoJump bodies, the aiStep pc window around the autoJumpTime countdown,
the updateAutoJump call-sites across LocalPlayer, the move() override walk.

Usage: python3 scripts/jar_work/autojump_172c.py [client.jar]
"""
import os
import sys
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)

walk_code = NS["walk_code"]
dump_method = NS["dump_method"]
methods_of = NS["methods_of"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

LP = "net.minecraft.client.player.LocalPlayer"

print("[1] updateAutoJump(FF) FULL body")
dump_method(z, LP, "updateAutoJump", 400)

print()
print("[2] canAutoJump() FULL body")
dump_method(z, LP, "canAutoJump", 200)

print()
print("[3] aiStep pc window 160-230 (the autoJumpTime countdown ride)")
ms = methods_of(z, LP)
for sig, (cp, body) in ms.items():
    if "aiStep" in sig and body:
        clen = int.from_bytes(body[4:8], "big")
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        for ln in lines[160:230]:
            print(ln)

print()
print("[4] call-sites of updateAutoJump + isAutoJumpEnabled across LocalPlayer")
for sig, (cp, body) in ms.items():
    if not body:
        continue
    clen = int.from_bytes(body[4:8], "big")
    code = body[8:8 + clen]
    try:
        lines = walk_code(cp, code)
    except Exception:
        continue
    hits = [ln for ln in lines if "updateAutoJump" in ln or "isAutoJumpEnabled" in ln]
    if hits:
        print(f"--- {sig}")
        for ln in hits:
            print("   " + ln)

#!/usr/bin/env python3
"""THE canInterruptScreen PROBE (the autopush-213 round): the
Minecraft.canInterruptScreen walk the F3+F6 else arm gates — the
body, the arms, the Screen method the verdict delegates.

Chain-loads controls_185.py helpers.

Usage: python3 scripts/jar_work/canint_213.py
"""
import os, zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)
methods_of = NS["methods_of"]; walk_code = NS["walk_code"]
struct = NS["struct"]

JAR = ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

print("=" * 72)
print("[A] Minecraft.canInterruptScreen body:")
MC = "net/minecraft/client/Minecraft"
meths = methods_of(z, MC)
found = False
for sig, (cp, body) in meths.items():
    if "canInterruptScreen" in sig:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        for ln in walk_code(cp, code):
            print("  ", ln)
        found = True
if not found:
    print("  (no canInterruptScreen method)")

print("=" * 72)
print("[B] Screen.canInterrupt / canRenderDebug methods:")
SC = "net/minecraft/client/gui/screens/Screen"
sm = methods_of(z, SC)
for sig, (cp, body) in sm.items():
    if "canInterrupt" in sig or "canRenderDebug" in sig:
        print("  METHOD:", sig)
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        for ln in walk_code(cp, code):
            print("    ", ln)

print("=" * 72)
print("[C] DebugOptionsScreen.isPauseScreen / onClose walk:")
DS = "net/minecraft/client/gui/screens/debug/DebugOptionsScreen"
dm = methods_of(z, DS)
for sig, (cp, body) in dm.items():
    if sig.startswith(("isPauseScreen", "onClose", "onRemoved", "tick")):
        print("  METHOD:", sig)
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        for ln in walk_code(cp, code):
            print("    ", ln)

print("=" * 72)
print("[D] DebugScreenProfile enum (the rows the profile buttons keep):")
DP = "net/minecraft/client/gui/components/debug/DebugScreenProfile"
try:
    pm = methods_of(z, DP)
    for sig in sorted(pm):
        print("  ", sig)
except Exception as ex:
    print("  skip:", ex)

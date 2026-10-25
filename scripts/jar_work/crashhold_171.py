#!/usr/bin/env python3
"""THE DEBUG CRASH'S OWN HOLD PROBE (the dev-171 round): the
KeyboardHandler.keyPress debugCrashKeyTime arm (pc 61-186 — the bl7
modifier==overlay comparison the usedDebugKeyAsModifier law answers,
the RAW isKeyDown poll the held-C the InputConstants walk serves, the
`> 0` state arm + the `-1` release-clear + the start's clock banks)
+ the KeyboardHandler.tick() expiry walk (pc 0-214 — the remaining
10000-(now-time) the ldc2_w constant keeps, the <2000 the
ReportedException "Manually triggered debug crash" the athrow serves,
the >=1000 cadence the count-0 YELLOW held row / the rest the RED
countdown rows speak) + the debugWarning/debugFeedback component
colors (the ChatFormatting.RED/YELLOW the decorateDebugComponent
wears) + the en_us lang rows. Chain-loads controls_185.py helpers.

Usage: python3 scripts/jar_work/crashhold_171.py
"""
import os, sys, zipfile, json

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)
methods_of = NS["methods_of"]; walk_code = NS["walk_code"]
utf = NS["utf"]; struct = NS["struct"]

JAR = os.path.join(os.path.dirname(HERE), "..", ".mc2rs", "classpath", "client.jar")
JAR = os.path.normpath(JAR)
z = zipfile.ZipFile(JAR)

meths = methods_of(z, "net/minecraft/client/KeyboardHandler")

print("[1] keyPress crash-hold arm (pc 25-240):")
for sig, (cp, body) in meths.items():
    if sig.startswith("keyPress"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        for ln in walk_code(cp, code):
            pc = int(str(ln).split(":")[0].strip())
            if 25 <= pc <= 240:
                print(ln)

print("[2] tick() FULL walk:")
for sig, (cp, body) in meths.items():
    if sig.startswith("tick()"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        for ln in walk_code(cp, code):
            print(ln)
        for idx in (1178, 1184, 1194, 1196):
            e = cp[idx]
            print(f"ldcw@{idx} =", repr(utf(cp, e[1])) if e[0] == 8 else e)
        break

print("[3] warning/feedback component colors:")
for sig, (cp, body) in meths.items():
    if "debugWarningComponent" in sig or "debugFeedbackComponent" in sig:
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        print(f"--- {sig} ---")
        for ln in walk_code(cp, code):
            print(ln)

print("[4] lang rows:")
data = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
lang = json.loads(data)
for k, v in lang.items():
    if "debug.crash" in k or "key.debug.crash" in k:
        print(f"  {k} = {v!r}")

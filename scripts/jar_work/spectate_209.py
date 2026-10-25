#!/usr/bin/env python3
"""THE SPECTATE LANE PROBE (the autopush-209 round): the
handleDebugKeys spectate matches arm (pc 420 — between copyRecreate
end 419 and switchGameMode 557, the copyloc_207.py lane-map scan
banks) + whatever body the arm calls (the inline walk or the helper
method the invoke serves) + the KeyboardHandler cp strings the lane
reads + the en_us lang rows. Chain-loads controls_185.py helpers.

Usage: python3 scripts/jar_work/spectate_209.py
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

CLS = "net/minecraft/client/KeyboardHandler"
meths = methods_of(z, CLS)

print("=" * 72)
print("[1] handleDebugKeys FULL walk (pc window 340-570 — the spectate")
print("    lane 420 the window opens, the switchGameMode 557 the close):")
invoked = set()
for sig, (cp, body) in meths.items():
    if sig.startswith("handleDebugKeys"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        lines = walk_code(cp, code)
        for ln in lines:
            try:
                pc = int(str(ln).split(":")[0].strip())
            except ValueError:
                continue
            if 340 <= pc <= 570:
                s = str(ln)
                print(s)
                if "Invoke" in s or "invoke" in s:
                    invoked.add(s.strip())
        # the cp strings the lane's constant pool keeps (Methodrefs + Strings)
        break

print("=" * 72)
print("[2] every KeyboardHandler method whose name smells spectate:")
for sig in meths:
    n = sig.split("(")[0].lower()
    if "spect" in n or "gamemode" in n or "creative" in n:
        body = meths[sig][1]
        print(f"  {sig}  body={None if body is None else len(body)}B")

print("=" * 72)
print("[3] KeyboardHandler cp strings containing 'spect'/'debug' (the")
print("    lang keys the ldcw serves):")
cp = None
for sig, (c, body) in meths.items():
    if c is not None:
        cp = c
        break
for i in range(1, len(cp)):
    try:
        e = cp[i]
    except IndexError:
        continue
    if not e:
        continue
    if e[0] == 8:
        s = utf(cp, e[1])
        if s and ("spect" in s.lower() or "debug" in s.lower() or "gamemode" in s.lower()):
            print(f"  cp[{i}] = {s!r}")

print("=" * 72)
print("[4] en_us lang rows (spectate / gamemode switch / debug category):")
names = [n for n in z.namelist() if n.endswith("en_us.json")]
print(f"  lang files: {names[:4]}")
for n in names:
    data = json.loads(z.read(n))
    for k, v in sorted(data.items()):
        kl = k.lower()
        if "spect" in kl or ("debug" in kl and "key" in kl):
            print(f"  {n}: {k} = {v!r}")

print("=" * 72)
print("[5] Minecraft.java spectate/gamemode-switch helpers (the delegate")
print("    the invoke serves — if handleDebugKeys calls out):")
for cls in ("net/minecraft/client/Minecraft", "net/minecraft/client/player/LocalPlayer"):
    try:
        m2 = methods_of(z, cls)
    except Exception as ex:
        print(f"  {cls}: {ex}")
        continue
    for sig in m2:
        n = sig.split("(")[0].lower()
        if "spect" in n or "gamemode" in n or "cycle" in n:
            body = m2[sig][1]
            print(f"  {cls}: {sig}  body={None if body is None else len(body)}B")

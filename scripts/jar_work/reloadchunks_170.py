#!/usr/bin/env python3
"""THE RELOAD CHUNKS' OWN LANE (dev-170): the handleDebugKeys window
the keyDebugReloadChunk field answers — the `matches` arm between the
advanced-tooltips lane (pc 297-349) and the copy-recreate lane (pc
351+) the options.txt binding `key.debug.reloadChunk:key.keyboard.a`
wears. The probe the lane's own truth reads: the field's seat, the
arm's byte walk (the allChanged call the chunk renderer's rebuild
banks, the chat row the lang word serves), the callee bodies (the
Minecraft.reloadChunks / LevelRenderer.allChanged pair the lane
rides), and the lang rows the feedback words keep.

Usage: python3 scripts/jar_work/reloadchunks_170.py <client.jar>
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

KH = "net.minecraft.client.KeyboardHandler"
MC = "net.minecraft.client.Minecraft"
LR = "net.minecraft.client.renderer.LevelRenderer"


def code_of(cls, sig):
    d = dict(methods_of(z, cls))
    for s, (cp, body) in d.items():
        if s == sig:
            if body is None:
                return d, None, None
            clen = struct.unpack_from(">I", body, 4)[0]
            return d, cp, body[8 : 8 + clen]
    return d, None, None


print("########## I. the keyDebugReloadChunk field seat (KeyboardHandler clinit)")
d = dict(methods_of(z, KH))
cp, body = d["<clinit>()V"]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
for i, t in enumerate(walk_code(cp, code)):
    if "reloadChunk" in t or "ReloadChunk" in t:
        print(f"  clinit[{i}]: {t.strip()}")

print()
print("########## II. handleDebugKeys — the reloadChunk arm's byte window")
HDK = "handleDebugKeys(Lnet/minecraft/client/input/KeyEvent;)Z"
d, cp, code = code_of(KH, HDK)
if code is None:
    raise SystemExit(f"{HDK} not found")
lines = list(walk_code(cp, code))
lo = hi = None
for i, t in enumerate(lines):
    if "keyDebugReloadChunk" in t:
        print(f"  [{i}]: {t.strip()}")
        lo = i
        hi = i
while hi is not None and hi + 1 < len(lines):
    nxt = lines[hi + 1]
    if "keyDebug" in nxt or nxt.strip().startswith("goto"):
        break
    hi += 1
if lo is not None:
    print(f"  arm window: pc-index {lo}..{hi} of {len(lines)}")
    for i in range(max(0, lo - 2), min(len(lines), hi + 12)):
        print(f"  [{i}]: {lines[i].strip()}")

print()
print("########## III. the callees the arm banks — reloadChunks / allChanged")
# what method does the arm call? print the full arm's invoke lines
if lo is not None:
    for i in range(lo, min(len(lines), hi + 12)):
        if "Method" in lines[i] or "Field" in lines[i]:
            print(f"  [{i}]: {lines[i].strip()}")

print()
print("########## IV. Minecraft.reloadChunks body (if the lane delegates)")
d2 = dict(methods_of(z, MC))
for sig, (c2, b2) in sorted(d2.items()):
    if "reloadChunk" in sig:
        print(f"  found: {sig}")
        if b2 is not None:
            clen2 = struct.unpack_from(">I", b2, 4)[0]
            for t in walk_code(c2, b2[8 : 8 + clen2]):
                print(f"    {t.strip()}")

print()
print("########## V. LevelRenderer.allChanged body")
d3 = dict(methods_of(z, LR))
for sig, (c2, b2) in sorted(d3.items()):
    if "allChanged" in sig:
        print(f"  found: {sig}")
        if b2 is not None:
            clen2 = struct.unpack_from(">I", b2, 4)[0]
            cnt = 0
            for t in walk_code(c2, b2[8 : 8 + clen2]):
                if cnt < 40:
                    print(f"    {t.strip()}")
                cnt += 1
            print(f"    ... ({cnt} insns total)")

print()
print("########## VI. the lang rows (debug.reload_chunks.*)")
import json
for n in z.namelist():
    if n.endswith("lang/en_us.json") and "minecraft" in n:
        table = json.loads(z.read(n).decode("utf-8"))
        for k in sorted(table):
            if "reload_chunks" in k or "debug.reload" in k:
                print(f"  {k} = {table[k]}")
        break

print()
print("########## VII. every addMessage/progress lane in the arm window's callee")
# reloadChunks on Minecraft — walk all its methods for 'allChanged' refs
for sig, (c2, b2) in sorted(d2.items()):
    if b2 is None:
        continue
    clen2 = struct.unpack_from(">I", b2, 4)[0]
    if clen2 == 0 or clen2 > 80000:
        continue
    code2 = b2[8 : 8 + clen2]
    try:
        ls = list(walk_code(c2, code2))
    except Exception:
        continue
    hits = [l for l in ls if "allChanged" in l]
    if hits:
        print(f"  Minecraft.{sig}: {len(hits)} allChanged refs")

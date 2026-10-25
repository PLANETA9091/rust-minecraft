#!/usr/bin/env python3
"""THE COPY LOCATION'S OWN LANE PROBE (the autopush-207 round): the
KeyboardHandler.handleDebugKeys keyDebugCopyLocation window + the
copyLocation() body the /tp format the walk answers + the
debug.copy_location.message lang row. Chain-loads controls_185.py
helpers.

Usage: python3 scripts/jar_work/copyloc_207.py
"""
import sys, zipfile

import os
HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)
methods_of = NS["methods_of"]; walk_code = NS["walk_code"]
utf = NS["utf"]; ref_name = NS["ref_name"]
parse_cp = NS["parse_cp"]; struct = NS["struct"]

JAR = ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

CLS = "net/minecraft/client/KeyboardHandler"

meths = methods_of(z, CLS)

print("=" * 72)
print("[1] method inventory (copyLocation-related):")
for sig in meths:
    name = sig.split("(")[0]
    if "opy" in name or "ocation" in name or "lipped" in name:
        body = meths[sig][1]
        print(f"  {sig}  body={None if body is None else len(body)}B")

print("=" * 72)
print("[2] handleDebugKeys FULL walk (pc window 300-900 — the copyRecreate")
print("    lane ends 419, the copyLocation lane the NEXT window keeps):")
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
            if 340 <= pc <= 760:
                print(ln)

print("=" * 72)
print("[3] the copyLocation() body — the /tp format the walk answers:")
for sig, (cp, body) in meths.items():
    if sig.startswith("copyLocation") or ("copyLocation" in sig and "(" in sig):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        print(f"--- {sig} ---")
        print(walk_code(cp, code))

print("=" * 72)
print("[4] KeyboardHandler cp strings containing 'location'/'debug.copy':")
cp, _ = meths["handleDebugKeys(*)"] if "handleDebugKeys(*)" in meths else (None, None)
# fall back: parse cp from any method
for sig, (cp, body) in meths.items():
    if cp is not None:
        break
for i in range(1, len(cp)):
    e = cp.get(i)
    if not e:
        continue
    if e[0] == 1:  # utf8
        s = e[1]
        if "location" in s.lower() or "debug.copy" in s.lower():
            print(f"  cp#{i}: {s!r}")

print("=" * 72)
print("[5] lang table rows (en_us.json inside the jar):")
try:
    data = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
    import json
    lang = json.loads(data)
    for k, v in lang.items():
        if "copy_location" in k or "copy" in k.lower() and "debug" in k:
            print(f"  {k} = {v!r}")
except KeyError:
    print("  en_us.json not at that path — scanning:")
    for n in z.namelist():
        if n.endswith("en_us.json") and "lang" in n:
            print("  found:", n)

#!/usr/bin/env python3
"""THE F3 DUMP VERSION'S OWN PROBE (the autopush-214 round): the
handleDebugKeys keyDebugDumpVersion lane the pc 1164 tail keeps — the
lane body, the cp strings, the en_us rows, the chord key the help row
speaks.

Chain-loads controls_185.py helpers.

Usage: python3 scripts/jar_work/dumpver_214.py
"""
import os, zipfile, json

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)
methods_of = NS["methods_of"]; walk_code = NS["walk_code"]
utf = NS["utf"]; struct = NS["struct"]

JAR = ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

CLS = "net/minecraft/client/KeyboardHandler"
meths = methods_of(z, CLS)

def walk_window(lo, hi, tag):
    print("=" * 72)
    print(f"[{tag}] handleDebugKeys pc {lo}-{hi}:")
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
                if lo <= pc <= hi:
                    print(ln)

walk_window(1158, 1300, "A-the dumpVersion lane body")

print("=" * 72)
print("[B] the cp entries the lane windows cite:")
for sig, (cp, body) in meths.items():
    if not sig.startswith("handleDebugKeys"):
        continue
    # deref the cp indices the lane walk cites — scan a wide band
    for idx in range(520, 570):
        if 0 <= idx < len(cp):
            e = cp[idx]
            if e[0] == 8:
                s = utf(cp, e[1])
                if "dump" in s or "version" in s.lower():
                    print(f"  cp[{idx}] str-> {s!r}")
    break

print("=" * 72)
print("[C] en_us rows (dump_version / dumpVersion):")
data = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
lang = json.loads(data)
for k, v in lang.items():
    if "dump_version" in k or "dumpVersion" in k or ("version" in k and "debug" in k):
        print(f"  {k} = {v!r}")

print("=" * 72)
print("[D] the Options clinit window — keyDebugDumpVersion default bind:")
OPT = "net/minecraft/client/Options"
om = methods_of(z, OPT)
for sig, (cp, body) in om.items():
    if "<init>" not in sig:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    lines = walk_code(cp, code)
    for idx, ln in enumerate(lines):
        s = str(ln)
        if "keyDebugDumpVersion" in s and "putfield" in s:
            lo = max(0, idx - 16)
            for j in range(lo, min(len(lines), idx + 2)):
                print("  ", lines[j])
            break
    break

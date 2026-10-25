#!/usr/bin/env python3
"""THE F3+F6 DEBUG OPTIONS' OWN PROBE (the autopush-213 round): the
DebugOptionsScreen class the lane opens — the zero-arg ctor, the rows
the screen wears (the categories / the toggles / the profiles), the
Options fields the checkboxes read-write, the KeyMapping default bind
(the F6 key the clinit banks), the lang rows.

Chain-loads controls_185.py helpers.

Usage: python3 scripts/jar_work/debugopt_213.py
"""
import sys, zipfile, json, re
import os

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)
methods_of = NS["methods_of"]; walk_code = NS["walk_code"]
utf = NS["utf"]; ref_name = NS["ref_name"]
parse_cp = NS["parse_cp"]; struct = NS["struct"]

JAR = ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

CLS = "net/minecraft/client/gui/screens/debug/DebugOptionsScreen"
print("=" * 72)
print("[A] DebugOptionsScreen methods:")
meths = methods_of(z, CLS)
for sig in sorted(meths):
    print("  ", sig)

print("=" * 72)
print("[B] DebugOptionsScreen <init> body:")
for sig, (cp, body) in meths.items():
    if sig.startswith("<init>"):
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8:8 + clen]
        for ln in walk_code(cp, code):
            print("  ", ln)
        break

print("=" * 72)
print("[C] DebugOptionsScreen cp strings (rows the screen keeps):")
cp_seen = False
for sig, (cp, body) in meths.items():
    if cp is None or cp_seen:
        continue
    for i in range(1, len(cp)):
        e = cp[i] if isinstance(cp, list) else cp.get(i)
        if not e or e[0] != 1:
            continue
        s = e[1]
        if len(s) > 2 and not s.startswith(("(", "L", "[", "net/", "java/", "com/")) \
           and "/" not in s and ";" not in s:
            print(f"  cp#{i}: {s!r}")
    cp_seen = True

print("=" * 72)
print("[D] DebugOptionsScreen field/method refs (the Options toggles):")
for sig, (cp, body) in meths.items():
    if cp is None:
        continue
    if not any(sig.startswith(p) for p in ("<init>", "init", "render", "onClose",
                                           "addRows", "lambda", "reposition",
                                           "update", "onOptions", "save")):
        continue
    seen = set()
    for i in range(1, len(cp)):
        e = cp[i] if isinstance(cp, list) else cp.get(i)
        if e and e[0] in (9, 10, 11, 12):  # fieldref/methodref/interface
            try:
                name = ref_name(cp, i)
            except Exception:
                continue
            if name and ("debug" in name.lower() or "Options" in name
                         or "Screen" in name or "reduced" in name.lower()):
                if name not in seen:
                    seen.add(name)
                    print(f"  [{sig.split('(')[0]}] {name}")

print("=" * 72)
print("[D2] DebugOptionsScreen init() body (the rows the screen serves):")
for sig, (cp, body) in meths.items():
    if not sig.startswith("init("):
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    for ln in walk_code(cp, code):
        print("  ", ln)
    break

print("=" * 72)
print("[D3] DebugOptionsScreen <clinit> body (the TITLE word):")
for sig, (cp, body) in meths.items():
    if not sig.startswith("<clinit>"):
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    for ln in walk_code(cp, code):
        print("  ", ln)
    break

print("=" * 72)
print("[E] the Options clinit window — keyDebugDebugOptions default bind:")
OPT = "net/minecraft/client/Options"
om = methods_of(z, OPT)
for sig, (cp, body) in om.items():
    if "<clinit>" not in sig:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    lines = walk_code(cp, code)
    for idx, ln in enumerate(lines):
        s = str(ln)
        if "keyDebugDebugOptions" in s:
            lo = max(0, idx - 14)
            for j in range(lo, min(len(lines), idx + 3)):
                print("  ", lines[j])
            break
    break

print("=" * 72)
print("[F] lang rows key.debug.* (the binds' own names):")
data = z.read("assets/minecraft/lang/en_us.json").decode("utf-8")
lang = json.loads(data)
for k, v in lang.items():
    if k.startswith("key.debug.") or "debugOption" in k:
        print(f"  {k} = {v!r}")

print("=" * 72)
print("[G] DebugOptionsScreen superclass + interfaces:")
raw = z.read(CLS + ".class")
m = re.search(rb"creens/Screen|AbstractSelectionList|screens/[A-Za-z]+Screen", raw[:4000])
# superclass via the constant pool is easier: reuse parse_cp
print("  (see [A] method list — the Screen extension the walk keeps)")

print("=" * 72)
print("[H] the checkbox row fields the screen toggles — Options debug* fields:")
raw = z.read(OPT + ".class")
names = set(re.findall(rb"debug[A-Z][A-Za-z]+", raw))
print("  field-name hits in Options.class:", sorted(n.decode() for n in names))

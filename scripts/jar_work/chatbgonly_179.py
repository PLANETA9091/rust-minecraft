#!/usr/bin/env python3
"""THE CHAT BAND'S OWN SCOPE PROBE (autopush-179): the `backgroundForChatOnly`
bool the accessibility array's index-5 seat hosts — the caption the indy
lambda speaks, the file row the visit-order adjacency the textbg row's AFTER
wears, the input band the `? raw : ARGB.colorFromFloat(textbg,0,0,0)` pair
the ChatComponent lane wears (the 177 record the law already banked — the
bytecode's own rows the probe re-answers).
"""
import struct
import sys
import zipfile

sys.path.insert(0, "scripts/jar_work")
core = {}
exec(compile(open("scripts/jar_work/fov_scale_tail_262.py").read().split("def main()")[0], "core", "exec"), core)

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")

def bodies_of(cls):
    cp, bs = core["method_bodies"](z, cls)
    return cp, bs

def dis(cp, body):
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    return core["disasm"](cp, code)

def find(bs, nm):
    for (n, d), b in bs.items():
        if n == nm:
            return b
    return None

# ---- 1. the caption key + the indy lambda (Options.<init> pc 2092 walk) ----
cp, bs = bodies_of("net.minecraft.client.Options")
body = find(bs, "<init>")
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8:8 + clen]
for _, l in dis(cp, code):
    if any(t in l for t in ["1129", "1131", "backgroundForChatOnly", "BOOLEAN_VALUES"]):
        print("INIT:", l)
print()

# resolve ldc_w #1129 -> the caption string
try:
    s = core["cp_get"](cp, 1129)
    print("CAPTION KEY #1129:", repr(s))
except Exception as e:
    print("cp_get 1129 fail:", repr(e))

# the indy #1131 -> the bootstrap method + the lambda impl
try:
    e = core["cp_get"](cp, 1131)
    print("INDY #1131 raw:", e if isinstance(e, str) else type(e))
except Exception as ex:
    print("indy fail:", repr(ex))
print()

# ---- 2. the CaptionBasedToString lambda: scan Options$ lambda methods ----
for suffix in ["Options$1", "Options$2", "Options$3", "Options$4"]:
    try:
        cp2, bs2 = bodies_of("net.minecraft.client." + suffix)
    except Exception:
        continue
    if bs2 is None:
        continue
    for (nm, desc), b in sorted(bs2.items()):
        if b is None or "lambda" not in nm:
            continue
        txt = "\n".join(l for _, l in dis(cp2, b))
        if "String" in txt or "Chat" in txt or "append" in txt or "ldc" in txt.lower() or "booleanValue" in txt:
            print("==", suffix, nm, desc)
            for _, l in dis(cp2, b):
                print("   ", l)
            print()

# ---- 3. the save key (processOptions ldcw@1896) ----
body = find(bs, "processOptions")
if body is not None:
    lines = dis(cp, body)
    for i, (pc, l) in enumerate(lines):
        if "backgroundForChatOnly" in l:
            # print the window: the key ldc two rows before
            for k in range(max(0, i - 4), min(len(lines), i + 3)):
                print("PROC:", lines[k][0], lines[k][1])
            print()
    # resolve the key strings near 1895/1896
    for idx in (1895, 1896, 1897):
        try:
            print("PROC KEY ldcw@%d:" % idx, repr(core["cp_get"](cp, idx)))
        except Exception as e:
            print("key", idx, "fail", repr(e))
print()

# ---- 4. the lang table's own row (en_us) ----
for path in z.namelist():
    if "en_us" in path.lower() and (path.endswith(".json") or path.endswith(".lang")):
        data = z.read(path).decode("utf-8", "replace")
        hits = [ln for ln in data.splitlines()
                if "backgroundForChatOnly" in ln or "chat.visibility" in ln
                or "options.chat" in ln]
        if hits:
            print("LANG FILE:", path)
            for ln in hits:
                print("   ", ln.strip()[:200])
            break
print()

# ---- 5. the consumer: the ChatComponent rows the getter wears ----
cc = "net.minecraft.client.gui.components.ChatComponent"
cp3, bs3 = bodies_of(cc)
for (nm, desc), b in sorted(bs3.items()):
    if b is None:
        continue
    txt = "\n".join(l for _, l in dis(cp3, b))
    if "backgroundForChatOnly" in txt:
        print("== CONSUMER", cc, "::", nm, desc)
        for _, l in dis(cp3, b):
            print("   ", l)
        print()

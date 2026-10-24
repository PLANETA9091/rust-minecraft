#!/usr/bin/env python3
"""THE HOTBAR ROWS PROBE (the dev-159 round): the key.hotbar.1..9
bindings the Options ctor the array wears (the registrations' own
order + the codes + the category), the lang rows the captions speak,
and the Minecraft.handleKeybinds hotbar loop the number-key lane
keeps (the consumeClick walk the slot select serves).

Usage: python3 scripts/jar_work/hotbar_159.py <client.jar>
"""
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
methods_of = ns["methods_of"]; parse_cp = ns["parse_cp"]; utf = ns["utf"]
class_utf8_words = ns["class_utf8_words"]; walk_code = ns["walk_code"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

print("########## A. lang rows (the hotbar slots speak)")
import json
lang = json.loads(z.read("assets/minecraft/lang/en_us.json"))
for k in sorted(lang):
    if k.startswith("key.hotbar."):
        print(f"  {k} = {lang[k]!r}")

print()
print("########## B. the ctor's hotbar registrations (the ldc walk)")
OPTS = "net.minecraft.client.Options"
z2 = zipfile.ZipFile(JAR)
d = z2.read("net/minecraft/client/Options.class")
cp, _ = parse_cp(d)
# find the String constants for key.hotbar.1..9
sites = {}
for i, e in enumerate(cp):
    if e and e[0] == "u" and utf(cp, i).startswith("key.hotbar."):
        u = i
        st = [j for j, e2 in enumerate(cp) if e2 and e2[0] == 8 and e2[1] == u]
        if st:
            sites[utf(cp, i)] = st[0]
print("String consts:", sites)
methods = methods_of(z, OPTS)
raw, body = methods["<init>(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V"]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8:8+clen]
lines = list(walk_code(raw, code))
for name, si in sorted(sites.items()):
    hits = [i for i, t in enumerate(lines) if t.endswith(f"ldcw@{si}")]
    for h in hits:
        print(f"===== {name} @ walk-line {h}")
        for t in lines[max(0, h-3):h+10]:
            print(t)
        print("     ----")

print()
print("########## C. handleKeybinds — the hotbar loop (the lane)")
MC = "net.minecraft.client.Minecraft"
try:
    d2 = dict(methods_of(z, MC))
    for sig in d2:
        if "handleKeybinds" in sig:
            print(f"*** {sig}")
            cp2, body2 = d2[sig]
            clen2 = struct.unpack_from(">I", body2, 4)[0]
            code2 = body2[8:8+clen2]
            out = list(walk_code(cp2, code2))
            hits = [i for i, t in enumerate(out) if "keyHotbar" in t or "selected" in t]
            shown = set()
            for i in hits:
                for j in range(max(0, i-6), min(len(out), i+10)):
                    if j not in shown:
                        print(out[j])
                        shown.add(j)
                print("     ----")
except Exception as e:
    print("!!", e)

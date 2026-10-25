#!/usr/bin/env python3
"""THE ADVANCED TOOLTIPS' OWN WALK (dev-168): the hdk_262.txt
pc 297-349 window the lane answers — the `options.advancedItemTooltips
= !options.advancedItemTooltips` the flip speaks, the
debugFeedbackTranslated the on/off strings wear, the `options.save()`
the persist the debug family's own save lane banks. The probe the
field's own truth reads: the ctor's default putfield, the save
walk's row presence (the OPTIONS table the row the file never wears?
the io walk the putfield answers), the lang rows the display words
keep, the debugFeedbackTranslated body the chat lane the overlay
serves.

Usage: python3 scripts/jar_work/advanced_tooltips_168.py <client.jar>
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

OPT = "net.minecraft.client.Options"

print("########## I. the advancedItemTooltips field — ctor default + type")
d = dict(methods_of(z, OPT))
cp, body = d["<init>(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V"]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
for t in walk_code(cp, code):
    if "advancedItemTooltips" in t:
        print(f"  ctor: {t.strip()}")

print()
print("########## II. the save() walk — does the row persist?")
for sig, (c2, b2) in sorted(d.items()):
    if sig.startswith("save"):
        clen2 = struct.unpack_from(">I", b2, 4)[0]
        code2 = b2[8 : 8 + clen2]
        lines = list(walk_code(c2, code2))
        hits = [l for l in lines if "advancedItemTooltips" in l]
        print(f"  {sig}: {len(lines)} insns, advancedItemTooltips hits={len(hits)}")
        for h in hits:
            print(f"      {h.strip()}")

print()
print("########## III. all advancedItemTooltips readers/writers in Options")
for sig, (c2, b2) in sorted(d.items()):
    if b2 is None:
        continue
    clen2 = struct.unpack_from(">I", b2, 4)[0]
    if clen2 == 0 or clen2 > 80000:
        continue
    code2 = b2[8 : 8 + clen2]
    try:
        lines = list(walk_code(c2, code2))
    except Exception:
        continue
    hits = [l for l in lines if "advancedItemTooltips" in l]
    if hits:
        print(f"  {sig}: {len(hits)} refs")

print()
print("########## IV. the lang rows (debug.show_advanced_tooltips.*)")
import json
for n in z.namelist():
    if n.endswith("lang/en_us.json") and "minecraft" in n:
        table = json.loads(z.read(n).decode("utf-8"))
        for k in sorted(table):
            if "advanced_tooltips" in k or "debug.feedback" in k:
                print(f"  {k} = {table[k]}")
        break

print()
print("########## V. debugFeedbackTranslated body — the display lane")
d2 = dict(methods_of(z, "net.minecraft.client.KeyboardHandler"))
for sig, (c2, b2) in sorted(d2.items()):
    if "debugFeedbackTranslated" in sig:
        clen2 = struct.unpack_from(">I", b2, 4)[0]
        code2 = b2[8 : 8 + clen2]
        print(f"  --- {sig}")
        for t in walk_code(c2, code2):
            print(f"    {t}")

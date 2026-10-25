#!/usr/bin/env python3
"""THE ADVANCED TOOLTIPS' OWN ROW (dev-168 part II): the
processOptions walk the field's file row answers — the two refs the
decode/encode closures wear, the key string the row's name speaks,
the default the null-arm keeps, the debugFeedbackComponent body the
display lane the overlay serves.

Usage: python3 scripts/jar_work/advanced_tooltips_row_168.py <client.jar>
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
d = dict(methods_of(z, OPT))

print("########## I. processOptions — the advancedItemTooltips window")
cp, body = d["processOptions(Lnet/minecraft/client/Options$FieldAccess;)V"]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
lines = list(walk_code(cp, code))
idxs = [i for i, l in enumerate(lines) if "advancedItemTooltips" in l]
print(f"  total {len(lines)} insns, refs at {idxs}")
for i in idxs:
    lo, hi = max(0, i - 12), min(len(lines), i + 6)
    for l in lines[lo:hi]:
        print(f"    {l.strip()}")
    print("    ...")

print()
print("########## II. the FieldAccess interface shape")
try:
    dd = dict(methods_of(z, "net.minecraft.client.Options$FieldAccess"))
    for sig, (c2, b2) in sorted(dd.items()):
        print(f"  {sig}")
except Exception as e:
    print("  not found:", e)

print()
print("########## III. debugFeedbackComponent body")
d2 = dict(methods_of(z, "net.minecraft.client.KeyboardHandler"))
for sig, (c2, b2) in sorted(d2.items()):
    if "debugFeedbackComponent" in sig and "(" in sig:
        if b2 is None:
            print(f"  --- {sig} (abstract)")
            continue
        clen2 = struct.unpack_from(">I", b2, 4)[0]
        code2 = b2[8 : 8 + clen2]
        print(f"  --- {sig}")
        for t in walk_code(c2, code2):
            print(f"    {t}")

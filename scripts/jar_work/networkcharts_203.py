#!/usr/bin/env python3
"""THE DEBUG NETWORK CHARTS'S OWN WALK (autopush-203): the next
registration the fpsCharts putfield the pc 3675 seat banks — the
`ldc_w cp#1516` the `key.debug.networkCharts` name answers, the
`KEYSYM, bipush 51` the '3' default wears, the `iconst_3` the
orderI=3 the FIVE-arg overload banks, the putfield
keyDebugNetworkCharts the honest field the seat speaks. THE NEXT
REGISTRATION the pc 3701 seat the later round banks (the cp#1521 the
`key.debug.lightmapTexture` string the `bipush 52` the 4 default the
walk resolves). The en_us lang row the display word keeps.

Usage: python3 scripts/jar_work/networkcharts_203.py <client.jar>
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

print("########## I. the next registrations (pc 3678-3743)")
d = dict(methods_of(z, OPT))
cp, body = d["<init>(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V"]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
for t in walk_code(cp, code):
    pc = t.split(":", 1)[0]
    try:
        p = int(pc)
    except ValueError:
        continue
    if 3678 <= p <= 3743:
        print(f"  {t}")

print()
print("########## II. the cp strings (the two-step indirection)")
for idx in (1516, 1521):
    try:
        e = cp[idx]
        print(f"  cp#{idx} -> {utf(cp, e[1])!r}")
    except Exception as ex:
        print(f"  cp#{idx} -> ({ex})")

print()
print("########## III. the en_us lang rows (key.debug.*)")
lang = None
for n in z.namelist():
    if n.endswith("lang/en_us.json") and "minecraft" in n:
        lang = n
        break
print(f"lang file: {lang}")
if lang:
    import json
    table = json.loads(z.read(lang).decode("utf-8"))
    for k in sorted(table):
        if k in ("key.debug.networkCharts", "key.debug.lightmapTexture"):
            print(f"  {k} = {table[k]}")

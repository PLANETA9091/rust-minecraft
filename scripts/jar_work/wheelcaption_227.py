#!/usr/bin/env python3
"""THE WHEEL'S OWN CAPTION (autopush-227): the Options lambda methods
the logMouse/unlogMouse the caption walk answers — the wheel row's
own display law speaks.
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

d = dict(methods_of(z, "net.minecraft.client.Options"))
for sig, (cp, body) in sorted(d.items()):
    if "lambda" not in sig:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8 : 8 + clen]
    txt = "\n".join(str(t) for t in walk_code(cp, code))
    if "logMouse" in txt or "unlogMouse" in txt or "mouseWheel" in txt:
        print(f"=== Options.{sig}")
        for t in walk_code(cp, code):
            print(f"  {t}")
        print()

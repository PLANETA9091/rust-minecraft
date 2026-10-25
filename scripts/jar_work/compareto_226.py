#!/usr/bin/env python3
"""THE COMPARETO'S OWN LAW (autopush-226): the KeyMapping.compareTo
bytecode the sort walk answers — the category/orderI/i18n-name keys
the exact ladder the KeyBindsList row order wears.
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

def dump_method(cls, needle):
    d = dict(methods_of(z, cls))
    for sig, (cp, body) in sorted(d.items()):
        if needle not in sig:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        print(f"=== {cls}.{sig}")
        for t in walk_code(cp, code):
            print(f"  {t}")
        print()

dump_method("net.minecraft.client.KeyMapping", "compareTo")
dump_method(
    "net.minecraft.client.gui.screens.options.controls.KeyBindsList", "<init>"
)

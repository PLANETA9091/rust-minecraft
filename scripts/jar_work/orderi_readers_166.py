#!/usr/bin/env python3
"""THE ORDERI'S OWN READERS (dev-166): the field the 5-arg ctor
banks — the KeyMapping methods the `getfield orderI` the getter
speaks, the KeyBindsList the sort walk the comparator answers (the
within-category ordering the orderI the list wears, or the
registration-order law the seat keeps).

Usage: python3 scripts/jar_work/orderi_readers_166.py <client.jar>
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

def dump_getfield(cls, needle):
    d = dict(methods_of(z, cls))
    for sig, (cp, body) in d.items():
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        lines = list(walk_code(cp, code))
        hits = [t for t in lines if needle in t]
        if hits:
            print(f"--- {cls}.{sig}")
            for t in lines:
                print(f"  {t}")
            print()

print("########## I. the KeyMapping methods the `orderI` the getfield answers")
dump_getfield("net.minecraft.client.KeyMapping", "orderI")

print("########## II. the KeyBindsList the orderI the walk answers")
dump_getfield("net.minecraft.client.gui.screens.options.controls.KeyBindsList", "orderI")

print("########## III. the KeyBindsList methods (the full signatures)")
d2 = dict(methods_of(z, "net.minecraft.client.gui.screens.options.controls.KeyBindsList"))
for sig in sorted(d2):
    print(f"  {sig}")

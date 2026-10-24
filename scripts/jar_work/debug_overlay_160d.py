#!/usr/bin/env python3
"""THE DEBUG WALK IV (dev-160): the Category's label the header key
answers, the register(String) the Identifier builds, the keyMappings
array assembly the save ladder order wears.

Usage: python3 scripts/jar_work/debug_overlay_160d.py <client.jar>
"""
import json
import struct
import sys
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]
methods_of = ns["methods_of"]
class_utf8_words = ns["class_utf8_words"]; list_methods = ns["list_methods"]
walk_code = ns["walk_code"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

OPT = "net.minecraft.client.Options"
CAT = "net.minecraft.client.KeyMapping$Category"

print("########## I. the Category walks (register / init / label / toString)")
dc = dict(methods_of(z, CAT))
for sig in [
    "register(Ljava/lang/String;)Lnet/minecraft/client/KeyMapping$Category;",
    "<init>(Lnet/minecraft/resources/Identifier;)V",
    "label()Lnet/minecraft/network/chat/Component;",
    "toString()Ljava/lang/String;",
    "id()Lnet/minecraft/resources/Identifier;",
]:
    try:
        cpc, bodyc = dc[sig]
        clenc = struct.unpack_from(">I", bodyc, 4)[0]
        codec = bodyc[8 : 8 + clenc]
        print(f"--- {sig}")
        for t in walk_code(cpc, codec):
            print(f"  {t}")
    except Exception as e:
        print(f"--- {sig} !! {e}")

print()
print("########## J. the keyMappings array assembly (the save ladder order)")
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
    if 3724 <= p <= 4700:
        print(f"  {t}")

#!/usr/bin/env python3
"""THE HITBOXES STRING'S OWN WALK (autopush-193): the cp#1436 utf8 the
CONSTANT_String entry answers — the `key.debug.showHitboxes` name the
ldc_w@3330 the pc 3327 walk banks. The cp#1441 the next row's own
string keeps (the clearChat the later round's seat).
"""
import struct
import zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]

z = zipfile.ZipFile(".mc2rs/classpath/client.jar")
OPT = "net.minecraft.client.Options"
d = dict(ns["methods_of"](z, OPT))
cp, body = d["<init>(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V"]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]

# the class's own cp (the methods_of walk the SAME cp the ctor wears)
cls_bytes = z.read("net/minecraft/client/Options.class")
cp_map, _ = parse_cp(cls_bytes)
for idx in (1436, 1441):
    kind, val = cp_map[idx]
    if kind == "u":
        print(f"cp#{idx}: utf8 -> {val.decode('utf-8', 'replace')}")
    elif kind == 8:  # CONSTANT_String -> utf8 index
        print(f"cp#{idx}: string -> {utf(cp_map, val)}")
    else:
        print(f"cp#{idx}: kind={kind} -> {val}")

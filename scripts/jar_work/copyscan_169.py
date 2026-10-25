#!/usr/bin/env python3
"""scan Minecraft.class method names for copy/recreate/clipboard"""
import struct, sys, zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
methods_of = ns["methods_of"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

for cls in ["net/minecraft/client/Minecraft", "net/minecraft/client/Minecraft$1"]:
    try:
        d = dict(methods_of(z, cls))
    except Exception as e:
        print(cls, "NOT FOUND:", e)
        continue
    print(f"===== {cls}: {len(d)} methods =====")
    for sig in sorted(d):
        low = sig.lower()
        if any(w in low for w in ("copy", "recreate", "clipboard", "createrec")):
            print("  ", sig)

#!/usr/bin/env python3
"""scan all classes for copyRecreate/clipboard methods (dev-169)"""
import struct, sys, zipfile

src = open("scripts/jar_work/controls_185.py").read()
ns = {}
exec(src.split("z = zipfile.ZipFile(JAR)")[0], ns)
methods_of = ns["methods_of"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

names = sys.argv[2].split(",") if len(sys.argv) > 2 else ["copyRecreate", "Recreate", "lipboard"]
hits = 0
for info in z.infolist():
    if not info.filename.endswith(".class") or "$" in info.filename:
        continue
    cls = info.filename[:-6]
    try:
        d = dict(methods_of(z, cls) or {})
    except Exception:
        continue
    for sig in d:
        if any(w in sig for w in names):
            print(f"{cls} :: {sig}")
            hits += 1
print(f"--- {hits} hits across jar")

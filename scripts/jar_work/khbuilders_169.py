#!/usr/bin/env python3
"""THE RECREATE COMMAND'S OWN BUILDERS + CP STRINGS (dev-169)"""
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


def dump(cls, sigwin):
    print(f"===== {cls} =====")
    try:
        d = dict(methods_of(z, cls) or {})
    except Exception as e:
        print("  CLASS NOT FOUND:", e)
        return
    for sig, (cp, body) in sorted(d.items()):
        if sigwin not in sig:
            continue
        if body is None:
            print(f"  --- {sig} (abstract/no body)")
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        print(f"  --- {sig} (max_locals={struct.unpack_from('>H', body, 2)[0]})")
        for t in walk_code(cp, code):
            print(f"    {t}")


dump("net/minecraft/client/KeyboardHandler", "copyCreateBlockCommand")
print()
dump("net/minecraft/client/KeyboardHandler", "copyCreateEntityCommand")

# deref the CP strings at the ldc_w indices the walk names
print("\n===== CP derefs (KeyboardHandler) =====")
d = dict(methods_of(z, "net/minecraft/client/KeyboardHandler"))
sig = "copyRecreateCommand(ZZ)V"
cp, body = d[sig]
clen = struct.unpack_from(">I", body, 4)[0]
code = body[8 : 8 + clen]
for t in walk_code(cp, code):
    if "ldcw@" in t:
        idx = int(t.split("ldcw@")[1].split()[0])
        e = cp[idx]
        try:
            s = utf(cp, e[1])
        except Exception as ex:
            s = f"<{ex}>"
        print(f"  ldcw@{idx} -> {s!r}")

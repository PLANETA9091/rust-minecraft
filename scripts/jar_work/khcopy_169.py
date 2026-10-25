#!/usr/bin/env python3
"""THE COPY RECREATE COMMAND'S OWN WALK (dev-169): the KeyboardHandler
copyRecreateCommand(ZZ)V body the hdk_262.txt pc 415 call answers —
the gates the lane's own walk serves (the hitResult type switch the
block/entity arms wear), the clipboard the TextFieldHelper setter
chains, the feedback rows the debugFeedback lanes speak.

Usage: python3 scripts/jar_work/khcopy_169.py <client.jar>
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


dump("net/minecraft/client/KeyboardHandler", "copyRecreateCommand")
print()
dump("net/minecraft/client/KeyboardHandler", "lambda$copyRecreateCommand")
print()
dump("net/minecraft/client/gui/font/TextFieldHelper", "createClipboardSetter")
dump("net/minecraft/client/gui/font/TextFieldHelper", "lambda$createClipboardSetter")

#!/usr/bin/env python3
"""THE DEBUG COPY RECREATE COMMAND'S OWN LANE (dev-169): the
hdk_262.txt pc 351-419 window the F3+I chord answers — the
`minecraft.copyRecreateCommand(!shift, shift)` the permission
gates wear, the used flag the unconditional istore_3 banks.
The walk the body's own reads: the hitResult null/type walk,
the block/entity recreate-command builders, the clipboard the
setClipboard call serves, the feedback rows the debugFeedback
lanes speak.

Usage: python3 scripts/jar_work/copyrecreate_169.py <client.jar>
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
        d = dict(methods_of(z, cls))
    except Exception as e:
        print("  CLASS NOT FOUND:", e)
        return {}
    for sig, (cp, body) in sorted(d.items()):
        if sigwin not in sig:
            continue
        clen = struct.unpack_from(">I", body, 4)[0]
        code = body[8 : 8 + clen]
        print(f"  --- {sig} (max_locals={struct.unpack_from('>H', body, 2)[0]})")
        for t in walk_code(cp, code):
            print(f"    {t}")
    return d


# 1. the copyRecreateCommand(ZZ)V body — the lane's own walk
dump("net/minecraft/client/Minecraft", "copyRecreateCommand")

# 2. the recreate-command builders the body names
print()
dump("net/minecraft/client/Minecraft", "createRecreateCommand")

# 3. the clipboard setter the body may call
print()
dump("net/minecraft/client/Minecraft", "setClipboard")

#!/usr/bin/env python3
"""THE NOTE BLOCK'S OWN THIRD PROBE (dev-54): the instrument field's
own boot assignment the BlockBehaviour$BlockStateBase ctor walks
speak — the above-block -> instrument mapping the Cache init rows
bank. Usage: python3 scripts/jar_work/note_block_682c.py <jar>
"""
import struct
import sys
import zipfile

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
src = open(__file__.rsplit("/", 1)[0] + "/note_block_682.py").read()
head = src[:src.index("z = zipfile.ZipFile(JAR)")]
ns = {"__name__": "probe"}
exec(head, ns)
parse_cp = ns["parse_cp"]; utf = ns["utf"]; walk_code = ns["walk_code"]
methods_of = ns["methods_of"]; dump_method = ns["dump_method"]

z = zipfile.ZipFile(JAR)

CLS = "net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase"
ms = methods_of(z, CLS)
for sig, (cp, body) in ms.items():
    if not body:
        continue
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    try:
        lines = walk_code(cp, code)
    except Exception:
        continue
    if any("putfield" in l and ".instrument:" in l for l in lines):
        print(f"--- {CLS}.{sig}  (writes .instrument)")
        for line in lines[:120]:
            print(line)

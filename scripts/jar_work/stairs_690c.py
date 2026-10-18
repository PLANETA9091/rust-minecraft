#!/usr/bin/env python3
"""Decode the two tableswitch instructions in StairBlock.getShape and
the getStairsShape walk (pc-accurate): which case (1..5 = STRAIGHT,
INNER_LEFT, INNER_RIGHT, OUTER_LEFT, OUTER_RIGHT ordinals) jumps to
which arm (facing / ccw / cw key)."""
import struct
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

def decode_tableswitch(cp, code, pc):
    """Decode tableswitch at pc; return (default, [(lo..hi, target)])."""
    pad = (4 - ((pc + 1) % 4)) % 4
    p = pc + 1 + pad
    default, lo, hi = struct.unpack_from(">iii", code, p)
    p += 12
    arms = []
    for i in range(hi - lo + 1):
        off = struct.unpack_from(">i", code, p)[0]
        arms.append((lo + i, pc + off))
        p += 4
    return (pc + default, arms, p)

ms = methods_of(z, "net.minecraft.world.level.block.StairBlock")
for meth in ("getShape", "getStairsShape", "canTakeShape"):
    sig = None
    for s in ms:
        if s.startswith(meth + "("):
            sig = s
            break
    cp, body = ms[sig]
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    print(f"--- {meth} ({sig})")
    p = 0
    while p < len(code):
        op = code[p]
        if op == 0xAA:  # tableswitch
            default, arms, np = decode_tableswitch(cp, code, p)
            print(f"  pc {p}: tableswitch default->{default}")
            for val, tgt in arms:
                print(f"    case {val} -> pc {tgt}")
            p = np
            continue
        # walk_code line for context: find pc mapping
        p += 1

# now print walk_code lines sorted by pc for the three methods
for meth in ("getShape", "getStairsShape", "canTakeShape"):
    sig = None
    for s in ms:
        if s.startswith(meth + "("):
            sig = s
            break
    cp, body = ms[sig]
    clen = struct.unpack_from(">I", body, 4)[0]
    code = body[8:8 + clen]
    print(f"=== full walk {meth}")
    for l in walk_code(cp, code):
        print("   ", l)

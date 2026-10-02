#!/usr/bin/env python3
"""Resolve the AnimationUtils constant-pool FLOAT constants the
animateCrossbowCharge/Hold bodies load — the exact f32 bits the
client's CROSSBOW_CHARGE/HOLD folds must carry."""
import struct
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp  # noqa: E402

import zipfile

JAR = "/home/z/my-project/client262.jar"
CLS = "net/minecraft/client/model/AnimationUtils.class"

with zipfile.ZipFile(JAR) as jar:
    data = jar.read(CLS)

cp, _ = parse_cp(data)


def show(idx):
    e = cp.get(idx)
    if not e:
        print(f"#{idx}: <missing>")
        return
    tag, val = e[0], e[1]
    if tag == "float":
        bits = struct.pack(">f", val).hex()
        v = struct.unpack(">f", struct.pack(">I", int(bits, 16)))[0]
        print(f"#{idx}  float   0x{bits}   {v!r}")
    elif tag == "int":
        print(f"#{idx}  int     {val!r}")
    elif tag == "method":
        print(f"#{idx}  method  {val!r}")
    else:
        print(f"#{idx}  tag={tag}  val={val!r}")


# charge: yRot sides #25/#26, xRot const #27, lerp args #32/#33, lerp end #19
# hold: charge yRot sides #7/#8, other yRot sides #15/#16, charge xRot #19+#23, other xRot #24
for idx in (7, 8, 15, 16, 19, 23, 24, 25, 26, 27, 32, 33, 17):
    show(idx)

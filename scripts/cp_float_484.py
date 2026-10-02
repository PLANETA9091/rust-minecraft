#!/usr/bin/env python3
"""Resolve the HumanoidModel constant-pool FLOAT constants the
poseBlockingArm body loads (CP #418/#419/#420/#421/#422/#242/#423/#417,
#95 known 0.5) — the exact f32 bits the client's BLOCK fold must carry."""
import struct
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp  # noqa: E402

import zipfile

JAR = "/home/z/my-project/client262.jar"
CLS = "net/minecraft/client/model/HumanoidModel.class"

with zipfile.ZipFile(JAR) as jar:
    data = jar.read(CLS)

cp, _ = parse_cp(data)
print("idx  tag     bits        value")
for idx in (418, 419, 420, 421, 422, 242, 423, 417, 95):
    e = cp.get(idx)
    if not e:
        print(f"{idx}: <missing>")
        continue
    tag, val = e[0], e[1]
    if tag == "float":
        bits = struct.pack(">f", val).hex()
        exact = struct.unpack(">f", struct.pack(">f", val))[0]
        print(f"#{idx}  float   0x{bits}   {val!r}   (exact {exact!r})")
    else:
        print(f"#{idx}  tag={tag}  val={val!r}")

#!/usr/bin/env python3
"""Resolve HumanoidModel CP #340 (the animateCrossbowCharge holder)
and list the method refs the class carries for the crossbow anims."""
import sys

sys.path.insert(0, __file__.rsplit("/", 1)[0])
from full_disasm import parse_cp  # noqa: E402

import zipfile

JAR = "/home/z/my-project/client262.jar"
CLS = "net/minecraft/client/model/HumanoidModel.class"

with zipfile.ZipFile(JAR) as jar:
    data = jar.read(CLS)

cp, _ = parse_cp(data)
for idx in (340,):
    e = cp.get(idx)
    print(f"#{idx}: tag={e[0]} val={e[1]!r}")

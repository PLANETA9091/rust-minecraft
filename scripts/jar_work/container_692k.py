#!/usr/bin/env python3
"""NINTH PASS: getChestType + candidatePartnerFacing + chestCanConnectTo +
getConnectedDirection walks (the double-chest connection law).
"""
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)
CB = "net.minecraft.world.level.block.ChestBlock"
for m in ("getChestType", "candidatePartnerFacing", "chestCanConnectTo",
          "getConnectedDirection", "getConnectedBlockPos"):
    dump_method(z, CB, m, max_pc=300)

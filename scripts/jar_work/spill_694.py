#!/usr/bin/env python3
"""THE CONTAINER SPILL'S OWN PROBE (the dev-67 round): the dig-broken
container's contents the item entities the dig speaks — javap
Containers / RandomizableContainer / ItemEntity the 26.2 jar (pinned
sha1 2dc72797) the dropContents walk the spill law banks, the
dropItemStack spread the position law speaks, the ItemEntity ctor the
throw impulse + the pickup delay the spawn carries, the setRemoved arm
the break beat the law rides.

Usage: python3 scripts/jar_work/spill_694.py <client.jar>
"""
import subprocess
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/cache/c9005c5562fc5135f33304b8553f928ccf787785"
z = zipfile.ZipFile(JAR)

def javap(cls, *opts):
    data = z.read(f"{cls.replace('.', '/')}.class")
    open("/tmp/_jv.class", "wb").write(data)
    cmd = ["javap"] + list(opts) + ["-c", "-p", "/tmp/_jv.class"]
    out = subprocess.run(cmd, capture_output=True, text=True).stdout
    return out

print("=== javap net.minecraft.world.Containers")
print(javap("net.minecraft.world.Containers"))

print("=== javap net.minecraft.world.entity.item.ItemEntity (ctors + fields)")
txt = javap("net.minecraft.world.entity.item.ItemEntity")
for line in txt.splitlines():
    if "ItemEntity(" in line or "static" in line or "final" in line:
        print(line)

print("=== javap -c ItemEntity ctors")
print(javap("net.minecraft.world.entity.item.ItemEntity"))

print("=== javap -c RandomizableContainer (setRemoved + the ctor walk)")
print(javap("net.minecraft.world.RandomizableContainer"))

#!/usr/bin/env python3
"""THE DOUBLE BREAK'S OWN PROBE (the dev-73 round): the dig-broken HALF of
a double chest the contents the break releases — ONE half's own 27 or the
PAIRED 54 the combined container wears? javap the 26.2 jar (pinned sha1
2dc72797): BlockEntity.preRemoveSideEffects the drop arm speaks,
RandomizableContainerBlockEntity + ChestBlockEntity the overrides keep,
ChestBlock.onRemove + ChestBlock.getContainer the pair walk names,
CompoundContainer the 54-slot merger the menu AND the drop share.

Usage: python3 scripts/jar_work/dblbreak_699.py <client.jar>
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


def javap_grep(cls, needle, *opts):
    txt = javap(cls, *opts)
    keep = False
    chunk = []
    for line in txt.splitlines():
        if line.strip().startswith(("public ", "protected ", "  static", "private ")) and "(" in line:
            keep = needle in line
            if keep:
                chunk.append("")
                chunk.append("  ---- " + line.strip())
        elif keep:
            chunk.append(line)
        if line.strip().startswith("}") and keep:
            keep = False
    return "\n".join(chunk)


print("=== javap net.minecraft.world.level.block.entity.BlockEntity.preRemoveSideEffects")
print(javap_grep("net.minecraft.world.level.block.entity.BlockEntity", "preRemoveSideEffects"))

print()
print("=== javap net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity (all)")
print(javap("net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity"))

print()
print("=== javap net.minecraft.world.level.block.entity.ChestBlockEntity (method list)")
txt = javap("net.minecraft.world.level.block.entity.ChestBlockEntity")
for line in txt.splitlines():
    if "(" in line and not line.strip().startswith("Code"):
        print(line)

print()
print("=== javap ChestBlockEntity.preRemoveSideEffects (if overridden)")
print(javap_grep("net.minecraft.world.level.block.entity.ChestBlockEntity", "preRemoveSideEffects"))

print()
print("=== javap net.minecraft.world.level.block.ChestBlock.onRemove")
print(javap_grep("net.minecraft.world.level.block.ChestBlock", "onRemove"))

print()
print("=== javap net.minecraft.world.level.block.ChestBlock (method list)")
txt = javap("net.minecraft.world.level.block.ChestBlock")
for line in txt.splitlines():
    if "(" in line and not line.strip().startswith("Code"):
        print(line)

print()
print("=== javap ChestBlock.getContainer (the pair walk)")
print(javap_grep("net.minecraft.world.level.block.ChestBlock", "getContainer"))

print()
print("=== javap net.minecraft.world.CompoundContainer (the 54 merger)")
print(javap("net.minecraft.world.CompoundContainer"))

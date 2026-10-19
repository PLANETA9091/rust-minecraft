#!/usr/bin/env python3
"""THE BLAST'S OWN FLAG PROBE (the dev-77 round): the explosion's block
clears — WHICH setBlock flag the 26.2 jar (pinned sha1 2dc72797) speaks,
the neighbor shape-update verdict the flag answers (UPDATE_NEIGHBORS the
ChestBlock.updateShape walk arms — the blast-killed half's survivor the
SINGLE wears?) and the CONTAINER SPILL verdict the same walk speaks
(preRemoveSideEffects the dropContents arm the blast path calls?).

javap targets:
  - ServerExplosion.finalizeExplosion — the per-cell walk, the
    onExplosionHit dispatch, the setBlock flag constants;
  - BlockBehaviour.onExplosionHit — the cited pc 179-191 setBlock tail;
  - BlockBehaviour$BlockStateBase.onExplosionHit — the override chain;
  - ChestBlock.wasExploded / ChestBlockEntity — the blast-side hooks.

Usage: python3 scripts/jar_work/blastflags_704.py <client.jar>
"""
import subprocess
import sys

sys.path.insert(0, "scripts/jar_work")
_helpers = open("scripts/jar_work/glass_685.py").read().split("z = zipfile.ZipFile(JAR)")[0]
exec(_helpers)

import zipfile  # noqa: E402

JAR = sys.argv[1] if len(sys.argv) > 1 else "tools/client.jar"
z = zipfile.ZipFile(JAR)


def javap(cls, *opts):
    data = z.read(f"{cls.replace('.', '/')}.class")
    open("/tmp/_jv.class", "wb").write(data)
    cmd = ["javap"] + list(opts) + ["-c", "-p", "/tmp/_jv.class"]
    out = subprocess.run(cmd, capture_output=True, text=True).stdout
    return out


def javap_method(cls, needle, *opts):
    txt = javap(cls, *opts)
    keep = False
    chunk = []
    for line in txt.splitlines():
        s = line.strip()
        if s.startswith(("public ", "protected ", "  static", "private ", "final ", "abstract ")) and "(" in s:
            keep = needle in s
            if keep:
                chunk.append("")
                chunk.append("  ---- " + s)
        elif keep:
            chunk.append(line)
        if s.startswith("}") and keep:
            keep = False
    return "\n".join(chunk)


print("=== 1. ServerExplosion.finalizeExplosion — the per-cell walk")
txt = javap_method("net.minecraft.world.level.ServerExplosion", "finalizeExplosion")
print(txt)

print("=== 2. BlockBehaviour.onExplosionHit — the cited setBlock tail")
print(javap_method("net.minecraft.world.level.block.state.BlockBehaviour", "onExplosionHit"))

print("=== 3. BlockBehaviour$BlockStateBase.onExplosionHit — the override chain")
print(javap_method("net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase", "onExplosionHit"))

print("=== 4. who calls preRemoveSideEffects (blast path?) — scan the big three")
for cls, meth in [
    ("net.minecraft.world.level.block.state.BlockBehaviour", "onExplosionHit"),
    ("net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase", "onExplosionHit"),
    ("net.minecraft.world.level.ServerExplosion", "finalizeExplosion"),
]:
    txt = javap_method(cls, meth)
    hits = [l.strip() for l in txt.splitlines() if "preRemoveSideEffects" in l]
    print(f"  {cls}: {hits or 'NO preRemoveSideEffects call'}")

print("=== 5. BlockEntity.preRemoveSideEffects — the dropContents arm (dev-73 verdict re-check)")
print(javap_method("net.minecraft.world.level.block.entity.BlockEntity", "preRemoveSideEffects"))

print("=== 6. ChestBlock blast-side hooks — wasExploded/onExplosionHit overrides?")
txt = javap("net.minecraft.world.level.block.ChestBlock", "-p")
print("\n".join(l for l in txt.splitlines() if "onExplosionHit" in l or "wasExploded" in l) or "  NO blast-side hooks on ChestBlock")

print("=== 7. dropResources-with-blockentity in the blast walk? (the container drop verdict)")
txt = javap_method("net.minecraft.world.level.ServerExplosion", "finalizeExplosion")
for needle in ("dropResources", "popResource", "onBlockExploded", "setBlock", "destroyBlock"):
    hits = [l.strip() for l in txt.splitlines() if needle in l]
    print(f"  {needle}: {hits or 'absent'}")

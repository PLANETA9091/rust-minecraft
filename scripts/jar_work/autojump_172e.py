#!/usr/bin/env python3
"""THE AUTOJUMP LAW PROBE e: index-windowed walks (pc-prefix optional) —
the aiStep autoJumpTime consumption + neighbors, the move() body, the
Entity gates (isStayingOnGroundSurface / isMoving).

Usage: python3 scripts/jar_work/autojump_172e.py [client.jar]
"""
import os
import sys
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
NS = {}
exec(open(os.path.join(HERE, "controls_185.py")).read().split("z = zipfile.ZipFile(JAR)")[0], NS)

walk_code = NS["walk_code"]
methods_of = NS["methods_of"]

JAR = sys.argv[1] if len(sys.argv) > 1 else ".mc2rs/classpath/client.jar"
z = zipfile.ZipFile(JAR)

LP = "net.minecraft.client.player.LocalPlayer"
EP = "net.minecraft.world.entity.Entity"


def lines_of(z, cls, sig_sub):
    ms = methods_of(z, cls)
    for sig, (cp, body) in ms.items():
        if sig_sub in sig and body:
            clen = int.from_bytes(body[4:8], "big")
            code = body[8:8 + clen]
            return sig, walk_code(cp, code)
    return None, None


def around(lines, needle, span):
    for i, ln in enumerate(lines):
        if needle in ln:
            lo, hi = max(0, i - span), min(len(lines), i + span)
            for j in range(lo, hi):
                mark = ">>" if i == j else "  "
                print(f"{mark} {lines[j]}")
            print("   ...")
            return
    print(f"   (no hit: {needle})")


print("[1] aiStep around autoJumpTime (both readsites, span 12)")
sig, lines = lines_of(z, LP, "aiStep()")
print(sig)
around(lines, "autoJumpTime", 14)

print()
print("[2] LocalPlayer.move full body")
sig, lines = lines_of(z, LP, "move(Lnet/minecraft/world/entity/MoverType;")
print(sig)
for ln in lines[:60]:
    print(ln)

print()
print("[3] Entity.isStayingOnGroundSurface")
sig, lines = lines_of(z, EP, "isStayingOnGroundSurface")
if lines:
    print(sig)
    for ln in lines[:30]:
        print(ln)
else:
    print("(not in Entity)")

print()
print("[4] Entity.isMoving")
sig, lines = lines_of(z, EP, "isMoving()")
if lines:
    print(sig)
    for ln in lines[:20]:
        print(ln)

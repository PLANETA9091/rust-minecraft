#!/usr/bin/env python3
"""THE AUTOJUMP LAW PROBE d (the dev-172 round): the updateAutoJump tail
(pc 810+, the trigger the walk banks), the aiStep autoJumpTime window by
pc, the move() body (the caller's args the law reads).

Usage: python3 scripts/jar_work/autojump_172d.py [client.jar]
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
ms = methods_of(z, LP)


def lines_of(sig_sub):
    for sig, (cp, body) in ms.items():
        if sig_sub in sig and body:
            clen = int.from_bytes(body[4:8], "big")
            code = body[8:8 + clen]
            return walk_code(cp, code)
    return None


print("[1] updateAutoJump tail pc 810..end")
lines = lines_of("updateAutoJump(")
start = next(i for i, ln in enumerate(lines) if ln.startswith("810:"))
for ln in lines[start:]:
    print(ln)

print()
print("[2] aiStep window pc 170..205 (autoJumpTime countdown)")
lines = lines_of("aiStep()")
start = next(i for i, ln in enumerate(lines) if ln.startswith("170:"))
end = next(i for i, ln in enumerate(lines) if ln.startswith("205:"))
for ln in lines[start:end]:
    print(ln)

print()
print("[3] LocalPlayer.move body")
lines = lines_of("move(Lnet/minecraft/world/entity/MoverType;")
for ln in lines[:80]:
    print(ln)

print()
print("[4] Entity.isStayingOnGroundSurface + isMoving walk")
EP = "net.minecraft.world.entity.Entity"
ems = methods_of(z, EP)
for sig, (cp, body) in ems.items():
    if body and ("isStayingOnGroundSurface" in sig or ("isMoving" in sig and "(Z)" not in sig)):
        clen = int.from_bytes(body[4:8], "big")
        code = body[8:8 + clen]
        print(f"--- Entity.{sig}")
        for ln in walk_code(cp, code)[:30]:
            print(ln)

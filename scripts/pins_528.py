#!/usr/bin/env python3
"""pins_528.py — the BRUSH (case 9) oracle (HumanoidModel.
poseRightArm/poseLeftArm case 9, pcs 294-325, the f32 chain the jar's
own bytecode keeps). Every op rounds: the client's fold must reproduce
the EXACT bits these chains speak.

brush body (the ONE-ARM fold, the walk's own xRot the body READS —
poseLeftArm pc 294-323 the same constants the mirror keeps):
  arm.xRot = f32(f32(arm.xRot * 0.5) - #407)
  arm.yRot = 0.0
CP = 0.5 (0x3F000000), #407 = 0.6283185482025146 (0x3F20D97C = pi/5).
THE VERDICT the use gate's BRUSH branch answers (AvatarRenderer.
getArmPose pcs 125-135: the stack's getUseAnimation() == BRUSH the
acmp — the RMB brush hold the verdict wears, the idle brush no
verdict the ITEM fall-through keeps). THE CLINIT'S OWN FLAG PAIR:
BRUSH(9,0,0) — twoHanded 0, affectsOffhandPose 0 — the gate the
second posing's chain never cancels (the FALSE row the BLOCK pair
shares; the left stack's ITEM fold the gate-open chain reaches).

the walk's own arm.xRot before the fold: AvatarModel resets + the
swing/attack lanes the client's build_posed_parts already wore — the
scenes pin the rest-state zeros and the crouch's raised arms the wrap
walk.
"""
import math
import struct


def f32(x):
    return struct.unpack(">f", struct.pack(">f", x))[0]


def bits(x):
    return struct.unpack(">I", struct.pack(">f", x))[0]


HALF = f32(0.5)                    # CP, 0x3F000000
PI_5 = f32(0.6283185482025146)     # CP #407
ITEM_PI10 = f32(0.3141592741012573)  # the ITEM fold's pi/10 contrast

print(f"HALF bits {bits(HALF):08X}  PI_5 bits {bits(PI_5):08X}"
      f"  ITEM_PI10 bits {bits(ITEM_PI10):08X}")

# SCENE Z (the rest-state arm: the walk's own zeros the fold reads):
# xRot = f32(f32(0.0 * 0.5) - PI_5) = -PI_5, yRot = 0.0
z = f32(f32(f32(0.0) * HALF) - PI_5)
print(f"rest xRot bits {bits(z):08X}  ({z!r})   yRot bits {bits(f32(0.0)):08X}")

# SCENE C (the crouch's own +0.4 raise the fold READS — the walk law
# the ITEM fold shares): xRot = f32(f32(0.4 * 0.5) - PI_5), the
# two-rounding chain the bits keep
raise04 = f32(0.4)
c = f32(f32(f32(raise04) * HALF) - PI_5)
print(f"crouch raise {raise04!r} bits {bits(raise04):08X}"
      f"  halved bits {bits(f32(f32(raise04) * HALF)):08X}"
      f" -> folded bits {bits(c):08X}  ({c!r})")

# SCENE L (the off-hand mirror): the left arm the same chain the same
# constants keep — the iconst_0 flag the body never reads (the fold
# the left method's own pc 294-323 the same bits speaks)
l = f32(f32(f32(0.0) * HALF) - PI_5)
print(f"scene-L xRot folded bits {bits(l):08X}  ({l!r})")

# THE GATE-OPEN CONTRAST (the affectsOffhand FALSE the second posing's
# chain NEVER cancels — the CONTRAST the trident TRUE row speaks): the
# left stack's ITEM verdict the chain REACHES — the ITEM fold the bits
# prove (the walk's own zero the −π/10 wears)
g = f32(f32(f32(0.0) * HALF) - ITEM_PI10)
print(f"gate-open left ITEM fold bits {bits(g):08X}  ({g!r})")

# THE ITEM FALL-THROUGH (the idle brush no verdict the ITEM keeps):
# the same −π/10 the idle scene wears
i = f32(f32(f32(0.0) * HALF) - ITEM_PI10)
print(f"idle brush ITEM fall-through bits {bits(i):08X}  ({i!r})")

# THE BOTH-FLAGS DEFENSE (the chain's own order the brush branch
# picks — the spear lane the tag law the same stack never carries):
# the brush fold the −π/5 the branch wears
print(f"both-flags brush branch bits {bits(z):08X}")

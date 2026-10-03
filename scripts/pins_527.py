#!/usr/bin/env python3
"""pins_527.py — the THROW_TRIDENT (case 4) oracle (HumanoidModel.
poseRightArm/poseLeftArm case 4, pcs 119-148, the f32 chain the jar's
own bytecode keeps). Every op rounds: the client's fold must reproduce
the EXACT bits these chains speak.

throw trident body (the ONE-ARM fold, the right flag the arm answers —
poseLeftArm pc 119-148 the same constants the mirror keeps):
  arm.xRot = f32(f32(arm.xRot * 0.5) - #260)
  arm.yRot = 0.0
CP #95 = 0.5 (0x3F000000), #260 = 3.1415927410125732 (0x40490FDB = pi).
THE VERDICT the use gate's TRIDENT branch answers (AvatarRenderer.
getArmPose pcs 81-91: the stack's getUseAnimation() == TRIDENT the
acmp — the RMB charge the verdict wears, the idle trident the SPEARS
tag the row table excludes the ITEM fall-through keeps). THE CLINIT'S
OWN FLAG PAIR: THROW_TRIDENT(4,0,1) — twoHanded 0, affectsOffhandPose
1 — the gate the second posing's CHAIN cancels (the one-arm body the
other arm's own verdict rides).

the walk's own arm.xRot before the fold: AvatarModel resets + the
swing/attack lanes the client's build_posed_parts already wore — the
scenes pin the rest-state zeros and the raised arms the wrap walk.
"""
import math
import struct


def f32(x):
    return struct.unpack(">f", struct.pack(">f", x))[0]


def bits(x):
    return struct.unpack(">I", struct.pack(">f", x))[0]


HALF = f32(0.5)                    # CP #95
PI = f32(3.1415927410125732)       # CP #260
TO_RAD = f32(math.pi / 180.0)

print(f"HALF bits {bits(HALF):08X}  PI bits {bits(PI):08X}")

# SCENE Z (the rest-state arm: the walk's own zeros the fold reads):
# xRot = f32(f32(0.0 * 0.5) - PI) = -PI, yRot = 0.0
z = f32(f32(f32(0.0) * HALF) - PI)
print(f"rest xRot bits {bits(z):08X}  ({z!r})   yRot bits {bits(f32(0.0)):08X}")

# SCENE W (the wrap walk): the walk cycle the arm ride — arm.xRot at
# the swing's peak the client's cos walk wears. Pin the chain from a
# raised arm x = 1.5707964 (pi/2):
w_in = f32(1.5707963705062866)
w = f32(f32(w_in * HALF) - PI)
print(f"raised xRot {w_in!r} bits {bits(w_in):08X} -> folded bits {bits(w):08X}  ({w!r})")

# SCENE P (the head's own pitch the walk pre-rides: xRot = pitch in
# radians the to_radians law keeps — the same input the block fold's
# raised 0.4 start speaks): pitch 60 deg:
p_in = f32(f32(60.0) * TO_RAD)
p = f32(f32(p_in * HALF) - PI)
print(f"pitch-60 arm xRot {p_in!r} bits {bits(p_in):08X} -> folded bits {bits(p):08X}  ({p!r})")

# SCENE L (the off-hand mirror): the left arm the same chain the same
# constants keep — the iconst_0 flag the body never reads (the fold
# the left method's own pc 119-148 the same bits speaks)
l = f32(f32(f32(-0.75) * HALF) - PI)
print(f"scene-L xRot(-0.75) folded bits {bits(l):08X}  ({l!r})")

# THE ITEM CONTRAST (the case-1 fold the using-trident never wears):
# xRot = x*0.5 - #279 (0.3141592741012573, 0x3EA0D97C = pi/10)
ITEM_PI10 = f32(0.3141592741012573)
i = f32(f32(f32(0.0) * HALF) - ITEM_PI10)
print(f"ITEM fold rest bits {bits(i):08X}  ({i!r})  const bits {bits(ITEM_PI10):08X}")
